use std::collections::HashMap;

use async_trait::async_trait;
use ethers::signers::Signer;

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct NativeStaking;

#[async_trait]
impl Operation for NativeStaking {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let native_stake_info = CommonDeps::native_staking_info(&config)?;

        if native_stake_info.operator_address != native_stake_info.private_key_signer.address() {
            return Err("Currently native staking is supported to self address".to_string());
        }

        let token_balance = native_stake_info
            .staking_token
            .balance_of(native_stake_info.private_key_signer.address())
            .call()
            .await
            .map_err(|e| format!("Failed making call to staking token contract {}", e))?;

        if token_balance < native_stake_info.staking_amount {
            return Err("Insufficient staking token available".to_string());
        }

        let token_allowance = native_stake_info
            .staking_token
            .allowance(
                native_stake_info.private_key_signer.address(),
                native_stake_info.native_staking.address(),
            )
            .call()
            .await
            .map_err(|e| format!("Failed making call to staking token contract {}", e))?;

        if token_allowance < native_stake_info.staking_amount {
            let token_approval_transaction = CommonDeps::send_and_confirm(
                native_stake_info
                    .staking_token
                    .approve(
                        native_stake_info.native_staking.address(),
                        native_stake_info.staking_amount,
                    )
                    .send(),
            )
            .await
            .map_err(|e| format!("Token approval failed: {}", e))?;

            println!("Token Approval: {}", token_approval_transaction);
        }

        let native_stake_tx = CommonDeps::send_and_confirm(
            native_stake_info
                .native_staking
                .stake(
                    native_stake_info.staking_token.address(),
                    native_stake_info.operator_address,
                    native_stake_info.staking_amount,
                )
                .send(),
        )
        .await
        .map_err(|e| format!("Native Staking Transaction failed: {}", e))?;

        println!("Native Staking Transaction: {}", native_stake_tx);

        Ok(())
    }
}

#[async_trait]
impl Operation for NativeStaking {
    async fn request_withdrawal(&self, config: HashMap<String, String>) -> Result<(), String> {
        let native_stake_info = CommonDeps::native_staking_info(&config)?;

        // Step 1: Call the requestStakeWithdrawal function
        let request_tx = CommonDeps::send_and_confirm(
            native_stake_info
                .native_staking
                .request_stake_withdrawal(native_stake_info.operator_address)
                .send(),
        )
        .await
        .map_err(|e| format!("Request Stake Withdrawal failed: {}", e))?;

        println!("Stake Withdrawal Requested: {}", request_tx);

        // Step 2: Verify the indexer update
        let indexer_url = config
            .get("indexer_url")
            .ok_or("Missing indexer_url in config")?;

        // Query the indexer for withdrawal requests
        let response = reqwest::get(indexer_url)
            .await
            .map_err(|e| format!("Failed to fetch from indexer: {}", e))?
            .text()
            .await
            .map_err(|e| format!("Failed to parse indexer response: {}", e))?;

        let parsed_response: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| format!("Invalid JSON response: {}", e))?;

        // Validate that the operator's withdrawal request is included
        if let Some(requests) = parsed_response["withdrawal_requests"].as_array() {
            let operator_address = native_stake_info.operator_address.to_string();

            if requests.iter().any(|req| req["operator"].as_str() == Some(&operator_address)) {
                println!("Indexer updated with the withdrawal request for operator: {}", operator_address);
                Ok(())
            } else {
                Err("Indexer did not update with the new withdrawal request.".to_string())
            }
        } else {
            Err("No withdrawal requests found in the indexer.".to_string())
        }
    }
}


#[async_trait]
impl Operation for NativeStaking {
    async fn read_withdrawal_ids(&self, config: HashMap<String, String>) -> Result<(), String> {
        let indexer_url = config
            .get("indexer_url")
            .ok_or("Missing indexer_url in config")?;

        // Fetch withdrawal requests
        let response = reqwest::get(indexer_url)
            .await
            .map_err(|e| format!("Failed to fetch from indexer: {}", e))?
            .text()
            .await
            .map_err(|e| format!("Failed to parse indexer response: {}", e))?;

        let parsed_response: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| format!("Invalid JSON response: {}", e))?;

        // Extract withdrawal IDs
        if let Some(requests) = parsed_response["withdrawal_requests"].as_array() {
            for request in requests {
                if let Some(id) = request["id"].as_str() {
                    println!("Withdrawal ID: {}", id);
                }
            }
        } else {
            return Err("No withdrawal requests found".to_string());
        }

        Ok(())
    }
}

#[async_trait]
impl Operation for NativeStaking {
    async fn confirm_withdrawal(&self, config: HashMap<String, String>) -> Result<(), String> {
        let native_stake_info = CommonDeps::native_staking_info(&config)?;

        get_config_ref!(config, "withdrawal_id", withdrawal_id);
        let withdrawal_id = withdrawal_id
            .parse::<U256>()
            .map_err(|e| format!("Invalid withdrawal ID: {}", e))?;

        // Call the withdrawStake function
        let confirm_tx = CommonDeps::send_and_confirm(
            native_stake_info
                .native_staking
                .withdraw_stake(withdrawal_id)
                .send(),
        )
        .await
        .map_err(|e| format!("Confirm Withdrawal failed: {}", e))?;

        println!("Withdrawal Confirmed: {}", confirm_tx);

        Ok(())
    }
}