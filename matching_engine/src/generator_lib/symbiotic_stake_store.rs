use ethers::types::Address;
use im::HashMap;

use crate::utility::TokenTracker;

#[derive(Debug, Clone)]
pub struct SymbioticStakeStore {
    operators: HashMap<Address, TokenTracker>,
}

impl SymbioticStakeStore {
    pub fn new() -> Self {
        Self {
            operators: HashMap::new(),
        }
    }
}
