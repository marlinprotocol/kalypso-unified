use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct GenerateProofResponse {
    pub proof: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InputPayload {
    pub public: String,
    pub secrets: Option<String>,
}
