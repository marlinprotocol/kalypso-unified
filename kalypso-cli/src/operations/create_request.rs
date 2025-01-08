use std::collections::HashMap;

use async_trait::async_trait;
use ethers::providers::Middleware;
use ethers::{core::rand, signers::Signer, types::U256};

use crate::send_with_optional_gas;
use crate::{common_deps::CommonDeps, operations::compute_pcrs::non_confidential_pcrs};

use super::Operation;

pub struct ConfidentialRequest;

#[async_trait]
impl Operation for ConfidentialRequest {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let confidential_request_info = CommonDeps::confidential_request_info(&config)?;
        let non_confidential_pcrs = non_confidential_pcrs();
        let non_confidential_market_kalypso_image_id =
            kalypso_helper::image_id_helpers::get_kalypso_image_id_from_pcrs(
                non_confidential_pcrs.pcr0_vec.into(),
                non_confidential_pcrs.pcr1_vec.into(),
                non_confidential_pcrs.pcr2_vec.into(),
            );

        let market_data = confidential_request_info
            .proof_marketplace
            .market_data(confidential_request_info.market_id)
            .call()
            .await
            .map_err(|e| format!("Failed making call to proof marketplace contract {}", e))?;

        if market_data.1 == non_confidential_market_kalypso_image_id.0 {
            return Err("This market is a confidential market".to_string());
        }

        let token_balance = confidential_request_info
            .payment_token
            .balance_of(confidential_request_info.private_key_signer.address())
            .call()
            .await
            .map_err(|e| format!("Failed making call to payment token contract {}", e))?;

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
            .map_err(|e| format!("Failed making call to payment token contract {}", e))?;

        if token_allowance < confidential_request_info.max_proof_generation_cost {
            let token_approval_transaction =
                send_with_optional_gas!(confidential_request_info.payment_token.approve(
                    confidential_request_info.proof_marketplace.address(),
                    confidential_request_info.max_proof_generation_cost,
                ))
                .map_err(|e| format!("Token approval failed: {}", e))?;

            println!("Token Approval: {}", token_approval_transaction);
        }

        let compressed_private_inputs =
            compress_data(&confidential_request_info.private_inputs.to_vec())
                .map_err(|e| format!("Failed private input compression {}", e))?;

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
        .map_err(|e| format!("Failed Encryption: {}", e))?;

        let latest_l2_block = confidential_request_info
            .provider_http
            .get_block_number()
            .await
            .map_err(|e| {
                format!(
                    "Failed fetching latest kalypso block number from chain: {}",
                    e
                )
            })?;

        let latest_l1_block = matching_engine_helpers::utility::get_l1_block_from_l2_block(
            &confidential_request_info.kalypso_rpc_url,
            latest_l2_block.as_u64().into(),
        )
        .await;

        if latest_l1_block.is_none() {
            return Err("Failed fetching latest 11 block".into());
        }

        let proof_request_transaction =
            send_with_optional_gas!(confidential_request_info.proof_marketplace.create_bid(
                bindings::proof_marketplace::Bid {
                    market_id: confidential_request_info.market_id,
                    reward: confidential_request_info.max_proof_generation_cost,
                    expiry: (latest_l1_block.unwrap().as_u64() + 200).into(),
                    time_taken_for_proof_generation: confidential_request_info
                        .max_proof_generation_time,
                    deadline: U256::zero(),
                    refund_address: confidential_request_info.private_key_signer.address(),
                    prover_data: confidential_request_info.inputs,
                },
                0.into(),                       // `secret_type` argument
                encrypted_private_input.into(), // `encrypted private_inputs` argument
                acl.into(),                     // `acl` argument
                vec![].into(),                  // extra data
            ))
            .map_err(|e| format!("Failed Creating Proof Request Transaction: {}", e))?;

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
        let non_confidential_market_kalypso_image_id =
            kalypso_helper::image_id_helpers::get_kalypso_image_id_from_pcrs(
                non_confidential_pcrs.pcr0_vec.into(),
                non_confidential_pcrs.pcr1_vec.into(),
                non_confidential_pcrs.pcr2_vec.into(),
            );

        let market_data = non_confidential_request_info
            .proof_marketplace
            .market_data(non_confidential_request_info.market_id)
            .call()
            .await
            .map_err(|e| format!("Failed making call to proof marketplace contract: {}", e))?;

        if market_data.1 != non_confidential_market_kalypso_image_id.0 {
            return Err("This market is not a confidential market".to_string());
        }

        let token_balance = non_confidential_request_info
            .payment_token
            .balance_of(non_confidential_request_info.private_key_signer.address())
            .call()
            .await
            .map_err(|e| format!("Failed making call to payment token contract {}", e))?;

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
            .map_err(|e| format!("Failed making call to payment token contract {}", e))?;

        if token_allowance < non_confidential_request_info.max_proof_generation_cost {
            let token_approval_transaction =
                send_with_optional_gas!(non_confidential_request_info.payment_token.approve(
                    non_confidential_request_info.proof_marketplace.address(),
                    non_confidential_request_info.max_proof_generation_cost,
                ))
                .map_err(|e| format!("Token approval failed: {}", e))?;

            println!("Token Approval: {}", token_approval_transaction);
        }

        let latest_l2_block = non_confidential_request_info
            .provider_http
            .get_block_number()
            .await
            .map_err(|e| {
                format!(
                    "Failed fetching latest kalypso block number from chain: {}",
                    e
                )
            })?;

        let latest_l1_block = matching_engine_helpers::utility::get_l1_block_from_l2_block(
            &non_confidential_request_info.kalypso_rpc_url,
            latest_l2_block.as_u64().into(),
        )
        .await;

        if latest_l1_block.is_none() {
            return Err("Failed fetching latest 11 block".into());
        }

        let proof_request_transaction =
            send_with_optional_gas!(non_confidential_request_info.proof_marketplace.create_bid(
                bindings::proof_marketplace::Bid {
                    market_id: non_confidential_request_info.market_id,
                    reward: non_confidential_request_info.max_proof_generation_cost,
                    expiry: (latest_l1_block.unwrap().as_u64() + 200).into(),
                    time_taken_for_proof_generation: non_confidential_request_info
                        .max_proof_generation_time,
                    deadline: U256::zero(),
                    refund_address: non_confidential_request_info.private_key_signer.address(),
                    prover_data: non_confidential_request_info.inputs,
                },
                0.into(),      // `secret_type` argument
                vec![].into(), // `private_inputs` argument
                vec![].into(), // `acl` argument
                vec![].into(), // `extra` data
            ))
            .map_err(|e| format!("Failed Creating Proof Request Transaction: {}", e))?;

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
        .map_err(|e| format!("Failed Private Input Encoding: {}", e))?;
    let compressed_data = encoder
        .finish()
        .map_err(|e| format!("Failed private input compression: {}", e))?;
    Ok(compressed_data)
}

use rand::RngCore;
// for kalypso-specific, market_id is associated data
fn prepare_encrypted_data(
    data: &[u8],
    pubkey: &[u8],
    associated_data: U256,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let mut cipher = vec![0; 32];
    rand::thread_rng().fill_bytes(&mut cipher);

    let encrypted_data =
        kalypso_helper::secret_inputs_helpers::encrypt_aes_gcm(data, pubkey, associated_data)
            .map_err(|e| format!("Failed making call to payment token contract {}", e))?;
    let acl = kalypso_helper::secret_inputs_helpers::encrypt_ecies(pubkey, &cipher)
        .map_err(|e| format!("{}. {}", "Failed encrypting cipher key".to_string(), e))?;

    Ok((encrypted_data, acl))
}
