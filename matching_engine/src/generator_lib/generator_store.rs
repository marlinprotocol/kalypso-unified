use ethers::core::types::Address;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::MutexGuard;

use std::collections::HashMap;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::generator_state::GeneratorState;
use super::key_store::KeyStore;

#[derive(Debug, Clone)]
pub struct GeneratorStore {
    // Change key to tuple (Address, U256)
    generators: HashMap<Address, Generator>,
    generator_markets: HashMap<(Address, U256), GeneratorInfoPerMarket>,
    state_index: HashMap<GeneratorState, Vec<(Address, U256)>>,
    address_index: HashMap<Address, Vec<U256>>, // to easily fetch all generators by address
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone)]
pub struct GeneratorInfoPerMarket {
    pub address: Address,
    pub market_id: U256,
    pub total_stake: U256,
    pub compute_required_per_request: U256,
    pub proof_generation_cost: U256,
    pub proposed_time: U256,
    pub active_requests: U256,
    pub proofs_submitted: U256,
    pub state: Option<GeneratorState>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone)]
pub struct Generator {
    pub address: Address,
    pub reward_address: Address,
    pub total_stake: U256,
    pub sum_of_compute_allocations: U256,
    pub compute_consumed: U256,
    pub stake_locked: U256,
    pub active_market_places: U256,
    pub declared_compute: U256,
    pub intended_stake_util: U256,
    pub intended_compute_util: U256,
    pub generator_data: Option<Bytes>,
}

impl Default for GeneratorStore {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratorStore {
    pub fn new() -> Self {
        GeneratorStore {
            generators: HashMap::new(),
            generator_markets: HashMap::new(),
            state_index: HashMap::new(),
            address_index: HashMap::new(),
        }
    }

    pub fn all_generators_address(self) -> Vec<Address> {
        self.generators.keys().cloned().collect()
    }

    pub fn insert(&mut self, generator: Generator) {
        let address = generator.address;
        if !self.generators.contains_key(&generator.address) {
            self.generators.insert(address, generator);
        }
    }

    pub fn insert_markets(&mut self, generator_market: GeneratorInfoPerMarket) {
        let address = generator_market.address;
        let market_id = generator_market.market_id;
        let compute_allocation = generator_market.compute_required_per_request;

        if let Some(generator) = self.generators.get_mut(&address) {
            generator.active_market_places.add_assign(U256::one());
            generator
                .sum_of_compute_allocations
                .add_assign(compute_allocation);
        }

        if let Some(state) = &generator_market.state {
            self.state_index
                .entry(*state)
                .or_default()
                .push((address, market_id));
        }

        self.address_index
            .entry(address)
            .or_default()
            .push(market_id);

        self.generator_markets
            .insert((address, market_id), generator_market);
    }

    pub fn get_by_address_and_market(
        &self,
        address: &Address,
        market_id: &U256,
    ) -> Option<&GeneratorInfoPerMarket> {
        self.generator_markets.get(&(*address, *market_id))
    }

    pub fn remove_by_address_and_market(&mut self, address: &Address, market_id: &U256) {
        if let Some(generator_market) = self.generator_markets.remove(&(*address, *market_id)) {
            let compute_allocation = generator_market.compute_required_per_request;
            if let Some(state) = &generator_market.state {
                if let Some(vec) = self.state_index.get_mut(state) {
                    vec.retain(|&(a, m)| a != *address || m != *market_id);
                }
            }

            if let Some(generator) = self.generators.get_mut(address) {
                generator
                    .sum_of_compute_allocations
                    .sub_assign(compute_allocation);
                generator.active_market_places.sub_assign(U256::one());
            }
        }
    }

    pub fn remove_by_address(&mut self, address: &Address) {
        self.generators.remove(address);
    }

    pub fn add_extra_stake(&mut self, address: &Address, amount: &U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.total_stake = generator.total_stake.add(amount);

            if let Some(markets) = self.address_index.get(address) {
                for elem in markets {
                    if let Some(generator_market) =
                        self.generator_markets.get_mut(&(*address, *elem))
                    {
                        generator_market.total_stake = generator.total_stake;
                    }
                }
            }
        }
    }

    pub fn update_intended_stake_util(&mut self, address: &Address, new_stake_util: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.intended_stake_util = new_stake_util;
        }
    }

    pub fn remove_stake(&mut self, address: &Address, amount: &U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.total_stake = generator.total_stake.sub(amount);

            if let Some(markets) = self.address_index.get(address) {
                for elem in markets {
                    if let Some(generator_market) =
                        self.generator_markets.get_mut(&(*address, *elem))
                    {
                        generator_market.total_stake = generator.total_stake;
                    }
                }
            }
        }
    }

    pub fn update_reward_address(&mut self, address: &Address, new_reward_address: Address) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.reward_address = new_reward_address;
        }
    }

    pub fn add_extra_compute(&mut self, address: &Address, compute: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.declared_compute = generator.declared_compute.add(compute);
        }
    }

    pub fn update_intended_compute_util(&mut self, address: &Address, new_compute_util: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.intended_compute_util = new_compute_util;
        }
    }

    pub fn remove_compute(&mut self, address: &Address, compute: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.declared_compute = generator.declared_compute.sub(compute);
        }
    }

    pub fn update_state(&mut self, address: &Address, market_id: &U256, new_state: GeneratorState) {
        if let Some(generator_market) = self.generator_markets.get_mut(&(*address, *market_id)) {
            if let Some(old_state) = &generator_market.state {
                if let Some(vec) = self.state_index.get_mut(old_state) {
                    vec.retain(|&a| a != (*address, *market_id));
                }
            }

            generator_market.state = Some(new_state);
            self.state_index
                .entry(new_state)
                .or_default()
                .push((*address, *market_id));
        }
    }

    pub fn update_on_assigned_task(&mut self, address: &Address, market_id: &U256) {
        if let Some(generator_market) = self.generator_markets.get_mut(&(*address, *market_id)) {
            generator_market.active_requests.add_assign(U256::one());
        }
    }

    pub fn update_on_submit_proof(&mut self, address: &Address, market_id: &U256) {
        if let Some(generator_market) = self.generator_markets.get_mut(&(*address, *market_id)) {
            generator_market.active_requests.sub_assign(U256::one());
            generator_market.proofs_submitted.add_assign(U256::one());
        }
    }

    pub fn update_on_slashing(&mut self, address: &Address, market_id: &U256) {
        if let Some(generator_market) = self.generator_markets.get_mut(&(*address, *market_id)) {
            generator_market.active_requests.sub_assign(U256::one());
        }
    }

    pub fn update_on_stake_locked(&mut self, address: &Address, stake_locked: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.stake_locked.add_assign(stake_locked);
        }
    }

    pub fn update_on_stake_released(&mut self, address: &Address, stake_released: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.stake_locked.sub_assign(stake_released);
        }
    }

    pub fn update_on_compute_locked(&mut self, address: &Address, compute_locked: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.compute_consumed.add_assign(compute_locked);
        }
    }

    pub fn update_on_compute_released(&mut self, address: &Address, compute_released: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.compute_consumed.sub_assign(compute_released);
        }
    }

    #[allow(unused)]
    pub fn get_all_markets_generator(&self, address: &Address) -> Vec<&GeneratorInfoPerMarket> {
        match self.address_index.get(address) {
            Some(market_ids) => market_ids
                .iter()
                .filter_map(|m_id| self.generator_markets.get(&(*address, *m_id)))
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn get_by_address(&self, address: &Address) -> Option<&Generator> {
        self.generators.get(address)
    }
}

#[derive(Clone)]
pub struct GeneratorQueryResult<'a> {
    generator_markets: Vec<&'a GeneratorInfoPerMarket>,
}

impl<'a> GeneratorQueryResult<'a> {
    // Initialize with a collection of generators
    pub fn new(generator_markets: Vec<&'a GeneratorInfoPerMarket>) -> Self {
        Self { generator_markets }
    }

    // Filter by reward
    pub fn filter_by_reward(mut self, task_reward: U256) -> Self {
        log::debug!("Filter by reward");
        self.generator_markets
            .retain(|&gen| gen.proof_generation_cost.lt(&task_reward));
        self
    }

    // Filter by market ID
    pub fn filter_by_market_id(mut self, market_id: U256) -> Self {
        log::debug!("Filter by market id");
        self.generator_markets
            .retain(|&gen| gen.market_id == market_id);
        self
    }

    // Filter by state
    #[allow(unused)]
    pub fn filter_by_state(mut self, state: GeneratorState) -> Self {
        self.generator_markets
            .retain(|&gen| gen.state == Some(state));
        self
    }

    // Final getter to consume the object and retrieve the filtered generators
    pub fn result(self) -> Vec<&'a GeneratorInfoPerMarket> {
        self.generator_markets
    }
}

// Adding query methods to the `GeneratorStore`
impl GeneratorStore {
    #[allow(unused)]
    pub fn query(&self) -> GeneratorQueryResult {
        GeneratorQueryResult::new(self.generator_markets.values().collect())
    }

    pub fn query_by_state(&self, state: GeneratorState) -> GeneratorQueryResult {
        log::debug!("Check query by state");
        let generators_market = match self.state_index.get(&state) {
            Some(pairs) => pairs
                .iter()
                .filter_map(|&(address, market_id)| {
                    self.generator_markets.get(&(address, market_id))
                })
                .collect(),
            None => Vec::new(),
        };
        GeneratorQueryResult::new(generators_market)
    }

    #[allow(unused)]
    pub fn query_by_address(&self, address: Address) -> GeneratorQueryResult {
        let generators = match self.address_index.get(&address) {
            Some(market_ids) => market_ids
                .iter()
                .filter_map(|m_id| self.generator_markets.get(&(address, *m_id)))
                .collect(),
            None => Vec::new(),
        };
        GeneratorQueryResult::new(generators)
    }

    pub fn filter_by_has_idle_compute(
        &self,
        generator_query: GeneratorQueryResult,
    ) -> GeneratorQueryResult {
        let generator_array = generator_query.result();
        let mut generator_result = vec![];
        for elem in generator_array {
            if let Some(generator) = self.generators.get(&elem.address) {
                let idle_compute = generator.declared_compute.sub(generator.compute_consumed);
                let utilization = generator.intended_compute_util;
                let exponent: U256 = 1000000000000000000_i64.into();
                if utilization >= exponent && idle_compute.gt(&elem.compute_required_per_request) {
                    generator_result.push(
                        self.generator_markets
                            .get(&(elem.address, elem.market_id))
                            .unwrap(),
                    );
                }
            }
        }

        GeneratorQueryResult::new(generator_result)
    }

    pub fn filter_by_available_stake(
        &self,
        generator_query: GeneratorQueryResult,
        min_stake: U256,
    ) -> GeneratorQueryResult {
        let generator_array = generator_query.result();
        let mut generator_result = vec![];
        for elem in generator_array {
            if let Some(generator) = self.generators.get(&elem.address) {
                let remaining_stake = generator.total_stake.sub(generator.stake_locked);
                let utilization = generator.intended_stake_util;
                let exponent: U256 = 1000000000000000000_i64.into();
                if utilization >= exponent && remaining_stake.gt(&min_stake) {
                    generator_result.push(
                        self.generator_markets
                            .get(&(elem.address, elem.market_id))
                            .unwrap(),
                    );
                }
            }
        }
        GeneratorQueryResult::new(generator_result)
    }

    pub fn filter_by_has_private_inputs_support(
        &self,
        generator_query: GeneratorQueryResult,
        key_store: MutexGuard<'_, KeyStore>,
    ) -> GeneratorQueryResult {
        let generator_array = generator_query.result();
        let mut generator_result = vec![];
        for elem in generator_array {
            if let Some(generator) = self.generators.get(&elem.address) {
                let ecies_pub_key =
                    key_store.get_by_address(&generator.address, elem.market_id.as_u64());
                if ecies_pub_key.is_some() {
                    generator_result.push(
                        self.generator_markets
                            .get(&(elem.address, elem.market_id))
                            .unwrap(),
                    );
                }
            }
        }
        GeneratorQueryResult::new(generator_result)
    }
}
