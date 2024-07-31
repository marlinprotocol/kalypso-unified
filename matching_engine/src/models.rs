use serde::{Deserialize, Serialize};

use crate::ask::LocalAskStatus;
#[derive(Deserialize, Serialize)]
pub struct DecryptRequest {
    pub market_id: String,
    pub private_input: String,
    pub acl: String,
    pub signature: String,
    pub ivs_pubkey: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRequestResponse {
    pub encrypted_data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WelcomeResponse {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLatestBlockNumberResponse {
    pub block_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStatusResponse {
    pub local_ask_status: LocalAskStatus,
}
