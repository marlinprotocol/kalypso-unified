use super::cache::CachedResponse;
use crate::ask_lib::ask_status::AskState;
use crate::ask_lib::ask_store::{AskManagementRead, CompletedProofsManagement, ProofCounters};
use crate::generator_lib::generator_store::GeneratorStore;
use crate::generator_lib::native_stake_store::NativeStakingStore;
use crate::generator_lib::symbiotic_stake_store::SymbioticStakeStore;
use crate::market_metadata::{MarketSetupData, MinHardware};
use crate::models::WelcomeResponse;
use crate::utility::{TokenAmount, TokenTracker};
use crate::{ask_lib::ask_store::LocalAskStore, market_metadata::MarketMetadataStore};
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
pub struct MarketResponse {
    /// List of markets
    result: Vec<Market>,
    /// Total number of generators registered across all markets
    registered_generators: usize,
    /// deprecated: Total stake across all generators (need native+sybmbiotic combined)
    total_stake: Vec<TokenAmount>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Market {
    /// id of the market
    market_id: String,
    /// name of the market (optional)
    name: Option<String>,
    /// hardware requirement for generators to participate in the market
    hardware_requirement: MinHardware,
    /// total proofs generated in the market
    total_proofs_generated: String,
    /// requests in progress in the market
    requests_in_progress: String,
    /// median time per proof in the market
    median_time_per_proof: String,
    /// median cost per proof in the market
    median_cost_per_proof: String,
    /// failed requests in the market
    failed_requests: String,
    /// total earnings in the market
    total_earnings: String,
    /// deprecated: slashing penalty in the market (no penaltly for now)
    slashing_penalty: Vec<TokenAmount>,
    /// deprecated: status of the market
    status: bool,
    /// setup data of the market
    market_setup_data: MarketSetupData,
    /// number of generators registered in the market
    registered_generators: usize,
}

type CachedMarketResponse = CachedResponse<MarketResponse>;

use once_cell::sync::Lazy;

static MARKET_RESPONSE: Lazy<RwLock<CachedMarketResponse>> =
    Lazy::new(|| RwLock::new(CachedMarketResponse::new()));

#[utoipa::path(
    get,
    path = "/ui/markets",
    responses(
        (status = 200, description = "Market Response", body = MarketResponse),
        (status = 423, description = "Resource Locked", body = WelcomeResponse),
    ),
    tag = "UI"
)]
pub async fn total_market_info(
    _local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    _local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
    _local_native_store: Data<Arc<RwLock<NativeStakingStore>>>,
    _local_symbiotic_store: Data<Arc<RwLock<SymbioticStakeStore>>>,
) -> actix_web::Result<HttpResponse> {
    // Step 1: Check if there's a cached response (lock for reading)
    try_read_and_get_if_valid!(MARKET_RESPONSE, market_cache, Duration::from_millis(100));
    drop(market_cache);

    try_read_or_lock!(_local_ask_store, local_ask_store);
    try_read_or_lock!(_local_market_store, local_market_store);
    try_read_or_lock!(_local_generator_store, local_generator_store);
    try_read_or_lock!(_local_native_store, local_native_store);
    try_read_or_lock!(_local_symbiotic_store, local_symbiotic_store);

    // Step 2: If the cache is invalid, recompute the response
    let new_response = recompute_market_response(
        local_market_store,
        local_ask_store,
        local_generator_store,
        local_native_store,
        local_symbiotic_store,
    )
    .await;

    {
        // Store the newly computed response in the cache
        match MARKET_RESPONSE.try_write() {
            Ok(mut cache_write_lock) => {
                cache_write_lock.store(new_response.clone());
            }
            _ => {
                log::warn!("Failed Caching market response");
            }
        };
    }
    // Return the newly computed response
    return Ok(HttpResponse::Ok().json(new_response));
}

async fn recompute_market_response<'a>(
    local_market_store: RwLockReadGuard<'a, MarketMetadataStore>,
    local_ask_store: RwLockReadGuard<'a, LocalAskStore>,
    local_generator_store: RwLockReadGuard<'a, GeneratorStore>,
    local_native_store: RwLockReadGuard<'a, NativeStakingStore>,
    local_symbiotic_store: RwLockReadGuard<'a, SymbioticStakeStore>,
) -> MarketResponse {
    log::trace!("Starting recompute_market_response");

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
        // Extract all market metadata
        let all_markets_meta = local_market_store.get_all_markets().to_owned(); // Clone to own the data

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
            let total_proofs = local_ask_store.get_proof_count(market_id);
            proof_counts_map.insert(market_id.clone(), total_proofs.to_string());

            // Extract requests_in_progress
            let requests_in_progress = local_ask_store
                .get_by_ask_state_except_complete(AskState::Assigned)
                .filter_by_market_id(market_id.clone())
                .get_count();
            requests_in_progress_map.insert(market_id.clone(), requests_in_progress);

            // Extract median_time_per_proof
            let median_time = local_market_store
                .get_median_proof_time_market_wise(market_id)
                .to_owned()
                .to_string();
            median_time_map.insert(market_id.clone(), median_time);

            // Extract median_cost_per_proof
            let median_cost = local_market_store
                .get_median_proof_cost_market_wise(market_id)
                .to_owned()
                .to_string();
            median_cost_map.insert(market_id.clone(), median_cost);

            // Extract failed_requests
            let failed_requests = local_ask_store
                .get_failed_request_count_by_market_id(market_id)
                .to_owned()
                .to_string();
            failed_requests_map.insert(market_id.clone(), failed_requests);

            // Extract total_earnings
            let total_earnings = local_market_store
                .get_earnings(market_id)
                .to_owned()
                .unwrap_or(U256::zero())
                .to_string();
            total_earnings_map.insert(market_id.clone(), total_earnings);

            // Extract slashing_penalty
            let slashing_penalty = local_native_store.tokens_to_lock.clone()
                + local_symbiotic_store.tokens_to_lock.clone();
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

    log::trace!("Released locks on MarketMetadataStore and LocalAskStore");

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
        let _slashing_penalty = slashing_penalty_map
            .get(&market_id)
            .cloned()
            .unwrap_or_default();

        // Construct the Market struct
        let market = Market {
            market_id: market_id.clone().to_string(),
            name: meta.deserialize_market_bytes().zk_app_name,
            hardware_requirement: meta.deserialize_market_bytes().min_hardware,
            total_proofs_generated,
            requests_in_progress: requests_in_progress.to_string(),
            median_time_per_proof,
            median_cost_per_proof,
            failed_requests,
            total_earnings,
            slashing_penalty: TokenTracker::default().to_token_amount(),
            status: true, // Adjust as needed
            market_setup_data: meta.deserialize_market_bytes(),
            registered_generators: local_generator_store.get_all_by_market_id(&market_id).len(),
        };

        markets.push(market);
    }

    log::trace!("Finished processing market data");

    MarketResponse {
        result: markets,
        registered_generators: local_generator_store.all_generators_address().len(),
        total_stake: (local_generator_store.total_native_stake_accross_all_generators()
            + local_generator_store.total_symbiotic_stake_across_all_generators())
        .to_token_amount(),
    }
}
