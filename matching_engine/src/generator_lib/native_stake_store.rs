use crate::utility::TokenTracker;
use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeStakingStore {
    pub tokens_to_lock: TokenTracker,
}

impl Default for NativeStakingStore {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeStakingStore {
    fn new() -> Self {
        Self {
            tokens_to_lock: TokenTracker::new(),
        }
    }
}

impl NativeStakingOperations for NativeStakingStore {
    fn set_lock_token(&mut self, token: Address, amount: U256) {
        self.tokens_to_lock.force_set(token, amount);
    }

    fn remove_lock_token(&mut self, token: Address) {
        self.tokens_to_lock.force_remove(token);
    }

    fn tokens_to_lock(&self) -> TokenTracker {
        self.tokens_to_lock.clone()
    }
}

pub trait NativeStakingOperations {
    /// Sets the lock for a given token with the specified amount.
    fn set_lock_token(&mut self, token: Address, amount: U256);

    /// Removes the lock for a given token.
    fn remove_lock_token(&mut self, token: Address);

    fn tokens_to_lock(&self) -> TokenTracker;
}
