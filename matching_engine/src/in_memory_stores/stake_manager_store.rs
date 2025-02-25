use ethers::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::generator_lib::stake_manager_store::StakeManagerOperations;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeManagerStore {
    enabled_pools: HashSet<Address>,
}

impl Default for StakeManagerStore {
    fn default() -> Self {
        Self::new()
    }
}

impl StakeManagerStore {
    fn new() -> Self {
        Self {
            enabled_pools: HashSet::new(),
        }
    }
}

impl StakeManagerOperations for StakeManagerStore {
    /// Adds a new address to the enabled pools.
    ///
    /// Returns `true` if the address was not already present in the set.
    fn add(&mut self, address: Address) -> bool {
        self.enabled_pools.insert(address)
    }

    /// Removes an address from the enabled pools.
    ///
    /// Returns `true` if the address was present and removed.
    fn remove(&mut self, address: &Address) -> bool {
        self.enabled_pools.remove(address)
    }

    /// Checks if an address exists in the enabled pools.
    fn exists(&self, address: &Address) -> bool {
        self.enabled_pools.contains(address)
    }

    /// Retrieves all enabled pool addresses as a vector.
    fn get_all(&self) -> Vec<Address> {
        self.enabled_pools.iter().cloned().collect()
    }
}
