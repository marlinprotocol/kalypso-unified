use std::{collections::HashMap, fs, io};

use async_trait::async_trait;
use ethers::{signers::Signer, types::Address};

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct UpdateGeneratorMeta;

#[async_trait]
impl Operation for UpdateGeneratorMeta {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let generator_meta_info = CommonDeps::update_generator_meta_info(&config)?;

        let generator_data = generator_meta_info
            .read_generator_registry
            .prover_manager(generator_meta_info.private_key_signer.address())
            .call()
            .await
            .map_err(|e| format!("Failed Reading Generator Registry Contract {}", e))?;

        if generator_data.0.eq(&Address::zero()) {
            return Err(format!("Please 'Register' operator first"));
        }

        let generator_meta_paths = ["./generatormeta.json"];

        let data = read_file_from_paths(&generator_meta_paths)
            .map_err(|e| format!("Failed reading generatormeta.json {}", e))?;

        let generator_meta: matching_engine_helpers::generator_lib::generator_store::GeneratorMeta =
            serde_json::from_str(&data).map_err(|e| {
                format!("Failed deserde generatormeta.json into GeneratorMeta {}", e)
            })?;

        let json_string = serde_json::to_string(&generator_meta)
            .map_err(|e| format!("Failed converting GeneratorMeta to string{}", e))?;

        let generator_metadata: Vec<u8> = json_string.into_bytes();

        let tx_hash = CommonDeps::send_and_confirm(
            generator_meta_info
                .generator_registry
                .update_prover_data(generator_metadata.into())
                .send(),
        )
        .await?;

        // Print the transaction hash
        println!("Update Generator Metadata Transaction: {}", tx_hash);

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
