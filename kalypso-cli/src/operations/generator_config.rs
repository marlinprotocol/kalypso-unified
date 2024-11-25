use std::{collections::HashMap, error::Error};

use async_trait::async_trait;

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct GeneratorConfig;

#[async_trait]
impl Operation for GeneratorConfig {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let load_generator_config_info = CommonDeps::load_operator_config_info(&config)?;
        load_config(
            load_generator_config_info.generator_client_url,
            load_generator_config_info.config,
        )
        .await
        .map_err(|e| format!("Failed loading generator config into the enclave. {}", e))?;

        Ok(())
    }
}

async fn load_config(
    generator_client_url: String,
    config: generator_client::model::GeneratorConfigSetupRequestBody,
) -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client with a timeout (optional but recommended)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let full_url = format!("{}/api/generatorConfigSetup", generator_client_url);

    // Prepare the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);

    // Send the POST request
    let response = client
        .post(&full_url)
        .headers(headers)
        .json(&config)
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
            "Failed to load config. HTTP Status: {}, Message: {}",
            status, json_response.message
        )
        .into())
    }
}
