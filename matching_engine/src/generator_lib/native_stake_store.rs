use crate::utility::TokenTracker;
use ethers::types::{Address, U256};

pub trait NativeStakingOperations {
    /// Sets the lock for a given token with the specified amount.
    fn set_lock_token(&mut self, token: Address, amount: U256);

    /// Removes the lock for a given token.
    fn remove_lock_token(&mut self, token: Address);

    fn tokens_to_lock(&self) -> TokenTracker;
}
