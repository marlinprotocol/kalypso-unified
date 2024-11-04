// src/operations/complete_registration.rs

use crate::common_deps::CommonDeps;
use crate::operations::Operation;
use async_trait::async_trait;
use ethers::signers::Signer;
use ethers::types::Address;
use std::collections::HashMap;

/// Struct representing the "Complete Registration" operation
pub struct CompleteRegistration;

/// Implementation of the Operation trait for CompleteRegistration
#[async_trait]
impl Operation for CompleteRegistration {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        // Initialize common dependencies
        let generator_info = CommonDeps::generator_registration_instance(&config)?;

        match generator_info
            .generator_registry
            .generator_registry(generator_info.private_key_signer.address())
            .call()
            .await
        {
            Ok(data) => {
                if data.0.eq(&Address::zero()) {
                    let tx_hash = CommonDeps::send_and_confirm(
                        generator_info
                            .generator_registry
                            .register(
                                generator_info.reward_address,
                                generator_info.declared_compute,
                                vec![12, 23].into(),
                            )
                            .send(),
                    )
                    .await?;

                    // Print the transaction hash
                    println!("{}", tx_hash);
                } else {
                    return Err(format!("Generator is already registered"));
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
