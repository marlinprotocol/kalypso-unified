use ethers::types::{Address, U256};
use im::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
    generator_lib::symbiotic_stake_store::{
        OperatorStakeManagement, SlashResult, SlashResultManagement, TokenLockManagement,
        VaultSnapshot, VaultSnapshotManagement,
    },
    utility::TokenTracker,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbioticStakeStore {
    pub operators: HashMap<Address, TokenTracker>,
    pub vault_snapshots: HashMap<U256, HashMap<U256, VaultSnapshot>>, // vault snapshot indexed with captures timestamps, then index
    pub slash_results: HashMap<U256, HashMap<U256, SlashResult>>, // slash result indexed with captures timestamps, then index
    pub vault_snapshot_indexes: Vec<U256>,
    pub slash_result_indexes: Vec<U256>,
    pub tokens_to_lock: TokenTracker,
}

impl SymbioticStakeStore {
    fn new() -> Self {
        Self {
            operators: HashMap::new(),
            tokens_to_lock: TokenTracker::new(),
            vault_snapshots: HashMap::new(),
            slash_results: HashMap::new(),
            vault_snapshot_indexes: vec![],
            slash_result_indexes: vec![],
        }
    }
}

impl Default for SymbioticStakeStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OperatorStakeManagement for SymbioticStakeStore {
    fn note_down_stake(
        &mut self,
        operator: &Address,
        token_address: &Address,
        absolute_stake: &U256,
    ) {
        // Check if the operator exists in the HashMap
        if let Some(token_tracker) = self.operators.get_mut(operator) {
            // If it exists, add to the existing token stake instead of replacing
            token_tracker.add_token(token_address, absolute_stake);
        } else {
            // If it does not exist, create a new TokenTracker and add the token and amount
            let mut new_tracker = TokenTracker::new();
            new_tracker.add_token(token_address, absolute_stake);
            self.operators.insert(*operator, new_tracker);
        }
    }

    fn get_complete_token_info(&self, operator: &Address) -> Option<&TokenTracker> {
        self.operators.get(operator)
    }

    fn get_latest_stake_info(&self, operator: &Address, token_address: &Address) -> U256 {
        let token_tracker = self.get_complete_token_info(operator);

        if token_tracker.is_none() {
            return 0.into();
        }

        let token_tracker = token_tracker.unwrap();

        token_tracker.get_balance(token_address)
    }

    fn clean_operators(&mut self) {
        self.operators = HashMap::new();
    }

    fn operators(&self) -> HashMap<Address, TokenTracker> {
        self.operators.clone()
    }
}

impl TokenLockManagement for SymbioticStakeStore {
    fn set_lock_token(&mut self, token: Address, amount: U256) {
        self.tokens_to_lock.force_set(token, amount);
    }

    fn remove_lock_token(&mut self, token: Address) {
        self.tokens_to_lock.force_remove(token);
    }

    fn tokens_to_lock(&self) -> TokenTracker {
        self.tokens_to_lock.clone()
    }
}

impl VaultSnapshotManagement for SymbioticStakeStore {
    fn store_vault_snapshot(
        &mut self,
        captured_timestamp: U256,
        index: U256,
        snapshot: VaultSnapshot,
    ) {
        self.vault_snapshots
            .entry(captured_timestamp)
            .or_insert_with(HashMap::new)
            .insert(index, snapshot);
        if !self.vault_snapshot_indexes.contains(&index) {
            self.vault_snapshot_indexes.push(index);
        }
    }

    fn get_vault_snapshot(&self, captured_timestamp: U256, index: U256) -> Option<&VaultSnapshot> {
        self.vault_snapshots
            .get(&captured_timestamp)
            .and_then(|snapshots| snapshots.get(&index))
    }

    fn get_all_vault_snapshots(&self, captured_timestamp: U256) -> Vec<VaultSnapshot> {
        self.vault_snapshots
            .get(&captured_timestamp)
            .map(|snapshots| snapshots.values().cloned().collect())
            .unwrap_or_default()
    }

    fn vault_snapshots(&self) -> HashMap<U256, HashMap<U256, VaultSnapshot>> {
        self.vault_snapshots.clone()
    }

    fn vault_snapshot_indexes(&self) -> Vec<U256> {
        self.vault_snapshot_indexes.clone()
    }
}
impl SlashResultManagement for SymbioticStakeStore {
    fn store_slash_result(&mut self, captured_timestamp: U256, index: U256, result: SlashResult) {
        self.slash_results
            .entry(captured_timestamp)
            .or_insert_with(HashMap::new)
            .insert(index, result);
        if !self.slash_result_indexes.contains(&index) {
            self.slash_result_indexes.push(index);
        }
    }

    fn get_slash_result(&self, captured_timestamp: U256, index: U256) -> Option<&SlashResult> {
        self.slash_results
            .get(&captured_timestamp)
            .and_then(|results| results.get(&index))
    }

    fn get_all_slash_results(&self, captured_timestamp: U256) -> Vec<SlashResult> {
        self.slash_results
            .get(&captured_timestamp)
            .map(|results| results.values().cloned().collect())
            .unwrap_or_default()
    }

    fn slash_result(&self) -> HashMap<U256, HashMap<U256, SlashResult>> {
        self.slash_results.clone()
    }

    fn slash_result_indexes(&self) -> Vec<U256> {
        self.slash_result_indexes.clone()
    }
}
