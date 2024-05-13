use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use secret_input_helpers::secret_inputs_helpers;
use ethers::core::types::{U256, U64};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::ask::*;

// use crate::utility;

#[derive(Serialize)]
struct WelcomeResponse {
    status: &'static str,
}

pub async fn welcome() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(WelcomeResponse { status: "ok" }))
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct GetCipherRequest {
    ask_id: String,
    signature: String,
    hash_message: [u8; 32],
}

#[derive(Serialize)]
struct GetCipherResponse {
    signed_message: String,
}

#[allow(unused_doc_comments)]
pub async fn get_cipher(
    _payload: web::Json<GetCipherRequest>,
    _market_store: Data<Arc<Mutex<MarketMetadataStore>>>,
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
) -> actix_web::Result<HttpResponse> {
    /**
     * 1. Read the input payload
     * 2. check the signature of the payload (use ecrecover accordingly)
     * 3. get ask id from the payload done
     * 4. ivs-signer = ecrecover(payload) done
     * 5. check if ivs-signer is same as askid.marketId.ivsSigner (to avoid spam). Use all local stores available.
     * 6. read ask secrets from acl from store, (if task is proof generated, then it must be not there in ask-store, then send "request is already processed")
     * 7. if proof is not generated, check inputs and secret match. If matched send response "it is valid request"
     * 8. If invalid request, send signed message "struct GetCipherResponse"
     */
    // let ivs_signer = utility::derive_address_from_signature(_payload.signature, _payload.hash_message);

    // let ask_id: String = _payload.ask_id;
    // let ask_id_u256: U256 = U256::from_dec_str(&ask_id).expect("Failed to parse string");

    // let local_ask_store = _local_ask_store.lock().await;

    // let local_ask: Option<&LocalAsk> = local_ask_store.get_by_ask_id(&ask_id_u256);

    // let market_id: U256 = local_ask.unwrap().market_id;

    // let market_metadata_store = _market_store.lock().await;

    // let market_metadata: Option<&MarketMetadata> = market_metadata_store.get_market_by_market_id(&market_id);

    // let ivs_signer_from_store: ethers::types::H160 = market_metadata.unwrap().ivs_signer;

    // match(ivs_signer, ivs_signer_from_store){
    // (a, b) if a == b => println!("ivs_signer are equal"),
    // (a, b) => println!("ivs_signer not matchting"),
    // }
    Ok(HttpResponse::NotImplemented().json(GetCipherResponse {
        signed_message: "not implemented".into(),
    }))
}

#[derive(Serialize)]
struct GetStatusResponse {
    local_ask_status: LocalAskStatus,
}

pub async fn get_status(
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
) -> actix_web::Result<HttpResponse> {
    let local_ask_store = _local_ask_store.lock().await;

    Ok(HttpResponse::Ok().json(GetStatusResponse {
        local_ask_status: local_ask_store.get_ask_status(),
    }))
}

#[derive(Deserialize)]
pub struct GetAskStatus {
    ask_id: String,
}

#[derive(Serialize)]
pub struct GetAskStatusResponse {
    state: String,
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

#[derive(Serialize)]
pub struct GetLatestBlockNumberResponse {
    block_number: String,
}

pub async fn get_latest_block_number(
    _shared_parsed_block: Data<Arc<Mutex<U64>>>,
) -> actix_web::Result<HttpResponse> {
    let latest_parsed_block = _shared_parsed_block.lock().await;
    let ask_id: String = _payload.ask_id.clone();
    let ask_id_u256: U256 = U256::from_dec_str(&ask_id).expect("Failed to parse string");

    let local_ask: Option<&LocalAsk> = local_ask_store.get_by_ask_id(&ask_id_u256);

    
    Ok(HttpResponse::Ok().json(GetLatestBlockNumberResponse {
        block_number: latest_parsed_block.to_string(),
    }))
}

#[derive(Deserialize)]
pub struct GetRequest {
    ask_id: string,
    has_private_inputs: bool,
    private_input: p_i, 
    key: p_k,
}


#[derive(serialize)]
pub struct GetRequestResponse {
    encrpyted_data : string,   
}

pub async fn get_request(
    _payload: web::Json<GetAskStatus>,
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
) -> actix_web::Result<HttpResponse> {
    let has_private_inputs: bool = _payload.has_private_inputs.clone();
    if !has_private_inputs{
        Ok(HttpResponse::UNAUTHORIZED().json(GetRequestResponse {
            encrpyted_data:"UNAUTHORIZED",
        }))
    }
    else{
        let ask_id: String = _payload.ask_id.clone();
        let ask_id_u256: U256 = U256::from_dec_str(&ask_id).expect("Failed to parse string");

        let local_ask: Option<&LocalAsk> = local_ask_store.get_by_ask_id(&ask_id_u256);
        let serialized = serde_json::to_string(&local_ask).unwrap();

        let key = _payload.key.clone();
        let bytes: Vec<u8> = serialized.as_bytes().to_vec();

        let encrypted_aes_data = secret_inputs_helpers::encrypt_aes_gcm(&bytes, &key, market_id).unwrap();

        Ok(HttpResponse::Ok().json(GetRequestResponse {
            encrpyted_data: encrypted_aes_data.to_string(),
        }))
    }
}
