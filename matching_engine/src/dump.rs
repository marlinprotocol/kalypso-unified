use crate::{
    ask_lib::ask_store::LocalAskStore,
    costs::CostStore,
    encrypted_dump::EncryptedDump,
    generator_lib::{
        generator_store::GeneratorStore, key_store::KeyStore,
        native_stake_store::NativeStakingStore, stake_manager_store::StakeManagerStore,
        symbiotic_stake_store::SymbioticStakeStore,
    },
    market_metadata::MarketMetadataStore,
    MatchingEngineConfig,
};
use async_trait::async_trait;
use ecies::{PublicKey, SecretKey};
use ethers::prelude::*;
use kalypso_helper::secret_inputs_helpers;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{str::FromStr, sync::Arc};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Dump {
    pub market_metadata_store: MarketMetadataStore,
    pub local_ask_store: LocalAskStore,
    pub generator_store: GeneratorStore,
    pub native_staking_store: NativeStakingStore,
    pub symbiotic_stake_store: SymbioticStakeStore,
    pub cost_store: CostStore,
    pub key_store: KeyStore,
    pub stake_manager_store: StakeManagerStore,
    pub parsed_block: U64,
}

#[async_trait]
impl ToEncryptedDump for Dump {
    async fn create_encrypted_dump(
        &self,
    ) -> Result<EncryptedDump, Box<dyn std::error::Error + Send + Sync>> {
        // Load matching engine configuration
        let config_path = "../matching_engine_config/matching_engine_config.json".to_string();
        let alt_config_path = "./matching_engine_config/matching_engine_config.json".to_string();
        let file_content =
            fs::read_to_string(config_path).or_else(|_| fs::read_to_string(alt_config_path))?;
        let config: MatchingEngineConfig = serde_json::from_str(&file_content)?;

        let rpc_url = config.clone().rpc_url;
        let provider_http = Provider::<Http>::try_from(&rpc_url)?;
        let client = Arc::new(provider_http.clone());

        let proof_market_place_var = config.clone().proof_market_place;
        let proof_market_place_addr = Address::from_str(&proof_market_place_var).unwrap();

        let entity_key_registry_var = config.clone().entity_registry;
        let entity_key_registry_address = Address::from_str(&entity_key_registry_var).unwrap();

        let entity_key_registry = bindings::entity_key_registry::EntityKeyRegistry::new(
            entity_key_registry_address,
            client.clone(),
        );

        // Get the matching engine keys from contract
        let mut ecies_public_keys = vec![];
        let matching_engine_key = entity_key_registry
            .pub_key(proof_market_place_addr, U256::from(0))
            .call()
            .await?;

        let mut extended_pub_key = vec![0x04];
        extended_pub_key.extend_from_slice(&matching_engine_key);

        // Now, `extended_pub_key` is a 65-byte vector with `04` prepended.
        let pub_key_array: &[u8; 65] = extended_pub_key.as_slice().try_into()?;
        let me_public_key = ecies::PublicKey::parse(pub_key_array)?;
        let me_public_key = me_public_key.serialize_compressed();
        ecies_public_keys.push(me_public_key.to_vec());

        // Ensure this matching engine can decrypt
        let private_key = hex::decode(config.matching_engine_key)?;
        let private_key: &[u8; 32] = private_key.as_slice().try_into()?;
        let sk = SecretKey::parse(private_key)?;

        let public_key = PublicKey::from_secret_key(&sk);
        let public_key = public_key.serialize_compressed();

        ecies_public_keys.push(public_key.to_vec());

        // Check matching engine key in the ecies key list
        let dump_value = serde_json::to_value(self)?;
        let dump = serde_json::to_vec(&dump_value)?;
        let encrypted_data =
            secret_inputs_helpers::encrypt_data_with_aes_and_multi_ecies(ecies_public_keys, &dump)
                .map_err(|e| {
                    Box::<dyn std::error::Error + Send + Sync>::from(format!(
                        "Encryption error: {}",
                        e
                    ))
                })?;

        let encrypted_dump = EncryptedDump {
            encrypted: encrypted_data.encrypted_data,
            acls: encrypted_data.acls,
        };
        Ok(encrypted_dump)
    }
}

#[async_trait]
pub trait ToEncryptedDump {
    async fn create_encrypted_dump(
        &self,
    ) -> Result<EncryptedDump, Box<dyn std::error::Error + Send + Sync>>;
}
