use crate::ask_lib::ask::LocalAsk;
use crate::ask_lib::ask_store::LocalAskStore;
use crate::models::WelcomeResponse;
use crate::utility::address_to_string;
use crate::utility::bytes_to_string;
use actix_web::web;
use actix_web::HttpResponse;
use ethers::types::Address;
use ethers::types::U256;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;

use crate::ask_lib::ask_status::AskState;
use crate::generator_lib::generator_store::GeneratorStore;
use actix_web::web::Data;
use std::sync::Arc;

#[derive(Deserialize, Clone, Copy, Serialize, Debug, Hash, Eq, PartialEq)]
pub struct QueryParams {
    active_jobs_skip: Option<usize>,
    active_jobs: Option<usize>,
    completed_jobs_skip: Option<usize>,
    completed_jobs: Option<usize>,
    slashing_history_skip: Option<usize>,
    slashing_history: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneratorResponse {
    operator: Operator,
    reward_address: String,
    active_jobs: String,
    no_of_markets: String,
    total_earnings: String,
    total_slashed: Vec<TokenAmount>,
    total_delegations: Vec<TokenAmount>,
    markets: Vec<Market>,
    active_jobs_list: Vec<Job>,
    completed_jobs_list: Vec<Job>,
    slashing_history: Vec<Slash>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Slash {
    timestamp: String,
    market: MarketInfo,
    request: String, // Transaction Hash
    price_offered: String,
    expected_time: String,
    slashing_penalty: TokenAmount,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Job {
    market: MarketInfo,
    requestor: String,
    inputs: String,
    deadline: String,
    cost: String,
    time_taken_for_proof_generation: Option<String>,
    proof: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MarketInfo {
    name: Option<String>,
    id: String,
    token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Market {
    name: Option<String>,
    id: String,
    earnings_to_date: String,
    proofs_missed: String,
    slashing_penalties_incured: String,
    pending_proofs: String,
    min_hardware_requirement: MinHardware,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MinHardware {
    instance_type: String,
    vcpus: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TokenAmount {
    token: String,
    amount: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Operator {
    name: Option<String>,
    address: String,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct GeneratorQuery {
    generator: Address,
    query: QueryParams,
}

pub async fn single_generator(
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    _local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
    path: web::Path<(String,)>,
    query: web::Query<QueryParams>,
) -> actix_web::Result<HttpResponse> {
    let generator_id: Address = match path.into_inner().0.parse() {
        Ok(data) => data,
        _ => {
            return Ok(HttpResponse::BadRequest().json(WelcomeResponse {
                status: "Invalid Generator Id".into(),
            }))
        }
    };

    let generator_query = GeneratorQuery {
        generator: generator_id,
        query: QueryParams {
            active_jobs_skip: query.active_jobs_skip,
            active_jobs: query.active_jobs,
            completed_jobs_skip: query.completed_jobs_skip,
            completed_jobs: query.completed_jobs,
            slashing_history_skip: query.slashing_history_skip,
            slashing_history: query.slashing_history,
        },
    };

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

    // Step 1: Recompute the response every time
    let new_response = recompute_single_generator_response(
        generator_id,
        generator_query,
        local_ask_store,
        local_generator_store,
    )
    .await;

    if new_response.is_none() {
        return Ok(HttpResponse::NotFound().json(WelcomeResponse {
            status: "Generator Data Not Found".into(),
        }));
    }

    // Return the newly computed response
    return Ok(HttpResponse::Ok().json(new_response));
}

async fn recompute_single_generator_response<'a>(
    generator_id: Address,
    query: GeneratorQuery,
    local_ask_store: RwLockReadGuard<'a, LocalAskStore>,
    local_generator_store: RwLockReadGuard<'a, GeneratorStore>,
) -> Option<GeneratorResponse> {
    let generator_data = local_generator_store.get_by_address(&generator_id);

    if generator_data.is_none() {
        return None;
    }

    let generator_data = generator_data.unwrap();
    let all_markets_of_generator =
        local_generator_store.get_all_markets_of_generator(&generator_id);

    Some(GeneratorResponse {
        operator: Operator {
            name: Some("todo".into()),
            address: address_to_string(&generator_id),
        },
        reward_address: address_to_string(&generator_data.reward_address),
        active_jobs: all_markets_of_generator
            .clone()
            .into_iter()
            .map(|info| info.active_requests)
            .fold(U256::zero(), |a, x| a + x)
            .to_string(),
        no_of_markets: all_markets_of_generator.len().to_string(),
        total_earnings: local_generator_store
            .get_total_earning(&generator_id)
            .unwrap_or_default()
            .to_string(),
        total_slashed: vec![TokenAmount {
            amount: local_generator_store
                .get_total_slashing(&generator_id)
                .unwrap_or_default()
                .to_string(),
            token: "POND".into(),
        }],
        total_delegations: vec![TokenAmount {
            token: "POND".into(),
            amount: generator_data.total_stake.to_string(),
        }],
        markets: all_markets_of_generator
            .clone()
            .into_iter()
            .map(|info| Market {
                name: None,
                id: info.market_id.to_string(),
                earnings_to_date: local_generator_store
                    .get_earning_per_market(&generator_id, &info.market_id)
                    .unwrap_or_default()
                    .to_string(),
                proofs_missed: all_markets_of_generator
                    .clone()
                    .into_iter()
                    .map(|info| info.proofs_slashed)
                    .fold(U256::zero(), |a, x| a + x)
                    .to_string(),
                slashing_penalties_incured: all_markets_of_generator
                    .clone()
                    .into_iter()
                    .map(|info| info.proofs_slashed)
                    .fold(U256::zero(), |a, x| a + x)
                    .to_string(),
                pending_proofs: all_markets_of_generator
                    .clone()
                    .into_iter()
                    .map(|info| info.active_requests)
                    .fold(U256::zero(), |a, x| a + x)
                    .to_string(),
                min_hardware_requirement: MinHardware {
                    instance_type: "todo".into(),
                    vcpus: 1234,
                },
            })
            .collect(),
        active_jobs_list: local_ask_store
            .get_by_ask_state_except_complete(AskState::Assigned)
            .result()
            .map(|mut asks| {
                asks.sort_by(|a, b| b.ask_id.cmp(&a.ask_id));
                let local_asks = asks
                    .into_iter()
                    .filter(|ask| match &ask.generator {
                        Some(addr) => addr == &generator_id,
                        None => false,
                    })
                    .collect::<Vec<LocalAsk>>()
                    .into_iter()
                    .skip(query.query.active_jobs_skip.unwrap_or_default())
                    .take(query.query.active_jobs.unwrap_or_else(|| 10))
                    .collect::<Vec<LocalAsk>>();

                local_asks
                    .into_iter()
                    .map(|a| Job {
                        market: MarketInfo {
                            name: None,
                            id: a.market_id.to_string(),
                            token: "POND".to_string(),
                        },
                        requestor: address_to_string(&a.prover_refund_address),
                        inputs: bytes_to_string(&a.prover_data),
                        deadline: a.deadline.to_string(),
                        cost: a.reward.to_string(),
                        time_taken_for_proof_generation: None,
                        proof: None,
                    })
                    .collect()
            })
            .unwrap_or_default(),
        completed_jobs_list: local_ask_store
            .get_completed_proof_of_generator(
                &generator_id,
                query.query.completed_jobs_skip.unwrap_or_default(),
                query.query.completed_jobs.unwrap_or_else(|| 10),
            )
            .into_iter()
            .map(|ask| Job {
                market: MarketInfo {
                    name: None,
                    id: ask.market_id.to_string(),
                    token: "POND".to_string(),
                },
                requestor: address_to_string(&ask.prover_refund_address),
                inputs: bytes_to_string(&ask.prover_data),
                deadline: ask.deadline.to_string(),
                cost: ask.reward.to_string(),
                time_taken_for_proof_generation: None,
                proof: None,
            })
            .collect::<Vec<Job>>(),
        slashing_history: local_generator_store
            .get_slashing_records(&generator_id)
            .into_iter()
            .map(|record| Slash {
                timestamp: record.expected_time.to_string(),
                market: MarketInfo {
                    name: None,
                    id: record.market_id.to_string(),
                    token: "POND".into(),
                },
                request: record.slashing_tx,
                price_offered: record.price_offered.to_string(),
                expected_time: record.expected_time.to_string(),
                slashing_penalty: TokenAmount {
                    token: "POND".into(),
                    amount: record.slashing_penalty.to_string(),
                },
            })
            .collect(),
    })
}
