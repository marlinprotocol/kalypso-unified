use super::ask_status::AskState;
use ethers::core::types::U256;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct LocalAsk {
    pub ask_id: U256,
    pub market_id: U256,
    pub reward: U256,
    pub expiry: U256,
    pub deadline: U256,
    pub prover_refund_address: Address,
    pub prover_data: Bytes,
    pub has_private_inputs: bool,
    pub secret_data: Option<Bytes>,
    pub secret_acl: Option<Bytes>,
    pub state: Option<AskState>,
    pub generator: Option<Address>,
    pub invalid_secret_flag: bool,
}
