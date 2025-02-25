use ethers::core::types::Address;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utility::TokenTracker;

use super::generator_state::GeneratorState;

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct Generator {
    pub address: Address,
    pub reward_address: Address,
    pub total_native_stake: TokenTracker,
    pub total_symbiotic_stake: TokenTracker,
    pub sum_of_compute_allocations: U256,
    pub compute_consumed: U256,
    pub native_stake_locked: TokenTracker,
    pub symbiotic_stake_locked: TokenTracker,
    pub active_market_places: U256,
    pub declared_compute: U256,
    pub intended_stake_util: U256,
    pub intended_compute_util: U256,
    pub generator_data: Bytes,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorMeta {
    /// The display name of the generator or application.
    #[serde(alias = "display_name")]
    pub display_name: Option<String>,

    /// A brief description of the generator or application.
    #[serde(alias = "display_description")]
    pub display_description: Option<String>,

    /// The official website URL.
    pub website: Option<String>,

    /// Twitter handle or URL.
    pub twitter: Option<String>,

    /// Discord invite link or server URL.
    pub discord: Option<String>,

    /// URL to the logo image, enhancing brand recognition.
    #[serde(alias = "logo_url")]
    pub logo_url: Option<String>,

    /// URL to a banner image for promotional purposes.
    #[serde(alias = "banner_url")]
    pub banner_url: Option<String>,

    /// Contact email for support or inquiries.
    #[serde(alias = "contact_email")]
    pub contact_email: Option<String>,

    /// GitHub repository URL for open-source projects.
    pub github: Option<String>,

    /// LinkedIn profile or company page URL.
    pub linkedin: Option<String>,

    /// Medium blog URL for updates and articles.
    pub medium: Option<String>,

    /// Reddit community URL.
    pub reddit: Option<String>,

    /// YouTube channel URL for tutorials and updates.
    pub youtube: Option<String>,

    /// Instagram profile URL for visual content.
    pub instagram: Option<String>,

    /// Repository URL, useful if different from GitHub.
    #[serde(alias = "repo_url")]
    pub repo_url: Option<String>,

    /// Current version of the generator or application.
    #[serde(default)]
    pub version: Option<String>,

    /// Categories that classify the generator or application.
    #[serde(default)]
    pub categories: Vec<String>,

    /// Tags for better searchability and organization.
    pub tags: Vec<String>,

    /// License information, e.g., MIT, GPL.
    pub license: Option<String>,

    /// URL to the Terms of Service.
    #[serde(alias = "terms_of_service_url")]
    pub terms_of_service_url: Option<String>,

    /// URL to the Privacy Policy.
    #[serde(alias = "privacy_policy_url")]
    pub privacy_policy_url: Option<String>,
}

impl Generator {
    pub fn deserialize_generator_bytes(&self) -> GeneratorMeta {
        // Convert bytes to a UTF-8 string
        let json_str = std::str::from_utf8(&self.generator_data).unwrap_or("{}");

        // Deserialize JSON string into MarketSetupData struct
        serde_json::from_str(json_str).unwrap_or_default()
    }
}
