use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

// Ensure that the path to AddressTokenPair is correct in your project.
use crate::utility::AddressTokenPair;

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
