use crate::ask::*;
use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::core::types::U256;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize)]
struct GetStatusResponse {
    local_ask_status: LocalAskStatus,
}

#[derive(Deserialize)]
pub struct GetAskStatus {
    ask_id: String,
}

#[derive(Serialize)]
pub struct GetAskStatusResponse {
    state: String,
}

pub async fn get_status(
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
) -> actix_web::Result<HttpResponse> {
    let local_ask_store = _local_ask_store.lock().await;

    Ok(HttpResponse::Ok().json(GetStatusResponse {
        local_ask_status: local_ask_store.get_ask_status(),
    }))
}

pub async fn get_ask_status_askid(
    _payload: web::Json<GetAskStatus>,
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
) -> actix_web::Result<HttpResponse> {
    let local_ask_store = _local_ask_store.lock().await;
    let ask_id: String = _payload.ask_id.clone();
    let ask_id_u256: U256 = U256::from_dec_str(&ask_id).expect("Failed to parse string");

    let local_ask: Option<&LocalAsk> = local_ask_store.get_by_ask_id(&ask_id_u256);

    let ask_state_enum: Option<AskState> = local_ask.unwrap().state;

    let ask_state = match ask_state_enum {
        Some(AskState::Null) => "NULL",
        Some(AskState::Create) => "Create",
        Some(AskState::UnAssigned) => "UnAssigned",
        Some(AskState::Assigned) => "Assigned",
        Some(AskState::Complete) => "Complete",
        Some(AskState::DeadlineCrossed) => "DeadlineCrossed",
        Some(AskState::InvalidSecret) => "InvalidSecret",
        None => "None", // Handle the None case
    };

    Ok(HttpResponse::Ok().json(GetAskStatusResponse {
        state: ask_state.to_owned(),
    }))
}
