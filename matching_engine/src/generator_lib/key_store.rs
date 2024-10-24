use ethers::core::types::Address;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone)]
pub struct Key {
    pub address: Address,
    key_index: u64,
    ecies_pub_key: Option<Bytes>,
}

impl Key {
    pub fn ecies_pub_key(&self) -> Option<Bytes> {
        self.ecies_pub_key.clone()
    }
}

impl Key {
    pub fn new(address: Address, key_index: u64, ecies_pub_key: Option<Bytes>) -> Self {
        Self {
            address,
            key_index,
            ecies_pub_key,
        }
    }
}

#[derive(Debug)]
pub struct KeyStore {
    keys: HashMap<(Address, u64), Key>, // Using u64 as a stand-in for uint256.
}

impl KeyStore {
    pub fn new() -> Self {
        KeyStore {
            keys: HashMap::new(),
        }
    }

    // Assuming you now need to pass the u64 value along with the Key
    pub fn insert(&mut self, address: Address, value: u64, key: Key) {
        self.keys.insert((address, value), key);
    }

    // Updated to reflect the tuple key
    pub fn get_by_address(&self, address: &Address, value: u64) -> Option<Key> {
        self.keys.get(&(*address, value)).cloned()
    }

    // Updated to reflect the tuple key
    pub fn remove_by_address(&mut self, address: &Address, value: u64) {
        self.keys.remove(&(*address, value));
    }

    // Updated to reflect the tuple key
    pub fn update_pub_key(&mut self, address: &Address, value: u64, new_pub_key: Option<Bytes>) {
        if let Some(key) = self.keys.get_mut(&(*address, value)) {
            key.ecies_pub_key = new_pub_key;
        }
    }
}
