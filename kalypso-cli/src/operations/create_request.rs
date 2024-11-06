use std::collections::HashMap;

use async_trait::async_trait;
use ethers::{signers::Signer, types::U256};

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct NonConfidentialRequest;

#[async_trait]
impl Operation for NonConfidentialRequest {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let non_confidential_request_info = CommonDeps::non_confidential_request_info(&config)?;

        // let market_data = non_confidential_request_info.proof_marketplace.market_data(non_confidential_request_info.market_id).call().await.map_err(|_| {
        //     "Failed making call to proof marketplace contract.".to_string()
        // })?;

        let token_balance = non_confidential_request_info
            .payment_token
            .balance_of(non_confidential_request_info.private_key_signer.address())
            .call()
            .await
            .map_err(|_| "Failed making call to payment token contract.".to_string())?;

        if non_confidential_request_info.max_proof_generation_cost > token_balance {
            return Err("Insufficient payment token available".to_string());
        }

        let token_allowance = non_confidential_request_info
            .payment_token
            .allowance(
                non_confidential_request_info.private_key_signer.address(),
                non_confidential_request_info.proof_marketplace.address(),
            )
            .call()
            .await
            .map_err(|_| "Failed making call to payment token contract.".to_string())?;

        if token_allowance < non_confidential_request_info.max_proof_generation_cost {
            let token_approval_transaction = CommonDeps::send_and_confirm(
                non_confidential_request_info
                    .payment_token
                    .approve(
                        non_confidential_request_info.proof_marketplace.address(),
                        non_confidential_request_info.max_proof_generation_cost,
                    )
                    .send(),
            )
            .await
            .map_err(|e| format!("Token approval failed: {}", e))?;

            println!("Token Approval: {}", token_approval_transaction);
        }

        let proof_request_transaction = CommonDeps::send_and_confirm(
            non_confidential_request_info
                .proof_marketplace
                .create_ask(
                    bindings::proof_marketplace::Ask {
                        market_id: non_confidential_request_info.market_id,
                        reward: non_confidential_request_info.max_proof_generation_cost,
                        expiry: (10000000000000 as u64).into(),
                        time_taken_for_proof_generation: non_confidential_request_info
                            .max_proof_generation_time,
                        deadline: U256::zero(),
                        refund_address: non_confidential_request_info.private_key_signer.address(),
                        prover_data: non_confidential_request_info.inputs,
                    },
                    0.into(),      // `secret_type` argument
                    vec![].into(), // `private_inputs` argument
                    vec![].into(), // `acl` argument
                )
                .send(),
        )
        .await?;

        // Print the transaction hash
        println!("proof request transaction: {}", proof_request_transaction);

        Ok(())
    }
}
