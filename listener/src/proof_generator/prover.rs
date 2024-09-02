use ethers::types::Bytes;
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

#[derive(Debug, Clone)]
pub enum Proof {
    ValidProof(Bytes),
    InvalidProof(Bytes),
}

pub trait Prover {
    async fn check_inputs(&self) -> Result<ivs::models::CheckInputResponse, Box<dyn Error>>;
    async fn generate_proof(
        &self,
    ) -> Result<generator::models::GenerateProofResponse, Box<dyn Error>>;
    async fn generate_attestation_for_invalid_inputs(
        &self,
    ) -> Result<generator::models::GenerateProofResponse, Box<dyn Error>>;
    async fn verify_inputs_and_proof(
        &self,
        proof: &[u8],
    ) -> Result<ivs::models::VerifyInputAndProofResponse, Box<dyn Error>>;

    async fn get_proof(&self) -> Result<Proof, Box<dyn Error>> {
        let check_input = self.check_inputs().await?;
        if check_input.valid {
            let proof = self.generate_proof().await?;
            let check_proof = self.verify_inputs_and_proof(proof.proof.as_ref()).await;
            match check_proof {
                Ok(data) => {
                    if data.is_input_and_proof_valid {
                        log::info!("Generated Proof is Valid: Rechecked with IVS");
                    } else {
                        log::warn!(
                            "Generated Proof is Invalid: after 2 round of checking with IVS"
                        );
                    }
                }
                _ => {
                    log::error!("Generated Proof could not be verified against IVS");
                }
            }
            Ok(Proof::ValidProof(proof.proof.into()))
        } else {
            let proof = self.generate_attestation_for_invalid_inputs().await?;
            Ok(Proof::InvalidProof(proof.proof.into()))
        }
    }
}

pub async fn post_request<T, R>(
    client: &Client,
    url: &str,
    payload: &T,
) -> Result<R, Box<dyn Error>>
where
    T: Serialize,
    R: DeserializeOwned,
{
    let response = client.post(url).json(payload).send().await?;

    if response.status().is_success() {
        log::info!("POST: {} | Response: {}", url, response.status());
        let response_payload = response.json::<R>().await?;
        Ok(response_payload)
    } else {
        log::error!("POST: {} | Response: {}", url, response.status());
        Err(Box::new(response.error_for_status().unwrap_err()))
    }
}
