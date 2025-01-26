use std::collections::HashMap;

use async_trait::async_trait;

use crate::common_deps::CommonDeps;

use super::Operation;

macro_rules! check_role {
    (
        $registry:expr,
        $registry_name:expr,
        $address:expr,
        $address_alias:expr,
        $role_fn:ident,
        $role_label:expr
    ) => {{
        // Fetch the specified role from the registry
        let role =
            $registry.$role_fn().call().await.map_err(|e| {
                format!("Fetch {} for {} failed: {}", $role_label, $registry_name, e)
            })?;

        // Check if the address has the specified role
        let has_role = $registry
            .has_role(role, $address)
            .call()
            .await
            .map_err(|e| format!("Fetch has_role for {} failed: {}", $registry_name, e))?;

        // If the address does not have the role, log a message
        if !has_role {
            println!(
                "{:?}({:?}) does not have role {} in {}",
                $address_alias, $address, $role_label, $registry_name
            );
        }
    }};
}

pub struct RoleCheck;

#[async_trait]
impl Operation for RoleCheck {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        println!("\n");

        let role_management_info = CommonDeps::role_management_check_info(&config)?;

        let address_to_check = &role_management_info.address_to_check_role_for;

        // Check roles for each registry using the macro
        check_role!(
            role_management_info.entity_registry,
            "ENTITY_KEY_REGISTRY",
            *address_to_check,
            "CUSTOM_ADDRESS",
            default_admin_role,
            "DEFAULT_ADMIN_ROLE"
        );

        check_role!(
            role_management_info.proof_marketplace,
            "PROOF_MARKETPLACE",
            *address_to_check,
            "CUSTOM_ADDRESS",
            default_admin_role,
            "DEFAULT_ADMIN_ROLE"
        );

        check_role!(
            role_management_info.generator_registry,
            "GENERATOR_REGISTRY",
            *address_to_check,
            "CUSTOM_ADDRESS",
            default_admin_role,
            "DEFAULT_ADMIN_ROLE"
        );

        check_role!(
            role_management_info.staking_manager,
            "STAKING_MANAGER",
            *address_to_check,
            "CUSTOM_ADDRESS",
            default_admin_role,
            "DEFAULT_ADMIN_ROLE"
        );

        check_role!(
            role_management_info.native_staking,
            "NATIVE_STAKING",
            *address_to_check,
            "CUSTOM_ADDRESS",
            default_admin_role,
            "DEFAULT_ADMIN_ROLE"
        );

        check_role!(
            role_management_info.symbiotic_staking_reward,
            "SYMBIOTIC_STAKING_REWARD",
            *address_to_check,
            "CUSTOM_ADDRESS",
            default_admin_role,
            "DEFAULT_ADMIN_ROLE"
        );

        check_role!(
            role_management_info.symbiotic_staking,
            "SYMBIOTIC_STAKING",
            *address_to_check,
            "CUSTOM_ADDRESS",
            bridge_enclave_updater_role,
            "BRIDGE_ENCLAVE_UPDATER_ROLE"
        );

        println!("\n");

        check_role!(
            role_management_info.entity_registry,
            "ENTITY_KEY_REGISTRY",
            role_management_info.generator_registry.address(),
            "GENERATOR_REGISTRY",
            key_register_role,
            "KEY_REGISTER_ROLE"
        );

        check_role!(
            role_management_info.entity_registry,
            "ENTITY_KEY_REGISTRY",
            role_management_info.proof_marketplace.address(),
            "PROOF_MARKETPLACE",
            key_register_role,
            "KEY_REGISTER_ROLE"
        );

        check_role!(
            role_management_info.entity_registry,
            "ENTITY_KEY_REGISTRY",
            *address_to_check,
            "CUSTOM_ADDRESS",
            moderator_role,
            "MODERATOR_ROLE"
        );

        check_role!(
            role_management_info.proof_marketplace,
            "PROOF_MARKETPLACE",
            *address_to_check,
            "CUSTOM_ADDRESS",
            updater_role,
            "UPDATER_ROLE"
        );

        check_role!(
            role_management_info.proof_marketplace,
            "PROOF_MARKETPLACE",
            role_management_info.symbiotic_staking.address(),
            "SYMBIOTIC_STAKING",
            symbiotic_staking_role,
            "SYMBIOTIC_STAKING_ROLE"
        );

        check_role!(
            role_management_info.generator_registry,
            "GENERATOR_REGISTRY",
            role_management_info.proof_marketplace.address(),
            "PROOF_MARKETPLACE",
            proof_market_place_role,
            "PROOF_MARKETPLACE_ROLE"
        );

        check_role!(
            role_management_info.native_staking,
            "NATIVE_STAKING",
            role_management_info.staking_manager.address(),
            "STAKING_MANAGER",
            staking_manager_role,
            "STAKING_MANAGER_ROLE"
        );

        check_role!(
            role_management_info.staking_manager,
            "STAKING_MANAGER",
            role_management_info.generator_registry.address(),
            "GENERATOR_REGISTRY",
            prover_manager_role,
            "PROVER_MANAGER_ROLE"
        );

        check_role!(
            role_management_info.staking_manager,
            "STAKING_MANAGER",
            role_management_info.symbiotic_staking.address(),
            "SYMBIOTIC_STAKING",
            symbiotic_staking_role,
            "SYMBIOTIC_STAKING_ROLE"
        );

        check_role!(
            role_management_info.symbiotic_staking,
            "SYMBIOTIC_STAKING",
            role_management_info.staking_manager.address(),
            "STAKING_MANAGER",
            staking_manager_role,
            "STAKING_MANAGER_ROLE"
        );

        check_role!(
            role_management_info.symbiotic_staking_reward,
            "SYMBIOTIC_STAKING_REWARD",
            role_management_info.symbiotic_staking.address(),
            "SYMBIOTIC_STAKING",
            symbiotic_staking_role,
            "SYMBIOTIC_STAKING_ROLE"
        );

        println!("\n");

        Ok(())
    }
}
