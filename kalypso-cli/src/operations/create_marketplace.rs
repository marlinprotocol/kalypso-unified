use crate::{common_deps::CommonDeps, operations::Operation};
use async_trait::async_trait;
use ethers::signers::Signer;
use std::collections::HashMap;

pub struct CreateMarketplace;

#[async_trait]
impl Operation for CreateMarketplace {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let market_create_info = CommonDeps::market_create_info(&config)?;

        let token_balance = market_create_info
            .payment_token
            .balance_of(market_create_info.private_key_signer.address())
            .call()
            .await
            .map_err(|e| format!("Failed making call to payment token contract {}", e))?;

        let market_creation_cost = {
            let result = market_create_info
                .proof_marketplace
                .market_creation_cost()
                .call()
                .await
                .map_err(|e| format!("Failed making call to proof marketplace contract {}", e))?;

            result + 1
        };

        if market_creation_cost > token_balance {
            return Err("Insufficient payment token available".to_string());
        }

        let token_allowance = market_create_info
            .payment_token
            .allowance(
                market_create_info.private_key_signer.address(),
                market_create_info.proof_marketplace.address(),
            )
            .call()
            .await
            .map_err(|e| format!("Failed making call to payment token contract {}", e))?;

        if token_allowance < market_creation_cost {
            let token_approval_transaction = CommonDeps::send_and_confirm(
                market_create_info
                    .payment_token
                    .approve(
                        market_create_info.proof_marketplace.address(),
                        market_creation_cost,
                    )
                    .send(),
            )
            .await
            .map_err(|e| format!("Token approval failed: {}", e))?;

            println!("Token Approval: {}", token_approval_transaction);
        }

        let market_creation_transaction = CommonDeps::send_and_confirm(
            market_create_info
                .proof_marketplace
                .create_market(
                    vec![12, 23].into(),
                    market_create_info.verifier_wrapper,
                    market_create_info.prover_pcrs.into(),
                    market_create_info.ivs_pcrs.into(),
                )
                .send(),
        )
        .await
        .map_err(|e| format!("Market Creation Transaction failed: {}", e))?;

        println!(
            "Market Creation Transaction: {}",
            market_creation_transaction
        );

        Ok(())
    }
}
