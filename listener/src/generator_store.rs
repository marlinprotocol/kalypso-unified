use ecies::{PublicKey, SecretKey};
use ethers::{types::Address, types::U256};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Generator {
    pub address: Address,
    pub supported_market_ids: Vec<U256>,
    pub ecies_priv_key: SecretKey,
    pub ecies_pub_key: PublicKey,
}

pub struct GeneratorStore {
    store: HashMap<Address, Generator>,
}

impl GeneratorStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn add_generator(&mut self, generator: Generator) {
        self.store.insert(generator.address, generator);
    }

    #[allow(dead_code)]
    fn remove_generator(&mut self, address: &Address) -> Option<Generator> {
        self.store.remove(address)
    }

    pub fn get_generator(&self, address: &Address) -> Option<&Generator> {
        self.store.get(address)
    }

    pub fn count(&self) -> usize {
        self.store.len()
    }

    #[allow(dead_code)]
    pub fn add_supported_market(
        &mut self,
        address: &Address,
        market_id: U256,
    ) -> Result<(), String> {
        match self.store.get_mut(address) {
            Some(generator) => {
                generator.supported_market_ids.push(market_id);
                Ok(())
            }
            None => Err(format!("No generator found with address: {:?}", address)),
        }
    }

    #[allow(dead_code)]
    pub fn remove_supported_market(
        &mut self,
        address: &Address,
        market_id: U256,
    ) -> Result<(), String> {
        match self.store.get_mut(address) {
            Some(generator) => {
                generator.supported_market_ids.retain(|&id| id != market_id);
                Ok(())
            }
            None => Err(format!("No generator found with address: {:?}", address)),
        }
    }
}
