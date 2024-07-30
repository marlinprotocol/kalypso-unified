use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct GenerateProofResponse {
    pub proof: String,
}
