use ethers::core::types::U256;
use ethers::prelude::*;

use super::{
    ask::LocalAsk,
    ask_query::AskQueryResult,
    ask_status::{AskState, LocalAskStatus},
    Proof, RemoveReason,
};

/// Trait 1a – AskManagementWrite:
/// Contains functions for creating, updating, removing.
pub trait AskManagementWrite {
    fn insert(&mut self, ask: LocalAsk);
    fn remove_ask_only_if_completed(&mut self, ask_id: &U256, reason: RemoveReason);
    fn modify_state(&mut self, ask_id: &U256, new_state: AskState);
    fn update_ask_generator(&mut self, ask_id: &U256, new_generator: Option<Address>);
    fn update_ask_acl(&mut self, ask_id: &U256, new_acl: Option<Bytes>);
    fn update_deadline(&mut self, ask_id: &U256, deadline: U256);
    fn store_valid_proof(
        &mut self,
        ask_id: &U256,
        proof: Bytes,
        proof_time: U256,
        proof_cost: U256,
        proof_transaction: String,
    );
    fn note_invalid_inputs(&mut self, ask_id: &U256, proof_cost: U256, proof_transaction: String);
    fn note_proof_denied(&mut self, ask_id: &U256, proof_transaction: String);
}

/// Trait 1b – AskManagementRead:
/// Contains functions for querying asks.
pub trait AskManagementRead {
    fn get_proving_time(&self, ask_id: &U256) -> Option<U256>;
    fn get_proving_cost(&self, ask_id: &U256) -> Option<U256>;
    fn get_proof_transaction(&self, ask_id: &U256) -> Option<String>;
    fn get_proof_by_ask_id(&self, ask_id: &U256) -> Option<Proof>;
    fn get_by_market_id(&self, market_id: &U256) -> AskQueryResult;
    fn get_by_ask_state_except_complete(&self, state: AskState) -> AskQueryResult;
    fn get_cleanup_asks(&self) -> AskQueryResult;
    fn get_by_ask_id(&self, ask_id: &U256) -> Option<LocalAsk>;
    fn get_ask_status(&self) -> LocalAskStatus;
}

/// Trait 2 – RequestorCounters:
/// Provides functions to query requestor-related counters.
pub trait RequestorCounters {
    fn total_requestor_count(&self) -> usize;
    fn total_requestors_by_market_count(&self, market_id: &U256) -> usize;
}

/// Trait 3 – ProofCounters:
/// Functions to query the proof counters.
pub trait ProofCounters {
    fn get_proof_count(&self, market_id: &U256) -> usize;
    fn get_total_proof_count(&self) -> usize;
}

/// Trait 4 – MarketRequestCounters:
/// Functions for market-based request counts.
pub trait MarketRequestCounters {
    fn get_request_count_by_market_id(&self, market_id: &U256) -> usize;
    fn get_total_request_count(&self) -> usize;
}

/// Trait 5 – CompletedProofsManagement:
/// Combines failed request counters and completed proofs queries.
pub trait CompletedProofsManagement {
    fn get_failed_request_count_by_market_id(&self, market_id: &U256) -> usize;
    fn get_failed_request_count(&self) -> usize;
    fn get_recent_completed_proofs(&self, n: usize) -> Vec<LocalAsk>;
    fn get_completed_proof_of_generator(
        &self,
        generator: &Address,
        skip: usize,
        count: usize,
    ) -> Vec<LocalAsk>;
    fn get_completed_proofs_of_market(
        &self,
        market_id: &U256,
        skip: usize,
        count: usize,
    ) -> Vec<LocalAsk>;
}

/// Trait 6 – TimingOperations:
/// Includes the functions for handling proof cycle and job timestamp updates and queries.
pub trait TimingOperations {
    #[deprecated(note = "Preferably don't read it anywhere")]
    fn get_proof_proof_cycle_completed_on(&self, ask_id: &U256) -> Option<U256>;
    fn update_proof_proof_cycle_completed_on(&mut self, ask_id: &U256, submitted_on: U256);

    fn get_job_completed_on_timestamp(&self, ask_id: &U256) -> Option<U256>;
    fn update_job_completed_on_timestamp(&mut self, ask_id: &U256, completed_on_timestamp: U256);
    fn get_job_matched_on_timestamp(&self, ask_id: &U256) -> Option<U256>;
    fn update_job_matched_on_timestamp(&mut self, ask_id: &U256, matched_on_timestamp: U256);
    fn get_job_created_on_timestamp(&self, ask_id: &U256) -> Option<U256>;
    fn update_job_created_on_timestamp(&mut self, ask_id: &U256, created_on_timestamp: U256);
    fn get_overall_proving_time(&self, ask_id: &U256) -> Option<U256>;
}
