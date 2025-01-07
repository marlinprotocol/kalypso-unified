use std::collections::HashMap;

use async_trait::async_trait;

use crate::{common_deps::CommonDeps, send_with_optional_gas};

use super::Operation;

pub struct WhitelistProverImage;

#[async_trait]
impl Operation for WhitelistProverImage {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let whitelist_prover_info = CommonDeps::whitelist_prover_image_info(&config)?;

        let whitelist_prover_transaction =
            send_with_optional_gas!(whitelist_prover_info.proof_marketplace.add_extra_images(
                whitelist_prover_info.market_id,
                vec![whitelist_prover_info.prover_image_id],
                vec![],
            ))
            .map_err(|e| format!("Whitelist Prover Transaction failed: {}", e))?;

        println!(
            "Whitelist Prover Transaction: {}",
            whitelist_prover_transaction
        );

        Ok(())
    }
}

pub struct WhitelistVerificationImage;

#[async_trait]
impl Operation for WhitelistVerificationImage {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let whitelist_ivs_info = CommonDeps::whitelist_verification_image_info(&config)?;

        let whitelist_ivs_transaction =
            send_with_optional_gas!(whitelist_ivs_info.proof_marketplace.add_extra_images(
                whitelist_ivs_info.market_id,
                vec![],
                vec![whitelist_ivs_info.verification_image_id],
            ))
            .map_err(|e| format!("Whitelist IVS Transaction failed: {}", e))?;

        println!("Whitelist IVS Transaction: {}", whitelist_ivs_transaction);

        Ok(())
    }
}
