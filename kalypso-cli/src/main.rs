// src/main.rs

mod common_deps;
mod config;
mod operations;
mod prompts;

use config::Config;
use dialoguer::{theme::ColorfulTheme, Select};
use operations::get_operation;
use prompts::Prompter;
use std::collections::HashMap;
use std::process;

#[macro_export]
macro_rules! try_read_contract_error {
    ($error:expr, $error_type:path, $contract_label:expr) => {{
        if let Some(contract_error) = $error.decode_contract_revert::<$error_type>() {
            eprintln!("{:?} Error: {:?}", $contract_label, contract_error);
        }
    }};
}

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
        CommonDeps::send_and_confirm(__tx_builder.send()).await
    }};
}

#[tokio::main]
async fn main() {
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
                eprintln!("Error: Failed to select operation: {}", e);
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
                eprintln!("Error: {}", e);
                process::exit(1);
            })
    } else {
        prompter
            .prompt(&selected_operation.required_prompts)
            .unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                process::exit(1);
            })
    };

    // Retrieve the corresponding operation handler
    let operation = get_operation(&selected_operation.name).unwrap_or_else(|| {
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
        }
        Err(e) => {
            eprintln!(
                "Error during operation '{}': {}",
                selected_operation.name, e
            );
            process::exit(1);
        }
    }
}
