use std::fs;
use std::io;

use dotenv::dotenv;
use matching_engine::{Dump, MatchingEngine, MatchingEngineConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load matching engine configuration
    let config_paths = [
        "../matching_engine_config/matching_engine_config.json",
        "./matching_engine_config/matching_engine_config.json",
    ];
    let config_content = read_file_from_paths(&config_paths)?;
    let config: MatchingEngineConfig = serde_json::from_str(&config_content)?;

    // Get the indexer port from environment variables
    let indexer_port =
        std::env::var("INDEXER_PORT").expect("INDEXER_PORT environment variable is not set");
    let indexer_port: Option<u16> = indexer_port.parse().ok();

    // Initialize the matching engine
    let matching_engine = MatchingEngine::from_config(config, indexer_port);

    // Attempt to load the dump file
    let dump_paths = [
        "../matching_engine_config/dump.json",
        "./matching_engine_config/dump.json",
    ];
    match read_file_from_paths(&dump_paths) {
        Ok(dump_content) => {
            let dump: Dump = serde_json::from_str(&dump_content)?;
            matching_engine.run_from_dump(dump).await?;
        }
        Err(_) => {
            matching_engine.run().await?;
        }
    }

    Ok(())
}

// Helper function to read a file from multiple possible paths
fn read_file_from_paths(paths: &[&str]) -> io::Result<String> {
    for path in paths {
        if let Ok(content) = fs::read_to_string(path) {
            return Ok(content);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "File not found in any of the specified paths",
    ))
}
