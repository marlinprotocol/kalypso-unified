// src/operations/mod.rs

pub mod claim_rewards;
pub mod complete_registration;
pub mod compute_pcrs;
pub mod create_marketplace;
pub mod create_request;
pub mod generator_config;
pub mod join_marketplace;
pub mod leave_or_request_leave_marketplace;
pub mod programs;
pub mod read_proof;
pub mod request;
pub mod stake;
pub mod symbiotic_opt_in;
pub mod update_encryption_key;
pub mod update_generator_meta;
pub mod update_market_metadata;
pub mod whitelist;

// ... Add other operation modules here

use async_trait::async_trait;
use std::collections::HashMap;

/// Trait that all operations must implement
#[async_trait]
pub trait Operation: Send + Sync {
    /// Executes the operation with the provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - A HashMap containing prompt values required for the operation
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the operation succeeds
    /// * `Err(String)` with an error message if the operation fails
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String>;
}

/// Factory function to retrieve an operation handler based on its name
///
/// # Arguments
///
/// * `name` - The name of the operation as defined in `config.rs`
///
/// # Returns
///
/// * `Some(Box<dyn Operation>)` if the operation is implemented
/// * `None` if the operation is not found
pub fn get_operation(name: &str) -> Option<Box<dyn Operation>> {
    match name {
        "Register" => Some(Box::new(complete_registration::CompleteRegistration)),
        "Join Marketplace" => Some(Box::new(join_marketplace::JoinMarketplace)),
        "Request To Leave Marketplace" => Some(Box::new(
            leave_or_request_leave_marketplace::RequestMarketPlaceExit,
        )),
        "Leave Marketplace" => Some(Box::new(
            leave_or_request_leave_marketplace::LeaveMarketPlace,
        )),
        "Create Marketplace" => Some(Box::new(create_marketplace::CreateMarketplace)),
        "Read Attestation" => Some(Box::new(compute_pcrs::ReadAttestation)),
        "Compute PCRs" => Some(Box::new(compute_pcrs::ComputePcrs)),
        "Non-Confidential Market PCRS" => Some(Box::new(compute_pcrs::NonConfidentialMarketPcrs)),
        "Claim Rewards" => Some(Box::new(claim_rewards::ClaimRewardsInfo)),
        "Native Stake" => Some(Box::new(stake::NativeStaking)),
        "Create Proof Request (non confidential market)" => {
            Some(Box::new(create_request::NonConfidentialRequest))
        }
        "Create Proof Request (confidential market)" => {
            Some(Box::new(create_request::ConfidentialRequest))
        }
        "Discard Request" => Some(Box::new(request::DiscardRequest)),
        "Whitelist Prover Image" => Some(Box::new(whitelist::WhitelistProverImage)),
        "Whitelist IVS Image" => Some(Box::new(whitelist::WhitelistVerificationImage)),
        "Update Encryption Key" => Some(Box::new(update_encryption_key::UpdateEncryptionKey)),
        "Add IVS Key" => Some(Box::new(update_encryption_key::AddIvsKey)),
        "Read Proof Bytes" => Some(Box::new(read_proof::ReadProof)),
        "Request Symbiotic Stake" => Some(Box::new(symbiotic_opt_in::SymbioticOptIn)),
        "Load Generator Config" => Some(Box::new(generator_config::GeneratorConfig)),
        "Start Enclave Program" => Some(Box::new(programs::StartProgam)),
        "Stop Enclave Program" => Some(Box::new(programs::StopProgram)),
        "Test Enclave Connection" => Some(Box::new(programs::TestConnection)),
        "Benchmark Prover" => Some(Box::new(programs::Benchmark)),
        "Symbiotic Operator Register" => {
            Some(Box::new(symbiotic_opt_in::SymbioticOperatorRegister))
        }
        "Update Market Metadata" => Some(Box::new(update_market_metadata::UpdateMarketMetadata)),
        "Update Generator Metadata" => Some(Box::new(update_generator_meta::UpdateGeneratorMeta)),
        _ => unimplemented!(),
    }
}
