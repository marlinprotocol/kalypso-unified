use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InputPayload {
    pub public: String,
    pub secrets: Option<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InvalidInputPayload {
    pub ask_id: String,
    pub public: String,
    pub secrets: Option<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CheckInputResponse {
    pub valid: bool,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct EncryptedInputPayload {
    pub acl: String,
    pub encrypted_secrets: String,
    pub me_decryption_url: String,
    pub market_id: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct VerifyInputsAndProof {
    pub public_input: Option<String>,
    pub private_input: String,
    pub proof: String,
}
