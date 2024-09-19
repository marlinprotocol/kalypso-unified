use serde::{Deserialize, Serialize};

use crate::{
    ask::{AskState, LocalAskStatus},
    generator_lib::generator_state::GeneratorState,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAskStatus {
    pub ask_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAskStatusResponse {
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProofResponse {
    pub status: String,
    pub proof: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketInfo {
    pub market_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MarketInfoResponse {
    pub market_info: String,
    pub asks: Option<Vec<AskInfoToSend>>,
    pub generator_info: Option<GeneratorsInfoForMarket>,
}

use ethers::core::types::{Address, U256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct AskInfoToSend {
    pub ask_id: U256,
    pub market_id: U256,
    pub reward: U256,
    pub expiry: U256,
    pub proving_time: U256,
    pub deadline: U256,
    pub has_private_inputs: bool,
    pub state: Option<AskState>,
    pub generator: Option<Address>,
}

#[derive(Serialize, Deserialize)]
pub struct GeneratorInfo {
    pub generator_address: Address,
    pub stake_locked: U256,
    pub total_stake: U256,
    pub compute_consumed: U256,
    pub declared_compute: U256,
    pub compute_required_per_request: U256,
    pub proof_generation_cost: U256,
    pub proposed_time: U256,
    pub active_requests: U256,
    pub proofs_submitted: U256,
    pub state: Option<GeneratorState>,
}

#[derive(Serialize, Deserialize)]
pub struct GeneratorsInfoForMarket {
    pub count: i32,
    pub generators: Vec<GeneratorInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceResponse {
    pub balance: Option<ethers::types::U256>,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct MarketStatsResponse {
    pub market_info: String,
    pub generator_count: Option<usize>,
    pub proofs_generated: Option<usize>,
    pub proofs_pending: Option<usize>,
}
