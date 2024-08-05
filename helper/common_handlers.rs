use crate::response::response;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{post, web, Responder};
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use validator::Validate;

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct SignAddress {
    #[validate(required(message = "address was not provided in the JSON body"))]
    pub address: Option<String>,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct SignAttestation {
    #[validate(required(message = "attestation bytes were not provided in the JSON body"))]
    pub attestation: Option<String>,
    #[validate(required(message = "address was not provided in the JSON body"))]
    pub address: Option<String>,
}

impl SignAttestation {
    #[allow(unused)]
    pub fn new(attestation: &str, address: &str) -> SignAttestation {
        SignAttestation {
            attestation: Some(attestation.to_string()),
            address: Some(address.to_string()),
        }
    }
}

// Sign Address
#[post("/signAddress")]
async fn sign_address(
    jsonbody: web::Json<SignAddress>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    //Validating inputs
    let json_input = &jsonbody.0;
    if let Err(err) = json_input.validate() {
        log::error!("{}", err);
        return response(
            "Invalid payload",
            StatusCode::BAD_REQUEST,
            Some(Value::String(err.to_string())),
        );
    }
    let addy_to_be_signed = json_input.address.as_ref().unwrap();
    let ecies_priv_key = {
        let key = ecies_priv_key.lock().unwrap().clone();
        hex::encode(key)
    };
    let signed = sign_addy(ecies_priv_key, addy_to_be_signed).await.unwrap();
    let signature = json!({
        "r": ethers::types::H256::from_uint(&signed.r),
        "s": ethers::types::H256::from_uint(&signed.s),
        "v": signed.v
    });
    response("Address signed", StatusCode::OK, Some(signature))
}

// Sign Attestaion
#[post("/signAttestation")]
async fn sign_attestation(
    jsonbody: web::Json<SignAttestation>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    // Validating inputs
    let json_input = &jsonbody.0;
    if let Err(err) = json_input.validate() {
        log::error!("{}", err);
        return response(
            "Invalid attestation",
            StatusCode::BAD_REQUEST,
            Some(Value::String(err.to_string())),
        );
    }

    let ecies_priv_key = {
        let key = ecies_priv_key.lock().unwrap().clone();
        hex::encode(key)
    };

    let signed = sign_attest(ecies_priv_key, jsonbody.0).await.unwrap();
    let signature = json!({
        "r": ethers::types::H256::from_uint(&signed.r),
        "s": ethers::types::H256::from_uint(&signed.s),
        "v": signed.v
    });
    response("Attestation signed", StatusCode::OK, Some(signature))
}

async fn sign_addy(
    ecies_private_key: String,
    address: &str,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let signer = ecies_private_key.clone().parse::<LocalWallet>().unwrap();
    let values = vec![ethers::abi::Token::Address(Address::from_str(address)?)];
    let encoded = ethers::abi::encode(&values);
    let digest = ethers::utils::keccak256(encoded);
    let signature = signer.sign_message(ethers::types::H256(digest)).await?;
    Ok(signature)
}

async fn sign_attest(
    ecies_private_key: String,
    attestation: SignAttestation,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let signer = ecies_private_key.parse::<LocalWallet>().unwrap();
    let attestation_bytes = attestation.attestation.unwrap();
    let attestation_string: Vec<&str> = attestation_bytes.split('x').collect();
    let attestation_decoded = hex::decode(attestation_string[1]).unwrap();
    let address = attestation.address.unwrap();
    let values = vec![
        ethers::abi::Token::Bytes(attestation_decoded),
        ethers::abi::Token::Address(Address::from_str(&address)?),
    ];
    let encoded = ethers::abi::encode(&values);
    let digest = ethers::utils::keccak256(encoded);
    let signature = signer.sign_message(ethers::types::H256(digest)).await?;
    Ok(signature)
}
