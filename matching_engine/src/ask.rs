use ethers::core::types::U256;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

#[allow(unused)]
pub enum Comparison {
    Equal,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Hash)]
pub enum AskState {
    #[default]
    Null,
    Create,
    UnAssigned,
    Assigned,
    Complete,
    DeadlineCrossed,
    // added latter and not in contract
    InvalidSecret,
}

impl std::fmt::Debug for AskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value_str = match self {
            AskState::Null => "This is a Null state",
            AskState::Create => "The ask was created",
            AskState::UnAssigned => "The ask is unassignable (likely because of expiry)",
            AskState::Assigned => "The ask is assigned",
            AskState::Complete => "The ask is complete",
            AskState::DeadlineCrossed => "The ask deadline has been crossed",
            AskState::InvalidSecret => "The secret for the ask is invalid",
        };
        write!(f, "{}", value_str)
    }
}

pub fn get_ask_state(state: u8) -> AskState {
    match state {
        0 => AskState::Null,
        1 => AskState::Create,
        2 => AskState::UnAssigned,
        3 => AskState::Assigned,
        4 => AskState::Complete,
        5 => AskState::DeadlineCrossed,
        // added latter and not in contract
        6 => AskState::InvalidSecret,
        _ => AskState::Null,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct MarketMetadata {
    pub market_id: U256,
    pub verifier: Address,
    pub prover_image_id: [u8; 32],
    pub slashing_penalty: U256,
    pub activation_block: U256,
    pub ivs_image_id: [u8; 32],
    pub metadata: Bytes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct LocalAsk {
    pub ask_id: U256,
    pub market_id: U256,
    pub reward: U256,
    pub expiry: U256,
    pub proving_time: U256,
    pub deadline: U256,
    pub prover_refund_address: Address,
    pub prover_data: Bytes,
    pub has_private_inputs: bool,
    pub secret_data: Option<Bytes>,
    pub secret_acl: Option<Bytes>,
    pub state: Option<AskState>,
    pub generator: Option<Address>,
    pub invalid_secret_flag: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalAskStatus {
    pub created: usize,
    pub unassigned: usize,
    pub assigned: usize,
    pub completed: usize,
    pub deadline_crossed: usize,
    pub invalid_secret: usize,
}

impl Ord for LocalAsk {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ask_id.cmp(&other.ask_id)
    }
}

pub struct LocalAskStore {
    asks_by_id: HashMap<U256, LocalAsk>,
    market_id_index: HashMap<U256, Vec<LocalAsk>>,
    state_index: HashMap<AskState, Vec<LocalAsk>>,
}

pub struct AskQueryResult {
    asks: Option<Vec<LocalAsk>>,
}

impl AskQueryResult {
    #[allow(unused)]
    pub fn sort_by_expiry(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.sort_by(|a, b| a.expiry.cmp(&b.expiry));
        }
        self
    }

    pub fn result(self) -> Option<Vec<LocalAsk>> {
        self.asks
    }

    pub fn sort_by_ask_id(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.sort_by(|a, b| a.ask_id.cmp(&b.ask_id));
        }
        self
    }

    #[allow(unused)]
    pub fn filter_by_market_id(self, market_id: U256) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| ask.market_id == market_id)
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn sort_by_proving_time(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.sort_by(|a, b| a.proving_time.cmp(&b.proving_time));
        }
        self
    }

    #[allow(unused)]
    pub fn sort_by_reward(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.sort_by(|a, b| a.reward.cmp(&b.reward));
        }
        self
    }

    #[allow(unused)]
    pub fn sort_by_deadline(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.sort_by(|a, b| a.deadline.cmp(&b.deadline));
        }
        self
    }

    #[allow(unused)]
    pub fn filter_by_has_private_inputs(self, value: bool) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| ask.has_private_inputs == value)
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    pub fn filter_by_flag(self, value: bool) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| ask.invalid_secret_flag == value)
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    fn compare(value: U256, other: U256, comparison: &Comparison) -> bool {
        match comparison {
            Comparison::Equal => value == other,
            Comparison::LessThan => value < other,
            Comparison::GreaterThan => value > other,
            Comparison::LessThanOrEqual => value <= other,
            Comparison::GreaterThanOrEqual => value >= other,
        }
    }

    #[allow(unused)]
    pub fn filter_by_expiry(self, value: U256, comparison: Comparison) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| Self::compare(ask.expiry, value, &comparison))
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn filter_by_proving_time(self, value: U256, comparison: Comparison) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| Self::compare(ask.proving_time, value, &comparison))
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn filter_by_reward(self, value: U256, comparison: Comparison) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| Self::compare(ask.reward, value, &comparison))
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn filter_by_deadline(self, value: U256, comparison: Comparison) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| Self::compare(ask.deadline, value, &comparison))
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn filter_by_prover_refund_address(self, address: Address) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_iter()
                .filter(|ask| ask.prover_refund_address == address)
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    pub fn get_count(self) -> usize {
        self.asks.map(|v| v.len()).unwrap_or(0)
    }
}

impl LocalAskStore {
    pub fn new() -> Self {
        LocalAskStore {
            asks_by_id: HashMap::new(),
            market_id_index: HashMap::new(),
            state_index: HashMap::new(),
        }
    }

    pub fn insert(&mut self, ask: LocalAsk) {
        self.asks_by_id.insert(ask.ask_id, ask.clone());

        self.market_id_index
            .entry(ask.market_id)
            .or_default()
            .push(ask.clone());

        if let Some(state) = ask.state {
            self.state_index.entry(state).or_default().push(ask);
        }
    }

    #[allow(unused)]
    pub fn remove_by_ask_id(&mut self, ask_id: &U256) {
        if let Some(ask) = self.asks_by_id.remove(ask_id) {
            if let Some(vec) = self.market_id_index.get_mut(&ask.market_id) {
                vec.retain(|a| a.ask_id != *ask_id);
            }

            if let Some(state) = ask.state {
                if let Some(vec) = self.state_index.get_mut(&state) {
                    vec.retain(|a| a.ask_id != *ask_id);
                }
            }
        }
    }

    pub fn modify_state(&mut self, ask_id: &U256, new_state: AskState) {
        if let Some(ask) = self.asks_by_id.get_mut(ask_id) {
            if let Some(old_state) = ask.state.take() {
                if let Some(vec) = self.state_index.get_mut(&old_state) {
                    vec.retain(|a| a.ask_id != *ask_id);
                }
            }

            ask.state = Some(new_state);
            self.state_index
                .entry(new_state)
                .or_default()
                .push(ask.clone());
        }
    }

    // pub fn update_task_id(&mut self, ask_id: &U256, new_task_id: Option<U256>) {
    //     if let Some(ask) = self.asks_by_id.get_mut(ask_id) {
    //         ask.task_id = new_task_id;
    //     }
    // }

    pub fn update_ask_generator(&mut self, ask_id: &U256, new_generator: Option<Address>) {
        if let Some(ask) = self.asks_by_id.get_mut(ask_id) {
            ask.generator = new_generator;
        }
    }

    pub fn update_ask_acl(&mut self, ask_id: &U256, new_acl: Option<Bytes>) {
        if let Some(ask) = self.asks_by_id.get_mut(ask_id) {
            ask.secret_acl = new_acl;
        }
    }

    pub fn get_by_market_id(&self, market_id: &U256) -> AskQueryResult {
        AskQueryResult {
            asks: self.market_id_index.get(market_id).cloned(),
        }
    }

    pub fn get_by_state(&self, state: AskState) -> AskQueryResult {
        AskQueryResult {
            asks: self.state_index.get(&state).cloned(),
        }
    }

    #[allow(unused)]
    pub fn get_by_ask_id(&self, ask_id: &U256) -> Option<&LocalAsk> {
        self.asks_by_id.get(ask_id)
    }

    pub fn get_ask_status(&self) -> LocalAskStatus {
        let created = self.get_by_state(AskState::Create).get_count();
        let unassigned = self.get_by_state(AskState::UnAssigned).get_count();
        let assigned = self.get_by_state(AskState::Assigned).get_count();
        let completed = self.get_by_state(AskState::Complete).get_count();
        let deadline_crossed = self.get_by_state(AskState::DeadlineCrossed).get_count();
        let invalid_secret = self.get_by_state(AskState::InvalidSecret).get_count();

        let local_ask_status = LocalAskStatus {
            created,
            unassigned,
            assigned,
            completed,
            deadline_crossed,
            invalid_secret,
        };

        local_ask_status
    }
}

pub struct MarketMetadataStore {
    market_by_id: HashMap<U256, MarketMetadata>,
}

impl MarketMetadataStore {
    pub fn new() -> Self {
        MarketMetadataStore {
            market_by_id: HashMap::new(),
        }
    }

    pub fn insert(&mut self, market: MarketMetadata) {
        self.market_by_id.insert(market.market_id, market.clone());
    }

    #[allow(unused)]
    pub fn remove_by_market_id(&mut self, market_id: &U256) {
        self.market_by_id.remove(market_id);
    }

    #[allow(unused)]
    pub fn get_market_by_market_id(&self, market_id: &U256) -> Option<&MarketMetadata> {
        self.market_by_id.get(market_id)
    }

    pub fn get_slashing_penalty_by_market_id(&self, market_id: &U256) -> Option<U256> {
        self.market_by_id
            .get(market_id)
            .map(|metadata| metadata.slashing_penalty)
    }

    #[allow(unused)]
    pub fn decode_market_verification_url_by_id(&self, market_id: &U256) -> Option<String> {
        let market_metadata = &self.market_by_id.get(market_id).unwrap().metadata;

        let metadata_str = market_metadata.to_string();
        let metadata_trim: Vec<_> = metadata_str.split('x').collect();
        let market_metadata_decoded = hex::decode(metadata_trim[1]).unwrap();
        let metadata_bytes: Bytes = market_metadata_decoded.into();

        let received_url = String::from_utf8(metadata_bytes.0.to_vec());
        match received_url {
            Ok(url) => {
                log::debug!("URL: {:?}", url.to_owned());
                Some(url.to_owned())
            }
            Err(_) => None,
        }
    }
}
