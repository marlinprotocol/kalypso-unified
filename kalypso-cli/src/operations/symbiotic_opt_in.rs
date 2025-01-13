use async_trait::async_trait;
use ethers::prelude::*;
use std::{collections::HashMap, sync::Arc};

use crate::common_deps::CommonDeps;
use kalypso_helper::send_with_optional_gas;

use super::Operation;

pub struct SymbioticOptinInfo;

#[async_trait]
impl Operation for SymbioticOptinInfo {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let symbiotic_optin_info = CommonDeps::symbiotic_opt_in_info(&config)?;

        // Initialize the provider
        let provider_http = Provider::<Http>::try_from(symbiotic_optin_info.symbiotic_rpc_url)
            .map_err(|e| format!("Invalid RPC URL: {}", e))?;

        // Initialize the SignerMiddleware with the provider and signer
        let client =
            SignerMiddleware::new(provider_http.clone(), symbiotic_optin_info.signer.clone());
        let client_arc = Arc::new(client);

        let vault_service = bindings::OptInService::new(
            symbiotic_optin_info.vault_opt_in_service,
            client_arc.clone(),
        );

        let network_service = bindings::OptInService::new(
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

        let operator_registry = bindings::operator_registry::OperatorRegistry::new(
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

        // Initialize the provider
        let provider_http = Provider::<Http>::try_from(symbiotic_register_info.symbiotic_rpc_url)
            .map_err(|e| format!("Invalid RPC URL: {}", e))?;

        // Initialize the SignerMiddleware with the provider and signer
        let client = SignerMiddleware::new(
            provider_http.clone(),
            symbiotic_register_info.signer.clone(),
        );
        let client_arc = Arc::new(client);

        let operator_registry = bindings::operator_registry::OperatorRegistry::new(
            symbiotic_register_info.symbiotic_operator_registry,
            client_arc.clone(),
        );

        let operator_registry_transaction_hash =
            send_with_optional_gas!(operator_registry.register_operator())
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

        // Initialize the provider
        let provider_http = Provider::<Http>::try_from(symbiotic_info.symbiotic_rpc_url)
            .map_err(|e| format!("Invalid RPC URL: {}", e))?;

        // Initialize the SignerMiddleware with the provider and signer
        let client = SignerMiddleware::new(provider_http.clone(), symbiotic_info.signer.clone());
        let client_arc = Arc::new(client);

        let vault_service =
            bindings::OptInService::new(symbiotic_info.vault_opt_in_service, client_arc.clone());

        let network_service =
            bindings::OptInService::new(symbiotic_info.network_opt_in_service, client_arc.clone());

        let vault_opt_in_transaction_hash =
            match send_with_optional_gas!(vault_service.opt_in(symbiotic_info.vault_address)) {
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

        let network_opt_in_transaction_hash =
            send_with_optional_gas!(network_service.opt_in(symbiotic_info.network_address))
                .map_err(|e| format!("Failed making network opt in operation {}", e))?;

        println!(
            "Network Opt In transaction: {}",
            network_opt_in_transaction_hash
        );

        Ok(())
    }
}

pub struct SetMiddlewareAddress;

#[async_trait]
impl Operation for SetMiddlewareAddress {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let set_middleware_info = CommonDeps::set_middleware_address_info(&config)?;

        let provider_http = Provider::<Http>::try_from(set_middleware_info.symbiotic_rpc_url)
            .map_err(|e| format!("Invalid RPC URL: {}", e))?;

        // Initialize the SignerMiddleware with the provider and signer
        let client =
            SignerMiddleware::new(provider_http.clone(), set_middleware_info.signer.clone());
        let client_arc = Arc::new(client);

        let middleware = bindings::middleware::Middleware::new(
            set_middleware_info.middleware_address,
            client_arc.clone(),
        );

        let tx_hash =
            send_with_optional_gas!(middleware.set_delegate(set_middleware_info.operator_address))
                .map_err(|e| {
                    format!(
                        "Failed setting operator address in kalypso middleware {}",
                        e
                    )
                })?;

        println!("Set Operator in Middleware Transaction: {}", tx_hash);
        Ok(())
    }
}
