use crate::ask_lib::ask::LocalAsk;
use crate::ask_lib::ask_status::AskState;
use crate::ask_lib::ask_store::LocalAskStore;
use crate::generator_lib::generator_store::{GeneratorMeta, GeneratorStore};
use crate::market_metadata::{MarketMetadataStore, MarketSetupData};
use crate::models::WelcomeResponse;
use crate::utility::{
    address_to_string, address_token_pair_to_token_amount, random_usize, TokenAmount, TokenTracker,
    USDC_TOKEN,
};
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

const DEFAULT_COUNT: &usize = &100;

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
struct RegisteredGenerator {
    details: GeneratorMeta,
    address: String,
    delegations: Vec<TokenAmount>,
    time: String,
    cost: TokenAmount,
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
    market_setup_data: MarketSetupData,
    registered_generator_list: Vec<RegisteredGenerator>,
    unmatched_jobs: Vec<Job>,
    completed_jobs: Vec<Job>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Job {
    requestor: String,
    time: String,
    cost: TokenAmount,
    ask_id: String,
    inputs: String,
    generator: Option<String>,
    status: AskState,
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
    let market_id: U256 = match U256::from_dec_str(&path.0) {
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
        return Ok(HttpResponse::Ok().json(cached_response.unwrap()));
    }

    drop(cached_response);

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

    let new_response = new_response.unwrap();

    match SINGLE_MARKET_RESPONSE.try_write() {
        Ok(mut data) => data.store(&market_query, new_response.clone()),
        _ => {
            log::warn!("Failed Caching Single Market response");
        }
    }

    return Ok(HttpResponse::Ok().json(new_response));
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

    let marketmetadata = marketmetadata.unwrap();

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
        .into_iter()
        .map(address_token_pair_to_token_amount)
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
                    .filter_by_market_id(market_id)
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
                    .filter_by_market_id(market_id)
                    .result();

                if result.is_some() {
                    result.unwrap().len()
                } else {
                    0
                }
            },
        },
        market_setup_data: marketmetadata.deserialize_market_bytes(),
        registered_generator_list: local_generator_store
            .get_all_by_market_id(&market_id)
            .iter()
            .filter_map(|element| {
                // Attempt to retrieve generator_info. If None, skip this element.
                local_generator_store
                    .get_by_address(&element.address)
                    .map(|generator_info| RegisteredGenerator {
                        details: generator_info.deserialize_generator_bytes(),
                        address: address_to_string(&element.address),
                        delegations: generator_info.total_stake.to_token_amount(),
                        time: element.proposed_time.to_string(),
                        cost: TokenAmount {
                            token: address_to_string(&USDC_TOKEN),
                            amount: element.proof_generation_cost.to_string(),
                        },
                    })
            })
            .collect::<Vec<RegisteredGenerator>>(),
        unmatched_jobs: local_ask_store
            .get_by_ask_state_except_complete(AskState::Create)
            .result()
            .map(|mut asks| {
                asks.sort_by(|a, b| a.ask_id.cmp(&b.ask_id));
                let local_asks = asks.into_iter().collect::<Vec<LocalAsk>>();

                local_asks
                    .into_iter()
                    .map(|a| Job {
                        ask_id: a.ask_id.to_string(),
                        requestor: address_to_string(&a.prover_refund_address),
                        cost: TokenAmount {
                            token: address_to_string(&USDC_TOKEN),
                            amount: a.reward.to_string(),
                        },
                        time: (a.deadline.saturating_sub(a.created_on)).to_string(),
                        inputs: a.prover_data.to_string(),
                        generator: None,
                        status: AskState::Create,
                    })
                    .collect()
            })
            .unwrap_or_default(),

        completed_jobs: local_ask_store
            .get_completed_proofs_of_market(&market_id, 0, DEFAULT_COUNT.clone())
            .into_iter()
            .map(|a| Job {
                ask_id: a.ask_id.to_string(),
                requestor: address_to_string(&a.prover_refund_address),
                cost: TokenAmount {
                    token: address_to_string(&USDC_TOKEN),
                    amount: a.reward.to_string(),
                },
                time: local_ask_store
                    .get_proving_time(&a.ask_id)
                    .unwrap_or_default()
                    .to_string(),
                inputs: a.prover_data.to_string(),
                generator: {
                    if a.generator.is_some() {
                        Some(address_to_string(&a.generator.unwrap()))
                    } else {
                        None
                    }
                },
                status: AskState::Complete,
            })
            .collect(),
    })
}
