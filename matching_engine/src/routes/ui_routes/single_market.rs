use crate::ask_lib::ask::LocalAsk;
use crate::ask_lib::ask_status::AskState;
use crate::ask_lib::ask_store::{
    AskManagementRead, CompletedProofsManagement, MarketRequestCounters, ProofCounters,
    RequestorCounters, TimingOperations,
};
use crate::generator_lib::generator_helper::get_matching_scores;
use crate::generator_lib::generator_store::GeneratorMeta;
use crate::generator_lib::native_stake_store::NativeStakingOperations;
use crate::generator_lib::symbiotic_stake_store::TokenLockManagement;
use crate::generator_lib::traits::{
    GeneratorAdditionalQuery, GeneratorAvailability, GeneratorEarningsAndSlashing, GeneratorQuery,
    GeneratorRegistration, JobMissedCounter,
};
use crate::market_metadata::{MarketMetadataStoreRead, MarketSetupData, MinHardware};
use crate::models::WelcomeResponse;
use crate::try_read_or_lock;
use crate::utility::{
    address_to_string, convert_to_option_string, tx_to_string, TokenAmount, TokenTracker,
    USDC_TOKEN,
};
use actix_web::web::{self, Data};
use actix_web::HttpResponse;
use ethers::types::U256;
use im::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::time::Duration;
use utoipa::ToSchema;

const DEFAULT_COUNT: &usize = &100;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct Jobs {
    /// Number of proofs generated
    proofs_generated: usize,
    /// Number of proofs pending
    inputs_challenged: usize,
    /// Number of proofs in progress
    proofs_pending: usize,
    /// Number of requests made
    proofs_in_progress: usize,
    /// Number of requests made
    requests_made: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct RegisteredGenerator {
    /// Generator details
    details: GeneratorMeta,
    /// Generator address
    address: String,
    /// deprecated (use .stake_break_down instead)
    delegations: Vec<TokenAmount>,
    /// deprecated: Time of registration
    time: String,
    /// price quoted by generator to generate the proof
    cost: TokenAmount,
    /// Stake Break Down
    stake_break_down: StakeBreakDown,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct SingleMarketResponse {
    /// Number of registered generators
    registered_generators: usize,
    /// deprecated: Slashing penalty (no slashing yet)
    slashing_penalty: Vec<TokenAmount>,
    /// Median cost of proof generation
    median_cost: String,
    /// Median time of proof generation
    median_proof_time: String,
    /// Total earnings of the market
    total_earnings: String,
    /// deprecated: Total slashed amount (no slashing yet)
    total_slashed: Vec<TokenAmount>,
    /// deprecated: Hardware requirement (use .market_setup_data.min_hardware instead)
    hardware_requirement: MinHardware,
    /// Minimum stake required
    min_stake: Vec<TokenAmount>,
    /// Jobs
    jobs: Jobs,
    /// Market setup data
    market_setup_data: MarketSetupData,
    /// List of registered generators
    registered_generator_list: Vec<RegisteredGenerator>,
    /// List of unmatched jobs
    unmatched_jobs: Vec<JobInfo>,
    /// List of completed jobs
    completed_jobs: Vec<JobInfo>,
    /// Slashing penalty per module
    slashing_penalty_list: TokenList,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct JobInfo {
    /// Requestor address
    requestor: String,

    /// deprecated: Time of request (use .proof_created_on_timestamp instead - .matched_on_timestamp)
    time: String,

    /// deprecated: Price quoted by generator (use .quote instead)
    cost: TokenAmount,

    /// Price quoted by requestor
    quote: TokenAmount,

    /// deprecated: (use .settlement instead)
    solved_in: Option<TokenAmount>,

    /// Price Settled by Generator
    settlement: Option<TokenAmount>,

    /// Ask ID
    ask_id: String,

    /// Inputs
    inputs: String,

    /// Transaction Via which job was created
    inputs_transaction: String,

    /// Generator Address (if the job is matched)
    generator: Option<String>,

    /// Generator Details (if the job is matched)
    generator_details: Option<GeneratorMeta>,

    /// Status of the job
    status: AskState,

    /// timestamp of which the job was created
    created_on_timestamp: Option<String>,

    /// timestamp of which the job was matched
    matched_on_timestamp: Option<String>,

    /// timestamp of which the proof was created
    proof_created_on_timestamp: Option<String>,

    /// Transaction Via which proof was created
    proof_transaction: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct MarketQuery {
    market: U256,
}

use super::cache::CachedResponse;
use super::dashboard::TokenList;
use super::single_generator::StakeBreakDown;

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

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct JobRespone {
    jobs: Jobs,
}

#[utoipa::path(
    get,
    path = "/market_jobs/{id}",
    responses(
        (status = 200, description = "Returns Jobs of a given market", body = JobRespone),
        (status = 423, description = "Parsing in progress" )
    ),
    params(
        ("id" = u64, Path, description = "Market ID"),
    ),
    tag = "UI"
)]
pub async fn jobs<
    AS: AskManagementRead
        + CompletedProofsManagement
        + ProofCounters
        + RequestorCounters
        + TimingOperations
        + MarketRequestCounters
        + Send
        + Sync,
>(
    _local_ask_store: Data<Arc<RwLock<AS>>>,
    path: web::Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let market_id: U256 = match U256::from_dec_str(&path.0) {
        Ok(data) => data,
        _ => {
            return Ok(HttpResponse::BadRequest().json(WelcomeResponse {
                status: "Invalid Market Id".into(),
            }))
        }
    };

    try_read_or_lock!(_local_ask_store, local_ask_store);

    let jobs = Jobs {
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
        requests_made: { local_ask_store.get_request_count_by_market_id(&market_id) },
        inputs_challenged: { local_ask_store.get_failed_request_count_by_market_id(&market_id) },
    };

    let response = JobRespone { jobs };

    return Ok(HttpResponse::Ok().json(response));
}

#[utoipa::path(
    get,
    path = "/ui/market/{id}",
    responses(
        (status = 200, description = "Returns Market Info", body = SingleMarketResponse),
        (status = 423, description = "Parsing in progress" )
    ),
    params(
        ("id" = u64, Path, description = "Market ID"),
    ),
    tag = "UI"
)]
pub async fn single_market<
    MS: MarketMetadataStoreRead + Send + Sync,
    AS: AskManagementRead
        + CompletedProofsManagement
        + ProofCounters
        + RequestorCounters
        + TimingOperations
        + MarketRequestCounters
        + Send
        + Sync,
    GS: GeneratorAdditionalQuery
        + GeneratorRegistration
        + GeneratorAvailability
        + GeneratorEarningsAndSlashing
        + Send
        + Sync,
    NS: NativeStakingOperations + Send + Sync,
    SS: TokenLockManagement + Send + Sync,
>(
    _local_market_store: Data<Arc<RwLock<MS>>>,
    _local_ask_store: Data<Arc<RwLock<AS>>>,
    _local_generator_store: Data<Arc<RwLock<GS>>>,
    _local_native_store: Data<Arc<RwLock<NS>>>,
    _local_symbiotic_store: Data<Arc<RwLock<SS>>>,
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
        Ok(data) => data.get(&market_query, Duration::from_millis(100)),
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

    try_read_or_lock!(_local_ask_store, local_ask_store);
    try_read_or_lock!(_local_market_store, local_market_store);
    try_read_or_lock!(_local_generator_store, local_generator_store);
    try_read_or_lock!(_local_native_store, local_native_store);
    try_read_or_lock!(_local_symbiotic_store, local_symbiotic_store);

    let new_response = recompute_single_market_response(
        market_id,
        local_market_store,
        local_ask_store,
        local_generator_store,
        local_native_store,
        local_symbiotic_store,
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

#[utoipa::path(
    get,
    path = "/ui/market_generator_score/{id}",
    responses(
        (status = 200, description = "Returns Generator score for market", body = Vec<MatchingScoreResponse>),
        (status = 423, description = "Parsing in progress" )
    ),
    params(
        ("id" = u64, Path, description = "Market ID"),
    ),
    tag = "UI"
)]
pub async fn get_matching_score<
    GS: JobMissedCounter + GeneratorQuery + GeneratorRegistration + Send + Sync,
>(
    _local_generator_store: Data<Arc<RwLock<GS>>>,
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

    try_read_or_lock!(_local_generator_store, local_generator_store);

    let new_response = get_generator_scores_of_market(market_id, local_generator_store).await;

    if new_response.is_empty() {
        return Ok(HttpResponse::NotFound().json(WelcomeResponse {
            status: "Market Data Not Found".into(),
        }));
    }

    return Ok(HttpResponse::Ok().json(new_response));
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct MatchingScoreResponse {
    generator_address: String,
    generator_name: Option<String>,
    score: String,
}

async fn get_generator_scores_of_market<
    'a,
    GS: JobMissedCounter + GeneratorQuery + GeneratorRegistration,
>(
    market_id: U256,
    local_generator_store: RwLockReadGuard<'a, GS>,
) -> Vec<MatchingScoreResponse> {
    let generator_info_per_market = local_generator_store
        .query_by_market_id_and_only_active(&market_id)
        .result();
    let mut missed_jobs = std::collections::HashMap::new();
    for generator_info in generator_info_per_market.iter() {
        let missed = local_generator_store.get_job_missed_count_in_window(&generator_info.address);
        missed_jobs.insert(generator_info.address, missed);
    }

    let matching_scores = get_matching_scores(&generator_info_per_market, &missed_jobs);

    let responses = generator_info_per_market
        .iter()
        .zip(matching_scores.iter())
        .map(|(generator_info, score)| MatchingScoreResponse {
            generator_address: format!("{:?}", generator_info.address.to_string()),
            generator_name: local_generator_store
                .get_by_address(&generator_info.address)
                .map(|a| a.deserialize_generator_bytes().display_name)
                .flatten(),
            score: score.to_string(),
        })
        .collect::<Vec<MatchingScoreResponse>>();

    responses
}

async fn recompute_single_market_response<
    'a,
    MS: MarketMetadataStoreRead,
    AS: AskManagementRead
        + CompletedProofsManagement
        + ProofCounters
        + RequestorCounters
        + TimingOperations
        + MarketRequestCounters,
    GS: GeneratorAdditionalQuery
        + GeneratorRegistration
        + GeneratorAvailability
        + GeneratorEarningsAndSlashing,
    NS: NativeStakingOperations,
    SS: TokenLockManagement,
>(
    market_id: U256,
    local_market_store: RwLockReadGuard<'a, MS>,
    local_ask_store: RwLockReadGuard<'a, AS>,
    local_generator_store: RwLockReadGuard<'a, GS>,
    local_native_store: RwLockReadGuard<'a, NS>,
    local_symbiotic_store: RwLockReadGuard<'a, SS>,
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

    let task_requirement = TokenList {
        native: local_native_store
            .tokens_to_lock()
            .await
            .clone()
            .to_token_amount(),
        symbiotic: local_symbiotic_store
            .tokens_to_lock()
            .clone()
            .to_token_amount(),
    };

    let slashing_penalty = local_native_store.tokens_to_lock().await.clone()
        + local_symbiotic_store.tokens_to_lock().clone();

    Some(SingleMarketResponse {
        median_cost,
        median_proof_time,
        registered_generators: registered_generators.len(),
        // slashing_penalty: slashing_penalty.to_token_amount(),// TODO enable this after slashing is enabled
        // slashing_penalty: TokenTracker::default().to_token_amount(),
        slashing_penalty_list: task_requirement.clone(),
        slashing_penalty: slashing_penalty.to_token_amount(),
        total_earnings: local_market_store
            .get_earnings(&market_id)
            .unwrap_or_default()
            .to_string(),
        total_slashed: registered_generators
            .into_iter()
            .map(|elem| {
                local_generator_store
                    .get_slashing_per_generator_per_market(&elem.address, &market_id)
                    .unwrap_or_else(TokenTracker::new)
            })
            .fold(TokenTracker::new(), |acc, elem| acc + elem)
            .to_token_amount(),
        hardware_requirement: marketmetadata.deserialize_market_bytes().min_hardware,
        min_stake: slashing_penalty.to_token_amount(),
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
            requests_made: { local_ask_store.get_request_count_by_market_id(&market_id) },
            inputs_challenged: {
                local_ask_store.get_failed_request_count_by_market_id(&market_id)
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
                        delegations: (generator_info.clone().total_native_stake
                            + generator_info.clone().total_symbiotic_stake)
                            .to_token_amount(),
                        time: element.proposed_time.to_string(),
                        cost: TokenAmount {
                            token: address_to_string(&USDC_TOKEN),
                            amount: element.proof_generation_cost.to_string(),
                        },
                        stake_break_down: StakeBreakDown {
                            total_native_stake: generator_info
                                .clone()
                                .total_native_stake
                                .to_token_amount(),
                            total_native_stake_locked: generator_info
                                .native_stake_locked
                                .to_token_amount(),
                            total_symbiotic_stake: generator_info
                                .clone()
                                .total_symbiotic_stake
                                .to_token_amount(),
                            total_symbiotic_stake_locked: generator_info
                                .symbiotic_stake_locked
                                .to_token_amount(),
                            available_native_stake: (generator_info.total_native_stake
                                - generator_info.native_stake_locked)
                                .to_token_amount(),
                            available_symbiotic_stake: (generator_info.total_symbiotic_stake
                                - generator_info.symbiotic_stake_locked)
                                .to_token_amount(),
                        },
                    })
            })
            .collect::<Vec<RegisteredGenerator>>(),
        unmatched_jobs: local_ask_store
            .get_by_ask_state_except_complete(AskState::Create)
            .filter_by_market_id(market_id)
            .result()
            .map(|mut asks| {
                asks.sort_by(|a, b| a.ask_id.cmp(&b.ask_id));
                let local_asks = asks.into_iter().collect::<Vec<LocalAsk>>();

                local_asks
                    .into_iter()
                    .map(|a| JobInfo {
                        ask_id: a.ask_id.to_string(),
                        requestor: address_to_string(&a.prover_refund_address),
                        cost: TokenAmount {
                            token: address_to_string(&USDC_TOKEN),
                            amount: a.reward.to_string(),
                        },
                        quote: TokenAmount {
                            token: address_to_string(&USDC_TOKEN),
                            amount: a.reward.to_string(),
                        },
                        solved_in: None,
                        settlement: None,
                        time: a.time_requested_for_proof_generation.to_string(),
                        inputs: a.prover_data.to_string(),
                        generator: None,
                        status: AskState::Create,
                        created_on_timestamp: convert_to_option_string(
                            local_ask_store.get_job_created_on_timestamp(&a.ask_id),
                        ),
                        matched_on_timestamp: convert_to_option_string(
                            local_ask_store.get_job_matched_on_timestamp(&a.ask_id),
                        ),
                        proof_created_on_timestamp: convert_to_option_string(
                            local_ask_store.get_job_completed_on_timestamp(&a.ask_id),
                        ),
                        inputs_transaction: tx_to_string(&a.create_transaction),
                        generator_details: None,
                        proof_transaction: None,
                    })
                    .collect()
            })
            .unwrap_or_default(),

        completed_jobs: local_ask_store
            .get_completed_proofs_of_market(&market_id, 0, DEFAULT_COUNT.clone())
            .into_iter()
            .map(|a| JobInfo {
                ask_id: a.ask_id.to_string(),
                requestor: address_to_string(&a.prover_refund_address),
                cost: TokenAmount {
                    token: address_to_string(&USDC_TOKEN),
                    amount: a.reward.to_string(),
                },
                quote: TokenAmount {
                    token: address_to_string(&USDC_TOKEN),
                    amount: a.reward.to_string(),
                },
                solved_in: Some(TokenAmount {
                    token: address_to_string(&USDC_TOKEN),
                    amount: local_ask_store
                        .get_proving_cost(&a.ask_id)
                        .unwrap_or_default()
                        .to_string(),
                }),
                settlement: Some(TokenAmount {
                    token: address_to_string(&USDC_TOKEN),
                    amount: local_ask_store
                        .get_proving_cost(&a.ask_id)
                        .unwrap_or_default()
                        .to_string(),
                }),
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
                generator_details: {
                    if a.generator.is_some() {
                        let proof_generated_by = a.generator.unwrap().clone();
                        let proof_generated_by =
                            local_generator_store.get_by_address(&proof_generated_by);
                        proof_generated_by
                            .map(|generator_info| generator_info.deserialize_generator_bytes())
                    } else {
                        None
                    }
                },
                status: AskState::Complete,
                created_on_timestamp: convert_to_option_string(
                    local_ask_store.get_job_created_on_timestamp(&a.ask_id),
                ),
                matched_on_timestamp: convert_to_option_string(
                    local_ask_store.get_job_matched_on_timestamp(&a.ask_id),
                ),
                proof_created_on_timestamp: convert_to_option_string(
                    local_ask_store.get_job_completed_on_timestamp(&a.ask_id),
                ),
                inputs_transaction: tx_to_string(&a.create_transaction),
                proof_transaction: local_ask_store.get_proof_transaction(&a.ask_id),
            })
            .collect(),
    })
}
