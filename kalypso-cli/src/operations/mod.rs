// src/operations/mod.rs

pub mod complete_registration;
pub mod compute_pcrs;
pub mod create_marketplace;
pub mod join_marketplace;
pub mod leave_or_request_leave_marketplace;
pub mod stake;

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
        "Stake" => Some(Box::new(stake::Stake)),
        "Request To Leave Marketplace" => Some(Box::new(
            leave_or_request_leave_marketplace::RequestMarketPlaceExit,
        )),
        "Leave Marketplace" => Some(Box::new(
            leave_or_request_leave_marketplace::LeaveMarketPlace,
        )),
        "Create Marketplace" => Some(Box::new(create_marketplace::CreateMarketplace)),
        "Compute PCRs" => Some(Box::new(compute_pcrs::ComputePcrs)),
        _ => unimplemented!(),
    }
}
