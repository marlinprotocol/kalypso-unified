use crate::{
    ask_lib::{ask_status::AskState, ask_store::AskManagementRead, Proof},
    models::{
        GetAskStatus, GetAskStatusResponse, GetProofResponse, GetStatusResponse, WelcomeResponse,
    },
};
use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::core::types::U256;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[utoipa::path(
    get,
    path = "/stats/getStatus",
    responses(
        (status = 200, description = "Return high level status of Bids", body = GetStatusResponse),
        (status = 423, description = "Parsing in progress" )
    ),
    tag = "Manage"
)]
pub async fn get_status<AS: AskManagementRead + Send + Sync>(
    _local_ask_store: Data<Arc<RwLock<AS>>>,
) -> actix_web::Result<HttpResponse> {
    let local_ask_store = {
        match _local_ask_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    return Ok(HttpResponse::Ok().json(GetStatusResponse {
        local_ask_status: local_ask_store.get_ask_status(),
    }));
}

#[utoipa::path(
    post,
    path = "/stats/getProof",
    request_body = GetAskStatus,
    responses(
        (status = 200, description = "Return proof for Bid", body = GetProofResponse),
        (status = 423, description = "Parsing in progress" )
    ),
    tag = "Manage"
)]
pub async fn get_ask_proof_by_ask_id<AS: AskManagementRead + Send + Sync>(
    _payload: web::Json<GetAskStatus>,
    _local_ask_store: Data<Arc<RwLock<AS>>>,
) -> actix_web::Result<HttpResponse> {
    let ask_id: String = _payload.ask_id.clone();
    let ask_id_u256: U256 = U256::from_dec_str(&ask_id).expect("Failed to parse string");
    let local_ask_store = {
        match _local_ask_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let proof = local_ask_store.get_proof_by_ask_id(&ask_id_u256);

    match proof {
        Some(proof) => match proof {
            Proof::ValidProof(valid_proof) => {
                return Ok(HttpResponse::Ok().json(GetProofResponse {
                    status: "Found".into(),
                    proof: valid_proof.to_vec(),
                }))
            }
            Proof::InvalidInputAttestation => {
                return Ok(HttpResponse::NotFound().json(GetProofResponse {
                    status: "Invalid Inputs Detected".into(),
                    proof: vec![],
                }))
            }
            Proof::FailedProofGeneration => {
                return Ok(HttpResponse::ExpectationFailed().json(GetProofResponse {
                    status: "Proof Request was not completed by generator".into(),
                    proof: vec![],
                }))
            }
        },
        _ => {
            return Ok(HttpResponse::NotFound().json(GetProofResponse {
                status: "Not Found".into(),
                proof: vec![],
            }))
        }
    }
}

#[utoipa::path(
    post,
    path = "/stats/getAskStatus",
    request_body = GetAskStatus,
    responses(
        (status = 200, description = "Return status of bid", body = GetAskStatusResponse),
        (status = 423, description = "Parsing in progress" )
    ),
    tag = "Manage"
)]
pub async fn get_ask_status_askid<AS: AskManagementRead + Send + Sync>(
    _payload: web::Json<GetAskStatus>,
    _local_ask_store: Data<Arc<RwLock<AS>>>,
) -> actix_web::Result<HttpResponse> {
    let local_ask_store = {
        match _local_ask_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let ask_id: String = _payload.ask_id.clone();
    let ask_id_u256: U256 = U256::from_dec_str(&ask_id).expect("Failed to parse string");

    let local_ask = match local_ask_store.get_by_ask_id(&ask_id_u256) {
        Some(data) => data,
        None => {
            return Ok(HttpResponse::NotFound().json(GetAskStatusResponse {
                state: "Request Not Found".to_owned(),
            }))
        }
    };

    let ask_state_enum = match local_ask.state {
        Some(data) => data,
        None => {
            return Ok(HttpResponse::NotFound().json(GetAskStatusResponse {
                state: "Request State Not Found".to_owned(),
            }))
        }
    };

    let ask_state = match ask_state_enum {
        AskState::Null => "NULL",
        AskState::Create => "Create",
        AskState::UnAssigned => "UnAssigned",
        AskState::Assigned => "Assigned",
        AskState::Complete => "Complete",
        AskState::DeadlineCrossed => "DeadlineCrossed",
        AskState::InvalidSecret => "InvalidSecret",
    };

    return Ok(HttpResponse::Ok().json(GetAskStatusResponse {
        state: ask_state.to_owned(),
    }));
}

#[utoipa::path(
    get,
    path = "/stats/getAsk/{id}",
    responses(
        (status = 200, description = "Return bid details"),
        (status = 404, description = "Parsing in progress", body = GetAskStatusResponse )
    ),
    params(
        ("id" = u64, Path, description = "Bid ID"),
    ),
    tag = "Manage"
)]
pub async fn get_ask<AS: AskManagementRead + Send + Sync>(
    _local_ask_store: Data<Arc<RwLock<AS>>>,
    path: web::Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let ask_id = match U256::from_dec_str(&path.into_inner().0) {
        Ok(data) => data,
        _ => {
            return Ok(HttpResponse::BadRequest().json(WelcomeResponse {
                status: "Invalid Generator Id".into(),
            }))
        }
    };

    let local_ask_store = {
        match _local_ask_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let ask = local_ask_store.get_by_ask_id(&ask_id);

    if ask.is_none() {
        return Ok(HttpResponse::NotFound().json(GetAskStatusResponse {
            state: "Request Not Found".to_owned(),
        }));
    }

    let mut ask = ask.unwrap();
    ask.secret_data = None;

    let response = json!({
        "ask_id": ask.ask_id.to_string(),
        "market_id": ask.market_id.to_string(),
        "reward": ask.reward.to_string(),
        "expiry": ask.expiry.to_string(),
        "deadline": ask.deadline.to_string(),
        "time_requested_for_proof_generation": ask.time_requested_for_proof_generation.to_string(),
        "prover_refund_address": format!("{:?}", ask.prover_refund_address),
        "prover_data": hex::encode(&ask.prover_data),
        "has_private_inputs": ask.has_private_inputs,
        "state": ask.state.as_ref().map(|state| format!("{:?}", state)),
        "generator": ask.generator.map(|addr| format!("{:?}", addr)),
        "invalid_secret_flag": ask.invalid_secret_flag,
        "created_on": ask.created_on.to_string(),
        "create_transaction": format!("{:?}", ask.create_transaction),
    });

    return Ok(HttpResponse::Locked().json(response));
}
