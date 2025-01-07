// src/operations/complete_registration.rs

use crate::common_deps::CommonDeps;
use crate::operations::Operation;
use crate::send_with_optional_gas;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct RequestMarketPlaceExit;

#[async_trait]
impl Operation for RequestMarketPlaceExit {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        // Initialize common dependencies
        let request_exit_info = CommonDeps::marketplace_exit_info(&config)?;

        let tx_hash = send_with_optional_gas!(request_exit_info
            .generator_registry
            .request_for_exit_marketplace(request_exit_info.market_id))?;

        // Print the transaction hash
        println!("Request Marketplace Exit tx: {}", tx_hash);
        Ok(())
    }
}

pub struct LeaveMarketPlace;

#[async_trait]
impl Operation for LeaveMarketPlace {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        // Initialize common dependencies
        let request_exit_info = CommonDeps::marketplace_exit_info(&config)?;

        let tx_hash = send_with_optional_gas!(request_exit_info
            .generator_registry
            .leave_marketplace(request_exit_info.market_id))?;

        // Print the transaction hash
        println!("Marketplace Exit tx: {}", tx_hash);
        Ok(())
    }
}
