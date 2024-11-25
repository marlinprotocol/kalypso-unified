use crate::{common_deps::CommonDeps, operations::Operation};
use async_trait::async_trait;
use ethers::{signers::Signer, types::Address};
use std::collections::HashMap;

pub struct JoinMarketplace;

#[async_trait]
impl Operation for JoinMarketplace {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let generator_join_market = CommonDeps::generator_join_market_instance(&config)?;

        match generator_join_market
            .generator_registry
            .generator_registry(generator_join_market.private_key_signer.address())
            .call()
            .await
        {
            Ok(data) => {
                if data.0.eq(&Address::zero()) {
                    return Err(format!("Please 'Register' operator first"));
                } else {
                    let info_per_market = generator_join_market
                        .generator_registry
                        .generator_info_per_market(
                            generator_join_market.private_key_signer.address(),
                            generator_join_market.market_id,
                        )
                        .call()
                        .await
                        .map_err(|e| {
                            format!("Failed making call to generator registry contract {}", e)
                        })?;

                    if info_per_market.0 != 0 {
                        return Err(format!("Generator has already joined the market."));
                    }

                    let tx_hash = CommonDeps::send_and_confirm(
                        generator_join_market
                            .generator_registry
                            .join_marketplace(
                                generator_join_market.market_id,
                                generator_join_market.compute_per_request_required,
                                generator_join_market.proof_generation_cost,
                                generator_join_market.proposed_time,
                                false,
                                vec![].into(),
                                vec![].into(),
                            )
                            .send(),
                    )
                    .await?;

                    // Print the transaction hash
                    println!("{}", tx_hash);
                }
            }
            Err(_) => {
                return Err(format!(
                    "Failed making call to generator registry contract."
                ));
            }
        }
        Ok(())
    }
}
