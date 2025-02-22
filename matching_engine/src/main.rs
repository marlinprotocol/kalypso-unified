use std::fs;
use std::io;

use dotenv::dotenv;
use matching_engine::dump::Dump;
use matching_engine::encrypted_dump::EncryptedDump;
use matching_engine::{in_memory_matching_engine::InMemoryMatchingEngine, MatchingEngineConfig};

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
    let matching_engine = InMemoryMatchingEngine::from_config(config, indexer_port);

    // Attempt to load the dump file
    let encrypted_dump_paths = [
        "../matching_engine_config/encrypted_dump.json",
        "./matching_engine_config/encrypted_dump.json",
    ];

    match read_file_from_paths(&encrypted_dump_paths) {
        Ok(dump_content) => {
            let enc_dump: EncryptedDump = serde_json::from_str(&dump_content)?;
            matching_engine
                .run_from_encrypted_dump(enc_dump, encrypted_dump_paths[1].to_string())
                .await?;
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // Handle the NotFound error case
            let plain_dump = [
                "../matching_engine_config/dump.json",
                "./matching_engine_config/dump.json",
            ];

            match read_file_from_paths(&plain_dump) {
                Ok(dump_content) => {
                    let dump: Dump = serde_json::from_str(&dump_content)?;
                    matching_engine
                        .run_from_dump(dump, encrypted_dump_paths[1].to_string())
                        .await?;
                }
                Err(_) => {
                    matching_engine
                        .run(encrypted_dump_paths[1].to_string())
                        .await?;
                }
            }
        }
        Err(_) => {
            matching_engine
                .run(encrypted_dump_paths[1].to_string())
                .await?;
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
