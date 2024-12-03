use async_trait::async_trait;
use ethers::prelude::*;
use std::{collections::HashMap, sync::Arc};

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct SymbioticOptinInfo;

#[async_trait]
impl Operation for SymbioticOptinInfo {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        abigen!(
            OptInService,
            r#"[
                {
                    "inputs": [
                        {
                            "internalType": "address",
                            "name": "who",
                            "type": "address"
                        },
                        {
                            "internalType": "address",
                            "name": "where",
                            "type": "address"
                        }
                    ],
                    "name": "isOptedIn",
                    "outputs": [
                        {
                            "internalType": "bool",
                            "name": "",
                            "type": "bool"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                }
            ]"#
        );

        let symbiotic_optin_info = CommonDeps::symbiotic_opt_in_info(&config)?;

        // Initialize the provider
        let provider_http = Provider::<Http>::try_from(symbiotic_optin_info.symbiotic_rpc_url)
            .map_err(|e| format!("Invalid RPC URL: {}", e))?;

        // Initialize the SignerMiddleware with the provider and signer
        let client =
            SignerMiddleware::new(provider_http.clone(), symbiotic_optin_info.signer.clone());
        let client_arc = Arc::new(client);

        let vault_service = OptInService::new(
            symbiotic_optin_info.vault_opt_in_service,
            client_arc.clone(),
        );

        let network_service = OptInService::new(
            symbiotic_optin_info.network_opt_in_service,
            client_arc.clone(),
        );

        let is_opted_in_vault = vault_service
            .is_opted_in(
                symbiotic_optin_info.signer.address(),
                symbiotic_optin_info.vault_address,
            )
            .call()
            .await
            .map_err(|e| {
                format!(
                    "Failed Calling Vault OptIn Service:{:?}: {}",
                    symbiotic_optin_info.vault_opt_in_service, e
                )
            })?;

        if is_opted_in_vault {
            println!(
                "Operator: {:?} is already opted in vault: {:?}",
                symbiotic_optin_info.signer.address(),
                symbiotic_optin_info.vault_address
            );
        } else {
            eprintln!(
                "Operator: {:?} is not opted in vault: {:?}",
                symbiotic_optin_info.signer.address(),
                symbiotic_optin_info.vault_address
            );
        }

        let is_opted_in_network = network_service
            .is_opted_in(
                symbiotic_optin_info.signer.address(),
                symbiotic_optin_info.network_address,
            )
            .call()
            .await
            .map_err(|e| {
                format!(
                    "Failed Calling Network OptIn Service:{:?}: {}",
                    symbiotic_optin_info.network_opt_in_service, e
                )
            })?;

        if is_opted_in_network {
            println!(
                "Operator: {:?} is already opted in network: {:?}",
                symbiotic_optin_info.signer.address(),
                symbiotic_optin_info.network_address
            );
        } else {
            eprintln!(
                "Operator: {:?} is not opted in network: {:?}",
                symbiotic_optin_info.signer.address(),
                symbiotic_optin_info.network_address
            );
        }

        Ok(())
    }
}

pub struct SymbioticOperatorRegistrationInfo;

#[async_trait]
impl Operation for SymbioticOperatorRegistrationInfo {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        abigen!(
            IRegistry,
            r#"[
                {
                    "anonymous": false,
                    "inputs": [
                        {
                            "indexed": true,
                            "internalType": "address",
                            "name": "entity",
                            "type": "address"
                        }
                    ],
                    "name": "AddEntity",
                    "type": "event"
                },
                {
                    "inputs": [
                        {
                            "internalType": "address",
                            "name": "account",
                            "type": "address"
                        }
                    ],
                    "name": "isEntity",
                    "outputs": [
                        {
                            "internalType": "bool",
                            "name": "",
                            "type": "bool"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "totalEntities",
                    "outputs": [
                        {
                            "internalType": "uint256",
                            "name": "",
                            "type": "uint256"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "uint256",
                            "name": "index",
                            "type": "uint256"
                        }
                    ],
                    "name": "entity",
                    "outputs": [
                        {
                            "internalType": "address",
                            "name": "",
                            "type": "address"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                }
            ]"#
        );

        let symbiotic_register_info = CommonDeps::symbiotic_operator_registry_info(&config)?;

        // Initialize the provider
        let provider_http = Provider::<Http>::try_from(symbiotic_register_info.symbiotic_rpc_url)
            .map_err(|e| format!("Invalid RPC URL: {}", e))?;

        // Initialize the SignerMiddleware with the provider and signer
        let client = SignerMiddleware::new(
            provider_http.clone(),
            symbiotic_register_info.signer.clone(),
        );
        let client_arc = Arc::new(client);

        let operator_registry = IRegistry::new(
            symbiotic_register_info.symbiotic_operator_registry,
            client_arc.clone(),
        );

        let is_operator = operator_registry
            .is_entity(symbiotic_register_info.signer.address())
            .call()
            .await
            .map_err(|e| format!("Failed Calling Symbiotic Operator Registry Contract: {}", e))?;

        if is_operator {
            println!(
                "Operator: {:?} is already a symbiotic operator",
                symbiotic_register_info.signer.address()
            );
        } else {
            println!(
                "Operator: {:?} is not symbiotic operator. Please register",
                symbiotic_register_info.signer.address()
            );
        }
        Ok(())
    }
}

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

        let vault_opt_in_transaction_hash = match CommonDeps::send_and_confirm(
            vault_service.opt_in(symbiotic_info.vault_address).send(),
        )
        .await
        {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed making vault opt in operation {}", e);
                eprintln!("Run the operation again to retry");
                "Failed Vault Opt IN".into()
            }
        };

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
