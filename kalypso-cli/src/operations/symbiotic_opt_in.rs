use async_trait::async_trait;
use ethers::prelude::*;
use std::{collections::HashMap, sync::Arc};

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct SymbioticOptIn;

#[async_trait]
impl Operation for SymbioticOptIn {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let symbiotic_info = CommonDeps::symbiotic_opt_in_info(&config)?;

        abigen!(
            Service,
            r#"
                [
                    {
                        "inputs": [
                            {
                                "internalType": "address",
                                "name": "network",
                                "type": "address"
                            }
                        ],
                        "name": "optIn",
                        "outputs": [],
                        "stateMutability": "nonpayable",
                        "type": "function"
                    }
                ]
            "#
        );

        // Initialize the provider
        let provider_http = Provider::<Http>::try_from(symbiotic_info.symbiotic_rpc_url)
            .map_err(|e| format!("Invalid RPC URL: {}", e))?;

        // Initialize the SignerMiddleware with the provider and signer
        let client = SignerMiddleware::new(provider_http.clone(), symbiotic_info.signer.clone());
        let client_arc = Arc::new(client);

        let vault_service = Service::new(symbiotic_info.vault_opt_in_service, client_arc.clone());

        let network_service =
            Service::new(symbiotic_info.network_opt_in_service, client_arc.clone());

        let vault_opt_in_transaction_hash =
            CommonDeps::send_and_confirm(vault_service.opt_in(symbiotic_info.vault_address).send())
                .await
                .map_err(|_| "Failed making vault opt in operation.".to_string())?;

        println!(
            "Vault Opt In transaction: {}",
            vault_opt_in_transaction_hash
        );

        let network_opt_in_transaction_hash = CommonDeps::send_and_confirm(
            network_service
                .opt_in(symbiotic_info.network_address)
                .send(),
        )
        .await
        .map_err(|_| "Failed making network opt in operation.".to_string())?;

        println!(
            "Network Opt In transaction: {}",
            network_opt_in_transaction_hash
        );

        Ok(())
    }
}
