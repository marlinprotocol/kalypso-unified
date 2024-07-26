use ethers::types::{Bytes, U256};
use serde::Serialize;

use super::prover::{post_request, CheckInputResponse, GenerateProofResponse, Prover};

pub struct NonConfidentialProver {
    input_verification_executable_check_input_url: String,
    input_verification_executable_generate_proof_for_invalid_inputs_url: String,
    prover_executable_generate_proof_url: String,
    ask_id: U256,
    public: Bytes,
}

impl NonConfidentialProver {
    pub fn new(
        input_verification_executable_check_input_url: String,
        input_verification_executable_generate_proof_for_invalid_inputs_url: String,
        prover_executable_generate_proof_url: String,
        ask_id: U256,
        public: Bytes,
    ) -> Self {
        NonConfidentialProver {
            input_verification_executable_check_input_url,
            input_verification_executable_generate_proof_for_invalid_inputs_url,
            prover_executable_generate_proof_url,
            ask_id,
            public,
        }
    }
}

#[derive(Serialize)]
pub struct InputPayload {
    pub public: String,
}

#[derive(Serialize)]
pub struct InvalidInputPayload {
    pub ask_id: String,
    pub public: String,
}

impl Prover for NonConfidentialProver {
    async fn check_inputs(
        &self,
    ) -> Result<super::prover::CheckInputResponse, Box<dyn std::error::Error>> {
        let payload = InputPayload {
            public: hex::encode(&self.public),
        };

        post_request::<InputPayload, CheckInputResponse>(
            &self.input_verification_executable_check_input_url,
            &payload,
        )
        .await
    }

    async fn generate_proof(
        &self,
    ) -> Result<super::prover::GenerateProofResponse, Box<dyn std::error::Error>> {
        let payload = InputPayload {
            public: hex::encode(&self.public),
        };

        post_request::<InputPayload, GenerateProofResponse>(
            &self.prover_executable_generate_proof_url,
            &payload,
        )
        .await
    }

    async fn generate_proof_for_invalid_inputs(
        &self,
    ) -> Result<super::prover::GenerateProofResponse, Box<dyn std::error::Error>> {
        let payload: InvalidInputPayload = InvalidInputPayload {
            ask_id: self.ask_id.to_string(),
            public: hex::encode(&self.public),
        };

        post_request::<InvalidInputPayload, GenerateProofResponse>(
            &self.input_verification_executable_generate_proof_for_invalid_inputs_url,
            &payload,
        )
        .await
    }
}
