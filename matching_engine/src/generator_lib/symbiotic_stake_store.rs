use ethers::{
    abi::ParamType,
    types::{Address, U256},
};
use im::HashMap;
use serde::{Deserialize, Serialize};

use crate::utility::TokenTracker;

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

pub trait OperatorStakeManagement {
    /// Record the stake for a given operator.
    fn note_down_stake(
        &mut self,
        operator: &Address,
        token_address: &Address,
        absolute_stake: &U256,
    );

    /// Get the complete token tracking info for an operator.
    fn get_complete_token_info(&self, operator: &Address) -> Option<&TokenTracker>;

    /// Retrieve the latest stake balance for an operator for a specific token.
    fn get_latest_stake_info(&self, operator: &Address, token_address: &Address) -> U256;

    /// Clear all operator stake data.
    fn clean_operators(&mut self);

    /// all operators
    fn operators(&self) -> HashMap<Address, TokenTracker>;
}

/// Trait for managing lock tokens.
pub trait TokenLockManagement {
    /// Set (or update) the amount of tokens to lock.
    fn set_lock_token(&mut self, token: Address, amount: U256);

    /// Remove the lock for a given token.
    fn remove_lock_token(&mut self, token: Address);

    fn tokens_to_lock(&self) -> TokenTracker;
}

/// Trait for managing vault snapshots.
pub trait VaultSnapshotManagement {
    /// Store a vault snapshot at the given capture timestamp and index.
    fn store_vault_snapshot(
        &mut self,
        captured_timestamp: U256,
        index: U256,
        snapshot: VaultSnapshot,
    );

    /// Retrieve a vault snapshot by capture timestamp and index.
    fn get_vault_snapshot(&self, captured_timestamp: U256, index: U256) -> Option<&VaultSnapshot>;

    /// Retrieve all vault snapshots for a given capture timestamp.
    fn get_all_vault_snapshots(&self, captured_timestamp: U256) -> Vec<VaultSnapshot>;
    fn vault_snapshots(&self) -> HashMap<U256, HashMap<U256, VaultSnapshot>>;
    fn vault_snapshot_indexes(&self) -> Vec<U256>;
}

/// Trait for managing slash results.
pub trait SlashResultManagement {
    /// Store a slash result at the given capture timestamp and index.
    fn store_slash_result(&mut self, captured_timestamp: U256, index: U256, result: SlashResult);

    /// Retrieve a slash result by capture timestamp and index.
    fn get_slash_result(&self, captured_timestamp: U256, index: U256) -> Option<&SlashResult>;

    /// Retrieve all slash results for a given capture timestamp.
    fn get_all_slash_results(&self, captured_timestamp: U256) -> Vec<SlashResult>;

    fn slash_result(&self) -> HashMap<U256, HashMap<U256, SlashResult>>;
    fn slash_result_indexes(&self) -> Vec<U256>;
}
