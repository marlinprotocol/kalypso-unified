use super::cache::CachedResponse;
use crate::generator_lib::native_stake_store::NativeStakingStore;
use crate::generator_lib::symbiotic_stake_store::SymbioticStakeStore;
use crate::models::WelcomeResponse;
use crate::utility::{
    address_to_string, bytes_to_string, convert_to_option_string, tx_to_string, TokenAmount,
};
use crate::{
    ask_lib::ask_store::LocalAskStore, generator_lib::generator_store::GeneratorStore,
    market_metadata::MarketMetadataStore,
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

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct DashboardResponse {
    markets_created: usize,
    registered_generators: usize,
    proofs_generated: usize,
    markets: Vec<Market>,
    recent_proofs: Vec<RecentProof>,
    task_assignment_requirements: TaskRequirements,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct TaskRequirements {
    native: Vec<TokenAmount>,
    symbiotic: Vec<TokenAmount>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct Market {
    id: String,
    name: Option<String>,
    token: String,
    median_time: String,
    median_cost: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct RecentProof {
    market: Market,
    requestor: String,
    inputs: String,
    generator: Generator,
    time: String,
    cost: String,
    inputs_transaction: String,
    proof_link: String,
    created_on_timestamp: Option<String>,
    matched_on_timestamp: Option<String>,
    proof_created_on_timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
struct Generator {
    name: Option<String>,
    address: String,
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
pub async fn get_dashboard(
    _local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    _local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
    _local_native_store: Data<Arc<RwLock<NativeStakingStore>>>,
    _local_symbiotic_store: Data<Arc<RwLock<SymbioticStakeStore>>>,
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

async fn recompute_dashboard_response<'a>(
    local_market_store: RwLockReadGuard<'a, MarketMetadataStore>,
    local_ask_store: RwLockReadGuard<'a, LocalAskStore>,
    local_generator_store: RwLockReadGuard<'a, GeneratorStore>,
    local_native_store: RwLockReadGuard<'a, NativeStakingStore>,
    local_symbiotic_store: RwLockReadGuard<'a, SymbioticStakeStore>,
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
        let recent_completed_proofs = local_ask_store.get_recent_completed_proofs(20).clone(); // Clone to release the lock
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
                    .get_proving_time(&ask_request.ask_id)
                    .unwrap_or(U256::zero())
                    .to_string(),
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
            },
            time,
            cost,
            inputs_transaction: tx_to_string(&ask_request.create_transaction),
            proof_link,
            created_on_timestamp: convert_to_option_string(
                local_ask_store.get_job_created_on_timestamp(&ask_request.ask_id),
            ),
            matched_on_timestamp: convert_to_option_string(
                local_ask_store.get_job_matched_on_timestamp(&ask_request.ask_id),
            ),
            proof_created_on_timestamp: convert_to_option_string(
                local_ask_store.get_job_completed_on_timestamp(&ask_request.ask_id),
            ),
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
        markets,
        recent_proofs,
        task_assignment_requirements: TaskRequirements {
            native: local_native_store.tokens_to_lock.to_token_amount(),
            symbiotic: local_symbiotic_store.tokens_to_lock.to_token_amount(),
        },
    }
}
