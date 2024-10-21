use super::cache::CachedResponse;
use crate::ask_lib::ask_status::AskState;
use crate::{ask_lib::ask_store::LocalAskStore, market_metadata::MarketMetadataStore};
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::types::U256;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
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
    total_proofs_generated: usize,
    requests_in_progress: usize,
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

static MARKET_RESPONSE: Lazy<Mutex<CachedMarketResponse>> =
    Lazy::new(|| Mutex::new(CachedMarketResponse::new()));

pub async fn market_info(
    _local_market_store: Data<Arc<Mutex<MarketMetadataStore>>>,
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
) -> actix_web::Result<HttpResponse> {
    let response = MARKET_RESPONSE
        .lock()
        .await
        .get_or_recompute(Duration::from_secs(5), || async {
            let local_market_store = _local_market_store.lock().await;
            let local_ask_store = _local_ask_store.lock().await;

            let all_market_meta_data = local_market_store.get_all_markets();
            let markets = all_market_meta_data
                .into_par_iter()
                .map(|meta| Market {
                    market_id: meta.market_id.to_string(),
                    name: None,
                    hardware_requirement: MinHardware {
                        instance_type: "todo".into(),
                        vcpus: 1234,
                    },
                    total_proofs_generated: local_ask_store.get_proof_count(&meta.market_id),
                    requests_in_progress: local_ask_store
                        .get_by_ask_state_except_complete(AskState::Assigned)
                        .filter_by_market_id(meta.market_id)
                        .get_count(),
                    median_time_per_proof: local_market_store
                        .get_median_proof_time_market_wise(&meta.market_id)
                        .to_string(),
                    median_cost_per_proof: local_market_store
                        .get_median_proof_cost_market_wise(&meta.market_id)
                        .to_string(),
                    failed_requests: local_ask_store
                        .get_failed_request_count_by_market_id(&meta.market_id)
                        .to_string(),
                    total_earnings: local_market_store
                        .get_earnings(&meta.market_id)
                        .unwrap_or(U256::zero())
                        .to_string(),
                    slashing_penalty: vec![TokenAmount {
                        token: "POND".to_string(),
                        amount: meta.slashing_penalty.to_string(),
                    }],
                    status: true,
                })
                .collect::<Vec<Market>>();
            MarketResponse { result: markets }
        })
        .await;
    Ok(HttpResponse::Ok().json(response))
}
