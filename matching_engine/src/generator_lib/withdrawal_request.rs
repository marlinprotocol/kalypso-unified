use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd)]
pub struct WithdrawlRequest {
    pub account: Address,
    pub token: Address,
    pub amount: U256,
    pub index: U256,
    pub timestamp: U256, // We'll ignore this in Hash and PartialEq/Eq
}

// Manually implement PartialEq (ignore `timestamp`)
impl PartialEq for WithdrawlRequest {
    fn eq(&self, other: &Self) -> bool {
        self.account == other.account
            && self.token == other.token
            && self.amount == other.amount
            && self.index == other.index
        // `timestamp` is intentionally not compared
    }
}

// With PartialEq implemented, we can do Eq trivially
impl Eq for WithdrawlRequest {}

// Manually implement Hash (ignore `timestamp`)
impl Hash for WithdrawlRequest {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.account.hash(state);
        self.token.hash(state);
        self.amount.hash(state);
        self.index.hash(state);
        // `timestamp` is intentionally not hashed
    }
}
