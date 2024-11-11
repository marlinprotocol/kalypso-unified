use std::collections::HashMap;

use async_trait::async_trait;
use ethers::providers::Middleware;

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct ReadProof;

#[async_trait]
impl Operation for ReadProof {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let read_proof_info = CommonDeps::read_proof_info(&config)?;

        let filter = read_proof_info
            .proof_marketplace
            .proof_created_filter()
            .filter
            .topic0(read_proof_info.ask_id);

        let logs = read_proof_info
            .provider_http
            .get_logs(&filter)
            .await
            .map_err(|e| format!("Failed Reading Proof: {}", e))?;

        for log in logs {
            println!("Log: {:?}", log);
        }

        Ok(())
    }
}
