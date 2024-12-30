use ethers::types::{Address, U256};
use im::HashMap;
use serde::{Deserialize, Serialize};

use crate::utility::TokenTracker;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbioticStakeStore {
    operators: HashMap<Address, TokenTracker>,
    vault_snapshots: HashMap<U256, VaultSnapshot>, // vault snapshot indexed with captures timestamps
    pub tokens_to_lock: TokenTracker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultSnapshot {
    pub transmitter: Address,
    pub index: U256,
    pub captured_timestamp: U256,
    pub num_of_transactions: U256,
    pub image_id: Vec<u8>,
    pub snapshot_data: Vec<u8>,
    pub proof: Vec<u8>,
}

impl SymbioticStakeStore {
    pub fn new() -> Self {
        Self {
            operators: HashMap::new(),
            tokens_to_lock: TokenTracker::new(),
            vault_snapshots: HashMap::new(),
        }
    }
}

impl SymbioticStakeStore {
    pub fn upsert_stake(
        &mut self,
        operator: &Address,
        token_address: &Address,
        absolute_stake: &U256,
    ) {
        // Check if the operator exists in the HashMap
        if let Some(token_tracker) = self.operators.get_mut(operator) {
            // If it exists, replaces the token and amount to the existing TokenTracker
            token_tracker.replace_token(token_address, absolute_stake);
        } else {
            // If it does not exist, create a new TokenTracker and add the token and amount
            let mut new_tracker = TokenTracker::new();
            new_tracker.add_token(token_address, absolute_stake);
            self.operators.insert(*operator, new_tracker);
        }
    }

    pub fn get_complete_token_info(&self, operator: &Address) -> Option<&TokenTracker> {
        self.operators.get(operator)
    }

    pub fn get_latest_stake_info(&self, operator: &Address, token_address: &Address) -> U256 {
        let token_tracker = self.get_complete_token_info(operator);

        if token_tracker.is_none() {
            return 0.into();
        }

        let token_tracker = token_tracker.unwrap();

        token_tracker.get_balance(token_address)
    }
}

impl SymbioticStakeStore {
    pub fn set_lock_token(&mut self, token: Address, amount: U256) {
        self.tokens_to_lock.force_set(token, amount);
    }

    pub fn remove_lock_token(&mut self, token: Address) {
        self.tokens_to_lock.force_remove(token);
    }
}

impl SymbioticStakeStore {
    pub fn store_vault_snapshot(&mut self, captured_timestamp: U256, snapshot: VaultSnapshot) {
        self.vault_snapshots.insert(captured_timestamp, snapshot);
    }
}
