// Local crate imports
use crate::{
    ask_lib::{
        ask::LocalAsk,
        ask_status::AskState,
        ask_store::{AskManagementRead, CompletedProofsManagement, TimingOperations},
    },
    generator_lib::traits::JobMissedCounter,
    utility::USDC_TOKEN_STRING,
};

use crate::generator_lib::{
    delegation::{Operation, Source},
    generator_store::GeneratorMeta,
    key_store::{KeyInfo, KeyStoreOperations},
    native_stake_store::NativeStakingOperations,
    symbiotic_stake_store::TokenLockManagement,
    traits::{
        GeneratorAdditionalQuery, GeneratorAvailability, GeneratorEarningsAndSlashing,
        GeneratorRegistration, WithdrawalManagement,
    },
};

use crate::market_metadata::MarketMetadataStoreRead;
use crate::models::WelcomeResponse;
use crate::try_read_or_lock;
use crate::utility::{
    address_to_string, address_token_pair_to_token_amount, bytes_to_string,
    convert_to_option_string, tx_to_string, TokenAmount, TokenTracker, USDC_TOKEN,
};

use super::cache::CachedResponse;

// External crate imports
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use ethers::types::{Address, U256};
use im::HashMap;
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::time::Duration;
use utoipa::{IntoParams, ToSchema};

type CachedSingleGeneratorResponse = CachedResponse<GeneratorResponse>;

const DEFAULT_COUNT: usize = 100;

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

#[derive(Deserialize, Clone, Copy, Serialize, Debug, Hash, Eq, PartialEq, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct QueryParams {
    /// Number of active jobs to skip
    active_jobs_skip: Option<usize>,

    /// Number of active jobs to return
    active_jobs: Option<usize>,

    /// Number of completed jobs to skip
    completed_jobs_skip: Option<usize>,

    /// Number of completed jobs to return
    completed_jobs: Option<usize>,

    /// Number of slashing history to skip
    slashing_history_skip: Option<usize>,

    /// Number of slashing history to return
    slashing_history: Option<usize>,

    /// Number of delegation to skip
    delegation_skip: Option<usize>,

    /// Number of delegation to return
    delegation_count: Option<usize>,

    /// Number of withdrawal to skip
    withdrawal_skip: Option<usize>,

    /// Number of withdrawal to return
    withdrawal_count: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct GeneratorResponse {
    /// deprecated: Operator Details (use .details instead)
    operator: Operator,

    /// Generator Details
    details: GeneratorMeta,

    /// Reward Address of the generator
    reward_address: String,

    /// deprecated: Kalypso Points (instead fetch from points subgraph directly)
    kalypso_points: String,

    /// Active jobs of the generator
    active_jobs: String,

    /// Jobs Missed
    jobs_missed: String,

    /// Number of markets Generator has participated in
    no_of_markets: String,

    /// Total earnings of the generator
    total_earnings: String,

    /// deprecated: Total Slashed (no slashing exists as of now)
    total_slashed: Vec<TokenAmount>,

    /// deprecated: Total Delegations (use .stake_break_down)
    total_delegations: Vec<TokenAmount>,

    /// Details of the market generator is participating in
    markets: Vec<Market>,

    /// Active Jobs List
    active_jobs_list: Vec<Job>,

    /// Completed Jobs List
    completed_jobs_list: Vec<Job>,

    /// deprecated: Slashing History (no slashing exists as of now)
    slashing_history: Vec<Slash>,

    /// deprecated: Available Stake (use .stake_break_down)
    available_stake: Vec<TokenAmount>,

    /// deprecated: Stake Locked (use .stake_break_down)
    stake_locked: Vec<TokenAmount>,

    /// Transaction via which the generator has received delegations (including stake and delegatation)
    delegations: Vec<DelegateOperation>,

    /// deprecated: My Delegations (use .stake_break_down)
    my_delegations: Vec<TokenAmount>,

    /// deprecated: Withdrawal Requests (use .withdrawal_requests)
    withdrawal_requests: Vec<WithdrawRequest>,

    /// Stake Break Down
    stake_break_down: StakeBreakDown,

    /// Compute Break Down
    compute_break_down: ComputeBreakDown,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct StakeBreakDown {
    /// Total Native Stake
    pub total_native_stake: Vec<TokenAmount>,

    /// Total Native Stake Locked
    pub total_native_stake_locked: Vec<TokenAmount>,

    /// Total Symbiotic Stake
    pub total_symbiotic_stake: Vec<TokenAmount>,

    /// Total Symbiotic Stake Locked
    pub total_symbiotic_stake_locked: Vec<TokenAmount>,

    /// Available Native Stake (i.e total_native_stake - total_native_stake_locked)
    pub available_native_stake: Vec<TokenAmount>,

    /// Available Symbiotic Stake (i.e total_symbiotic_stake - total_symbiotic_stake_locked)
    pub available_symbiotic_stake: Vec<TokenAmount>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct ComputeBreakDown {
    /// Total Compute declared by the generator
    pub total_compute: String,

    /// Compute Locked for the generator
    pub compute_locked: String,

    /// Compute Available for the generator (i.e total_compute - compute_locked)
    pub compute_available: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct WithdrawRequest {
    /// Account Address
    account: String,

    /// Token Address for withdrawal
    token: String,

    /// Amount to withdraw
    amount: String,

    /// Withdrawal Timestamp
    index: String,

    /// Timestamp at which withdrawal will be processed
    withdrawal_timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct DelegateOperation {
    /// Delegation Amount
    delegation: TokenAmount,

    /// Source of the delegation (Native or Symbiotic)
    source: String,

    /// Operation (Delegate or UnDelegate)
    operation: String,

    /// Block Number
    block_number: String,

    /// Transaction Index
    transaction_index: String,

    /// Log Index
    log_index: String,

    /// Transaction Hash
    tx: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct Slash {
    /// Bid ID
    ask_id: Option<String>,

    /// deprecated: Slashing Epoch Timestamp
    timestamp: String,

    /// Market Info
    market: MarketInfo,

    /// Price Offered
    price_offered: TokenAmount,

    /// Slashing Penalty (no penalty exists as of now)
    slashing_penalty: TokenAmount,

    /// deprecated: Slashing Epoch Timestamp
    slasing_epoch_timestamp: Option<String>,

    /// Source of the slashing (Native or Symbiotic) (no slashing exists as of now)
    source: String,

    /// Transaction hash of the slashing transaction
    slashing_transaction_hash: String,

    /// Transactino hash of the inputs
    inputs_transaction: String,

    /// Requested Proving time
    time_requested_for_proof_generation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct Job {
    /// Bid ID
    ask_id: String,

    /// Market Info
    market: MarketInfo,

    /// Requestor Address
    requestor: String,

    /// Inputs
    inputs: String,

    /// Transaction Hash
    inputs_transaction: String,

    ///  Deadline by which the job should be completed by the generator
    deadline: String,

    /// deprecated: Don't use it (use .quote instead)
    cost: String,

    /// Time taken for proof generation (will be null if proof is not generated yet)
    time_taken_for_proof_generation: Option<String>,

    /// Proof (will be null if proof is not generated yet)
    proof: Option<String>,

    /// Proof Transaction (will be null if proof is not generated yet)
    proof_transaction: Option<String>,

    /// Quote (by requestor)
    quote: TokenAmount,

    /// deprecated: Solved In (use .settlement instead)
    solved_in: Option<TokenAmount>,

    /// deprecated: Settlement Price (use .settlement instead)
    settlement: Option<TokenAmount>,

    /// timestamp of which the job was created
    created_on_timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct MarketInfo {
    /// Market Name
    name: Option<String>,

    /// Market ID
    id: String,
    /// deprecated: (use task_assignement_requirement which is global)
    token: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct Market {
    /// Market Name
    name: Option<String>,

    /// Market ID
    id: String,

    /// Earnings to date
    earnings_to_date: String,

    /// Proofs Missed
    proofs_missed: String,

    /// Proofs Generated
    proofs_generated: String,

    /// Pending Proofs
    pending_proofs: String,

    /// deprecated: Slashing Penalties Incured (no slashing exists as of now)
    slashing_penalties_incured: String,

    /// Min Hardware Requirement (if null means not provided by market maker)
    min_hardware_requirement: Option<MinHardware>,

    /// Enclave key. A null value indicates that the operator hasn't set an enclave key for this market or that the market does not require one.
    enclave_key: Option<KeyInfo>,

    /// deprecated: Kalypso Points (instead fetch from points subgraph directly)
    kalypso_points: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct MinHardware {
    /// Instance Type (if null means not provided by market maker)
    instance_type: Option<String>,
    /// Number of vCPUs (if null means not provided by market maker)
    vcpus: Option<usize>,
    /// Number of vGPUs (if null means not provided by market maker)
    vgpus: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct Operator {
    /// Operator Name
    name: Option<String>,
    /// Operator Address
    address: String,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct GeneratorQuery {
    generator: Address,
    query: QueryParams,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct WithdrawalResponse {
    /// Withdrawal Requests
    withdrawal_requests: Vec<WithdrawRequest>,
}

#[utoipa::path(
    get,
    path = "/ui/withdrawals/{id}",
    responses(
        (status = 200, description = "Returns Withdrawal Info of the operator", body = WithdrawalResponse),
        (status = 423, description = "Parsing in progress" )
    ),
    params(
        ("id" = String, Path, description = "Operator/Generator Address"),
    ),
    tag = "UI"
)]
pub async fn withdrawal_request<
    GS: GeneratorAdditionalQuery
        + GeneratorRegistration
        + GeneratorAvailability
        + GeneratorEarningsAndSlashing
        + WithdrawalManagement
        + Send
        + Sync,
>(
    _local_generator_store: Data<Arc<RwLock<GS>>>,
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

    let withdrawal_requests = local_generator_store
        .get_withdrawl_requests(&generator_id)
        .iter()
        .skip(0)
        .take(DEFAULT_COUNT)
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

#[utoipa::path(
    get,
    path = "/ui/generator/{id}",
    responses(
        (status = 200, description = "Return Operator Details", body = GeneratorResponse),
        (status = 423, description = "Parsing in progress" )
    ),
    params(
        ("id" = String, Path, description = "Operator/Generator Address"),
        QueryParams
    ),
    tag = "UI"
)]
pub async fn single_generator<
    MS: MarketMetadataStoreRead + Send + Sync,
    AS: AskManagementRead + CompletedProofsManagement + TimingOperations + Send + Sync,
    GS: GeneratorAdditionalQuery
        + GeneratorRegistration
        + GeneratorAvailability
        + GeneratorEarningsAndSlashing
        + JobMissedCounter
        + WithdrawalManagement
        + Send
        + Sync,
    NS: NativeStakingOperations + Send + Sync,
    SS: TokenLockManagement + Send + Sync,
    KS: KeyStoreOperations + Send + Sync,
>(
    _local_ask_store: Data<Arc<RwLock<AS>>>,
    _local_generator_store: Data<Arc<RwLock<GS>>>,
    _local_key_store: Data<Arc<RwLock<KS>>>,
    _local_native_store: Data<Arc<RwLock<NS>>>,
    _local_symbiotic_store: Data<Arc<RwLock<SS>>>,
    _local_market_store: Data<Arc<RwLock<MS>>>,

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
            delegation_skip: query.delegation_skip,
            delegation_count: query.delegation_count,
            withdrawal_skip: query.withdrawal_skip,
            withdrawal_count: query.withdrawal_count,
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

async fn recompute_single_generator_response<
    'a,
    MS: MarketMetadataStoreRead,
    AS: AskManagementRead + CompletedProofsManagement + TimingOperations,
    GS: GeneratorAdditionalQuery
        + GeneratorRegistration
        + GeneratorAvailability
        + GeneratorEarningsAndSlashing
        + JobMissedCounter
        + WithdrawalManagement,
    NS: NativeStakingOperations,
    SS: TokenLockManagement,
    KS: KeyStoreOperations,
>(
    generator_id: Address,
    query: GeneratorQuery,
    local_ask_store: RwLockReadGuard<'a, AS>,
    local_generator_store: RwLockReadGuard<'a, GS>,
    local_key_store: RwLockReadGuard<'a, KS>,
    local_native_store: RwLockReadGuard<'a, NS>,
    local_symbiotic_store: RwLockReadGuard<'a, SS>,
    local_market_store: RwLockReadGuard<'a, MS>,
) -> Option<GeneratorResponse> {
    let generator_data = local_generator_store.get_by_address(&generator_id);

    if generator_data.is_none() {
        return None;
    }

    let generator_data = generator_data.unwrap();
    let all_markets_of_generator =
        local_generator_store.get_all_markets_of_generator(&generator_id);

    let (all_tokens_supported, _): (Vec<Address>, Vec<U256>) =
        (local_native_store.tokens_to_lock().await.clone()
            + local_symbiotic_store.tokens_to_lock().clone())
        .to_address_token_pair()
        .into_iter()
        .unzip();

    let details = generator_data.deserialize_generator_bytes();

    let jobs_missed = all_markets_of_generator
        .clone()
        .into_iter()
        .map(|info| info.proofs_slashed)
        .fold(U256::zero(), |a, x| a + x)
        .to_string();

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
        jobs_missed,
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
                name: local_market_store
                    .get_market_by_market_id(&info.market_id)
                    .and_then(|a| a.deserialize_market_bytes().zk_app_name),
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
                enclave_key: {
                    let result =
                        local_key_store.get_by_address(&info.address, info.market_id.as_u64());
                    if result.is_some() {
                        Some(result.unwrap().to_key_info())
                    } else {
                        None
                    }
                },
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
                    .take(query.query.active_jobs.unwrap_or_else(|| DEFAULT_COUNT))
                    .collect::<Vec<LocalAsk>>();

                local_asks
                    .into_iter()
                    .map(|a| Job {
                        ask_id: a.ask_id.to_string(),
                        market: MarketInfo {
                            name: local_market_store
                                .get_market_by_market_id(&a.market_id)
                                .and_then(|a| a.deserialize_market_bytes().zk_app_name),
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
                        quote: TokenAmount {
                            token: address_to_string(&USDC_TOKEN),
                            amount: a.reward.to_string(),
                        },
                        solved_in: None,
                        settlement: None,
                        created_on_timestamp: convert_to_option_string(
                            local_ask_store.get_job_created_on_timestamp(&a.ask_id),
                        ),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        completed_jobs_list: local_ask_store
            .get_completed_proof_of_generator(
                &generator_id,
                query.query.completed_jobs_skip.unwrap_or_default(),
                query.query.completed_jobs.unwrap_or_else(|| DEFAULT_COUNT),
            )
            .into_iter()
            .take(DEFAULT_COUNT)
            .map(|ask| Job {
                ask_id: ask.ask_id.to_string(),
                market: MarketInfo {
                    name: local_market_store
                        .get_market_by_market_id(&ask.market_id)
                        .and_then(|a| a.deserialize_market_bytes().zk_app_name),
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
                quote: TokenAmount {
                    token: address_to_string(&USDC_TOKEN),
                    amount: ask.reward.to_string(),
                },
                solved_in: Some(TokenAmount {
                    token: address_to_string(&USDC_TOKEN),
                    amount: local_ask_store
                        .get_proving_cost(&ask.ask_id)
                        .unwrap_or_default()
                        .to_string(),
                }),
                settlement: Some(TokenAmount {
                    token: address_to_string(&USDC_TOKEN),
                    amount: local_ask_store
                        .get_proving_cost(&ask.ask_id)
                        .unwrap_or_default()
                        .to_string(),
                }),
                created_on_timestamp: convert_to_option_string(
                    local_ask_store.get_job_created_on_timestamp(&ask.ask_id),
                ),
            })
            .collect::<Vec<Job>>(),
        slashing_history: local_generator_store
            .get_slashing_records(&generator_id)
            .into_iter()
            .skip(query.query.slashing_history_skip.unwrap_or_default())
            .take(
                query
                    .query
                    .slashing_history
                    .unwrap_or_else(|| DEFAULT_COUNT),
            )
            .map(|record| Slash {
                ask_id: Some(record.ask_id.to_string()),
                slasing_epoch_timestamp: convert_to_option_string(Some(record.slashing_timestamp)),
                timestamp: record.slashing_block_number.to_string(),
                market: MarketInfo {
                    name: local_market_store
                        .get_market_by_market_id(&record.market_id)
                        .and_then(|a| a.deserialize_market_bytes().zk_app_name),
                    id: record.market_id.to_string(),
                    token: all_tokens_supported
                        .iter()
                        .map(|a| address_to_string(a))
                        .collect::<Vec<String>>(),
                },
                slashing_transaction_hash: record.slashing_tx,
                price_offered: TokenAmount {
                    token: USDC_TOKEN_STRING.to_string(),
                    amount: record.price_offered.to_string(),
                },
                slashing_penalty: address_token_pair_to_token_amount(record.slashing_penalty),
                source: record.source.to_string(),
                inputs_transaction: "0xabcd".to_string(),
                time_requested_for_proof_generation: record.expected_time.to_string(),
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
            .skip(query.query.delegation_skip.unwrap_or_default())
            .take(
                query
                    .query
                    .delegation_count
                    .unwrap_or_else(|| DEFAULT_COUNT),
            )
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
            .skip(query.query.withdrawal_skip.unwrap_or_default())
            .take(
                query
                    .query
                    .withdrawal_count
                    .unwrap_or_else(|| DEFAULT_COUNT),
            )
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
