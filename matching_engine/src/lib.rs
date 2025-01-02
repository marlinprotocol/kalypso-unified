pub mod ask_lib;
pub mod costs;
pub mod counters;
pub mod generator_lib;
pub mod market_metadata;
pub mod models;
pub mod utility;

mod jobs;
mod log_processor;
mod routes;

#[macro_use]
mod macros;

use ask_lib::ask_store::LocalAskStore;
use generator_lib::native_stake_store::NativeStakingStore;
use generator_lib::stake_manager_store::StakeManagerStore;
use generator_lib::symbiotic_stake_store::SymbioticStakeStore;
use market_metadata::MarketMetadataStore;

use costs::CostStore;
use ethers::prelude::*;
use generator_lib::{generator_store::GeneratorStore, key_store::KeyStore};
use jobs::{parser::LogParser, server::MatchingEngineServer};
use models::{GetAskStatus, MarketInfo};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use service_check_helper::{Request, RequestType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use ecies::{PublicKey, SecretKey};
use ethers::types::{U256, U64};
use kalypso_helper::secret_inputs_helpers;
use std::fs;

pub fn get_welcome_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/welcome".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Checks server reach".into(),
    }
}

pub fn get_latest_block_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/getLatestBlock".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Checks upto which block ME has reached".into(),
    }
}

pub fn get_key_balance_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/getKeyBalance".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Checks balance of gas key".into(),
    }
}

pub fn get_status_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/getStatus".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Checks ME overal status".into(),
    }
}

pub fn get_single_ask_status_request<R>(
    input_payload: Option<GetAskStatus>,
    expected_status_code: StatusCode,
    info: String,
) -> Request<GetAskStatus, R> {
    Request {
        request_type: RequestType::POST(
            input_payload.unwrap_or_else(|| GetAskStatus { ask_id: "1".into() }),
        ),
        service_endpoint: "/getAskStatus".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code,
        info,
    }
}

pub fn get_single_market_info<R>(
    input_payload: Option<MarketInfo>,
    expected_status_code: StatusCode,
    info: String,
) -> Request<MarketInfo, R> {
    Request {
        request_type: RequestType::POST(input_payload.unwrap_or_else(|| MarketInfo {
            market_id: "3".into(),
        })),
        service_endpoint: "/marketInfo".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code,
        info,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchingEngineConfig {
    pub rpc_url: String,
    pub chain_id: String,
    pub matching_engine_key: String,
    pub relayer_private_key: String,
    pub proof_market_place: String,
    pub generator_registry: String,
    pub entity_registry: String,
    #[serde(default = "default_symbiotic_staking")]
    pub symbiotic_staking: String,
    #[serde(default = "default_native_staking")]
    pub native_staking: String,
    #[serde(default = "default_staking_manager")]
    pub staking_manager: String,
    pub start_block: String,
}

// Default function for symbiotic_staking
// Adding the to avoid changes in ME Client for now
fn default_symbiotic_staking() -> String {
    "0xE7136641cB2c94d318779c3B6BEb997dC5B2E574".to_string()
}

fn default_native_staking() -> String {
    "0xe9d2Bcc597f943ddA9EDf356DAC7C6A713dDE113".to_string()
}

fn default_staking_manager() -> String {
    "0xCe75C0E25b2c70415b237273345105402aEbe79F".to_string()
}

pub struct MatchingEngine {
    config: MatchingEngineConfig,
    matching_engine_port: u16,
}

pub enum DumpType {
    Encrypted(EncryptedDump),
    Regular(Dump),
}

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

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct EncryptedDump {
    encrypted: Vec<u8>,
    acls: Vec<Vec<u8>>,
}

impl Dump {
    pub async fn create_encrypted_dump(&self) -> Result<EncryptedDump, Box<dyn std::error::Error>> {
        // Load matching engine configuration
        let config_path = "../matching_engine_config/matching_engine_config.json".to_string();
        let alt_config_path = "./matching_engine_config/matching_engine_config.json".to_string();
        let file_content = fs::read_to_string(config_path)
            .or_else(|_| fs::read_to_string(alt_config_path))
            .unwrap();
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
        let matching_engine_key = entity_key_registry
            .pub_key(proof_market_place_addr, U256::from(0))
            .call()
            .await
            .unwrap();

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
                &dump,
            )
            .unwrap();
            let encrypted_dump = EncryptedDump {
                encrypted: encrypted_data.encrypted_data,
                acls: encrypted_data.acls,
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
        let file_content = fs::read_to_string(config_path)
            .or_else(|_| fs::read_to_string(alt_config_path))
            .unwrap();
        let config: MatchingEngineConfig = serde_json::from_str(&file_content).unwrap();
        let me_private_key_vec = hex::decode(config.matching_engine_key).unwrap();

        let encrypted_dump = self.clone().encrypted;
        let mut decrypted_dump: Dump = Dump::default();

        // let mut counter = 0;
        for acl in self.clone().acls {
            // counter = counter + 1;
            // println!("Loop {:?}, ACL {:?}", counter, acl);
            let decrypted = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
                &encrypted_dump,
                &acl,
                &me_private_key_vec,
                Some(U256::from(1)),
            );
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

impl MatchingEngine {
    pub fn from_config(config: MatchingEngineConfig, matching_engine_port: Option<u16>) -> Self {
        Self {
            config,
            matching_engine_port: matching_engine_port.unwrap_or(3000),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rpc_url: String,
        chain_id: String,
        matching_engine_key: String,
        relayer_private_key: String,
        proof_market_place: String,
        generator_registry: String,
        entity_registry: String,
        symbiotic_staking: String,
        native_staking: String,
        staking_manager: String,
        start_block: String,
        matching_engine_port: Option<u16>,
    ) -> Self {
        let config: MatchingEngineConfig = MatchingEngineConfig {
            rpc_url,
            chain_id,
            matching_engine_key,
            relayer_private_key,
            proof_market_place,
            generator_registry,
            entity_registry,
            symbiotic_staking,
            native_staking,
            staking_manager,
            start_block,
        };

        Self::from_config(config, matching_engine_port)
    }

    pub async fn run_from_encrypted_dump(
        &self,
        encrypted_dump: EncryptedDump,
        path_to_snapshot: String,
    ) -> anyhow::Result<()> {
        let dump = encrypted_dump.get_dump().unwrap();
        self.run_from_dump(dump, path_to_snapshot).await
    }

    pub async fn run_from_dump(&self, dump: Dump, path_to_snapshot: String) -> anyhow::Result<()> {
        // wrapping around is case to shared across threads
        let shared_local_ask_store = Arc::new(RwLock::new(dump.local_ask_store.clone()));
        let shared_generator_store = Arc::new(RwLock::new(dump.generator_store.clone()));
        let shared_market_store = Arc::new(RwLock::new(dump.market_metadata_store.clone()));
        let shared_key_store = Arc::new(RwLock::new(dump.key_store.clone()));
        let shared_cost_store = Arc::new(RwLock::new(dump.cost_store.clone()));
        let shared_symbiotic_staking_store =
            Arc::new(RwLock::new(dump.symbiotic_stake_store.clone()));
        let shared_native_store = Arc::new(RwLock::new(dump.native_staking_store.clone()));
        let shared_stake_manager_store = Arc::new(RwLock::new(dump.stake_manager_store.clone()));
        let shared_parsed_block_number_store = Arc::new(RwLock::new(dump.parsed_block.clone()));

        self._run(
            shared_local_ask_store,
            shared_generator_store,
            shared_market_store,
            shared_key_store,
            shared_cost_store,
            shared_symbiotic_staking_store,
            shared_native_store,
            shared_stake_manager_store,
            shared_parsed_block_number_store,
            path_to_snapshot,
        )
        .await
    }

    pub async fn run(&self, path_to_snapshot: String) -> anyhow::Result<()> {
        let local_ask_store = LocalAskStore::new();
        let generator_list_store = GeneratorStore::new();
        let key_list_store = KeyStore::new();
        let cost_store = CostStore::new();
        let market_list_store = MarketMetadataStore::new();
        let symbiotic_staking_store = SymbioticStakeStore::new();
        let native_staking_store = NativeStakingStore::new();
        let stake_manager_store = StakeManagerStore::new();
        let start_block_string = self.config.clone().start_block;

        // wrapping around is case to shared across threads
        let shared_local_ask_store = Arc::new(RwLock::new(local_ask_store));
        let shared_generator_store = Arc::new(RwLock::new(generator_list_store));
        let shared_market_store = Arc::new(RwLock::new(market_list_store));
        let shared_key_store = Arc::new(RwLock::new(key_list_store));
        let shared_cost_store = Arc::new(RwLock::new(cost_store));
        let shared_symbiotic_staking_store = Arc::new(RwLock::new(symbiotic_staking_store));
        let shared_native_store = Arc::new(RwLock::new(native_staking_store));
        let shared_stake_manager_store = Arc::new(RwLock::new(stake_manager_store));
        let shared_parsed_block_number_store = Arc::new(RwLock::new(
            U64::from_dec_str(&start_block_string).expect("Unable to rad start_block"),
        ));

        self._run(
            shared_local_ask_store,
            shared_generator_store,
            shared_market_store,
            shared_key_store,
            shared_cost_store,
            shared_symbiotic_staking_store,
            shared_native_store,
            shared_stake_manager_store,
            shared_parsed_block_number_store,
            path_to_snapshot,
        )
        .await
    }

    async fn _run(
        &self,
        shared_local_ask_store: Arc<RwLock<LocalAskStore>>,
        shared_generator_store: Arc<RwLock<GeneratorStore>>,
        shared_market_store: Arc<RwLock<MarketMetadataStore>>,
        shared_key_store: Arc<RwLock<KeyStore>>,
        shared_cost_store: Arc<RwLock<CostStore>>,
        shared_symbiotic_staking_store: Arc<RwLock<SymbioticStakeStore>>,
        shared_native_store: Arc<RwLock<NativeStakingStore>>,
        shared_stake_manager_store: Arc<RwLock<StakeManagerStore>>,
        shared_parsed_block_number_store: Arc<RwLock<U64>>,
        path_to_snapshot: String,
    ) -> anyhow::Result<()> {
        let relayer_key_balance = Arc::new(RwLock::new(ethers::types::U256::zero()));

        let rpc_url = self.config.clone().rpc_url;
        let chain_id = self.config.clone().chain_id;

        let matching_engine_key = self.config.clone().matching_engine_key;
        let matching_engine_signer = matching_engine_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(U64::from_dec_str(&chain_id).unwrap().as_u64());

        let relayer_key = self.config.clone().relayer_private_key;
        let relayer_signer = relayer_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(U64::from_dec_str(&chain_id).unwrap().as_u64());

        log::info!(
            "matching engine address {:?}",
            matching_engine_signer.clone().address()
        );

        log::info!("relayer address {:?}", relayer_signer.clone().address());

        let provider_http = Provider::<Http>::try_from(&rpc_url)
            .unwrap()
            // .with_signer(matching_engine_signer.clone());
            .with_signer(relayer_signer.clone());
        // let relay_key_balance = provider_http.get_balance(relayer_signer.address(), None).await;.

        let client = Arc::new(provider_http.clone());

        // Creating contract instance for proof market place
        let proof_market_place_var = self.config.clone().proof_market_place;
        let proof_marketplace_address = Address::from_str(&proof_market_place_var).unwrap();

        let proof_marketplace = bindings::proof_marketplace::ProofMarketplace::new(
            proof_marketplace_address,
            client.clone(),
        );

        // Creating contract instance for generator registry
        let generator_registry_var = self.config.clone().generator_registry;
        let generator_registry_address = Address::from_str(&generator_registry_var).unwrap();

        let generator_registry = bindings::generator_registry::GeneratorRegistry::new(
            generator_registry_address,
            client.clone(),
        );

        let entity_key_registry_var = self.config.clone().entity_registry;
        let entity_key_registry_address = Address::from_str(&entity_key_registry_var).unwrap();

        let entity_key_registry = bindings::entity_key_registry::EntityKeyRegistry::new(
            entity_key_registry_address,
            client.clone(),
        );

        let shared_entity_key_registry = bindings::entity_key_registry::EntityKeyRegistry::new(
            entity_key_registry_address,
            client.clone(),
        );
        let shared_entity_key = Arc::new(RwLock::new(shared_entity_key_registry));
        let shared_entity_key_registry = Arc::clone(&shared_entity_key);

        let symbiotic_staking_var = self.config.clone().symbiotic_staking;
        let symbiotic_staking_address = Address::from_str(&symbiotic_staking_var).unwrap();
        let shared_symbiotic_staking = bindings::symbiotic_staking::SymbioticStaking::new(
            symbiotic_staking_address,
            client.clone(),
        );

        let native_staking_var = self.config.clone().native_staking;
        let native_staking_address = Address::from_str(&native_staking_var).unwrap();
        let shared_native_staking =
            bindings::native_staking::NativeStaking::new(native_staking_address, client.clone());

        let staking_manager_var = self.config.clone().staking_manager;
        let staking_manager_address = Address::from_str(&staking_manager_var).unwrap();
        let shared_staking_manager =
            bindings::staking_manager::StakingManager::new(staking_manager_address, client.clone());

        let shared_parsed_block = Arc::clone(&shared_parsed_block_number_store);

        let shared_market_data = Arc::clone(&shared_market_store);
        let shared_generator_data = Arc::clone(&shared_generator_store);
        let shared_local_ask_data = Arc::clone(&shared_local_ask_store);

        let matching_engine_key_for_server = hex::decode(matching_engine_key.clone()).unwrap();
        let shared_matching_key = Arc::new(RwLock::new(matching_engine_key_for_server));
        let shared_matching_key_clone = Arc::clone(&shared_matching_key);

        let unhandled_logs = Arc::new(RwLock::new(vec![]));

        let should_stop = Arc::new(AtomicBool::new(false));
        let stop_handle_clone = should_stop.clone();

        let mut handles = vec![];

        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            stop_handle_clone.store(true, Ordering::Release);
        });

        let server = MatchingEngineServer::new(
            shared_market_data,
            shared_local_ask_data,
            shared_parsed_block.clone(),
            shared_matching_key_clone,
            shared_entity_key_registry,
            shared_generator_data,
            shared_native_store.clone(),
            shared_symbiotic_staking_store.clone(),
            shared_key_store.clone(),
            shared_cost_store.clone(),
            shared_stake_manager_store.clone(),
            relayer_key_balance.clone(),
            should_stop.clone(),
            unhandled_logs.clone(),
        );

        let matching_engine_port = self.matching_engine_port;

        let server_handle: JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> =
            tokio::spawn(async move {
                server
                    .start_server(matching_engine_port, false)
                    .await
                    .unwrap();
                Ok(())
            });
        handles.push(server_handle);

        let confirmations = 5; // ideally this should be more
        let block_range = 20000; // Number of blocks to fetch logs from at once
        let should_stop_clone = should_stop.clone();

        let log_parser = LogParser::new(
            should_stop_clone,
            rpc_url,
            relayer_signer.clone(),
            shared_parsed_block,
            block_range.into(),
            confirmations.into(),
            proof_marketplace.clone(),
            generator_registry,
            entity_key_registry,
            shared_symbiotic_staking,
            shared_native_staking,
            shared_staking_manager,
            matching_engine_key,
            vec![], //TODO! fetch these slave keys using Oyster KMS
            shared_local_ask_store.clone(),
            shared_generator_store,
            shared_market_store.clone(),
            shared_key_store,
            shared_cost_store,
            shared_symbiotic_staking_store,
            shared_native_store,
            shared_stake_manager_store,
            chain_id,
            unhandled_logs,
            path_to_snapshot,
        );

        let parser = Arc::new(log_parser);

        let parser_handle = tokio::spawn(async move {
            parser.parse().await.unwrap();
            Ok(())
        });

        handles.push(parser_handle);

        for handle in handles {
            let _ = handle.await;
        }

        println!("All tasks completed or shutdown.");

        Ok(())
    }
}
