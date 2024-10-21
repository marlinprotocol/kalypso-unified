use super::cache::CachedResponse;
use crate::generator_lib::generator_store::GeneratorStore;
use crate::utility::address_to_string;
use actix_web::web::Data;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use ethers::types::U256;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GeneratorResponse {
    result: Vec<Operator>,
}

type CachedGeneratorResponse = CachedResponse<GeneratorResponse>;

use once_cell::sync::Lazy;

static GENERATOR_RESPONSE: Lazy<Mutex<CachedGeneratorResponse>> =
    Lazy::new(|| Mutex::new(CachedGeneratorResponse::new()));

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
    _local_generator_store: Data<Arc<Mutex<GeneratorStore>>>,
) -> actix_web::Result<HttpResponse> {
    let mut cache_lock = GENERATOR_RESPONSE.lock().await;
    let response = cache_lock
        .get_or_recompute(
            Duration::from_secs(5),
            recompute_generator_response(_local_generator_store.clone()),
        )
        .await;

    Ok(HttpResponse::Ok().json(response))
}

async fn recompute_generator_response(
    _local_generator_store: Data<Arc<Mutex<GeneratorStore>>>,
) -> GeneratorResponse {
    let local_generator_store = _local_generator_store.lock().await;
    let all_generators = local_generator_store.all_generators_address();

    let result = all_generators
        .into_iter()
        .map(|generator_address| {
            let operator_data = local_generator_store
                .get_by_address(&generator_address)
                .unwrap();
            let all_markets_of_generator =
                local_generator_store.get_all_markets_of_generator(&generator_address);

            Operator {
                name: Some(address_to_string(&operator_data.address)),
                address: address_to_string(&operator_data.address),
                delegations: vec![TokenAmount {
                    token: "POND".into(),
                    amount: operator_data.total_stake.to_string(),
                }],
                markets: all_markets_of_generator
                    .clone()
                    .into_iter()
                    .map(|info_per_market| Market {
                        name: info_per_market.market_id.to_string(),
                        token: vec!["POND".into()],
                        id: info_per_market.market_id.to_string(),
                    })
                    .collect::<Vec<Market>>(),
                earnings_to_date: local_generator_store
                    .get_total_earning(&generator_address)
                    .unwrap_or_default()
                    .to_string(),
                proofs_generated: all_markets_of_generator
                    .clone()
                    .into_iter()
                    .map(|info| info.proofs_submitted)
                    .fold(U256::zero(), |a, x| a + x)
                    .to_string(),
                proofs_missed: all_markets_of_generator
                    .clone()
                    .into_iter()
                    .map(|info| info.proofs_slashed)
                    .fold(U256::zero(), |a, x| a + x)
                    .to_string(),
                pending_proofs: all_markets_of_generator
                    .clone()
                    .into_iter()
                    .map(|info| info.active_requests)
                    .fold(U256::zero(), |a, x| a + x)
                    .to_string(),
                current_stake: vec![TokenAmount {
                    token: "POND".into(),
                    amount: operator_data.total_stake.to_string(),
                }],
            }
        })
        .collect::<Vec<Operator>>();

    GeneratorResponse { result }
}
