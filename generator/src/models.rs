use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InputPayload {
    pub public: String,
    pub secrets: Option<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct TestResponse {
    pub data: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct GenerateProofResponse {
    pub proof: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct BenchmarkResponse {
    pub data: String,
    pub time_in_ms: u64,
}
