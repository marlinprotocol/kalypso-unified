use super::cache::CachedResponse;
use crate::ask_lib::ask_status::AskState;
use crate::{ask_lib::ask_store::LocalAskStore, market_metadata::MarketMetadataStore};
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::types::U256;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketResponse {
    result: Vec<Market>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Market {
    market_id: String,
    name: Option<String>,
    hardware_requirement: MinHardware,
    total_proofs_generated: String,
    requests_in_progress: String,
    median_time_per_proof: String,
    median_cost_per_proof: String,
    failed_requests: String,
    total_earnings: String,
    slashing_penalty: Vec<TokenAmount>,
    status: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TokenAmount {
    token: String,
    amount: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MinHardware {
    instance_type: String,
    vcpus: usize,
}

type CachedMarketResponse = CachedResponse<MarketResponse>;

use once_cell::sync::Lazy;

static MARKET_RESPONSE: Lazy<RwLock<CachedMarketResponse>> =
    Lazy::new(|| RwLock::new(CachedMarketResponse::new()));

pub async fn total_market_info(
    _local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
) -> actix_web::Result<HttpResponse> {
    // Step 1: Check if there's a cached response (lock for reading)

    if let Some(response) = MARKET_RESPONSE
        .read()
        .await
        .get_if_valid(Duration::from_secs(5))
    {
        // Return the cached response if valid
        return Ok(HttpResponse::Ok().json(response));
    }

    // Step 2: If the cache is invalid, recompute the response
    let new_response = recompute_market_response(_local_market_store, _local_ask_store).await;

    {
        let mut cache_lock = MARKET_RESPONSE.write().await;
        // Store the newly computed response in the cache
        cache_lock.store(new_response.clone());
    }
    // Return the newly computed response
    return Ok(HttpResponse::Ok().json(new_response));
}

async fn recompute_market_response(
    _local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
) -> MarketResponse {
    log::info!("Starting recompute_market_response");

    // Step 1: Acquire both locks and extract all necessary data within a scoped block
    let (
        all_markets_meta,
        proof_counts_map,
        requests_in_progress_map,
        median_time_map,
        median_cost_map,
        failed_requests_map,
        total_earnings_map,
        slashing_penalty_map,
    ) = {
        // Scoped block to limit the duration of the locks
        let market_store = _local_market_store.read().await;
        log::warn!("market_store read available");
        let ask_store = _local_ask_store.read().await;
        log::warn!("ask_store read available");

        // Extract all market metadata
        let all_markets_meta = market_store.get_all_markets().clone(); // Clone to own the data

        // Initialize HashMaps to store extracted data for quick lookup
        let mut proof_counts_map = std::collections::HashMap::new();
        let mut requests_in_progress_map = std::collections::HashMap::new();
        let mut median_time_map = std::collections::HashMap::new();
        let mut median_cost_map = std::collections::HashMap::new();
        let mut failed_requests_map = std::collections::HashMap::new();
        let mut total_earnings_map = std::collections::HashMap::new();
        let mut slashing_penalty_map = std::collections::HashMap::new();

        // Iterate over each market metadata and extract relevant data
        for meta in &all_markets_meta {
            let market_id = &meta.market_id;

            // Extract total_proofs_generated
            let total_proofs = ask_store.get_proof_count(market_id);
            proof_counts_map.insert(market_id.clone(), total_proofs.to_string());

            // Extract requests_in_progress
            let requests_in_progress = ask_store
                .get_by_ask_state_except_complete(AskState::Assigned)
                .filter_by_market_id(market_id.clone())
                .get_count();
            requests_in_progress_map.insert(market_id.clone(), requests_in_progress);

            // Extract median_time_per_proof
            let median_time = market_store
                .get_median_proof_time_market_wise(market_id)
                .to_string();
            median_time_map.insert(market_id.clone(), median_time);

            // Extract median_cost_per_proof
            let median_cost = market_store
                .get_median_proof_cost_market_wise(market_id)
                .to_string();
            median_cost_map.insert(market_id.clone(), median_cost);

            // Extract failed_requests
            let failed_requests = ask_store
                .get_failed_request_count_by_market_id(market_id)
                .to_string();
            failed_requests_map.insert(market_id.clone(), failed_requests);

            // Extract total_earnings
            let total_earnings = market_store
                .get_earnings(market_id)
                .unwrap_or(U256::zero())
                .to_string();
            total_earnings_map.insert(market_id.clone(), total_earnings);

            // Extract slashing_penalty
            let slashing_penalty = meta.slashing_penalty.to_string();
            slashing_penalty_map.insert(market_id.clone(), slashing_penalty);
        }

        (
            all_markets_meta,
            proof_counts_map,
            requests_in_progress_map,
            median_time_map,
            median_cost_map,
            failed_requests_map,
            total_earnings_map,
            slashing_penalty_map,
        )
    }; // Both locks are released here

    log::debug!("Released locks on MarketMetadataStore and LocalAskStore");

    // Step 2: Process the data using explicit loops without holding any locks
    let mut markets = Vec::with_capacity(all_markets_meta.len());

    for meta in all_markets_meta {
        let market_id = meta.market_id.clone();

        // Retrieve all pre-extracted data from the HashMaps
        let total_proofs_generated = proof_counts_map
            .get(&market_id)
            .cloned()
            .unwrap_or_default();
        let requests_in_progress = requests_in_progress_map.get(&market_id).unwrap_or(&0);
        let median_time_per_proof = median_time_map.get(&market_id).cloned().unwrap_or_default();
        let median_cost_per_proof = median_cost_map.get(&market_id).cloned().unwrap_or_default();
        let failed_requests = failed_requests_map
            .get(&market_id)
            .cloned()
            .unwrap_or_default();
        let total_earnings = total_earnings_map
            .get(&market_id)
            .cloned()
            .unwrap_or_default();
        let slashing_penalty = slashing_penalty_map
            .get(&market_id)
            .cloned()
            .unwrap_or_default();

        // Construct the Market struct
        let market = Market {
            market_id: market_id.clone().to_string(),
            name: None, // Adjust as needed
            hardware_requirement: MinHardware {
                instance_type: "todo".into(), // Replace with actual data
                vcpus: 1234,                  // Replace with actual data
            },
            total_proofs_generated,
            requests_in_progress: requests_in_progress.to_string(),
            median_time_per_proof,
            median_cost_per_proof,
            failed_requests,
            total_earnings,
            slashing_penalty: vec![TokenAmount {
                token: "POND".to_string(),
                amount: slashing_penalty,
            }],
            status: true, // Adjust as needed
        };

        markets.push(market);
    }

    log::info!("Finished processing market data");

    MarketResponse { result: markets }
}
