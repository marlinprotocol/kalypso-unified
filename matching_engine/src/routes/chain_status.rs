use actix_web::web::Data;
use actix_web::HttpResponse;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use ethers::core::types::U64;

#[derive(Serialize)]
pub struct GetLatestBlockNumberResponse {
    block_number: String,
}

pub async fn get_latest_block_number(
    _shared_parsed_block: Data<Arc<Mutex<U64>>>,
) -> actix_web::Result<HttpResponse> {
    let latest_parsed_block = _shared_parsed_block.lock().await;

    Ok(HttpResponse::Ok().json(GetLatestBlockNumberResponse {
        block_number: latest_parsed_block.to_string(),
    }))
}

#[derive(Serialize)]
struct WelcomeResponse {
    status: &'static str,
}

pub async fn welcome() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(WelcomeResponse { status: "ok" }))
}
