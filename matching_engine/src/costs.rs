use async_trait::async_trait;
use ethers::types::U256;

/// A trait that defines all operations for a CostStore.

#[async_trait]
pub trait CostStoreOperations {
    /// Creates a new CostStore.
    fn new() -> Self
    where
        Self: Sized;

    /// Inserts or updates a key-value pair.
    async fn upsert(&mut self, key: u8, value: U256);

    /// Removes a key-value pair by key.
    async fn remove(&mut self, key: u8) -> Option<U256>;

    /// Retrieves the value associated with a key.
    async fn get(&self, key: u8) -> Option<U256>;
}
