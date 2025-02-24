use ethers::types::Address;

pub trait StakeManagerOperations {
    /// Adds a new address to the enabled pools.
    /// Returns `true` if the address was not already present.
    fn add(&mut self, address: Address) -> bool;

    /// Removes an address from the enabled pools.
    /// Returns `true` if the address was present and removed.
    fn remove(&mut self, address: &Address) -> bool;

    /// Checks if an address exists in the enabled pools.
    fn exists(&self, address: &Address) -> bool;

    /// Retrieves all enabled pool addresses as a vector.
    fn get_all(&self) -> Vec<Address>;
}
