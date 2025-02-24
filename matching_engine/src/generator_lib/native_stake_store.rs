use crate::utility::TokenTracker;
use async_trait::async_trait;
use ethers::types::{Address, U256};

#[async_trait]
pub trait NativeStakingOperations {
    /// Sets the lock for a given token with the specified amount.
    async fn set_lock_token(&mut self, token: Address, amount: U256);

    /// Removes the lock for a given token.
    async fn remove_lock_token(&mut self, token: Address);

    async fn tokens_to_lock(&self) -> TokenTracker;
}
