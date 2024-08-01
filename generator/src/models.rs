use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InputPayload {
    pub public: Vec<u8>,
    pub secrets: Option<Vec<u8>>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct GenerateProofResponse {
    pub proof: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct TestResponse {
    pub data: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct BenchmarkResponse {
    pub data: String,
    pub time_in_ms: u128,
}
