use async_trait::async_trait;
use ethers::prelude::*;
use std::{collections::HashMap, sync::Arc};

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct SymbioticOperatorRegister;

#[async_trait]
impl Operation for SymbioticOperatorRegister {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let symbiotic_register_info = CommonDeps::symbiotic_operator_registry_info(&config)?;
        abigen!(
            OperatorRegistryContract,
            r#"[
                {
                    "inputs": [],
                    "name": "registerOperator",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                }
            ]"#
        );

        // Initialize the provider
        let provider_http = Provider::<Http>::try_from(symbiotic_register_info.symbiotic_rpc_url)
            .map_err(|e| format!("Invalid RPC URL: {}", e))?;

        // Initialize the SignerMiddleware with the provider and signer
        let client = SignerMiddleware::new(
            provider_http.clone(),
            symbiotic_register_info.signer.clone(),
        );
        let client_arc = Arc::new(client);

        let operator_registry = OperatorRegistryContract::new(
            symbiotic_register_info.symbiotic_operator_registry,
            client_arc.clone(),
        );

        let operator_registry_transaction_hash =
            CommonDeps::send_and_confirm(operator_registry.register_operator().send())
                .await
                .map_err(|e| format!("Failed making symbiotic operator registry {}", e))?;

        println!(
            "Operator Registry transaction: {}",
            operator_registry_transaction_hash
        );

        Ok(())
    }
}

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
                .map_err(|e| format!("Failed making vault opt in operation {}", e))?;

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
        .map_err(|e| format!("Failed making network opt in operation {}", e))?;

        println!(
            "Network Opt In transaction: {}",
            network_opt_in_transaction_hash
        );

        Ok(())
    }
}
