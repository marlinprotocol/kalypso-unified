use super::cache::CachedResponse;
use crate::generator_lib::generator_store::{GeneratorMeta, GeneratorStore};
use crate::generator_lib::native_stake_store::NativeStakingStore;
use crate::generator_lib::symbiotic_stake_store::SymbioticStakeStore;
use crate::models::WelcomeResponse;
use crate::utility::{address_to_string, TokenAmount, TokenTracker};
use crate::{try_read_and_get_if_valid, try_read_or_lock};
use actix_web::web::Data;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use ethers::types::{Address, U256};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GeneratorResponse {
    result: Vec<Operator>,
    registered_generators: usize,
    total_staked: Vec<TokenAmount>,
}

type CachedGeneratorResponse = CachedResponse<GeneratorResponse>;

use once_cell::sync::Lazy;

static GENERATOR_RESPONSE: Lazy<RwLock<CachedGeneratorResponse>> =
    Lazy::new(|| RwLock::new(CachedGeneratorResponse::new()));

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Operator {
    details: GeneratorMeta,
    address: String,
    delegations: Vec<TokenAmount>,
    markets: Vec<Market>,
    earnings_to_date: String,
    proofs_generated: String,
    proofs_missed: String,
    pending_proofs: String,
    current_stake: Vec<TokenAmount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Market {
    id: String,
    name: String,
    token: Vec<String>,
}

pub async fn get_generators_all(
    _local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
    _local_native_store: Data<Arc<RwLock<NativeStakingStore>>>,
    _local_symbiotic_store: Data<Arc<RwLock<SymbioticStakeStore>>>,
) -> actix_web::Result<HttpResponse> {
    try_read_and_get_if_valid!(GENERATOR_RESPONSE, generator_cache, Duration::from_secs(10));
    drop(generator_cache);

    try_read_or_lock!(_local_generator_store, local_generator_store);
    try_read_or_lock!(_local_native_store, local_native_store);
    try_read_or_lock!(_local_symbiotic_store, local_symbiotic_store);

    // Step 2: If the cache is invalid, recompute the response
    let new_response = recompute_generator_response(
        local_generator_store,
        local_native_store,
        local_symbiotic_store,
    )
    .await;

    {
        // Store the newly computed response in the cache
        match GENERATOR_RESPONSE.try_write() {
            Ok(mut cache_write_lock) => {
                cache_write_lock.store(new_response.clone());
            }
            _ => {
                log::warn!("Failed Caching generator response");
            }
        };
    }

    // Return the newly computed response
    return Ok(HttpResponse::Ok().json(new_response));
}

async fn recompute_generator_response<'a>(
    local_generator_store: RwLockReadGuard<'a, GeneratorStore>,
    local_native_store: RwLockReadGuard<'a, NativeStakingStore>,
    local_symbiotic_store: RwLockReadGuard<'a, SymbioticStakeStore>,
) -> GeneratorResponse {
    // Step 1: Acquire the lock and extract all necessary data
    let generator_details = {
        let all_generators = { local_generator_store.all_generators_address().to_owned() };

        // Prepare a vector to hold generator details
        let mut generator_details = Vec::with_capacity(all_generators.len());

        for generator_address in &all_generators {
            // Clone generator_address to ensure ownership
            let generator_address = generator_address.clone();

            // Retrieve and clone operator_data
            if let Some(operator_data) = local_generator_store.get_by_address(&generator_address) {
                // Retrieve and clone all_markets_of_generator
                let all_markets_of_generator = local_generator_store
                    .get_all_markets_of_generator(&generator_address)
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>(); // Ensure deep cloning

                // Retrieve and clone total_earning
                let total_earning = local_generator_store
                    .get_total_earning(&generator_address)
                    .unwrap_or_else(|| U256::zero())
                    .clone();

                // Push the cloned data into generator_details
                generator_details.push((
                    generator_address,
                    operator_data,
                    all_markets_of_generator,
                    total_earning,
                ));
            }
        }

        generator_details
    };

    // Step 2: Process the data outside the locked scope using explicit loops
    let mut result = Vec::with_capacity(generator_details.len());
    let mut total_stake = TokenTracker::new();

    for (_, operator_data, all_markets_of_generator, total_earning) in generator_details {
        total_stake +=
            operator_data.clone().total_native_stake + operator_data.clone().total_symbiotic_stake;

        // Construct the markets
        let mut markets = Vec::with_capacity(all_markets_of_generator.len());
        let (all_tokens_supported, _): (Vec<Address>, Vec<U256>) =
            (local_native_store.tokens_to_lock.clone()
                + local_symbiotic_store.tokens_to_lock.clone())
            .to_address_token_pair()
            .into_iter()
            .unzip();
        for info_per_market in &all_markets_of_generator {
            let market = Market {
                name: info_per_market.market_id.to_string(),
                token: all_tokens_supported
                    .iter()
                    .map(|a| address_to_string(a))
                    .collect::<Vec<String>>(),
                id: info_per_market.market_id.to_string(),
            };
            markets.push(market);
        }

        // Calculate proofs_generated, proofs_missed, and pending_proofs
        let mut proofs_generated = U256::zero();
        let mut proofs_missed = U256::zero();
        let mut pending_proofs = U256::zero();

        for info in &all_markets_of_generator {
            proofs_generated += info.proofs_submitted;
            proofs_missed += info.proofs_slashed;
            pending_proofs += info.active_requests;
        }

        let delegations = (operator_data.total_native_stake.clone()
            + operator_data.total_symbiotic_stake.clone())
        .to_token_amount()
        .to_owned();
        let current_stake = delegations.clone();
        // Construct the Operator struct
        let operator = Operator {
            details: operator_data.deserialize_generator_bytes(),
            address: address_to_string(&operator_data.address),
            delegations,
            markets,
            earnings_to_date: total_earning.to_string(),
            proofs_generated: proofs_generated.to_string(),
            proofs_missed: proofs_missed.to_string(),
            pending_proofs: pending_proofs.to_string(),
            current_stake,
        };

        result.push(operator);
    }

    let registered_generators = result.len();

    GeneratorResponse {
        result,
        registered_generators,
        total_staked: total_stake.to_token_amount(),
    }
}
