use std::collections::HashMap;

use async_trait::async_trait;
use ethers::types::U256;
use matching_engine_helpers::ask_lib::ask_status::AskState;
use serde::Deserialize;

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct ReadProof;

#[async_trait]
impl Operation for ReadProof {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let read_proof_info = CommonDeps::read_proof_info(&config)?;

        let ask_state = read_proof_info
            .proof_marketplace
            .get_bid_state(read_proof_info.ask_id)
            .call()
            .await
            .map_err(|e| format!("Unable to read proof marketplace contract: {}", e))?;

        let ask_state = matching_engine_helpers::ask_lib::ask_status::get_ask_state(ask_state);

        if ask_state == AskState::Complete {
            println!("Request is complete");
            let proof = read_proof(&read_proof_info.ask_id, read_proof_info.indexer_url)
                .await
                .map_err(|e| format!("Unable to read proof from indexer: {}", e))?;

            println!("{}", proof);

            return Ok(());
        }

        if ask_state == AskState::Null {
            return Err("Invalid Ask Request ID".to_string());
        }

        Ok(())
    }
}

use std::error::Error;
#[derive(Debug, Deserialize, Clone)]
struct Proof {
    status: String,
    proof: Vec<u8>,
}

async fn read_proof(ask_id: &U256, indexer_url: String) -> Result<String, Box<dyn Error>> {
    // Initialize the HTTP client with a timeout (optional but recommended)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let full_url = format!("{}/stats/getProof", indexer_url);

    // Prepare the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);

    // Prepare the JSON payload, converting U256 to a string
    let payload = serde_json::json!({
        "ask_id": ask_id.to_string()
    });

    // Send the POST request
    let response = client
        .post(&full_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    // Capture the HTTP status before moving the response
    let status = response.status();

    // Read the response body as text
    let response_text = response.text().await?;

    // Attempt to parse the JSON response into the Proof struct
    let proof_response: Proof = match serde_json::from_str(&response_text) {
        Ok(proof) => proof,
        Err(e) => {
            return Err(format!(
                "Failed to parse JSON response. HTTP Status: {}, Body: {}, Error: {}",
                status, response_text, e
            )
            .into())
        }
    };

    // Check the HTTP status and the JSON status
    if !status.is_success() && proof_response.status.to_lowercase() != "Invalid Inputs Detected" {
        return Ok("Prover Detected Invalid Inputs. An attestation was submitted to prove that inputs were invalid".into());
    }

    // Encode the proof bytes to a hex string and return
    Ok(format!("0x{}", hex::encode(proof_response.proof)))
}
