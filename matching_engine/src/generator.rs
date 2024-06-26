use ethers::core::types::Address;
use ethers::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use tokio::sync::MutexGuard;

use rand::Rng;
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

#[derive(Default, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Hash, Copy)]
pub enum GeneratorState {
    #[default]
    Null,
    Joined,
    NoComputeAvailable,
    Wip,
    RequestedForExit,
    PendingConfirmation, // Not present in contracts
}

impl std::fmt::Debug for GeneratorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value_str = match self {
            GeneratorState::Null => "This is a Null state",
            GeneratorState::Joined => "The generator has joined",
            GeneratorState::NoComputeAvailable => "The generator has no compute available",
            GeneratorState::Wip => "Work in progress state",
            GeneratorState::RequestedForExit => "The generator requested for exit",
            GeneratorState::PendingConfirmation => "The generator is selected for task",
        };
        write!(f, "{}", value_str)
    }
}

impl Clone for GeneratorState {
    fn clone(&self) -> Self {
        match self {
            GeneratorState::Null => GeneratorState::Null,
            GeneratorState::Joined => GeneratorState::Joined,
            GeneratorState::NoComputeAvailable => GeneratorState::NoComputeAvailable,
            GeneratorState::Wip => GeneratorState::Wip,
            GeneratorState::RequestedForExit => GeneratorState::RequestedForExit,
            GeneratorState::PendingConfirmation => GeneratorState::PendingConfirmation,
        }
    }
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
pub struct Key {
    pub address: Address,
    pub key_index: u64,
    pub ecies_pub_key: Option<Bytes>,
}

#[derive(Debug)]
pub struct KeyStore {
    keys: HashMap<(Address, u64), Key>, // Using u64 as a stand-in for uint256.
}

pub fn get_generator_state(state: u8) -> GeneratorState {
    match state {
        0 => GeneratorState::Null,
        1 => GeneratorState::Joined,
        2 => GeneratorState::NoComputeAvailable,
        3 => GeneratorState::Wip,
        4 => GeneratorState::RequestedForExit,
        _ => GeneratorState::Null,
    }
}

pub fn sort_by_total_stake(
    mut generators: Vec<&GeneratorInfoPerMarket>,
) -> Vec<&GeneratorInfoPerMarket> {
    generators.sort_by(|a, b| a.total_stake.cmp(&b.total_stake));
    generators
}

pub fn sort_by_proof_generation_cost(
    mut generators: Vec<&GeneratorInfoPerMarket>,
) -> Vec<&GeneratorInfoPerMarket> {
    generators.sort_by(|a, b| a.proof_generation_cost.cmp(&b.proof_generation_cost));
    generators
}

pub fn sort_by_proposed_time(
    mut generators: Vec<&GeneratorInfoPerMarket>,
) -> Vec<&GeneratorInfoPerMarket> {
    generators.sort_by(|a, b| a.proposed_time.cmp(&b.proposed_time));
    generators
}

pub fn sort_by_proofs_submitted(
    mut generators: Vec<&GeneratorInfoPerMarket>,
) -> Vec<&GeneratorInfoPerMarket> {
    generators.sort_by(|a, b| a.proofs_submitted.cmp(&b.proofs_submitted));
    generators
}

pub fn idle_generator_selector(
    generators: Vec<&GeneratorInfoPerMarket>,
) -> Vec<GeneratorInfoPerMarket> {
    // sort generators based on total stake
    let vec_by_stake = sort_by_total_stake(generators.clone());

    // sort generators based on proof generation cost
    let vec_by_cost = sort_by_proof_generation_cost(generators.clone());

    // sort generators based on proofs submitted
    let vec_by_proofs = sort_by_proofs_submitted(generators.clone());

    // sort generator based on proof generation time
    let vec_by_time = sort_by_proposed_time(generators.clone());

    // Calculating generator score and collecting values
    let mut generator_percentiles = vec![];
    for elem in generators {
        // calculating percentile by total stake
        let percentile_by_stake = get_percentile_by_position(&vec_by_stake, elem);

        // calculating percentile by proof generation cost
        let percentile_by_cost = get_percentile_by_position(&vec_by_cost, elem);

        // calculating percentile by proofs submitted
        let percentile_by_proofs = get_percentile_by_position(&vec_by_proofs, elem);

        // calculating percentile by proof generation time
        let percentile_by_time = get_percentile_by_position(&vec_by_time, elem);

        // calculating generator score for each generator
        let percentile_weights = vec![
            ((100.0 - percentile_by_time), 40.0),
            (percentile_by_stake, 30.0),
            (percentile_by_proofs, 20.0),
            ((100.0 - percentile_by_cost), 10.0),
        ];

        let generator_score = get_generator_score(percentile_weights);

        // Collecting it in a vector
        generator_percentiles.push((elem.clone(), unsafe {
            generator_score.floor().to_int_unchecked::<usize>()
        }));
    }

    // Sorting generators based on scores
    generator_percentiles.sort_by(|a, b| a.1.cmp(&b.1));
    generator_percentiles.reverse();

    // Selecting only the generators with 5 highest generator scores
    let mut to_return = vec![];
    let mut counter = 5;
    for elem in generator_percentiles {
        to_return.push(elem.0.clone());
        counter -= 1;
        if counter == 0 {
            break;
        }
    }
    to_return
}

fn get_percentile_by_position(
    vec: &Vec<&GeneratorInfoPerMarket>,
    generator: &GeneratorInfoPerMarket,
) -> f64 {
    let index = vec.iter().position(|&x| x == generator).unwrap() as f64;
    let total_generators = vec.len() as f64;

    (index / total_generators) * 100_f64
}

fn get_generator_score(vec: Vec<(f64, f64)>) -> f64 {
    let mut sum: f64 = 0.0;
    for elem in vec {
        sum.add_assign(elem.0 * elem.1);
    }

    sum.div(100.0)
}

pub fn random_generator_selection(
    vec: Vec<GeneratorInfoPerMarket>,
) -> Option<GeneratorInfoPerMarket> {
    if vec.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let element = &vec[rng.gen_range(0..vec.len())];
        Some(element.clone())
    }
}

#[allow(unused)]
pub fn weighted_generator_selection(
    vec: Vec<GeneratorInfoPerMarket>,
) -> Option<GeneratorInfoPerMarket> {
    let mut rng = rand::thread_rng();
    let mut generator = vec[0].clone();

    match vec.len() {
        1 => (),
        2 => {
            let mut weights = [0.4, 0.6];
            let mut tmp = 0.0;
            for i in 0..2 {
                tmp += weights[i];
                weights[i] = tmp;
            }

            let selector = rng.gen_range(0.0..1.0);
            for i in 0..2 {
                if selector < weights[i] {
                    generator = vec[i].clone();
                    break;
                }
            }
        }
        3 => {
            let mut weights = [0.2, 0.5, 0.3];
            let mut tmp = 0.0;
            for i in 0..3 {
                tmp += weights[i];
                weights[i] = tmp;
            }

            let selector = rng.gen_range(0.0..1.0);
            for i in 0..3 {
                if selector < weights[i] {
                    generator = vec[i].clone();
                    break;
                }
            }
        }
        4 => {
            let mut weights = [0.1, 0.3, 0.4, 0.2];
            let mut tmp = 0.0;
            for i in 0..4 {
                tmp += weights[i];
                weights[i] = tmp;
            }

            let selector = rng.gen_range(0.0..1.0);
            for i in 0..4 {
                if selector < weights[i] {
                    generator = vec[i].clone();
                    break;
                }
            }
        }
        5 => {
            let mut weights = [0.1, 0.3, 0.2, 0.1, 0.3];
            let mut tmp = 0.0;
            for i in 0..5 {
                tmp += weights[i];
                weights[i] = tmp;
            }

            let selector = rng.gen_range(0.0..1.0);
            for i in 0..5 {
                if selector < weights[i] {
                    generator = vec[i].clone();
                    break;
                }
            }
        }
        _ => log::error!("Invalid length of generators"),
    }

    Some(generator)
}

impl Ord for Generator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.address.cmp(&other.address)
    }
}

#[derive(Debug, Clone)]
pub struct GeneratorStore {
    // Change key to tuple (Address, U256)
    generators: HashMap<Address, Generator>,
    generator_markets: HashMap<(Address, U256), GeneratorInfoPerMarket>,
    state_index: HashMap<GeneratorState, Vec<(Address, U256)>>,
    address_index: HashMap<Address, Vec<U256>>, // to easily fetch all generators by address
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
        self.generators.insert(address, generator);
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

    pub fn update_on_assigned_task(
        &mut self,
        address: &Address,
        market_id: &U256,
        slashing_penalty: U256,
    ) {
        if let Some(generator_market) = self.generator_markets.get_mut(&(*address, *market_id)) {
            generator_market.active_requests.add_assign(U256::one());

            if let Some(generator) = self.generators.get_mut(address) {
                generator
                    .compute_consumed
                    .add_assign(generator_market.compute_required_per_request);
                generator.stake_locked.add_assign(slashing_penalty);
            }
        }
    }

    pub fn update_on_submit_proof(
        &mut self,
        address: &Address,
        market_id: &U256,
        slashing_penalty: U256,
    ) {
        if let Some(generator_market) = self.generator_markets.get_mut(&(*address, *market_id)) {
            generator_market.active_requests.sub_assign(U256::one());
            generator_market.proofs_submitted.add_assign(U256::one());

            if let Some(generator) = self.generators.get_mut(address) {
                generator
                    .compute_consumed
                    .sub_assign(generator_market.compute_required_per_request);
                generator.stake_locked.sub_assign(slashing_penalty);
            }
        }
    }

    pub fn update_on_slashing(
        &mut self,
        address: &Address,
        market_id: &U256,
        slashing_penalty: U256,
    ) {
        if let Some(generator_market) = self.generator_markets.get_mut(&(*address, *market_id)) {
            generator_market.active_requests.sub_assign(U256::one());

            if let Some(generator) = self.generators.get_mut(address) {
                generator
                    .compute_consumed
                    .sub_assign(generator_market.compute_required_per_request);
                generator.stake_locked.sub_assign(slashing_penalty);
                generator.total_stake.sub_assign(slashing_penalty);
            }
        }
    }

    #[allow(unused)]
    pub fn get_all_by_address(&self, address: &Address) -> Vec<&GeneratorInfoPerMarket> {
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
                if utilization >= exponent {
                    if idle_compute.gt(&elem.compute_required_per_request) {
                        generator_result.push(
                            self.generator_markets
                                .get(&(elem.address, elem.market_id))
                                .unwrap(),
                        );
                    }
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
                if utilization >= exponent {
                    if remaining_stake.gt(&min_stake) {
                        generator_result.push(
                            self.generator_markets
                                .get(&(elem.address, elem.market_id))
                                .unwrap(),
                        );
                    }
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
    pub fn get_by_address(&self, address: &Address, value: u64) -> Option<&Key> {
        self.keys.get(&(address.clone(), value))
    }

    // Updated to reflect the tuple key
    pub fn remove_by_address(&mut self, address: &Address, value: u64) {
        self.keys.remove(&(address.clone(), value));
    }

    // Updated to reflect the tuple key
    pub fn update_pub_key(&mut self, address: &Address, value: u64, new_pub_key: Option<Bytes>) {
        if let Some(key) = self.keys.get_mut(&(address.clone(), value)) {
            key.ecies_pub_key = new_pub_key;
        }
    }
}
