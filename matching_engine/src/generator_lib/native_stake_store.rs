use crate::utility::TokenTracker;
use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NativeStakingStore {
    pub tokens_to_lock: TokenTracker,
}

impl NativeStakingStore {
    pub fn new() -> Self {
        Self {
            tokens_to_lock: TokenTracker::new(),
        }
    }
}

impl NativeStakingStore {
    pub fn set_lock_token(&mut self, token: Address, amount: U256) {
        self.tokens_to_lock.force_set(token, amount);
    }

    pub fn remove_lock_token(&mut self, token: Address) {
        self.tokens_to_lock.force_remove(token);
    }
}
