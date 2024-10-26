use ethers::core::types::U256;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    counters::median_counter::MedianCounter,
    utility::{AddressTokenPair, TokenTracker},
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarketSetupData {
    /// The display name of the zkApp.
    zk_app_name: Option<String>,

    /// URL to the prover code repository or resource.
    prover_code: Option<String>,

    /// URL to the verifier code repository or resource.
    verifier_code: Option<String>,

    /// URL to the prover Oyster image or related resource.
    prover_oyster_image: Option<String>,

    /// URL to the input/output verifier, optional for private markets.
    input_output_verifier_url: Option<String>,

    /// A brief description of the zkApp.
    description: Option<String>,

    /// The version of the zkApp.
    version: Option<String>,

    /// URL to the official website of the zkApp.
    website: Option<String>,

    /// Twitter handle or URL related to the zkApp.
    twitter: Option<String>,

    /// Discord invite link or server URL for the zkApp community.
    discord: Option<String>,

    /// GitHub repository URL for the zkApp.
    github: Option<String>,

    /// License information for the zkApp (e.g., MIT, GPL).
    license: Option<String>,

    /// Categories that classify the zkApp.
    categories: Vec<String>,

    /// Tags for better searchability and organization.
    tags: Vec<String>,

    /// Contact email for support or inquiries.
    contact_email: Option<String>,

    /// URL to the Terms of Service.
    terms_of_service_url: Option<String>,

    /// URL to the Privacy Policy.
    privacy_policy_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct MarketMetadata {
    pub market_id: U256,
    pub verifier: Address,
    pub prover_image_id: [u8; 32],
    pub slashing_penalty: TokenTracker,
    pub activation_block: U256,
    pub ivs_image_id: [u8; 32],
    pub metadata: Bytes,
}

impl MarketMetadata {
    pub fn deserialize_market_bytes(&self) -> Option<MarketSetupData> {
        // Convert bytes to a UTF-8 string
        let json_str = std::str::from_utf8(&self.metadata).ok()?;

        // Deserialize JSON string into MarketSetupData struct
        let data: MarketSetupData = serde_json::from_str(json_str).ok()?;

        Some(data)
    }
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
        self.market_by_id.len()
    }

    pub fn get_all_markets(&self) -> Vec<MarketMetadata> {
        self.market_by_id.values().cloned().collect()
    }
}

impl MarketMetadataStore {
    pub fn note_proof_submission_stats(
        &mut self,
        market_id: &U256,
        proof_time: U256,
        proof_cost: U256,
    ) {
        // Minimize lock time by splitting into smaller steps
        self.median_proof_cost_tracker
            .insert(market_id.clone(), proof_cost);

        self.median_proof_time_tracker
            .insert(market_id.clone(), proof_time);

        if let Some(existing_earning) = self.earnings.get_mut(market_id) {
            *existing_earning = existing_earning.saturating_add(proof_cost);
        } else {
            self.earnings.insert(*market_id, proof_cost);
        }
    }

    pub fn get_median_proof_time(&self) -> U256 {
        self.median_proof_time_tracker
            .median_all()
            .unwrap_or_else(U256::zero)
    }

    pub fn get_median_proof_time_market_wise(&self, market_id: &U256) -> U256 {
        self.median_proof_time_tracker
            .median_by_key(market_id)
            .unwrap_or_else(U256::zero)
    }

    pub fn get_median_proof_cost(&self) -> U256 {
        self.median_proof_cost_tracker
            .median_all()
            .unwrap_or_else(U256::zero)
    }

    pub fn get_median_proof_cost_market_wise(&self, market_id: &U256) -> U256 {
        self.median_proof_cost_tracker
            .median_by_key(market_id)
            .unwrap_or_else(U256::zero)
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
        // Insert market metadata, minimizing lock time
        self.market_by_id.insert(market.market_id, market);
    }

    #[allow(unused)]
    pub fn remove_by_market_id(&mut self, market_id: &U256) {
        // Remove market metadata
        self.market_by_id.remove(market_id);
    }

    pub fn get_market_by_market_id(&self, market_id: &U256) -> Option<MarketMetadata> {
        // Retrieve market metadata without holding a lock for too long
        self.market_by_id.get(market_id).cloned()
    }

    pub fn get_slashing_penalty_by_market_id(
        &self,
        market_id: &U256,
    ) -> Option<Vec<AddressTokenPair>> {
        // Safely access market metadata to retrieve the slashing penalty
        self.market_by_id
            .get(market_id)
            .map(|metadata| metadata.slashing_penalty.to_address_token_pair())
    }

    #[allow(unused)]
    pub fn decode_market_verification_url_by_id(&self, market_id: &U256) -> Option<String> {
        let market_metadata = self.market_by_id.get(market_id)?;

        let metadata_str = market_metadata.metadata.to_string();
        let metadata_trim: Vec<_> = metadata_str.split('x').collect();
        let market_metadata_decoded = hex::decode(metadata_trim[1]).ok()?;
        let metadata_bytes: Bytes = market_metadata_decoded.into();

        let received_url = String::from_utf8(metadata_bytes.0.to_vec()).ok();
        if let Some(url) = received_url {
            log::debug!("URL: {:?}", url);
            Some(url)
        } else {
            None
        }
    }
}

impl MarketMetadataStore {
    pub fn get_earnings(&self, market_id: &U256) -> Option<U256> {
        // Safely access the earnings map
        self.earnings.get(market_id).cloned()
    }
}
