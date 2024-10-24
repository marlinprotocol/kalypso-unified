// const dashboardData = {
// registeredGenerators: 291,
// medianTime: '3.5 Sec',
// medianCost: '$0.75',
// slashingPenalty: '5%',
// totalEarnings: 'USDT 1,234,567',
// totalSlashed: 'USDT 98,765',
// hardwareReq: {
// vCpu: 0,
// vGpu: 0,
// enclave: '',
// minStake: 0
// },
// jobs:{
// total:300,
// active:0,
// unMatched:0,
// }
// };

use crate::ask_lib::ask_status::AskState;
use crate::ask_lib::ask_store::LocalAskStore;
use crate::generator_lib::generator_store::GeneratorStore;
use crate::market_metadata::MarketMetadataStore;
use crate::models::WelcomeResponse;
use crate::utility::{address_token_pair_to_token_amount, random_usize, TokenAmount, TokenTracker};
use actix_web::web::{self, Data};
use actix_web::HttpResponse;
use ethers::types::U256;
use im::HashMap;
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MinHardware {
    instance_type: String,
    vcpus: usize,
    enclave: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Jobs {
    proofs_generated: usize,
    proofs_pending: usize,
    proofs_in_progress: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SingleMarketResponse {
    registered_generators: usize,
    slashing_penalty: Vec<TokenAmount>,
    median_cost: String,
    median_proof_time: String,
    total_earnings: String,
    total_slashed: Vec<TokenAmount>,
    hardware_requirement: MinHardware,
    min_stake: Vec<TokenAmount>,
    jobs: Jobs,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct MarketQuery {
    market: U256,
}

use super::cache::CachedResponse;

type CachedSingleMarketResponse = CachedResponse<SingleMarketResponse>;

struct CachedMarketResponse {
    data: HashMap<MarketQuery, CachedSingleMarketResponse>,
}

static SINGLE_MARKET_RESPONSE: Lazy<RwLock<CachedMarketResponse>> =
    Lazy::new(|| RwLock::new(CachedMarketResponse::new()));

impl CachedMarketResponse {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, query: &MarketQuery, timeout: Duration) -> Option<SingleMarketResponse> {
        let cache = self.data.get(query);
        if cache.is_none() {
            None
        } else {
            cache.unwrap().get_if_valid(timeout)
        }
    }

    pub fn store(&mut self, query: &MarketQuery, response: SingleMarketResponse) {
        // Attempt to get a mutable reference to the cache
        if let Some(cache) = self.data.get_mut(query) {
            // If the cache exists, store the response mutably
            cache.store(response);
        } else {
            // If the cache does not exist, create a new one
            let mut cache = CachedSingleMarketResponse::new();
            cache.store(response);
            self.data.insert(*query, cache);
        }
    }
}

pub async fn single_market(
    _local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    _local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
    path: web::Path<(String,)>,
    // query: web::Query<QueryParams>, // If required add latter
) -> actix_web::Result<HttpResponse> {
    let market_id: U256 = match path.into_inner().0.parse() {
        Ok(data) => data,
        _ => {
            return Ok(HttpResponse::BadRequest().json(WelcomeResponse {
                status: "Invalid Market Id".into(),
            }))
        }
    };

    let market_query = MarketQuery {
        market: market_id,
        // add queryParams here if required
    };

    let cached_response = match SINGLE_MARKET_RESPONSE.try_read() {
        Ok(data) => data.get(&market_query, Duration::from_secs(10)),
        _ => {
            return Ok(HttpResponse::Locked().json(WelcomeResponse {
                status: "Resource Busy".into(),
            }))
        }
    };

    if cached_response.is_some() {
        return Ok(HttpResponse::Ok().json(cached_response));
    }

    let local_ask_store = {
        match _local_ask_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let local_market_store = {
        match _local_market_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let local_generator_store = {
        match _local_generator_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let new_response = recompute_single_market_response(
        market_id,
        local_market_store,
        local_ask_store,
        local_generator_store,
    )
    .await;

    if new_response.is_none() {
        return Ok(HttpResponse::NotFound().json(WelcomeResponse {
            status: "Market Data Not Found".into(),
        }));
    }

    match SINGLE_MARKET_RESPONSE.try_write() {
        Ok(mut data) => data.store(&market_query, new_response.clone().unwrap()),
        _ => {
            log::warn!("Failed Caching Single Market response");
        }
    }

    return Ok(HttpResponse::Ok().json(new_response.unwrap()));
}

async fn recompute_single_market_response<'a>(
    market_id: U256,
    local_market_store: RwLockReadGuard<'a, MarketMetadataStore>,
    local_ask_store: RwLockReadGuard<'a, LocalAskStore>,
    local_generator_store: RwLockReadGuard<'a, GeneratorStore>,
) -> Option<SingleMarketResponse> {
    let marketmetadata = local_market_store.get_market_by_market_id(&market_id);

    if marketmetadata.is_none() {
        return None;
    }

    let median_cost = local_market_store
        .get_median_proof_cost_market_wise(&market_id)
        .to_string();
    let median_proof_time = local_market_store
        .get_median_proof_time_market_wise(&market_id)
        .to_string();
    let registered_generators = local_generator_store.get_all_by_market_id(&market_id);

    let local_generator_store_arc = Arc::new(local_generator_store.clone());

    let slashing_penalty = local_market_store
        .get_slashing_penalty_by_market_id(&market_id)
        .unwrap_or_default() // Simplified unwrapping
        .into_iter() // Convert Vec into an iterator
        .map(address_token_pair_to_token_amount) // Apply the transformation
        .collect::<Vec<TokenAmount>>();

    Some(SingleMarketResponse {
        median_cost,
        median_proof_time,
        registered_generators: registered_generators.len(),
        slashing_penalty: slashing_penalty.clone(),
        total_earnings: local_market_store
            .get_earnings(&market_id)
            .unwrap_or_default()
            .to_string(),
        total_slashed: registered_generators
            .into_par_iter()
            .map(|elem| {
                let store = Arc::clone(&local_generator_store_arc);
                match store.get_slashing_per_generator_per_market(&elem.address, &market_id) {
                    Some(slashed) => slashed,
                    None => TokenTracker::new(),
                }
            })
            .reduce(|| TokenTracker::new(), |acc, elem| acc + elem)
            .to_token_amount(),
        hardware_requirement: MinHardware {
            instance_type: "todo".into(),
            vcpus: random_usize(),
            enclave: true,
        },
        min_stake: slashing_penalty,
        jobs: Jobs {
            proofs_generated: local_ask_store.get_proof_count(&market_id),
            proofs_pending: {
                let result = local_ask_store
                    .get_by_ask_state_except_complete(AskState::Create)
                    .result();

                if result.is_some() {
                    result.unwrap().len()
                } else {
                    0
                }
            },
            proofs_in_progress: {
                let result = local_ask_store
                    .get_by_ask_state_except_complete(AskState::Assigned)
                    .result();

                if result.is_some() {
                    result.unwrap().len()
                } else {
                    0
                }
            },
        },
    })
}
