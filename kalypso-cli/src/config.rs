// src/config.rs

use serde::Deserialize;
use std::collections::HashMap;

/// Struct representing each prompt's configuration
#[derive(Debug, Deserialize)]
pub struct PromptConfig {
    pub field: String,
    pub prompt: String,
    pub secret: bool,
    pub env_var: String,
}

/// Struct representing each operation's configuration
#[derive(Debug, Deserialize)]
pub struct OperationConfig {
    pub name: String,
    pub description: String,
    pub required_prompts: Vec<String>,
}

/// Struct representing the overall configuration file
#[derive(Debug, Deserialize)]
struct ConfigFile {
    pub prompts: Vec<PromptConfig>,
    pub operations: Vec<OperationConfig>,
}

/// Main configuration struct
pub struct Config {
    pub prompts: HashMap<String, PromptConfig>,
    pub operations: Vec<OperationConfig>,
}

const CONFIG_JSON: &str = include_str!("config.json");

impl Config {
    /// Loads and parses the configuration from `config.json` and `.env`
    pub fn new() -> Self {
        dotenv::dotenv().ok(); // Load environment variables from .env

        let config_file: ConfigFile =
            serde_json::from_str(&CONFIG_JSON).expect("Invalid JSON in config.json");

        // Convert prompts to a HashMap for easy access by field name
        let prompts_map = config_file
            .prompts
            .into_iter()
            .map(|prompt| (prompt.field.clone(), prompt))
            .collect::<HashMap<String, PromptConfig>>();

        // Validate that all required prompts for each operation exist
        for operation in &config_file.operations {
            for prompt_field in &operation.required_prompts {
                if !prompts_map.contains_key(prompt_field) {
                    eprintln!(
                        "Error: Operation '{}' requires undefined prompt '{}'",
                        operation.name, prompt_field
                    );
                    std::process::exit(1);
                }
            }
        }

        Config {
            prompts: prompts_map,
            operations: config_file.operations,
        }
    }

    /// Retrieve a prompt configuration by field name
    pub fn get_prompt(&self, field: &str) -> Option<&PromptConfig> {
        self.prompts.get(field)
    }

    /// Retrieve all operations
    pub fn get_operations(&self) -> &Vec<OperationConfig> {
        &self.operations
    }

    /// Get environment variable value by env_var name
    pub fn get_env(&self, env_var: &str) -> Option<String> {
        std::env::var(env_var).ok()
    }
}
