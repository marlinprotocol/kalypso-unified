use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct SecretInputPayload {
    pub secrets: String,
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
pub struct VerifyInputsAndSecrets {
    pub public: String,
    pub secrets: Option<String>,
}
