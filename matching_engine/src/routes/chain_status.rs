use crate::{
    latest_block_store::LatestBlockStoreTrait,
    models::{BalanceResponse, GetLatestBlockNumberResponse, WelcomeResponse},
};
use actix_web::web::Data;
use actix_web::HttpResponse;
use std::sync::Arc;
use tokio::sync::RwLock;

use ethers::types::U256;

#[utoipa::path(
    get,
    path = "/stats/getLatestBlock",
    responses(
        (status = 200, description = "Return the latest block till engine has parsed", body = GetLatestBlockNumberResponse),
        (status = 423, description = "Parsing in progress", body = WelcomeResponse )
    ),
    tag = "Manage"
)]
pub async fn get_latest_block_number<BS: LatestBlockStoreTrait + Send + Sync>(
    _shared_parsed_block: Data<Arc<RwLock<BS>>>,
) -> actix_web::Result<HttpResponse> {
    let latest_parsed_block = {
        match _shared_parsed_block.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    return Ok(HttpResponse::Ok().json(GetLatestBlockNumberResponse {
        block_number: latest_parsed_block.get_latest_block().to_string(),
    }));
}

#[utoipa::path(
    get,
    path = "/stats/welcome",
    responses(
        (status = 200),
    ),
    tag = "Manage"
)]
pub async fn welcome() -> actix_web::Result<HttpResponse> {
    return Ok(HttpResponse::Ok().json(WelcomeResponse {
        status: "ok".into(),
    }));
}

#[utoipa::path(
    get,
    path = "/stats/getKeyBalance",
    responses(
        (status = 200, description = "Return Balance of Relayer"),
        (status = 423, description = "Parsing in progress", body = WelcomeResponse )
    ),
    tag = "Manage"
)]
pub async fn gas_key_balance(balance: Data<Arc<RwLock<U256>>>) -> actix_web::Result<HttpResponse> {
    let threshold = U256::from_dec_str("10000000000000").unwrap();
    let data = {
        match balance.try_read() {
            Ok(data) => *data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    if data > threshold {
        return Ok(HttpResponse::Ok().json(BalanceResponse {
            status: "ok".into(),
            balance: Some(data),
        }));
    } else {
        return Ok(HttpResponse::ExpectationFailed().json(BalanceResponse {
            status: "less balance".into(),
            balance: Some(data),
        }));
    }
}
