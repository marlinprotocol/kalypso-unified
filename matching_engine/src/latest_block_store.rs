use ethers::types::U64;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LatestBlockStore {
    block: U64,
}

impl LatestBlockStore {
    pub fn new(latest_block: U64) -> Self {
        Self {
            block: latest_block,
        }
    }

    pub fn new_from_dec_string(latest_block: String) -> anyhow::Result<Self> {
        let latest_block = U64::from_dec_str(&latest_block)?;
        Ok(Self {
            block: latest_block,
        })
    }
}

impl LatestBlockStoreTrait for LatestBlockStore {
    fn get_latest_block(&self) -> U64 {
        self.block
    }

    fn set_latest_block(&mut self, block: U64) {
        self.block = block;
    }
}

pub trait LatestBlockStoreTrait {
    fn get_latest_block(&self) -> U64;
    fn set_latest_block(&mut self, block: U64);
}
