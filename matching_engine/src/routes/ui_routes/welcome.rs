use ethers::types::{U256, U64};
use serde::{Serialize, Deserialize};
use std::fs;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use ecies::{PublicKey, SecretKey};
use kalypso_helper::secret_inputs_helpers;

use crate::ask_lib::ask_store::LocalAskStore;
use ethers::providers::Provider;
use ethers::signers::LocalWallet;
use ethers::middleware::MiddlewareBuilder;
use ethers::signers::Signer;
use ethers::types::Address;
use ethers::providers::Http;
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

#[derive(Serialize, Clone, Debug)]
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

    let encrypted_dump = dump.create_encrypted_dump().await.unwrap();

    // Return the JSON response
    Ok(HttpResponse::Ok().json(encrypted_dump))
}

impl Dump {
    pub async fn create_encrypted_dump(&self) -> Result<EncryptedDump, Box<dyn std::error::Error>> {
        // Load matching engine configuration
        let config_path = "../matching_engine_config/matching_engine_config.json".to_string();
        let alt_config_path = "./matching_engine_config/matching_engine_config.json".to_string();
        let file_content =
            fs::read_to_string(config_path).or_else(|_| fs::read_to_string(alt_config_path)).unwrap();
        let config: MatchingEngineConfig = serde_json::from_str(&file_content).unwrap();

        let rpc_url = config.clone().rpc_url;
        let chain_id = config.clone().chain_id;

        let relayer_key = config.clone().relayer_private_key;
        let relayer_signer = relayer_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(U64::from_dec_str(&chain_id).unwrap().as_u64());

        let provider_http = Provider::<Http>::try_from(&rpc_url)
            .unwrap()
            // .with_signer(matching_engine_signer.clone());
            .with_signer(relayer_signer.clone());

        let client = Arc::new(provider_http.clone());
        
        let proof_market_place_var = config.clone().proof_market_place;
        let proof_market_place_addr = Address::from_str(&proof_market_place_var).unwrap();

        let entity_key_registry_var = config.clone().entity_registry;
        let entity_key_registry_address = Address::from_str(&entity_key_registry_var).unwrap();

        let entity_key_registry = bindings::entity_key_registry::EntityKeyRegistry::new(
            entity_key_registry_address,
            client.clone(),
        );

        // Get the matching engine keys
        let mut ecies_public_keys = vec![];
        let matching_engine_key = entity_key_registry.pub_key(
            proof_market_place_addr, 
            U256::from(0)
        ).call().await.unwrap();

        let mut extended_pub_key = vec![0x04];
        extended_pub_key.extend_from_slice(&matching_engine_key);

        // Now, `extended_pub_key` is a 65-byte vector with `04` prepended.
        let pub_key_array: &[u8; 65] = extended_pub_key.as_slice().try_into().unwrap();
        let me_public_key = ecies::PublicKey::parse(pub_key_array).unwrap();
        let me_public_key = me_public_key.serialize_compressed();
        ecies_public_keys.push(me_public_key.to_vec());
        
        // Ensure this matching engine can decrypt
        let private_key = hex::decode(config.matching_engine_key).unwrap();
        let private_key: &[u8; 32] = private_key.as_slice().try_into().unwrap();
        let sk = SecretKey::parse(private_key).unwrap();

        let public_key = PublicKey::from_secret_key(&sk);
        let public_key = public_key.serialize_compressed();

        // Check matching engine key in the ecies key list
        if ecies_public_keys.contains(&public_key.to_vec()) {
            let dump_value = serde_json::to_value(self).unwrap();
            let dump = serde_json::to_vec(&dump_value).unwrap();
            let encrypted_data = secret_inputs_helpers::encrypt_data_with_aes_and_multi_ecies(
                ecies_public_keys, 
                &dump
            ).unwrap();
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
    pub fn get_dump(&self) -> Result<Dump, Box<dyn std::error::Error>> {
        // Load matching engine configuration
        let config_path = "../matching_engine_config/matching_engine_config.json".to_string();
        let alt_config_path = "./matching_engine_config/matching_engine_config.json".to_string();
        let file_content =
            fs::read_to_string(config_path).or_else(|_| fs::read_to_string(alt_config_path)).unwrap();
        let config: MatchingEngineConfig = serde_json::from_str(&file_content).unwrap();
        let me_private_key_vec = hex::decode(config.matching_engine_key).unwrap();

        let encrypted_dump = self.clone().encrypted;
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
        // let mut counter = 0;
        for acl in self.clone().acls {
            // counter = counter + 1;
            // println!("Loop {:?}, ACL {:?}", counter, acl);
            let decrypted = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
                &encrypted_dump,
                &acl,
                &me_private_key_vec, 
                Some(U256::from(1)));
            match decrypted {
                Ok(data) => {
                    // println!("OK, Loop {:?}", counter);
                    decrypted_dump = serde_json::from_slice(&data).unwrap();
                    break;
                }
                Err(e) => {
                    // println!("Err, Loop {:?}", counter);
                    log::warn!("Error: ecies key mismatch {:?}", e);
                    continue;
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
    use ethers::types::U64;
    use crate::costs::CostStore;
    use crate::ask_lib::ask_store::LocalAskStore;
    use crate::generator_lib::key_store::KeyStore;
    use crate::market_metadata::MarketMetadataStore;
    use crate::generator_lib::generator_store::GeneratorStore;
    use crate::generator_lib::stake_manager_store::StakeManagerStore;
    use crate::generator_lib::native_stake_store::NativeStakingStore;
    use crate::generator_lib::symbiotic_stake_store::SymbioticStakeStore;
    
    #[tokio::test]
    async fn test_encryption_and_decrytion() {
        // // fetch sample dump
        // let dump_path = "../matching_engine_config/dump.json".to_string();
        // let alt_dump_path = "./matching_engine_config/dump.json".to_string();
        // let file_content =
        //     fs::read_to_string(dump_path).or_else(|_| fs::read_to_string(alt_dump_path)).unwrap();
        // let dump: Dump = serde_json::from_str(&file_content).unwrap();

        // create default dump
        let dump = Dump {
            market_metadata_store: Some(MarketMetadataStore::new()), 
            local_ask_store: Some(LocalAskStore::new()), 
            generator_store: Some(GeneratorStore::new()), 
            native_staking_store: Some(NativeStakingStore::new()), 
            symbiotic_stake_store: Some(SymbioticStakeStore::new()), 
            cost_store: Some(CostStore::new()), 
            key_store: Some(KeyStore::new()), 
            stake_manager_store: Some(StakeManagerStore::new()), 
            parsed_block: Some(U64::default())
        };

        let encrypted_dump = dump.create_encrypted_dump().await.unwrap();

        let decrypted_dump = encrypted_dump.get_dump().unwrap();
        let decrypted_dump_str = serde_json::to_string(&decrypted_dump).unwrap();
        dbg!(decrypted_dump_str);
    }
}