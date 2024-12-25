use ethers::types::U64;
use serde::{Serialize, Deserialize};
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
#[derive(Serialize, Deserialize, Clone)]
struct Dump {
    market_metadata_store: Option<MarketMetadataStore>,
    local_ask_store: Option<LocalAskStore>,
    generator_store: Option<GeneratorStore>,
    native_staking_store: Option<NativeStakingStore>,
    symbiotic_stake_store: Option<SymbioticStakeStore>,
    cost_store: Option<CostStore>,
    key_store: Option<KeyStore>,
    stake_manager_store: Option<StakeManagerStore>,
    parsed_block: Option<U64>,
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
        market_metadata_store: Some(market_store.clone()),
        local_ask_store: Some(ask_store.clone()),
        generator_store: Some(generator_store.clone()),
        native_staking_store: Some(native_store.clone()),
        symbiotic_stake_store: Some(symbiotic_store.clone()),
        cost_store: Some(cost_store.clone()),
        key_store: Some(key_store.clone()),
        stake_manager_store: Some(stake_manager_store.clone()),
        parsed_block: Some(parsed_block.clone()),
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
        market_metadata_store: Some(market_store.clone()),
        local_ask_store: Some(ask_store.clone()),
        generator_store: Some(generator_store.clone()),
        native_staking_store: Some(native_store.clone()),
        symbiotic_stake_store: Some(symbiotic_store.clone()),
        cost_store: Some(cost_store.clone()),
        key_store: Some(key_store.clone()),
        stake_manager_store: Some(stake_manager_store.clone()),
        parsed_block: Some(parsed_block.clone()),
    };

    let encrypted_dump = dump.create_encrypted_dump(ecies_keys.clone()).unwrap();

    // Return the JSON response
    Ok(HttpResponse::Ok().json(encrypted_dump))
}

impl Dump {
    pub fn create_encrypted_dump(&self, ecies_public_keys: Vec<Vec<u8>>) -> Result<EncryptedDump, Box<dyn std::error::Error>> {
        // Load matching engine configuration
        let config_path = "../matching_engine_config/matching_engine_config.json".to_string();
        let alt_config_path = "./matching_engine_config/matching_engine_config.json".to_string();
        let file_content =
            fs::read_to_string(config_path).or_else(|_| fs::read_to_string(alt_config_path)).unwrap();
        let config: MatchingEngineConfig = serde_json::from_str(&file_content).unwrap();
        let private_key = hex::decode(config.matching_engine_key).unwrap();
        let private_key: &[u8; 32] = private_key.as_slice().try_into().unwrap();
        let sk = SecretKey::parse(private_key).unwrap();

        let public_key = PublicKey::from_secret_key(&sk);
        let public_key = public_key.serialize_compressed().iter().cloned().collect();

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
    pub fn get_dump(&self, ecies_private_key: Vec<u8>) -> Result<Dump, Box<dyn std::error::Error>> {
        let encrypted_value = serde_json::to_value(self.clone().encrypted).unwrap();
        let encrypted_dump = serde_json::to_vec(&encrypted_value).unwrap();
        let mut decrypted_dump: Dump = Dump { 
            market_metadata_store: None, 
            local_ask_store: None, 
            generator_store: None, 
            native_staking_store: None, 
            symbiotic_stake_store: None, 
            cost_store: None, 
            key_store: None, 
            stake_manager_store: None, 
            parsed_block: None
        };
        for acl in self.clone().acls {
            let decrypted = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
                &encrypted_dump,
                &acl,
                &ecies_private_key, 
                None);
            match decrypted {
                Ok(data) => {
                    decrypted_dump = serde_json::from_slice(&data).unwrap();
                }
                Err(e) => {
                    log::warn!("Error: ecies key mismatch {:?}", e);
                }
            }
        }
        Ok(decrypted_dump)
    }
}

#[cfg(test)]
mod tests {
    use super::Dump;
    use std::fs;
    use serde_json;
    
    #[test]
    fn test_encryption_and_decrytion() {
        // fetch sample dump
        let dump_path = "../matching_engine_config/dump.json".to_string();
        let alt_dump_path = "./matching_engine_config/dump.json".to_string();
        let file_content =
            fs::read_to_string(dump_path).or_else(|_| fs::read_to_string(alt_dump_path)).unwrap();
        let dump: Dump = serde_json::from_str(&file_content).unwrap();

        // Encryption
        let private_key = "e2a16eece5f9e388ebe73b791343e1a98d86a17376e87be5155ec7cf9c78f069".as_bytes().to_vec();
        let public_key = "0xAB85EDad6e4Dc27493530A2CAa9332Aa38FecFB1".as_bytes().to_vec();

        let me_public_key = "0x378b45251c732E190ccf74A0FC971DF73559CA67".as_bytes().to_vec();
        let ecies_public_keys = vec![public_key, me_public_key];

        let encrypted_dump = dump.create_encrypted_dump(ecies_public_keys).unwrap();

        let decrypted_dump = encrypted_dump.get_dump(private_key).unwrap();

        assert_eq!(serde_json::to_string(&decrypted_dump).unwrap(), serde_json::to_string(&dump).unwrap());
    }
}