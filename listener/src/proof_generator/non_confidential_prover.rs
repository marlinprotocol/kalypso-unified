use ethers::types::{Bytes, U256};
use std::error::Error;

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
        Self {
            input_verification_executable_check_input_url,
            input_verification_executable_generate_proof_for_invalid_inputs_url,
            prover_executable_generate_proof_url,
            ask_id,
            public,
        }
    }

    fn prepare_payload(&self) -> (Vec<u8>, Option<Vec<u8>>) {
        (self.public.to_vec(), None) // No secrets in NonConfidentialProver
    }
}

type BoxError = Box<dyn Error>;

impl Prover for NonConfidentialProver {
    async fn check_inputs(&self) -> Result<ivs::models::CheckInputResponse, BoxError> {
        let (public, secrets) = self.prepare_payload();
        let payload = ivs::models::InputPayload { public, secrets };

        post_request(
            &self.input_verification_executable_check_input_url,
            &payload,
        )
        .await
    }

    async fn generate_proof(&self) -> Result<generator::models::GenerateProofResponse, BoxError> {
        let (public, secrets) = self.prepare_payload();
        let payload = generator::models::InputPayload { public, secrets };

        post_request(&self.prover_executable_generate_proof_url, &payload).await
    }

    async fn generate_attestation_for_invalid_inputs(
        &self,
    ) -> Result<generator::models::GenerateProofResponse, BoxError> {
        let (public, _) = self.prepare_payload(); // `secrets` is always `None`
        let payload = ivs::models::InvalidInputPayload {
            ask_id: self.ask_id,
            public,
            secrets: None,
        };

        post_request(
            &self.input_verification_executable_generate_proof_for_invalid_inputs_url,
            &payload,
        )
        .await
    }
}
