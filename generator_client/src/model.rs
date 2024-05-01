use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

pub struct SupervisordResponse {
    pub output: String,
    pub status: bool,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ApiKeyFile {
    pub api_key: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GeneratorConfig {
    pub address: String,
    pub ecies_private_key: String,
    pub data: String,
    pub supported_markets: Vec<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct MarketDetails {
    pub port: String,
    pub ivs_url: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct RuntimeConfig {
    pub ws_url: String,
    pub http_url: String,
    pub private_key: String,
    pub proof_market_place: String,
    pub generator_registry: String,
    pub start_block: i32,
    pub chain_id: i32,
    pub params_path: String,
    pub payment_token: String,
    pub staking_token: String,
    pub attestation_verifier: String,
    pub entity_registry: String,
    pub markets: HashMap<String, MarketDetails>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GeneratorConfigFile {
    pub generator_config: Vec<GeneratorConfig>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct RuntimeConfigFile {
    pub runtime_config: RuntimeConfig,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct SetupRequestBodyGeneratorConfig {
    #[validate(
        contains(pattern = "0x", message = "The address should start with 0x"),
        required(message = "address was not provided in the generator details.")
    )]
    pub address: Option<String>,
    #[validate(required(message = "data was not provided in the generator details."))]
    pub data: Option<String>,
    #[validate(required(
        message = "supported_markets was not provided in the generator details."
    ))]
    pub supported_markets: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct SetupRequestBodyRuntimeConfig {
    #[validate(
        required(message = "A valid wss rpc url (ws_url) was not provided in the runtime_config"),
        contains(pattern = "wss", message = "wss url is required")
    )]
    pub ws_url: Option<String>,
    #[validate(
        required(
            message = "A valid http rpc url (http_url) was not provided in the runtime_config"
        ),
        contains(pattern = "http", message = "wss url is required")
    )]
    pub http_url: Option<String>,
    #[validate(required(message = "private_key was not provided in the runtime_config"))]
    pub private_key: Option<String>,
    #[validate(required(message = "proof_market_place was not provided in the runtime_config"))]
    pub proof_market_place: Option<String>,
    #[validate(required(message = "generator_registry was not provided in the runtime_config"))]
    pub generator_registry: Option<String>,
    #[validate(
        range(min = 1, message = "Block number should be at least 1"),
        required(message = "start_block was not provided in the runtime_config")
    )]
    pub start_block: Option<i32>,
    #[validate(
        range(min = 1, message = "Chain ID should be at least 1"),
        required(message = "chain_id was not provided in the runtime_config")
    )]
    pub chain_id: Option<i32>,
    pub payment_token: Option<String>,
    #[validate(required(message = "staking_token was not provided in the runtime_config"))]
    pub staking_token: Option<String>,
    #[validate(required(message = "attestation_verifier was not provided in the runtime_config"))]
    pub attestation_verifier: Option<String>,
    #[validate(required(message = "entity_registry was not provided in the runtime_config"))]
    pub entity_registry: Option<String>,
    pub markets: HashMap<String, MarketDetails>,
}

#[derive(Serialize, Validate, Deserialize)]
pub struct GeneratorConfigSetupRequestBody {
    #[validate(required(message = "generator_config was not provided in the JSON body"))]
    pub generator_config: Option<Vec<SetupRequestBodyGeneratorConfig>>,
    #[validate(required(message = "runtime_config was not provided in the JSON body"))]
    pub runtime_config: Option<SetupRequestBodyRuntimeConfig>,
}

#[derive(Serialize, Validate, Deserialize)]
pub struct GetRsaPublicKeyRequest {
    #[validate(
        contains(pattern = "0x", message = "The address should start with 0x"),
        required(message = "generator_address was not provided in the JSON body")
    )]
    pub generator_address: Option<String>,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct UpdateRuntimeConfig {
    #[validate(contains(pattern = "wss", message = "wss url is required"))]
    pub ws_url: Option<String>,
    #[validate(contains(pattern = "http", message = "http url is required"))]
    pub http_url: Option<String>,
    pub private_key: Option<String>,
    pub proof_market_place: Option<String>,
    pub generator_registry: Option<String>,
    pub start_block: Option<i32>,
    pub chain_id: Option<i32>,
    pub payment_token: Option<String>,
    pub staking_token: Option<String>,
    pub attestation_verifier: Option<String>,
    pub entity_registry: Option<String>,
    pub markets: Option<HashMap<String, MarketDetails>>,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct AddNewGenerator {
    #[validate(
        contains(pattern = "0x", message = "The address should start with 0x"),
        required(message = "address was not provided in the JSON body")
    )]
    pub address: Option<String>,
    #[validate(required(message = "data was not provided in the JSON body"))]
    pub data: Option<String>,
    #[validate(required(message = "supported_markets array was not provided in the JSON body"))]
    pub supported_markets: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct RemoveGenerator {
    #[validate(
        contains(pattern = "0x", message = "The address should start with 0x"),
        required(message = "address was not provided in the JSON body")
    )]
    pub address: Option<String>,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct UpdateGeneratorConfig {
    #[validate(
        contains(pattern = "0x", message = "The address should start with 0x"),
        required(message = "address was not provided in the JSON body")
    )]
    pub address: Option<String>,
    pub data: Option<String>,
    pub supported_markets: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct GetGeneratorPublicKeys {
    #[validate(
        contains(pattern = "0x", message = "The address should start with 0x"),
        required(message = "generator_address was not provided in the JSON body")
    )]
    pub generator_address: Option<String>,
}

pub struct ValidationResponse {
    pub status: bool,
    pub message: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GeneratorPublicKeys {
    pub generator_public_key: String,
    pub generator_ecies_public_key: String,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct SignAddress {
    #[validate(required(message = "address was not provided in the JSON body"))]
    pub address: Option<String>,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct SupervisordInputBody {
    #[validate(required(message = "program_name was not provided in the JSON body"))]
    pub program_name: Option<String>,
}

#[derive(Serialize, Debug, Validate, Deserialize)]
pub struct SignAttestation {
    #[validate(required(message = "attestation bytes were not provided in the JSON body"))]
    pub attestation: Option<String>,
    #[validate(required(message = "address was not provided in the JSON body"))]
    pub address: Option<String>,
}
