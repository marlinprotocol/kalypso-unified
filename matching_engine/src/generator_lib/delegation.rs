use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::hash::{Hash, Hasher};

// Ensure that the path to AddressTokenPair is correct in your project.
use crate::utility::AddressTokenPair;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationStore {
    by_generator: HashMap<Address, BTreeSet<Delegation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    pub delegation: AddressTokenPair,
    pub source: Source,
    pub operation: Operation,
    // block_number | transaction_index | log_index | Source | Operation is unique
    pub block_number: U64,
    pub transaction_index: U64,
    pub log_index: U256,
    pub tx: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Operation {
    Delegate,
    UnDelegate,
    Slash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Source {
    Native,
    Symbiotic,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Delegate => write!(f, "Delegate"),
            Operation::UnDelegate => write!(f, "UnDelegate"),
            Operation::Slash => write!(f, "Slash"),
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::Native => write!(f, "Native"),
            Source::Symbiotic => write!(f, "Symbiotic"),
        }
    }
}

impl PartialEq for Delegation {
    fn eq(&self, other: &Self) -> bool {
        self.block_number == other.block_number
            && self.transaction_index == other.transaction_index
            && self.log_index == other.log_index
            && self.source == other.source
            && self.operation == other.operation
    }
}

impl Eq for Delegation {}

impl Hash for Delegation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.block_number.hash(state);
        self.transaction_index.hash(state);
        self.log_index.hash(state);
        self.source.hash(state);
        self.operation.hash(state);
    }
}

impl PartialOrd for Delegation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Delegation {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.block_number.cmp(&other.block_number) {
            Ordering::Equal => match self.transaction_index.cmp(&other.transaction_index) {
                Ordering::Equal => match self.log_index.cmp(&other.log_index) {
                    Ordering::Equal => match self.source.cmp(&other.source) {
                        Ordering::Equal => self.operation.cmp(&other.operation),
                        other_order => other_order,
                    },
                    other_order => other_order,
                },
                other_order => other_order,
            },
            other_order => other_order,
        }
    }
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
