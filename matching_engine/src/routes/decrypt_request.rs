use crate::models::{DecryptRequest, GetRequestResponse};
use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::core::types::U256;
use kalypso_helper::secret_inputs_helpers;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::ask::*;
use crate::utility;

use super::EntityRegistryInstance;

pub async fn decrypt_request(
    _payload: web::Json<DecryptRequest>,
    _market_store: Data<Arc<Mutex<MarketMetadataStore>>>,
    _matching_engine_key: Data<Arc<Mutex<Vec<u8>>>>,
    _entity_key_registry: EntityRegistryInstance,
) -> actix_web::Result<HttpResponse> {
    let entity_key_registry = _entity_key_registry.lock().await;
    let signer = utility::derive_address_from_signature(&_payload.signature, &_payload.market_id)
        .expect("Failed to recover signature");

    let ivs_pubkey: String = _payload.ivs_pubkey.clone();
    let ivs_pubkey_vec = hex::decode(ivs_pubkey.clone()).expect("invalid_ivs_key");

    if utility::public_key_to_address(&ivs_pubkey.clone()).unwrap() != signer {
        return Ok(HttpResponse::BadRequest().json(json!({
            "status": "invalid key ivs"
        })));
    }

    let image_id_in_er = entity_key_registry
        .get_verified_key(signer)
        .call()
        .await
        .unwrap();

    let image_blacklisted = entity_key_registry
        .black_listed_images(image_id_in_er)
        .call()
        .await
        .unwrap();

    if image_blacklisted {
        return Ok(HttpResponse::Unauthorized().json(GetRequestResponse {
            encrypted_data: "BlackListed".to_string(),
        }));
    }

    let market_id: String = _payload.market_id.clone();
    let market_id_u256: U256 = U256::from_dec_str(&market_id).expect("Failed to parse string");

    let family_id = utility::ivs_family_id(&market_id);

    let result = entity_key_registry
        .allow_only_verified_family(family_id, signer)
        .call()
        .await;

    if result.is_err() {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "status": "ImageNotInFamily"
        })))
    }
    println!("Image in family");

    // locks must be dropped..
    // let market_store = _market_store.lock().await;

    // matching engine has no idea about the extra images that are being added to the market, they must be indexed first in pm.rs and then it's data to be used here
    // till then commenting out this part
    // let image_id_locally = match market_store.get_market_by_market_id(&market_id_u256) {
    //     Some(data) => data.ivs_image_id,
    //     None => {
    //         return Ok(HttpResponse::ExpectationFailed().json(json!({
    //             "status": "No Info About the market"
    //         })))
    //     }
    // };

    // dbg!(hex::encode(image_id_locally));
    // dbg!(hex::encode(image_id_in_er));
    // if image_id_locally != image_id_in_er {
    //     return Ok(HttpResponse::BadRequest().json(json!({
    //         "status": "Request doesn't originate from required IVS"
    //     })))
    // }

    let secret_data = hex::decode(_payload.private_input.clone()).expect("invalid_data");
    let acl = hex::decode(_payload.acl.clone()).expect("invalid acl data");
    let matching_engine_key = _matching_engine_key.lock().await;
    let decrypted_secret_data = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
        &secret_data,
        &acl,
        &matching_engine_key.clone(),
        Some(market_id_u256),
    )
    .expect("Failed to get decrypted inputs");

    let encrypted_ecies_data =
        secret_inputs_helpers::encrypt_ecies(&ivs_pubkey_vec, &decrypted_secret_data).unwrap();

    let serialized = hex::encode(encrypted_ecies_data);

    return Ok(HttpResponse::Ok().json(GetRequestResponse {
        encrypted_data: serialized,
    }))
}
