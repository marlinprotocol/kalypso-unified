use ethers::types::{Bytes, U256};
use std::error::Error;

use super::prover::{post_request, Prover};

pub struct ConfidentialProver {
    input_verification_executable_check_input_url: String,
    input_verification_executable_generate_proof_for_invalid_inputs_url: String,
    prover_executable_generate_proof_url: String,
    ask_id: U256,
    public: Bytes,
    secrets: Bytes,
}

impl ConfidentialProver {
    pub fn new(
        input_verification_executable_check_input_url: String,
        input_verification_executable_generate_proof_for_invalid_inputs_url: String,
        prover_executable_generate_proof_url: String,
        ask_id: U256,
        public: Bytes,
        secrets: Bytes,
    ) -> Self {
        ConfidentialProver {
            input_verification_executable_check_input_url,
            input_verification_executable_generate_proof_for_invalid_inputs_url,
            prover_executable_generate_proof_url,
            ask_id,
            public,
            secrets,
        }
    }
}

impl Prover for ConfidentialProver {
    async fn check_inputs(&self) -> Result<ivs::models::CheckInputResponse, Box<dyn Error>> {
        let payload = ivs::models::InputPayload {
            public: hex::encode(&self.public),
            secrets: Some(hex::encode(&self.secrets)),
        };

        post_request::<ivs::models::InputPayload, ivs::models::CheckInputResponse>(
            &self.input_verification_executable_check_input_url,
            &payload,
        )
        .await
    }

    async fn generate_proof(
        &self,
    ) -> Result<generator::models::GenerateProofResponse, Box<dyn Error>> {
        let payload = ivs::models::InputPayload {
            public: hex::encode(&self.public),
            secrets: Some(hex::encode(&self.secrets)),
        };

        post_request::<ivs::models::InputPayload, generator::models::GenerateProofResponse>(
            &self.prover_executable_generate_proof_url,
            &payload,
        )
        .await
    }

    async fn generate_attestation_for_invalid_inputs(
        &self,
    ) -> Result<generator::models::GenerateProofResponse, Box<dyn Error>> {
        let payload: ivs::models::InvalidInputPayload = ivs::models::InvalidInputPayload {
            ask_id: self.ask_id.to_string(),
            public: hex::encode(&self.public),
            secrets: Some(hex::encode(&self.secrets)),
        };

        post_request::<ivs::models::InvalidInputPayload, generator::models::GenerateProofResponse>(
            &self.input_verification_executable_generate_proof_for_invalid_inputs_url,
            &payload,
        )
        .await
    }
}
