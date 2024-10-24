use crate::{
    ask_lib::{
        ask_status::AskState,
        ask_store::{LocalAskStore, Proof},
    },
    models::{
        GetAskStatus, GetAskStatusResponse, GetProofResponse, GetStatusResponse, WelcomeResponse,
    },
};
use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::core::types::U256;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn get_status(
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
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

pub async fn get_ask_proof_by_ask_id(
    _payload: web::Json<GetAskStatus>,
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
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

pub async fn get_ask_status_askid(
    _payload: web::Json<GetAskStatus>,
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
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
