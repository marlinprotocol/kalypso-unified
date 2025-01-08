use std::{collections::HashMap, fs, io};

use async_trait::async_trait;

use crate::{common_deps::CommonDeps, send_with_optional_gas};

use super::Operation;

pub struct UpdateMarketMetadata;

#[async_trait]
impl Operation for UpdateMarketMetadata {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let market_meta_update_info = CommonDeps::update_market_metadata_info(&config)?;

        let meta_paths = ["./marketmeta.json"];

        let data = read_file_from_paths(&meta_paths)
            .map_err(|e| format!("Failed reading marketmeta.json {}", e))?;

        let market_meta: matching_engine_helpers::market_metadata::MarketSetupData =
            serde_json::from_str(&data)
                .map_err(|e| format!("Failed deserde marketmeta.json into MarketMetadata{}", e))?;

        let json_string = serde_json::to_string(&market_meta)
            .map_err(|e| format!("Failed converting MarketMetadata to string{}", e))?;

        let market_metadata: Vec<u8> = json_string.into_bytes();

        let tx_hash = send_with_optional_gas!(market_meta_update_info
            .proof_marketplace
            .update_market_metadata(market_meta_update_info.market_id, market_metadata.into()))
        .map_err(|e| format!("Failed Update Market Metadata Transaction: {}", e))?;

        // Print the transaction hash
        println!("Update Market Metadata Transaction: {}", tx_hash);

        Ok(())
    }
}

// Helper function to read a file from multiple possible paths
pub fn read_file_from_paths(paths: &[&str]) -> io::Result<String> {
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
