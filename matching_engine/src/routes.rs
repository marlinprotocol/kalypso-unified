use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::types::{Address, U256, U64};
use ethers::middleware::SignerMiddleware;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use hex::decode;
use secret_input_helpers::secret_inputs_helpers;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::ask::*;

use crate::generator::{GeneratorState, GeneratorStore};
use crate::utility;
use crate::utility::ivs_family_id;
use crate::utility::public_key_to_address;

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

    Ok(HttpResponse::Ok().json(GetLatestBlockNumberResponse {
        block_number: latest_parsed_block.to_string(),
    }))
}

#[derive(Deserialize)]
pub struct GetPrivInput {
    ask_id: String,
    ivs_pubkey: String,
    signature: String,
}

#[derive(Serialize)]
pub struct GetRequestResponse {
    encrpyted_data: String,
}

pub async fn get_priv_input(
    _payload: web::Json<GetPrivInput>,
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
    _matching_engine_key: Data<Arc<Mutex<Vec<u8>>>>,
    _entity_key_registry: Data<
        Arc<
            Mutex<
                bindings::entity_key_registry::EntityKeyRegistry<
                    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
                >,
            >,
        >,
    >,
) -> actix_web::Result<HttpResponse> {
    let local_ask_store = { _local_ask_store.lock().await };
    let ask_id: String = _payload.ask_id.clone();
    let ask_id_u256: U256 = U256::from_dec_str(&ask_id).expect("Failed to parse string");

    let local_ask: Option<&LocalAsk> = local_ask_store.get_by_ask_id(&ask_id_u256);

    if !local_ask.unwrap().has_private_inputs {
        return Ok(HttpResponse::BadRequest().json(json!({
            "status": "invalid"
        })));
    }

    let matching_engine_key = _matching_engine_key.lock().await;
    let entity_key_registry = _entity_key_registry.lock().await;
    let signer = utility::derive_address_from_signature(&_payload.signature, &_payload.ask_id)
        .expect("Failed to recover signature");

    let ivs_pubkey: String = _payload.ivs_pubkey.clone();

    if public_key_to_address(&ivs_pubkey).unwrap() != signer {
        return Ok(HttpResponse::BadRequest().json(json!({
            "status": "invalid key ivs"
        })));
    }

    let ivs_pubkey_vec = decode(ivs_pubkey).unwrap();

    let image = entity_key_registry
        .get_verified_key(signer)
        .call()
        .await
        .unwrap();

    let image_blacklisted = entity_key_registry
        .black_listed_images(image)
        .call()
        .await
        .unwrap();

    if image_blacklisted {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "status": "BlackListed"
        })));
    }

    let family_id = ivs_family_id(&ask_id);

    let result = entity_key_registry
        .allow_only_verified_family(family_id, signer)
        .call()
        .await;

    match result {
        Ok(_) => {
            println!("Image in family");
        }
        Err(err) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "status": "ImageNotInFamily"
            })))
        }
    }

    let decrypted_secret_data = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
        &local_ask.unwrap().secret_data.clone().unwrap(),
        &local_ask.unwrap().secret_acl.clone().unwrap(),
        &matching_engine_key.clone(),
        local_ask.unwrap().market_id,
    )
    .expect("Failed to get private inputs for the ask id");

    let encrypted_ecies_data =
        secret_inputs_helpers::encrypt_ecies(&decrypted_secret_data, &ivs_pubkey_vec).unwrap();

    let serialized = serde_json::to_string(&encrypted_ecies_data).unwrap();

    Ok(HttpResponse::Ok().json(GetRequestResponse {
        encrpyted_data: serialized,
    }))
}

#[derive(Deserialize)]
pub struct DecryptRequest {
    market_id: String,
    private_input: String,
    acl: String,
    signature: String,
    ivs_pubkey: String,
}

pub async fn decrypt_request(
    _payload: web::Json<DecryptRequest>,
    _market_store: Data<Arc<Mutex<MarketMetadataStore>>>,
    _matching_engine_key: Data<Arc<Mutex<Vec<u8>>>>,
    _entity_key_registry: Data<
        Arc<
            Mutex<
                bindings::entity_key_registry::EntityKeyRegistry<
                    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
                >,
            >,
        >,
    >,
) -> actix_web::Result<HttpResponse> {
    let entity_key_registry = _entity_key_registry.lock().await;
    let signer = utility::derive_address_from_signature(&_payload.signature, &_payload.market_id)
        .expect("Failed to recover signature");

    let ivs_pubkey: String = _payload.ivs_pubkey.clone();
    let ivs_pubkey_vec = hex::decode(ivs_pubkey.clone()).expect("invalid_ivs_key");

    if public_key_to_address(&ivs_pubkey.clone()).unwrap() != signer {
        return Ok(HttpResponse::BadRequest().json(json!({
            "status": "invalid key ivs"
        })));
    }

    let image = entity_key_registry
        .get_verified_key(signer)
        .call()
        .await
        .unwrap();

    let image_blacklisted = entity_key_registry
        .black_listed_images(image)
        .call()
        .await
        .unwrap();

    if image_blacklisted {
        return Ok(HttpResponse::Unauthorized().json(GetRequestResponse {
            encrpyted_data: "BlackListed".to_string(),
        }));
    }

    let market_store = _market_store.lock().await;
    let market_id: String = _payload.market_id.clone();
    let market_id_u256: U256 = U256::from_dec_str(&market_id).expect("Failed to parse string");

    let family_id = ivs_family_id(&market_id);

    let result = entity_key_registry
        .allow_only_verified_family(family_id, signer)
        .call()
        .await;

    match result {
        Ok(_) => {
            println!("Image in family");
        }
        Err(err) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "status": "ImageNotInFamily"
            })))
        }
    }
    let market = market_store.get_market_by_market_id(&market_id_u256);

    let image_id = market.unwrap().ivs_image_id;

    if image_id != image {
        return Ok(HttpResponse::Unauthorized().json(GetRequestResponse {
            encrpyted_data: "Image ID Mismatch".to_string(),
        }));
    }

    let secret_data = hex::decode(_payload.private_input.clone()).expect("invalid_data");
    let acl = hex::decode(_payload.acl.clone()).expect("invalid acl data");
    let matching_engine_key = _matching_engine_key.lock().await;
    let decrypted_secret_data = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
        &secret_data,
        &acl,
        &matching_engine_key.clone(),
        market_id_u256,
    )
    .expect("Failed to get private inputs for the ask id");

    let encrypted_ecies_data =
        secret_inputs_helpers::encrypt_ecies(&ivs_pubkey_vec, &decrypted_secret_data).unwrap();

    let serialized = serde_json::to_string(&encrypted_ecies_data).unwrap();

    Ok(HttpResponse::Ok().json(GetRequestResponse {
        encrpyted_data: serialized,
    }))
}

#[derive(Deserialize)]
pub struct MarketInfo {
    market_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct AskInfoToSend {
    pub ask_id: U256,
    pub market_id: U256,
    pub reward: U256,
    pub expiry: U256,
    pub proving_time: U256,
    pub deadline: U256,
    pub has_private_inputs: bool,
    pub state: Option<AskState>,
    pub generator: Option<Address>,
}

#[derive(Serialize)]
pub struct MarketInfoResponse {
    market_info: String,
    asks: Option<Vec<AskInfoToSend>>,
    generator_info: Option<GeneratorsInfoForMarket>,
}

#[derive(Serialize)]
pub struct GeneratorInfo {
    generator_address: Address,
    stake_locked: U256,
    total_stake: U256,
    compute_consumed: U256,
    declared_compute: U256,
    compute_required_per_request: U256,
    proof_generation_cost: U256,
    proposed_time: U256,
    active_requests: U256,
    proofs_submitted: U256,
    state: Option<GeneratorState>,
}

#[derive(Serialize)]
pub struct GeneratorsInfoForMarket {
    count: i32,
    generators: Vec<GeneratorInfo>,
}

pub async fn market_info(
    _payload: web::Json<MarketInfo>,
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
    _generator_store: Data<Arc<Mutex<GeneratorStore>>>,
) -> actix_web::Result<HttpResponse> {
    let market_id: String = _payload.market_id.clone();
    let market_id_u256 = U256::from_dec_str(&market_id);

    if market_id_u256.is_err() {
        return Ok(HttpResponse::BadRequest().json(MarketInfoResponse {
            market_info: "invalid market id".into(),
            asks: None,
            generator_info: None,
        }));
    }

    let market_id_u256 = market_id_u256.unwrap();

    let asks = {
        let local_ask_store = _local_ask_store.lock().await;
        let asks = local_ask_store
            .get_by_market_id(&market_id_u256)
            .sort_by_ask_id()
            .result();

        if asks.is_none() {
            None
        } else {
            let asks = asks.unwrap();
            let asks: Vec<AskInfoToSend> = asks
                .iter()
                .map(|ask| AskInfoToSend {
                    ask_id: ask.ask_id,
                    market_id: ask.market_id,
                    reward: ask.reward,
                    expiry: ask.expiry,
                    proving_time: ask.proving_time,
                    deadline: ask.deadline,
                    has_private_inputs: ask.has_private_inputs,
                    state: ask.state,
                    generator: ask.generator,
                })
                .collect();
            Some(asks)
        }
    };

    let generator_info = {
        let generator_store = _generator_store.lock().await;
        let all_generators = generator_store.clone().all_generators_address();

        let mut count = 0;
        let mut generators = vec![];
        for generator in all_generators {
            if let Some(generator_info) =
                generator_store.get_by_address_and_market(&generator, &market_id_u256)
            {
                let generator_data = { generator_store.get_by_address(&generator).unwrap() };
                count += 1;
                generators.push(GeneratorInfo {
                    generator_address: generator,
                    stake_locked: generator_data.stake_locked,
                    total_stake: generator_data.total_stake,
                    compute_consumed: generator_data.compute_consumed,
                    declared_compute: generator_data.declared_compute,
                    compute_required_per_request: generator_info.compute_required_per_request,
                    proof_generation_cost: generator_info.proof_generation_cost,
                    proposed_time: generator_info.proposed_time,
                    active_requests: generator_info.active_requests,
                    proofs_submitted: generator_info.proofs_submitted,
                    state: generator_info.state,
                })
            }
        }
        Some(GeneratorsInfoForMarket { count, generators })
    };

    Ok(HttpResponse::Ok().json(MarketInfoResponse {
        market_info: market_id.into(),
        asks,
        generator_info,
    }))
}
