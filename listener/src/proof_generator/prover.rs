use ethers::types::Bytes;
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, sync::Arc};
use tokio::sync::Semaphore;

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

    fn should_skip_input_verification(&self) -> bool;

    async fn get_proof(
        &self,
        valid_proof_semaphore: Arc<Semaphore>,
        invalid_inputs_semaphore: Arc<Semaphore>,
    ) -> Result<Proof, Box<dyn Error>> {
        if self.should_skip_input_verification() {
            let proof_permit = valid_proof_semaphore
                .acquire()
                .await
                .expect("Failed to acquire proof semaphore");
            let proof = self.generate_proof().await?;
            drop(proof_permit); // not needed but explicity dropping it
            return Ok(Proof::ValidProof(proof.proof.into()));
        }

        let check_input = self.check_inputs().await?;
        if check_input.valid {
            let proof_permit = valid_proof_semaphore
                .acquire()
                .await
                .expect("Failed to acquire proof semaphore");

            let proof = self.generate_proof().await?;

            drop(proof_permit); // not needed but explicity dropping  it
            let check_proof = self.verify_inputs_and_proof(proof.proof.as_ref()).await;
            match check_proof {
                Ok(data) => {
                    if data.is_input_and_proof_valid {
                        log::info!("is_input_and_proof_valid");
                    } else {
                        if cfg!(feature = "testnet") {
                            log::warn!("is_input_and_proof_is_invalid");
                            log::warn!("invalid proofs will be rejected in future");
                        } else {
                            log::error!("is_input_and_proof_is_invalid");
                            log::error!("invalid proofs will be rejected in future");
                        }
                    }
                }
                _ => {
                    if cfg!(feature = "testnet") {
                        log::warn!("Generated Proof could not be verified against IVS");
                    } else {
                        log::error!("Generated Proof could not be verified against IVS");
                    }
                }
            }
            Ok(Proof::ValidProof(proof.proof.into()))
        } else {
            let invalid_inputs_permit = invalid_inputs_semaphore
                .acquire()
                .await
                .expect("Failed to acquire proof semaphore");

            let proof = self.generate_attestation_for_invalid_inputs().await?;

            drop(invalid_inputs_permit);
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
