use super::ask_status::AskState;
use ethers::core::types::U256;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct LocalAsk {
    pub ask_id: U256,
    pub market_id: U256,
    pub reward: U256,
    pub expiry: U256,
    pub deadline: U256,
    pub prover_refund_address: Address,
    pub prover_data: Bytes,
    pub has_private_inputs: bool,
    pub secret_data: Option<Bytes>,
    pub secret_acl: Option<Bytes>,
    pub state: Option<AskState>,
    pub generator: Option<Address>,
    pub invalid_secret_flag: bool,
}

use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

impl Hash for LocalAsk {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ask_id.hash(state);
    }
}

impl Ord for LocalAsk {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ask_id.cmp(&other.ask_id) // Compare by ask_id
    }
}

// CompletedProofs struct with multiple indexes and a counter
pub struct CompletedProofs {
    // Index by address (generator), with sorted sets of LocalAsks
    completed_by_generator: HashMap<Address, BTreeSet<LocalAsk>>,

    // Index by market_id, with sorted sets of LocalAsks
    completed_by_market: HashMap<U256, BTreeSet<LocalAsk>>,

    // Counter to track total number of completed proofs
    counter: usize,
}

impl CompletedProofs {
    pub fn new() -> Self {
        Self {
            completed_by_generator: HashMap::new(),
            completed_by_market: HashMap::new(),
            counter: 0,
        }
    }

    // Insert a new proof
    pub fn insert(&mut self, ask: LocalAsk) {
        if let Some(generator) = ask.generator {
            // Insert into the generator index
            let generator_set = self
                .completed_by_generator
                .entry(generator)
                .or_insert_with(BTreeSet::new);
            generator_set.insert(ask.clone()); // Clone to insert into multiple indexes

            // Insert into the market index
            let market_set = self
                .completed_by_market
                .entry(ask.market_id)
                .or_insert_with(BTreeSet::new);
            market_set.insert(ask); // No need to clone again here

            // Increment the counter
            self.counter += 1;
        }
    }

    // Get all proofs for a specific generator (across all markets)
    pub fn get_all_proofs_for_generator(&self, generator: &Address) -> Option<&BTreeSet<LocalAsk>> {
        self.completed_by_generator.get(generator)
    }

    // Get all proofs for a specific market (across all generators)
    pub fn get_all_proofs_for_market(&self, market_id: &U256) -> Option<&BTreeSet<LocalAsk>> {
        self.completed_by_market.get(market_id)
    }

    // Get the total number of proofs
    pub fn total_proofs(&self) -> usize {
        self.counter
    }

    pub fn get_recent_completed_proofs(&self, n: usize) -> Vec<LocalAsk> {
        let mut all_proofs: Vec<LocalAsk> = Vec::new();

        // Directly access the private field `completed_by_market`
        for market_set in self.completed_by_market.values() {
            all_proofs.extend(market_set.iter().cloned());
        }

        // Sort the proofs by ask_id in descending order (most recent first)
        all_proofs.sort_by(|a, b| b.ask_id.cmp(&a.ask_id));

        // Return the most recent n proofs
        all_proofs.into_iter().take(n).collect()
    }
}
