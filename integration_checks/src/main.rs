use clap::{arg, Command};
use serde::de::DeserializeOwned;
use std::{error::Error, fmt::Debug, str::FromStr};

// Define an enum for the 'type' argument
#[derive(Debug, Clone)]
enum ToolType {
    Prover,
    Ivs,
    MatchingEngine,
}

impl FromStr for ToolType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "prover" => Ok(ToolType::Prover),
            "ivs" => Ok(ToolType::Ivs),
            "matching_engine" => Ok(ToolType::MatchingEngine),
            _ => Err("Invalid type. Must be 'prover' or 'ivs'.".into()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Kalypso Integration Checker")
        .version("1.0")
        .author("Kalypso Admin")
        .about("A CLI tool that takes checks whether kalypso executables are built correctly")
        .arg(arg!([url] "Sets the URL to use").required(true))
        .arg(
            arg!([type] "Sets the type (prover or ivs)")
                .required(true)
                .value_parser(parse_tool_type),
        )
        .get_matches();

    let url = matches.get_one::<String>("url").unwrap();
    let tool_type = matches.get_one::<ToolType>("type").unwrap();

    println!("URL: {}", url);
    println!("Type: {:?}", tool_type);

    let _ = match tool_type {
        ToolType::Prover => test_generator_services(url).await,
        ToolType::Ivs => test_ivs_services(url).await,
        ToolType::MatchingEngine => test_matching_engine_services(url).await,
    };

    Ok(())
}

async fn test_matching_engine_services(matching_engine_url: &String) -> Result<(), Box<dyn Error>> {
    let matching_engine_service_checker = generator::ServiceChecker {
        server_url: matching_engine_url.into(),
        services: vec![Box::new(matching_engine::get_welcome_request::<
            matching_engine::models::WelcomeResponse,
        >())],
    };

    matching_engine_service_checker.check_all_services().await;
    Ok(())
}

async fn test_generator_services(generator_url: &String) -> Result<(), Box<dyn Error>> {
    let generator_service = generator::ServiceChecker {
        server_url: generator_url.into(),
        services: vec![
            Box::new(generator::get_test_request::<generator::models::TestResponse>()),
            Box::new(generator::get_benchmark_request::<
                generator::models::BenchmarkResponse,
            >()),
            Box::new(generator::generate_proof_request::<
                generator::models::GenerateProofResponse,
            >(
                create_payload("./integration_checks/proverCustomData/generate_proof_payload.json")
                    .await,
            )),
        ],
    };
    generator_service.check_all_services().await;
    Ok(())
}

async fn test_ivs_services(ivs_url: &String) -> Result<(), Box<dyn Error>> {
    let ivs_service_checker = generator::ServiceChecker {
        server_url: ivs_url.into(),
        services: vec![
            Box::new(ivs::get_test_request::<generator::models::TestResponse>()),
            Box::new(
                ivs::generate_input_request::<ivs::models::CheckInputResponse>(
                    create_payload("./integration_checks/ivsCustomData/check_input_payload.json")
                        .await,
                ),
            ),
            Box::new(ivs::generate_invalid_input_request::<
                generator::models::GenerateProofResponse,
            >(
                create_payload(
                    "./integration_checks/ivsCustomData/check_invalid_input_payload.json",
                )
                .await,
            )),
            Box::new(ivs::generate_check_encrypted_inputs_request::<
                ivs::models::CheckInputResponse,
            >(
                create_payload(
                    "./integration_checks/ivsCustomData/check_encrypted_input_payload.json",
                )
                .await,
            )),
            Box::new(ivs::generate_verify_inputs_and_proof_request::<
                ivs::models::CheckInputResponse,
            >(
                create_payload(
                    "integration_checks/ivsCustomData/verify_inputs_and_proof_payload.json",
                )
                .await,
            )),
        ],
    };
    ivs_service_checker.check_all_services().await;
    Ok(())
}
fn parse_tool_type(value: &str) -> Result<ToolType, String> {
    value.parse::<ToolType>()
}

async fn create_payload<T: DeserializeOwned>(path: &str) -> Option<T> {
    use tokio::fs;
    let json_data = fs::read_to_string(path).await;
    if json_data.is_err() {
        return None;
    };

    let deserialized_payload: Option<T> = serde_json::from_str(&json_data.unwrap()).ok();
    deserialized_payload
}
