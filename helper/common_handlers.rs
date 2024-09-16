use crate::response::response;
use crate::secret_inputs_helpers::{decrypt_ecies, encrypt_ecies};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{post, web, Responder};
use anyhow::Context;
use ethers::abi::{encode, Token};
use ethers::core::utils::keccak256;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use validator::Validate;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct SCHPayload {
    request: Vec<u8>,
    response_key: Vec<u8>,
    signature: Vec<u8>,
}

pub trait ToPayload<T> {
    fn to_payload(&self, ecies_priv_key: &Vec<u8>) -> anyhow::Result<T>;
}

impl<T> ToPayload<T> for SCHPayload
where
    T: for<'de> Deserialize<'de>,
{
    fn to_payload(&self, ecies_priv_key: &Vec<u8>) -> anyhow::Result<T> {
        let sch_payload = self.verify()?.decrypt(ecies_priv_key)?;
        let request_str = std::str::from_utf8(&sch_payload.request)
            .context("Failed to convert request to UTF-8 string")?;

        let payload: T = serde_json::from_str(request_str)
            .context("Failed to deserialize request into payload")?;

        Ok(payload)
    }
}

impl SCHPayload {
    fn verify(&self) -> anyhow::Result<Self> {
        let response_key = self.response_key.clone();
        let request = self.request.clone();

        let tokens = vec![
            Token::Bytes(request.clone()),
            Token::Bytes(response_key.clone()),
        ];

        let encoded = encode(&tokens);

        let digest = keccak256(&encoded);

        use ethers::types::Signature;
        use std::convert::TryFrom;

        let signature = match Signature::try_from(&self.signature[..]) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        let recovered_address = match signature.recover(digest) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        use ethers::core::k256::elliptic_curve::sec1::ToEncodedPoint;
        use ethers::core::k256::PublicKey as K256PublicKey;

        let response_pubkey = match K256PublicKey::from_sec1_bytes(&response_key) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        let response_pubkey_bytes = response_pubkey.to_encoded_point(false);
        let response_pubkey_uncompressed = &response_pubkey_bytes.as_bytes()[1..];
        let response_address = Address::from_slice(&keccak256(response_pubkey_uncompressed)[12..]);

        if recovered_address == response_address {
            Ok(self.clone())
        } else {
            Err(anyhow::Error::msg(
                "Signature verification failed.".to_string(),
            ))
        }
    }

    fn decrypt(&self, ecies_private_key: &[u8]) -> anyhow::Result<Self> {
        let request = match decrypt_ecies(ecies_private_key, &self.request) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        Ok(SCHPayload {
            request,
            response_key: self.response_key.clone(),
            signature: self.signature.clone(),
        })
    }
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct SCHResponse {
    response: Vec<u8>,
}

pub trait ToSchResponse {
    fn to_sch_response(&self, response_payload: Value) -> anyhow::Result<SCHResponse>;
}

impl ToSchResponse for SCHPayload {
    fn to_sch_response(&self, response_payload: Value) -> anyhow::Result<SCHResponse> {
        let json_str = serde_json::to_string(&response_payload)
            .context("Failed to serialize response payload to JSON")?;

        let utf8_bytes = json_str.as_bytes().to_vec();

        let encrypted_data = match encrypt_ecies(&utf8_bytes, &self.response_key) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        Ok(SCHResponse {
            response: encrypted_data,
        })
    }
}

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

    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };

    let signed_address = _sign_address(json_input, &ecies_priv_key).await;

    if signed_address.is_some() {
        response("Address signed", StatusCode::OK, signed_address)
    } else {
        response("Address signing failed", StatusCode::BAD_REQUEST, None)
    }
}

// Sign Address Encrypted
#[post("/signAddressEncrypted")]
async fn sign_address_encrypted(
    jsonbody: web::Json<SCHPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let ecies_priv_key = {
        let key = ecies_priv_key.lock().unwrap().clone();
        key
    };

    let json_input: SignAddress = match jsonbody.0.to_payload(&ecies_priv_key) {
        Ok(data) => data,
        Err(e) => return response(&e.to_string(), StatusCode::BAD_REQUEST, None),
    };

    let signed_address = _sign_address(&json_input, &ecies_priv_key).await;

    if signed_address.is_some() {
        let signed_address = signed_address.unwrap();
        let sch_response = match jsonbody.0.to_sch_response(signed_address) {
            Ok(data) => data,
            Err(e) => return response(&e.to_string(), StatusCode::BAD_REQUEST, None),
        };

        response(
            "Address signed",
            StatusCode::OK,
            Some(serde_json::to_value(&sch_response).unwrap()),
        )
    } else {
        response("Address signing failed", StatusCode::BAD_REQUEST, None)
    }
}

async fn _sign_address(body: &SignAddress, ecies_priv_key: &Vec<u8>) -> Option<Value> {
    let addy_to_be_signed = body.address.as_ref().unwrap();
    let ecies_priv_key = hex::encode(ecies_priv_key);
    let signed = sign_addy(ecies_priv_key, addy_to_be_signed).await.unwrap();
    let signature = json!({
        "r": ethers::types::H256::from_uint(&signed.r),
        "s": ethers::types::H256::from_uint(&signed.s),
        "v": signed.v
    });

    Some(signature)
    // response("Address signed", StatusCode::OK, )
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
