use async_trait::async_trait;
use ethers::prelude::*;
use ethers::{abi::ParamType, core::rand::rngs::OsRng, types::U64};
use reqwest::header::{HeaderMap, HeaderValue};
use std::{collections::HashMap, error::Error};

use crate::common_deps::CommonDeps;
use crate::send_with_optional_gas;

use super::update_encryption_key::get_address_signature;
use super::{update_generator_meta::read_file_from_paths, Operation};

pub struct LoadMatchingEngineConfig;

#[async_trait]
impl Operation for LoadMatchingEngineConfig {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let start_program_info = CommonDeps::start_matching_engine_program_info(&config)?;

        let attestation = kalypso_helper::pcr_helpers::build_attestation_vec_raw(
            &start_program_info.matching_engine_attestation_utility,
            false,
        )
        .await
        .map_err(|e| format!("Failed making building attestation {}", e))?;

        let public_key_after_verification =
            verify_attestation_with_pcrs(&start_program_info.matching_engine_pcrs, &attestation)
                .await
                .map_err(|e| format!("{}", e))?;

        load_matching_engine_config(
            start_program_info.matching_engine_client_url,
            &public_key_after_verification,
            start_program_info.chain_id,
        )
        .await
        .map_err(|e| format!("Failed to start program in the enclave: {}", e))?;

        Ok(())
    }
}

pub struct StartProgam;

#[async_trait]
impl Operation for StartProgam {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let start_program_info = CommonDeps::start_matching_engine_program_info(&config)?;

        let attestation = kalypso_helper::pcr_helpers::build_attestation_vec_raw(
            &start_program_info.matching_engine_attestation_utility,
            false,
        )
        .await
        .map_err(|e| format!("Failed making building attestation {}", e))?;

        let public_key_after_verification =
            verify_attestation_with_pcrs(&start_program_info.matching_engine_pcrs, &attestation)
                .await
                .map_err(|e| format!("{}", e))?;

        start_program_encrypted(
            start_program_info.matching_engine_client_url,
            &public_key_after_verification,
            start_program_info.chain_id,
        )
        .await
        .map_err(|e| format!("Failed to start program in the enclave: {}", e))?;

        Ok(())
    }
}

pub struct StopProgram;

#[async_trait]
impl Operation for StopProgram {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        // start program and stop program require same args
        let start_program_info = CommonDeps::start_matching_engine_program_info(&config)?;

        let attestation = kalypso_helper::pcr_helpers::build_attestation_vec_raw(
            &start_program_info.matching_engine_attestation_utility,
            false,
        )
        .await
        .map_err(|e| format!("Failed making building attestation {}", e))?;

        let public_key_after_verification =
            verify_attestation_with_pcrs(&start_program_info.matching_engine_pcrs, &attestation)
                .await
                .map_err(|e| format!("{}", e))?;

        stop_program_encrypted(
            start_program_info.matching_engine_client_url,
            &public_key_after_verification,
            start_program_info.chain_id,
        )
        .await
        .map_err(|e| format!("Failed to start program in the enclave: {}", e))?;

        Ok(())
    }
}

pub async fn verify_attestation_with_pcrs(
    enclave_pcrs: &Vec<u8>,
    attestation: &Vec<u8>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let types = vec![ParamType::Bytes, ParamType::Bytes, ParamType::Bytes];

    // Decode the proof
    let decoded = ethers::abi::decode(&types, enclave_pcrs)
        .map_err(|e| format!("Failed decoding matching pcrs {}", e))?;
    let pcr0 = decoded[0]
        .clone()
        .into_bytes()
        .unwrap()
        .try_into()
        .expect("PCR0 must be exactly 48 bytes");
    let pcr1 = decoded[1]
        .clone()
        .into_bytes()
        .unwrap()
        .try_into()
        .expect("PCR1 must be exactly 48 bytes");
    let pcr2 = decoded[2]
        .clone()
        .into_bytes()
        .unwrap()
        .try_into()
        .expect("PCR2 must be exactly 48 bytes");

    // Create a fixed-size array containing the three PCRs
    let pcrs: [[u8; 48]; 3] = [pcr0, pcr1, pcr2];

    let public_key_after_verification =
        kalypso_helper::pcr_helpers::verify(attestation.to_vec(), pcrs, 1000)
            .map_err(|e| format!("Attestation Verification Failed {}", e))?;

    Ok(public_key_after_verification)
}

async fn start_program_encrypted(
    matching_engine_client_url: String,
    enclave_pub_key: &Vec<u8>,
    chain_id: U64,
) -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client with a timeout (optional but recommended)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let full_url = format!(
        "{}/api/startMatchingEngineEncrypted",
        matching_engine_client_url
    );

    // Prepare the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);

    let mut rng: OsRng = OsRng;
    let self_priv_key = ecies::SecretKey::random(&mut rng);

    let payload = serde_json::json!({});
    let encrypted_payload = kalypso_helper::sch_request::prepare_sch_payload(
        payload,
        enclave_pub_key,
        &self_priv_key.serialize(),
        &chain_id,
    )
    .await?;

    // Send the POST request
    let response = client
        .post(&full_url)
        .headers(headers)
        .json(&encrypted_payload)
        .send()
        .await?;

    // Capture the HTTP status before moving the response
    let status = response.status();

    // Read the response body as text
    let response_text = response.text().await?;

    let json_response: kalypso_helper::response::JsonResponse =
        match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                return Err(format!(
                    "Failed to parse JSON response. HTTP Status: {}, Body: {}, Error: {}",
                    status, response_text, e
                )
                .into())
            }
        };

    // Check the HTTP status and the JSON message
    if status.is_success() {
        println!("{:?}", json_response);
        Ok(())
    } else {
        // Return error with message
        Err(format!(
            "Failed to start program. HTTP Status: {}, Message: {}",
            status, json_response.message
        )
        .into())
    }
}

async fn stop_program_encrypted(
    matching_engine_client_url: String,
    enclave_pub_key: &Vec<u8>,
    chain_id: U64,
) -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client with a timeout (optional but recommended)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let full_url = format!(
        "{}/api/stopMatchingEngineEncrypted",
        matching_engine_client_url
    );

    // Prepare the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);

    let mut rng: OsRng = OsRng;
    let self_priv_key = ecies::SecretKey::random(&mut rng);

    let payload = serde_json::json!({});
    let encrypted_payload = kalypso_helper::sch_request::prepare_sch_payload(
        payload,
        enclave_pub_key,
        &self_priv_key.serialize(),
        &chain_id,
    )
    .await?;

    // Send the POST request
    let response = client
        .post(&full_url)
        .headers(headers)
        .json(&encrypted_payload)
        .send()
        .await?;

    // Capture the HTTP status before moving the response
    let status = response.status();

    // Read the response body as text
    let response_text = response.text().await?;

    let json_response: kalypso_helper::response::JsonResponse =
        match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                return Err(format!(
                    "Failed to parse JSON response. HTTP Status: {}, Body: {}, Error: {}",
                    status, response_text, e
                )
                .into())
            }
        };

    // Check the HTTP status and the JSON message
    if status.is_success() {
        println!("{:?}", json_response);
        Ok(())
    } else {
        // Return error with message
        Err(format!(
            "Failed to start program. HTTP Status: {}, Message: {}",
            status, json_response.message
        )
        .into())
    }
}

async fn load_matching_engine_config(
    matching_engine_client_url: String,
    enclave_pub_key: &Vec<u8>,
    chain_id: U64,
) -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client with a timeout (optional but recommended)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let full_url = format!(
        "{}/api/matchingEngineConfigSetupEncrypted",
        matching_engine_client_url
    );

    // Prepare the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);

    let mut rng: OsRng = OsRng;
    let self_priv_key = ecies::SecretKey::random(&mut rng);

    let matching_engine_config = ["./matching_engine_config.json"];

    let data = read_file_from_paths(&matching_engine_config)
        .map_err(|e| format!("Failed reading matching_engine_config.json {}", e))?;

    let matching_engine_config: matching_engine_client::model::MatchingEngineConfigSetupRequestBody =
            serde_json::from_str(&data).map_err(|e| {
                format!("Failed deserde matching_engine_config.json into MatchingEngineConfigSetupRequestBody {}", e)
            })?;

    let encrypted_payload = kalypso_helper::sch_request::prepare_sch_payload(
        matching_engine_config,
        enclave_pub_key,
        &self_priv_key.serialize(),
        &chain_id,
    )
    .await?;

    // Send the POST request
    let response = client
        .post(&full_url)
        .headers(headers)
        .json(&encrypted_payload)
        .send()
        .await?;

    // Capture the HTTP status before moving the response
    let status = response.status();

    // Read the response body as text
    let response_text = response.text().await?;

    let json_response: kalypso_helper::response::JsonResponse =
        match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                return Err(format!(
                    "Failed to parse JSON response. HTTP Status: {}, Body: {}, Error: {}",
                    status, response_text, e
                )
                .into())
            }
        };

    // Check the HTTP status and the JSON message
    if status.is_success() {
        println!("{:?}", json_response);
        Ok(())
    } else {
        // Return error with message
        Err(format!(
            "Failed to start program. HTTP Status: {}, Message: {}",
            status, json_response.message
        )
        .into())
    }
}

pub struct SetMatchingEngineImage;

#[async_trait]
impl Operation for SetMatchingEngineImage {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let set_image_info = CommonDeps::matching_engine_image_info(&config)?;

        let updater_role = set_image_info
            .proof_marketplace
            .updater_role()
            .call()
            .await
            .map_err(|e| format!("Failed Reading Updater Role from proof marketplace: {}", e))?;

        let has_role = set_image_info
            .proof_marketplace
            .has_role(updater_role.into(), set_image_info.signer.address())
            .await
            .map_err(|e| format!("Failed Reading Updater Role from proof marketplace: {}", e))?;

        if !has_role {
            return Err(format!(
                "{:?} does not have role to update the matching engine",
                set_image_info.signer.address()
            ));
        }

        let set_image_transaction = send_with_optional_gas!(set_image_info
            .proof_marketplace
            .set_matching_engine_image(set_image_info.matching_engine_pcrs.into()))
        .map_err(|e| format!("Set Matching Engine Image Transaction failed: {}", e))?;

        println!(
            "Set Matching Engine Image Transaction: {}",
            set_image_transaction
        );

        Ok(())
    }
}

pub struct VerifyMatchingEngineKeys;

#[async_trait]
impl Operation for VerifyMatchingEngineKeys {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let verify_matching_engine_config = CommonDeps::verify_matching_engine_keys_info(&config)?;

        let updater_role = verify_matching_engine_config
            .proof_marketplace
            .updater_role()
            .call()
            .await
            .map_err(|e| format!("Failed Reading Updater Role from proof marketplace: {}", e))?;

        let has_role = verify_matching_engine_config
            .proof_marketplace
            .has_role(
                updater_role.into(),
                verify_matching_engine_config.signer.address(),
            )
            .await
            .map_err(|e| format!("Failed Reading Updater Role from proof marketplace: {}", e))?;

        if !has_role {
            return Err(format!(
                "{:?} does not have role to update the matching engine",
                verify_matching_engine_config.signer.address()
            ));
        }

        let attestation = kalypso_helper::pcr_helpers::build_attestation_vec_raw(
            &verify_matching_engine_config.matching_engine_attestation_utility,
            false,
        )
        .await
        .map_err(|e| format!("Failed making building attestation {}", e))?;

        let keys_after_verification = verify_attestation_with_pcrs(
            &verify_matching_engine_config.matching_engine_pcrs,
            &attestation,
        )
        .await
        .map_err(|e| format!("Failed Verifying attestation {}", e))?;

        println!(
            "Verified Enclave Pubkey: {}",
            hex::encode(keys_after_verification)
        );

        let address_str = hex::encode(&verify_matching_engine_config.proof_marketplace.address());
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let address_signature = get_address_signature(
            &address_str,
            false,
            &verify_matching_engine_config.matching_engine_client_url,
            headers,
        )
        .await
        .map_err(|e| {
            format!(
                "Failed fetching the address signature for verifying the keys {}",
                e
            )
        })?;

        let verify_matching_engine_keys = send_with_optional_gas!(verify_matching_engine_config
            .proof_marketplace
            .verify_matching_engine(attestation.into(), address_signature.into()))
        .map_err(|e| format!("Verify Matching Engine Key Transaction failed: {}", e))?;

        println!(
            "Verify Matching Engine Keys Transaction: {}",
            verify_matching_engine_keys
        );

        Ok(())
    }
}
