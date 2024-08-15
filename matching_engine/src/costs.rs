use ethers::types::U256;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CostStore {
    data: HashMap<u8, U256>,
}

impl CostStore {
    // Create a new CostStore
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    // Upsert function: insert or update a key-value pair
    pub fn upsert(&mut self, key: u8, value: U256) {
        self.data.insert(key, value);
    }

    // Remove function: remove a key-value pair by key
    pub fn remove(&mut self, key: u8) -> Option<U256> {
        self.data.remove(&key)
    }

    // Get function: retrieve a value by key
    pub fn get(&self, key: u8) -> Option<&U256> {
        self.data.get(&key)
    }
}
