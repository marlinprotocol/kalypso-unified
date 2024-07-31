use ethers::types::{Bytes, U256};

use super::prover::{post_request, Prover};

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

impl Prover for NonConfidentialProver {
    async fn check_inputs(
        &self,
    ) -> Result<ivs::models::CheckInputResponse, Box<dyn std::error::Error>> {
        let payload = ivs::models::InputPayload {
            public: hex::encode(&self.public),
            secrets: None,
        };

        post_request::<ivs::models::InputPayload, ivs::models::CheckInputResponse>(
            &self.input_verification_executable_check_input_url,
            &payload,
        )
        .await
    }

    async fn generate_proof(
        &self,
    ) -> Result<generator::models::GenerateProofResponse, Box<dyn std::error::Error>> {
        let payload = ivs::models::InputPayload {
            public: hex::encode(&self.public),
            secrets: None,
        };

        post_request::<ivs::models::InputPayload, generator::models::GenerateProofResponse>(
            &self.prover_executable_generate_proof_url,
            &payload,
        )
        .await
    }

    async fn generate_attestation_for_invalid_inputs(
        &self,
    ) -> Result<generator::models::GenerateProofResponse, Box<dyn std::error::Error>> {
        let payload: ivs::models::InvalidInputPayload = ivs::models::InvalidInputPayload {
            ask_id: self.ask_id.to_string(),
            public: hex::encode(&self.public),
            secrets: None,
        };

        post_request::<ivs::models::InvalidInputPayload, generator::models::GenerateProofResponse>(
            &self.input_verification_executable_generate_proof_for_invalid_inputs_url,
            &payload,
        )
        .await
    }
}
