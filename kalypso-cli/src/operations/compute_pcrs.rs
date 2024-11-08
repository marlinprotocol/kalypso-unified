use crate::common_deps::CommonDeps;
use crate::operations::Operation;
use async_trait::async_trait;
use bytes::Bytes;
use ethers::types::U256;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

pub struct NonConfidentialMarketPcrs;

#[async_trait]
impl Operation for NonConfidentialMarketPcrs {
    async fn execute(&self, _config: HashMap<String, String>) -> Result<(), String> {
        let pcr0_vec = vec![0; 48];
        let pcr1_vec = vec![0; 48];
        let pcr2_vec = vec![0; 48];

        let encoded = ethers::abi::encode(&[
            ethers::abi::Token::Bytes(pcr0_vec),
            ethers::abi::Token::Bytes(pcr1_vec),
            ethers::abi::Token::Bytes(pcr2_vec),
        ]);

        let image_id = format!("0x{}", hex::encode(encoded));

        print!("Image ID: \n{}", image_id);

        println!("\n\nSave Image For Further");

        Ok(())
    }
}

pub struct ComputePcrs;

#[async_trait]
impl Operation for ComputePcrs {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let compute_pcrs_info = CommonDeps::compute_pcrs_info(&config)?;
        let attestation_stream = build_attestation(&compute_pcrs_info.attestation_utility, false)
            .await
            .map_err(|_| "Failed Building Attestations.".to_string())?;

        let attestation_data: Vec<u8> = attestation_stream
            .fold(Vec::new(), |mut acc, item| async {
                match item {
                    Ok(bytes) => {
                        acc.extend_from_slice(&bytes);
                        acc
                    }
                    Err(e) => {
                        println!("Error while receiving data: {}", e);
                        acc
                    }
                }
            })
            .await;

        println!("Attestation Data Length: {}", attestation_data.len());

        let image_id = get_image_id(
            &compute_pcrs_info.attestation_verifier,
            attestation_data,
            false,
        )
        .await
        .map_err(|_| "Failed Computing Image ID".to_string())?;

        print!("Image ID: \n{}", image_id);

        println!("\n\nSave Image For Further");

        Ok(())
    }
}

pub async fn build_attestation(
    base_url: &str,
    print_logs: bool,
) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, Box<dyn Error>> {
    let attestation_end_point = utility_url(base_url, "/attestation/raw");

    if print_logs {
        println!("build attestation {}", attestation_end_point);
    }

    let client = Client::new();
    let response = client.get(&attestation_end_point).send().await?;

    // Check if the response status is successful (2xx)
    if !response.status().is_success() {
        println!("status code: {}", response.status());
        return Err("failed building the attestation".into());
    }

    // Get the response body as a stream of bytes
    let stream = response.bytes_stream();

    Ok(stream)
}

fn utility_url(base_url: &str, path: &str) -> String {
    format!("{}{}", base_url, path)
}

#[derive(Debug, Deserialize)]
struct AttestationVerifierResponse {
    signature: String,
    secp256k1_public: String,
    pcr0: String,
    pcr1: String,
    pcr2: String,
    timestamp: usize,
}

// Function to get attestation by sending attestation_data to the verifier
pub async fn get_image_id(
    base_url: &str,
    attestation_data: Vec<u8>,
    print_logs: bool,
) -> Result<String, Box<dyn Error>> {
    // Construct the verify endpoint URL
    let verify_endpoint = utility_url(base_url, "/verify/raw");

    if print_logs {
        println!("Sending attestation data to {}", verify_endpoint);
    }

    let client = Client::new();
    let response = client
        .post(&verify_endpoint)
        .header("Content-Type", "application/octet-stream")
        .body(attestation_data)
        .send()
        .await?;

    if !response.status().is_success() {
        if print_logs {
            println!(
                "Attestation verifier responded with status: {}",
                response.status()
            );
        }
        return Err("Failed to verify attestation".into());
    }

    let verifier_response: AttestationVerifierResponse = response.json().await?;

    if print_logs {
        println!("Fetched attestation successfully");
        println!("Verifier response: {:?}", verifier_response);
    }

    // Extract and process secp256k1_public
    let ecies_pubkey = format!("0x{}", verifier_response.secp256k1_public);
    if ecies_pubkey.len() != 130 {
        return Err("secp pub key length incorrect".into());
    }

    // Decode hex strings to bytes
    let signature_bytes = hex::decode(&verifier_response.signature.trim_start_matches("0x"))?;
    let pcr0_bytes = hex::decode(&verifier_response.pcr0.trim_start_matches("0x"))?;
    let pcr1_bytes = hex::decode(&verifier_response.pcr1.trim_start_matches("0x"))?;
    let pcr2_bytes = hex::decode(&verifier_response.pcr2.trim_start_matches("0x"))?;

    let _timestamp_u256 = U256::from(verifier_response.timestamp);
    let _signature_vec = signature_bytes;
    let _ecies_pubkey_vec =
        hex::decode(&verifier_response.secp256k1_public.trim_start_matches("0x"))?;

    let pcr0_vec = pcr0_bytes;
    let pcr1_vec = pcr1_bytes;
    let pcr2_vec = pcr2_bytes;

    let encoded = ethers::abi::encode(&[
        ethers::abi::Token::Bytes(pcr0_vec.into()),
        ethers::abi::Token::Bytes(pcr1_vec.into()),
        ethers::abi::Token::Bytes(pcr2_vec.into()),
    ]);

    Ok(format!("0x{}", hex::encode(encoded)))
}

pub struct ReadAttestation;

#[async_trait]
impl Operation for ReadAttestation {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let read_attestation_info = CommonDeps::read_attestation_info(&config)?;

        let attestation_stream =
            build_attestation(&read_attestation_info.attestation_utility, false)
                .await
                .map_err(|_| "Failed Building Attestations.".to_string())?;

        let attestation_data: Vec<u8> = attestation_stream
            .fold(Vec::new(), |mut acc, item| async {
                match item {
                    Ok(bytes) => {
                        acc.extend_from_slice(&bytes);
                        acc
                    }
                    Err(e) => {
                        println!("Error while receiving data: {}", e);
                        acc
                    }
                }
            })
            .await;

        println!("Attestation Data Length: {}\n", attestation_data.len());

        let attestation_data = format!("0x{}", hex::encode(attestation_data));

        print!("Attestation: \n{}", attestation_data);

        println!("\n\nSave Image For Further");
        Ok(())
    }
}

// Function to get attestation by sending attestation_data to the verifier
pub async fn get_verified_attestation(
    base_url: &str,
    attestation_data: Vec<u8>,
    print_logs: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Construct the verify endpoint URL
    let verify_endpoint = utility_url(base_url, "/verify/raw");

    if print_logs {
        println!("Sending attestation data to {}", verify_endpoint);
    }

    let client = Client::new();
    let response = client
        .post(&verify_endpoint)
        .header("Content-Type", "application/octet-stream")
        .body(attestation_data)
        .send()
        .await?;

    if !response.status().is_success() {
        if print_logs {
            println!(
                "Attestation verifier responded with status: {}",
                response.status()
            );
        }
        return Err("Failed to verify attestation".into());
    }

    let verifier_response: AttestationVerifierResponse = response.json().await?;

    if print_logs {
        println!("Fetched attestation successfully");
        println!("Verifier response: {:?}", verifier_response);
    }

    // Extract and process secp256k1_public
    let ecies_pubkey = format!("0x{}", verifier_response.secp256k1_public);
    if ecies_pubkey.len() != 130 {
        return Err("secp pub key length incorrect".into());
    }

    // Decode hex strings to bytes
    let signature_bytes = hex::decode(&verifier_response.signature.trim_start_matches("0x"))?;
    let pcr0_bytes = hex::decode(&verifier_response.pcr0.trim_start_matches("0x"))?;
    let pcr1_bytes = hex::decode(&verifier_response.pcr1.trim_start_matches("0x"))?;
    let pcr2_bytes = hex::decode(&verifier_response.pcr2.trim_start_matches("0x"))?;

    let timestamp_u256 = U256::from(verifier_response.timestamp);
    let signature_vec = signature_bytes;
    let ecies_pubkey_vec =
        hex::decode(&verifier_response.secp256k1_public.trim_start_matches("0x"))?;

    let pcr0_vec = pcr0_bytes;
    let pcr1_vec = pcr1_bytes;
    let pcr2_vec = pcr2_bytes;

    let encoded = ethers::abi::encode(&[
        ethers::abi::Token::Bytes(signature_vec.into()),
        ethers::abi::Token::Bytes(ecies_pubkey_vec.into()),
        ethers::abi::Token::Bytes(pcr0_vec.into()),
        ethers::abi::Token::Bytes(pcr1_vec.into()),
        ethers::abi::Token::Bytes(pcr2_vec.into()),
        ethers::abi::Token::Uint(timestamp_u256),
    ]);

    Ok(encoded)
}
