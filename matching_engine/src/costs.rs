use ethers::types::U256;

/// A trait that defines all operations for a CostStore.
pub trait CostStoreOperations {
    /// Creates a new CostStore.
    fn new() -> Self
    where
        Self: Sized;

    /// Inserts or updates a key-value pair.
    fn upsert(&mut self, key: u8, value: U256);

    /// Removes a key-value pair by key.
    fn remove(&mut self, key: u8) -> Option<U256>;

    /// Retrieves the value associated with a key.
    fn get(&self, key: u8) -> Option<U256>;
}
