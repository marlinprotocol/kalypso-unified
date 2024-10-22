use super::cache::CachedResponse;
use crate::generator_lib::generator_store::GeneratorStore;
use crate::utility::address_to_string;
use actix_web::web::Data;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use ethers::types::U256;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GeneratorResponse {
    result: Vec<Operator>,
}

type CachedGeneratorResponse = CachedResponse<GeneratorResponse>;

use once_cell::sync::Lazy;

static GENERATOR_RESPONSE: Lazy<RwLock<CachedGeneratorResponse>> =
    Lazy::new(|| RwLock::new(CachedGeneratorResponse::new()));

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Operator {
    name: Option<String>,
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
struct TokenAmount {
    token: String,
    amount: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Market {
    name: String,
    token: Vec<String>,
    id: String,
}

pub async fn get_generators_all(
    _local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
) -> actix_web::Result<HttpResponse> {
    // Step 1: Check if there's a cached response (lock for reading)
    let cache_lock = GENERATOR_RESPONSE.read().await;

    if let Some(response) = cache_lock.get_if_valid(Duration::from_secs(5)) {
        // Return the cached response if valid
        return Ok(HttpResponse::Ok().json(response));
    }
    drop(cache_lock); // Release lock before acquiring a write lock

    // Step 2: If the cache is invalid, recompute the response
    let mut cache_lock = GENERATOR_RESPONSE.write().await;
    let new_response = recompute_generator_response(_local_generator_store).await;

    // Store the newly computed response in the cache
    cache_lock.store(new_response.clone());

    // Return the newly computed response
    Ok(HttpResponse::Ok().json(new_response))
}

async fn recompute_generator_response(
    _local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
) -> GeneratorResponse {
    // Step 1: Acquire the lock and extract all necessary data
    let generator_details = {
        let store = _local_generator_store.read().await;
        let all_generators = { store.all_generators_address().to_owned() };

        // Prepare a vector to hold generator details
        let mut generator_details = Vec::with_capacity(all_generators.len());

        for generator_address in &all_generators {
            // Clone generator_address to ensure ownership
            let generator_address = generator_address.clone();

            // Retrieve and clone operator_data
            if let Some(operator_data) = store.get_by_address(&generator_address).cloned() {
                // Retrieve and clone all_markets_of_generator
                let all_markets_of_generator = store
                    .get_all_markets_of_generator(&generator_address)
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>(); // Ensure deep cloning

                // Retrieve and clone total_earning
                let total_earning = store
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

    for (_, operator_data, all_markets_of_generator, total_earning) in generator_details {
        // Construct the delegations
        let delegations = vec![TokenAmount {
            token: "POND".into(),
            amount: operator_data.total_stake.to_string(),
        }];

        // Construct the current stake
        let current_stake = vec![TokenAmount {
            token: "POND".into(),
            amount: operator_data.total_stake.to_string(),
        }];

        // Construct the markets
        let mut markets = Vec::with_capacity(all_markets_of_generator.len());
        for info_per_market in &all_markets_of_generator {
            let market = Market {
                name: info_per_market.market_id.to_string(),
                token: vec!["POND".into()],
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

        // Construct the Operator struct
        let operator = Operator {
            name: Some(address_to_string(&operator_data.address)),
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

    GeneratorResponse { result }
}
