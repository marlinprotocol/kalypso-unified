use std::{collections::HashMap, sync::Once};

use async_trait::async_trait;
use ethers::{signers::Signer, types::Address};
use serde::{Deserialize, Serialize};

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct RequestNativeStakeWithdrawal;

#[async_trait]
impl Operation for RequestNativeStakeWithdrawal {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        unimplemented!()
    }
}

pub struct ReadWithdrawalRequestIds;

#[async_trait]
impl Operation for ReadWithdrawalRequestIds {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let read_stake_data_info = CommonDeps::read_stake_data_info(&config)?;
        let withdrawal_requests = read_staking_data(
            &read_stake_data_info.operator_address,
            read_stake_data_info.indexer_url,
        )
        .await
        .map_err(|e| format!("Unable to read stake data from indexer: {}", e))?;

        static INIT: Once = Once::new();
        INIT.call_once(|| {
            println!(
                "\n\nWithdrawal Requests for Operator {:?}\n\n",
                read_stake_data_info.operator_address
            );
            println!(
                "{:<42} | {:<42} | {:>42} | {:<5}",
                "Account", "Amount", "Token", "Index"
            );
            println!("{:-<42}-+-{:-<42}-+-{:-<42}-+-{:-<5}", "", "", "", "");
        });

        for withdrawal in withdrawal_requests {
            println!(
                "{:<42} | {:<42} | {:>42} | {:<5}",
                withdrawal.account, withdrawal.amount, withdrawal.token, withdrawal.index
            );
        }

        println!();
        Ok(())
    }
}

pub struct ProcessWithdrawalRequests;

#[async_trait]
impl Operation for ProcessWithdrawalRequests {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        unimplemented!()
    }
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WithdrawRequest {
    account: String,
    token: String,
    amount: String,
    index: String,
}

// Define the structure of the entire API response
#[derive(Deserialize, Debug)]
struct ApiResponse {
    withdrawal_requests: Vec<WithdrawRequest>,
    // Include other fields if necessary
}

use reqwest::header::{HeaderMap, CONTENT_TYPE};
use std::error::Error;

async fn read_staking_data(
    operator_address: &Address,
    indexer_url: String,
) -> Result<Vec<WithdrawRequest>, Box<dyn Error>> {
    // Initialize the HTTP client with a timeout (optional but recommended)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Construct the full URL
    let full_url = format!("{}/ui/generator/{:?}", indexer_url, operator_address);

    // Prepare the headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    // Make the GET request
    let response = client.get(&full_url).headers(headers).send().await?;

    // Check if the response status is success
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    // Parse the JSON response
    let api_response: ApiResponse = response.json().await?;

    Ok(api_response.withdrawal_requests)
}
