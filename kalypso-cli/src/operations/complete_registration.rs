// src/operations/complete_registration.rs

use crate::common_deps::CommonDeps;
use crate::operations::Operation;
use async_trait::async_trait;
use ethers::signers::Signer;
use ethers::types::Address;
use kalypso_helper::send_with_optional_gas;
use std::collections::HashMap;

/// Struct representing the "Complete Registration" operation
pub struct CompleteRegistration;

/// Implementation of the Operation trait for CompleteRegistration
#[async_trait]
impl Operation for CompleteRegistration {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        // Initialize common dependencies
        let generator_info = CommonDeps::generator_registration_instance(&config)?;

        let generator_meta: matching_engine_helpers::generator_lib::generator_store::GeneratorMeta =
            matching_engine_helpers::generator_lib::generator_store::GeneratorMeta {
                display_name: Some(generator_info.display_name),
                display_description: Some(generator_info.display_description),
                website: Some(generator_info.website),
                twitter: Some(generator_info.twitter),
                discord: None,
                logo_url: None,
                banner_url: None,
                contact_email: None,
                github: None,
                linkedin: None,
                medium: None,
                reddit: None,
                youtube: None,
                instagram: None,
                repo_url: None,
                version: None,
                categories: vec![],
                tags: vec![],
                license: None,
                terms_of_service_url: None,
                privacy_policy_url: None,
            };

        let json_string = serde_json::to_string(&generator_meta)
            .map_err(|e| format!("Failed converting GeneratorMeta to string{}", e))?;

        let generator_metadata: Vec<u8> = json_string.into_bytes();

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
