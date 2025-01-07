// src/main.rs

mod common_deps;
mod config;
mod operations;
mod prompts;

use config::Config;
use dialoguer::{theme::ColorfulTheme, Select};
use log::{error, info};
use operations::get_operation;
use prompts::Prompter;
use std::collections::HashMap;
use std::process;

#[macro_export]
macro_rules! send_with_optional_gas {
    ($builder:expr) => {{
        // Initialize the transaction builder as mutable
        let mut __tx_builder = $builder;

        // Conditionally set the gas limit based on the feature flag
        if cfg!(feature = "force_transactions") {
            __tx_builder = __tx_builder.gas(1_000_000);
        }

        // Send and confirm the transaction, handling errors
        CommonDeps::send_and_confirm(__tx_builder.send())
            .await
            .map_err(|e| format!("Claim Reward Transaction failed: {}", e))
    }};
}

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    // Load configuration
    let config = Config::new();

    // Retrieve all operations from the configuration
    let operations = {
        let operations = config.get_operations();
        let mut operations = operations.clone();

        operations.sort();

        operations
    };

    if operations.is_empty() {
        error!("No operations defined in config.json.");
        eprintln!("Error: No operations available. Please check your configuration.");
        process::exit(1);
    }

    let env_operation = std::env::var("OPERATION_NAME").ok();

    // Initialize a variable to hold the selected operation
    let selected_operation = if let Some(op_name) = env_operation {
        // Step 2: Validate the operation name from the environment
        match operations.iter().find(|op| op.name == op_name) {
            Some(op) => {
                println!("Using operation from environment variable: {}", op.name);
                op
            }
            None => {
                // If the operation name from the environment is invalid, log an error and exit
                error!("Invalid operation name in OPERATION_NAME: {}", op_name);
                eprintln!("Error: '{}' is not a valid operation.", op_name);
                process::exit(1);
            }
        }
    } else {
        // Step 3: Fallback to interactive selection if the environment variable is not set
        let operation_names: Vec<&str> = operations.iter().map(|op| op.name.as_str()).collect();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an operation")
            .items(&operation_names)
            .default(0)
            .interact()
            .unwrap_or_else(|e| {
                error!("Failed to interact with user: {}", e);
                eprintln!("Error: Failed to select operation.");
                process::exit(1);
            });

        &operations[selection]
    };

    println!(
        "\nYou selected: {} - {}\n",
        selected_operation.name, selected_operation.description
    );

    // Initialize the prompter with the configuration
    let prompter = Prompter::new(&config);

    // Collect required prompts for the selected operation
    let final_config: HashMap<String, String> = if std::env::var("OPERATION_NAME").is_ok() {
        prompter
            .expect_in_env(&selected_operation.required_prompts)
            .unwrap_or_else(|e| {
                error!("Failed to collect prompts: {}", e);
                eprintln!("Error: {}", e);
                process::exit(1);
            })
    } else {
        prompter
            .prompt(&selected_operation.required_prompts)
            .unwrap_or_else(|e| {
                error!("Failed to collect prompts: {}", e);
                eprintln!("Error: {}", e);
                process::exit(1);
            })
    };

    // Retrieve the corresponding operation handler
    let operation = get_operation(&selected_operation.name).unwrap_or_else(|| {
        error!(
            "Operation '{}' is not implemented.",
            selected_operation.name
        );
        eprintln!(
            "Error: Operation '{}' is not implemented.",
            selected_operation.name
        );
        process::exit(1);
    });

    // Execute the operation
    match operation.execute(final_config).await {
        Ok(_) => {
            println!(
                "Operation '{}' completed successfully.",
                selected_operation.name
            );
            info!(
                "Operation '{}' completed successfully.",
                selected_operation.name
            );
        }
        Err(e) => {
            error!(
                "Error during operation '{}': {}",
                selected_operation.name, e
            );
            eprintln!(
                "Error during operation '{}': {}",
                selected_operation.name, e
            );
            process::exit(1);
        }
    }
}
