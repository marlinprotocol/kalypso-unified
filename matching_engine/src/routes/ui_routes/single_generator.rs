use crate::ask_lib::ask::LocalAsk;
use crate::ask_lib::ask_store::LocalAskStore;
use crate::generator_lib::delegation::Operation;
use crate::generator_lib::delegation::Source;
use crate::generator_lib::generator_store::GeneratorMeta;
use crate::generator_lib::key_store::Key;
use crate::generator_lib::key_store::KeyStore;
use crate::generator_lib::native_stake_store::NativeStakingStore;
use crate::generator_lib::symbiotic_stake_store::SymbioticStakeStore;
use crate::market_metadata::MarketMetadataStore;
use crate::models::WelcomeResponse;
use crate::try_read_or_lock;
use crate::utility::address_to_string;
use crate::utility::address_token_pair_to_token_amount;
use crate::utility::bytes_to_string;
use crate::utility::convert_to_option_string;
use crate::utility::tx_to_string;
use crate::utility::TokenAmount;
use crate::utility::TokenTracker;
use actix_web::web;
use actix_web::HttpResponse;
use ethers::types::Address;
use ethers::types::U256;
use im::HashMap;
use once_cell::sync::Lazy;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use tokio::time::Duration;

use crate::ask_lib::ask_status::AskState;
use crate::generator_lib::generator_store::GeneratorStore;
use actix_web::web::Data;
use std::sync::Arc;

use super::cache::CachedResponse;

type CachedSingleGeneratorResponse = CachedResponse<GeneratorResponse>;

const DEFAULT_COUNT: &usize = &100;

struct CachedGeneratorResponse {
    data: HashMap<GeneratorQuery, CachedSingleGeneratorResponse>,
}

static SINGLE_GENERATOR_RESPONSE: Lazy<RwLock<CachedGeneratorResponse>> =
    Lazy::new(|| RwLock::new(CachedGeneratorResponse::new()));

impl CachedGeneratorResponse {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, query: &GeneratorQuery, timeout: Duration) -> Option<GeneratorResponse> {
        self.data
            .get(query)
            .and_then(|cache| cache.get_if_valid(timeout))
    }

    pub fn store(&mut self, query: &GeneratorQuery, response: GeneratorResponse) {
        // Attempt to get a mutable reference to the cache
        if let Some(cache) = self.data.get_mut(query) {
            // If the cache exists, store the response mutably
            cache.store(response);
        } else {
            // If the cache does not exist, create a new one
            let mut cache = CachedSingleGeneratorResponse::new();
            cache.store(response);
            self.data.insert(*query, cache);
        }
    }
}

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
    details: GeneratorMeta,
    reward_address: String,
    kalypso_points: String,
    active_jobs: String,
    no_of_markets: String,
    total_earnings: String,
    total_slashed: Vec<TokenAmount>,
    total_delegations: Vec<TokenAmount>,
    markets: Vec<Market>,
    active_jobs_list: Vec<Job>,
    completed_jobs_list: Vec<Job>,
    slashing_history: Vec<Slash>,
    available_stake: Vec<TokenAmount>,
    stake_locked: Vec<TokenAmount>,
    delegations: Vec<DelegateOperation>,
    my_delegations: Vec<TokenAmount>,
    withdrawal_requests: Vec<WithdrawRequest>,
    stake_break_down: StakeBreakDown,
    compute_break_down: ComputeBreakDown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StakeBreakDown {
    pub total_native_stake: Vec<TokenAmount>,
    pub total_native_stake_locked: Vec<TokenAmount>,
    pub total_symbiotic_stake: Vec<TokenAmount>,
    pub total_symbiotic_stake_locked: Vec<TokenAmount>,
    pub available_native_stake: Vec<TokenAmount>,
    pub available_symbiotic_stake: Vec<TokenAmount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeBreakDown {
    pub total_compute: String,
    pub compute_locked: String,
    pub compute_available: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WithdrawRequest {
    account: String,
    token: String,
    amount: String,
    index: String,
    withdrawal_timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DelegateOperation {
    delegation: TokenAmount,
    source: String,
    operation: String,
    block_number: String,
    transaction_index: String,
    log_index: String,
    tx: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Slash {
    ask_id: Option<String>,
    timestamp: String,
    market: MarketInfo,
    request: String, // Transaction Hash
    price_offered: String,
    slashing_penalty: TokenAmount,
    slasing_epoch_timestamp: Option<String>,
    source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Job {
    ask_id: String,
    market: MarketInfo,
    requestor: String,
    inputs: String,
    inputs_transaction: String,
    deadline: String,
    cost: String,
    time_taken_for_proof_generation: Option<String>,
    proof: Option<String>,
    proof_transaction: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MarketInfo {
    name: Option<String>,
    id: String,
    token: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Market {
    name: Option<String>,
    id: String,
    earnings_to_date: String,
    proofs_missed: String,
    proofs_generated: String,
    pending_proofs: String,
    slashing_penalties_incured: String,
    min_hardware_requirement: Option<MinHardware>,
    enclave_key: Option<Key>,
    kalypso_points: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MinHardware {
    instance_type: Option<String>,
    vcpus: Option<usize>,
    vgpus: Option<usize>,
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

pub async fn withdrawal_request(
    _local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
    path: web::Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let generator_id: Address = match path.into_inner().0.parse() {
        Ok(data) => data,
        _ => {
            return Ok(HttpResponse::BadRequest().json(WelcomeResponse {
                status: "Invalid Generator Id".into(),
            }))
        }
    };

    try_read_or_lock!(_local_generator_store, local_generator_store);

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct WithdrawalResponse {
        withdrawal_requests: Vec<WithdrawRequest>,
    }

    let withdrawal_requests = local_generator_store
        .get_withdrawl_requests(&generator_id)
        .iter()
        .map(|a| WithdrawRequest {
            account: address_to_string(&a.account),
            index: a.index.to_string(),
            token: address_to_string(&a.token),
            amount: a.amount.to_string(),
            withdrawal_timestamp: a.timestamp.to_string(),
        })
        .collect::<Vec<WithdrawRequest>>();

    let response = WithdrawalResponse {
        withdrawal_requests,
    };

    return Ok(HttpResponse::Ok().json(response));
}

pub async fn single_generator(
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    _local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
    _local_key_store: Data<Arc<RwLock<KeyStore>>>,
    _local_native_store: Data<Arc<RwLock<NativeStakingStore>>>,
    _local_symbiotic_store: Data<Arc<RwLock<SymbioticStakeStore>>>,
    _local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,

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

    let cached_response = match SINGLE_GENERATOR_RESPONSE.try_read() {
        Ok(data) => data.get(&generator_query, Duration::from_millis(100)),
        _ => {
            return Ok(HttpResponse::Locked().json(WelcomeResponse {
                status: "Resource Busy".into(),
            }))
        }
    };

    if cached_response.is_some() {
        return Ok(HttpResponse::Ok().json(cached_response));
    }

    drop(cached_response);

    try_read_or_lock!(_local_ask_store, local_ask_store);
    try_read_or_lock!(_local_key_store, local_key_store);
    try_read_or_lock!(_local_generator_store, local_generator_store);
    try_read_or_lock!(_local_native_store, local_native_store);
    try_read_or_lock!(_local_symbiotic_store, local_symbiotic_store);
    try_read_or_lock!(_local_market_store, local_market_store);

    // Step 1: Recompute the response every time
    let new_response = recompute_single_generator_response(
        generator_id,
        generator_query,
        local_ask_store,
        local_generator_store,
        local_key_store,
        local_native_store,
        local_symbiotic_store,
        local_market_store,
    )
    .await;

    if new_response.is_none() {
        return Ok(HttpResponse::NotFound().json(WelcomeResponse {
            status: "Generator Data Not Found".into(),
        }));
    }

    let new_response = new_response.unwrap();

    match SINGLE_GENERATOR_RESPONSE.try_write() {
        Ok(mut data) => data.store(&generator_query, new_response.clone()),
        _ => {
            log::warn!("Failed Caching Single Generator response");
        }
    }

    // Return the newly computed response
    return Ok(HttpResponse::Ok().json(new_response));
}

async fn recompute_single_generator_response<'a>(
    generator_id: Address,
    query: GeneratorQuery,
    local_ask_store: RwLockReadGuard<'a, LocalAskStore>,
    local_generator_store: RwLockReadGuard<'a, GeneratorStore>,
    local_key_store: RwLockReadGuard<'a, KeyStore>,
    local_native_store: RwLockReadGuard<'a, NativeStakingStore>,
    local_symbiotic_store: RwLockReadGuard<'a, SymbioticStakeStore>,
    local_market_store: RwLockReadGuard<'a, MarketMetadataStore>,
) -> Option<GeneratorResponse> {
    let generator_data = local_generator_store.get_by_address(&generator_id);

    if generator_data.is_none() {
        return None;
    }

    let generator_data = generator_data.unwrap();
    let all_markets_of_generator =
        local_generator_store.get_all_markets_of_generator(&generator_id);

    let (all_tokens_supported, _): (Vec<Address>, Vec<U256>) =
        (local_native_store.tokens_to_lock.clone() + local_symbiotic_store.tokens_to_lock.clone())
            .to_address_token_pair()
            .into_iter()
            .unzip();

    let details = generator_data.deserialize_generator_bytes();
    Some(GeneratorResponse {
        operator: Operator {
            name: details.display_name.clone(),
            address: address_to_string(&generator_id),
        },
        details,
        kalypso_points: local_generator_store
            .get_kalypso_points(&generator_id)
            .unwrap_or_default()
            .to_string(),
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
        total_slashed: local_generator_store
            .get_total_slashing(&generator_id)
            .unwrap_or_default()
            .to_token_amount(),
        total_delegations: (generator_data.clone().total_native_stake
            + generator_data.clone().total_symbiotic_stake)
            .to_token_amount(),
        available_stake: (local_generator_store
            .get_available_native_stake(&generator_id)
            .unwrap_or_default()
            + local_generator_store
                .get_available_symbiotic_stake(&generator_id)
                .unwrap_or_default())
        .to_token_amount(),
        stake_break_down: StakeBreakDown {
            total_native_stake: generator_data.clone().total_native_stake.to_token_amount(),
            total_native_stake_locked: generator_data.native_stake_locked.to_token_amount(),
            total_symbiotic_stake: generator_data
                .clone()
                .total_symbiotic_stake
                .to_token_amount(),
            total_symbiotic_stake_locked: generator_data.symbiotic_stake_locked.to_token_amount(),
            available_native_stake: (generator_data.total_native_stake
                - generator_data.native_stake_locked)
                .to_token_amount(),
            available_symbiotic_stake: (generator_data.total_symbiotic_stake
                - generator_data.symbiotic_stake_locked)
                .to_token_amount(),
        },
        stake_locked: (local_generator_store
            .get_native_stake_locked(&generator_id)
            .unwrap_or_default()
            + local_generator_store
                .get_symbiotic_stake_locked(&generator_id)
                .unwrap_or_default())
        .to_token_amount(),
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
                proofs_missed: info.proofs_slashed.to_string(),
                proofs_generated: info.proofs_submitted.to_string(),
                slashing_penalties_incured: info.proofs_slashed.to_string(),
                pending_proofs: info.active_requests.to_string(),
                min_hardware_requirement: {
                    let info = local_market_store.get_market_by_market_id(&info.market_id);
                    if info.is_none() {
                        None
                    } else {
                        let market_setup_data = info.unwrap().deserialize_market_bytes();
                        Some(MinHardware {
                            instance_type: market_setup_data.min_hardware.instance_type,
                            vcpus: market_setup_data.min_hardware.vcpus,
                            vgpus: market_setup_data.min_hardware.vgpus,
                        })
                    }
                },
                kalypso_points: local_generator_store
                    .get_kalypso_points_per_market(&generator_id, &info.market_id)
                    .unwrap_or_default()
                    .to_string(),
                enclave_key: local_key_store.get_by_address(&info.address, info.market_id.as_u64()),
            })
            .collect(),
        active_jobs_list: local_ask_store
            .get_by_ask_state_except_complete(AskState::Assigned)
            .result()
            .map(|mut asks| {
                asks.sort_by(|a, b| a.ask_id.cmp(&b.ask_id));
                let local_asks = asks
                    .into_par_iter()
                    .filter(|ask| match &ask.generator {
                        Some(addr) => addr == &generator_id,
                        None => false,
                    })
                    .collect::<Vec<LocalAsk>>()
                    .into_iter()
                    .skip(query.query.active_jobs_skip.unwrap_or_default())
                    .take(query.query.active_jobs.unwrap_or_else(|| *DEFAULT_COUNT))
                    .collect::<Vec<LocalAsk>>();

                local_asks
                    .into_par_iter()
                    .map(|a| Job {
                        ask_id: a.ask_id.to_string(),
                        market: MarketInfo {
                            name: None,
                            id: a.market_id.to_string(),
                            token: all_tokens_supported
                                .iter()
                                .map(|a| address_to_string(a))
                                .collect::<Vec<String>>(),
                        },
                        requestor: address_to_string(&a.prover_refund_address),
                        inputs: bytes_to_string(&a.prover_data),
                        deadline: a.deadline.to_string(),
                        cost: a.reward.to_string(),
                        time_taken_for_proof_generation: None,
                        proof: None,
                        proof_transaction: None,
                        inputs_transaction: tx_to_string(&a.create_transaction),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        completed_jobs_list: local_ask_store
            .get_completed_proof_of_generator(
                &generator_id,
                query.query.completed_jobs_skip.unwrap_or_default(),
                query.query.completed_jobs.unwrap_or_else(|| *DEFAULT_COUNT),
            )
            .into_par_iter()
            .map(|ask| Job {
                ask_id: ask.ask_id.to_string(),
                market: MarketInfo {
                    name: None,
                    id: ask.market_id.to_string(),
                    token: all_tokens_supported
                        .iter()
                        .map(|a| address_to_string(a))
                        .collect::<Vec<String>>(),
                },
                requestor: address_to_string(&ask.prover_refund_address),
                inputs: bytes_to_string(&ask.prover_data),
                deadline: ask.deadline.to_string(),
                cost: ask.reward.to_string(),
                time_taken_for_proof_generation: Some(
                    local_ask_store
                        .get_proving_time(&ask.ask_id)
                        .unwrap_or_default()
                        .to_string(),
                ),
                proof: Some(
                    local_ask_store
                        .get_proof_by_ask_id(&ask.ask_id)
                        .unwrap_or_default()
                        .to_string(),
                ),
                proof_transaction: local_ask_store.get_proof_transaction(&ask.ask_id),
                inputs_transaction: tx_to_string(&ask.create_transaction),
            })
            .collect::<Vec<Job>>(),
        slashing_history: local_generator_store
            .get_slashing_records(&generator_id)
            .into_par_iter()
            .map(|record| Slash {
                ask_id: Some(record.ask_id.to_string()),
                slasing_epoch_timestamp: convert_to_option_string(Some(record.slashing_timestamp)),
                timestamp: record.slashing_block_number.to_string(),
                market: MarketInfo {
                    name: None,
                    id: record.market_id.to_string(),
                    token: all_tokens_supported
                        .iter()
                        .map(|a| address_to_string(a))
                        .collect::<Vec<String>>(),
                },
                request: record.slashing_tx,
                price_offered: record.price_offered.to_string(),
                slashing_penalty: address_token_pair_to_token_amount(record.slashing_penalty),
                source: record.source.to_string(),
            })
            .collect(),
        delegations: local_generator_store
            .get_delegations(
                &generator_id,
                vec![Operation::Delegate, Operation::UnDelegate],
                None,
                None,
            )
            .iter()
            .map(|element| DelegateOperation {
                delegation: address_token_pair_to_token_amount(element.delegation),
                source: element.source.to_string(),
                operation: element.operation.to_string(),
                block_number: element.block_number.to_string(),
                transaction_index: element.transaction_index.to_string(),
                log_index: element.log_index.to_string(),
                tx: element.tx.clone(),
            })
            .collect(),
        my_delegations: {
            let mut token_tracker = TokenTracker::new();

            local_generator_store
                .get_delegations(
                    &generator_id,
                    vec![Operation::Delegate, Operation::UnDelegate],
                    None,
                    Some(usize::MAX),
                )
                .into_iter()
                .filter(|delegation| delegation.source == Source::Native)
                .for_each(|delegation| {
                    token_tracker.add_token(&delegation.delegation.0, &delegation.delegation.1)
                });

            token_tracker.to_token_amount()
        },
        withdrawal_requests: local_generator_store
            .get_withdrawl_requests(&generator_id)
            .iter()
            .map(|a| WithdrawRequest {
                account: address_to_string(&a.account),
                index: a.index.to_string(),
                token: address_to_string(&a.token),
                amount: a.amount.to_string(),
                withdrawal_timestamp: a.timestamp.to_string(),
            })
            .collect(),
        compute_break_down: ComputeBreakDown {
            total_compute: generator_data.declared_compute.to_string(),
            compute_locked: generator_data.compute_consumed.to_string(),
            compute_available: (generator_data
                .declared_compute
                .saturating_sub(generator_data.compute_consumed))
            .to_string(),
        },
    })
}
