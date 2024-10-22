use super::cache::CachedResponse;
use crate::utility::{address_to_string, bytes_to_string};
use crate::{
    ask_lib::ask_store::LocalAskStore, generator_lib::generator_store::GeneratorStore,
    market_metadata::MarketMetadataStore,
};
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::types::U256;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DashboardResponse {
    markets_created: usize,
    registered_generators: usize,
    proofs_generated: usize,
    markets: Vec<Market>,
    recent_proofs: Vec<RecentProof>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Market {
    name: String,
    token: String,
    median_time: String,
    median_cost: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RecentProof {
    market: Market,
    requestor: String,
    inputs: String,
    generator: Generator,
    time: String,
    cost: String,
    proof_link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Generator {
    name: Option<String>,
    address: String,
}

type CachedDashboardResponse = CachedResponse<DashboardResponse>;

use once_cell::sync::Lazy;

// Define a global instance of CachedDashboardResponse
static DASHBOARD_RESPONSE: Lazy<RwLock<CachedDashboardResponse>> =
    Lazy::new(|| RwLock::new(CachedDashboardResponse::new()));

pub async fn get_dashboard(
    _local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    _local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
) -> actix_web::Result<HttpResponse> {
    // Step 1: Check if there's a cached response (read lock)
    let cache_read_lock = DASHBOARD_RESPONSE.read().await;

    if let Some(response) = cache_read_lock.get_if_valid(Duration::from_secs(5)) {
        // Return the cached response if valid
        return Ok(HttpResponse::Ok().json(response));
    }
    drop(cache_read_lock); // Drop the read lock to allow write lock acquisition

    // Step 2: If the cache is invalid, recompute the response (write lock)
    let mut cache_write_lock = DASHBOARD_RESPONSE.write().await;
    let new_response = recompute_dashboard_response(
        _local_market_store,
        _local_ask_store,
        _local_generator_store,
    )
    .await;

    // Store the newly computed response in the cache
    cache_write_lock.store(new_response.clone());

    // Return the newly computed response
    Ok(HttpResponse::Ok().json(new_response))
}

async fn recompute_dashboard_response(
    local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,
    local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
) -> DashboardResponse {
    // Step 1: Retrieve all market metadata and count of markets
    let (all_markets, count_markets, market_median_map) = {
        let store = local_market_store.read().await;
        let all_markets = store.get_all_markets().clone(); // Clone to release the lock early
        let count_markets = store.count_markets();

        // Create a map of market_id to (median_time, median_cost)
        let mut market_median_map = std::collections::HashMap::new();
        for meta in &all_markets {
            let market_id = &meta.market_id;
            let median_time = store
                .get_median_proof_time_market_wise(market_id)
                .to_string();
            let median_cost = store
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
        if let Some((median_time, median_cost)) = market_median_map.get(market_id) {
            let market = Market {
                name: market_id.to_string(),
                token: "USDC".into(),
                median_time: median_time.clone(),
                median_cost: median_cost.clone(),
            };
            markets.push(market);
        } else {
            // Handle cases where median data might be missing
            markets.push(Market {
                name: market_id.to_string(),
                token: "USDC".into(),
                median_time: "0".into(),
                median_cost: "0".into(),
            });
        }
    }

    // Step 3: Retrieve recent completed proofs and total proof count
    let (recent_completed_proofs, total_proof_count) = {
        let store = local_ask_store.read().await;
        let recent_completed_proofs = store.get_recent_completed_proofs(20).clone(); // Clone to release the lock
        let total_proof_count = store.get_total_proof_count();
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
            name: market_id.to_string(),
            token: "USDC".into(),
            median_time,
            median_cost,
        };

        // Retrieve proof details
        let (time, cost, proof_link) = {
            let store = local_ask_store.read().await;
            (
                store
                    .get_proving_time(&ask_request.ask_id)
                    .unwrap_or(U256::zero())
                    .to_string(),
                store
                    .get_proving_cost(&ask_request.ask_id)
                    .unwrap_or(U256::zero())
                    .to_string(),
                store
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
                name: None, // Assuming no name is available
                address: generator_address,
            },
            time,
            cost,
            proof_link,
        };

        recent_proofs.push(proof);
    }

    // Step 5: Retrieve the count of registered generators
    let registered_generators = {
        let store = local_generator_store.read().await;
        store.all_generators_address().len()
    };

    // Step 6: Assemble the final `DashboardResponse`
    DashboardResponse {
        markets_created: count_markets,
        registered_generators,
        proofs_generated: total_proof_count,
        markets,
        recent_proofs,
    }
}
