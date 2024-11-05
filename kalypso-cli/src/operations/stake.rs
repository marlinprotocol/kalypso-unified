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
            .map_err(|_| "Failed making call to staking token contract.".to_string())?;

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
            .map_err(|_| "Failed making call to staking token contract.".to_string())?;

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
