use std::collections::HashMap;

use async_trait::async_trait;
use ethers::types::U256;

use crate::common_deps::CommonDeps;
use kalypso_helper::send_with_optional_gas;

use super::Operation;

pub struct ReadRewardsInfo;

#[async_trait]
impl Operation for ReadRewardsInfo {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let read_rewards_info = CommonDeps::read_rewards_info(&config)?;

        let available_rewards = read_rewards_info
            .proof_marketplace
            .prover_claimable_fee_reward(read_rewards_info.operator)
            .call()
            .await
            .map_err(|e| {
                format!(
                    "{}. {}",
                    "Failed making call to proof marketplace contract".to_string(),
                    e
                )
            })?;

        println!("Available Rewards: {}", available_rewards);

        Ok(())
    }
}

pub struct ClaimRewardsInfo;

#[async_trait]
impl Operation for ClaimRewardsInfo {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let claim_rewards_info = CommonDeps::claim_rewards_info(&config)?;

        let available_rewards = claim_rewards_info
            .proof_marketplace
            .prover_claimable_fee_reward(claim_rewards_info.reward_address)
            .call()
            .await
            .map_err(|e| {
                format!(
                    "{}. {}",
                    "Failed making call to proof marketplace contract".to_string(),
                    e
                )
            })?;

        if available_rewards.eq(&U256::zero()) {
            return Err("No Rewards available to claim".to_string());
        }

        println!();
        println!("Trying to claim {} reward tokens", available_rewards);

        let claim_reward_transaction = send_with_optional_gas!(claim_rewards_info
            .proof_marketplace
            .claim_prover_fee_reward(claim_rewards_info.reward_address))
        .map_err(|e| format!("Claim Reward Transaction failed: {}", e))?;

        println!("Claim Reward Transaction: {}", claim_reward_transaction);

        Ok(())
    }
}
