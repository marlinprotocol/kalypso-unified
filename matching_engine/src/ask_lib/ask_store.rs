use ethers::core::types::U256;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::counters::counters::GenericCounters;

use super::{
    ask::{CompletedProofs, LocalAsk},
    ask_status::{AskState, Comparison, LocalAskStatus},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Proof {
    ValidProof(Bytes),
    // latter we may need to store the invalid input attestations here
}
pub struct LocalAskStore {
    asks_by_id: HashMap<U256, LocalAsk>,
    market_id_index: HashMap<U256, Vec<LocalAsk>>,
    state_index: HashMap<AskState, Vec<LocalAsk>>,
    proofs: HashMap<U256, Proof>,
    proof_counter_by_market: GenericCounters<U256, U256>, // Count by U256(i.e AskId), Also sub-count by U256(i.e marketId)
    request_counter_by_requestors: GenericCounters<U256, Address>, // Count by Address(i.e requestor), Also sub-count by U256(i.e marketId)
    proving_time_taken: HashMap<U256, U256>,
    proving_cost_taken: HashMap<U256, U256>,
    proof_transaction: HashMap<U256, String>,
    failed_request_counter_by_market: GenericCounters<U256, U256>, // Count by U256(i.e AskId), Also sub-count by U256(i.e marketId)
    completed_proofs: CompletedProofs,
}

pub struct AskQueryResult {
    asks: Option<Vec<LocalAsk>>,
}

impl AskQueryResult {
    #[allow(unused)]
    pub fn sort_by_expiry(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.sort_by(|a, b| a.expiry.cmp(&b.expiry));
        }
        self
    }

    pub fn result(self) -> Option<Vec<LocalAsk>> {
        self.asks
    }

    pub fn sort_by_ask_id(mut self, asc: bool) -> Self {
        if let Some(ref mut asks) = self.asks {
            if asc {
                // Sort in ascending order
                asks.sort_by(|a, b| a.ask_id.cmp(&b.ask_id));
            } else {
                // Sort in descending order
                asks.sort_by(|a, b| b.ask_id.cmp(&a.ask_id));
            }
        }
        self
    }

    pub fn filter_by_market_id(self, market_id: U256) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter() // Parallel iterator over asks
                .filter(|ask| ask.market_id == market_id) // Filter by market_id
                .collect::<Vec<_>>() // Collect filtered results
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn sort_by_reward(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.sort_by(|a, b| a.reward.cmp(&b.reward));
        }
        self
    }

    #[allow(unused)]
    pub fn sort_by_deadline(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.sort_by(|a, b| a.deadline.cmp(&b.deadline));
        }
        self
    }

    #[allow(unused)]
    pub fn filter_by_has_private_inputs(self, value: bool) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| ask.has_private_inputs == value)
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    pub fn filter_by_flag(self, value: bool) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| ask.invalid_secret_flag == value)
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    fn compare(value: U256, other: U256, comparison: &Comparison) -> bool {
        match comparison {
            Comparison::Equal => value == other,
            Comparison::LessThan => value < other,
            Comparison::GreaterThan => value > other,
            Comparison::LessThanOrEqual => value <= other,
            Comparison::GreaterThanOrEqual => value >= other,
        }
    }

    #[allow(unused)]
    pub fn filter_by_expiry(self, value: U256, comparison: Comparison) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| Self::compare(ask.expiry, value, &comparison))
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn filter_by_reward(self, value: U256, comparison: Comparison) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| Self::compare(ask.reward, value, &comparison))
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn filter_by_deadline(self, value: U256, comparison: Comparison) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| Self::compare(ask.deadline, value, &comparison))
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn filter_by_prover_refund_address(self, address: Address) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| ask.prover_refund_address == address)
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    pub fn get_count(self) -> usize {
        self.asks.map(|v| v.len()).unwrap_or(0)
    }
}

impl Default for LocalAskStore {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalAskStore {
    pub fn new() -> Self {
        LocalAskStore {
            asks_by_id: HashMap::new(),
            market_id_index: HashMap::new(),
            state_index: HashMap::new(),
            proofs: HashMap::new(),
            proof_counter_by_market: GenericCounters::new(),
            request_counter_by_requestors: GenericCounters::new(),
            proving_cost_taken: HashMap::new(),
            proving_time_taken: HashMap::new(),
            proof_transaction: HashMap::new(),
            failed_request_counter_by_market: GenericCounters::new(),
            completed_proofs: CompletedProofs::new(),
        }
    }

    pub fn insert(&mut self, ask: LocalAsk) {
        self.asks_by_id.insert(ask.ask_id, ask.clone());
        self.request_counter_by_requestors
            .insert(ask.market_id, ask.prover_refund_address);

        self.market_id_index
            .entry(ask.market_id)
            .or_default()
            .push(ask.clone());

        if let Some(state) = ask.state {
            self.state_index.entry(state).or_default().push(ask);
        }
    }

    pub fn remove_ask_only_if_completed(&mut self, ask_id: &U256) {
        if let Some(ask) = self.asks_by_id.remove(ask_id) {
            // Check if the ask's state is Some and Complete, else return early
            if ask.state != Some(AskState::Complete) {
                return;
            }

            // Remove from market_id_index if it exists
            if let Some(vec) = self.market_id_index.get_mut(&ask.market_id) {
                vec.retain(|a| a.ask_id != *ask_id);
            }

            // Remove from state_index if it exists
            if let Some(state) = ask.state {
                if let Some(vec) = self.state_index.get_mut(&state) {
                    vec.retain(|a| a.ask_id != *ask_id);
                }
            }

            // Insert the completed ask into the HashSet
            self.completed_proofs.insert(ask);
        }
    }

    pub fn modify_state(&mut self, ask_id: &U256, new_state: AskState) {
        if let Some(ask) = self.asks_by_id.get_mut(ask_id) {
            if let Some(old_state) = ask.state.take() {
                if let Some(vec) = self.state_index.get_mut(&old_state) {
                    vec.retain(|a| a.ask_id != *ask_id);
                }
            }

            ask.state = Some(new_state);
            self.state_index
                .entry(new_state)
                .or_default()
                .push(ask.clone());
        }
    }

    pub fn update_ask_generator(&mut self, ask_id: &U256, new_generator: Option<Address>) {
        if let Some(ask) = self.asks_by_id.get_mut(ask_id) {
            ask.generator = new_generator;
        }
    }

    pub fn update_ask_acl(&mut self, ask_id: &U256, new_acl: Option<Bytes>) {
        if let Some(ask) = self.asks_by_id.get_mut(ask_id) {
            ask.secret_acl = new_acl;
        }
    }

    pub fn store_valid_proof(
        &mut self,
        ask_id: &U256,
        proof: Bytes,
        proof_time: U256,
        proof_cost: U256,
        proof_transaction: String,
    ) {
        match self.asks_by_id.get_mut(ask_id) {
            Some(ask_data) => {
                self.proofs.insert(*ask_id, Proof::ValidProof(proof));
                self.proof_counter_by_market
                    .insert(ask_data.market_id, ask_data.ask_id);
                self.proving_time_taken.insert(*ask_id, proof_time);
                self.proving_cost_taken.insert(*ask_id, proof_cost);
                self.proof_transaction.insert(*ask_id, proof_transaction);
            }
            _ => {}
        }
    }

    pub fn note_invalid_proof(&mut self, ask_id: &U256) {
        match self.asks_by_id.get_mut(ask_id) {
            Some(ask_data) => {
                self.failed_request_counter_by_market
                    .insert(ask_data.market_id, ask_data.ask_id);
            }
            _ => {}
        }
    }

    pub fn get_proving_time(&self, ask_id: &U256) -> Option<U256> {
        self.proving_time_taken.get(ask_id).cloned()
    }
    pub fn get_proving_cost(&self, ask_id: &U256) -> Option<U256> {
        self.proving_cost_taken.get(ask_id).cloned()
    }

    pub fn get_proof_transaction(&self, ask_id: &U256) -> Option<String> {
        self.proof_transaction.get(ask_id).cloned()
    }

    pub fn get_proof_by_ask_id(&self, ask_id: &U256) -> Option<Proof> {
        self.proofs.get(ask_id).cloned()
    }

    pub fn get_by_market_id(&self, market_id: &U256) -> AskQueryResult {
        AskQueryResult {
            asks: self.market_id_index.get(market_id).cloned(),
        }
    }

    pub fn get_by_ask_state_except_complete(&self, state: AskState) -> AskQueryResult {
        if state == AskState::Complete {
            return AskQueryResult { asks: None };
        }
        AskQueryResult {
            asks: self.state_index.get(&state).cloned(),
        }
    }

    pub fn get_cleanup_asks(&self) -> AskQueryResult {
        AskQueryResult {
            asks: self.state_index.get(&AskState::Complete).cloned(),
        }
    }

    #[allow(unused)]
    pub fn get_by_ask_id(&self, ask_id: &U256) -> Option<&LocalAsk> {
        self.asks_by_id.get(ask_id)
    }

    pub fn get_ask_status(&self) -> LocalAskStatus {
        let created = self
            .get_by_ask_state_except_complete(AskState::Create)
            .get_count();
        let unassigned = self
            .get_by_ask_state_except_complete(AskState::UnAssigned)
            .get_count();
        let assigned = self
            .get_by_ask_state_except_complete(AskState::Assigned)
            .get_count();
        let completed = self.completed_proofs.total_proofs();
        let deadline_crossed = self
            .get_by_ask_state_except_complete(AskState::DeadlineCrossed)
            .get_count();
        let invalid_secret = self
            .get_by_ask_state_except_complete(AskState::InvalidSecret)
            .get_count();

        LocalAskStatus {
            created,
            unassigned,
            assigned,
            completed,
            deadline_crossed,
            invalid_secret,
        }
    }
}

impl LocalAskStore {
    // Get the total number of unique requestors across all markets
    pub fn total_requestor_count(&self) -> usize {
        self.request_counter_by_requestors.total_count()
    }

    // Get the number of requestors for a specific market
    pub fn total_requestors_by_market_count(&self, market_id: &U256) -> usize {
        self.request_counter_by_requestors.key_count(market_id)
    }
}

impl LocalAskStore {
    pub fn get_proof_count(&self, market_id: &U256) -> usize {
        self.proof_counter_by_market.key_count(market_id)
    }

    pub fn get_total_proof_count(&self) -> usize {
        self.proof_counter_by_market.total_count()
    }
}

impl LocalAskStore {
    pub fn get_failed_request_count_by_market_id(&self, market_id: &U256) -> usize {
        self.failed_request_counter_by_market.key_count(market_id)
    }

    pub fn get_failed_request_count(&self) -> usize {
        self.failed_request_counter_by_market.total_count()
    }
}

impl LocalAskStore {
    pub fn get_recent_completed_proofs(&self, n: usize) -> Vec<LocalAsk> {
        self.completed_proofs.get_recent_completed_proofs(n)
    }

    pub fn get_completed_proof_of_generator(
        &self,
        generator: &Address,
        skip: usize,
        count: usize,
    ) -> Vec<LocalAsk> {
        if let Some(generator_proofs) = self
            .completed_proofs
            .get_all_proofs_for_generator(generator)
        {
            // Skip the specified number of results and take 'count' results
            generator_proofs
                .iter()
                .skip(skip) // Skip the first 'skip' elements
                .take(count) // Take the next 'count' elements
                .cloned() // Clone the elements since we're returning Vec<LocalAsk>
                .collect()
        } else {
            // If no proofs exist for this generator, return an empty vector
            Vec::new()
        }
    }
}
