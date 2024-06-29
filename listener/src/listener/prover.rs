use ethers::types::Bytes;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize)]
pub struct CheckInputResponse {
    pub valid: bool,
}

#[derive(Deserialize)]
pub struct GenerateProofResponse {
    pub proof: String,
}

#[derive(Debug, Clone)]
pub enum Proof {
    ValidProof(Bytes),
    InvalidProof(Bytes),
}

pub trait Prover {
    async fn check_inputs(&self) -> Result<CheckInputResponse, Box<dyn Error>>;
    async fn generate_proof(&self) -> Result<GenerateProofResponse, Box<dyn Error>>;
    async fn generate_proof_for_invalid_inputs(
        &self,
    ) -> Result<GenerateProofResponse, Box<dyn Error>>;

    async fn get_proof(&self) -> Result<Proof, Box<dyn Error>> {
        let check_input = self.check_inputs().await?;
        if check_input.valid {
            let proof = self.generate_proof().await?;
            let proof = hex::decode(proof.proof)?;
            return Ok(Proof::ValidProof(proof.into()));
        } else {
            let proof = self.generate_proof_for_invalid_inputs().await?;
            let proof = hex::decode(proof.proof)?;
            return Ok(Proof::InvalidProof(proof.into()));
        }
    }
}

pub async fn post_request<T, R>(url: &str, payload: &T) -> Result<R, Box<dyn Error>>
where
    T: Serialize,
    R: DeserializeOwned,
{
    let client = Client::new();
    let response = client.post(url).json(payload).send().await?;

    if response.status().is_success() {
        let response_payload = response.json::<R>().await?;
        Ok(response_payload)
    } else {
        Err(Box::new(response.error_for_status().unwrap_err()))
    }
}
