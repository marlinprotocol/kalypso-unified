use std::{collections::HashMap, error::Error};

use async_trait::async_trait;

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct Benchmark;

#[async_trait]
impl Operation for Benchmark {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let benchmark_info = CommonDeps::benchmark_info(&config)?;

        benchmark(benchmark_info.benchmark_url)
            .await
            .map_err(|e| format!("Failed to benchmark prover: {}", e))?;

        Ok(())
    }
}

pub struct TestConnection;

#[async_trait]
impl Operation for TestConnection {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let test_connection_info = CommonDeps::test_connection_info(&config)?;

        test_connection(test_connection_info.enclave_client_url)
            .await
            .map_err(|e| format!("Failed to connect enclave client: {}", e))?;

        Ok(())
    }
}

pub struct StopProgram;

#[async_trait]
impl Operation for StopProgram {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let start_program_info = CommonDeps::start_program_info(&config)?;

        stop_program(
            start_program_info.generator_client_url,
            start_program_info.prover_program_name,
        )
        .await
        .map_err(|e| format!("Failed to stop program in the enclave: {}", e))?;

        Ok(())
    }
}

pub struct StartProgam;

#[async_trait]
impl Operation for StartProgam {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let start_program_info = CommonDeps::start_program_info(&config)?;

        start_program(
            start_program_info.generator_client_url,
            start_program_info.prover_program_name,
        )
        .await
        .map_err(|e| format!("Failed to start program in the enclave: {}", e))?;

        Ok(())
    }
}

async fn start_program(
    generator_client_url: String,
    program_name: String,
) -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client with a timeout (optional but recommended)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let full_url = format!("{}/api/startProgram", generator_client_url);

    // Prepare the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);

    let payload = serde_json::json!({
        "program_name": program_name.to_string()
    });

    // Send the POST request
    let response = client
        .post(&full_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    // Capture the HTTP status before moving the response
    let status = response.status();

    // Read the response body as text
    let response_text = response.text().await?;

    let json_response: kalypso_helper::response::JsonResponse =
        match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                return Err(format!(
                    "Failed to parse JSON response. HTTP Status: {}, Body: {}, Error: {}",
                    status, response_text, e
                )
                .into())
            }
        };

    // Check the HTTP status and the JSON message
    if status.is_success() {
        println!("{:?}", json_response);
        Ok(())
    } else {
        // Return error with message
        Err(format!(
            "Failed to start program. HTTP Status: {}, Message: {}",
            status, json_response.message
        )
        .into())
    }
}

async fn stop_program(
    generator_client_url: String,
    program_name: String,
) -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client with a timeout (optional but recommended)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let full_url = format!("{}/api/stopProgram", generator_client_url);

    // Prepare the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);

    let payload = serde_json::json!({
        "program_name": program_name.to_string()
    });

    // Send the POST request
    let response = client
        .post(&full_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    // Capture the HTTP status before moving the response
    let status = response.status();

    // Read the response body as text
    let response_text = response.text().await?;

    let json_response: kalypso_helper::response::JsonResponse =
        match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                return Err(format!(
                    "Failed to parse JSON response. HTTP Status: {}, Body: {}, Error: {}",
                    status, response_text, e
                )
                .into())
            }
        };

    // Check the HTTP status and the JSON message
    if status.is_success() {
        println!("{:?}", json_response);
        Ok(())
    } else {
        // Return error with message
        Err(format!(
            "Failed to stop program. HTTP Status: {}, Message: {}",
            status, json_response.message
        )
        .into())
    }
}

async fn test_connection(enclave_client_url: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let full_url = format!("{}/api/test", enclave_client_url);

    // Prepare the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);

    // Send the POST request
    let response = client.get(&full_url).headers(headers).send().await?;

    // Capture the HTTP status before moving the response
    let status = response.status();

    // Check the HTTP status and the JSON message
    if status.is_success() {
        Ok(())
    } else {
        // Return error with message
        Err(format!("Failed to  test connection. HTTP Status: {}", status).into())
    }
}

async fn benchmark(benchmark_url: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let full_url = format!("{}/api/benchmark", benchmark_url);

    // Prepare the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);

    // Send the POST request
    let response = client.get(&full_url).headers(headers).send().await?;

    // Capture the HTTP status before moving the response
    let status = response.status();

    // Read the response body as text
    let response_text = response.text().await?;

    let json_response: generator::models::BenchmarkResponse =
        match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                return Err(format!(
                    "Failed to parse JSON response. HTTP Status: {}, Body: {}, Error: {}",
                    status, response_text, e
                )
                .into())
            }
        };

    // Check the HTTP status and the JSON message
    if status.is_success() {
        println!("{:?}", json_response);
        Ok(())
    } else {
        // Return error with message
        Err(format!("Failed to benchmark prover. HTTP Status: {}", status).into())
    }
}
