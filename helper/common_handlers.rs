use crate::response::response;
use crate::secret_inputs_helpers::{decrypt_ecies, encrypt_ecies};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{post, web, Responder};
use anyhow::Context;
use ethers::abi::{encode, Token};
use ethers::core::utils::keccak256;
use ethers::prelude::*;
use rand::rngs::OsRng;
use rand::RngCore;
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

        let signature = match Signature::try_from(&self.signature[..]) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        let recovered_address = match signature.recover(ethers::utils::hash_message(digest)) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        let hash = keccak256(&response_key[1..]);
        let expected_address = H160::from_slice(&hash[12..]).into();

        if recovered_address == expected_address {
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
    salt: Vec<u8>,
    signature: Vec<u8>,
}

pub trait ToSchResponse {
    fn to_sch_response(
        &self,
        response_payload: Value,
        enclave_key: Vec<u8>,
    ) -> impl std::future::Future<Output = anyhow::Result<SCHResponse>> + Send;
}

impl ToSchResponse for SCHPayload {
    async fn to_sch_response(
        &self,
        response_payload: Value,
        enclave_key: Vec<u8>,
    ) -> anyhow::Result<SCHResponse> {
        let json_str = serde_json::to_string(&response_payload)
            .context("Failed to serialize response payload to JSON")?;

        let utf8_bytes = json_str.as_bytes().to_vec();

        let encrypted_data = match encrypt_ecies(&self.response_key, &utf8_bytes) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        let salt = {
            let mut salt = vec![0u8; 64];
            OsRng.fill_bytes(&mut salt);
            salt
        };

        let values = vec![
            ethers::abi::Token::Bytes(encrypted_data.clone()),
            ethers::abi::Token::Bytes(salt.clone()),
        ];

        let encoded = ethers::abi::encode(&values);
        let digest = ethers::utils::keccak256(encoded);

        let enclave_key = hex::encode(&enclave_key);
        let enclave_signer = enclave_key.parse::<LocalWallet>().unwrap();

        let signature = match enclave_signer
            .sign_message(ethers::types::H256(digest))
            .await
        {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        Ok(SCHResponse {
            response: encrypted_data,
            salt,
            signature: signature.to_vec(),
        })
    }
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct SignAddress {
    #[validate(required(message = "address was not provided in the JSON body"))]
    pub address: Option<String>,
}

#[derive(Serialize, Debug, Validate, Deserialize, Clone)]
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
    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };

    let json_input: SignAddress = match jsonbody.0.to_payload(&ecies_priv_key) {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", &e.to_string());
            return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
        }
    };

    let signed_address = _sign_address(&json_input, &ecies_priv_key).await;

    if signed_address.is_some() {
        let signed_address = signed_address.unwrap();
        let sch_response = match jsonbody
            .0
            .to_sch_response(signed_address, ecies_priv_key.clone())
            .await
        {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", &e.to_string());
                return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
            }
        };

        return response(
            "Address signed",
            StatusCode::OK,
            Some(serde_json::to_value(&sch_response).unwrap()),
        );
    } else {
        return response("Address signing failed", StatusCode::BAD_REQUEST, None);
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

    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };

    let signed_attestation = _sign_attestation(json_input, &ecies_priv_key).await;

    if signed_attestation.is_some() {
        return response("Attestation signed", StatusCode::OK, signed_attestation);
    } else {
        return response("Attestation signing failed", StatusCode::BAD_REQUEST, None);
    }
}

// Sign Attestaion Encrypted
#[post("/signAttestationEncrypted")]
async fn sign_attestation_encrypted(
    jsonbody: web::Json<SCHPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };

    let json_input: SignAttestation = match jsonbody.0.to_payload(&ecies_priv_key) {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", &e.to_string());
            return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
        }
    };

    let signed_attestation = _sign_attestation(&json_input, &ecies_priv_key).await;

    if signed_attestation.is_some() {
        let signed_attestation = signed_attestation.unwrap();
        let sch_response = match jsonbody
            .0
            .to_sch_response(signed_attestation, ecies_priv_key.clone())
            .await
        {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", &e.to_string());
                return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
            }
        };

        return response(
            "Attestation signed",
            StatusCode::OK,
            Some(serde_json::to_value(&sch_response).unwrap()),
        );
    } else {
        return response("Attestation signing failed", StatusCode::BAD_REQUEST, None);
    }
}

async fn _sign_attestation(body: &SignAttestation, ecies_priv_key: &Vec<u8>) -> Option<Value> {
    let ecies_priv_key = hex::encode(ecies_priv_key);
    let signed = sign_attest(ecies_priv_key, body.clone()).await.unwrap();
    let signature = json!({
        "r": ethers::types::H256::from_uint(&signed.r),
        "s": ethers::types::H256::from_uint(&signed.s),
        "v": signed.v
    });

    Some(signature)
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
