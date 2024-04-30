use serde::{Deserialize, Serialize};
use validator::Validate;

pub struct SupervisordResponse {
    pub output: String,
    pub status: bool,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ApiKeyFile {
    pub api_key: String,
}

pub struct ApiGenerationResponse {
    pub api_key: String,
    pub status: bool,
    pub message: String,
}

pub struct VerifyApiResponse {
    pub status: bool,
    pub message: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct IvsConfig {
    pub secp256k1_private_key: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct IvsPublicKeys {
    pub ivs_public_key: String,
    pub ivs_ecies_public_key: String,
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
    pub fn new(attestation: &str, address: &str) -> SignAttestation {
        SignAttestation {
            attestation: Some(attestation.to_string()),
            address: Some(address.to_string()),
        }
    }
}
