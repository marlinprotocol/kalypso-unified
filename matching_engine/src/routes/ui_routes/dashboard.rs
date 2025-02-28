use super::cache::CachedResponse;
use super::single_generator::StakeBreakDown;
use crate::ask_lib::ask_store::{
    AskManagementRead, CompletedProofsManagement, ProofCounters, RequestorCounters,
    TimingOperations,
};

use crate::generator_lib::native_stake_store::NativeStakingOperations;
use crate::generator_lib::symbiotic_stake_store::TokenLockManagement;
use crate::generator_lib::traits::{GeneratorAdditionalQuery, GeneratorRegistration};
use crate::market_metadata::MarketMetadataStoreRead;
use crate::models::WelcomeResponse;
use crate::utility::{
    address_to_string, bytes_to_string, convert_to_option_string, tx_to_string, TokenAmount,
    USDC_TOKEN,
};
use crate::{try_read_and_get_if_valid, try_read_or_lock};
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::types::U256;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::time::Duration;
use utoipa::ToSchema;

const DEFAULT_COUNT: &usize = &100;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct DashboardResponse {
    /// The total number of markets created.
    markets_created: usize,

    /// The total number of registered generators.
    registered_generators: usize,

    /// The total number of proofs generated.
    proofs_generated: usize,

    /// The total number of unique requestors.
    unique_requestors: usize,

    /// A list of markets.
    markets: Vec<Market>,

    /// A list of recent proofs.
    recent_proofs: Vec<RecentProof>,

    /// The task assignment requirements.
    task_assignment_requirements: TokenList,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct TokenList {
    pub native: Vec<TokenAmount>,
    pub symbiotic: Vec<TokenAmount>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct Market {
    /// Market Id
    id: String,
    /// Name of the market. It is null if market maker has not provided
    name: Option<String>,
    /// deprecated. not use this field
    token: String,
    /// median time take to generate proof in the market
    median_time: String,
    /// median settlement price in this market
    median_cost: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct RecentProof {
    /// ID of the job
    id: String,
    /// Market which the proof belongs to
    market: Market,
    /// Requestor
    requestor: String,
    /// Public inputs of the proof-job
    inputs: String,
    /// Information of the proof generator
    generator: Generator,
    /// Deprecated: Time taken in seconds to generate the proof
    time: String,
    /// Deprecated: Cost Incurred in generating the proof
    cost: String,
    /// Input Transaction Hash
    inputs_transaction: String,
    /// Proof Transaction Hash
    proof_transaction: String,
    /// decprecated: Proof Transaction Hash (use proof_transaction instead)
    proof_link: String,
    /// Timestamp on which job was created
    created_on_timestamp: Option<String>,
    /// Timestamp on which job was matched with generator
    matched_on_timestamp: Option<String>,
    /// Timestamp on which job's proof was submitted by geneator
    proof_created_on_timestamp: Option<String>,
    /// Price Quoted by Proof Requestor
    quote: TokenAmount,
    /// Price for which the job was settled
    settlement: Option<TokenAmount>,
    /// deprecated: Price for which the job was settled (use settlement)
    solved_in: Option<TokenAmount>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct Generator {
    /// Name of the generator
    name: Option<String>,
    /// Address of the generator
    address: String,

    /// stake break down
    stake_break_down: Option<StakeBreakDown>,
}

type CachedDashboardResponse = CachedResponse<DashboardResponse>;

use once_cell::sync::Lazy;

// Define a global instance of CachedDashboardResponse
static DASHBOARD_RESPONSE: Lazy<RwLock<CachedDashboardResponse>> =
    Lazy::new(|| RwLock::new(CachedDashboardResponse::new()));

#[utoipa::path(
    get,
    path = "/ui/dashboard",
    responses(
        (status = 200, description = "Dashboard Response", body = DashboardResponse),
        (status = 423, description = "Resource Locked", body = WelcomeResponse),
    ),
    tag = "UI"
)]
pub async fn get_dashboard<
    MS: MarketMetadataStoreRead + Send + Sync,
    AS: AskManagementRead
        + CompletedProofsManagement
        + ProofCounters
        + RequestorCounters
        + TimingOperations
        + Send
        + Sync,
    GS: GeneratorAdditionalQuery + GeneratorRegistration + Send + Sync,
    NS: NativeStakingOperations + Send + Sync,
    SS: TokenLockManagement + Send + Sync,
>(
    _local_market_store: Data<Arc<RwLock<MS>>>,
    _local_ask_store: Data<Arc<RwLock<AS>>>,
    _local_generator_store: Data<Arc<RwLock<GS>>>,
    _local_native_store: Data<Arc<RwLock<NS>>>,
    _local_symbiotic_store: Data<Arc<RwLock<SS>>>,
) -> actix_web::Result<HttpResponse> {
    try_read_and_get_if_valid!(
        DASHBOARD_RESPONSE,
        dashboard_cache,
        Duration::from_millis(100)
    );
    drop(dashboard_cache);

    try_read_or_lock!(_local_ask_store, local_ask_store);
    try_read_or_lock!(_local_market_store, local_market_store);
    try_read_or_lock!(_local_generator_store, local_generator_store);
    try_read_or_lock!(_local_native_store, local_native_store);
    try_read_or_lock!(_local_symbiotic_store, local_symbiotic_store);

    // Step 2: If the cache is invalid, recompute the response (write lock)
    let new_response = recompute_dashboard_response(
        local_market_store,
        local_ask_store,
        local_generator_store,
        local_native_store,
        local_symbiotic_store,
    )
    .await;

    {
        // Store the newly computed response in the cache
        match DASHBOARD_RESPONSE.try_write() {
            Ok(mut cache_write_lock) => {
                cache_write_lock.store(new_response.clone());
            }
            _ => {
                log::warn!("Failed Caching Dashboard response");
            }
        };
    }

    // Return the newly computed response
    return Ok(HttpResponse::Ok().json(new_response));
}

async fn recompute_dashboard_response<
    'a,
    MS: MarketMetadataStoreRead,
    AS: AskManagementRead
        + CompletedProofsManagement
        + ProofCounters
        + RequestorCounters
        + TimingOperations,
    GS: GeneratorAdditionalQuery + GeneratorRegistration,
    NS: NativeStakingOperations,
    SS: TokenLockManagement,
>(
    local_market_store: RwLockReadGuard<'a, MS>,
    local_ask_store: RwLockReadGuard<'a, AS>,
    local_generator_store: RwLockReadGuard<'a, GS>,
    local_native_store: RwLockReadGuard<'a, NS>,
    local_symbiotic_store: RwLockReadGuard<'a, SS>,
) -> DashboardResponse {
    // Step 1: Retrieve all market metadata and count of markets
    let (all_markets, count_markets, market_median_map) = {
        let all_markets = local_market_store.get_all_markets().clone(); // Clone to release the lock early
        let count_markets = local_market_store.count_markets();

        // Create a map of market_id to (median_time, median_cost)
        let mut market_median_map = std::collections::HashMap::new();
        for meta in &all_markets {
            let market_id = &meta.market_id;
            let median_time = local_market_store
                .get_median_proof_time_market_wise(market_id)
                .to_string();
            let median_cost = local_market_store
                .get_median_proof_cost_market_wise(market_id)
                .to_string();
            market_median_map.insert(market_id.clone(), (median_time, median_cost));
        }

        (all_markets, count_markets, market_median_map)
    };

    // Step 2: Construct the `markets` vector
    let mut markets = Vec::with_capacity(all_markets.len());
    for meta in &all_markets {
        let market_id = &meta.market_id;
        let name = &meta.deserialize_market_bytes().zk_app_name;
        if let Some((median_time, median_cost)) = market_median_map.get(market_id) {
            let market = Market {
                id: market_id.to_string(),
                name: name.clone(),
                token: "USDC".into(),
                median_time: median_time.clone(),
                median_cost: median_cost.clone(),
            };
            markets.push(market);
        } else {
            // Handle cases where median data might be missing
            markets.push(Market {
                id: market_id.to_string(),
                name: name.clone(),
                token: "USDC".into(),
                median_time: "0".into(),
                median_cost: "0".into(),
            });
        }
    }

    // Step 3: Retrieve recent completed proofs and total proof count
    let (recent_completed_proofs, total_proof_count) = {
        let recent_completed_proofs = local_ask_store
            .get_recent_completed_proofs(*DEFAULT_COUNT)
            .clone(); // Clone to release the lock
        let total_proof_count = local_ask_store.get_total_proof_count();
        (recent_completed_proofs, total_proof_count)
    };

    // Step 4: Construct the `recent_proofs` vector
    let mut recent_proofs = Vec::with_capacity(recent_completed_proofs.len());
    for ask_request in recent_completed_proofs {
        let market_id = &ask_request.market_id;

        // Retrieve median data from the precomputed map
        let (median_time, median_cost) = market_median_map
            .get(market_id)
            .map(|(t, c)| (t.clone(), c.clone()))
            .unwrap_or(("0".to_string(), "0".to_string())); // Default values if not found

        let market = Market {
            id: market_id.to_string(),
            name: {
                let result = local_market_store.get_market_by_market_id(market_id);
                if result.is_none() {
                    None
                } else {
                    result.unwrap().deserialize_market_bytes().zk_app_name
                }
            },
            token: "USDC".into(),
            median_time,
            median_cost,
        };

        // Retrieve proof details
        let (time, cost, proof_link) = {
            (
                local_ask_store
                    .get_job_completed_on_timestamp(&ask_request.ask_id)
                    .and_then(|completed| {
                        local_ask_store
                            .get_job_matched_on_timestamp(&ask_request.ask_id)
                            .map(|matched| completed - matched)
                    })
                    .unwrap_or(0.into()),
                local_ask_store
                    .get_proving_cost(&ask_request.ask_id)
                    .unwrap_or(U256::zero())
                    .to_string(),
                local_ask_store
                    .get_proof_transaction(&ask_request.ask_id)
                    .unwrap_or_default(),
            )
        };

        // Assume that `ask_request.generator` is `Some`, handle `None` if necessary
        let generator_address = match ask_request.generator {
            Some(addr) => address_to_string(&addr),
            None => "Unknown".into(), // Default or handle appropriately
        };

        let proof = RecentProof {
            id: ask_request.ask_id.to_string(),
            market,
            requestor: address_to_string(&ask_request.prover_refund_address),
            inputs: bytes_to_string(&ask_request.prover_data),
            generator: Generator {
                name: ask_request.generator.and_then(|addr| {
                    local_generator_store
                        .get_by_address(&addr)
                        .and_then(|g| g.deserialize_generator_bytes().display_name)
                }),
                address: generator_address,
                stake_break_down: {
                    ask_request.generator.and_then(|addr| {
                        local_generator_store
                            .get_by_address(&addr)
                            .map(|generator_data| StakeBreakDown {
                                total_native_stake: generator_data
                                    .clone()
                                    .total_native_stake
                                    .to_token_amount(),
                                total_native_stake_locked: generator_data
                                    .native_stake_locked
                                    .to_token_amount(),
                                total_symbiotic_stake: generator_data
                                    .clone()
                                    .total_symbiotic_stake
                                    .to_token_amount(),
                                total_symbiotic_stake_locked: generator_data
                                    .symbiotic_stake_locked
                                    .to_token_amount(),
                                available_native_stake: (generator_data.total_native_stake
                                    - generator_data.native_stake_locked)
                                    .to_token_amount(),
                                available_symbiotic_stake: (generator_data.total_symbiotic_stake
                                    - generator_data.symbiotic_stake_locked)
                                    .to_token_amount(),
                            })
                    })
                },
            },
            time: time.to_string(),
            cost,
            inputs_transaction: tx_to_string(&ask_request.create_transaction),
            proof_link: proof_link.clone(),
            proof_transaction: proof_link,
            created_on_timestamp: convert_to_option_string(
                local_ask_store.get_job_created_on_timestamp(&ask_request.ask_id),
            ),
            matched_on_timestamp: convert_to_option_string(
                local_ask_store.get_job_matched_on_timestamp(&ask_request.ask_id),
            ),
            proof_created_on_timestamp: convert_to_option_string(
                local_ask_store.get_job_completed_on_timestamp(&ask_request.ask_id),
            ),
            quote: TokenAmount {
                token: address_to_string(&USDC_TOKEN),
                amount: ask_request.reward.to_string(),
            },
            solved_in: Some(TokenAmount {
                token: address_to_string(&USDC_TOKEN),
                amount: local_ask_store
                    .get_proving_cost(&ask_request.ask_id)
                    .unwrap_or_default()
                    .to_string(),
            }),
            settlement: Some(TokenAmount {
                token: address_to_string(&USDC_TOKEN),
                amount: local_ask_store
                    .get_proving_cost(&ask_request.ask_id)
                    .unwrap_or_default()
                    .to_string(),
            }),
        };

        recent_proofs.push(proof);
    }

    // Step 5: Retrieve the count of registered generators
    let registered_generators = { local_generator_store.all_generators_address().len() };

    // Step 6: Assemble the final `DashboardResponse`
    DashboardResponse {
        markets_created: count_markets,
        registered_generators,
        proofs_generated: total_proof_count,
        unique_requestors: local_ask_store.total_requestor_count(),
        markets,
        recent_proofs,
        task_assignment_requirements: TokenList {
            native: local_native_store.tokens_to_lock().await.to_token_amount(),
            symbiotic: local_symbiotic_store.tokens_to_lock().to_token_amount(),
        },
    }
}
