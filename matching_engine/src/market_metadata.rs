use ethers::core::types::U256;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::counters::median_counter::MedianCounter;

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

pub struct MarketMetadataStore {
    market_by_id: HashMap<U256, MarketMetadata>,
    median_proof_time_tracker: MedianCounter<U256, U256>, //<MarketId, Time in blocks>
    median_proof_cost_tracker: MedianCounter<U256, U256>, //<MarketId, Cost in USDC>
    earnings: HashMap<U256, U256>,                        // market to usdc earning
}

impl Default for MarketMetadataStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketMetadataStore {
    pub fn count_markets(&self) -> usize {
        self.market_by_id.iter().count()
    }

    pub fn get_all_markets(&self) -> Vec<MarketMetadata> {
        self.market_by_id
            .iter()
            .map(|(_, market_metadata)| market_metadata.clone()) // Extract and clone the MarketMetadata
            .collect() // Collect into a Vec
    }
}

impl MarketMetadataStore {
    pub fn note_proof_submission_stats(
        &mut self,
        market_id: &U256,
        proof_time: U256,
        proof_cost: U256,
    ) {
        self.median_proof_cost_tracker
            .insert(market_id.clone(), proof_cost);
        self.median_proof_time_tracker
            .insert(market_id.clone(), proof_time);

        self.earnings
            .entry(*market_id)
            .and_modify(|e| *e = e.saturating_add(proof_cost))
            .or_insert(proof_cost);
    }

    pub fn get_median_proof_time(&self) -> U256 {
        self.median_proof_time_tracker
            .median_all()
            .unwrap_or_else(|| U256::zero())
    }

    pub fn get_median_proof_time_market_wise(&self, market_id: &U256) -> U256 {
        self.median_proof_time_tracker
            .median_by_key(market_id)
            .unwrap_or_else(|| U256::zero())
    }

    pub fn get_median_proof_cost(&self) -> U256 {
        self.median_proof_cost_tracker
            .median_all()
            .unwrap_or_else(|| U256::zero())
    }

    pub fn get_median_proof_cost_market_wise(&self, market_id: &U256) -> U256 {
        self.median_proof_cost_tracker
            .median_by_key(market_id)
            .unwrap_or_else(|| U256::zero())
    }
}

impl MarketMetadataStore {
    pub fn new() -> Self {
        MarketMetadataStore {
            market_by_id: HashMap::new(),
            median_proof_cost_tracker: MedianCounter::new(),
            median_proof_time_tracker: MedianCounter::new(),
            earnings: HashMap::new(),
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

impl MarketMetadataStore {
    pub fn get_earnings(&self, market_id: &U256) -> Option<U256> {
        self.earnings.get(market_id).cloned()
    }
}
