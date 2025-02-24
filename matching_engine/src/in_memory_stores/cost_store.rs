use ethers::types::U256;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::costs::CostStoreOperations;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct CostStore {
    data: HashMap<u8, U256>,
}

impl CostStoreOperations for CostStore {
    // Create a new CostStore
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    // Upsert function: insert or update a key-value pair
    fn upsert(&mut self, key: u8, value: U256) {
        self.data.insert(key, value);
    }

    // Remove function: remove a key-value pair by key
    fn remove(&mut self, key: u8) -> Option<U256> {
        self.data.remove(&key)
    }

    // Get function: retrieve a value by key
    fn get(&self, key: u8) -> Option<U256> {
        self.data.get(&key).cloned()
    }
}
