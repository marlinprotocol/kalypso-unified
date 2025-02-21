use ethers::prelude::*;
use ethers::types::U256;
use im::HashMap;
use im::HashSet;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::counters::median_counter::MedianCounter;

#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MarketSetupData {
    /// The display name of the zkApp.
    #[serde(alias = "zk_app_name")]
    pub zk_app_name: Option<String>,

    /// URL to the prover code repository or resource.
    #[serde(alias = "prover_code")]
    prover_code: Option<String>,

    /// URL to the verifier code repository or resource.
    #[serde(alias = "verifier_code")]
    verifier_code: Option<String>,

    /// URL to the prover Oyster image or related resource.
    #[serde(alias = "prover_oyster_image")]
    prover_oyster_image: Option<String>,

    /// URL to the input/output verifier, optional for private markets.
    #[serde(alias = "input_output_verifier_url")]
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
    #[serde(default)]
    categories: Vec<String>,

    /// Tags for better searchability and organization.
    #[serde(default)]
    tags: Vec<String>,

    /// Public hosted IVS if any. Useful especially for non-confidential market
    #[serde(default)]
    public_ivs: Vec<String>,

    /// Contact email for support or inquiries.
    contact_email: Option<String>,

    /// URL to the Terms of Service.
    #[serde(alias = "terms_of_service_url")]
    terms_of_service_url: Option<String>,

    /// URL to the Privacy Policy.
    #[serde(alias = "privacy_policy_url")]
    privacy_policy_url: Option<String>,

    #[serde(alias = "min_hardware", default)]
    pub min_hardware: MinHardware,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct MinHardware {
    pub instance_type: Option<String>,
    pub vcpus: Option<usize>,
    pub vgpus: Option<usize>,
    pub enclave_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarketMetadata {
    pub market_id: U256,
    pub verifier: Address,
    pub activation_block: U256,
    pub metadata: Bytes,
    pub prover_images: HashSet<H256>,
    pub ivs_images: HashSet<H256>,
}

impl MarketMetadata {
    /// Deserializes the raw metadata bytes into a `MarketSetupData` struct.
    ///
    /// If deserialization fails at any step, it returns a default `MarketSetupData` instance.
    pub fn deserialize_market_bytes(&self) -> MarketSetupData {
        // Convert bytes to a UTF-8 string. If conversion fails, use an empty JSON object.
        let json_str = std::str::from_utf8(&self.metadata).unwrap_or("{}");

        // Deserialize JSON string into `MarketSetupData` struct.
        let mut setup_data: MarketSetupData = serde_json::from_str(json_str).unwrap_or_default();

        // Update the `enclave_required` field.
        setup_data.min_hardware.enclave_required = !self.is_non_confidential_market();

        setup_data
    }
}
use crate::utility::deserialize_u256_map;
use crate::utility::serialize_u256_map;

#[derive(Serialize, Deserialize, Clone)]
pub struct MarketMetadataStore {
    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    market_by_id: HashMap<U256, MarketMetadata>,

    median_proof_time_tracker: MedianCounter<U256>, //<MarketId, Time in blocks>

    median_proof_cost_tracker: MedianCounter<U256>, //<MarketId, Cost in USDC>

    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    earnings: HashMap<U256, U256>, // market to usdc earning
}

impl Default for MarketMetadataStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketMetadata {
    pub fn is_non_confidential_market(&self) -> bool {
        self.prover_images.contains(
            &kalypso_helper::image_id_helpers::hashed_image_id_for_non_confidential_market(),
        )
    }
}

impl MarketMetadataStoreRead for MarketMetadataStore {
    fn count_markets(&self) -> usize {
        self.market_by_id.len()
    }

    fn get_all_markets(&self) -> Vec<MarketMetadata> {
        self.market_by_id.values().cloned().collect()
    }

    fn get_median_proof_time(&self) -> U256 {
        self.median_proof_time_tracker
            .median_all()
            .unwrap_or_else(U256::zero)
    }

    fn get_median_proof_time_market_wise(&self, market_id: &U256) -> U256 {
        self.median_proof_time_tracker
            .median_by_key(market_id)
            .unwrap_or_else(U256::zero)
    }

    fn get_median_proof_cost(&self) -> U256 {
        self.median_proof_cost_tracker
            .median_all()
            .unwrap_or_else(U256::zero)
    }

    fn get_median_proof_cost_market_wise(&self, market_id: &U256) -> U256 {
        self.median_proof_cost_tracker
            .median_by_key(market_id)
            .unwrap_or_else(U256::zero)
    }

    fn get_market_by_market_id(&self, market_id: &U256) -> Option<MarketMetadata> {
        // Retrieve market metadata without holding a lock for too long
        self.market_by_id.get(market_id).cloned()
    }

    fn get_earnings(&self, market_id: &U256) -> Option<U256> {
        // Safely access the earnings map
        self.earnings.get(market_id).cloned()
    }
}

impl MarketMetadataStoreWrite for MarketMetadataStore {
    fn new() -> Self {
        MarketMetadataStore {
            market_by_id: HashMap::new(),
            median_proof_cost_tracker: MedianCounter::new(),
            median_proof_time_tracker: MedianCounter::new(),
            earnings: HashMap::new(),
        }
    }

    fn insert(&mut self, market: MarketMetadata) {
        // Insert market metadata, minimizing lock time
        self.market_by_id.insert(market.market_id, market);
    }

    #[allow(unused)]
    fn remove_by_market_id(&mut self, market_id: &U256) {
        // Remove market metadata
        self.market_by_id.remove(market_id);
    }

    fn note_proof_submission_stats_for_valid_proof(
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

    fn note_proof_submission_stats_for_invalid_inputs(
        &mut self,
        market_id: &U256,
        proof_cost: U256,
    ) {
        // Minimize lock time by splitting into smaller steps
        self.median_proof_cost_tracker
            .insert(market_id.clone(), proof_cost);

        if let Some(existing_earning) = self.earnings.get_mut(market_id) {
            *existing_earning = existing_earning.saturating_add(proof_cost);
        } else {
            self.earnings.insert(*market_id, proof_cost);
        }
    }

    fn add_prover_image(&mut self, market_id: U256, image: H256) {
        if let Some(metadata) = self.market_by_id.get_mut(&market_id) {
            metadata.prover_images.insert(image);
        }
    }

    // Remove a prover image by market_id
    fn remove_prover_image(&mut self, market_id: U256, image: H256) {
        if let Some(metadata) = self.market_by_id.get_mut(&market_id) {
            metadata.prover_images.remove(&image);
        }
    }

    // Add an IVS image by market_id
    fn add_ivs_image(&mut self, market_id: U256, image: H256) {
        if let Some(metadata) = self.market_by_id.get_mut(&market_id) {
            metadata.ivs_images.insert(image);
        }
    }

    // Remove an IVS image by market_id
    fn remove_ivs_image(&mut self, market_id: U256, image: H256) {
        if let Some(metadata) = self.market_by_id.get_mut(&market_id) {
            metadata.ivs_images.remove(&image);
        }
    }

    fn update_marketmeta_bytes(&mut self, market_id: U256, metadata: Bytes) {
        if let Some(marketmetadata) = self.market_by_id.get_mut(&market_id) {
            marketmetadata.metadata = metadata
        }
    }
}

pub trait MarketMetadataStoreRead {
    /// Returns the total number of markets.
    fn count_markets(&self) -> usize;

    /// Returns a vector of all stored MarketMetadata.
    fn get_all_markets(&self) -> Vec<MarketMetadata>;

    /// Returns the median proof time across all markets.
    fn get_median_proof_time(&self) -> U256;

    /// Returns the median proof time for a specific market.
    fn get_median_proof_time_market_wise(&self, market_id: &U256) -> U256;

    /// Returns the median proof cost across all markets.
    fn get_median_proof_cost(&self) -> U256;

    /// Returns the median proof cost for a specific market.
    fn get_median_proof_cost_market_wise(&self, market_id: &U256) -> U256;

    /// Retrieves market metadata for a given market id.
    fn get_market_by_market_id(&self, market_id: &U256) -> Option<MarketMetadata>;

    /// Retrieves earnings for a given market id.
    fn get_earnings(&self, market_id: &U256) -> Option<U256>;
}

/// =============================
/// Write (Mutating) Operations Trait
/// =============================
pub trait MarketMetadataStoreWrite {
    /// Creates a new MarketMetadataStore instance.
    fn new() -> Self
    where
        Self: Sized;

    /// Inserts new market metadata.
    fn insert(&mut self, market: MarketMetadata);

    /// Removes market metadata by market id.
    fn remove_by_market_id(&mut self, market_id: &U256);

    /// Records proof submission statistics for a valid proof.
    fn note_proof_submission_stats_for_valid_proof(
        &mut self,
        market_id: &U256,
        proof_time: U256,
        proof_cost: U256,
    );

    /// Records proof submission statistics for invalid inputs.
    fn note_proof_submission_stats_for_invalid_inputs(
        &mut self,
        market_id: &U256,
        proof_cost: U256,
    );

    /// Adds a prover image to the market metadata.
    fn add_prover_image(&mut self, market_id: U256, image: H256);

    /// Removes a prover image from the market metadata.
    fn remove_prover_image(&mut self, market_id: U256, image: H256);

    /// Adds an IVS image to the market metadata.
    fn add_ivs_image(&mut self, market_id: U256, image: H256);

    /// Removes an IVS image from the market metadata.
    fn remove_ivs_image(&mut self, market_id: U256, image: H256);

    /// Updates the metadata bytes for a given market id.
    fn update_marketmeta_bytes(&mut self, market_id: U256, metadata: Bytes);
}
