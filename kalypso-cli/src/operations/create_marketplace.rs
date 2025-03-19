use crate::{common_deps::CommonDeps, operations::Operation};
use async_trait::async_trait;
use ethers::signers::Signer;
use futures::StreamExt;
use kalypso_helper::send_with_optional_gas;
use std::collections::HashMap;

pub struct VerifyKeyInTeeVerifier;

#[async_trait]
impl Operation for VerifyKeyInTeeVerifier {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let verify_key_in_tee_verifier_info =
            CommonDeps::verify_key_in_tee_verifier_wrapper_info(&config)?;

        let attestation_stream = kalypso_helper::pcr_helpers::build_attestation(
            &verify_key_in_tee_verifier_info.attestation_utility,
            false,
        )
        .await
        .map_err(|e| format!("Failed Building Attestations {}", e))?;

        let attestation_data: Vec<u8> = attestation_stream
            .fold(Vec::new(), |mut acc, item| async {
                match item {
                    Ok(bytes) => {
                        acc.extend_from_slice(&bytes);
                        acc
                    }
                    Err(e) => {
                        println!("Error while receiving data: {}", e);
                        acc
                    }
                }
            })
            .await;

        let ecies_pubkey =
            kalypso_helper::pcr_helpers::get_pubkey_from_attestation(attestation_data.clone())
                .map_err(|e| format!("Failed Getting Pubkey From Attestation {}", e))?;

        println!("Ecies Pubkey of the enclave: {}", hex::encode(ecies_pubkey));

        let verified_attestation = kalypso_helper::pcr_helpers::get_verified_attestation(
            &verify_key_in_tee_verifier_info.attestation_verifier_url,
            attestation_data,
            false,
        )
        .await
        .map_err(|e| format!("Failed Verifying attestation {}", e))?;

        let transaction = send_with_optional_gas!(verify_key_in_tee_verifier_info
            .verifier_wraooer
            .verify_key(verified_attestation.into()))
        .map_err(|e| format!("Transaction failed: {}", e))?;

        println!("Verify Key Transaction: {}", transaction);
        Ok(())
    }
}

pub struct AddImageToTeeVerifier;

#[async_trait]
impl Operation for AddImageToTeeVerifier {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let add_prover_to_tee_verifier_info =
            CommonDeps::add_prover_to_tee_verifier_wrapper_info(&config)?;
        let transaction = send_with_optional_gas!(add_prover_to_tee_verifier_info
            .verifier_wraooer
            .add_enclave_image_to_family(add_prover_to_tee_verifier_info.prover_pcrs.into(),))
        .map_err(|e| format!("Transaction failed: {}", e))?;

        println!("Add Image Transaction: {}", transaction);
        Ok(())
    }
}

pub struct AddIvsToMarket;

#[async_trait]
impl Operation for AddIvsToMarket {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let add_ivs_to_market_info = CommonDeps::add_ivs_to_market_info(&config)?;
        let transaction =
            send_with_optional_gas!(add_ivs_to_market_info.proof_marketplace.add_extra_images(
                add_ivs_to_market_info.market_id,
                vec![],
                vec![add_ivs_to_market_info.ivs_pcrs.into()],
            ))
            .map_err(|e| format!("Transaction failed: {}", e))?;

        println!("Add IVS Image Transaction: {}", transaction);
        Ok(())
    }
}

pub struct AddProverToMarket;

#[async_trait]
impl Operation for AddProverToMarket {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let add_prover_to_market_info = CommonDeps::add_prover_to_market_info(&config)?;
        let transaction = send_with_optional_gas!(add_prover_to_market_info
            .proof_marketplace
            .add_extra_images(
                add_prover_to_market_info.market_id,
                vec![add_prover_to_market_info.prover_pcrs.into()],
                vec![],
            ))
        .map_err(|e| format!("Transaction failed: {}", e))?;

        println!("Add Prover Image Transaction: {}", transaction);
        Ok(())
    }
}

pub struct CreateTeeVerifier;

#[async_trait]
impl Operation for CreateTeeVerifier {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let tee_verifier_create_info = CommonDeps::tee_verifier_wrapper_deployer_args(&config)?;
        let tee_verifier_creation_transaction = send_with_optional_gas!(tee_verifier_create_info
            .tee_verifier_deployer
            .create_tee_verifier_wrapper(
                tee_verifier_create_info.admin,
                tee_verifier_create_info.attestation_verifier,
                vec![tee_verifier_create_info.prover_pcrs.into()],
            ))
        .map_err(|e| format!("Tee Verifier Creation Transaction failed: {}", e))?;

        println!(
            "Tee Verifier Creation Transaction: {}",
            tee_verifier_creation_transaction
        );
        Ok(())
    }
}

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
            let token_approval_transaction =
                send_with_optional_gas!(market_create_info.payment_token.approve(
                    market_create_info.proof_marketplace.address(),
                    market_creation_cost,
                ))
                .map_err(|e| format!("Token approval failed: {}", e))?;

            println!("Token Approval: {}", token_approval_transaction);
        }

        let market_creation_transaction =
            send_with_optional_gas!(market_create_info.proof_marketplace.create_market(
                vec![12, 23].into(),
                market_create_info.verifier_wrapper,
                market_create_info.prover_pcrs.into(),
                market_create_info.ivs_pcrs.into(),
            ))
            .map_err(|e| format!("Market Creation Transaction failed: {}", e))?;

        println!(
            "Market Creation Transaction: {}",
            market_creation_transaction
        );

        Ok(())
    }
}
