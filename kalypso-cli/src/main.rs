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

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    // Load configuration
    let config = Config::new();

    // Retrieve all operations from the configuration
    let operations = config.get_operations();

    if operations.is_empty() {
        error!("No operations defined in config.json.");
        eprintln!("Error: No operations available. Please check your configuration.");
        process::exit(1);
    }

    // Present a selection menu to the user
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

    let selected_operation = &operations[selection];
    println!(
        "\nYou selected: {} - {}\n",
        selected_operation.name, selected_operation.description
    );

    // Initialize the prompter with the configuration
    let prompter = Prompter::new(&config);

    // Collect required prompts for the selected operation
    let final_config: HashMap<String, String> = prompter
        .prompt(&selected_operation.required_prompts)
        .unwrap_or_else(|e| {
            error!("Failed to collect prompts: {}", e);
            eprintln!("Error: {}", e);
            process::exit(1);
        });

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
