// src/common_deps.rs

use ethers::prelude::*;
use std::future::Future;
use std::sync::Arc;

macro_rules! get_config_ref {
    ($config:expr, $key:expr, $var:ident) => {
        let $var = $config.get($key).ok_or(concat!("Missing '", $key, "'"))?;
    };
}

/// Struct holding common dependencies required by multiple operations
pub struct GeneratorRegister {
    pub private_key_signer: LocalWallet,
    pub generator_registry: bindings::generator_registry::GeneratorRegistry<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub reward_address: Address,
    pub declared_compute: U256,
}

pub struct GeneratorJoinMarket {
    pub private_key_signer: LocalWallet,
    pub generator_registry: bindings::generator_registry::GeneratorRegistry<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    #[allow(unused)] //will be used latter
    pub proof_marketplace: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub market_id: U256,
    pub compute_per_request_required: U256,
    pub proof_generation_cost: U256,
    pub proposed_time: U256,
}

pub struct StakeForGenerator {
    pub private_key_signer: LocalWallet,
    pub generator_registry: bindings::generator_registry::GeneratorRegistry<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    pub staking_token: bindings::ierc20::IERC20<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub staking_address: Address,
    pub stake: U256,
}

pub struct CommonDeps;

impl CommonDeps {
    pub async fn send_and_confirm<P, E>(
        send_future: impl Future<Output = Result<PendingTransaction<'_, P>, E>>,
    ) -> Result<String, String>
    where
        P: JsonRpcClient + Send + Sync + 'static,
        E: std::fmt::Display,
    {
        // Await the send operation
        let pending_tx = send_future
            .await
            .map_err(|e| format!("Failed to send transaction: {}", e))?;

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

    pub fn stake_for_generator(
        config: &std::collections::HashMap<String, String>,
    ) -> Result<StakeForGenerator, String> {
        get_config_ref!(config, "private_key", private_key);
        get_config_ref!(config, "rpc_url", rpc_url);
        get_config_ref!(config, "generator_registry", generator_registry_address);
        get_config_ref!(config, "staking_address", staking_address);
        get_config_ref!(config, "stake", stake);
        get_config_ref!(config, "chain_id", chain_id);
        get_config_ref!(config, "staking_token", staking_token);

        let (generator_registry, private_key_signer) = get_generator_registry_instance(
            private_key,
            chain_id,
            generator_registry_address,
            rpc_url,
        )?;

        let staking_address = staking_address
            .parse::<Address>()
            .map_err(|e| format!("Invalid Staking Address: {}", e))?;

        let stake =
            U256::from_dec_str(stake.as_str()).map_err(|e| format!("Invalid Stake: {}", e))?;

        let (staking_token, _) =
            get_token_instance(private_key, chain_id, &staking_token, rpc_url)?;

        Ok(StakeForGenerator {
            private_key_signer,
            generator_registry,
            staking_address,
            stake,
            staking_token,
        })
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
        bindings::generator_registry::GeneratorRegistry<
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
    let generator_registry = bindings::generator_registry::GeneratorRegistry::new(
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
    pub generator_registry: bindings::generator_registry::GeneratorRegistry<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
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
        get_config_ref!(config, "attestion_verifier_url", attestion_verifier_url);

        Ok(ComputePcrsInfo {
            attestation_utility: attestation_server_url.to_string(),
            attestation_verifier: attestion_verifier_url.to_string(),
        })
    }
}
