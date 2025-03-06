use crate::{generator_lib::native_stake_store::NativeStakingOperations, utility::TokenTracker};
use async_trait::async_trait;
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

#[async_trait]
impl NativeStakingOperations for NativeStakingStore {
    async fn set_lock_token(&mut self, token: Address, amount: U256) {
        self.tokens_to_lock.force_set(token, amount);
    }

    async fn remove_lock_token(&mut self, token: Address) {
        self.tokens_to_lock.force_remove(token);
    }

    async fn tokens_to_lock(&self) -> TokenTracker {
        self.tokens_to_lock.clone()
    }

    fn get_slashing_penalty_percent(&self, _token: Address) -> U256 {
        U256::zero()
    }

    fn set_slashing_penalty_percent(&mut self, _token: Address, _percent: U256) {
        unimplemented!("set_slashing_penalty_percent are not implemented in NativeStakingStore/NativeStakingModule")
    }
}
