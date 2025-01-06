use crate::common_deps::CommonDeps;
use crate::operations::matching_engine_programs::verify_attestation_with_pcrs;
use crate::operations::Operation;
use async_trait::async_trait;

use ethers::types::U256;
use futures::StreamExt;
use kalypso_helper::pcr_helpers::AttestationVerifierResponse;
use reqwest::Client;
use std::collections::HashMap;
use std::error::Error;

pub struct NonConfidentialMarketPcrs;

#[async_trait]
impl Operation for NonConfidentialMarketPcrs {
    async fn execute(&self, _config: HashMap<String, String>) -> Result<(), String> {
        let image_id = non_confidential_market_pcrs();
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
        let attestation_stream = kalypso_helper::pcr_helpers::build_attestation(
            &compute_pcrs_info.attestation_utility,
            false,
        )
        .await
        .map_err(|e| format!("Failed Building Attestations: {}", e))?;

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
        .map_err(|e| format!("Failed Computing Image ID. {}", e))?;

        print!("Image ID: \n{}", image_id);

        println!("\n\nSave Image For Further");

        Ok(())
    }
}

// Function to get attestation by sending attestation_data to the verifier
pub async fn get_image_id(
    base_url: &str,
    attestation_data: Vec<u8>,
    print_logs: bool,
) -> Result<String, Box<dyn Error>> {
    // Construct the verify endpoint URL
    let verify_endpoint = kalypso_helper::pcr_helpers::utility_url(base_url, "/verify/raw");

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
        ethers::abi::Token::Bytes(pcr0_vec.clone().into()),
        ethers::abi::Token::Bytes(pcr1_vec.clone().into()),
        ethers::abi::Token::Bytes(pcr2_vec.clone().into()),
    ]);

    let hashed_image_id = kalypso_helper::image_id_helpers::get_kalypso_image_id_from_pcrs(
        pcr0_vec.into(),
        pcr1_vec.into(),
        pcr2_vec.into(),
    );

    println!(
        "Hashed Image ID: {}",
        format!("0x{}", hex::encode(hashed_image_id))
    );

    Ok(format!("0x{}", hex::encode(encoded)))
}

pub struct ReadAttestation;

#[async_trait]
impl Operation for ReadAttestation {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let read_attestation_info = CommonDeps::read_attestation_info(&config)?;

        let attestation_stream = kalypso_helper::pcr_helpers::build_attestation(
            &read_attestation_info.attestation_utility,
            false,
        )
        .await
        .map_err(|e| format!("Failed Building Attestations: {}", e))?;

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

pub fn non_confidential_market_pcrs() -> String {
    let non_confidential_pcrs = non_confidential_pcrs();

    let encoded = ethers::abi::encode(&[
        ethers::abi::Token::Bytes(non_confidential_pcrs.pcr0_vec),
        ethers::abi::Token::Bytes(non_confidential_pcrs.pcr1_vec),
        ethers::abi::Token::Bytes(non_confidential_pcrs.pcr2_vec),
    ]);

    let image_id = format!("0x{}", hex::encode(encoded));

    image_id
}

pub struct PCRS {
    pub pcr0_vec: Vec<u8>,
    pub pcr1_vec: Vec<u8>,
    pub pcr2_vec: Vec<u8>,
}
pub fn non_confidential_pcrs() -> PCRS {
    let pcr0_vec = vec![0; 48];
    let pcr1_vec = vec![0; 48];
    let pcr2_vec = vec![0; 48];

    PCRS {
        pcr0_vec,
        pcr1_vec,
        pcr2_vec,
    }
}

pub struct VerifyAttestion;

#[async_trait]
impl Operation for VerifyAttestion {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let verify_attestation_info = CommonDeps::verify_remote_attestation_info(&config)?;

        let attestation = kalypso_helper::pcr_helpers::build_attestation_vec(
            &verify_attestation_info.attestation_utility,
            false,
        )
        .await
        .map_err(|e| format!("Failed making building attestation {}", e))?;

        let keys_after_verification =
            verify_attestation_with_pcrs(&verify_attestation_info.enclave_pcrs, &attestation)
                .await
                .map_err(|e| format!("Failed Verifying attestation {}", e))?;

        println!(
            "Verified Enclave Pubkey: {}",
            hex::encode(keys_after_verification)
        );
        Ok(())
    }
}
