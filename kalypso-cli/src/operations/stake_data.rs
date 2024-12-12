use std::collections::HashMap;

use async_trait::async_trait;
use ethers::types::Address;
use serde::Deserialize;

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct StakeData;

#[async_trait]
impl Operation for StakeData {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let read_stake_data_info = CommonDeps::read_stake_data_info(&config)?;

        task_assignment_requirements(read_stake_data_info.indexer_url.clone())
            .await
            .map_err(|e| format!("Unable to read task data from indexer: {}", e))?;
        read_staking_data(
            &read_stake_data_info.operator_address,
            read_stake_data_info.indexer_url,
        )
        .await
        .map_err(|e| format!("Unable to read stake data from indexer: {}", e))?;
        Ok(())
    }
}

// Define the structure of each stake entry
#[derive(Deserialize, Debug)]
struct StakeEntry {
    token: String,
    amount: String,
}

// Define the structure of the stake breakdown
#[derive(Deserialize, Debug)]
struct StakeBreakDown {
    total_native_stake: Vec<StakeEntry>,
    total_native_stake_locked: Vec<StakeEntry>,
    total_symbiotic_stake: Vec<StakeEntry>,
    total_symbiotic_stake_locked: Vec<StakeEntry>,
    available_native_stake: Vec<StakeEntry>,
    available_symbiotic_stake: Vec<StakeEntry>,
}

// Define the structure of the entire API response
#[derive(Deserialize, Debug)]
struct ApiResponse {
    stake_break_down: StakeBreakDown,
    // Include other fields if necessary
}

use std::sync::Once;
// Initialize a Once instance for printing headers only once
static STAKING_INIT: Once = Once::new();
static TASK_INIT: Once = Once::new();

use reqwest::header::{HeaderMap, CONTENT_TYPE};
use std::error::Error;

// Define the structure for task assignment requirements
#[derive(Deserialize, Debug)]
struct TaskAssignmentRequirements {
    native: Vec<TaskEntry>,
    symbiotic: Vec<TaskEntry>,
}

// Define the structure of each task entry
#[derive(Deserialize, Debug)]
struct TaskEntry {
    token: String,
    amount: String,
}

// Define the structure of the entire API response for dashboard
#[derive(Deserialize, Debug)]
struct DashboardResponse {
    task_assignment_requirements: TaskAssignmentRequirements,
    // Include other fields if necessary
}

async fn task_assignment_requirements(indexer_url: String) -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client with a timeout (optional but recommended)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Construct the full URL
    let full_url = format!("{}/ui/dashboard", indexer_url);

    // Prepare the headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    // Make the GET request
    let response = client.get(&full_url).headers(headers).send().await?;

    // Check if the response status is success
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    // Parse the JSON response
    let dashboard_response: DashboardResponse = response.json().await?;

    // Define the table headers
    TASK_INIT.call_once(|| {
        println!("\n\nUsers need to have one of the tokens in the Native section and any one of the tokens in the Symbiotic section to get a job.\n\n");
        println!(
            "{:<20} | {:<42} | {:>20}",
            "Requirement Type", "Token", "Amount"
        );
        println!("{:-<20}-+-{:-<42}-+-{:-<20}", "", "", "");
    });

    // Helper function to print each task entry
    fn print_task(requirement_type: &str, tasks: &Vec<TaskEntry>) {
        for task in tasks {
            println!(
                "{:<20} | {:<42} | {:>20}",
                requirement_type, task.token, task.amount
            );
        }
    }

    // Process and print the task_assignment_requirements data in tabular format
    print_task(
        "Native",
        &dashboard_response.task_assignment_requirements.native,
    );
    print_task(
        "Symbiotic",
        &dashboard_response.task_assignment_requirements.symbiotic,
    );

    Ok(())
}

async fn read_staking_data(
    operator_address: &Address,
    indexer_url: String,
) -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client with a timeout (optional but recommended)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Construct the full URL
    let full_url = format!("{}/ui/generator/{:?}", indexer_url, operator_address);

    // Prepare the headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    // Make the GET request
    let response = client.get(&full_url).headers(headers).send().await?;

    // Check if the response status is success
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    // Parse the JSON response
    let api_response: ApiResponse = response.json().await?;

    // Define the table headers
    STAKING_INIT.call_once(|| {
        println!(
            "\n\nTokens Delegated to operator {:?}\n\n",
            operator_address
        );
        println!(
            "{:<30} | {:<42} | {:>20}",
            "Stake Category", "Token", "Amount"
        );
        println!("{:-<30}-+-{:-<42}-+-{:-<20}", "", "", "");
    });

    // Helper function to print each stake entry
    fn print_stake(category: &str, stakes: &Vec<StakeEntry>) {
        for stake in stakes {
            println!(
                "{:<30} | {:<42} | {:>20}",
                category, stake.token, stake.amount
            );
        }
    }

    // Process and print the stake_break_down data in tabular format
    print_stake(
        "Total Native Stake",
        &api_response.stake_break_down.total_native_stake,
    );
    print_stake(
        "Total Native Stake Locked",
        &api_response.stake_break_down.total_native_stake_locked,
    );
    print_stake(
        "Total Symbiotic Stake",
        &api_response.stake_break_down.total_symbiotic_stake,
    );
    print_stake(
        "Total Symbiotic Stake Locked",
        &api_response.stake_break_down.total_symbiotic_stake_locked,
    );
    print_stake(
        "Available Native Stake",
        &api_response.stake_break_down.available_native_stake,
    );
    print_stake(
        "Available Symbiotic Stake",
        &api_response.stake_break_down.available_symbiotic_stake,
    );

    println!();
    Ok(())
}
