use ethers::types::U64;
use serde::Serialize;
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;
use ecies::{PublicKey, SecretKey};
use kalypso_helper::secret_inputs_helpers;

use crate::ask_lib::ask_store::LocalAskStore;
use crate::costs::CostStore;
use crate::generator_lib::generator_store::GeneratorStore;
use crate::generator_lib::key_store::KeyStore;
use crate::generator_lib::native_stake_store::NativeStakingStore;
use crate::generator_lib::stake_manager_store::StakeManagerStore;
use crate::generator_lib::symbiotic_stake_store::SymbioticStakeStore;
use crate::market_metadata::MarketMetadataStore;
use crate::MatchingEngineConfig;

use crate::{models::WelcomeResponse, try_read_or_lock};
use actix_web::{web::Data, HttpResponse};

pub async fn welcome() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(WelcomeResponse {
        status: "welcome to ui routes!.".into(),
    }))
}

// Define the Dump struct
#[derive(Serialize, Clone)]
struct Dump<'a> {
    market_metadata_store: Option<&'a MarketMetadataStore>,
    local_ask_store: Option<&'a LocalAskStore>,
    generator_store: Option<&'a GeneratorStore>,
    native_staking_store: Option<&'a NativeStakingStore>,
    symbiotic_stake_store: Option<&'a SymbioticStakeStore>,
    cost_store: Option<&'a CostStore>,
    key_store: Option<&'a KeyStore>,
    stake_manager_store: Option<&'a StakeManagerStore>,
    parsed_block: Option<&'a U64>,
}

#[derive(Serialize, Clone)]
struct EncryptedDump {
    encrypted: Vec<u8>,
    acls: Vec<Vec<u8>>,
}

pub async fn get_dump(
    local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,
    local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
    local_native_store: Data<Arc<RwLock<NativeStakingStore>>>,
    local_symbiotic_store: Data<Arc<RwLock<SymbioticStakeStore>>>,
    local_cost_store: Data<Arc<RwLock<CostStore>>>,
    local_key_store: Data<Arc<RwLock<KeyStore>>>,
    local_stake_manager_store: Data<Arc<RwLock<StakeManagerStore>>>,
    local_parsed_block: Data<Arc<RwLock<U64>>>,
) -> actix_web::Result<HttpResponse> {
    try_read_or_lock!(local_market_store, market_store);
    try_read_or_lock!(local_ask_store, ask_store);
    try_read_or_lock!(local_generator_store, generator_store);
    try_read_or_lock!(local_native_store, native_store);
    try_read_or_lock!(local_symbiotic_store, symbiotic_store);
    try_read_or_lock!(local_cost_store, cost_store);
    try_read_or_lock!(local_key_store, key_store);
    try_read_or_lock!(local_stake_manager_store, stake_manager_store);
    try_read_or_lock!(local_parsed_block, parsed_block);

    let dump = Dump {
        market_metadata_store: Some(&*market_store),
        local_ask_store: Some(&*ask_store),
        generator_store: Some(&*generator_store),
        native_staking_store: Some(&*native_store),
        symbiotic_stake_store: Some(&*symbiotic_store),
        cost_store: Some(&*cost_store),
        key_store: Some(&*key_store),
        stake_manager_store: Some(&*stake_manager_store),
        parsed_block: Some(&*parsed_block),
    };

    // Return the JSON response
    Ok(HttpResponse::Ok().json(dump))
}

pub async fn get_encrypted_dump(
    local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,
    local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
    local_native_store: Data<Arc<RwLock<NativeStakingStore>>>,
    local_symbiotic_store: Data<Arc<RwLock<SymbioticStakeStore>>>,
    local_cost_store: Data<Arc<RwLock<CostStore>>>,
    local_key_store: Data<Arc<RwLock<KeyStore>>>,
    local_stake_manager_store: Data<Arc<RwLock<StakeManagerStore>>>,
    ecies_public_keys: Data<Arc<RwLock<Vec<Vec<u8>>>>>,
    local_parsed_block: Data<Arc<RwLock<U64>>>,
) -> actix_web::Result<HttpResponse> {
    try_read_or_lock!(local_market_store, market_store);
    try_read_or_lock!(local_ask_store, ask_store);
    try_read_or_lock!(local_generator_store, generator_store);
    try_read_or_lock!(local_native_store, native_store);
    try_read_or_lock!(local_symbiotic_store, symbiotic_store);
    try_read_or_lock!(local_cost_store, cost_store);
    try_read_or_lock!(local_key_store, key_store);
    try_read_or_lock!(local_stake_manager_store, stake_manager_store);
    try_read_or_lock!(ecies_public_keys, ecies_keys);
    try_read_or_lock!(local_parsed_block, parsed_block);

    let dump = Dump {
        market_metadata_store: Some(&*market_store),
        local_ask_store: Some(&*ask_store),
        generator_store: Some(&*generator_store),
        native_staking_store: Some(&*native_store),
        symbiotic_stake_store: Some(&*symbiotic_store),
        cost_store: Some(&*cost_store),
        key_store: Some(&*key_store),
        stake_manager_store: Some(&*stake_manager_store),
        parsed_block: Some(&*parsed_block),
    };

    let encrypted_dump = dump.create_encrypted_dump(ecies_keys.clone()).await. unwrap();

    // Return the JSON response
    Ok(HttpResponse::Ok().json(encrypted_dump))
}

impl<'a> Dump<'a> {
    async fn create_encrypted_dump(&self, ecies_public_keys: Vec<Vec<u8>>) -> Result<EncryptedDump, Box<dyn std::error::Error>> {
        // Load matching engine configuration
        let config_path = "../../../matching_engine_config/matching_engine_config.json".to_string();
        let alt_config_path = "./../../matching_engine_config/matching_engine_config.json".to_string();
        let file_content =
            fs::read_to_string(config_path).or_else(|_| fs::read_to_string(alt_config_path)).unwrap();
        let config: MatchingEngineConfig = serde_json::from_str(&file_content).unwrap();
        let private_key = hex::decode(config.matching_engine_key).unwrap();
        let private_key: &[u8; 32] = private_key.as_slice().try_into().unwrap();
        let sk = SecretKey::parse(private_key).unwrap();

        let public_key = PublicKey::from_secret_key(&sk);
        let public_key = public_key.serialize_compressed().to_vec();

        // Check matching engine key in the ecies key list
        if ecies_public_keys.contains(&public_key) {
            let dump_value = serde_json::to_value(self).unwrap();
            let dump = serde_json::to_vec(&dump_value).unwrap();
            let encrypted_data = secret_inputs_helpers::encrypt_data_with_aes_and_multi_ecies(ecies_public_keys, &dump).unwrap();
            let encrypted_dump = EncryptedDump{
                encrypted: encrypted_data.encrypted_data,
                acls: encrypted_data.acls
            };
            Ok(encrypted_dump)
        } else {
            Err("Matching engine key not found in key list".into())
        }

        
    }
}

impl EncryptedDump {
    pub async fn get_dump_plaintxt(&self, ecies_private_key: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encrypted_value = serde_json::to_value(self.clone().encrypted).unwrap();
        let encrypted_dump = serde_json::to_vec(&encrypted_value).unwrap();
        let mut decrypted_dump: Vec<u8> = vec![];
        for acl in self.clone().acls {
            let decrypted = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
                &encrypted_dump,
                &acl,
                &ecies_private_key, 
                None);
            match decrypted {
                Ok(data) => {
                    decrypted_dump = data;
                }
                Err(e) => {
                    log::warn!("Error: ecies key mismatch {:?}", e);
                }
            }
        }
        Ok(decrypted_dump)
    }
}