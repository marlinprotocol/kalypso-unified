use ethers::core::types::Address;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::MutexGuard;

use std::collections::HashMap;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::generator_query::GeneratorQueryResult;
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

// create, update the generator store
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

    fn get_all_markets_generator(&self, address: &Address) -> Vec<&GeneratorInfoPerMarket> {
        match self.address_index.get(address) {
            Some(market_ids) => market_ids
                .iter()
                .filter_map(|m_id| self.generator_markets.get(&(*address, *m_id)))
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn pause_assignments_across_all_markets(&mut self, address: &Address) {
        // Collect the market IDs to be updated first, avoiding an immutable borrow later
        let all_market_ids: Vec<U256> = self
            .get_all_markets_generator(address)
            .iter()
            .map(|single_market| single_market.market_id) // Collect U256 (market_id)
            .collect();

        // Now process them with mutable access
        for market_id in all_market_ids {
            self.update_state(address, &market_id, GeneratorState::PendingConfirmation);
        }
    }

    pub fn resume_assignments_accross_all_markets(&mut self, address: &Address) {
        // Collect the market IDs to be updated first, avoiding an immutable borrow later
        let all_market_ids: Vec<U256> = self
            .get_all_markets_generator(address)
            .iter()
            .map(|single_market| single_market.market_id) // Collect U256 (market_id)
            .collect();

        // Now process them with mutable access
        for market_id in all_market_ids {
            self.update_state(address, &market_id, GeneratorState::Joined);
        }
    }

    pub fn get_by_address(&self, address: &Address) -> Option<&Generator> {
        self.generators.get(address)
    }
}

impl GeneratorStore {
    pub fn get_available_compute(&self, address: Address) -> Option<U256> {
        self.generators
            .get(&address)
            .map(|generator| generator.declared_compute.sub(generator.compute_consumed))
    }

    pub fn get_available_stake(&self, address: Address) -> Option<U256> {
        self.generators
            .get(&address)
            .map(|generator| generator.total_stake.sub(generator.stake_locked))
    }

    pub fn get_all_by_market_id(&self, market_id: &U256) -> Option<Vec<GeneratorInfoPerMarket>> {
        let all_generators = self.clone().all_generators_address();
        let mut generator_for_given_market = vec![];
        for generator in all_generators {
            let generator_info_per_market = self.get_by_address_and_market(&generator, &market_id);
            if generator_info_per_market.is_some() {
                generator_for_given_market.push(generator_info_per_market.unwrap().clone());
            }
        }

        Some(generator_for_given_market)
    }
}

// add methods to generate the query
impl GeneratorStore {
    #[allow(unused)]
    pub fn query(&self) -> GeneratorQueryResult {
        GeneratorQueryResult::new(self.generator_markets.values().collect())
    }

    pub fn query_by_states(&self, states: Vec<GeneratorState>) -> GeneratorQueryResult {
        log::debug!("Check query by states");

        let mut generators_market = Vec::new();

        // Iterate over each state in the slice
        for state in states {
            if let Some(pairs) = self.state_index.get(&state) {
                for &(address, market_id) in pairs {
                    if let Some(generator_market) =
                        self.generator_markets.get(&(address, market_id))
                    {
                        // Clone the generator market so that we have an owned value
                        generators_market.push(generator_market);
                    }
                }
            }
        }

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
}

// more complex queries, but not a good way to do
impl GeneratorStore {
    pub fn filter_by_has_idle_compute(
        &self,
        generator_query: GeneratorQueryResult,
    ) -> GeneratorQueryResult {
        let generator_array = generator_query.result();
        let mut generator_result = vec![];
        for elem in generator_array {
            if let Some(generator) = self.generators.get(&elem.address) {
                let idle_compute = generator.declared_compute.sub(generator.compute_consumed);
                if idle_compute.ge(&elem.compute_required_per_request) {
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
                if remaining_stake.ge(&min_stake) {
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

#[cfg(test)]
mod tests {
    use crate::generator_lib::generator_helper::idle_generator_selector;

    use super::{Generator, GeneratorInfoPerMarket, GeneratorState, GeneratorStore};
    use ethers::{
        core::rand::{self, seq::SliceRandom},
        types::{Address, H160, U256},
    };

    #[test]
    fn test_insert_remove_generators() {
        let mut generator_store = GeneratorStore::new();

        let generator1 = Generator {
            address: Address::random(),
            reward_address: Address::random(),
            total_stake: U256::from_dec_str("123123").unwrap(),
            sum_of_compute_allocations: U256::from_dec_str("12312312").unwrap(),
            compute_consumed: U256::from_dec_str("12312").unwrap(),
            stake_locked: U256::from_dec_str("12312").unwrap(),
            active_market_places: U256::from_dec_str("12").unwrap(),
            declared_compute: U256::from_dec_str("123123").unwrap(),
            intended_stake_util: U256::from_dec_str("123123").unwrap(),
            intended_compute_util: U256::from_dec_str("123123").unwrap(),
            generator_data: None,
        };

        generator_store.insert(generator1.clone());

        let generator = generator_store.get_by_address(&generator1.clone().address);
        assert!(generator.is_some());
        assert_eq!(generator.unwrap().reward_address, generator1.reward_address);
    }

    #[test]
    fn test_stake_and_compute() {
        let mut generator_store = create_new_store_with_generators(2);
        let random_generator = get_random_generator(&generator_store);

        // Perform your stake and compute operations on `generator`
        assert_eq!(
            random_generator.total_stake,
            U256::from_dec_str("100").unwrap()
        );
        assert_eq!(
            random_generator.sum_of_compute_allocations,
            U256::from_dec_str("100").unwrap()
        );

        generator_store
            .add_extra_compute(&random_generator.address, U256::from_dec_str("1").unwrap());

        assert_eq!(
            generator_store
                .get_by_address(&random_generator.address)
                .unwrap()
                .declared_compute,
            U256::from_dec_str("101").unwrap()
        );

        generator_store.remove_compute(&random_generator.address, U256::from_dec_str("2").unwrap());

        assert_eq!(
            generator_store
                .get_by_address(&random_generator.address)
                .unwrap()
                .declared_compute,
            U256::from_dec_str("99").unwrap()
        );

        generator_store
            .add_extra_stake(&random_generator.address, &U256::from_dec_str("5").unwrap());

        assert_eq!(
            generator_store
                .get_by_address(&random_generator.address)
                .unwrap()
                .total_stake,
            U256::from_dec_str("105").unwrap()
        );

        generator_store.remove_stake(
            &random_generator.address,
            &U256::from_dec_str("15").unwrap(),
        );

        assert_eq!(
            generator_store
                .get_by_address(&random_generator.address)
                .unwrap()
                .total_stake,
            U256::from_dec_str("90").unwrap()
        );
    }

    #[test]
    fn test_markets() {
        let mut generator_store = create_new_store_with_generators(4);
        let random_generator = get_random_generator(&generator_store);

        let random_generator_info_per_market = get_random_market_info_for_generator(
            &random_generator.address,
            random_generator.total_stake,
            "1".into(),
        );
        generator_store.insert_markets(random_generator_info_per_market);

        let generator_info_per_market = generator_store.get_by_address_and_market(
            &random_generator.address,
            &U256::from_dec_str("1").unwrap(),
        );

        assert!(generator_info_per_market.is_some());
        assert_eq!(
            generator_info_per_market.unwrap().address,
            random_generator.address
        );

        assert_eq!(
            generator_store
                .get_all_markets_generator(&random_generator.address)
                .len(),
            1
        );

        generator_store.remove_by_address_and_market(
            &random_generator.address,
            &U256::from_dec_str("1").unwrap(),
        );

        let generator_info_per_market = generator_store.get_by_address_and_market(
            &random_generator.address,
            &U256::from_dec_str("1").unwrap(),
        );

        assert!(generator_info_per_market.is_none());
    }

    #[test]
    fn test_matches() {
        let mut generator_store = create_new_store_with_generators(4);

        let all_generators = { generator_store.clone().all_generators_address() };
        for generator in all_generators {
            let generator = generator_store.get_by_address(&generator).unwrap();
            let random_generator_info_per_market = get_random_market_info_for_generator(
                &generator.address,
                generator.total_stake,
                "1".into(),
            );
            generator_store.insert_markets(random_generator_info_per_market);
        }

        let all_generator_per_market_query =
            generator_store.query_by_states(vec![GeneratorState::Joined]);

        assert_eq!(all_generator_per_market_query.clone().result().len(), 4);

        let idle_generators: Vec<GeneratorInfoPerMarket> = idle_generator_selector(
            all_generator_per_market_query
                .clone()
                .filter_by_market_id(U256::from_dec_str("1").unwrap())
                .result(),
        );

        assert_eq!(idle_generators.len(), 4);

        for idle_generator in idle_generators {
            generator_store.update_on_compute_locked(
                &idle_generator.address,
                U256::from_dec_str("100").unwrap(),
            );
            let available_compute = generator_store
                .get_available_compute(idle_generator.address)
                .unwrap();
            assert_eq!(available_compute, U256::zero());
        }

        let all_generator_per_market_query = generator_store.filter_by_has_idle_compute(
            generator_store.query_by_states(vec![GeneratorState::Joined]),
        );

        assert_eq!(all_generator_per_market_query.clone().result().len(), 0);
    }

    #[test]
    fn test_matches_2() {
        let mut generator_store = create_new_store_with_generators(4);

        let all_generators = { generator_store.clone().all_generators_address() };
        for generator in all_generators {
            let generator = generator_store.get_by_address(&generator).unwrap();
            let random_generator_info_per_market = get_random_market_info_for_generator(
                &generator.address,
                generator.total_stake,
                "1".into(),
            );
            generator_store.insert_markets(random_generator_info_per_market);
        }

        let all_generator_per_market_query =
            generator_store.query_by_states(vec![GeneratorState::Joined]);

        assert_eq!(all_generator_per_market_query.clone().result().len(), 4);

        let idle_generators: Vec<GeneratorInfoPerMarket> = idle_generator_selector(
            all_generator_per_market_query
                .clone()
                .filter_by_market_id(U256::from_dec_str("1").unwrap())
                .result(),
        );

        assert_eq!(idle_generators.len(), 4);

        generator_store.update_on_compute_locked(
            &idle_generators[0].address,
            U256::from_dec_str("100").unwrap(),
        );
        let available_compute = generator_store
            .get_available_compute(idle_generators[0].address)
            .unwrap();
        assert_eq!(available_compute, U256::zero());

        let all_generator_per_market_query = generator_store.filter_by_has_idle_compute(
            generator_store.query_by_states(vec![GeneratorState::Joined]),
        );

        assert_eq!(all_generator_per_market_query.clone().result().len(), 3);
    }

    #[test]
    fn test_matches_3() {
        let mut generator_store = create_new_store_with_generators(4);

        let all_generators = { generator_store.clone().all_generators_address() };
        for generator in all_generators {
            let generator = generator_store.get_by_address(&generator).unwrap();
            let random_generator_info_per_market = get_random_market_info_for_generator(
                &generator.address,
                generator.total_stake,
                "1".into(),
            );
            generator_store.insert_markets(random_generator_info_per_market);
        }

        let all_generator_per_market_query =
            generator_store.query_by_states(vec![GeneratorState::Joined]);

        assert_eq!(all_generator_per_market_query.clone().result().len(), 4);

        let idle_generators: Vec<GeneratorInfoPerMarket> = idle_generator_selector(
            all_generator_per_market_query
                .clone()
                .filter_by_market_id(U256::from_dec_str("1").unwrap())
                .result(),
        );

        assert_eq!(idle_generators.len(), 4);

        generator_store.update_on_compute_locked(
            &idle_generators[0].address,
            U256::from_dec_str("99").unwrap(),
        );
        let available_compute = generator_store
            .get_available_compute(idle_generators[0].address)
            .unwrap();
        assert_eq!(available_compute, U256::one());

        let all_generator_per_market_query = generator_store.filter_by_has_idle_compute(
            generator_store.query_by_states(vec![GeneratorState::Joined]),
        );

        let idle_generators = all_generator_per_market_query.clone().result();
        dbg!(&idle_generators);
        assert_eq!(idle_generators.len(), 4);
    }

    fn get_random_market_info_for_generator(
        generator: &H160,
        total_stake: U256,
        market_id: String,
    ) -> GeneratorInfoPerMarket {
        GeneratorInfoPerMarket {
            address: *generator,
            market_id: U256::from_dec_str(&market_id).unwrap(),
            total_stake,
            compute_required_per_request: U256::from_dec_str("1").unwrap(),
            proof_generation_cost: U256::from_dec_str("5").unwrap(),
            proposed_time: U256::from_dec_str("10").unwrap(),
            active_requests: U256::from_dec_str("0").unwrap(),
            proofs_submitted: U256::from_dec_str("0").unwrap(),
            state: Some(GeneratorState::Joined),
        }
    }

    fn get_random_generator(generator_store: &GeneratorStore) -> Generator {
        let all_generator_addresses = generator_store.clone().all_generators_address();
        let random_generator = all_generator_addresses
            .choose(&mut rand::thread_rng())
            .unwrap();
        generator_store
            .get_by_address(random_generator)
            .unwrap()
            .clone()
    }

    // helpers in tests.
    fn create_new_store_with_generators(n: usize) -> GeneratorStore {
        let mut generator_store = GeneratorStore::new();

        for _ in 0..n {
            let generator = Generator {
                address: Address::random(),
                reward_address: Address::random(),
                total_stake: U256::from_dec_str("100").unwrap(),
                sum_of_compute_allocations: U256::from_dec_str("100").unwrap(),
                compute_consumed: U256::from_dec_str("0").unwrap(),
                stake_locked: U256::from_dec_str("0").unwrap(),
                active_market_places: U256::from_dec_str("0").unwrap(),
                declared_compute: U256::from_dec_str("100").unwrap(),
                intended_stake_util: U256::from_dec_str("1000000000000000000").unwrap(),
                intended_compute_util: U256::from_dec_str("1000000000000000000").unwrap(),
                generator_data: None,
            };

            generator_store.insert(generator);
        }

        generator_store
    }
}
