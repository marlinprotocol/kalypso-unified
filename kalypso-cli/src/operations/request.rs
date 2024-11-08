use std::collections::HashMap;

use async_trait::async_trait;

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct DiscardRequest;

#[async_trait]
impl Operation for DiscardRequest {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let discard_request_info = CommonDeps::discard_request_info(&config)?;

        let tx_hash = CommonDeps::send_and_confirm(
            discard_request_info
                .proof_marketplace
                .discard_request(discard_request_info.ask_id)
                .send(),
        )
        .await?;

        // Print the transaction hash
        println!("{}", tx_hash);

        Ok(())
    }
}
