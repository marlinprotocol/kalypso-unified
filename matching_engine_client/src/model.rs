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
pub struct MatchingEngineConfig {
    pub rpc_url: String,
    pub chain_id: String,
    pub matching_engine_key: String,
    pub relayer_private_key: String,
    pub proof_market_place: String,
    pub generator_registry: String,
    pub start_block: String,
    pub payment_token: String,
    pub platform_token: String,
    pub attestation_verifier: String,
    pub entity_registry: String,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct MatchingEngineConfigSetupRequestBody {
    #[validate(required(message = "rpc_url was not provided in the matching_engine_config"))]
    pub rpc_url: Option<String>,
    #[validate(required(message = "chain_id was not provided in the matching_engine_config"))]
    pub chain_id: Option<String>,
    #[validate(required(
        message = "relayer_private_key was not provided in the matching_engine_config"
    ))]
    pub relayer_private_key: Option<String>,
    #[validate(required(
        message = "proof_market_place was not provided in the matching_engine_config"
    ))]
    pub proof_market_place: Option<String>,
    #[validate(required(
        message = "generator_registry was not provided in the matching_engine_config"
    ))]
    pub generator_registry: Option<String>,
    #[validate(required(message = "start_block was not provided in the matching_engine_config"))]
    pub start_block: Option<String>,
    #[validate(required(
        message = "payment_token was not provided in the matching_engine_config"
    ))]
    pub payment_token: Option<String>,
    #[validate(required(
        message = "platform_token was not provided in the matching_engine_config"
    ))]
    pub platform_token: Option<String>,
    #[validate(required(
        message = "attestation_verifier was not provided in the matching_engine_config"
    ))]
    pub attestation_verifier: Option<String>,
    #[validate(required(
        message = "entity_registry was not provided in the matching_engine_config"
    ))]
    pub entity_registry: Option<String>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct UpdateMatchingEngineConfig {
    pub rpc_url: Option<String>,
    pub chain_id: Option<String>,
    pub relayer_private_key: Option<String>,
    pub proof_market_place: Option<String>,
    pub generator_registry: Option<String>,
    pub start_block: Option<String>,
    pub payment_token: Option<String>,
    pub platform_token: Option<String>,
    pub attestation_verifier: Option<String>,
    pub entity_registry: Option<String>,
}

pub struct ValidationResponse {
    pub status: bool,
    pub message: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct MatchingEnginePublicKeys {
    pub matching_engine_public_key: String,
    pub matching_engine_ecies_public_key: String,
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
