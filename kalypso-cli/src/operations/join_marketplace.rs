use crate::{common_deps::CommonDeps, operations::Operation};
use async_trait::async_trait;
use ethers::{
    signers::Signer,
    types::{Address, H256, U256},
};
use kalypso_helper::send_with_optional_gas;
use kalypso_helper::try_read_contract_error;
use std::collections::HashMap;

pub struct JoinMarketplace;

#[async_trait]
impl Operation for JoinMarketplace {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let generator_join_market = CommonDeps::generator_join_market_instance(&config)?;

        if generator_join_market.commission > U256::from_dec_str("1000000000000000000").unwrap() {
            return Err("Operator Commission can't be more than 1000000000000000000".to_string());
        }

        match generator_join_market
            .generator_registry
            .prover_registry(generator_join_market.private_key_signer.address())
            .call()
            .await
        {
            Ok(data) => {
                if data.0.eq(&Address::zero()) {
                    return Err(format!("Please 'Register' operator first"));
                } else {
                    let info_per_market = generator_join_market
                        .generator_registry
                        .prover_info_per_market(
                            generator_join_market.private_key_signer.address(),
                            generator_join_market.market_id,
                        )
                        .call()
                        .await
                        .map_err(|e| format!("Failed calling generator registry contract {}", e))?;

                    if info_per_market.0 != 0 {
                        return Err(format!("Generator has already joined the market."));
                    }

                    let market_data = generator_join_market
                        .proof_marketplace
                        .market_data(generator_join_market.market_id)
                        .call()
                        .await
                        .map_err(|e| {
                            try_read_contract_error!(
                                e,
                                bindings::proof_marketplace::ProofMarketplaceErrors,
                                "ProofMarketplace"
                            );

                            try_read_contract_error!(
                                e,
                                bindings::error::ErrorErrors,
                                "OtherErrors"
                            );

                            format!(
                                "Failed reading market data from proof marketplace contract {}",
                                e
                            )
                        })?;

                    let tx_hash = send_with_optional_gas!(generator_join_market
                        .generator_registry
                        .join_marketplace(
                            generator_join_market.market_id,
                            generator_join_market.compute_per_request_required,
                            generator_join_market.proof_generation_cost,
                            generator_join_market.proposed_time,
                            generator_join_market.commission,
                            false,
                            vec![].into(),
                            vec![].into(),
                        ))
                    .map_err(|e| format!("Failed Creating Join Marketplace Transaction: {}", e))?;

                    // Print the transaction hash
                    println!("{}", tx_hash);

                    if H256::from_slice(&market_data.1.to_vec()) != kalypso_helper::image_id_helpers::hashed_image_id_for_non_confidential_market() {
                        println!("Market: {} is a confidential market. Please Update Encryption Key after doing so. Else the prover will not receive jobs", generator_join_market.market_id);
                    }
                }
            }
            Err(e) => {
                return Err(format!("{}", e));
            }
        }
        Ok(())
    }
}
