use std::collections::HashMap;

use async_trait::async_trait;
use ethers::{signers::Signer, types::H256};
use futures::StreamExt;

use crate::{common_deps::CommonDeps, operations::compute_pcrs};

use super::Operation;

pub struct AddIvsKey;
#[async_trait]
impl Operation for AddIvsKey {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let add_ivs_key_info = CommonDeps::add_ivs_key_info(&config)?;

        let attestation_stream =
            compute_pcrs::build_attestation(&add_ivs_key_info.attestation_utility, false)
                .await
                .map_err(|e| format!("Failed Building Attestations {}", e))?;

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

        let verified_attestation = compute_pcrs::get_verified_attestation(
            &add_ivs_key_info.attestation_verifier,
            attestation_data,
            false,
        )
        .await
        .map_err(|e| format!("Failed Verifying attestation {}", e))?;

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let enclave_signature = get_attestation_signature(
            hex::encode(&verified_attestation).as_ref(),
            hex::encode(
                add_ivs_key_info
                    .private_key_signer
                    .address()
                    .as_bytes()
                    .to_vec(),
            )
            .as_ref(),
            false,
            &add_ivs_key_info.enclave_client_url,
            headers,
        )
        .await
        .map_err(|e| format!("Failed Getting Attestation Signature {}", e))?;

        let add_ivskey_transaction = CommonDeps::send_and_confirm(
            add_ivs_key_info
                .generator_registry
                .add_ivs_key(
                    add_ivs_key_info.market_id,
                    verified_attestation.into(),
                    enclave_signature.into(),
                )
                .send(),
        )
        .await
        .map_err(|e| format!("Add IVS KEY Transaction failed: {}", e))?;

        println!("Add IVS Key Transaction: {}", add_ivskey_transaction);

        Ok(())
    }
}

pub struct UpdateEncryptionKey;

#[async_trait]
impl Operation for UpdateEncryptionKey {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let update_encryption_info = CommonDeps::update_encryption_info(&config)?;

        let market_data = update_encryption_info
            .proof_marketplace
            .market_data(update_encryption_info.market_id)
            .call()
            .await
            .map_err(|e| format!("Failed making call to proof marketplace contract {}", e))?;

        if H256::from_slice(&market_data.1.to_vec())
            == kalypso_helper::image_id_helpers::hashed_image_id_for_non_confidential_market()
        {
            return Err("non confidential markets don't need encryption key".to_string());
        }

        let attestation_stream =
            compute_pcrs::build_attestation(&update_encryption_info.attestation_utility, false)
                .await
                .map_err(|e| format!("Failed Building Attestations {}", e))?;

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

        let verified_attestation = compute_pcrs::get_verified_attestation(
            &update_encryption_info.attestation_verifier,
            attestation_data,
            false,
        )
        .await
        .map_err(|e| format!("Failed Verifying attestation {}", e))?;

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let enclave_signature = get_attestation_signature(
            hex::encode(&verified_attestation).as_ref(),
            hex::encode(
                update_encryption_info
                    .private_key_signer
                    .address()
                    .as_bytes()
                    .to_vec(),
            )
            .as_ref(),
            false,
            &update_encryption_info.enclave_client_url,
            headers,
        )
        .await
        .map_err(|e| format!("Failed Getting Attestation Signature {}", e))?;

        let update_encryption_key_transaction = CommonDeps::send_and_confirm(
            update_encryption_info
                .generator_registry
                .update_encryption_key(
                    update_encryption_info.market_id,
                    verified_attestation.into(),
                    enclave_signature.into(),
                )
                .send(),
        )
        .await
        .map_err(|e| format!("Update Encryption Key Transaction failed: {}", e))?;

        println!(
            "Update Encryption Key Transaction: {}",
            update_encryption_key_transaction
        );

        Ok(())
    }
}

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct SignAddressResponse {
    #[allow(unused)]
    message: String,
    data: SignAddressData,
}

#[derive(Deserialize)]
struct SignAddressData {
    r: String,
    s: String,
    v: u8,
}

#[allow(unused)]
pub async fn get_address_signature(
    address: &str,
    print_logs: bool,
    enclave_client_url: &str,
    headers: HeaderMap,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let full_url = format!("{}/api/signAddress", enclave_client_url);
    if print_logs {
        println!("Fetching signature from URL: {}", full_url);
    }

    let payload = serde_json::json!({ "address": format!("0x{}", address) });
    let client = reqwest::Client::new();
    let response = client
        .post(&full_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to send request: {}", e),
            )) as Box<dyn Error>
        })?;

    if !response.status().is_success() {
        return Err(format!("Error: {}", response.status()).into());
    }

    let response_json: SignAddressResponse = response.json().await?;
    let _v = match response_json.data.v {
        27 => "1b",
        28 => "1c",
        other => return Err(format!("Unexpected value for v: {}", other).into()),
    };

    let s_clean = response_json
        .data
        .s
        .strip_prefix("0x")
        .unwrap_or(&response_json.data.s);

    if !response_json.data.r.starts_with("0x") || s_clean.len() != 64 {
        return Err("Invalid `r` or `s` format in response".into());
    }

    let signature_str = format!(
        "{}{}{}",
        response_json.data.r.trim_start_matches("0x"),
        s_clean,
        _v
    );

    let signature_hex = signature_str.strip_prefix("0x").unwrap_or(&signature_str);
    let signature_bytes = hex::decode(signature_hex)?;
    Ok(signature_bytes)
}

pub async fn get_attestation_signature(
    attestation: &str,
    address: &str,
    print_logs: bool,
    enclave_client_url: &str,
    headers: HeaderMap,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let full_url = format!("{}/api/signAttestation", enclave_client_url);
    if print_logs {
        println!("Fetching signature from URL: {}", full_url);
    }

    let payload = serde_json::json!({ "attestation": format!("0x{}",attestation), "address": format!("0x{}", address) });
    let client = reqwest::Client::new();
    let response = client
        .post(&full_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to send request: {}", e),
            )) as Box<dyn Error>
        })?;

    if !response.status().is_success() {
        return Err(format!("Error: {}", response.status()).into());
    }

    let response_json: SignAddressResponse = response.json().await?;
    let _v = match response_json.data.v {
        27 => "1b",
        28 => "1c",
        other => return Err(format!("Unexpected value for v: {}", other).into()),
    };

    let s_clean = response_json
        .data
        .s
        .strip_prefix("0x")
        .unwrap_or(&response_json.data.s);

    if !response_json.data.r.starts_with("0x") || s_clean.len() != 64 {
        return Err("Invalid `r` or `s` format in response".into());
    }

    let signature_str = format!(
        "{}{}{}",
        response_json.data.r.trim_start_matches("0x"),
        s_clean,
        _v
    );

    let signature_hex = signature_str.strip_prefix("0x").unwrap_or(&signature_str);
    let signature_bytes = hex::decode(signature_hex)?;
    Ok(signature_bytes)
}
