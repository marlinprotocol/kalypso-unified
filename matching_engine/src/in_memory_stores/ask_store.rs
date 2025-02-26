use ethers::core::types::U256;
use ethers::prelude::*;
use im::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
    ask_lib::{
        self,
        ask_store::{
            AskManagementRead, AskManagementWrite, CompletedProofsManagement,
            MarketRequestCounters, ProofCounters, ProofMarketStakeLockManagement,
            RequestorCounters, TimingOperations,
        },
        AssociatedStakeLock,
    },
    counters::counters::GenericCounters,
    utility::TokenTracker,
};

use ask_lib::{
    ask::LocalAsk,
    ask_query::AskQueryResult,
    ask_status::{AskState, LocalAskStatus},
    Proof, RemoveReason,
};

#[derive(Deserialize, Serialize, Clone)]
pub struct LocalAskStore {
    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    asks_by_id: HashMap<U256, LocalAsk>,

    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    market_id_index: HashMap<U256, Vec<LocalAsk>>,

    // default serde should work here
    state_index: HashMap<AskState, Vec<LocalAsk>>,

    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    proofs: HashMap<U256, Proof>,

    proof_counter_by_market: GenericCounters<U256, U256>, // Count by U256(i.e Proofs), Also sub-count by U256(i.e marketId)

    request_counter_by_requestors: GenericCounters<U256, Address>, // Count by Address(i.e requestor), Also sub-count by U256(i.e marketId)

    request_counter_by_market_id: GenericCounters<U256, U256>, // Count by U256(i.e Asks), Also sub-count by U256(i.e marketId)

    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    proving_time_taken: HashMap<U256, U256>,

    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    proving_cost_taken: HashMap<U256, U256>,

    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    proof_transaction: HashMap<U256, String>,

    failed_request_counter_by_market: GenericCounters<U256, U256>, // Count by U256(i.e AskId), Also sub-count by U256(i.e marketId)

    // default serde should work here
    completed_proofs: CompletedProofs,

    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    proof_cycle_completed_on: HashMap<U256, U256>,

    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    job_created_on_timestamp: HashMap<U256, U256>,

    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    job_matched_on_timestamp: HashMap<U256, U256>,

    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    job_completed_on_timestamp: HashMap<U256, U256>,

    associated_stake_locks: HashMap<U256, AssociatedStakeLock>,
}

use crate::utility::deserialize_u256_map;
use crate::utility::serialize_u256_map;

use super::completed_proofs::{CompletedProofs, CompletedProofsTrait};

impl LocalAskStore {
    fn new() -> Self {
        LocalAskStore {
            asks_by_id: HashMap::new(),
            market_id_index: HashMap::new(),
            state_index: HashMap::new(),
            proofs: HashMap::new(),
            proof_counter_by_market: GenericCounters::new(),
            request_counter_by_requestors: GenericCounters::new(),
            request_counter_by_market_id: GenericCounters::new(),
            proving_cost_taken: HashMap::new(),
            proving_time_taken: HashMap::new(),
            proof_transaction: HashMap::new(),
            failed_request_counter_by_market: GenericCounters::new(),
            completed_proofs: CompletedProofs::new(),
            proof_cycle_completed_on: HashMap::new(),
            job_created_on_timestamp: HashMap::new(),
            job_matched_on_timestamp: HashMap::new(),
            job_completed_on_timestamp: HashMap::new(),
            associated_stake_locks: HashMap::new(),
        }
    }
}

impl Default for LocalAskStore {
    fn default() -> Self {
        Self::new()
    }
}

impl AskManagementWrite for LocalAskStore {
    fn insert(&mut self, ask: LocalAsk) {
        self.asks_by_id.insert(ask.ask_id, ask.clone());
        self.request_counter_by_requestors
            .insert(ask.market_id, ask.prover_refund_address);

        self.request_counter_by_market_id
            .insert(ask.market_id, ask.ask_id);

        self.market_id_index
            .entry(ask.market_id)
            .or_default()
            .push(ask.clone());

        if let Some(state) = ask.state {
            self.state_index.entry(state).or_default().push(ask);
        }
    }

    #[allow(unused)]
    fn remove_ask_only_if_completed(&mut self, ask_id: &U256, reason: RemoveReason) {
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

            // Insert the completed ask into the HashSet only in complete mode. otherwise enclave size will shoot up in runtime
            #[cfg(feature = "complete")]
            {
                // Store the completed proof
                self.completed_proofs.insert(ask, reason);
            }
        }
    }

    fn modify_state(&mut self, ask_id: &U256, new_state: AskState) {
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

    fn update_ask_generator(&mut self, ask_id: &U256, new_generator: Option<Address>) {
        if let Some(ask) = self.asks_by_id.get_mut(ask_id) {
            ask.generator = new_generator;
        }
    }

    fn update_ask_acl(&mut self, ask_id: &U256, new_acl: Option<Bytes>) {
        if let Some(ask) = self.asks_by_id.get_mut(ask_id) {
            ask.secret_acl = new_acl;
        }
    }

    fn update_deadline(&mut self, ask_id: &U256, deadline: U256) {
        if let Some(ask) = self.asks_by_id.get_mut(ask_id) {
            ask.deadline = deadline;
        }
    }

    fn store_valid_proof(
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

    fn note_invalid_inputs(&mut self, ask_id: &U256, proof_cost: U256, proof_transaction: String) {
        match self.asks_by_id.get_mut(ask_id) {
            Some(ask_data) => {
                self.proofs.insert(*ask_id, Proof::InvalidInputAttestation);
                self.failed_request_counter_by_market
                    .insert(ask_data.market_id, ask_data.ask_id);
                self.proof_transaction.insert(*ask_id, proof_transaction);
                self.proving_cost_taken.insert(*ask_id, proof_cost);
            }
            _ => {}
        }
    }

    fn note_proof_denied(&mut self, ask_id: &U256, proof_transaction: String) {
        match self.asks_by_id.get_mut(ask_id) {
            Some(ask_data) => {
                self.proofs.insert(*ask_id, Proof::FailedProofGeneration);
                self.failed_request_counter_by_market
                    .insert(ask_data.market_id, ask_data.ask_id);
                self.proof_transaction.insert(*ask_id, proof_transaction);
            }
            _ => {}
        }
    }
}

impl AskManagementRead for LocalAskStore {
    fn get_proving_time(&self, ask_id: &U256) -> Option<U256> {
        self.proving_time_taken.get(ask_id).cloned()
    }
    fn get_proving_cost(&self, ask_id: &U256) -> Option<U256> {
        self.proving_cost_taken.get(ask_id).cloned()
    }

    fn get_proof_transaction(&self, ask_id: &U256) -> Option<String> {
        self.proof_transaction.get(ask_id).cloned()
    }

    fn get_proof_by_ask_id(&self, ask_id: &U256) -> Option<Proof> {
        self.proofs.get(ask_id).cloned()
    }

    fn get_by_market_id(&self, market_id: &U256) -> AskQueryResult {
        AskQueryResult {
            asks: self.market_id_index.get(market_id).cloned(),
        }
    }

    fn get_by_ask_state_except_complete(&self, state: AskState) -> AskQueryResult {
        if state == AskState::Complete {
            return AskQueryResult { asks: None };
        }
        AskQueryResult {
            asks: self.state_index.get(&state).cloned(),
        }
    }

    fn get_cleanup_asks(&self) -> AskQueryResult {
        AskQueryResult {
            asks: self.state_index.get(&AskState::Complete).cloned(),
        }
    }

    fn get_by_ask_id(&self, ask_id: &U256) -> Option<LocalAsk> {
        self.asks_by_id.get(ask_id).cloned()
    }

    fn get_ask_status(&self) -> LocalAskStatus {
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

impl RequestorCounters for LocalAskStore {
    // Get the total number of unique requestors across all markets
    fn total_requestor_count(&self) -> usize {
        self.request_counter_by_requestors.total_count()
    }

    // Get the number of requestors for a specific market
    fn total_requestors_by_market_count(&self, market_id: &U256) -> usize {
        self.request_counter_by_requestors.key_count(market_id)
    }
}

impl ProofCounters for LocalAskStore {
    fn get_proof_count(&self, market_id: &U256) -> usize {
        self.proof_counter_by_market.key_count(market_id)
    }

    fn get_total_proof_count(&self) -> usize {
        self.proof_counter_by_market.total_count()
    }
}

impl MarketRequestCounters for LocalAskStore {
    fn get_request_count_by_market_id(&self, market_id: &U256) -> usize {
        self.request_counter_by_market_id.key_count(market_id)
    }

    fn get_total_request_count(&self) -> usize {
        self.request_counter_by_market_id.total_count()
    }
}

impl CompletedProofsManagement for LocalAskStore {
    fn get_failed_request_count_by_market_id(&self, market_id: &U256) -> usize {
        self.failed_request_counter_by_market.key_count(market_id)
    }

    fn get_failed_request_count(&self) -> usize {
        self.failed_request_counter_by_market.total_count()
    }

    fn get_recent_completed_proofs(&self, n: usize) -> Vec<LocalAsk> {
        self.completed_proofs.get_recent_completed_proofs(n)
    }

    fn get_completed_proof_of_generator(
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

    fn get_completed_proofs_of_market(
        &self,
        market_id: &U256,
        skip: usize,
        count: usize,
    ) -> Vec<LocalAsk> {
        if let Some(market_proofs) = self.completed_proofs.get_all_proofs_for_market(market_id) {
            // Skip the specified number of results and take 'count' results
            market_proofs
                .iter()
                .rev()
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

impl TimingOperations for LocalAskStore {
    fn get_proof_proof_cycle_completed_on(&self, ask_id: &U256) -> Option<U256> {
        self.proof_cycle_completed_on.get(ask_id).cloned()
    }

    fn update_proof_proof_cycle_completed_on(&mut self, ask_id: &U256, submitted_on: U256) {
        self.proof_cycle_completed_on
            .insert(ask_id.clone(), submitted_on);
    }

    fn get_job_completed_on_timestamp(&self, ask_id: &U256) -> Option<U256> {
        self.job_completed_on_timestamp.get(ask_id).cloned()
    }

    fn update_job_completed_on_timestamp(&mut self, ask_id: &U256, completed_on_timestamp: U256) {
        self.job_completed_on_timestamp
            .insert(ask_id.clone(), completed_on_timestamp);
    }

    fn get_job_matched_on_timestamp(&self, ask_id: &U256) -> Option<U256> {
        self.job_matched_on_timestamp.get(ask_id).cloned()
    }

    fn update_job_matched_on_timestamp(&mut self, ask_id: &U256, matched_on_timestamp: U256) {
        self.job_matched_on_timestamp
            .insert(ask_id.clone(), matched_on_timestamp);
    }

    fn get_job_created_on_timestamp(&self, ask_id: &U256) -> Option<U256> {
        self.job_created_on_timestamp.get(ask_id).cloned()
    }

    fn update_job_created_on_timestamp(&mut self, ask_id: &U256, created_on_timestamp: U256) {
        self.job_created_on_timestamp
            .insert(ask_id.clone(), created_on_timestamp);
    }

    fn get_overall_proving_time(&self, ask_id: &U256) -> Option<U256> {
        let created_on = self.get_job_created_on_timestamp(ask_id)?;
        let completed_on = self.get_job_completed_on_timestamp(ask_id)?;
        Some(completed_on.saturating_sub(created_on))
    }
}

impl ProofMarketStakeLockManagement for LocalAskStore {
    fn get_associated_stake_lock(&self, ask_id: &U256) -> Option<AssociatedStakeLock> {
        self.associated_stake_locks.get(ask_id).cloned()
    }

    fn add_associated_native_stake_lock(&mut self, ask_id: &U256, stake_locked: TokenTracker) {
        self.associated_stake_locks
            .entry(*ask_id)
            .and_modify(|lock| {
                lock.native = lock.native.clone() + stake_locked.clone();
            })
            .or_insert_with(|| AssociatedStakeLock {
                native: stake_locked,
                symbiotic: TokenTracker::default(),
            });
    }

    fn add_associated_symbiotic_stake_lock(&mut self, ask_id: &U256, stake_locked: TokenTracker) {
        self.associated_stake_locks
            .entry(*ask_id)
            .and_modify(|lock| {
                lock.symbiotic = lock.symbiotic.clone() + stake_locked.clone();
            })
            .or_insert_with(|| AssociatedStakeLock {
                native: TokenTracker::default(),
                symbiotic: stake_locked,
            });
    }

    fn delete_all_associated_stake_locks(&mut self, ask_id: &U256) {
        self.associated_stake_locks.remove(ask_id);
    }
}
