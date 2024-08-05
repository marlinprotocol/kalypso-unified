use serde::{Deserialize, Serialize};

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
