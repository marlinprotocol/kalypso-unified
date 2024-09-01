use std::fs;

use dotenv::dotenv;
use matching_engine::{MatchingEngine, MatchingEngineConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let matching_engine_config_path =
        "../matching_engine_config/matching_engine_config.json".to_string();
    let alt_matching_engine_config_path =
        "./matching_engine_config/matching_engine_config.json".to_string();
    let file_content = fs::read_to_string(&matching_engine_config_path)
        .or_else(|_| fs::read_to_string(&alt_matching_engine_config_path))?;
    let config: MatchingEngineConfig = serde_json::from_str(&file_content)?;

    let matching_engine = MatchingEngine::from_config(config);
    let _ = matching_engine.run().await;

    Ok(())
}
