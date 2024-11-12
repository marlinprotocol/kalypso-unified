use std::collections::HashMap;

use async_trait::async_trait;
use ethers::{signers::Signer, types::U256};

use crate::{
    common_deps::CommonDeps,
    operations::compute_pcrs::{get_kalypso_image_id_from_pcrs, non_confidential_pcrs},
};

use super::Operation;

pub struct ConfidentialRequest;

#[async_trait]
impl Operation for ConfidentialRequest {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let confidential_request_info = CommonDeps::confidential_request_info(&config)?;
        let non_confidential_pcrs = non_confidential_pcrs();
        let non_confidential_market_kalypso_image_id = get_kalypso_image_id_from_pcrs(
            non_confidential_pcrs.pcr0_vec.into(),
            non_confidential_pcrs.pcr1_vec.into(),
            non_confidential_pcrs.pcr2_vec.into(),
        );

        let market_data = confidential_request_info
            .proof_marketplace
            .market_data(confidential_request_info.market_id)
            .call()
            .await
            .map_err(|_| "Failed making call to proof marketplace contract.".to_string())?;

        if market_data.1 == non_confidential_market_kalypso_image_id.0 {
            return Err("This market is a confidential market".to_string());
        }

        let token_balance = confidential_request_info
            .payment_token
            .balance_of(confidential_request_info.private_key_signer.address())
            .call()
            .await
            .map_err(|_| "Failed making call to payment token contract.".to_string())?;

        if confidential_request_info.max_proof_generation_cost > token_balance {
            return Err("Insufficient payment token available".to_string());
        }

        let token_allowance = confidential_request_info
            .payment_token
            .allowance(
                confidential_request_info.private_key_signer.address(),
                confidential_request_info.proof_marketplace.address(),
            )
            .call()
            .await
            .map_err(|_| "Failed making call to payment token contract.".to_string())?;

        if token_allowance < confidential_request_info.max_proof_generation_cost {
            let token_approval_transaction = CommonDeps::send_and_confirm(
                confidential_request_info
                    .payment_token
                    .approve(
                        confidential_request_info.proof_marketplace.address(),
                        confidential_request_info.max_proof_generation_cost,
                    )
                    .send(),
            )
            .await
            .map_err(|e| format!("Token approval failed: {}", e))?;

            println!("Token Approval: {}", token_approval_transaction);
        }

        let compressed_private_inputs =
            compress_data(&confidential_request_info.private_inputs.to_vec())
                .map_err(|_| "Failed private input compression.".to_string())?;

        let matching_engine_key = confidential_request_info
            .entity_registry
            .pub_key(
                confidential_request_info.proof_marketplace.address(),
                0.into(),
            )
            .await
            .map_err(|e| {
                format!(
                    "Failed Fetching Matching Keys from Entity Key Registry: {}",
                    e
                )
            })?;

        let (encrypted_private_input, acl) = prepare_encrypted_data(
            &compressed_private_inputs,
            &matching_engine_key.to_vec(),
            confidential_request_info.market_id,
        )
        .map_err(|_| "Failed Encryption".to_string())?;

        let proof_request_transaction = CommonDeps::send_and_confirm(
            confidential_request_info
                .proof_marketplace
                .create_ask(
                    bindings::proof_marketplace::Ask {
                        market_id: confidential_request_info.market_id,
                        reward: confidential_request_info.max_proof_generation_cost,
                        expiry: (10000000000000 as u64).into(),
                        time_taken_for_proof_generation: confidential_request_info
                            .max_proof_generation_time,
                        deadline: U256::zero(),
                        refund_address: confidential_request_info.private_key_signer.address(),
                        prover_data: confidential_request_info.inputs,
                    },
                    0.into(),                       // `secret_type` argument
                    encrypted_private_input.into(), // `encrypted private_inputs` argument
                    acl.into(),                     // `acl` argument
                )
                .send(),
        )
        .await?;

        // Print the transaction hash
        println!("proof request transaction: {}", proof_request_transaction);

        Ok(())
    }
}

pub struct NonConfidentialRequest;

#[async_trait]
impl Operation for NonConfidentialRequest {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let non_confidential_request_info = CommonDeps::non_confidential_request_info(&config)?;
        let non_confidential_pcrs = non_confidential_pcrs();
        let non_confidential_market_kalypso_image_id = get_kalypso_image_id_from_pcrs(
            non_confidential_pcrs.pcr0_vec.into(),
            non_confidential_pcrs.pcr1_vec.into(),
            non_confidential_pcrs.pcr2_vec.into(),
        );

        let market_data = non_confidential_request_info
            .proof_marketplace
            .market_data(non_confidential_request_info.market_id)
            .call()
            .await
            .map_err(|_| "Failed making call to proof marketplace contract.".to_string())?;

        if market_data.1 != non_confidential_market_kalypso_image_id.0 {
            return Err("This market is not a confidential market".to_string());
        }

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

use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

fn compress_data(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(data)
        .map_err(|_| "Failed Private Input Encoding".to_string())?;
    let compressed_data = encoder
        .finish()
        .map_err(|_| "Failed private input compression".to_string())?;
    Ok(compressed_data)
}

use openssl::rand;
// for kalypso-specific, market_id is associated data
fn prepare_encrypted_data(
    data: &[u8],
    pubkey: &[u8],
    associated_data: U256,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let mut cipher = vec![0; 32];
    rand::rand_bytes(&mut cipher).map_err(|_| "Failed creating cipher".to_string())?;

    let encrypted_data =
        kalypso_helper::secret_inputs_helpers::encrypt_aes_gcm(data, pubkey, associated_data)
            .map_err(|_| "Failed making call to payment token contract.".to_string())?;
    let acl = kalypso_helper::secret_inputs_helpers::encrypt_ecies(pubkey, &cipher)
        .map_err(|_| "Failed encrypting cipher key".to_string())?;

    Ok((encrypted_data, acl))
}
