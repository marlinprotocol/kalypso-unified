use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fmt::Formatter, fmt::Result};

use crate::utility::TokenTracker;

pub mod ask;
pub mod ask_query;
pub mod ask_status;
pub mod ask_store;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Proof {
    ValidProof(Bytes),
    InvalidInputAttestation,
    FailedProofGeneration,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RemoveReason {
    ProofCreated,
    BidCancelled,
    ProofNotGenerated,
    InvalidInputsDetected,
}

impl Display for Proof {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Proof::ValidProof(bytes) => {
                write!(f, "valid: {}", hex::encode(bytes))
            }
            Proof::InvalidInputAttestation => {
                write!(f, "invalid inputs detected")
            }
            Proof::FailedProofGeneration => {
                write!(f, "failed proof generation")
            }
        }
    }
}

impl Default for Proof {
    fn default() -> Self {
        Proof::FailedProofGeneration
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssociatedStakeLock {
    pub native: TokenTracker,
    pub symbiotic: TokenTracker,
}
