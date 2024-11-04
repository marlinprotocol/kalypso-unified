use crate::common_deps::CommonDeps;
use crate::operations::Operation;
use async_trait::async_trait;
use ethers::types::U256;
use serde::Deserialize;
use std::collections::HashMap;

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

use bytes::Bytes;
use futures::{Stream, StreamExt};
use hex::decode;
use reqwest::Client;
use std::error::Error;

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
    #[allow(unused)]
    secp256k1_public: String,
    #[allow(unused)]
    signature: String,
    #[allow(unused)]
    pcr0: String,
    #[allow(unused)]
    pcr1: String,
    #[allow(unused)]
    pcr2: String,
    #[allow(unused)]
    timestamp: u64,
}

use ethers::abi::{encode, Token};

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
    let signature_bytes = decode(&verifier_response.signature.trim_start_matches("0x"))?;
    let pcr0_bytes = decode(&verifier_response.pcr0.trim_start_matches("0x"))?;
    let pcr1_bytes = decode(&verifier_response.pcr1.trim_start_matches("0x"))?;
    let pcr2_bytes = decode(&verifier_response.pcr2.trim_start_matches("0x"))?;

    let _timestamp_u256 = U256::from(verifier_response.timestamp);
    let _signature_vec = signature_bytes;
    let _ecies_pubkey_vec = decode(&verifier_response.secp256k1_public.trim_start_matches("0x"))?;

    let pcr0_vec = pcr0_bytes;
    let pcr1_vec = pcr1_bytes;
    let pcr2_vec = pcr2_bytes;

    let encoded = encode(&[
        Token::Bytes(pcr0_vec),
        Token::Bytes(pcr1_vec),
        Token::Bytes(pcr2_vec),
    ]);

    Ok(format!("0x{}", hex::encode(encoded)))
}
