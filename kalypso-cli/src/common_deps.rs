// src/common_deps.rs

use ethers::prelude::*;
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;

use crate::try_read_contract_error;

macro_rules! get_config_ref {
    ($config:expr, $key:expr, $var:ident) => {
        let $var = $config.get($key).ok_or(concat!("Missing '", $key, "'"))?;
    };
}

/// Struct holding common dependencies required by multiple operations
pub struct GeneratorRegister {
    pub private_key_signer: LocalWallet,
    pub generator_registry:
        bindings::prover_manager::ProverManager<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub reward_address: Address,
    pub declared_compute: U256,
    pub display_name: String,
    pub display_description: String,
    pub website: String,
    pub twitter: String,
}

pub struct GeneratorJoinMarket {
    pub private_key_signer: LocalWallet,
    pub generator_registry:
        bindings::prover_manager::ProverManager<SignerMiddleware<Provider<Http>, LocalWallet>>,

    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub market_id: U256,
    pub compute_per_request_required: U256,
    pub proof_generation_cost: U256,
    pub proposed_time: U256,
}

pub struct CommonDeps;

impl CommonDeps {
    pub async fn send_and_confirm<P, M>(
        send_future: impl Future<Output = Result<PendingTransaction<'_, P>, ContractError<M>>>,
    ) -> Result<String, String>
    where
        P: JsonRpcClient + Send + Sync + 'static,
        M: ethers::middleware::Middleware,
    {
        // Await the send operation
        let pending_tx = send_future.await.map_err(|e| {
            // read error bytes here
            eprintln!("\n=== Transaction Error ===");
            eprintln!("Error: {}", e);
            eprintln!("Attempting to parse contract-specific errors:\n");

            try_read_contract_error!(
                e,
                bindings::entity_key_registry::EntityKeyRegistryErrors,
                "EntityKeyRegistry"
            );

            try_read_contract_error!(
                e,
                bindings::native_staking::NativeStakingErrors,
                "NativeStaking"
            );

            try_read_contract_error!(
                e,
                bindings::proof_marketplace::ProofMarketplaceErrors,
                "ProofMarketplace"
            );

            try_read_contract_error!(
                e,
                bindings::prover_manager::ProverManagerErrors,
                "ProverManager"
            );

            try_read_contract_error!(
                e,
                bindings::staking_manager::StakingManagerErrors,
                "StakingManager"
            );

            try_read_contract_error!(
                e,
                bindings::symbiotic_staking::SymbioticStakingErrors,
                "SymbioticStaking"
            );

            try_read_contract_error!(
                e,
                bindings::symbiotic_staking_reward::SymbioticStakingRewardErrors,
                "SymbioticStakingRewards"
            );

            try_read_contract_error!(e, bindings::error::ErrorErrors, "OtherErrors");

            eprintln!("========================\n");
            format!("Failed to send transaction: {}", e)
        })?;

        // Await the confirmation with at least 1 confirmation
        let receipt = pending_tx
            .confirmations(1)
            .await
            .map_err(|e| format!("Failed to confirm transaction: {}", e))?
            .ok_or_else(|| "Transaction receipt not found".to_string())?;

        // Extract and return the transaction hash
        Ok(format!(
            "Transaction: {}",
            hex::encode(receipt.transaction_hash)
        ))
    }

    pub fn generator_join_market_instance(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<GeneratorJoinMarket, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "generator_registry", generator_registry_address);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "market_id", market_id);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "compute_per_request", compute_per_request);
        get_config_ref!(config, "proof_generation_cost", proof_generation_cost);
        get_config_ref!(config, "proposed_time", proposed_time);

        let (generator_registry, private_key_signer) = get_generator_registry_instance(
            private_key,
            chain_id,
            generator_registry_address,
            rpc_url,
        )?;

        let (proof_marketplace, _) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        let market_id = U256::from_dec_str(market_id.as_str())
            .map_err(|e| format!("Invalid Market Id: {}", e))?;

        let compute_per_request = U256::from_dec_str(compute_per_request.as_str())
            .map_err(|e| format!("Invalid Compute Per Request: {}", e))?;

        let proof_generation_cost = U256::from_dec_str(&proof_generation_cost.as_str())
            .map_err(|e| format!("Invalid Proof Generation Cost: {}", e))?;

        let proposed_time = U256::from_dec_str(&proposed_time.as_str())
            .map_err(|e| format!("Invalid Proposed Time: {}", e))?;

        Ok(GeneratorJoinMarket {
            private_key_signer,
            generator_registry,
            proof_marketplace,
            market_id,
            compute_per_request_required: compute_per_request,
            proof_generation_cost,
            proposed_time,
        })
    }

    /// Initializes CommonDeps from the provided configuration
    pub fn generator_registration_instance(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<GeneratorRegister, String> {
        // Extract and validate required fields
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "generator_registry", generator_registry_address);
        get_config_ref!(config, "reward_address", reward_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "declared_compute", declared_compute);

        get_config_ref!(config, "display_name", display_name);
        get_config_ref!(config, "display_description", display_description);
        get_config_ref!(config, "website", website);
        get_config_ref!(config, "twitter", twitter);

        let (generator_registry, private_key_signer) = get_generator_registry_instance(
            private_key,
            chain_id,
            generator_registry_address,
            rpc_url,
        )?;

        let reward_address = reward_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Reward Address: {}", e))?;

        let declared_compute = U256::from_dec_str(declared_compute.as_str())
            .map_err(|e| format!("Invalid Declared Compute: {}", e))?;

        Ok(GeneratorRegister {
            private_key_signer,
            generator_registry,
            reward_address,
            declared_compute,
            display_name: display_name.into(),
            display_description: display_description.into(),
            website: website.into(),
            twitter: twitter.into(),
        })
    }
}

fn get_generator_registry_instance(
    private_key: &str,
    chain_id: &str,
    generator_registry_address: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::prover_manager::ProverManager<SignerMiddleware<Provider<Http>, LocalWallet>>,
        LocalWallet,
    ),
    String,
> {
    // Parse the private key into a LocalWallet and set the chain ID
    let private_key_signer = private_key
        .parse::<LocalWallet>()
        .map_err(|e| format!("Failed to parse private key: {}", e))?
        .with_chain_id(
            chain_id
                .parse::<u64>()
                .map_err(|e| format!("Invalid chain_id: {}", e))?,
        );

    // Parse the Generator Registry address
    let generator_registry_address = generator_registry_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid Generator Registry address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the SignerMiddleware with the provider and signer
    let client = SignerMiddleware::new(provider_http.clone(), private_key_signer.clone());

    let client_arc = Arc::new(client);

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let generator_registry = bindings::prover_manager::ProverManager::new(
        generator_registry_address,
        client_arc.clone(),
    );

    Ok((generator_registry, private_key_signer))
}

fn get_proof_marketplace_instance(
    private_key: &str,
    chain_id: &str,
    proof_marketplace_address: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::proof_marketplace::ProofMarketplace<
            SignerMiddleware<Provider<Http>, LocalWallet>,
        >,
        LocalWallet,
    ),
    String,
> {
    // Parse the private key into a LocalWallet and set the chain ID
    let private_key_signer = private_key
        .parse::<LocalWallet>()
        .map_err(|e| format!("Failed to parse private key: {}", e))?
        .with_chain_id(
            chain_id
                .parse::<u64>()
                .map_err(|e| format!("Invalid chain_id: {}", e))?,
        );

    // Parse the Generator Registry address
    let proof_marketplace_address = proof_marketplace_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid Proof Marketplace address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the SignerMiddleware with the provider and signer
    let client = SignerMiddleware::new(provider_http.clone(), private_key_signer.clone());

    let client_arc = Arc::new(client);

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let proof_marketplace = bindings::proof_marketplace::ProofMarketplace::new(
        proof_marketplace_address,
        client_arc.clone(),
    );

    Ok((proof_marketplace, private_key_signer))
}

#[allow(unused)] //will be required latter
fn get_token_instance(
    private_key: &str,
    chain_id: &str,
    token_address: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::ierc20::IERC20<SignerMiddleware<Provider<Http>, LocalWallet>>,
        LocalWallet,
    ),
    String,
> {
    // Parse the private key into a LocalWallet and set the chain ID
    let private_key_signer = private_key
        .parse::<LocalWallet>()
        .map_err(|e| format!("Failed to parse private key: {}", e))?
        .with_chain_id(
            chain_id
                .parse::<u64>()
                .map_err(|e| format!("Invalid chain_id: {}", e))?,
        );

    // Parse the Generator Registry address
    let token_address = token_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid Token address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the SignerMiddleware with the provider and signer
    let client = SignerMiddleware::new(provider_http.clone(), private_key_signer.clone());

    let client_arc = Arc::new(client);

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let token_address = bindings::ierc20::IERC20::new(token_address, client_arc.clone());

    Ok((token_address, private_key_signer))
}

pub struct MarketExitInfo {
    #[allow(unused)] // will use latter
    pub private_key_signer: LocalWallet,
    pub generator_registry:
        bindings::prover_manager::ProverManager<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub market_id: U256,
}
impl CommonDeps {
    pub fn marketplace_exit_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<MarketExitInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "generator_registry", generator_registry_address);
        get_config_ref!(config, "market_id", market_id);
        get_config_ref!(config, "chain_id", chain_id);

        let (generator_registry, private_key_signer) = get_generator_registry_instance(
            private_key,
            chain_id,
            generator_registry_address,
            rpc_url,
        )?;

        let market_id = U256::from_dec_str(market_id.as_str())
            .map_err(|e| format!("Invalid Market Id: {}", e))?;

        Ok(MarketExitInfo {
            private_key_signer,
            generator_registry,
            market_id,
        })
    }
}

pub struct MarketCreateInfo {
    pub private_key_signer: LocalWallet,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub prover_pcrs: Vec<u8>,
    pub ivs_pcrs: Vec<u8>,
    pub payment_token: bindings::ierc20::IERC20<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub verifier_wrapper: Address,
}

impl CommonDeps {
    pub fn market_create_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<MarketCreateInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "prover_image_id", prover_pcrs);
        get_config_ref!(config, "verification_image_id", ivs_pcrs);
        get_config_ref!(config, "verifier_wrapper", verifier_wrapper);
        get_config_ref!(config, "payment_token", payment_token);

        let (proof_marketplace, private_key_signer) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        let prover_pcrs = {
            let trimmed_key = if prover_pcrs.starts_with("0x") || prover_pcrs.starts_with("0X") {
                &prover_pcrs[2..]
            } else {
                prover_pcrs
            };
            hex::decode(trimmed_key).map_err(|e| format!("Invalid Prover PCRs: {}", e))?
        };

        let ivs_pcrs = {
            let trimmed_key = if ivs_pcrs.starts_with("0x") || ivs_pcrs.starts_with("0X") {
                &ivs_pcrs[2..]
            } else {
                ivs_pcrs
            };
            hex::decode(trimmed_key).map_err(|e| format!("Invalid IVS PCRs: {}", e))?
        };

        let (payment_token, _) =
            get_token_instance(private_key, chain_id, &payment_token, rpc_url)?;

        let verifier_wrapper = verifier_wrapper
            .parse::<Address>()
            .map_err(|e| format!("Invalid Verifier Wrapper Address: {}", e))?;

        Ok(MarketCreateInfo {
            private_key_signer,
            proof_marketplace,
            prover_pcrs,
            ivs_pcrs,
            verifier_wrapper,
            payment_token,
        })
    }
}

pub struct ComputePcrsInfo {
    pub attestation_utility: String,
    pub attestation_verifier: String,
}

impl CommonDeps {
    pub fn compute_pcrs_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<ComputePcrsInfo, String> {
        get_config_ref!(config, "attestation_server_url", attestation_server_url);
        get_config_ref!(config, "attestation_verifier_url", attestation_verifier_url);

        Ok(ComputePcrsInfo {
            attestation_utility: attestation_server_url.to_string(),
            attestation_verifier: attestation_verifier_url.to_string(),
        })
    }
}

pub struct ClaimRewardsInfo {
    #[allow(unused)]
    pub private_key_signer: LocalWallet,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    #[allow(unused)]
    pub payment_token: bindings::ierc20::IERC20<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub reward_address: Address,
}

impl CommonDeps {
    pub fn claim_rewards_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<ClaimRewardsInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "reward_address", reward_address);
        get_config_ref!(config, "payment_token", payment_token);

        let (proof_marketplace, private_key_signer) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        let reward_address = reward_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Reward Address: {}", e))?;

        let (payment_token, _) =
            get_token_instance(private_key, chain_id, &payment_token, rpc_url)?;

        Ok(ClaimRewardsInfo {
            private_key_signer,
            proof_marketplace,
            payment_token,
            reward_address,
        })
    }
}

pub struct NativeStakeInfo {
    pub private_key_signer: LocalWallet,
    pub native_staking:
        bindings::native_staking::NativeStaking<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub staking_token: bindings::ierc20::IERC20<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub staking_amount: U256,
    pub operator_address: Address,
}

impl CommonDeps {
    pub fn native_staking_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<NativeStakeInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "native_staking", native_staking_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "staking_token", staking_token);
        get_config_ref!(config, "staking_amount", staking_amount);
        get_config_ref!(config, "operator_address", operator_address);

        let (staking_token, _) =
            get_token_instance(private_key, chain_id, &staking_token, rpc_url)?;

        let (native_staking, private_key_signer) =
            get_native_staking_instance(private_key, chain_id, native_staking_address, rpc_url)?;

        let staking_amount = U256::from_dec_str(staking_amount.as_str())
            .map_err(|e| format!("Invalid Staking Amount: {}", e))?;

        let operator_address = operator_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Operator Address: {}", e))?;

        Ok(NativeStakeInfo {
            private_key_signer,
            native_staking,
            staking_token,
            staking_amount,
            operator_address,
        })
    }
}

fn get_native_staking_instance(
    private_key: &str,
    chain_id: &str,
    native_staking_address: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::native_staking::NativeStaking<SignerMiddleware<Provider<Http>, LocalWallet>>,
        LocalWallet,
    ),
    String,
> {
    // Parse the private key into a LocalWallet and set the chain ID
    let private_key_signer = private_key
        .parse::<LocalWallet>()
        .map_err(|e| format!("Failed to parse private key: {}", e))?
        .with_chain_id(
            chain_id
                .parse::<u64>()
                .map_err(|e| format!("Invalid chain_id: {}", e))?,
        );

    // Parse the Generator Registry address
    let native_staking_address = native_staking_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid Native Staking address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the SignerMiddleware with the provider and signer
    let client = SignerMiddleware::new(provider_http.clone(), private_key_signer.clone());

    let client_arc = Arc::new(client);

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let native_staking =
        bindings::native_staking::NativeStaking::new(native_staking_address, client_arc.clone());

    Ok((native_staking, private_key_signer))
}
pub struct ReadAttestationInfo {
    pub attestation_utility: String,
}

impl CommonDeps {
    pub fn read_attestation_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<ReadAttestationInfo, String> {
        get_config_ref!(config, "attestation_server_url", attestation_server_url);

        Ok(ReadAttestationInfo {
            attestation_utility: attestation_server_url.to_string(),
        })
    }
}

pub struct NonConfidentialRequest {
    pub private_key_signer: LocalWallet,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub payment_token: bindings::ierc20::IERC20<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub max_proof_generation_cost: U256,
    pub max_proof_generation_time: U256,
    pub inputs: ethers::types::Bytes,
    pub market_id: U256,
    pub provider_http: Provider<Http>,
    pub kalypso_rpc_url: String,
}

impl CommonDeps {
    pub fn non_confidential_request_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<NonConfidentialRequest, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "payment_token", payment_token);
        get_config_ref!(config, "inputs", inputs);
        get_config_ref!(config, "market_id", market_id);

        get_config_ref!(
            config,
            "max_proof_generation_cost",
            max_proof_generation_cost
        );
        get_config_ref!(
            config,
            "max_proof_generation_time",
            max_proof_generation_time
        );

        let (proof_marketplace, private_key_signer) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        let (payment_token, _) =
            get_token_instance(private_key, chain_id, &payment_token, rpc_url)?;

        let max_proof_generation_cost = U256::from_dec_str(max_proof_generation_cost.as_str())
            .map_err(|e| format!("Max Proof Generation Cost: {}", e))?;

        let max_proof_generation_time = U256::from_dec_str(max_proof_generation_time.as_str())
            .map_err(|e| format!("Max Proof Generation Time: {}", e))?;

        let trimmed_inputs = if inputs.starts_with("0x") || inputs.starts_with("0X") {
            &inputs[2..]
        } else {
            inputs
        };
        if trimmed_inputs.len() % 2 != 0 {
            return Err("Hex string has an invalid length".to_string());
        }

        let inputs = hex::decode(trimmed_inputs)
            .map_err(|e| format!("Invalid Input Bytes: {}", e))?
            .into();

        let market_id = U256::from_dec_str(market_id.as_str())
            .map_err(|e| format!("Invalid Market Id: {}", e))?;

        let provider_http =
            Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

        Ok(NonConfidentialRequest {
            private_key_signer,
            proof_marketplace,
            payment_token,
            max_proof_generation_cost,
            max_proof_generation_time,
            inputs,
            market_id,
            provider_http,
            kalypso_rpc_url: rpc_url.clone(),
        })
    }
}

pub struct DiscardRequestInfo {
    #[allow(unused)]
    pub private_key_signer: LocalWallet,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub ask_id: U256,
}

impl CommonDeps {
    pub fn discard_request_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<DiscardRequestInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "ask_id", ask_id);

        let (proof_marketplace, private_key_signer) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        let ask_id =
            U256::from_dec_str(&ask_id.as_str()).map_err(|e| format!("Invalid Ask Id: {}", e))?;

        Ok(DiscardRequestInfo {
            private_key_signer,
            proof_marketplace,
            ask_id,
        })
    }
}

pub struct WhitelistProverImageInfo {
    #[allow(unused)]
    pub private_key_signer: LocalWallet,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub prover_image_id: Bytes,
    pub market_id: U256,
}

impl CommonDeps {
    pub fn whitelist_prover_image_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<WhitelistProverImageInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "prover_image_id", prover_pcrs);
        get_config_ref!(config, "market_id", market_id);

        let (proof_marketplace, private_key_signer) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        let prover_pcrs = {
            let trimmed_key = if prover_pcrs.starts_with("0x") || prover_pcrs.starts_with("0X") {
                &prover_pcrs[2..]
            } else {
                prover_pcrs
            };
            hex::decode(trimmed_key).map_err(|e| format!("Invalid Prover PCRs: {}", e))?
        };

        let market_id = U256::from_dec_str(market_id.as_str())
            .map_err(|e| format!("Invalid Market Id: {}", e))?;

        Ok(WhitelistProverImageInfo {
            private_key_signer,
            proof_marketplace,
            prover_image_id: prover_pcrs.into(),
            market_id,
        })
    }
}

pub struct WhitelistVerificationImageInfo {
    #[allow(unused)]
    pub private_key_signer: LocalWallet,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub verification_image_id: Bytes,
    pub market_id: U256,
}

impl CommonDeps {
    pub fn whitelist_verification_image_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<WhitelistVerificationImageInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "verification_image_id", ivs_pcrs);
        get_config_ref!(config, "market_id", market_id);

        let (proof_marketplace, private_key_signer) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        let ivs_pcrs = {
            let trimmed_key = if ivs_pcrs.starts_with("0x") || ivs_pcrs.starts_with("0X") {
                &ivs_pcrs[2..]
            } else {
                ivs_pcrs
            };
            hex::decode(trimmed_key).map_err(|e| format!("Invalid Prover PCRs: {}", e))?
        };

        let market_id = U256::from_dec_str(market_id.as_str())
            .map_err(|e| format!("Invalid Market Id: {}", e))?;

        Ok(WhitelistVerificationImageInfo {
            private_key_signer,
            proof_marketplace,
            verification_image_id: ivs_pcrs.into(),
            market_id,
        })
    }
}

pub struct UpdateEncryptionKeyInfo {
    #[allow(unused)]
    pub private_key_signer: LocalWallet,
    pub generator_registry:
        bindings::prover_manager::ProverManager<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<Provider<Http>>,
    pub attestation_utility: String,
    pub attestation_verifier: String,
    pub enclave_client_url: String,
    pub market_id: U256,
}

impl CommonDeps {
    pub fn update_encryption_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<UpdateEncryptionKeyInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "generator_registry", generator_registry_address);
        get_config_ref!(config, "chain_id", chain_id);

        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);

        get_config_ref!(config, "attestation_server_url", attestation_server_url);
        get_config_ref!(config, "attestation_verifier_url", attestation_verifier_url);
        get_config_ref!(config, "enclave_client_url", enclave_client_url);
        get_config_ref!(config, "market_id", market_id);

        let (generator_registry, private_key_signer) = get_generator_registry_instance(
            private_key,
            chain_id,
            generator_registry_address,
            rpc_url,
        )?;

        let market_id = U256::from_dec_str(market_id.as_str())
            .map_err(|e| format!("Invalid Market Id: {}", e))?;

        let (proof_marketplace, _) =
            get_proof_marketplace_instance_without_signer(proof_marketplace_address, rpc_url)?;

        Ok(UpdateEncryptionKeyInfo {
            private_key_signer,
            generator_registry,
            attestation_utility: attestation_server_url.to_string(),
            attestation_verifier: attestation_verifier_url.to_string(),
            enclave_client_url: enclave_client_url.to_string(),
            market_id,
            proof_marketplace,
        })
    }
}

pub type AddIvsKeyInfo = UpdateEncryptionKeyInfo;
impl CommonDeps {
    pub fn add_ivs_key_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<AddIvsKeyInfo, String> {
        CommonDeps::update_encryption_info(config)
    }
}

pub struct ReadProofInfo {
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<Provider<Http>>,
    #[allow(unused)]
    pub provider_http: Provider<Http>,
    pub ask_id: U256,
    pub indexer_url: String,
}

impl CommonDeps {
    pub fn read_proof_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<ReadProofInfo, String> {
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "ask_id", ask_id);
        get_config_ref!(config, "indexer_url", indexer_url);

        let ask_id =
            U256::from_dec_str(&ask_id.as_str()).map_err(|e| format!("Invalid Ask Id: {}", e))?;

        let (proof_marketplace, provider_http) =
            get_proof_marketplace_instance_without_signer(proof_marketplace_address, rpc_url)?;

        Ok(ReadProofInfo {
            proof_marketplace,
            provider_http,
            ask_id,
            indexer_url: indexer_url.to_string(),
        })
    }
}

fn get_proof_marketplace_instance_without_signer(
    proof_marketplace_address: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::proof_marketplace::ProofMarketplace<Provider<Http>>,
        Provider<Http>,
    ),
    String,
> {
    let proof_marketplace_address = proof_marketplace_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid Proof Marketplace address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let proof_marketplace = bindings::proof_marketplace::ProofMarketplace::new(
        proof_marketplace_address,
        provider_http.clone().into(),
    );

    Ok((proof_marketplace, provider_http))
}

pub struct ConfidentialRequest {
    pub private_key_signer: LocalWallet,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub payment_token: bindings::ierc20::IERC20<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub max_proof_generation_cost: U256,
    pub max_proof_generation_time: U256,
    pub inputs: ethers::types::Bytes,
    pub private_inputs: ethers::types::Bytes,
    pub market_id: U256,
    pub entity_registry: bindings::entity_key_registry::EntityKeyRegistry<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub provider_http: Provider<Http>,
    pub kalypso_rpc_url: String,
}

impl CommonDeps {
    pub fn confidential_request_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<ConfidentialRequest, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "payment_token", payment_token);
        get_config_ref!(config, "inputs", inputs);
        get_config_ref!(config, "market_id", market_id);
        get_config_ref!(config, "private_inputs", private_inputs);
        get_config_ref!(config, "entity_registry", entity_key_registry_address);

        get_config_ref!(
            config,
            "max_proof_generation_cost",
            max_proof_generation_cost
        );
        get_config_ref!(
            config,
            "max_proof_generation_time",
            max_proof_generation_time
        );

        let (proof_marketplace, private_key_signer) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        let (payment_token, _) =
            get_token_instance(private_key, chain_id, &payment_token, rpc_url)?;

        let max_proof_generation_cost = U256::from_dec_str(max_proof_generation_cost.as_str())
            .map_err(|e| format!("Max Proof Generation Cost: {}", e))?;

        let max_proof_generation_time = U256::from_dec_str(max_proof_generation_time.as_str())
            .map_err(|e| format!("Max Proof Generation Time: {}", e))?;

        let inputs = {
            let trimmed_inputs = if inputs.starts_with("0x") || inputs.starts_with("0X") {
                &inputs[2..]
            } else {
                inputs
            };
            if trimmed_inputs.len() % 2 != 0 {
                return Err("Hex string has an invalid length".to_string());
            }

            let inputs = hex::decode(trimmed_inputs)
                .map_err(|e| format!("Invalid Input Bytes: {}", e))?
                .into();

            inputs
        };

        let private_inputs = {
            let trimmed_inputs =
                if private_inputs.starts_with("0x") || private_inputs.starts_with("0X") {
                    &private_inputs[2..]
                } else {
                    private_inputs
                };
            if trimmed_inputs.len() % 2 != 0 {
                return Err("Hex string has an invalid length".to_string());
            }

            let private_inputs = hex::decode(trimmed_inputs)
                .map_err(|e| format!("Invalid Input Bytes: {}", e))?
                .into();

            private_inputs
        };

        let market_id = U256::from_dec_str(market_id.as_str())
            .map_err(|e| format!("Invalid Market Id: {}", e))?;

        let (entity_registry, _) = get_entity_key_registry_instance(
            private_key,
            chain_id,
            entity_key_registry_address,
            rpc_url,
        )?;

        let provider_http =
            Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

        Ok(ConfidentialRequest {
            private_key_signer,
            proof_marketplace,
            payment_token,
            max_proof_generation_cost,
            max_proof_generation_time,
            inputs,
            market_id,
            private_inputs,
            entity_registry,
            provider_http,
            kalypso_rpc_url: rpc_url.clone(),
        })
    }
}

fn get_entity_key_registry_instance(
    private_key: &str,
    chain_id: &str,
    entity_key_registry_address: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::entity_key_registry::EntityKeyRegistry<
            SignerMiddleware<Provider<Http>, LocalWallet>,
        >,
        LocalWallet,
    ),
    String,
> {
    // Parse the private key into a LocalWallet and set the chain ID
    let private_key_signer = private_key
        .parse::<LocalWallet>()
        .map_err(|e| format!("Failed to parse private key: {}", e))?
        .with_chain_id(
            chain_id
                .parse::<u64>()
                .map_err(|e| format!("Invalid chain_id: {}", e))?,
        );

    // Parse the Generator Registry address
    let entity_key_registry_address = entity_key_registry_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid Entity Key Registry address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the SignerMiddleware with the provider and signer
    let client = SignerMiddleware::new(provider_http.clone(), private_key_signer.clone());

    let client_arc = Arc::new(client);

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let entity_key_registry = bindings::entity_key_registry::EntityKeyRegistry::new(
        entity_key_registry_address,
        client_arc.clone(),
    );

    Ok((entity_key_registry, private_key_signer))
}

pub struct SymbioticOptInInfo {
    pub signer: LocalWallet,
    pub symbiotic_rpc_url: String,
    pub vault_opt_in_service: Address,
    pub network_opt_in_service: Address,
    pub vault_address: Address,
    pub network_address: Address,
}

impl CommonDeps {
    pub fn symbiotic_opt_in_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<SymbioticOptInInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "symbiotic_rpc_url", symbiotic_rpc_url);
        get_config_ref!(config, "symbiotic_chain_id", symbiotic_chain_id);
        get_config_ref!(config, "vault_opt_in_service", vault_opt_in_service_address);
        get_config_ref!(
            config,
            "network_opt_in_service",
            network_opt_in_service_address
        );

        get_config_ref!(config, "vault_address", vault_address);
        get_config_ref!(config, "network_address", network_address);

        let signer = private_key
            .parse::<LocalWallet>()
            .map_err(|e| format!("Failed to parse private key: {}", e))?
            .with_chain_id(
                symbiotic_chain_id
                    .parse::<u64>()
                    .map_err(|e| format!("Invalid symbiotic_chain_id: {}", e))?,
            );

        let vault_opt_in_service = vault_opt_in_service_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Vault Opt In Service Address: {}", e))?;

        let network_opt_in_service = network_opt_in_service_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Network Opt In Service Address: {}", e))?;

        let vault_address = vault_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Vault Address: {}", e))?;

        let network_address = network_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Network Address: {}", e))?;

        Ok(SymbioticOptInInfo {
            signer,
            symbiotic_rpc_url: symbiotic_rpc_url.into(),
            vault_opt_in_service,
            network_opt_in_service,
            vault_address,
            network_address,
        })
    }
}

pub struct LoadGeneratorConfigInfo {
    pub config: generator_client::model::GeneratorConfigSetupRequestBody,
    pub generator_client_url: String,
}

impl CommonDeps {
    pub fn load_operator_config_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<LoadGeneratorConfigInfo, String> {
        get_config_ref!(config, "generator_client_url", generator_client_url);
        get_config_ref!(config, "operator_address", operator_address);
        get_config_ref!(config, "market_id", market_id);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "generator_registry", generator_registry_address);
        get_config_ref!(config, "start_block", start_block);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "payment_token", payment_token);
        get_config_ref!(config, "staking_token", staking_token);
        get_config_ref!(config, "attestation_verifier", attestation_verifier_address);
        get_config_ref!(config, "entity_registry", entity_key_registry_address);
        get_config_ref!(config, "internal_prover_port", internal_prover_port);
        get_config_ref!(config, "input_verification_url", input_verification_url);

        let operator_address = operator_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Operator Address: {}", e))?;

        let market_id = U256::from_dec_str(market_id.as_str())
            .map_err(|e| format!("Invalid Market Id: {}", e))?;

        let proof_marketplace_address = proof_marketplace_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Proof Marketplace address: {}", e))?;

        let generator_registry_address = generator_registry_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Generator Registry address: {}", e))?;

        let start_block = U256::from_dec_str(&start_block.as_str())
            .map_err(|e| format!("Invalid Start Block: {}", e))?;

        let chain_id = U256::from_dec_str(&&chain_id.as_str())
            .map_err(|e| format!("Invalid Chain ID: {}", e))?;

        let payment_token = payment_token
            .parse::<Address>()
            .map_err(|e| format!("Invalid Payment Token Address: {}", e))?;

        let staking_token = staking_token
            .parse::<Address>()
            .map_err(|e| format!("Invalid Staking Token Address: {}", e))?;

        let attestation_verifier_address = attestation_verifier_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Attestation Verifier Address: {}", e))?;

        let entity_key_registry_address = entity_key_registry_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Entity Key Registry Address: {}", e))?;

        let internal_prover_port = U256::from_dec_str(&&internal_prover_port.as_str())
            .map_err(|e| format!("Internal Prover Port: {}", e))?;

        Ok(LoadGeneratorConfigInfo {
            config: generator_client::model::GeneratorConfigSetupRequestBody {
                generator_config: Some(vec![
                    generator_client::model::SetupRequestBodyGeneratorConfig {
                        address: Some(format!(
                            "0x{}",
                            hex::encode(operator_address.as_bytes().to_vec())
                        )),
                        data: Some("Some Data".into()),
                        supported_markets: Some(vec![market_id.to_string()]),
                    },
                ]),
                runtime_config: Some(generator_client::model::SetupRequestBodyRuntimeConfig {
                    ws_url: Some("wss:://not_used_so_using_some_dummy_value_to_pass".into()),
                    http_url: Some(rpc_url.to_string()),
                    private_key: Some(private_key.to_string()),
                    proof_market_place: Some(format!(
                        "0x{}",
                        hex::encode(proof_marketplace_address.as_bytes().to_vec())
                    )),
                    generator_registry: Some(format!(
                        "0x{}",
                        hex::encode(generator_registry_address.as_bytes().to_vec())
                    )),
                    start_block: Some(start_block.as_u32() as i32),
                    chain_id: Some(chain_id.as_u32() as i32),
                    payment_token: Some(format!(
                        "0x{}",
                        hex::encode(payment_token.as_bytes().to_vec())
                    )),
                    staking_token: Some(format!(
                        "0x{}",
                        hex::encode(staking_token.as_bytes().to_vec())
                    )),
                    attestation_verifier: Some(format!(
                        "0x{}",
                        hex::encode(attestation_verifier_address.as_bytes().to_vec())
                    )),
                    entity_registry: Some(format!(
                        "0x{}",
                        hex::encode(entity_key_registry_address.as_bytes().to_vec())
                    )),
                    markets: {
                        let mut markets = std::collections::HashMap::new();
                        // add market details and other info related to it here
                        markets.insert(
                            market_id.to_string(),
                            generator_client::model::MarketDetails {
                                port: internal_prover_port.to_string(),
                                ivs_url: input_verification_url.to_string(),
                            },
                        );
                        markets
                    },
                }),
            },
            generator_client_url: generator_client_url.to_string(),
        })
    }
}

pub struct ProgramInfo {
    pub generator_client_url: String,
    pub prover_program_name: String,
}

impl CommonDeps {
    pub fn start_program_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<ProgramInfo, String> {
        get_config_ref!(config, "generator_client_url", generator_client_url);
        get_config_ref!(config, "prover_program_name", prover_program_name);

        Ok(ProgramInfo {
            generator_client_url: generator_client_url.to_string(),
            prover_program_name: prover_program_name.to_string(),
        })
    }
}

pub struct TestEnclaveConnectionInfo {
    pub enclave_client_url: String,
}

impl CommonDeps {
    pub fn test_connection_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<TestEnclaveConnectionInfo, String> {
        get_config_ref!(config, "enclave_client_url", enclave_client_url);

        Ok(TestEnclaveConnectionInfo {
            enclave_client_url: enclave_client_url.to_string(),
        })
    }
}

pub struct BenchmarkInfo {
    pub benchmark_url: String,
}

impl CommonDeps {
    pub fn benchmark_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<BenchmarkInfo, String> {
        get_config_ref!(config, "benchmark_url", benchmark_url);

        Ok(BenchmarkInfo {
            benchmark_url: benchmark_url.to_string(),
        })
    }
}

pub struct SymbioticOperatorRegisterInfo {
    pub signer: LocalWallet,
    pub symbiotic_rpc_url: String,
    pub symbiotic_operator_registry: Address,
}

impl CommonDeps {
    pub fn symbiotic_operator_registry_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<SymbioticOperatorRegisterInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "symbiotic_rpc_url", symbiotic_rpc_url);
        get_config_ref!(config, "symbiotic_chain_id", symbiotic_chain_id);
        get_config_ref!(
            config,
            "symbiotic_operator_registry",
            symbiotic_operator_registry_address
        );

        let symbiotic_operator_registry = symbiotic_operator_registry_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Symbiotic Operator Registry Address: {}", e))?;

        let signer = private_key
            .parse::<LocalWallet>()
            .map_err(|e| format!("Failed to parse private key: {}", e))?
            .with_chain_id(
                symbiotic_chain_id
                    .parse::<u64>()
                    .map_err(|e| format!("Invalid symbiotic_chain_id: {}", e))?,
            );

        Ok(SymbioticOperatorRegisterInfo {
            signer,
            symbiotic_rpc_url: symbiotic_rpc_url.clone(),
            symbiotic_operator_registry,
        })
    }
}

pub struct UpdateMarketMetadataInfo {
    #[allow(unused)]
    pub private_key_signer: LocalWallet,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub market_id: U256,
}

impl CommonDeps {
    pub fn update_market_metadata_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<UpdateMarketMetadataInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "market_id", market_id);

        let market_id = U256::from_dec_str(market_id.as_str())
            .map_err(|e| format!("Invalid Market Id: {}", e))?;

        let (proof_marketplace, local_wallet) = update_proof_marketplace_metadata_patch_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        Ok(UpdateMarketMetadataInfo {
            private_key_signer: local_wallet,
            proof_marketplace,
            market_id,
        })
    }
}

fn update_proof_marketplace_metadata_patch_instance(
    private_key: &str,
    chain_id: &str,
    proof_marketplace_address: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::proof_marketplace::ProofMarketplace<
            SignerMiddleware<Provider<Http>, LocalWallet>,
        >,
        LocalWallet,
    ),
    String,
> {
    // Parse the private key into a LocalWallet and set the chain ID
    let private_key_signer = private_key
        .parse::<LocalWallet>()
        .map_err(|e| format!("Failed to parse private key: {}", e))?
        .with_chain_id(
            chain_id
                .parse::<u64>()
                .map_err(|e| format!("Invalid chain_id: {}", e))?,
        );

    // Parse the Generator Registry address
    let proof_marketplace_address = proof_marketplace_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid Proof Marketplace address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the SignerMiddleware with the provider and signer
    let client = SignerMiddleware::new(provider_http.clone(), private_key_signer.clone());

    let client_arc = Arc::new(client);

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let proof_marketplace =
        bindings::ProofMarketplace::new(proof_marketplace_address, client_arc.clone());

    Ok((proof_marketplace, private_key_signer))
}

pub struct UpdateGeneratorMetaInfo {
    pub generator_registry:
        bindings::prover_manager::ProverManager<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub read_generator_registry:
        bindings::prover_manager::ProverManager<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub private_key_signer: LocalWallet,
}

impl CommonDeps {
    pub fn update_generator_meta_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<UpdateGeneratorMetaInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "generator_registry", generator_registry_address);

        let (generator_registry, private_key_signer) = update_generator_meta_instance(
            private_key,
            chain_id,
            generator_registry_address,
            rpc_url,
        )?;

        let (read_generator_registry, _) = get_generator_registry_instance(
            private_key,
            chain_id,
            generator_registry_address,
            rpc_url,
        )?;

        Ok(UpdateGeneratorMetaInfo {
            generator_registry,
            private_key_signer,
            read_generator_registry,
        })
    }
}

fn update_generator_meta_instance(
    private_key: &str,
    chain_id: &str,
    generator_registry_address: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::prover_manager::ProverManager<SignerMiddleware<Provider<Http>, LocalWallet>>,
        LocalWallet,
    ),
    String,
> {
    // Parse the private key into a LocalWallet and set the chain ID
    let private_key_signer = private_key
        .parse::<LocalWallet>()
        .map_err(|e| format!("Failed to parse private key: {}", e))?
        .with_chain_id(
            chain_id
                .parse::<u64>()
                .map_err(|e| format!("Invalid chain_id: {}", e))?,
        );

    // Parse the Generator Registry address
    let generator_registry_address = generator_registry_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid Generator Registry address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the SignerMiddleware with the provider and signer
    let client = SignerMiddleware::new(provider_http.clone(), private_key_signer.clone());

    let client_arc = Arc::new(client);

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let generator_registry = bindings::prover_manager::ProverManager::new(
        generator_registry_address,
        client_arc.clone(),
    );

    Ok((generator_registry, private_key_signer))
}

pub struct ReadStakeDataInfo {
    pub operator_address: Address,
    pub indexer_url: String,
}

impl CommonDeps {
    pub fn read_stake_data_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<ReadStakeDataInfo, String> {
        get_config_ref!(config, "indexer_url", indexer_url);
        get_config_ref!(config, "operator_address", operator_address);

        let operator_address = operator_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Operator Address: {}", e))?;

        Ok(ReadStakeDataInfo {
            operator_address,
            indexer_url: indexer_url.to_string(),
        })
    }
}

pub struct SetOperatorCommission {
    pub private_key_signer: LocalWallet,
    pub rpc_url: String,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub operator_commission: U256,
}

impl CommonDeps {
    pub fn set_operator_commission_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<SetOperatorCommission, String> {
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "chain_id", chain_id);

        let (proof_marketplace, private_key_signer) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        get_config_ref!(config, "operator_commission", operator_commission);

        let operator_commission = U256::from_dec_str(&operator_commission.as_str())
            .map_err(|e| format!("Invalid Operator Commission: {}", e))?;

        Ok(SetOperatorCommission {
            private_key_signer,
            proof_marketplace,
            operator_commission,
            rpc_url: rpc_url.to_string(),
        })
    }
}

pub struct ReadRewardInfo {
    #[allow(unused)]
    pub operator: Address,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<Provider<Http>>,
}

impl CommonDeps {
    pub fn read_rewards_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<ReadRewardInfo, String> {
        get_config_ref!(config, "operator_address", operator_address);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);

        let (proof_marketplace, _) =
            get_proof_marketplace_instance_without_signer(proof_marketplace_address, rpc_url)?;

        let operator_address = operator_address
            .parse::<Address>()
            .map_err(|e| format!("Operator Address: {}", e))?;

        Ok(ReadRewardInfo {
            operator: operator_address,
            proof_marketplace,
        })
    }
}

pub struct RequestStakeWithdrawal {
    pub private_key_signer: LocalWallet,
    pub native_staking:
        bindings::native_staking::NativeStaking<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub staking_token: bindings::ierc20::IERC20<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub staking_amount: U256,
}

impl CommonDeps {
    pub fn request_stake_withdrawal_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<RequestStakeWithdrawal, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "native_staking", native_staking_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "staking_token", staking_token);
        get_config_ref!(config, "staking_amount", staking_amount);

        let (staking_token, _) =
            get_token_instance(private_key, chain_id, &staking_token, rpc_url)?;

        let (native_staking, private_key_signer) =
            get_native_staking_instance(private_key, chain_id, native_staking_address, rpc_url)?;

        let staking_amount = U256::from_dec_str(staking_amount.as_str())
            .map_err(|e| format!("Invalid Unstaking Amount: {}", e))?;

        Ok(RequestStakeWithdrawal {
            private_key_signer,
            native_staking,
            staking_token,
            staking_amount,
        })
    }
}

pub struct NativeStakeWithdrawal {
    pub private_key_signer: LocalWallet,
    pub native_staking:
        bindings::native_staking::NativeStaking<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub indexer_url: String,
}

impl CommonDeps {
    pub fn native_staking_withdrawal_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<NativeStakeWithdrawal, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "native_staking", native_staking_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "indexer_url", indexer_url);

        let (native_staking, private_key_signer) =
            get_native_staking_instance(private_key, chain_id, native_staking_address, rpc_url)?;

        Ok(NativeStakeWithdrawal {
            private_key_signer,
            native_staking,
            indexer_url: indexer_url.to_string(),
        })
    }
}

pub struct MatchingEngineProgram {
    pub matching_engine_client_url: String,
    pub matching_engine_attestation_utility: String,
    pub matching_engine_pcrs: Vec<u8>,
    pub chain_id: U64,
}

impl CommonDeps {
    pub fn start_matching_engine_program_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<MatchingEngineProgram, String> {
        get_config_ref!(config, "chain_id", chain_id);

        get_config_ref!(
            config,
            "matching_engine_client_url",
            matching_engine_client_url
        );

        get_config_ref!(
            config,
            "matching_engine_attestation_utility",
            matching_engine_attestation_utility
        );
        get_config_ref!(config, "matching_engine_image_id", matching_engine_pcrs);

        let matching_engine_pcrs = {
            let trimmed_key = if matching_engine_pcrs.starts_with("0x")
                || matching_engine_pcrs.starts_with("0X")
            {
                &matching_engine_pcrs[2..]
            } else {
                matching_engine_pcrs
            };
            hex::decode(trimmed_key).map_err(|e| format!("Invalid Prover PCRs: {}", e))?
        };

        Ok(MatchingEngineProgram {
            matching_engine_client_url: matching_engine_client_url.to_string(),
            matching_engine_attestation_utility: matching_engine_attestation_utility.to_string(),
            matching_engine_pcrs,
            chain_id: U64::from_str(&chain_id).unwrap(),
        })
    }
}

pub struct SetMiddlewareAddressInfo {
    pub signer: LocalWallet,
    pub symbiotic_rpc_url: String,
    pub middleware_address: Address,
    pub operator_address: Address,
}

impl CommonDeps {
    pub fn set_middleware_address_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<SetMiddlewareAddressInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "symbiotic_rpc_url", symbiotic_rpc_url);
        get_config_ref!(config, "symbiotic_chain_id", symbiotic_chain_id);
        get_config_ref!(config, "middleware_service", middleware_address);
        get_config_ref!(config, "operator_address", operator_address);

        let operator_address = operator_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Operator Address: {}", e))?;

        let middleware_address = middleware_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Middleware Address: {}", e))?;

        let signer = private_key
            .parse::<LocalWallet>()
            .map_err(|e| format!("Failed to parse private key: {}", e))?
            .with_chain_id(
                symbiotic_chain_id
                    .parse::<u64>()
                    .map_err(|e| format!("Invalid symbiotic_chain_id: {}", e))?,
            );

        Ok(SetMiddlewareAddressInfo {
            signer,
            symbiotic_rpc_url: symbiotic_rpc_url.into(),
            middleware_address,
            operator_address,
        })
    }
}

pub struct MatchingEngineImageInfo {
    pub matching_engine_pcrs: Vec<u8>,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub signer: LocalWallet,
}

impl CommonDeps {
    pub fn matching_engine_image_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<MatchingEngineImageInfo, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "chain_id", chain_id);

        let (proof_marketplace, signer) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        get_config_ref!(config, "matching_engine_image_id", matching_engine_pcrs);

        let matching_engine_pcrs = {
            let trimmed_key = if matching_engine_pcrs.starts_with("0x")
                || matching_engine_pcrs.starts_with("0X")
            {
                &matching_engine_pcrs[2..]
            } else {
                matching_engine_pcrs
            };
            hex::decode(trimmed_key).map_err(|e| format!("Invalid Prover PCRs: {}", e))?
        };

        Ok(MatchingEngineImageInfo {
            matching_engine_pcrs,
            proof_marketplace,
            signer,
        })
    }
}

pub struct VerifyMatchingEngineKeysInfo {
    pub matching_engine_client_url: String,
    pub matching_engine_attestation_utility: String,
    pub matching_engine_pcrs: Vec<u8>,
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub signer: LocalWallet,
    pub attestation_verifier: String,
    #[allow(unused)]
    pub chain_id: U64,
}

impl CommonDeps {
    pub fn verify_matching_engine_keys_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<VerifyMatchingEngineKeysInfo, String> {
        get_config_ref!(config, "chain_id", chain_id);

        get_config_ref!(
            config,
            "matching_engine_client_url",
            matching_engine_client_url
        );

        get_config_ref!(
            config,
            "matching_engine_attestation_utility",
            matching_engine_attestation_utility
        );
        get_config_ref!(config, "matching_engine_image_id", matching_engine_pcrs);

        get_config_ref!(config, "attestation_verifier_url", attestation_verifier_url);

        let matching_engine_pcrs = {
            let trimmed_key = if matching_engine_pcrs.starts_with("0x")
                || matching_engine_pcrs.starts_with("0X")
            {
                &matching_engine_pcrs[2..]
            } else {
                matching_engine_pcrs
            };
            hex::decode(trimmed_key).map_err(|e| format!("Invalid Prover PCRs: {}", e))?
        };

        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);

        let (proof_marketplace, signer) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        Ok(VerifyMatchingEngineKeysInfo {
            matching_engine_client_url: matching_engine_client_url.to_string(),
            matching_engine_attestation_utility: matching_engine_attestation_utility.to_string(),
            matching_engine_pcrs,
            chain_id: U64::from_str(&chain_id).unwrap(),
            proof_marketplace,
            signer,
            attestation_verifier: attestation_verifier_url.to_string(),
        })
    }
}

pub struct VerifyRemoteAttestationInfo {
    pub attestation_utility: String,
    pub enclave_pcrs: Vec<u8>,
}

impl CommonDeps {
    pub fn verify_remote_attestation_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<VerifyRemoteAttestationInfo, String> {
        get_config_ref!(config, "attestation_server_url", attestation_server_url);
        get_config_ref!(config, "enclave_image_id", enclave_image_id);

        let enclave_image_id = {
            let trimmed_key =
                if enclave_image_id.starts_with("0x") || enclave_image_id.starts_with("0X") {
                    &enclave_image_id[2..]
                } else {
                    enclave_image_id
                };
            hex::decode(trimmed_key).map_err(|e| format!("Invalid PCRs: {}", e))?
        };

        Ok(VerifyRemoteAttestationInfo {
            attestation_utility: attestation_server_url.to_string(),
            enclave_pcrs: enclave_image_id,
        })
    }
}

pub struct RoleManagementCheck {
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub generator_registry:
        bindings::prover_manager::ProverManager<SignerMiddleware<Provider<Http>, LocalWallet>>,

    pub entity_registry: bindings::entity_key_registry::EntityKeyRegistry<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,

    pub symbiotic_staking: bindings::symbiotic_staking::SymbioticStaking<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,

    pub staking_manager:
        bindings::staking_manager::StakingManager<SignerMiddleware<Provider<Http>, LocalWallet>>,

    pub native_staking:
        bindings::native_staking::NativeStaking<SignerMiddleware<Provider<Http>, LocalWallet>>,

    pub symbiotic_staking_reward: bindings::symbiotic_staking_reward::SymbioticStakingReward<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,

    pub address_to_check_role_for: Address,
}

impl CommonDeps {
    pub fn role_management_check_info(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<RoleManagementCheck, String> {
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "generator_registry", generator_registry_address);
        get_config_ref!(config, "proof_marketplace", proof_marketplace_address);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "entity_registry", entity_key_registry_address);
        get_config_ref!(config, "symbiotic_staking", symbiotic_staking);
        get_config_ref!(config, "native_staking", native_staking_address);
        get_config_ref!(config, "staking_manager", staking_manager);
        get_config_ref!(config, "symbiotic_staking_reward", symbiotic_staking_reward);

        get_config_ref!(
            config,
            "address_to_check_role_for",
            address_to_check_role_for
        );

        let address_to_check_role_for = address_to_check_role_for
            .parse::<Address>()
            .map_err(|e| format!("Invalid Address: {}", e))?;

        // some random 64 chars
        let private_key = "aaaabbbbaaaabbbbaaaabbbbaaaabbbbaaaabbbbaaaabbbbaaaabbbbaaaabbbb";

        let (generator_registry, _) = get_generator_registry_instance(
            private_key,
            chain_id,
            generator_registry_address,
            rpc_url,
        )?;

        let (proof_marketplace, _) = get_proof_marketplace_instance(
            private_key,
            chain_id,
            proof_marketplace_address,
            rpc_url,
        )?;

        let (entity_registry, _) = get_entity_key_registry_instance(
            private_key,
            chain_id,
            entity_key_registry_address,
            rpc_url,
        )?;

        let (symbiotic_staking, _) =
            get_symbiotic_instance(private_key, chain_id, symbiotic_staking, rpc_url)?;

        let (native_staking, _) =
            get_native_staking_instance(private_key, chain_id, native_staking_address, rpc_url)?;

        let (staking_manager, _) =
            get_staking_manager(private_key, chain_id, staking_manager, rpc_url)?;

        let (symbiotic_staking_reward, _) =
            get_symbiotic_staking_reward(private_key, chain_id, symbiotic_staking_reward, rpc_url)?;

        Ok(RoleManagementCheck {
            proof_marketplace,
            generator_registry,
            entity_registry,
            address_to_check_role_for,
            symbiotic_staking,
            native_staking,
            staking_manager,
            symbiotic_staking_reward,
        })
    }
}

fn get_symbiotic_instance(
    private_key: &str,
    chain_id: &str,
    symbiotic_staking_address: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::symbiotic_staking::SymbioticStaking<
            SignerMiddleware<Provider<Http>, LocalWallet>,
        >,
        LocalWallet,
    ),
    String,
> {
    // Parse the private key into a LocalWallet and set the chain ID
    let private_key_signer = private_key
        .parse::<LocalWallet>()
        .map_err(|e| format!("Failed to parse private key: {}", e))?
        .with_chain_id(
            chain_id
                .parse::<u64>()
                .map_err(|e| format!("Invalid chain_id: {}", e))?,
        );

    // Parse the Generator Registry address
    let symbiotic_staking_address = symbiotic_staking_address
        .parse::<Address>()
        .map_err(|e| format!("Invalid Symbiotic Staking address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the SignerMiddleware with the provider and signer
    let client = SignerMiddleware::new(provider_http.clone(), private_key_signer.clone());

    let client_arc = Arc::new(client);

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let symbiotic_staking = bindings::symbiotic_staking::SymbioticStaking::new(
        symbiotic_staking_address,
        client_arc.clone(),
    );

    Ok((symbiotic_staking, private_key_signer))
}

fn get_staking_manager(
    private_key: &str,
    chain_id: &str,
    stake_manager: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::staking_manager::StakingManager<SignerMiddleware<Provider<Http>, LocalWallet>>,
        LocalWallet,
    ),
    String,
> {
    // Parse the private key into a LocalWallet and set the chain ID
    let private_key_signer = private_key
        .parse::<LocalWallet>()
        .map_err(|e| format!("Failed to parse private key: {}", e))?
        .with_chain_id(
            chain_id
                .parse::<u64>()
                .map_err(|e| format!("Invalid chain_id: {}", e))?,
        );

    // Parse the Generator Registry address
    let stake_manager = stake_manager
        .parse::<Address>()
        .map_err(|e| format!("Invalid Staking Manager address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the SignerMiddleware with the provider and signer
    let client = SignerMiddleware::new(provider_http.clone(), private_key_signer.clone());

    let client_arc = Arc::new(client);

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let stake_manager =
        bindings::staking_manager::StakingManager::new(stake_manager, client_arc.clone());

    Ok((stake_manager, private_key_signer))
}

fn get_symbiotic_staking_reward(
    private_key: &str,
    chain_id: &str,
    symbiotic_staking_reward: &str,
    rpc_url: &str,
) -> Result<
    (
        bindings::symbiotic_staking_reward::SymbioticStakingReward<
            SignerMiddleware<Provider<Http>, LocalWallet>,
        >,
        LocalWallet,
    ),
    String,
> {
    // Parse the private key into a LocalWallet and set the chain ID
    let private_key_signer = private_key
        .parse::<LocalWallet>()
        .map_err(|e| format!("Failed to parse private key: {}", e))?
        .with_chain_id(
            chain_id
                .parse::<u64>()
                .map_err(|e| format!("Invalid chain_id: {}", e))?,
        );

    // Parse the Generator Registry address
    let symbiotic_staking_reward = symbiotic_staking_reward
        .parse::<Address>()
        .map_err(|e| format!("Invalid Symbiotic Staking Reward address: {}", e))?;

    // Initialize the provider
    let provider_http =
        Provider::<Http>::try_from(rpc_url).map_err(|e| format!("Invalid RPC URL: {}", e))?;

    // Initialize the SignerMiddleware with the provider and signer
    let client = SignerMiddleware::new(provider_http.clone(), private_key_signer.clone());

    let client_arc = Arc::new(client);

    // Initialize the Generator Registry contract instance with the signer-enabled client
    let symbiotic_staking_reward = bindings::symbiotic_staking_reward::SymbioticStakingReward::new(
        symbiotic_staking_reward,
        client_arc.clone(),
    );

    Ok((symbiotic_staking_reward, private_key_signer))
}
