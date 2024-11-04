use crate::{common_deps::CommonDeps, operations::Operation};
use async_trait::async_trait;
use ethers::{signers::Signer, types::Address};
use std::collections::HashMap;

pub struct Stake;

/// Implementation of the Operation trait for Stake
#[async_trait]
impl Operation for Stake {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        // Initialize dependencies
        let stake_for_generator = CommonDeps::stake_for_generator(&config)?;

        // Fetch generator registry data
        let data = stake_for_generator
            .generator_registry
            .generator_registry(stake_for_generator.staking_address)
            .call()
            .await
            .map_err(|_| "Failed making call to generator registry contract.")?;

        // Check if the operator is registered
        if data.0 == Address::zero() {
            return Err("Please 'Register' operator first".to_string());
        }

        // Fetch token balance
        let token_balance = stake_for_generator
            .staking_token
            .balance_of(stake_for_generator.private_key_signer.address())
            .call()
            .await
            .map_err(|_| "Failed making call to staking token contract.".to_string())?;

        // Check if the balance is sufficient
        if token_balance < stake_for_generator.stake {
            return Err("Insufficient staking token available".to_string());
        }

        let token_allowance = stake_for_generator
            .staking_token
            .allowance(
                stake_for_generator.private_key_signer.address(),
                stake_for_generator.generator_registry.address(),
            )
            .call()
            .await
            .map_err(|_| "Failed making call to staking token contract.".to_string())?;

        if token_allowance < stake_for_generator.stake {
            // Approve tokens for staking
            let token_approval_transaction = CommonDeps::send_and_confirm(
                stake_for_generator
                    .staking_token
                    .approve(
                        stake_for_generator.generator_registry.address(),
                        stake_for_generator.stake,
                    )
                    .send(),
            )
            .await
            .map_err(|e| format!("Token approval failed: {}", e))?;

            println!("Token Approval: {}", token_approval_transaction);
        }

        let staking_transaction = CommonDeps::send_and_confirm(
            stake_for_generator
                .generator_registry
                .stake(
                    stake_for_generator.staking_address,
                    stake_for_generator.stake,
                )
                .send(),
        )
        .await
        .map_err(|e| format!("Staking Transaction failed: {}", e))?;

        println!("Staking Transaction: {}", staking_transaction);

        Ok(())
    }
}
