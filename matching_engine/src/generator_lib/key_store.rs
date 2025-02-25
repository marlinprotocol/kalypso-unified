use ethers::core::types::Address;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone)]
pub struct Key {
    pub address: Address,
    pub key_index: u64,
    pub ecies_pub_key: Option<Bytes>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct KeyInfo {
    pub address: String,
    key_index: String,
    ecies_pub_key: Option<String>,
}

impl Key {
    pub fn to_key_info(&self) -> KeyInfo {
        let ecies_pub_key = {
            if self.ecies_pub_key.is_none() {
                None
            } else {
                Some(hex::encode(&self.ecies_pub_key().unwrap().to_vec()))
            }
        };
        KeyInfo {
            address: format!("{:?}", self.address),
            key_index: self.key_index.to_string(),
            ecies_pub_key,
        }
    }
}

impl Key {
    pub fn ecies_pub_key(&self) -> Option<Bytes> {
        self.ecies_pub_key.clone()
    }

    pub fn new(address: Address, key_index: u64, ecies_pub_key: Option<Bytes>) -> Self {
        Self {
            address,
            key_index,
            ecies_pub_key,
        }
    }
}

/// A trait that defines all operations for a KeyStore.
pub trait KeyStoreOperations {
    /// Inserts a key for the given address and value.
    fn insert(&mut self, address: Address, value: u64, key: Key);

    /// Retrieves a key by address and value.
    fn get_by_address(&self, address: &Address, value: u64) -> Option<Key>;

    /// Removes a key by address and value.
    fn remove_by_address(&mut self, address: &Address, value: u64);

    /// Updates the public key for the key associated with the given address and value.
    fn update_pub_key(&mut self, address: &Address, value: u64, new_pub_key: Option<Bytes>);
}
