// src/operations/complete_registration.rs

use crate::common_deps::CommonDeps;
use crate::operations::Operation;
use crate::send_with_optional_gas;
use async_trait::async_trait;
use ethers::signers::Signer;
use ethers::types::Address;
use serde::Serialize;
use std::collections::HashMap;

/// Struct representing the "Complete Registration" operation
pub struct CompleteRegistration;

/// Implementation of the Operation trait for CompleteRegistration
#[async_trait]
impl Operation for CompleteRegistration {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        // Initialize common dependencies
        let generator_info = CommonDeps::generator_registration_instance(&config)?;

        #[derive(Serialize)]
        struct Generator {
            display_name: String,
            display_description: String,
            website: String,
            twitter: String,
        }

        let generator = Generator {
            display_name: generator_info.display_name,
            display_description: generator_info.display_description,
            website: generator_info.website,
            twitter: generator_info.twitter,
        };

        let generator_json = serde_json::to_string(&generator)
            .map_err(|e| format!("Failed composing generator metadata. {}", e))?;

        let generator_metadata = generator_json.as_bytes();

        match generator_info
            .generator_registry
            .prover_registry(generator_info.private_key_signer.address())
            .call()
            .await
        {
            Ok(data) => {
                if data.0.eq(&Address::zero()) {
                    let tx_hash =
                        send_with_optional_gas!(generator_info.generator_registry.register(
                            generator_info.reward_address,
                            generator_info.declared_compute,
                            generator_metadata.to_vec().into(),
                        ))?;

                    // Print the transaction hash
                    println!("{}", tx_hash);
                } else {
                    return Err(format!("Generator is already registered"));
                }
            }
            Err(e) => {
                return Err(format!(
                    "Failed making call to generator registry contract. {}",
                    e
                ));
            }
        }

        Ok(())
    }
}
