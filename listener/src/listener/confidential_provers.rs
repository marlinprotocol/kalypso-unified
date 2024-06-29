use ethers::types::{Bytes, U256};
use serde::Serialize;
use std::error::Error;

use super::prover::{post_request, CheckInputResponse, GenerateProofResponse, Prover};

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

#[derive(Serialize)]
pub struct InputPayload {
    pub public: String,
    pub secrets: String,
}

#[derive(Serialize)]
pub struct InvalidInputPayload {
    pub ask_id: String,
    pub public: String,
    pub secrets: String,
}

impl Prover for ConfidentialProver {
    async fn check_inputs(&self) -> Result<CheckInputResponse, Box<dyn Error>> {
        let payload = InputPayload {
            public: hex::encode(&self.public),
            secrets: hex::encode(&self.secrets),
        };

        post_request::<InputPayload, CheckInputResponse>(
            &self.input_verification_executable_check_input_url,
            &payload,
        )
        .await
    }

    async fn generate_proof(&self) -> Result<GenerateProofResponse, Box<dyn Error>> {
        let payload = InputPayload {
            public: hex::encode(&self.public),
            secrets: hex::encode(&self.secrets),
        };

        post_request::<InputPayload, GenerateProofResponse>(
            &self.prover_executable_generate_proof_url,
            &payload,
        )
        .await
    }

    async fn generate_proof_for_invalid_inputs(
        &self,
    ) -> Result<GenerateProofResponse, Box<dyn Error>> {
        let payload: InvalidInputPayload = InvalidInputPayload {
            ask_id: self.ask_id.to_string(),
            public: hex::encode(&self.public),
            secrets: hex::encode(&self.secrets),
        };

        post_request::<InvalidInputPayload, GenerateProofResponse>(
            &self.input_verification_executable_generate_proof_for_invalid_inputs_url,
            &payload,
        )
        .await
    }
}
