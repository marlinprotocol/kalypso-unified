use bindings::shared_types::Ask;
use ethers::prelude::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct AskInputPayload {
    pub ask: Ask,
    pub private_input: Vec<u8>,
    pub ask_id: u64,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct TestResponse {
    pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct ProofGenerationResponse {
    pub message: String,
    pub data: Bytes,
}
