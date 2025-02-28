use crate::ask_lib::ask::LocalAsk;
use crate::ask_lib::RemoveReason;
use crate::utility::deserialize_u256_map;
use crate::utility::serialize_u256_map;
use ethers::core::types::U256;
use ethers::prelude::*;
use im::HashMap;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

// CompletedProofs struct with multiple indexes and a counter
#[derive(Serialize, Deserialize, Clone)]
pub struct CompletedProofs {
    // Index by address (generator), with sorted sets of LocalAsks
    completed_by_generator: HashMap<Address, BTreeSet<LocalAsk>>,

    // Index by market_id, with sorted sets of LocalAsks
    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    completed_by_market: HashMap<U256, BTreeSet<LocalAsk>>,

    // Counter to track total number of completed proofs
    counter: usize,
}

impl CompletedProofsTrait for CompletedProofs {
    fn new() -> Self {
        Self {
            completed_by_generator: HashMap::new(),
            completed_by_market: HashMap::new(),
            counter: 0,
        }
    }

    // Insert a new proof
    fn insert(&mut self, ask: LocalAsk, reason: RemoveReason) {
        // store only relevant ones
        if reason == RemoveReason::ProofNotGenerated
            || reason == RemoveReason::InvalidInputsDetected
            || reason == RemoveReason::BidCancelled
        {
            return;
        }

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
    fn get_all_proofs_for_generator(&self, generator: &Address) -> Option<&BTreeSet<LocalAsk>> {
        self.completed_by_generator.get(generator)
    }

    // Get all proofs for a specific market (across all generators)
    fn get_all_proofs_for_market(&self, market_id: &U256) -> Option<&BTreeSet<LocalAsk>> {
        self.completed_by_market.get(market_id)
    }

    // Get the total number of proofs
    fn total_proofs(&self) -> usize {
        self.counter
    }

    fn get_recent_completed_proofs(&self, n: usize) -> Vec<LocalAsk> {
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

pub trait CompletedProofsTrait {
    fn new() -> Self;
    fn insert(&mut self, ask: LocalAsk, reason: RemoveReason);
    fn get_all_proofs_for_generator(&self, generator: &Address) -> Option<&BTreeSet<LocalAsk>>;
    fn get_all_proofs_for_market(&self, market_id: &U256) -> Option<&BTreeSet<LocalAsk>>;
    fn total_proofs(&self) -> usize;
    fn get_recent_completed_proofs(&self, n: usize) -> Vec<LocalAsk>;
}
