use actix_web::web::Data;
use actix_web::HttpResponse;
use matching_engine::models::{BalanceResponse, GetLatestBlockNumberResponse, WelcomeResponse};
use std::sync::Arc;
use tokio::sync::Mutex;

use ethers::{core::types::U64, types::U256};

pub async fn get_latest_block_number(
    _shared_parsed_block: Data<Arc<Mutex<U64>>>,
) -> actix_web::Result<HttpResponse> {
    let latest_parsed_block = _shared_parsed_block.lock().await;

    Ok(HttpResponse::Ok().json(GetLatestBlockNumberResponse {
        block_number: latest_parsed_block.to_string(),
    }))
}

pub async fn welcome() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(WelcomeResponse {
        status: "ok".into(),
    }))
}

pub async fn gas_key_balance(balance: Data<Arc<Mutex<U256>>>) -> actix_web::Result<HttpResponse> {
    let threshold = U256::from_dec_str("10000000").unwrap();

    let data = { *balance.lock().await };

    if data > threshold {
        Ok(HttpResponse::Ok().json(BalanceResponse {
            status: "ok".into(),
            balance: Some(data),
        }))
    } else {
        return Ok(HttpResponse::Ok().json(BalanceResponse {
            status: "less balance".into(),
            balance: Some(data),
        }));
    }
}
