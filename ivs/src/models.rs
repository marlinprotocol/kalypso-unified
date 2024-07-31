use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InputPayload {
    pub public: String,
    pub secrets: Option<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct EncryptedInputPayload {
    pub acl: String,
    pub encrypted_secrets: String,
    pub me_decryption_url: String,
    pub market_id: String,
}

use bindings::shared_types::Ask;
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct AskPayload {
    pub ask: Ask,
    pub encrypted_secret: String,
    pub acl: String,
    pub ask_id: u64,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct VerifyInputsAndProof {
    pub public_input: Option<String>,
    pub private_input: String,
    pub proof: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct SimpleCheckInputResponse {
    pub is_input_valid: bool,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InvalidInputsAttestationResponse {
    pub signature: String,
    pub ask_id: u64,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct VerifyInputAndProofResponse {
    pub is_input_and_proof_valid: bool,
}
