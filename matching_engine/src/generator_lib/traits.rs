// generator_traits.rs

use ethers::core::types::{U256, U64};
use ethers::prelude::*;
use tokio::sync::RwLockReadGuard;

use crate::utility::TokenTracker;

use super::delegation::{Delegation, Operation, Source};
use super::generator_query::GeneratorQueryResult;
use super::generator_state::GeneratorState;
use super::generator_store::{Generator, GeneratorInfoPerMarket};
use super::key_store::KeyStoreOperations;
use super::withdrawal_request::WithdrawlRequest;
use super::SlashingRecord;

/// GeneratorRegistration:
/// Handles registration, removal, and basic queries for generators.
pub trait GeneratorRegistration {
    fn register_generator(&mut self, generator: Generator);
    fn register_generator_in_market(&mut self, generator_market: GeneratorInfoPerMarket);
    fn remove_by_address_and_market(&mut self, address: &Address, market_id: &U256);
    fn remove_by_address(&mut self, address: &Address);
    fn get_by_address(&self, address: &Address) -> Option<Generator>;
    fn is_active(&self, generator_address: &Address) -> bool;
}

/// GeneratorStakeComputeManagement:
/// Handles stake and compute operations such as adding, updating, and removing stakes
/// as well as compute amounts.
pub trait GeneratorStakeComputeManagement {
    fn add_extra_stake(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        amount: &U256,
        block_number: U64,
        transaction_index: U64,
        log_index: U256,
        tx: String,
        source: Source,
    );
    fn update_intended_stake_util(&mut self, address: &Address, new_stake_util: U256);
    fn remove_stake(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        amount: &U256,
        block_number: U64,
        transaction_index: U64,
        log_index: U256,
        tx: String,
        operation: Operation,
        source: Source,
    );
    fn update_reward_address(&mut self, address: &Address, new_reward_address: Address);
    fn add_extra_compute(&mut self, address: &Address, compute: U256);
    fn update_intended_compute_util(&mut self, address: &Address, new_compute_util: U256);
    fn remove_compute(&mut self, address: &Address, compute: U256);
}

/// GeneratorMarketManagement:
/// Handles market participation and state updates such as assignment,
/// proof submission, and pausing/resuming participation.
pub trait GeneratorMarketManagement {
    fn update_state(&mut self, address: &Address, market_id: &U256, new_state: GeneratorState);
    fn update_on_assigned_task(&mut self, address: &Address, market_id: &U256);
    fn update_on_submit_proof(
        &mut self,
        address: &Address,
        market_id: &U256,
        earning: &U256,
        block_number: &U64,
    );
    fn reduce_active_requests(&mut self, generator_address: &Address, market_id: &U256);
    fn pause_assignments_across_all_markets(&mut self, address: &Address);
    fn resume_assignments_accross_all_markets(&mut self, address: &Address);
}

/// GeneratorSlashingManagement:
/// Encapsulates functionality for recording slashing events.
pub trait GeneratorSlashingManagement {
    fn note_entry_slashing(
        &mut self,
        generator_address: &Address,
        ask_id: &U256,
        market_id: &U256,
        native_tokens_slashed: Vec<Address>,
        native_slashings: Vec<U256>,
        symbiotic_tokens_slashed: Vec<Address>,
        symbiotic_slashings: Vec<U256>,
        slashing_tx: String,
        price_offered: &U256,
        deadline: &U256,
        slashing_block_number: &U64,
        slashing_timestamp: &U256,
    );
}

/// GeneratorLockManagement:
/// Manages stake and compute locks (locking and releasing amounts).
pub trait GeneratorLockManagement {
    fn update_on_stake_locked(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_locked: U256,
        source: Source,
    );
    fn update_on_stake_released(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_released: U256,
        source: Source,
    );
    fn update_on_compute_locked(&mut self, address: &Address, compute_locked: U256);
    fn update_on_compute_released(&mut self, address: &Address, compute_released: U256);
}

/// GeneratorAvailability:
/// Provides queries for available compute, stakes, and locked amounts.
pub trait GeneratorAvailability {
    fn get_available_compute(&self, address: Address) -> Option<U256>;
    fn get_available_native_stake(&self, generator_address: &Address) -> Option<TokenTracker>;
    fn get_available_symbiotic_stake(&self, generator_address: &Address) -> Option<TokenTracker>;
    fn get_native_stake_locked(&self, generator_address: &Address) -> Option<TokenTracker>;
    fn get_symbiotic_stake_locked(&self, generator_address: &Address) -> Option<TokenTracker>;
    fn get_all_by_market_id(&self, market_id: &U256) -> Vec<GeneratorInfoPerMarket>;
}

/// GeneratorQuery:
/// Offers basic queries over generator data.
pub trait GeneratorQuery {
    fn query(&self) -> GeneratorQueryResult;
    fn query_by_market_id_and_only_active(&self, market_id: &U256) -> GeneratorQueryResult;
    fn query_by_states(&self, states: Vec<GeneratorState>) -> GeneratorQueryResult;
    fn query_by_address(&self, address: Address) -> GeneratorQueryResult;
}

/// GeneratorFilter:
/// Provides methods to filter generator query results based on compute, stake, or input support.
pub trait GeneratorFilter {
    fn filter_by_has_idle_compute(
        &self,
        generator_query: GeneratorQueryResult,
    ) -> GeneratorQueryResult;
    fn filter_by_available_native_stake(
        &self,
        generator_query: GeneratorQueryResult,
        min_stake: Vec<(Address, U256)>,
    ) -> GeneratorQueryResult;
    fn filter_by_available_symbiotic_stake(
        &self,
        generator_query: GeneratorQueryResult,
        min_stake: Vec<(Address, U256)>,
    ) -> GeneratorQueryResult;
}

pub trait GeneratorKeyStoreFilterInterfaceTrait<KS: KeyStoreOperations> {
    fn filter_by_has_private_inputs_support(
        &self,
        generator_query: GeneratorQueryResult,
        key_store: RwLockReadGuard<'_, KS>,
    ) -> GeneratorQueryResult;
}

/// GeneratorEarningsAndSlashing:
/// Provides methods to query earnings, points, and slashing information.
pub trait GeneratorEarningsAndSlashing {
    fn get_total_earning(&self, address: &Address) -> Option<U256>;
    fn get_earning_per_market(&self, address: &Address, market_id: &U256) -> Option<U256>;
    fn get_kalypso_points(&self, address: &Address) -> Option<U256>;
    fn get_kalypso_points_per_market(&self, address: &Address, market_id: &U256) -> Option<U256>;
    fn get_total_slashing(&self, generator_address: &Address) -> Option<TokenTracker>;
    fn get_slashing_per_generator_per_market(
        &self,
        address: &Address,
        market_id: &U256,
    ) -> Option<TokenTracker>;
    fn get_slashing_records(&self, address: &Address) -> Vec<SlashingRecord>;
}

/// WithdrawalManagement:
/// Handles insertion, retrieval, and removal of withdrawal requests.
pub trait WithdrawalManagement {
    fn insert_withdrawal_request(
        &mut self,
        operator_address: &Address,
        withdrawal_request: WithdrawlRequest,
    );
    fn get_withdrawl_requests(&self, operator_address: &Address) -> Vec<WithdrawlRequest>;
    fn remove_withdrawal_request(
        &mut self,
        operator_address: &Address,
        withdrawal_request: WithdrawlRequest,
    ) -> bool;
}

/// GeneratorMetadata:
/// Provides a method to update generator metadata.
pub trait GeneratorMetadata {
    fn update_generator_metadata(&mut self, generator_address: Address, generator_meta_data: Bytes);
}

/// JobMissedCounter:
/// Handles counting jobs missed by a generator.
pub trait JobMissedCounter {
    fn count_job_missed_by_generator(
        &mut self,
        generator_address: Address,
        timestamp: std::time::SystemTime,
    );
    fn get_job_missed_count(&self, generator_address: &Address) -> usize;
}

pub trait GeneratorAdditionalQuery {
    /// Returns a vector of all generator addresses.
    fn all_generators_address(&self) -> Vec<Address>;

    /// Retrieves delegations for a given generator address filtered by a set of operations,
    /// with optional pagination.
    fn get_delegations(
        &self,
        generator_address: &Address,
        operations: Vec<Operation>,
        skip: Option<usize>,
        count: Option<usize>,
    ) -> Vec<Delegation>;

    /// Returns the total native stake across all generators.
    fn total_native_stake_accross_all_generators(&self) -> TokenTracker;

    /// Returns the total symbiotic stake across all generators.
    fn total_symbiotic_stake_across_all_generators(&self) -> TokenTracker;

    /// Retrieves generator market info for a specific generator address and market.
    fn get_by_address_and_market(
        &self,
        address: &Address,
        market_id: &U256,
    ) -> Option<GeneratorInfoPerMarket>;

    /// Returns all market participations for a given generator address.
    fn get_all_markets_of_generator(&self, address: &Address) -> Vec<GeneratorInfoPerMarket>;
}
