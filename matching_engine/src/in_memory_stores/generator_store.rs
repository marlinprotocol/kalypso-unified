use ethers::core::types::Address;
use ethers::prelude::*;
use im::HashSet;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::RwLockReadGuard;

use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::sync::Arc;

use crate::counters::time_counter::EntryCounter;
use crate::generator_lib::delegation::{Delegation, Operation, Source};
use crate::generator_lib::generator_query::GeneratorQueryResult;
use crate::generator_lib::generator_state::GeneratorState;
use crate::generator_lib::generator_store::{Generator, GeneratorInfoPerMarket};
use crate::generator_lib::key_store::KeyStoreOperations;
use crate::generator_lib::points::get_points;
use crate::generator_lib::traits::{
    GeneratorAdditionalQuery, GeneratorAvailability, GeneratorEarningsAndSlashing, GeneratorFilter,
    GeneratorKeyStoreFilterInterfaceTrait, GeneratorLockManagement, GeneratorMarketManagement,
    GeneratorMetadata, GeneratorQuery, GeneratorRegistration, GeneratorSlashingManagement,
    GeneratorStakeComputeManagement, JobMissedCounter, WithdrawalManagement,
};
use crate::generator_lib::withdrawal_request::WithdrawlRequest;
use crate::generator_lib::SlashingRecord;
use crate::utility::{AddressTokenPair, TokenTracker};

use super::delegation_store::DelegationStore;

mod generator_markets_serde {
    use super::*;
    use serde::de::{self, MapAccess, Visitor};
    use serde::ser::SerializeMap;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(
        value: &HashMap<(Address, U256), GeneratorInfoPerMarket>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(value.len()))?;
        for ((address, u256), gen_info) in value {
            // Convert address to hex string (without 0x prefix)
            let address_str = format!("{:x}", address);
            // Convert U256 to decimal string
            let u256_str = u256.to_string();
            // Create a combined key
            let key = format!("{}|{}", address_str, u256_str);
            map.serialize_entry(&key, gen_info)?;
        }
        map.end()
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<(Address, U256), GeneratorInfoPerMarket>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapVisitor;

        impl<'de> Visitor<'de> for MapVisitor {
            type Value = HashMap<(Address, U256), GeneratorInfoPerMarket>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map with keys as 'address|u256' strings")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::new();
                while let Some((key, value)) =
                    access.next_entry::<String, GeneratorInfoPerMarket>()?
                {
                    let parts: Vec<&str> = key.splitn(2, '|').collect();
                    if parts.len() != 2 {
                        return Err(de::Error::custom(format!("invalid key format: {}", key)));
                    }
                    // Parse address from hex string
                    let address = Address::from_str(parts[0]).map_err(de::Error::custom)?;
                    // Parse U256 from decimal string
                    let u256 = U256::from_dec_str(parts[1]).map_err(de::Error::custom)?;
                    map.insert((address, u256), value);
                }
                Ok(map)
            }
        }

        deserializer.deserialize_map(MapVisitor)
    }
}

mod earnings_or_points_per_market_serde {
    use super::*;
    use serde::de::{self, MapAccess, Visitor};
    use serde::ser::SerializeMap;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(
        value: &HashMap<Address, HashMap<U256, U256>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(value.len()))?;
        for (address, inner_map) in value {
            // Convert address to hex string (without 0x prefix)
            let address_str = format!("{:x}", address);
            // Convert inner HashMap<U256, U256> to HashMap<String, String>
            let mut inner_map_serializable = HashMap::with_capacity(inner_map.len());
            for (u256_key, u256_value) in inner_map {
                let key_str = u256_key.to_string();
                let value_str = u256_value.to_string();
                inner_map_serializable.insert(key_str, value_str);
            }
            map.serialize_entry(&address_str, &inner_map_serializable)?;
        }
        map.end()
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<Address, HashMap<U256, U256>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EarningsPerMarketVisitor;

        impl<'de> Visitor<'de> for EarningsPerMarketVisitor {
            type Value = HashMap<Address, HashMap<U256, U256>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map of addresses to maps of U256 keys and U256 values")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::new();
                while let Some((address_str, inner_map_serializable)) =
                    access.next_entry::<String, HashMap<String, String>>()?
                {
                    // Parse address from hex string
                    let address = Address::from_str(&address_str).map_err(de::Error::custom)?;
                    let mut inner_map = HashMap::new();
                    for (key_str, value_str) in inner_map_serializable {
                        // Parse U256 keys and values from decimal strings
                        let u256_key = U256::from_dec_str(&key_str).map_err(de::Error::custom)?;
                        let u256_value =
                            U256::from_dec_str(&value_str).map_err(de::Error::custom)?;
                        inner_map.insert(u256_key, u256_value);
                    }
                    map.insert(address, inner_map);
                }
                Ok(map)
            }
        }

        deserializer.deserialize_map(EarningsPerMarketVisitor)
    }
}

mod slashing_per_generator_per_market_serde {
    use super::*;
    use serde::de::{self, MapAccess, Visitor};
    use serde::ser::SerializeMap;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(
        value: &HashMap<Address, HashMap<U256, TokenTracker>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(value.len()))?;
        for (address, inner_map) in value {
            // Convert Address to hex string without '0x' prefix
            let address_str = format!("{:x}", address);
            // Convert inner HashMap<U256, TokenTracker> to HashMap<String, TokenTracker>
            let mut inner_map_serializable = HashMap::with_capacity(inner_map.len());
            for (u256_key, token_tracker_value) in inner_map {
                let key_str = u256_key.to_string();
                inner_map_serializable.insert(key_str, token_tracker_value);
            }
            map.serialize_entry(&address_str, &inner_map_serializable)?;
        }
        map.end()
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<Address, HashMap<U256, TokenTracker>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SlashingPerGeneratorPerMarketVisitor;

        impl<'de> Visitor<'de> for SlashingPerGeneratorPerMarketVisitor {
            type Value = HashMap<Address, HashMap<U256, TokenTracker>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter
                    .write_str("a map of addresses to maps of U256 keys and TokenTracker values")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::new();
                while let Some((address_str, inner_map_serializable)) =
                    access.next_entry::<String, HashMap<String, TokenTracker>>()?
                {
                    // Parse Address from hex string
                    let address = Address::from_str(&address_str).map_err(de::Error::custom)?;
                    let mut inner_map = HashMap::new();
                    for (key_str, token_tracker_value) in inner_map_serializable {
                        // Parse U256 key from decimal string
                        let u256_key = U256::from_dec_str(&key_str).map_err(de::Error::custom)?;
                        inner_map.insert(u256_key, token_tracker_value);
                    }
                    map.insert(address, inner_map);
                }
                Ok(map)
            }
        }

        deserializer.deserialize_map(SlashingPerGeneratorPerMarketVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorStore {
    // Change key to tuple (Address, U256)
    generators: HashMap<Address, Generator>, // Generator -> Details

    #[serde(with = "generator_markets_serde")]
    generator_markets: HashMap<(Address, U256), GeneratorInfoPerMarket>, //[Generator, MarketId] -> MarketWiseInfo

    state_index: HashMap<GeneratorState, Vec<(Address, U256)>>, // State -> [Generator, MarketId]

    address_index: HashMap<Address, Vec<U256>>, // Generator -> [MarketId] participations

    earnings: HashMap<Address, U256>, // Generator -> TotalEarnings

    #[serde(with = "earnings_or_points_per_market_serde")]
    earnings_per_market: HashMap<Address, HashMap<U256, U256>>, // Generator -> Markets -> Earnings Per Market

    slashings: HashMap<Address, TokenTracker>, // Generator -> Total Slashings

    #[serde(with = "slashing_per_generator_per_market_serde")]
    slashing_per_generator_per_market: HashMap<Address, HashMap<U256, TokenTracker>>, // Generator -> Markets -> slashings per market

    slashing_records: HashMap<Address, Vec<SlashingRecord>>, // Generator -> Slashing Record

    delegation_store: DelegationStore,

    kalypso_points: HashMap<Address, U256>,

    #[serde(with = "earnings_or_points_per_market_serde")]
    kalypso_points_per_market: HashMap<Address, HashMap<U256, U256>>, // Generator -> Markets -> Kalypso Points Per Market

    withdrawl_requests: HashMap<Address, HashSet<WithdrawlRequest>>,

    jobs_missed_counter: EntryCounter<Address>, // Generator -> Jobs Missed Counter
}

impl GeneratorStore {
    fn new() -> Self {
        GeneratorStore {
            generators: HashMap::new(),
            generator_markets: HashMap::new(),
            state_index: HashMap::new(),
            address_index: HashMap::new(),
            earnings: HashMap::new(),
            earnings_per_market: HashMap::new(),
            slashings: HashMap::new(),
            slashing_per_generator_per_market: HashMap::new(),
            slashing_records: HashMap::new(),
            delegation_store: DelegationStore::new(),
            kalypso_points: HashMap::new(),
            kalypso_points_per_market: HashMap::new(),
            withdrawl_requests: HashMap::new(),
            jobs_missed_counter: EntryCounter::new(Duration::from_secs(60 * 60 * 48)), // 48 hours
        }
    }
}

impl Default for GeneratorStore {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratorRegistration for GeneratorStore {
    fn register_generator(&mut self, generator: Generator) {
        let address = generator.address;
        if !self.generators.contains_key(&generator.address) {
            self.generators.insert(address, generator);
        } else {
            if let Some(existing) = self.generators.get_mut(&generator.address) {
                // These commented fields are to be used as same ones before.

                // pub total_native_stake: TokenTracker,
                // pub total_symbiotic_stake: TokenTracker,
                // pub native_stake_locked: TokenTracker,
                // pub symbiotic_stake_locked: TokenTracker,

                existing.address = generator.address;
                existing.reward_address = generator.reward_address;
                existing.sum_of_compute_allocations = generator.sum_of_compute_allocations;
                existing.compute_consumed = generator.compute_consumed;
                existing.active_market_places = generator.active_market_places;
                existing.declared_compute = generator.declared_compute;
                existing.intended_stake_util = generator.intended_compute_util;
                existing.intended_compute_util = generator.intended_compute_util;
                existing.active = generator.active;
                existing.generator_data = generator.generator_data;
            }
        }
    }

    fn register_generator_in_market(&mut self, generator_market: GeneratorInfoPerMarket) {
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
            .entry((address, market_id))
            .or_insert(generator_market);
    }

    fn remove_by_address_and_market(&mut self, address: &Address, market_id: &U256) {
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

        if let Some(vec) = self.address_index.get_mut(&address) {
            vec.retain(|&id| id != market_id.clone());
            if vec.is_empty() {
                self.address_index.remove(&address);
            }
        }
    }

    fn remove_by_address(&mut self, address: &Address) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.active = false;
        }
    }

    fn get_by_address(&self, address: &Address) -> Option<Generator> {
        self.generators.get(address).cloned()
    }

    fn is_active(&self, generator_address: &Address) -> bool {
        if let Some(generator) = self.generators.get(generator_address) {
            generator.active.clone()
        } else {
            false
        }
    }
}

impl GeneratorStakeComputeManagement for GeneratorStore {
    fn add_extra_stake(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        amount: &U256,
        block_number: U64,
        transaction_index: U64,
        log_index: U256,
        tx: String,
        source: Source,
    ) {
        if let Some(generator) = self.generators.get_mut(generator_address) {
            match source {
                Source::Native => generator
                    .total_native_stake
                    .add_token(token_address, amount),
                Source::Symbiotic => generator
                    .total_symbiotic_stake
                    .add_token(token_address, amount),
            };

            self.delegation_store.add_delegation(
                generator_address,
                Delegation {
                    delegation: (*token_address, *amount),
                    source,
                    operation: Operation::Delegate,
                    block_number,
                    transaction_index,
                    log_index,
                    tx,
                },
            );
        }
    }

    fn update_intended_stake_util(&mut self, address: &Address, new_stake_util: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.intended_stake_util = new_stake_util;
        }
    }

    fn remove_stake(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        amount: &U256,
        block_number: U64,
        transaction_index: U64,
        log_index: U256,
        tx: String,
        operation: Operation,
        source: Source,
    ) {
        if let Some(generator) = self.generators.get_mut(generator_address) {
            match source {
                Source::Native => {
                    log::debug!("Existing Native Stake: {:?}", generator.total_native_stake);
                    log::debug!("Token to remove: {:?}, Amount: {:?}", token_address, amount);
                    generator
                        .total_native_stake
                        .sub_token(token_address, amount)
                        .unwrap();
                }

                Source::Symbiotic => {
                    log::debug!(
                        "Existing Symbiotic Stake: {:?}",
                        generator.total_symbiotic_stake
                    );
                    log::debug!("Token to remove: {:?}, Amount: {:?}", token_address, amount);
                    generator
                        .total_symbiotic_stake
                        .sub_token(token_address, amount)
                        .unwrap();
                }
            }

            self.delegation_store.add_delegation(
                generator_address,
                Delegation {
                    delegation: (*token_address, *amount),
                    source,
                    operation,
                    block_number,
                    transaction_index,
                    log_index,
                    tx,
                },
            );
        }
    }

    fn update_reward_address(&mut self, address: &Address, new_reward_address: Address) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.reward_address = new_reward_address;
        }
    }

    fn add_extra_compute(&mut self, address: &Address, compute: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.declared_compute = generator.declared_compute.add(compute);
        }
    }

    fn update_intended_compute_util(&mut self, address: &Address, new_compute_util: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.intended_compute_util = new_compute_util;
        }
    }

    fn remove_compute(&mut self, address: &Address, compute: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.declared_compute = generator.declared_compute.sub(compute);
        }
    }
}

impl GeneratorMarketManagement for GeneratorStore {
    fn update_state(&mut self, address: &Address, market_id: &U256, new_state: GeneratorState) {
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

    fn update_on_assigned_task(&mut self, address: &Address, market_id: &U256) {
        if let Some(generator_market) = self.generator_markets.get_mut(&(*address, *market_id)) {
            generator_market.active_requests.add_assign(U256::one());
        }
    }

    fn update_on_submit_proof(
        &mut self,
        address: &Address,
        market_id: &U256,
        earning: &U256,
        block_number: &U64,
    ) {
        if let Some(generator_market) = self.generator_markets.get_mut(&(*address, *market_id)) {
            generator_market.active_requests.sub_assign(U256::one());
            generator_market.proofs_submitted.add_assign(U256::one());
        }

        // Update the total earnings for the address
        self.earnings
            .entry(*address)
            .and_modify(|e| *e = e.add(*earning))
            .or_insert(*earning);

        // Update the earnings for the specific market
        self.earnings_per_market
            .entry(*address)
            .or_insert_with(HashMap::new)
            .entry(*market_id)
            .and_modify(|e| *e = e.add(*earning))
            .or_insert(*earning);

        let kalypso_points_per_proof = get_points(block_number.as_u64());
        self.kalypso_points
            .entry(*address)
            .and_modify(|e| *e = e.add(kalypso_points_per_proof))
            .or_insert(kalypso_points_per_proof);

        self.kalypso_points_per_market
            .entry(*address)
            .or_insert_with(HashMap::new)
            .entry(*market_id)
            .and_modify(|e| *e = e.add(kalypso_points_per_proof))
            .or_insert(kalypso_points_per_proof);
    }

    fn reduce_active_requests(&mut self, generator_address: &Address, market_id: &U256) {
        if let Some(generator_market) = self
            .generator_markets
            .get_mut(&(*generator_address, *market_id))
        {
            generator_market.active_requests.sub_assign(U256::one());
            generator_market.proofs_slashed.add_assign(U256::one());
        }
    }

    fn pause_assignments_across_all_markets(&mut self, address: &Address) {
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

    fn resume_assignments_accross_all_markets(&mut self, address: &Address) {
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
}

impl GeneratorSlashingManagement for GeneratorStore {
    fn note_entry_slashing(
        &mut self,
        generator_address: &Address,
        ask_id: &U256,
        market_id: &U256,
        native_tokens_slashed: Vec<Address>,
        native_slashings: Vec<U256>,
        symbiotic_tokens_slashed: Vec<Address>,
        symbiotic_slashings: Vec<U256>,
        slashing_tx: String,
        price_offered: &U256,
        deadline: &U256,
        slashing_block_number: &U64,
        slashing_timestamp: &U256,
    ) {
        for (token_address, slashing) in native_tokens_slashed.iter().zip(native_slashings.iter()) {
            // Update the slashing tracker for each token
            self.slashings
                .entry(*generator_address)
                .and_modify(|tracker| tracker.add_token(token_address, slashing)) // Modify the existing entry
                .or_insert_with(|| {
                    let mut tracker = TokenTracker::new(); // Create a new TokenTracker if none exists
                    tracker.add_token(token_address, slashing); // Add the slashing amount
                    tracker
                });

            self.slashing_per_generator_per_market
                .entry(*generator_address)
                .or_insert_with(HashMap::new) // Create the inner HashMap if it doesn't exist
                .entry(*market_id)
                .and_modify(|tracker| tracker.add_token(token_address, slashing)) // Modify the existing TokenTracker
                .or_insert_with(|| {
                    let mut tracker = TokenTracker::new(); // Create a new TokenTracker if none exists
                    tracker.add_token(token_address, slashing); // Add the slashing amount
                    tracker
                });
        }

        // Record the slashing event
        for (token_address, slashing) in native_tokens_slashed.iter().zip(native_slashings.iter()) {
            self.slashing_records
                .entry(*generator_address)
                .or_insert_with(Vec::new)
                .push(SlashingRecord {
                    ask_id: ask_id.clone(),
                    market_id: market_id.clone(),
                    slashing_tx: slashing_tx.clone(),
                    price_offered: price_offered.clone(),
                    expected_time: deadline.clone(),
                    slashing_penalty: (token_address.clone(), slashing.clone()),
                    slashing_block_number: slashing_block_number.clone(),
                    slashing_timestamp: slashing_timestamp.clone(),
                    source: Source::Native,
                });
        }

        for (token_address, slashing) in symbiotic_tokens_slashed
            .iter()
            .zip(symbiotic_slashings.iter())
        {
            // Update the slashing tracker for each token
            self.slashings
                .entry(*generator_address)
                .and_modify(|tracker| tracker.add_token(token_address, slashing)) // Modify the existing entry
                .or_insert_with(|| {
                    let mut tracker = TokenTracker::new(); // Create a new TokenTracker if none exists
                    tracker.add_token(token_address, slashing); // Add the slashing amount
                    tracker
                });

            self.slashing_per_generator_per_market
                .entry(*generator_address)
                .or_insert_with(HashMap::new) // Create the inner HashMap if it doesn't exist
                .entry(*market_id)
                .and_modify(|tracker| tracker.add_token(token_address, slashing)) // Modify the existing TokenTracker
                .or_insert_with(|| {
                    let mut tracker = TokenTracker::new(); // Create a new TokenTracker if none exists
                    tracker.add_token(token_address, slashing); // Add the slashing amount
                    tracker
                });
        }

        // Record the slashing event
        for (token_address, slashing) in symbiotic_tokens_slashed
            .iter()
            .zip(symbiotic_slashings.iter())
        {
            self.slashing_records
                .entry(*generator_address)
                .or_insert_with(Vec::new)
                .push(SlashingRecord {
                    ask_id: ask_id.clone(),
                    market_id: market_id.clone(),
                    slashing_tx: slashing_tx.clone(),
                    price_offered: price_offered.clone(),
                    expected_time: deadline.clone(),
                    slashing_penalty: (token_address.clone(), slashing.clone()),
                    slashing_block_number: slashing_block_number.clone(),
                    slashing_timestamp: slashing_timestamp.clone(),
                    source: Source::Symbiotic,
                });
        }
    }
}

impl GeneratorLockManagement for GeneratorStore {
    fn update_on_stake_locked(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_locked: U256,
        source: Source,
    ) {
        if let Some(generator) = self.generators.get_mut(generator_address) {
            match source {
                Source::Native => generator
                    .native_stake_locked
                    .add_token(token_address, &stake_locked),
                Source::Symbiotic => generator
                    .symbiotic_stake_locked
                    .add_token(token_address, &stake_locked),
            }
        }
    }

    fn update_on_stake_released(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_released: U256,
        source: Source,
    ) {
        if let Some(generator) = self.generators.get_mut(generator_address) {
            match source {
                Source::Native => generator
                    .native_stake_locked
                    .sub_token(token_address, &stake_released)
                    .unwrap(),
                Source::Symbiotic => generator
                    .symbiotic_stake_locked
                    .sub_token(token_address, &stake_released)
                    .unwrap(),
            }
        }
    }

    fn update_on_compute_locked(&mut self, address: &Address, compute_locked: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.compute_consumed.add_assign(compute_locked);
        }
    }

    fn update_on_compute_released(&mut self, address: &Address, compute_released: U256) {
        if let Some(generator) = self.generators.get_mut(address) {
            generator.compute_consumed.sub_assign(compute_released);
        }
    }
}

impl GeneratorAvailability for GeneratorStore {
    fn get_available_compute(&self, address: Address) -> Option<U256> {
        self.generators
            .get(&address)
            .map(|generator| generator.declared_compute.sub(generator.compute_consumed))
    }

    fn get_available_native_stake(&self, generator_address: &Address) -> Option<TokenTracker> {
        self.generators.get(&generator_address).map(|generator| {
            generator.total_native_stake.clone() - generator.native_stake_locked.clone()
        })
    }

    fn get_available_symbiotic_stake(&self, generator_address: &Address) -> Option<TokenTracker> {
        self.generators.get(&generator_address).map(|generator| {
            generator.total_symbiotic_stake.clone() - generator.symbiotic_stake_locked.clone()
        })
    }

    fn get_native_stake_locked(&self, generator_address: &Address) -> Option<TokenTracker> {
        self.generators
            .get(&generator_address)
            .map(|generator| generator.native_stake_locked.clone())
    }

    fn get_symbiotic_stake_locked(&self, generator_address: &Address) -> Option<TokenTracker> {
        self.generators
            .get(&generator_address)
            .map(|generator| generator.symbiotic_stake_locked.clone())
    }

    fn get_all_by_market_id(&self, market_id: &U256) -> Vec<GeneratorInfoPerMarket> {
        // Clone self and wrap it in an Arc for thread-safe sharing
        let self_arc = Arc::new(self.clone());

        // Retrieve all generator addresses
        let all_generators = self_arc.all_generators_address();

        // Process generators in parallel using Rayon
        let generator_for_given_market: Vec<GeneratorInfoPerMarket> = all_generators
            .par_iter()
            .filter_map(|generator| {
                // Clone the Arc to share ownership across threads
                let self_clone = Arc::clone(&self_arc);
                // Retrieve generator info for the given market
                self_clone.get_by_address_and_market(generator, market_id)
            })
            .collect();

        generator_for_given_market
    }
}

impl GeneratorQuery for GeneratorStore {
    #[allow(unused)]
    fn query(&self) -> GeneratorQueryResult {
        GeneratorQueryResult::new(self.generator_markets.values().collect())
    }

    fn query_by_market_id_and_only_active(&self, market_id: &U256) -> GeneratorQueryResult {
        log::debug!("Check query by market id");

        let generator_markets: Vec<&GeneratorInfoPerMarket> = self
            .generator_markets
            .par_iter() // Parallel iterator over the generator_markets HashMap
            .filter_map(|((_, gen_market_id), generator_info)| {
                // Check if the market_id matches
                if self.is_active(&generator_info.address) {
                    if gen_market_id == market_id {
                        Some(generator_info)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect(); // Collect matching generator markets into a Vec

        GeneratorQueryResult::new(generator_markets)
    }

    #[allow(unused)]
    fn query_by_states(&self, states: Vec<GeneratorState>) -> GeneratorQueryResult {
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
    fn query_by_address(&self, address: Address) -> GeneratorQueryResult {
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

impl GeneratorFilter for GeneratorStore {
    fn filter_by_has_idle_compute(
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

        log::debug!("Generator with idle compute: {}", generator_result.len());
        GeneratorQueryResult::new(generator_result)
    }

    fn filter_by_available_native_stake(
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
                        .total_native_stake
                        .clone()
                        .sub(generator.native_stake_locked.clone());

                    log::debug!(
                        "Filtering Generator by available native stake. Generator: {:?}",
                        generator.address
                    );
                    log::debug!(
                        "Generator: {:?} remaining native stake: {:?}",
                        generator.address,
                        remaining_stake
                    );
                    log::debug!(
                        "Generator: {:?} one of the token required in native stake: {:?}",
                        generator.address,
                        min_stake
                    );

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

        log::debug!(
            "Number of generators available with native stake {:?} = {}",
            min_stake,
            generator_result.len()
        );
        GeneratorQueryResult::new(generator_result)
    }

    fn filter_by_available_symbiotic_stake(
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
                        .total_symbiotic_stake
                        .clone()
                        .sub(generator.symbiotic_stake_locked.clone());

                    log::debug!(
                        "Filtering Generator by available symbiotic stake. Generator: {:?}",
                        generator.address
                    );
                    log::debug!(
                        "Generator: {:?} remaining symbiotic stake: {:?}",
                        generator.address,
                        remaining_stake
                    );
                    log::debug!(
                        "Generator: {:?} one of the token required in symbiotic stake: {:?}",
                        generator.address,
                        min_stake
                    );

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

        log::debug!(
            "Number of generators available with symbiotic stake {:?} = {}",
            min_stake,
            generator_result.len()
        );
        GeneratorQueryResult::new(generator_result)
    }
}

impl<KS: KeyStoreOperations> GeneratorKeyStoreFilterInterfaceTrait<KS> for GeneratorStore {
    fn filter_by_has_private_inputs_support(
        &self,
        generator_query: GeneratorQueryResult,
        key_store: RwLockReadGuard<'_, KS>,
    ) -> GeneratorQueryResult {
        let generator_array = generator_query.result();

        // Use rayon's parallel iterator to process in parallel
        let generator_result: Vec<&GeneratorInfoPerMarket> = generator_array
            .into_iter() // Convert to a parallel iterator
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

impl GeneratorEarningsAndSlashing for GeneratorStore {
    // Get total earnings for a specific address
    fn get_total_earning(&self, address: &Address) -> Option<U256> {
        self.earnings.get(address).cloned()
    }

    // Get earnings for a specific address and market
    fn get_earning_per_market(&self, address: &Address, market_id: &U256) -> Option<U256> {
        self.earnings_per_market
            .get(address)
            .and_then(|market_earnings| market_earnings.get(market_id).cloned())
    }

    // Get total earnings for a specific address
    fn get_kalypso_points(&self, address: &Address) -> Option<U256> {
        self.kalypso_points.get(address).cloned()
    }

    // Get earnings for a specific address and market
    fn get_kalypso_points_per_market(&self, address: &Address, market_id: &U256) -> Option<U256> {
        self.kalypso_points_per_market
            .get(address)
            .and_then(|market_earnings| market_earnings.get(market_id).cloned())
    }

    fn get_total_slashing(&self, generator_address: &Address) -> Option<TokenTracker> {
        self.slashings.get(generator_address).cloned()
    }

    fn get_slashing_per_generator_per_market(
        &self,
        address: &Address,
        market_id: &U256,
    ) -> Option<TokenTracker> {
        self.slashing_per_generator_per_market
            .get(address)
            .and_then(|elem| elem.get(market_id).cloned())
    }

    fn get_slashing_records(&self, address: &Address) -> Vec<SlashingRecord> {
        let data = self.slashing_records.get(address);
        if data.is_none() {
            return vec![];
        } else {
            return data.unwrap().clone();
        }
    }
}

impl WithdrawalManagement for GeneratorStore {
    fn insert_withdrawal_request(
        &mut self,
        operator_address: &Address,
        withdrawal_request: WithdrawlRequest,
    ) {
        self.withdrawl_requests
            .entry(operator_address.clone())
            .or_insert_with(HashSet::new)
            .insert(withdrawal_request);
    }

    fn get_withdrawl_requests(&self, operator_address: &Address) -> Vec<WithdrawlRequest> {
        match self.withdrawl_requests.get(operator_address) {
            Some(requests) => requests.iter().cloned().collect(),
            None => Vec::new(),
        }
    }

    fn remove_withdrawal_request(
        &mut self,
        operator_address: &Address,
        withdrawal_request: WithdrawlRequest,
    ) -> bool {
        if let Some(requests) = self.withdrawl_requests.get_mut(operator_address) {
            let removed = requests.remove(&withdrawal_request);
            // Optionally, remove the address entry if no more requests exist
            if requests.is_empty() {
                self.withdrawl_requests.remove(operator_address);
            }
            removed.is_some()
        } else {
            false
        }
    }
}

impl GeneratorMetadata for GeneratorStore {
    fn update_generator_metadata(
        &mut self,
        generator_address: Address,
        generator_meta_data: Bytes,
    ) {
        if let Some(generator) = self.generators.get_mut(&generator_address) {
            generator.generator_data = generator_meta_data;
        }
    }
}

impl JobMissedCounter for GeneratorStore {
    fn count_job_missed_by_generator(
        &mut self,
        generator_address: Address,
        timestamp: std::time::SystemTime,
    ) {
        self.jobs_missed_counter.add(generator_address, timestamp);
    }

    fn get_job_missed_count(&self, generator_address: &Address) -> usize {
        self.jobs_missed_counter.count(generator_address)
    }
}

// create, update the generator store
impl GeneratorAdditionalQuery for GeneratorStore {
    fn all_generators_address(&self) -> Vec<Address> {
        self.generators
            .par_iter()
            .map(|(address, _)| address.clone())
            .collect()
    }

    fn get_delegations(
        &self,
        generator_address: &Address,
        operations: Vec<Operation>,
        skip: Option<usize>,
        count: Option<usize>,
    ) -> Vec<Delegation> {
        self.delegation_store.get_delegations_by_operations(
            generator_address,
            operations,
            skip.unwrap_or(0),
            count.unwrap_or(100),
        )
    }

    fn total_native_stake_accross_all_generators(&self) -> TokenTracker {
        self.generators
            .par_iter()
            .map(|(_, data)| data.total_native_stake.clone()) // Clone if TokenTracker isn't Copy
            .reduce(
                || TokenTracker::new(),      // Identity element
                |acc, stake| acc.add(stake), // Combine function
            )
    }

    fn total_symbiotic_stake_across_all_generators(&self) -> TokenTracker {
        self.generators
            .par_iter()
            .map(|(_, data)| data.total_symbiotic_stake.clone()) // Clone if TokenTracker isn't Copy
            .reduce(
                || TokenTracker::new(),      // Identity element
                |acc, stake| acc.add(stake), // Combine function
            )
    }

    fn get_by_address_and_market(
        &self,
        address: &Address,
        market_id: &U256,
    ) -> Option<GeneratorInfoPerMarket> {
        self.generator_markets.get(&(*address, *market_id)).cloned()
    }

    fn get_all_markets_of_generator(&self, address: &Address) -> Vec<GeneratorInfoPerMarket> {
        match self.address_index.get(address) {
            Some(market_ids) => market_ids
                .par_iter()
                .filter_map(|m_id| self.generator_markets.get(&(*address, *m_id)).cloned())
                .collect(),
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        generator_lib::{
            generator_helper::select_idle_generators,
            traits::{
                GeneratorAdditionalQuery, GeneratorAvailability, GeneratorFilter,
                GeneratorLockManagement, GeneratorQuery, GeneratorRegistration,
                GeneratorStakeComputeManagement,
            },
        },
        utility::TokenTracker,
    };

    use super::{Generator, GeneratorInfoPerMarket, GeneratorState, GeneratorStore};
    use ethers::{
        core::rand::{self, seq::SliceRandom},
        types::{Address, H160, U256},
    };

    use rand::Rng;
    use std::time::Instant;

    pub const TEST_TOKEN_ADDRESS_ONE_STRING: &str = "0xB5570D4D39dD20F61dEf7C0d6846790360b89a18";
    pub const TEST_TOKEN_ADDRESS_TWO_STRING: &str = "0x854493FB9F844c8632140ffF9B66207B10027E8d";
    pub const TEST_TOKEN_ADDRESS_THREE_STRING: &str = "0x5E478CB7576906fe2a443684aDcD9A0dfc547abD";

    use once_cell::sync::Lazy;
    pub static TEST_TOKEN_ADDRESS_ONE: Lazy<Address> =
        Lazy::new(|| TEST_TOKEN_ADDRESS_ONE_STRING.parse::<Address>().unwrap());

    pub static TEST_TOKEN_ADDRESS_TWO: Lazy<Address> =
        Lazy::new(|| TEST_TOKEN_ADDRESS_TWO_STRING.parse::<Address>().unwrap());

    #[allow(unused)]
    pub static TEST_TOKEN_ADDRESS_THREE: Lazy<Address> =
        Lazy::new(|| TEST_TOKEN_ADDRESS_THREE_STRING.parse::<Address>().unwrap());

    #[test]
    fn test_insert_remove_generators() {
        let mut generator_store = GeneratorStore::new();

        let generator1 = Generator {
            address: Address::random(),
            reward_address: Address::random(),
            total_native_stake: TokenTracker::from_address_string_and_dec_string(
                vec![TEST_TOKEN_ADDRESS_ONE_STRING.to_string()],
                vec!["123123".into()],
            )
            .unwrap(),
            total_symbiotic_stake: TokenTracker::new(),
            sum_of_compute_allocations: U256::from_dec_str("12312312").unwrap(),
            compute_consumed: U256::from_dec_str("12312").unwrap(),
            native_stake_locked: TokenTracker::from_address_string_and_dec_string(
                vec![TEST_TOKEN_ADDRESS_ONE_STRING.to_string()],
                vec!["123123".into()],
            )
            .unwrap(),
            symbiotic_stake_locked: TokenTracker::new(),
            active_market_places: U256::from_dec_str("12").unwrap(),
            declared_compute: U256::from_dec_str("123123").unwrap(),
            intended_stake_util: U256::from_dec_str("123123").unwrap(),
            intended_compute_util: U256::from_dec_str("123123").unwrap(),
            generator_data: vec![].into(),
            active: true,
        };

        generator_store.register_generator(generator1.clone());

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
            random_generator.total_native_stake + random_generator.total_symbiotic_stake,
            TokenTracker::from_address_string_and_dec_string(
                vec![TEST_TOKEN_ADDRESS_ONE_STRING.to_string()],
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
            &TEST_TOKEN_ADDRESS_ONE,
            &U256::from_dec_str("5").unwrap(),
            0.into(),
            0.into(),
            0.into(),
            "".into(),
            crate::generator_lib::delegation::Source::Native,
        );

        assert_eq!(
            generator_store
                .get_by_address(&random_generator.address)
                .unwrap()
                .total_native_stake
                + generator_store
                    .get_by_address(&random_generator.address)
                    .unwrap()
                    .total_symbiotic_stake,
            TokenTracker::from_address_string_and_dec_string(
                vec![TEST_TOKEN_ADDRESS_ONE_STRING.to_string()],
                vec!["105".into()],
            )
            .unwrap()
        );

        generator_store.remove_stake(
            &random_generator.address,
            &TEST_TOKEN_ADDRESS_ONE,
            &U256::from_dec_str("15").unwrap(),
            0.into(),
            0.into(),
            0.into(),
            "".into(),
            crate::generator_lib::delegation::Operation::UnDelegate,
            crate::generator_lib::delegation::Source::Native,
        );

        assert_eq!(
            generator_store
                .get_by_address(&random_generator.address)
                .unwrap()
                .total_native_stake
                + generator_store
                    .get_by_address(&random_generator.address)
                    .unwrap()
                    .total_symbiotic_stake,
            TokenTracker::from_address_string_and_dec_string(
                vec![TEST_TOKEN_ADDRESS_ONE_STRING.to_string()],
                vec!["90".into()],
            )
            .unwrap()
        );

        generator_store.add_extra_stake(
            &random_generator.address,
            &TEST_TOKEN_ADDRESS_TWO,
            &U256::from_dec_str("20").unwrap(),
            0.into(),
            0.into(),
            0.into(),
            "".into(),
            crate::generator_lib::delegation::Source::Native,
        );

        assert_eq!(
            generator_store
                .get_by_address(&random_generator.address)
                .unwrap()
                .total_native_stake
                + generator_store
                    .get_by_address(&random_generator.address)
                    .unwrap()
                    .total_symbiotic_stake,
            TokenTracker::from_address_string_and_dec_string(
                vec![
                    TEST_TOKEN_ADDRESS_ONE_STRING.to_string(),
                    TEST_TOKEN_ADDRESS_TWO_STRING.to_string()
                ],
                vec!["90".into(), "20".into()],
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
        generator_store.register_generator_in_market(random_generator_info_per_market);

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
            generator_store.register_generator_in_market(random_generator_info_per_market);
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
            generator_store.register_generator_in_market(random_generator_info_per_market);
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
            generator_store.register_generator_in_market(random_generator_info_per_market);
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
                generator_store.register_generator_in_market(random_generator_info_per_market);
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
                    .query_by_market_id_and_only_active(&U256::from_dec_str(random_market).unwrap())
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
                    &TEST_TOKEN_ADDRESS_ONE,
                    U256::from_dec_str(stake_locked_on_request).unwrap(),
                    crate::generator_lib::delegation::Source::Native,
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
                total_native_stake: TokenTracker::from_address_string_and_dec_string(
                    vec![TEST_TOKEN_ADDRESS_ONE_STRING.to_string()],
                    vec![default_total_stake.clone()],
                )
                .unwrap(),
                total_symbiotic_stake: TokenTracker::new(),
                sum_of_compute_allocations: U256::from_dec_str(&default_sum_of_compute_allocations)
                    .unwrap(),
                compute_consumed: U256::from_dec_str("0").unwrap(),
                native_stake_locked: TokenTracker::from_address_string_and_dec_string(
                    vec![TEST_TOKEN_ADDRESS_ONE_STRING.to_string()],
                    vec!["0".into()],
                )
                .unwrap(),
                symbiotic_stake_locked: TokenTracker::new(),
                active_market_places: U256::from_dec_str("0").unwrap(),
                declared_compute: U256::from_dec_str(&default_declared_compute).unwrap(),
                intended_stake_util: U256::from_dec_str("1000000000000000000").unwrap(),
                intended_compute_util: U256::from_dec_str("1000000000000000000").unwrap(),
                generator_data: vec![].into(),
                active: true,
            };

            generator_store.register_generator(generator);
        }

        generator_store
    }
}
