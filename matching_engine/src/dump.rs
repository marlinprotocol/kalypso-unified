use crate::{
    encrypted_dump::EncryptedDump,
    in_memory_stores::{
        ask_store::LocalAskStore, cost_store::CostStore, generator_store::GeneratorStore,
        key_store::KeyStore, native_stake_store::NativeStakingStore,
        stake_manager_store::StakeManagerStore, symbiotic_stake_store::SymbioticStakeStore,
    },
    latest_block_store::LatestBlockStore,
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
use tokio::sync::RwLockReadGuard;

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
    pub parsed_block: LatestBlockStore,
}

use std::path::Path;
use tokio::io::AsyncWriteExt;

impl Dump {
    pub async fn local_backup(
        shared_market_store: RwLockReadGuard<'_, MarketMetadataStore>,
        shared_ask_store: RwLockReadGuard<'_, LocalAskStore>,
        shared_generator_store: RwLockReadGuard<'_, GeneratorStore>,
        shared_native_store: RwLockReadGuard<'_, NativeStakingStore>,
        shared_symbiotic_store: RwLockReadGuard<'_, SymbioticStakeStore>,
        shared_cost_store: RwLockReadGuard<'_, CostStore>,
        shared_key_store: RwLockReadGuard<'_, KeyStore>,
        shared_stake_manager_store: RwLockReadGuard<'_, StakeManagerStore>,
        shared_parsed_block: RwLockReadGuard<'_, LatestBlockStore>,
        path_to_snapshot: &Path,
    ) {
        let market_store = shared_market_store.clone();
        let ask_store = shared_ask_store.clone();
        let generator_store = shared_generator_store.clone();
        let native_store = shared_native_store.clone();
        let symbiotic_store = shared_symbiotic_store.clone();
        let cost_store = shared_cost_store.clone();
        let key_store = shared_key_store.clone();
        let stake_manager_store = shared_stake_manager_store.clone();
        let parsed_block = shared_parsed_block.clone();

        // Create a new Dump instance from these values.
        let dump_instance = Dump {
            market_metadata_store: market_store,
            local_ask_store: ask_store,
            generator_store: generator_store,
            native_staking_store: native_store,
            symbiotic_stake_store: symbiotic_store,
            cost_store: cost_store,
            key_store: key_store,
            stake_manager_store: stake_manager_store,
            parsed_block: parsed_block,
        };

        // Create the encrypted dump. If there's an error, log it and return early.
        let encrypted_dump = match dump_instance.create_encrypted_dump().await {
            Ok(data) => data,
            Err(err) => {
                log::error!("Error creating dump: {}", err);
                return;
            }
        };

        // Ensure the parent directory exists.
        if let Some(parent) = path_to_snapshot.parent() {
            if let Err(e) = tokio::fs::create_dir_all(parent).await {
                log::error!("Failed to create directory {:?}: {}", parent, e);
                // Decide if you want to return early here or continue.
            }
        }

        // Serialize the encrypted dump to a JSON string.
        let json_string = match serde_json::to_string(&encrypted_dump) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize encrypted dump: {}", e);
                return;
            }
        };

        // Write the JSON string to the file asynchronously.
        match tokio::fs::File::create(path_to_snapshot).await {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_string.as_bytes()).await {
                    log::error!("Failed to write to file {:?}: {}", path_to_snapshot, e);
                } else {
                    log::info!("Successfully backed up dump to {:?}", path_to_snapshot);
                }
            }
            Err(e) => {
                log::error!("Failed to create file {:?}: {}", path_to_snapshot, e);
            }
        }
    }
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
