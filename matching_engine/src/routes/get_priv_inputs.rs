use super::EntityRegistryInstance;
use super::GetRequestResponse;
use crate::ask::*;
use crate::utility;
use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::core::types::U256;
use hex::decode;
use kalypso_helper::secret_inputs_helpers;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct GetPrivInput {
    ask_id: String,
    ivs_pubkey: String,
    signature: String,
}

pub async fn get_priv_input(
    _payload: web::Json<GetPrivInput>,
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
    _matching_engine_key: Data<Arc<Mutex<Vec<u8>>>>,
    _entity_key_registry: EntityRegistryInstance,
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

    if utility::public_key_to_address(&ivs_pubkey).unwrap() != signer {
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

    let family_id = utility::ivs_family_id(&ask_id);

    let result = entity_key_registry
        .allow_only_verified_family(family_id, signer)
        .call()
        .await;

    match result {
        Ok(_) => {
            println!("Image in family");
        }
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "status": "ImageNotInFamily"
            })))
        }
    }

    let decrypted_secret_data = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
        &local_ask.unwrap().secret_data.clone().unwrap(),
        &local_ask.unwrap().secret_acl.clone().unwrap(),
        &matching_engine_key.clone(),
        Some(local_ask.unwrap().market_id),
    )
    .expect("Failed to get private inputs for the ask id");

    let encrypted_ecies_data =
        secret_inputs_helpers::encrypt_ecies(&decrypted_secret_data, &ivs_pubkey_vec).unwrap();

    let serialized = serde_json::to_string(&encrypted_ecies_data).unwrap();

    Ok(HttpResponse::Ok().json(GetRequestResponse {
        encrypted_data: serialized,
    }))
}
