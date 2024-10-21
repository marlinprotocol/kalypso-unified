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
use tokio::sync::Mutex;
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
static DASHBOARD_RESPONSE: Lazy<Mutex<CachedDashboardResponse>> =
    Lazy::new(|| Mutex::new(CachedDashboardResponse::new()));

pub async fn get_dashboard(
    _local_market_store: Data<Arc<Mutex<MarketMetadataStore>>>,
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
    _local_generator_store: Data<Arc<Mutex<GeneratorStore>>>,
) -> actix_web::Result<HttpResponse> {
    let response = DASHBOARD_RESPONSE
        .lock()
        .await
        .get_or_recompute(Duration::from_secs(5), || async {
            let local_market_store = _local_market_store.lock().await;
            let local_ask_store = _local_ask_store.lock().await;
            let local_generator_store = _local_generator_store.lock().await;

            let all_market_meta_data = local_market_store.get_all_markets();
            let markets = all_market_meta_data
                .into_iter()
                .map(|meta| {
                    let market_id = meta.market_id; // Assuming `market_id` is part of `MarketMetadata`

                    let median_time =
                        local_market_store.get_median_proof_time_market_wise(&market_id);

                    let median_cost =
                        local_market_store.get_median_proof_cost_market_wise(&market_id);

                    Market {
                        name: market_id.to_string(),
                        token: "USDC".into(),
                        median_time: median_time.to_string(),
                        median_cost: median_cost.to_string(),
                    }
                })
                .collect::<Vec<Market>>();

            let recent_proofs = {
                let result = local_ask_store.get_recent_completed_proofs(20);

                result
                    .into_iter()
                    .map(|ask_request| {
                        let market_id = ask_request.market_id; // Assuming `market_id` is part of `MarketMetadata`

                        let median_time =
                            local_market_store.get_median_proof_time_market_wise(&market_id);

                        let median_cost =
                            local_market_store.get_median_proof_cost_market_wise(&market_id);

                        let market = Market {
                            name: market_id.to_string(),
                            token: "USDC".into(),
                            median_time: median_time.to_string(),
                            median_cost: median_cost.to_string(),
                        };

                        RecentProof {
                            market,
                            requestor: address_to_string(&ask_request.prover_refund_address),
                            inputs: bytes_to_string(&ask_request.prover_data),
                            generator: Generator {
                                name: None,
                                address: address_to_string(&ask_request.generator.unwrap()),
                            },
                            time: local_ask_store
                                .get_proving_time(&ask_request.ask_id)
                                .unwrap_or_else(|| U256::zero())
                                .to_string(),
                            cost: local_ask_store
                                .get_proving_cost(&ask_request.ask_id)
                                .unwrap_or_else(|| U256::zero())
                                .to_string(),
                            proof_link: local_ask_store
                                .get_proof_transaction(&ask_request.ask_id)
                                .unwrap_or_default(),
                        }
                    })
                    .collect::<Vec<RecentProof>>()
            };

            DashboardResponse {
                markets_created: local_market_store.count_markets(),
                registered_generators: local_generator_store.all_generators_address().len(),
                proofs_generated: local_ask_store.get_total_proof_count(),
                markets,
                recent_proofs,
            }
        })
        .await;
    Ok(HttpResponse::Ok().json(response))
}

// Ok(HttpResponse::Ok().json(DashboardResponse {
//     markets_created: local_market_store.count_markets(),
//     registered_generators: local_generator_store.all_generators_address().len(),
//     proofs_generated: local_ask_store.get_total_proof_count(),
//     markets,
//     recent_proofs,
// }))
