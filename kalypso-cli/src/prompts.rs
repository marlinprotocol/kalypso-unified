// src/prompts.rs
use crate::config::Config;
use dialoguer::{Input, Password};
use ethers::prelude::*; // Import all necessary types and traits
use std::collections::HashMap;
use std::error::Error;

/// Type alias for validation functions
type Validator = fn(&str) -> Result<(), String>;

/// Struct responsible for handling user prompts
pub struct Prompter<'a> {
    config: &'a Config,
    validators: HashMap<String, Validator>,
}

impl<'a> Prompter<'a> {
    /// Creates a new Prompter instance with optional validators
    pub fn new(config: &'a Config) -> Self {
        let mut validators: HashMap<String, Validator> = HashMap::new();

        // Register validators for specific fields
        validators.insert("rpc_url".to_string(), validate_rpc_url as Validator);
        validators.insert("private_key".to_string(), validate_private_key as Validator);
        validators.insert("market_id".to_string(), validate_dec_str_id as Validator);
        validators.insert("chain_id".to_string(), validate_dec_str_id as Validator);

        validators.insert(
            "generator_registry".to_string(),
            validate_eth_address as Validator,
        );

        validators.insert(
            "proof_marketplace".to_string(),
            validate_eth_address as Validator,
        );

        validators.insert(
            "staking_manager".to_string(),
            validate_eth_address as Validator,
        );

        validators.insert(
            "symbiotic_staking".to_string(),
            validate_eth_address as Validator,
        );

        validators.insert(
            "symbiotic_staking_reward".to_string(),
            validate_eth_address as Validator,
        );

        validators.insert(
            "address_to_check_role_for".to_string(),
            validate_eth_address,
        );
        validators.insert("reward_address".to_string(), validate_eth_address);
        validators.insert("declared_compute".to_string(), validate_dec_str_id);
        validators.insert("stake".to_string(), validate_dec_str_id);
        validators.insert("compute_per_request".to_string(), validate_dec_str_id);
        validators.insert("proposed_time".to_string(), validate_dec_str_id);

        validators.insert("staking_address".into(), validate_eth_address);
        validators.insert("staking_token".to_string(), validate_eth_address);
        validators.insert("payment_token".to_string(), validate_eth_address);

        validators.insert("prover_image_id".to_string(), validate_image_id);
        validators.insert("enclave_image_id".to_string(), validate_image_id);
        validators.insert("matching_engine_image_id".to_string(), validate_image_id);
        validators.insert("verification_image_id".to_string(), validate_image_id);
        validators.insert("verifier_wrapper".to_string(), validate_eth_address);
        validators.insert("attestation_server_url".to_string(), validate_rpc_url);
        validators.insert("attestation_verifier_url".to_string(), validate_rpc_url);
        validators.insert("native_staking".to_string(), validate_eth_address);
        validators.insert("operator_address".to_string(), validate_eth_address);
        validators.insert("max_proof_generation_cost".to_string(), validate_dec_str_id);
        validators.insert("inputs".to_string(), validate_inputs);
        validators.insert("max_proof_generation_time".to_string(), validate_dec_str_id);
        validators.insert("ask_id".to_string(), validate_dec_str_id);
        validators.insert("enclave_client_url".to_string(), validate_rpc_url);
        validators.insert("private_inputs".to_string(), validate_inputs);
        validators.insert("entity_registry".to_string(), validate_eth_address);
        validators.insert("display_name".to_string(), validate_string);
        validators.insert("display_description".to_string(), validate_string);
        validators.insert("website".to_string(), validate_string);
        validators.insert("twitter".to_string(), validate_string);
        validators.insert("operator_commission".to_string(), validate_dec_str_id);
        validators.insert("matching_engine_client_url".to_string(), validate_rpc_url);
        validators.insert(
            "matching_engine_attestation_utility".to_string(),
            validate_rpc_url,
        );

        validators.insert(
            "symbiotic_rpc_url".to_string(),
            validate_rpc_url as Validator,
        );
        validators.insert(
            "symbiotic_chain_id".to_string(),
            validate_dec_str_id as Validator,
        );
        validators.insert("vault_opt_in_service".to_string(), validate_eth_address);
        validators.insert("middleware_service".to_string(), validate_eth_address);
        validators.insert("network_opt_in_service".to_string(), validate_eth_address);
        validators.insert("vault_address".to_string(), validate_eth_address);
        validators.insert("network_address".to_string(), validate_eth_address);
        validators.insert("indexer_url".to_string(), validate_rpc_url);
        validators.insert("generator_client_url".to_string(), validate_rpc_url);
        validators.insert("gas_key".to_string(), validate_private_key as Validator);
        validators.insert("start_block".to_string(), validate_dec_str_id);
        validators.insert("attestation_verifier".to_string(), validate_eth_address);
        validators.insert("internal_prover_port".to_string(), validate_dec_str_id);
        validators.insert("input_verification_url".to_string(), validate_rpc_url);
        validators.insert("prover_program_name".to_string(), validate_string);
        validators.insert("benchmark_url".to_string(), validate_rpc_url);
        validators.insert(
            "symbiotic_operator_registry".to_string(),
            validate_eth_address,
        );

        validators.insert(
            "confirmation".to_string(),
            validate_confirmation as Validator,
        );

        validators.insert(
            "tee_verifier_wrapper_deployer".to_string(),
            validate_eth_address,
        );
        // Add other field validators as needed

        Prompter { config, validators }
    }

    /// Prompts the user based on the list of required prompt fields
    ///
    /// Returns a HashMap of field names to user inputs if all validations pass
    /// Returns an error string if any prompt fails validation
    pub fn prompt(
        &self,
        required_fields: &[String],
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut final_config = HashMap::new();

        for field in required_fields {
            if let Some(prompt_config) = self.config.get_prompt(field) {
                // Check if the environment variable is set
                if let Some(env_val) = self.config.get_env(&prompt_config.env_var) {
                    println!(
                        "Using {} from environment variable {}",
                        prompt_config.field, prompt_config.env_var
                    );
                    final_config.insert(prompt_config.field.clone(), env_val);
                } else {
                    // Prompt the user with optional validation
                    let input = if prompt_config.secret {
                        Password::new()
                            .with_prompt(format!(
                                "[env_var = {}] {}",
                                &prompt_config.env_var, &prompt_config.prompt
                            ))
                            .interact()
                            .map_err(|e| format!("Failed to read input for '{}': {}", field, e))?
                    } else {
                        Input::new()
                            .with_prompt(format!(
                                "[env_var = {}] {}",
                                &prompt_config.env_var, &prompt_config.prompt
                            ))
                            .interact_text()
                            .map_err(|e| format!("Failed to read input for '{}': {}", field, e))?
                    };

                    // Validate input if a validator exists
                    if let Some(validator) = self.validators.get(field) {
                        validator(&input)?;
                    }

                    final_config.insert(prompt_config.field.clone(), input);
                }
            } else {
                return Err(format!("Prompt configuration for '{}' not found.", field).into());
            }
        }

        Ok(final_config)
    }

    pub fn expect_in_env(
        &self,
        required_fields: &[String],
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut final_config = HashMap::new();

        for field in required_fields {
            if let Some(prompt_config) = self.config.get_prompt(field) {
                // Check if the environment variable is set
                if let Some(env_val) = self.config.get_env(&prompt_config.env_var) {
                    println!(
                        "Using {} from environment variable {}",
                        prompt_config.field, prompt_config.env_var
                    );
                    final_config.insert(prompt_config.field.clone(), env_val);
                } else {
                    return Err(format!(
                        "Not Found {} from environment variable {}",
                        prompt_config.field, prompt_config.env_var
                    )
                    .into());
                }
            } else {
                return Err(format!("Prompt configuration for '{}' not found.", field).into());
            }
        }

        Ok(final_config)
    }
}

use url::Url;

/// Validates whether the provided RPC URL is well-formed and uses the HTTP or HTTPS scheme.
///
/// # Arguments
///
/// * `url_str` - A string slice that holds the RPC URL to validate.
///
/// # Returns
///
/// * `Ok(())` if the URL is valid and uses HTTP or HTTPS.
/// * `Err(String)` with an error message if the URL is invalid.
fn validate_rpc_url(url_str: &str) -> Result<(), String> {
    // Attempt to parse the URL
    match Url::parse(url_str) {
        Ok(url) => {
            // Check if the scheme is either HTTP or HTTPS
            match url.scheme() {
                "http" | "https" => Ok(()),
                other => Err(format!(
                    "Unsupported URL scheme '{}'. Only 'http' and 'https' are allowed.",
                    other
                )),
            }
        }
        Err(e) => Err(format!(
            "Invalid RPC URL: {}. Please provide a valid URL.",
            e
        )),
    }
}

/// Validator for Ethereum Private Keys using the `ethers` crate
///
/// # Arguments
///
/// * `key` - A string slice that holds the Ethereum private key to validate
///
/// # Returns
///
/// * `Ok(())` if the private key is valid
/// * `Err(String)` with an error message if the private key is invalid
fn validate_private_key(key: &str) -> Result<(), String> {
    // Attempt to parse the private key using ethers::prelude::LocalWallet
    match key.parse::<LocalWallet>() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!(
            "Invalid Ethereum private key: {}. Please try again.",
            e
        )),
    }
}

fn validate_dec_str_id(key: &str) -> Result<(), String> {
    match U256::from_dec_str(key) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Invalid Market Id: {}. Please try again.", e)),
    }
}

/// Validator for Confirmation prompts
fn validate_confirmation(response: &str) -> Result<(), String> {
    let response_lower = response.to_lowercase();
    if response_lower == "yes" {
        Ok(())
    } else {
        Err("Confirmation not received. Operation canceled.".to_string())
    }
}

fn validate_eth_address(key: &str) -> Result<(), String> {
    match key.parse::<Address>() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!(
            "Invalid Ethereum Address: {}. Please try again.",
            e
        )),
    }
}

fn validate_inputs(key: &str) -> Result<(), String> {
    let trimmed_key = if key.starts_with("0x") || key.starts_with("0X") {
        &key[2..]
    } else {
        key
    };
    if trimmed_key.len() % 2 != 0 {
        return Err("Hex string has an invalid length".to_string());
    }

    hex::decode(trimmed_key).map_err(|e| format!("{}.{}", "Invalid Input Bytes".to_string(), e))?;

    Ok(())
}

fn validate_image_id(key: &str) -> Result<(), String> {
    let trimmed_key = if key.starts_with("0x") || key.starts_with("0X") {
        &key[2..]
    } else {
        key
    };
    if trimmed_key.len() % 2 != 0 {
        return Err("Hex string has an invalid length".to_string());
    }

    let pcrs = hex::decode(trimmed_key).map_err(|e| {
        format!(
            "{}. {}",
            "Invalid Image ID: Hex decoding failed".to_string(),
            e
        )
    })?;
    let _image_id =
        get_image_id_from_pcrs(&pcrs).map_err(|e| format!("Error computing image ID: {}", e))?;

    Ok(())
}

// --- try to find some lib functions for these below rather than writting your own ------------ //
use ethers::abi::{decode, ParamType, Token};

fn get_image_id_from_pcrs(pcrs: &[u8]) -> Result<[u8; 32], Box<dyn Error>> {
    // Define the expected types: three dynamic bytes arrays
    let types = vec![ParamType::Bytes, ParamType::Bytes, ParamType::Bytes];

    // Decode the input `pcrs` according to the specified types
    let tokens = decode(&types, pcrs)?;

    if tokens.len() != 3 {
        return Err("Expected three PCRs after decoding".into());
    }

    // Extract each PCR as a byte vector
    let pcr0 = match &tokens[0] {
        Token::Bytes(b) => b,
        _ => return Err("PCR0 is not of type bytes".into()),
    };

    let pcr1 = match &tokens[1] {
        Token::Bytes(b) => b,
        _ => return Err("PCR1 is not of type bytes".into()),
    };

    let pcr2 = match &tokens[2] {
        Token::Bytes(b) => b,
        _ => return Err("PCR2 is not of type bytes".into()),
    };

    // Compute the image ID using the inner function
    let image_id = get_image_id_from_pcrs_inner(pcr0, pcr1, pcr2)?;

    Ok(image_id)
}

use sha3::{Digest, Keccak256};

fn get_image_id_from_pcrs_inner(
    pcr0: &[u8],
    pcr1: &[u8],
    pcr2: &[u8],
) -> Result<[u8; 32], Box<dyn Error>> {
    // Initialize the Keccak-256 hasher
    let mut hasher = Keccak256::new();

    // Update the hasher with each PCR
    hasher.update(pcr0);
    hasher.update(pcr1);
    hasher.update(pcr2);

    // Finalize the hash computation
    let result = hasher.finalize();

    // Convert the hash result into a fixed-size array
    let image_id: [u8; 32] = result
        .as_slice()
        .try_into()
        .map_err(|e| format!("{}. {}", "Hash output is not 32 bytes", e))?;

    Ok(image_id)
}
// Add more validators as needed

fn validate_string(key: &str) -> Result<(), String> {
    if key.len() > 0 {
        return Ok(());
    }

    return Err("cannot be empty".into());
}
