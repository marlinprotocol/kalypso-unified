use ethers::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StakeManagerStore {
    enabled_pools: HashSet<Address>,
}

impl StakeManagerStore {
    /// Adds a new address to the enabled pools.
    ///
    /// Returns `true` if the address was not already present in the set.
    pub fn add(&mut self, address: Address) -> bool {
        self.enabled_pools.insert(address)
    }

    /// Removes an address from the enabled pools.
    ///
    /// Returns `true` if the address was present and removed.
    pub fn remove(&mut self, address: &Address) -> bool {
        self.enabled_pools.remove(address)
    }

    /// Checks if an address exists in the enabled pools.
    pub fn exists(&self, address: &Address) -> bool {
        self.enabled_pools.contains(address)
    }

    /// Retrieves all enabled pool addresses as a vector.
    pub fn get_all(&self) -> Vec<Address> {
        self.enabled_pools.iter().cloned().collect()
    }
}

impl StakeManagerStore {
    pub fn new() -> Self {
        Self {
            enabled_pools: HashSet::new(),
        }
    }
}
