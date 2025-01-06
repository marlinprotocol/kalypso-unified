use bytes::Bytes;
use ethers::types::U256;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AttestationVerifierResponse {
    pub signature: String,
    pub secp256k1_public: String,
    pub pcr0: String,
    pub pcr1: String,
    pub pcr2: String,
    pub timestamp: usize,
}

pub fn utility_url(base_url: &str, path: &str) -> String {
    format!("{}{}", base_url, path)
}

pub async fn build_attestation(
    base_url: &str,
    print_logs: bool,
) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, Box<dyn Error>> {
    let attestation_end_point = utility_url(base_url, "/attestation/raw");

    build_attestation_raw(&attestation_end_point, print_logs).await
}

pub async fn build_attestation_raw(
    attestation_end_point: &str,
    print_logs: bool,
) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, Box<dyn Error>> {
    if print_logs {
        println!("build attestation {}", attestation_end_point);
    }

    let client = Client::new();
    let response = client.get(attestation_end_point).send().await?;

    // Check if the response status is successful (2xx)
    if !response.status().is_success() {
        println!("status code: {}", response.status());
        return Err("failed building the attestation".into());
    }

    // Get the response body as a stream of bytes
    let stream = response.bytes_stream();

    Ok(stream)
}

pub async fn build_attestation_vec(
    attestation_end_point: &str,
    print_logs: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let attestation_stream = build_attestation(attestation_end_point, print_logs)
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

    Ok(attestation_data)
}

pub async fn build_attestation_vec_raw(
    base_url: &str,
    print_logs: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let attestation_stream = build_attestation_raw(base_url, print_logs)
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

    Ok(attestation_data)
}

// Function to get attestation by sending attestation_data to the verifier
pub async fn get_verified_attestation(
    verifier_url: &str,
    attestation_data: Vec<u8>,
    print_logs: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Construct the verify endpoint URL
    let verify_endpoint = utility_url(verifier_url, "/verify/raw");

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

pub async fn verify_attestation(
    verifier_url: &str,
    attestation_data: Vec<u8>,
    print_logs: bool,
) -> Result<AttestationVerifierResponse, Box<dyn Error>> {
    let verify_endpoint = utility_url(verifier_url, "/verify/raw");

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

    Ok(verifier_response)
}

use std::collections::BTreeMap;

use aws_nitro_enclaves_cose::{crypto::Openssl, CoseSign1};
use serde_cbor::{self, value, value::Value};

pub fn parse_attestation_doc(
    attestation_doc: &[u8],
) -> Result<(CoseSign1, BTreeMap<Value, Value>, AttestationData), AttestationError> {
    let cosesign1 = CoseSign1::from_bytes(&attestation_doc)
        .map_err(|e| AttestationError::ParseFailed(format!("cose: {e}")))?;
    let payload = cosesign1
        .get_payload::<Openssl>(None)
        .map_err(|e| AttestationError::ParseFailed(format!("cose payload: {e}")))?;
    let cbor = serde_cbor::from_slice::<Value>(&payload)
        .map_err(|e| AttestationError::ParseFailed(format!("cbor: {e}")))?;
    let attestation_doc = value::from_value::<BTreeMap<Value, Value>>(cbor.clone())
        .map_err(|e| AttestationError::ParseFailed(format!("doc: {e}")))?;

    let another_attestation_doc = value::from_value::<AttestationData>(cbor)
        .map_err(|e| AttestationError::ParseFailed(format!("doc: {e}")))?;

    Ok((cosesign1, attestation_doc, another_attestation_doc))
}

pub fn verify_with_timestamp(
    attestation_doc_cbor: Vec<u8>,
    pcrs: [[u8; 48]; 3],
    timestamp: usize,
) -> Result<Vec<u8>, AttestationError> {
    oyster::verify_with_timestamp(attestation_doc_cbor, pcrs, timestamp)
}

pub fn verify(
    attestation_doc_cbor: Vec<u8>,
    pcrs: [[u8; 48]; 3],
    max_age: usize,
) -> Result<Vec<u8>, AttestationError> {
    oyster::verify(attestation_doc_cbor, pcrs, max_age)
}

#[cfg(test)]
mod tests {
    use super::{build_attestation_vec, parse_attestation_doc, verify_attestation};

    #[tokio::test]
    async fn test_verified_attestation_with_verifier() {
        let result = build_attestation_vec("http://3.110.146.109:1500", false).await;

        assert!(
            result.is_ok(),
            "Expected Ok(_), got Err({:?})",
            result.err()
        );

        if let Ok(attestation_vec) = result {
            assert!(
                !attestation_vec.is_empty(),
                "Attestation vector should not be empty"
            );

            let verified_result =
                verify_attestation("http://13.201.207.60:1400", attestation_vec, false).await;

            assert!(
                verified_result.is_ok(),
                "Expected Ok(_), got Err({:?})",
                verified_result.err()
            );

            match serde_json::to_string_pretty(&verified_result.unwrap()) {
                Ok(json_string) => {
                    // Step 3: Print the JSON string
                    println!("Verified Attestation as JSON:\n{}", json_string);
                }
                Err(e) => {
                    eprintln!("Error serializing Verified Attestation: {}", e);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_pcrs_decode() {
        let result = build_attestation_vec("http://3.110.146.109:1500", false).await;

        assert!(
            result.is_ok(),
            "Expected Ok(_), got Err({:?})",
            result.err()
        );

        if let Ok(attestation_vec) = result {
            assert!(
                !attestation_vec.is_empty(),
                "Attestation vector should not be empty"
            );
            let parsed_result = parse_attestation_doc(&attestation_vec);
            assert!(parsed_result.is_ok(), "Expected Ok(_)");

            let (_, _, doc) = parsed_result.unwrap();

            match serde_json::to_string_pretty(&doc.to_attestation_response()) {
                Ok(json_string) => {
                    // Step 3: Print the JSON string
                    println!("AttestationResponse as JSON:\n{}", json_string);
                }
                Err(e) => {
                    eprintln!("Error serializing AttestationResponse: {}", e);
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttestationData {
    pcrs: Option<BTreeMap<u32, Value>>,
    nonce: Option<Value>,
    digest: Option<Value>,
    cabundle: Option<Value>,
    module_id: Option<Value>,
    timestamp: Option<Value>,
    user_data: Option<Value>,
    public_key: Option<Value>,
    certificate: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttestationResponse {
    pcrs: Option<Vec<String>>,
    nonce: Option<()>,
    digest: Option<String>,
    cabundle: Option<String>,
    module_id: Option<String>,
    timestamp: Option<String>,
    user_data: Option<()>,
    public_key: Option<String>,
    certificate: Option<String>,
}

impl AttestationData {
    pub fn to_attestation_response(&self) -> AttestationResponse {
        let pcrs = if self.pcrs.is_none() {
            None
        } else {
            let pcrs = self.pcrs.clone().unwrap();

            let mut to_return = Vec::with_capacity(16);
            for (_, value) in pcrs.into_iter() {
                let pcr = match value {
                    Value::Bytes(vec) => hex::encode(vec),
                    _ => unimplemented!(),
                };
                to_return.push(pcr);
            }

            Some(to_return)
        };

        let nonce = if self.nonce.is_none() {
            None
        } else {
            unimplemented!()
        };

        let digest = if self.digest.is_none() {
            None
        } else {
            let digest = self.digest.clone().unwrap();
            let digest = match digest {
                Value::Text(val) => val,
                _ => unimplemented!(),
            };

            Some(digest)
        };

        let cabundle = if self.cabundle.is_none() {
            None
        } else {
            let result = process_cabundle(&self.cabundle.clone().unwrap()).unwrap();
            Some(result)
        };

        let module_id = if self.module_id.is_none() {
            None
        } else {
            let module_id = self.module_id.clone().unwrap();
            let module_id = match module_id {
                Value::Text(val) => val,
                _ => unimplemented!(),
            };

            Some(module_id)
        };

        let timestamp = if self.timestamp.is_none() {
            None
        } else {
            let timestamp = self.timestamp.clone().unwrap();
            let timestamp = match timestamp {
                Value::Integer(val) => val.to_string(),
                _ => unimplemented!(),
            };

            Some(timestamp)
        };

        let user_data = if self.user_data.is_none() { None } else { None };

        let public_key = if self.public_key.is_none() {
            None
        } else {
            let public_key = self.public_key.clone().unwrap();
            let public_key = match public_key {
                Value::Bytes(val) => hex::encode(val),
                _ => unimplemented!(),
            };

            Some(public_key)
        };

        let certificate = if self.certificate.is_none() {
            None
        } else {
            let certificate = self.certificate.clone().unwrap();
            let certificate = match certificate {
                Value::Bytes(val) => der_to_pem(&val),
                _ => unimplemented!(),
            };

            Some(certificate)
        };

        AttestationResponse {
            pcrs,
            nonce,
            digest,
            cabundle,
            module_id,
            timestamp,
            user_data,
            public_key,
            certificate,
        }
    }
}

fn process_cabundle(cabundle: &Value) -> Result<String, String> {
    match cabundle {
        Value::Array(cert_values) => {
            let mut pem_bundle = String::new();

            for cert_value in cert_values {
                match cert_value {
                    Value::Bytes(der_bytes) => {
                        let pem = der_to_pem(der_bytes);
                        pem_bundle.push_str(&pem);
                        pem_bundle.push('\n'); // Ensure separation between certificates
                    }
                    _ => return Err("Expected each CA bundle element to be Bytes".to_string()),
                }
            }

            Ok(pem_bundle)
        }
        _ => Err("Expected cabundle to be an Array".to_string()),
    }
}

use pem::{encode as pem_encode, Pem};

use crate::oyster::{self, AttestationError};

fn der_to_pem(der_bytes: &[u8]) -> String {
    let pem = Pem {
        tag: String::from("CERTIFICATE"),
        contents: der_bytes.to_vec(),
    };
    pem_encode(&pem)
}
