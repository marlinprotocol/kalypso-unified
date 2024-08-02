use ethers::types::{Bytes, U256};
use std::error::Error;

use super::prover::{post_request, Prover};

pub struct NonConfidentialProver {
    input_verification_executable_check_input_url: String,
    input_verification_executable_generate_proof_for_invalid_inputs_url: String,
    input_and_proof_verify_url: String,
    prover_executable_generate_proof_url: String,
    ask_id: U256,
    public: Bytes,
    client: reqwest::Client,
}

impl NonConfidentialProver {
    pub fn new(
        input_verification_executable_check_input_url: String,
        input_verification_executable_generate_proof_for_invalid_inputs_url: String,
        input_and_proof_verify_url: String,
        prover_executable_generate_proof_url: String,
        ask_id: U256,
        public: Bytes,
    ) -> Self {
        Self {
            input_verification_executable_check_input_url,
            input_verification_executable_generate_proof_for_invalid_inputs_url,
            input_and_proof_verify_url,
            prover_executable_generate_proof_url,
            ask_id,
            public,
            client: reqwest::Client::new(),
        }
    }

    fn prepare_payload(&self) -> (Vec<u8>, Option<Vec<u8>>) {
        (self.public.to_vec(), None) // No secrets in NonConfidentialProver
    }
}

type BoxError = Box<dyn Error>;

impl Prover for NonConfidentialProver {
    async fn check_inputs(&self) -> Result<ivs::models::CheckInputResponse, BoxError> {
        let (public, _) = self.prepare_payload();
        let payload = generator::models::InputPayload::only_public_inputs(public);

        post_request(
            &self.client,
            &self.input_verification_executable_check_input_url,
            &payload,
        )
        .await
    }

    async fn generate_proof(&self) -> Result<generator::models::GenerateProofResponse, BoxError> {
        let (public, _) = self.prepare_payload();
        let payload = generator::models::InputPayload::only_public_inputs(public);

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
        let (public, _) = self.prepare_payload(); // `secrets` is always `None`

        let payload = ivs::models::InvalidInputPayload::only_public_inputs(self.ask_id, public);

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
            private_input: None,
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
