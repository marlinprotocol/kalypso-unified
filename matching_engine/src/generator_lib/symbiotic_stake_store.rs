use ethers::{
    abi::ParamType,
    types::{Address, U256},
};
use im::HashMap;
use serde::{Deserialize, Serialize};

use crate::utility::TokenTracker;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SymbioticStakeStore {
    pub operators: HashMap<Address, TokenTracker>,
    pub vault_snapshots: HashMap<U256, HashMap<U256, VaultSnapshot>>, // vault snapshot indexed with captures timestamps, then index
    pub slash_results: HashMap<U256, HashMap<U256, SlashResult>>, // slash result indexed with captures timestamps, then index
    pub vault_snapshot_indexes: Vec<U256>,
    pub slash_result_indexes: Vec<U256>,
    pub tokens_to_lock: TokenTracker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashResult {
    pub transmitter: Address,
    pub index: U256,
    pub captured_timestamp: U256,
    pub num_of_transactions: U256,
    pub image_id: Vec<u8>,
    pub slash_data: Vec<u8>,
    pub proof: Vec<u8>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorSnapshot {
    pub operator: Address,
    pub vault: Address,
    pub stake_token: Address,
    pub amount: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorSlashRecord {
    pub job_id: U256,
    pub operator: Address,
    pub reward_address: Address,
}

impl VaultSnapshot {
    pub fn get_attestation(self) -> Option<Vec<u8>> {
        let types = vec![ParamType::Bytes, ParamType::Bytes];

        // Decode the proof
        let decoded = ethers::abi::decode(&types, &self.proof).ok()?;

        // Extract bytes if decoding was successful
        let signature = decoded[0].clone().into_bytes().unwrap();
        let attestation_data = decoded[1].clone().into_bytes().unwrap();

        log::debug!("Signature: {}", hex::encode(signature));

        Some(attestation_data)
    }

    pub fn decode_vault_snapshot(self) -> Vec<OperatorSnapshot> {
        let stake_data_decoded = ethers::abi::decode(
            &[ParamType::Array(Box::new(ParamType::Tuple(vec![
                ParamType::Address,
                ParamType::Address,
                ParamType::Address,
                ParamType::Uint(256),
            ])))],
            &self.snapshot_data,
        );

        if stake_data_decoded.is_err() {
            return vec![];
        }

        let stake_data_decoded = stake_data_decoded.unwrap();

        if stake_data_decoded.len() == 0 {
            return vec![];
        }

        let stake_data_decoded = stake_data_decoded[0].clone().into_array();

        if stake_data_decoded.is_none() {
            return vec![];
        }

        let stake_data_decoded = stake_data_decoded.unwrap();

        let mut operator_stake_snapshots = vec![];

        for token in 0..stake_data_decoded.len() {
            let snapshot_tuple = stake_data_decoded[token].clone().into_tuple().unwrap();

            if snapshot_tuple.len() != 4 {
                continue;
            }

            let operator = match snapshot_tuple[0].clone().into_address() {
                Some(data) => data,
                None => continue,
            };
            let vault = match snapshot_tuple[1].clone().into_address() {
                Some(data) => data,
                None => continue,
            };
            let stake_token = match snapshot_tuple[2].clone().into_address() {
                Some(data) => data,
                None => continue,
            };
            let amount = match snapshot_tuple[3].clone().into_uint() {
                Some(data) => data,
                None => continue,
            };

            operator_stake_snapshots.push(OperatorSnapshot {
                operator,
                vault,
                stake_token,
                amount,
            });
        }

        operator_stake_snapshots
    }
}

impl SlashResult {
    pub fn get_attestation(self) -> Option<Vec<u8>> {
        let types = vec![ParamType::Bytes, ParamType::Bytes];

        // Decode the proof
        let decoded = ethers::abi::decode(&types, &self.proof).ok()?;

        // Extract bytes if decoding was successful
        let signature = decoded[0].clone().into_bytes().unwrap();
        let attestation_data = decoded[1].clone().into_bytes().unwrap();

        log::debug!("Signature: {}", hex::encode(signature));

        Some(attestation_data)
    }

    pub fn decode_slash_result(self) -> Vec<OperatorSlashRecord> {
        let slash_result_decoded = ethers::abi::decode(
            &[ParamType::Array(Box::new(ParamType::Tuple(vec![
                ParamType::Uint(256),
                ParamType::Address,
                ParamType::Address,
            ])))],
            &self.slash_data,
        );

        if slash_result_decoded.is_err() {
            return vec![];
        }

        let slash_result_decoded = slash_result_decoded.unwrap();

        if slash_result_decoded.len() == 0 {
            return vec![];
        }

        let slash_result_decoded = slash_result_decoded[0].clone().into_array();

        if slash_result_decoded.is_none() {
            return vec![];
        }

        let slash_result_decoded = slash_result_decoded.unwrap();

        let mut operator_slash_records = vec![];

        for token in 0..slash_result_decoded.len() {
            let operator_slash_record = slash_result_decoded[token].clone().into_tuple().unwrap();

            if operator_slash_record.len() != 3 {
                continue;
            }

            let job_id = match operator_slash_record[0].clone().into_uint() {
                Some(data) => data,
                None => continue,
            };
            let operator = match operator_slash_record[1].clone().into_address() {
                Some(data) => data,
                None => continue,
            };
            let reward_address = match operator_slash_record[2].clone().into_address() {
                Some(data) => data,
                None => continue,
            };

            operator_slash_records.push(OperatorSlashRecord {
                job_id,
                operator,
                reward_address,
            });
        }

        operator_slash_records
    }
}

impl SymbioticStakeStore {
    pub fn new() -> Self {
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

    pub fn clean_operators(&mut self) {
        self.operators = HashMap::new();
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
    pub fn store_vault_snapshot(
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

    pub fn get_vault_snapshot(
        &self,
        captured_timestamp: U256,
        index: U256,
    ) -> Option<&VaultSnapshot> {
        self.vault_snapshots
            .get(&captured_timestamp)
            .and_then(|snapshots| snapshots.get(&index))
    }

    pub fn get_all_vault_snapshots(&self, captured_timestamp: U256) -> Vec<VaultSnapshot> {
        self.vault_snapshots
            .get(&captured_timestamp)
            .map(|snapshots| snapshots.values().cloned().collect())
            .unwrap_or_default()
    }

    pub fn store_slash_result(
        &mut self,
        captured_timestamp: U256,
        index: U256,
        result: SlashResult,
    ) {
        self.slash_results
            .entry(captured_timestamp)
            .or_insert_with(HashMap::new)
            .insert(index, result);
        if !self.slash_result_indexes.contains(&index) {
            self.slash_result_indexes.push(index);
        }
    }

    pub fn get_slash_result(&self, captured_timestamp: U256, index: U256) -> Option<&SlashResult> {
        self.slash_results
            .get(&captured_timestamp)
            .and_then(|results| results.get(&index))
    }

    pub fn get_all_slash_results(&self, captured_timestamp: U256) -> Vec<SlashResult> {
        self.slash_results
            .get(&captured_timestamp)
            .map(|results| results.values().cloned().collect())
            .unwrap_or_default()
    }
}
