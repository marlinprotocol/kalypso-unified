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
    pub time_requested_for_proof_generation: U256,
    pub prover_refund_address: Address,
    pub prover_data: Bytes,
    pub has_private_inputs: bool,
    pub secret_data: Option<Bytes>,
    pub secret_acl: Option<Bytes>,
    pub state: Option<AskState>,
    pub generator: Option<Address>,
    pub invalid_secret_flag: bool,
    pub created_on: U256,
    pub created_on_l1: U256,
    pub create_transaction: H256,
}

use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

impl Hash for LocalAsk {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ask_id.hash(state);
    }
}

impl Ord for LocalAsk {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ask_id.cmp(&other.ask_id) // Compare by ask_id
    }
}
