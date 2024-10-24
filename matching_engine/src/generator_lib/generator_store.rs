use ethers::core::types::Address;
use ethers::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLockReadGuard;

use std::collections::HashMap;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::utility::{AddressTokenPair, TokenTracker};

use super::generator_query::GeneratorQueryResult;
use super::generator_state::GeneratorState;
use super::key_store::KeyStore;

#[derive(Debug, Clone)]
pub struct GeneratorStore {
    // Change key to tuple (Address, U256)
    generators: HashMap<Address, Generator>, // Generator -> Details
    generator_markets: HashMap<(Address, U256), GeneratorInfoPerMarket>, //[Generator, MarketId] -> MarketWiseInfo
    state_index: HashMap<GeneratorState, Vec<(Address, U256)>>, // State -> [Generator, MarketId]
    address_index: HashMap<Address, Vec<U256>>, // Generator -> [MarketId] participations
    earnings: HashMap<Address, U256>,           // Generator -> TotalEarnings
    earnings_per_market: HashMap<Address, HashMap<U256, U256>>, // Generator -> Markets -> Earnings Per Market
    slashings: HashMap<Address, TokenTracker>,                  // Generator -> Total Slashings
    slashings_per_market: HashMap<Address, HashMap<U256, TokenTracker>>, // Generator -> Markets -> slashings per market
    slashing_records: HashMap<Address, Vec<SlashingRecord>>, // Generator -> Slashing Record
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct SlashingRecord {
    pub ask_id: U256,
    pub market_id: U256,
    pub slashing_tx: String,
    pub price_offered: U256,
    pub expected_time: U256,
    pub slashing_penalty: AddressTokenPair,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone)]
pub struct GeneratorInfoPerMarket {
    pub address: Address,
    pub market_id: U256,
    // pub total_stake: TokenTracker,
    pub compute_required_per_request: U256,
    pub proof_generation_cost: U256,
    pub proposed_time: U256,
    pub active_requests: U256,
    pub proofs_submitted: U256,
    pub proofs_slashed: U256,
    pub state: Option<GeneratorState>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone)]
pub struct Generator {
    pub address: Address,
    pub reward_address: Address,
    pub total_stake: TokenTracker,
    pub sum_of_compute_allocations: U256,
    pub compute_consumed: U256,
    pub stake_locked: TokenTracker,
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
            earnings: HashMap::new(),
            earnings_per_market: HashMap::new(),
            slashings: HashMap::new(),
            slashings_per_market: HashMap::new(),
            slashing_records: HashMap::new(),
        }
    }

    pub fn all_generators_address(&self) -> Vec<Address> {
        self.generators
            .par_iter()
            .map(|(address, _)| address.clone())
            .collect()
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
    ) -> Option<GeneratorInfoPerMarket> {
        self.generator_markets.get(&(*address, *market_id)).cloned()
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

    pub fn add_extra_stake(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        amount: &U256,
    ) {
        if let Some(generator) = self.generators.get_mut(generator_address) {
            generator.total_stake.add_token(token_address, amount);
        }
    }

    pub fn update_intended_stake_util(&mut self, address: &Address, new_stake_util: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.intended_stake_util = new_stake_util;
        }
    }

    pub fn remove_stake(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        amount: &U256,
    ) {
        if let Some(generator) = self.generators.get_mut(generator_address) {
            generator
                .total_stake
                .sub_token_saturating(token_address, amount);
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

    pub fn update_on_submit_proof(&mut self, address: &Address, market_id: &U256, earning: &U256) {
        if let Some(generator_market) = self.generator_markets.get_mut(&(*address, *market_id)) {
            generator_market.active_requests.sub_assign(U256::one());
            generator_market.proofs_submitted.add_assign(U256::one());
        }

        // Update the total earnings for the address
        self.earnings
            .entry(*address)
            .and_modify(|e| *e = e.saturating_add(*earning))
            .or_insert(*earning);

        // Update the earnings for the specific market
        self.earnings_per_market
            .entry(*address)
            .or_insert_with(HashMap::new)
            .entry(*market_id)
            .and_modify(|e| *e = e.saturating_add(*earning))
            .or_insert(*earning);
    }

    pub fn update_on_slashing(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        ask_id: &U256,
        market_id: &U256,
        slashing: &U256,
        slashing_tx: String,
        price_offered: &U256,
        deadline: &U256,
    ) {
        if let Some(generator_market) = self
            .generator_markets
            .get_mut(&(*generator_address, *market_id))
        {
            generator_market.active_requests.sub_assign(U256::one());
            generator_market.proofs_slashed.add_assign(U256::one());
        }

        self.slashings
            .entry(*generator_address)
            .and_modify(|tracker| tracker.add_token(token_address, slashing)) // Modify the existing entry
            .or_insert({
                let mut tracker = TokenTracker::new(); // Create a new TokenTracker if none exists
                tracker.add_token(token_address, slashing); // Add the slashing amount
                tracker
            });

        self.slashings_per_market
            .entry(*generator_address)
            .or_insert_with(HashMap::new) // Create the inner HashMap if it doesn't exist
            .entry(*market_id)
            .and_modify(|tracker| tracker.add_token(token_address, slashing)) // Modify the existing TokenTracker
            .or_insert({
                let mut tracker = TokenTracker::new(); // Create a new TokenTracker if none exists
                tracker.add_token(token_address, slashing); // Add the slashing amount
                tracker
            });

        self.slashing_records
            .entry(*generator_address)
            .or_insert_with(Vec::new)
            .push(SlashingRecord {
                ask_id: ask_id.clone(),
                market_id: market_id.clone(),
                slashing_tx,
                price_offered: price_offered.clone(),
                expected_time: deadline.clone(),
                slashing_penalty: (token_address.clone(), slashing.clone()),
            });
    }

    pub fn update_on_stake_locked(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_locked: U256,
    ) {
        if let Some(generator) = self.generators.get_mut(generator_address) {
            generator
                .stake_locked
                .add_token(token_address, &stake_locked);
        }
    }

    pub fn update_on_stake_released(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_released: U256,
    ) {
        if let Some(generator) = self.generators.get_mut(generator_address) {
            generator
                .stake_locked
                .sub_token_saturating(token_address, &stake_released);
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

    pub fn get_all_markets_of_generator(&self, address: &Address) -> Vec<GeneratorInfoPerMarket> {
        match self.address_index.get(address) {
            Some(market_ids) => market_ids
                .par_iter()
                .filter_map(|m_id| self.generator_markets.get(&(*address, *m_id)).cloned())
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn pause_assignments_across_all_markets(&mut self, address: &Address) {
        // Collect the market IDs to be updated first, avoiding an immutable borrow later
        let all_market_ids: Vec<U256> = self
            .get_all_markets_of_generator(address)
            .par_iter()
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
            .get_all_markets_of_generator(address)
            .par_iter()
            .map(|single_market| single_market.market_id) // Collect U256 (market_id)
            .collect();

        // Now process them with mutable access
        for market_id in all_market_ids {
            self.update_state(address, &market_id, GeneratorState::Joined);
        }
    }

    pub fn get_by_address(&self, address: &Address) -> Option<Generator> {
        self.generators.get(address).cloned()
    }
}

impl GeneratorStore {
    pub fn get_available_compute(&self, address: Address) -> Option<U256> {
        self.generators
            .get(&address)
            .map(|generator| generator.declared_compute.sub(generator.compute_consumed))
    }

    pub fn get_available_stake(&self, generator_address: Address) -> Option<TokenTracker> {
        self.generators.get(&generator_address).map(|generator| {
            generator
                .total_stake
                .clone()
                .sub(generator.stake_locked.clone())
        })
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

    pub fn query_by_market_id(&self, market_id: &U256) -> GeneratorQueryResult {
        log::debug!("Check query by market id");

        let generator_markets: Vec<&GeneratorInfoPerMarket> = self
            .generator_markets
            .par_iter() // Parallel iterator over the generator_markets HashMap
            .filter_map(|((_, gen_market_id), generator_info)| {
                // Check if the market_id matches
                if gen_market_id == market_id {
                    Some(generator_info)
                } else {
                    None
                }
            })
            .collect(); // Collect matching generator markets into a Vec

        GeneratorQueryResult::new(generator_markets)
    }

    #[allow(unused)]
    pub fn query_by_states(&self, states: Vec<GeneratorState>) -> GeneratorQueryResult {
        log::debug!("Check query by states");

        let generators_market: Vec<&GeneratorInfoPerMarket> = states
            .into_par_iter() // Convert the Vec<GeneratorState> to a parallel iterator
            .filter_map(|state| {
                // For each state, get the associated pairs
                self.state_index.get(&state)
            })
            .flat_map(|pairs| {
                // For each pair, iterate in parallel and get the generator markets
                pairs.into_par_iter().filter_map(|&(address, market_id)| {
                    self.generator_markets.get(&(address, market_id))
                })
            })
            .collect();

        GeneratorQueryResult::new(generators_market)
    }

    #[allow(unused)]
    pub fn query_by_address(&self, address: Address) -> GeneratorQueryResult {
        let generators = match self.address_index.get(&address) {
            Some(market_ids) => market_ids
                .par_iter()
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

        // Use rayon's parallel iterator to process in parallel
        let generator_result: Vec<&GeneratorInfoPerMarket> = generator_array
            .into_par_iter() // Convert to a parallel iterator
            .filter_map(|elem| {
                // Try to get the generator from the store
                if let Some(generator) = self.generators.get(&elem.address) {
                    let idle_compute = generator.declared_compute.sub(generator.compute_consumed);

                    // Check if the idle compute is greater than or equal to the required compute
                    if idle_compute.ge(&elem.compute_required_per_request) {
                        // If so, retrieve the generator market and return it
                        self.generator_markets.get(&(elem.address, elem.market_id))
                    } else {
                        None // Otherwise, filter it out
                    }
                } else {
                    None // If generator doesn't exist, filter it out
                }
            })
            .collect(); // Collect the results into a Vec

        GeneratorQueryResult::new(generator_result)
    }

    pub fn filter_by_available_stake(
        &self,
        generator_query: GeneratorQueryResult,
        min_stake: Vec<AddressTokenPair>, // Now accepting a vector of AddressTokenPairs
    ) -> GeneratorQueryResult {
        let generator_array = generator_query.result();

        // Use rayon's parallel iterator to process in parallel
        let generator_result: Vec<&GeneratorInfoPerMarket> = generator_array
            .into_par_iter() // Convert the array to a parallel iterator
            .filter_map(|elem| {
                // Try to get the generator from the store
                if let Some(generator) = self.generators.get(&elem.address) {
                    let remaining_stake = generator
                        .total_stake
                        .clone()
                        .sub(generator.stake_locked.clone());

                    // Check if at least one of the AddressTokenPairs in min_stake meets the condition
                    let is_valid = min_stake
                        .iter()
                        .any(|min_stake_pair| remaining_stake.has_more_than_or_eq(min_stake_pair));

                    // If valid, retrieve the generator market and return it
                    if is_valid {
                        self.generator_markets.get(&(elem.address, elem.market_id))
                    } else {
                        None // Otherwise, filter it out
                    }
                } else {
                    None // If generator doesn't exist, filter it out
                }
            })
            .collect(); // Collect the results into a Vec

        GeneratorQueryResult::new(generator_result)
    }

    pub fn filter_by_has_private_inputs_support(
        &self,
        generator_query: GeneratorQueryResult,
        key_store: RwLockReadGuard<'_, KeyStore>,
    ) -> GeneratorQueryResult {
        let generator_array = generator_query.result();

        // Use rayon's parallel iterator to process in parallel
        let generator_result: Vec<&GeneratorInfoPerMarket> = generator_array
            .into_par_iter() // Convert to a parallel iterator
            .filter_map(|elem| {
                // Try to get the generator from the store
                if let Some(generator) = self.generators.get(&elem.address) {
                    // Lock the key store and check for ECIES public key
                    let ecies_pub_key =
                        key_store.get_by_address(&generator.address, elem.market_id.as_u64());
                    if ecies_pub_key.is_some() {
                        // If the key exists, retrieve the generator market
                        return self.generator_markets.get(&(elem.address, elem.market_id));
                    }
                }
                None // If no generator or no ECIES public key, filter it out
            })
            .collect(); // Collect the results into a Vec

        GeneratorQueryResult::new(generator_result)
    }
}

impl GeneratorStore {
    // Get total earnings for a specific address
    pub fn get_total_earning(&self, address: &Address) -> Option<U256> {
        self.earnings.get(address).cloned()
    }

    // Get earnings for a specific address and market
    #[allow(unused)]
    pub fn get_earning_per_market(&self, address: &Address, market_id: &U256) -> Option<U256> {
        self.earnings_per_market
            .get(address)
            .and_then(|market_earnings| market_earnings.get(market_id).cloned())
    }

    pub fn get_total_slashing(&self, generator_address: &Address) -> Option<TokenTracker> {
        self.slashings.get(generator_address).cloned()
    }

    // Get earnings for a specific address and market
    #[allow(unused)]
    pub fn get_slashing_per_market(
        &self,
        address: &Address,
        market_id: &U256,
    ) -> Option<TokenTracker> {
        self.slashings_per_market
            .get(address)
            .and_then(|market_earnings| market_earnings.get(market_id).cloned())
    }

    pub fn get_slashing_records(&self, address: &Address) -> Vec<SlashingRecord> {
        let data = self.slashing_records.get(address);
        if data.is_none() {
            return vec![];
        } else {
            return data.unwrap().clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        generator_lib::generator_helper::select_idle_generators,
        utility::{TokenTracker, TEST_TOKEN_ADDRESS, TEST_TOKEN_ADDRESS_STRING},
    };

    use super::{Generator, GeneratorInfoPerMarket, GeneratorState, GeneratorStore};
    use ethers::{
        core::rand::{self, seq::SliceRandom},
        types::{Address, H160, U256},
    };

    use rand::Rng;
    use std::time::Instant;

    #[test]
    fn test_insert_remove_generators() {
        let mut generator_store = GeneratorStore::new();

        let generator1 = Generator {
            address: Address::random(),
            reward_address: Address::random(),
            total_stake: TokenTracker::from_address_string_and_dec_string(
                vec![TEST_TOKEN_ADDRESS_STRING.to_string()],
                vec!["123123".into()],
            )
            .unwrap(),
            sum_of_compute_allocations: U256::from_dec_str("12312312").unwrap(),
            compute_consumed: U256::from_dec_str("12312").unwrap(),
            stake_locked: TokenTracker::from_address_string_and_dec_string(
                vec![TEST_TOKEN_ADDRESS_STRING.to_string()],
                vec!["123123".into()],
            )
            .unwrap(),
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
        let mut generator_store = create_new_store_with_generators(2, None, None, None);
        let random_generator = get_random_generator(&generator_store);

        // Perform your stake and compute operations on `generator`
        assert_eq!(
            random_generator.total_stake,
            TokenTracker::from_address_string_and_dec_string(
                vec![TEST_TOKEN_ADDRESS_STRING.to_string()],
                vec!["100".into()],
            )
            .unwrap()
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

        generator_store.add_extra_stake(
            &random_generator.address,
            &TEST_TOKEN_ADDRESS,
            &U256::from_dec_str("5").unwrap(),
        );

        assert_eq!(
            generator_store
                .get_by_address(&random_generator.address)
                .unwrap()
                .total_stake,
            TokenTracker::from_address_string_and_dec_string(
                vec![TEST_TOKEN_ADDRESS_STRING.to_string()],
                vec!["105".into()],
            )
            .unwrap()
        );

        generator_store.remove_stake(
            &random_generator.address,
            &TEST_TOKEN_ADDRESS,
            &U256::from_dec_str("15").unwrap(),
        );

        assert_eq!(
            generator_store
                .get_by_address(&random_generator.address)
                .unwrap()
                .total_stake,
            TokenTracker::from_address_string_and_dec_string(
                vec![TEST_TOKEN_ADDRESS_STRING.to_string()],
                vec!["90".into()],
            )
            .unwrap()
        );
    }

    #[test]
    fn test_markets() {
        let mut generator_store = create_new_store_with_generators(4, None, None, None);
        let random_generator = get_random_generator(&generator_store);

        let random_generator_info_per_market =
            get_random_market_info_for_generator(&random_generator.address, "1".into());
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
                .get_all_markets_of_generator(&random_generator.address)
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
        let mut generator_store = create_new_store_with_generators(4, None, None, None);

        let all_generators = { generator_store.clone().all_generators_address() };
        for generator in all_generators {
            let generator = generator_store.get_by_address(&generator).unwrap();
            let random_generator_info_per_market =
                get_random_market_info_for_generator(&generator.address, "1".into());
            generator_store.insert_markets(random_generator_info_per_market);
        }

        let all_generator_per_market_query =
            generator_store.query_by_states(vec![GeneratorState::Joined]);

        assert_eq!(all_generator_per_market_query.clone().result().len(), 4);

        let idle_generators: Vec<GeneratorInfoPerMarket> = select_idle_generators(
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
        let mut generator_store = create_new_store_with_generators(4, None, None, None);

        let all_generators = { generator_store.clone().all_generators_address() };
        for generator in all_generators {
            let generator = generator_store.get_by_address(&generator).unwrap();
            let random_generator_info_per_market =
                get_random_market_info_for_generator(&generator.address, "1".into());
            generator_store.insert_markets(random_generator_info_per_market);
        }

        let all_generator_per_market_query =
            generator_store.query_by_states(vec![GeneratorState::Joined]);

        assert_eq!(all_generator_per_market_query.clone().result().len(), 4);

        let idle_generators: Vec<GeneratorInfoPerMarket> = select_idle_generators(
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
        let generator_count = 4;
        let mut generator_store =
            create_new_store_with_generators(generator_count, None, None, None);

        let all_generators = { generator_store.clone().all_generators_address() };
        for generator in all_generators {
            let generator = generator_store.get_by_address(&generator).unwrap();
            let random_generator_info_per_market =
                get_random_market_info_for_generator(&generator.address, "1".into());
            generator_store.insert_markets(random_generator_info_per_market);
        }

        let all_generator_per_market_query =
            generator_store.query_by_states(vec![GeneratorState::Joined]);

        assert_eq!(
            all_generator_per_market_query.clone().result().len(),
            generator_count
        );

        let idle_generators: Vec<GeneratorInfoPerMarket> = select_idle_generators(
            all_generator_per_market_query
                .clone()
                .filter_by_market_id(U256::from_dec_str("1").unwrap())
                .result(),
        );

        assert_eq!(idle_generators.len(), generator_count);

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
        assert_eq!(idle_generators.len(), 4);
    }

    #[test]
    fn test_matches_stress() {
        let generator_count = 1000;
        let markets: Vec<String> = (0..1000)
            .flat_map(|index| vec![index.to_string()])
            .collect();

        let mut generator_store = create_new_store_with_generators(
            generator_count,
            Some("10000000000000".into()),
            Some("10000000000000".into()),
            Some("10000000000000".into()),
        );

        let all_generators = { generator_store.clone().all_generators_address() };
        for generator in all_generators {
            for market in markets.clone() {
                // First borrow: get the generator by address
                let generator = generator_store.get_by_address(&generator).unwrap();

                // Now drop the immutable borrow by extracting necessary info
                let random_generator_info_per_market =
                    get_random_market_info_for_generator(&generator.address, market);

                // Second borrow: insert markets
                generator_store.insert_markets(random_generator_info_per_market);
            }
        }

        assert_eq!(
            generator_store
                .query_by_states(vec![GeneratorState::Joined])
                .result()
                .len(),
            generator_count * markets.len()
        );

        assert_eq!(
            generator_store
                .query_by_states(vec![GeneratorState::Joined])
                .filter_by_market_id(U256::one())
                .result()
                .len(),
            generator_count
        );

        let compute_locked_on_request = "1".into();
        let stake_locked_on_request = "1".into();
        let total_requests: usize = 16 * 10 * 2; //16 tps, 10 assignments per tx, 2 times

        let start_time = Instant::now();
        for _ in 0..total_requests {
            // Clone the result to avoid holding a reference while mutating generator_store
            let mut rng = rand::thread_rng();
            let random_market = &markets[rng.gen_range(0..markets.len())];
            let idle_generators: Vec<GeneratorInfoPerMarket> = select_idle_generators(
                generator_store
                    .query_by_market_id(&U256::from_dec_str(random_market).unwrap())
                    .filter_by_state(vec![GeneratorState::Joined, GeneratorState::Wip])
                    .result(),
            );

            assert_eq!(idle_generators.len(), generator_count);

            for idle_generator in idle_generators {
                // Mutable borrow happens here, but no immutable borrow exists at the same time
                generator_store.update_on_compute_locked(
                    &idle_generator.address,
                    U256::from_dec_str(compute_locked_on_request).unwrap(),
                );
                generator_store.update_on_stake_locked(
                    &idle_generator.address,
                    &TEST_TOKEN_ADDRESS,
                    U256::from_dec_str(stake_locked_on_request).unwrap(),
                );
            }
        }
        let duration = start_time.elapsed();
        println!(
            "Stress Test Matching, Requests: {}, Generators x Markets  = {} x {}, Took: {:?}",
            total_requests,
            generator_count,
            markets.len(),
            duration
        );
    }

    fn get_random_market_info_for_generator(
        generator: &H160,
        market_id: String,
    ) -> GeneratorInfoPerMarket {
        GeneratorInfoPerMarket {
            address: *generator,
            market_id: U256::from_dec_str(&market_id).unwrap(),
            compute_required_per_request: U256::from_dec_str("1").unwrap(),
            proof_generation_cost: U256::from_dec_str("5").unwrap(),
            proposed_time: U256::from_dec_str("10").unwrap(),
            active_requests: U256::from_dec_str("0").unwrap(),
            proofs_submitted: U256::from_dec_str("0").unwrap(),
            proofs_slashed: U256::from_dec_str("0").unwrap(),
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
    fn create_new_store_with_generators(
        n: usize,
        default_total_stake: Option<String>,
        default_sum_of_compute_allocations: Option<String>,
        default_declared_compute: Option<String>,
    ) -> GeneratorStore {
        let mut generator_store = GeneratorStore::new();
        let default_total_stake = default_total_stake.unwrap_or_else(|| "100".into());
        let default_sum_of_compute_allocations =
            default_sum_of_compute_allocations.unwrap_or_else(|| "100".into());
        let default_declared_compute = default_declared_compute.unwrap_or_else(|| "100".into());

        for _ in 0..n {
            let generator = Generator {
                address: Address::random(),
                reward_address: Address::random(),
                total_stake: TokenTracker::from_address_string_and_dec_string(
                    vec![TEST_TOKEN_ADDRESS_STRING.to_string()],
                    vec![default_total_stake.clone()],
                )
                .unwrap(),
                sum_of_compute_allocations: U256::from_dec_str(&default_sum_of_compute_allocations)
                    .unwrap(),
                compute_consumed: U256::from_dec_str("0").unwrap(),
                stake_locked: TokenTracker::from_address_string_and_dec_string(
                    vec![TEST_TOKEN_ADDRESS_STRING.to_string()],
                    vec!["0".into()],
                )
                .unwrap(),
                active_market_places: U256::from_dec_str("0").unwrap(),
                declared_compute: U256::from_dec_str(&default_declared_compute).unwrap(),
                intended_stake_util: U256::from_dec_str("1000000000000000000").unwrap(),
                intended_compute_util: U256::from_dec_str("1000000000000000000").unwrap(),
                generator_data: None,
            };

            generator_store.insert(generator);
        }

        generator_store
    }
}
