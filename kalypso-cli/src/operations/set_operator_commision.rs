use async_trait::async_trait;
use ethers::prelude::*;
use ethers::{contract::abigen, types::U256};
use std::{collections::HashMap, sync::Arc};

use crate::common_deps::CommonDeps;
use crate::send_with_optional_gas;

use super::Operation;

pub struct SetOperatorCommision;

#[async_trait]
impl Operation for SetOperatorCommision {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let set_operator_commission_info = CommonDeps::set_operator_commission_info(&config)?;

        if set_operator_commission_info.operator_commission
            > U256::from_dec_str("1000000000000000000").unwrap()
        {
            return Err("Operator Commission can't be more than 1000000000000000000".to_string());
        }

        // using manual abigen here because old contract bindings don't have differnt signature

        abigen!(
            SetOperatorRewardContract,
            r#"
            [
                {
                    "inputs": [
                    {
                        "internalType": "uint256",
                        "name": "_rewardShare",
                        "type": "uint256"
                    }
                    ],
                    "name": "setOperatorRewardShare",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                }
            ]
        "#
        );

        // Initialize the provider
        let provider_http = Provider::<Http>::try_from(set_operator_commission_info.rpc_url)
            .map_err(|e| format!("Invalid RPC URL: {}", e))?;

        // Initialize the SignerMiddleware with the provider and signer
        let client = SignerMiddleware::new(
            provider_http.clone(),
            set_operator_commission_info.private_key_signer.clone(),
        );
        let client_arc = Arc::new(client);

        // this is same was proof market place
        let set_reward_share_instance = SetOperatorRewardContract::new(
            set_operator_commission_info.proof_marketplace.address(),
            client_arc.clone(),
        );

        let set_commission_hash = send_with_optional_gas!(set_reward_share_instance
            .set_operator_reward_share(set_operator_commission_info.operator_commission))
        .map_err(|e| format!("Failed making call to proof marketplace {}", e))?;

        println!(
            "Set Reward Commission Transaction Hash: {}",
            set_commission_hash
        );

        Ok(())
    }
}
