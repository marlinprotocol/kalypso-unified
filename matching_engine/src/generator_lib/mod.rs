use ethers::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utility::AddressTokenPair;

pub mod delegation;
pub mod generator_helper;
pub mod generator_query;
pub mod generator_state;
pub mod generator_store;
pub mod key_store;
pub mod native_stake_store;
pub mod points;
pub mod stake_manager_store;
pub mod symbiotic_stake_store;
pub mod traits;
pub mod withdrawal_request;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct SlashingRecord {
    pub ask_id: U256,
    pub slashing_block_number: U64,
    pub market_id: U256,
    pub slashing_tx: String,
    pub price_offered: U256,
    pub expected_time: U256,
    pub slashing_penalty: AddressTokenPair,
    pub slashing_timestamp: U256,
    pub source: delegation::Source,
}
