use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InvalidInputPayload {
    pub ask_id: ethers::types::U256,
    pub public: Vec<u8>,
    pub secrets: Option<Vec<u8>>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CheckInputResponse {
    pub valid: bool,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct EncryptedInputPayload {
    pub acl: Vec<u8>,
    pub encrypted_secrets: Vec<u8>,
    pub me_decryption_url: String,
    pub market_id: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct VerifyInputsAndProof {
    pub public_input: Option<Vec<u8>>,
    pub private_input: Option<Vec<u8>>,
    pub proof: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct VerifyInputAndProofResponse {
    pub is_input_and_proof_valid: bool,
}
