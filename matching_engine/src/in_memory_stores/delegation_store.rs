use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

use crate::generator_lib::delegation::{Delegation, Operation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationStore {
    by_generator: HashMap<Address, BTreeSet<Delegation>>,
}

impl DelegationStore {
    /// Creates a new DelegationStore.
    pub fn new() -> Self {
        DelegationStore {
            by_generator: HashMap::new(),
        }
    }

    /// Adds a delegation to the store.
    ///
    /// # Arguments
    ///
    /// * `generator` - The address of the generator.
    /// * `delegation` - The delegation to add.
    pub fn add_delegation(&mut self, generator: &Address, delegation: Delegation) {
        self.by_generator
            .entry(*generator)
            .or_insert_with(BTreeSet::new)
            .insert(delegation);
    }

    /// Retrieves a paginated list of delegations for a given generator.
    ///
    /// # Arguments
    ///
    /// * `generator` - The address of the generator whose delegations are to be retrieved.
    /// * `skip` - The number of delegations to skip.
    /// * `count` - The maximum number of delegations to return.
    ///
    /// # Returns
    ///
    /// A `Vec<Delegation>` containing the requested delegations. If the generator
    /// does not exist or there are not enough delegations, the returned vector
    /// will contain as many delegations as available after skipping.
    pub fn get_delegations_by_operations(
        &self,
        generator: &Address,
        operations: Vec<Operation>,
        skip: usize,
        count: usize,
    ) -> Vec<Delegation> {
        // Attempt to retrieve the delegations for the specified generator.
        if let Some(delegations_set) = self.by_generator.get(generator) {
            delegations_set
                .iter() // Create an iterator over &Delegation
                .filter(|delegation| operations.contains(&delegation.operation)) // Filter by operations
                .skip(skip) // Skip the first `skip` delegations
                .take(count) // Take the next `count` delegations
                .cloned() // Clone each Delegation to return owned instances
                .collect() // Collect into a Vec<Delegation>
        } else {
            Vec::new() // Generator not found; return an empty Vec
        }
    }
}
