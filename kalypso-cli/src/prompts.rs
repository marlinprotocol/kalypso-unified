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

        validators.insert("reward_address".to_string(), validate_eth_address);
        validators.insert("declared_compute".to_string(), validate_dec_str_id);
        validators.insert("stake".to_string(), validate_dec_str_id);
        validators.insert("compute_per_request".to_string(), validate_dec_str_id);
        validators.insert("proposed_time".to_string(), validate_dec_str_id);

        validators.insert("staking_address".into(), validate_eth_address);
        validators.insert("staking_token".to_string(), validate_eth_address);

        validators.insert(
            "confirmation".to_string(),
            validate_confirmation as Validator,
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
                        prompt_config.field.to_uppercase(),
                        prompt_config.env_var
                    );
                    final_config.insert(prompt_config.field.clone(), env_val);
                } else {
                    // Prompt the user with optional validation
                    let input = if prompt_config.secret {
                        Password::new()
                            .with_prompt(&prompt_config.prompt)
                            .interact()
                            .map_err(|e| format!("Failed to read input for '{}': {}", field, e))?
                    } else {
                        Input::new()
                            .with_prompt(&prompt_config.prompt)
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
// Add more validators as needed
