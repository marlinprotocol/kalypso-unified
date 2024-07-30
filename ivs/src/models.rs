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
