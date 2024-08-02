use ethers::types::{Bytes, U256};
use std::error::Error;

use super::prover::{post_request, Prover};

pub struct ConfidentialProver {
    input_verification_executable_check_input_url: String,
    input_verification_executable_generate_proof_for_invalid_inputs_url: String,
    input_and_proof_verify_url: String,
    prover_executable_generate_proof_url: String,
    ask_id: U256,
    public: Bytes,
    secrets: Bytes,
    client: reqwest::Client,
}

impl ConfidentialProver {
    pub fn new(
        input_verification_executable_check_input_url: String,
        input_verification_executable_generate_proof_for_invalid_inputs_url: String,
        input_and_proof_verify_url: String,
        prover_executable_generate_proof_url: String,
        ask_id: U256,
        public: Bytes,
        secrets: Bytes,
    ) -> Self {
        Self {
            input_verification_executable_check_input_url,
            input_verification_executable_generate_proof_for_invalid_inputs_url,
            input_and_proof_verify_url,
            prover_executable_generate_proof_url,
            ask_id,
            public,
            secrets,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
        }
    }

    fn prepare_payload(&self) -> (Vec<u8>, Option<Vec<u8>>) {
        (self.public.to_vec(), Some(self.secrets.to_vec()))
    }
}

type BoxError = Box<dyn Error>;

impl Prover for ConfidentialProver {
    async fn check_inputs(&self) -> Result<ivs::models::CheckInputResponse, BoxError> {
        let (public, secrets) = self.prepare_payload();
        let payload = generator::models::InputPayload { public, secrets };
        post_request(
            &self.client,
            &self.input_verification_executable_check_input_url,
            &payload,
        )
        .await
    }

    async fn generate_proof(&self) -> Result<generator::models::GenerateProofResponse, BoxError> {
        let (public, secrets) = self.prepare_payload();
        let payload = generator::models::InputPayload { public, secrets };

        post_request(
            &self.client,
            &self.prover_executable_generate_proof_url,
            &payload,
        )
        .await
    }

    async fn generate_attestation_for_invalid_inputs(
        &self,
    ) -> Result<generator::models::GenerateProofResponse, BoxError> {
        let (public, secrets) = self.prepare_payload();
        let payload = ivs::models::InvalidInputPayload {
            ask_id: self.ask_id,
            public,
            secrets,
        };

        post_request(
            &self.client,
            &self.input_verification_executable_generate_proof_for_invalid_inputs_url,
            &payload,
        )
        .await
    }

    async fn verify_inputs_and_proof(
        &self,
        proof: &Vec<u8>,
    ) -> Result<ivs::models::VerifyInputAndProofResponse, Box<dyn Error>> {
        let input_and_proof_payload = ivs::models::VerifyInputsAndProof {
            public_input: Some(self.public.to_vec()),
            private_input: Some(self.secrets.to_vec()),
            proof: proof.clone(),
        };
        post_request(
            &self.client,
            &self.input_and_proof_verify_url,
            &input_and_proof_payload,
        )
        .await
    }
}
