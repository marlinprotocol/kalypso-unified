use actix_web::web::Data;
use actix_web::HttpResponse;
use matching_engine::models::{GetLatestBlockNumberResponse, WelcomeResponse};
use std::sync::Arc;
use tokio::sync::Mutex;

use ethers::core::types::U64;

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
