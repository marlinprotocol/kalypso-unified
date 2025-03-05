use market_metadata::MarketMetadataStore;

use costs::CostStoreOperations;
use ethers::prelude::*;
use jobs::{parser::LogParser, server::MatchingEngineServer};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use ethers::types::U64;

use crate::in_memory_stores::cost_store::CostStore;
use crate::in_memory_stores::generator_store::GeneratorStore;
use crate::in_memory_stores::key_store::KeyStore;
use crate::in_memory_stores::native_stake_store::NativeStakingStore;
use crate::in_memory_stores::stake_manager_store::StakeManagerStore;
use crate::in_memory_stores::symbiotic_stake_store::SymbioticStakeStore;
use crate::postgres_stores::initialize_pool::init_pool;
use crate::postgres_stores::models::{AskDatabase, PrivateInputStore};
use crate::{costs, jobs, market_metadata, MatchingEngineConfig};

pub struct PostgresMatchingEngine {
    config: MatchingEngineConfig,
    matching_engine_port: u16,
}

impl PostgresMatchingEngine {
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

    pub async fn run(&self, path_to_snapshot: String) -> anyhow::Result<()> {
        let database_url = "DATABASE_URL";
        let pool = init_pool(&database_url);

        // Create both structs sharing the same pool.
        let postgres_ask_store = AskDatabase {
            pool: pool.clone(),
            private_store: PrivateInputStore::new(),
        };

        let generator_list_store = GeneratorStore::default();
        let key_list_store = KeyStore::default();
        let cost_store = CostStore::new();
        let market_list_store = MarketMetadataStore::default();
        let symbiotic_staking_store = SymbioticStakeStore::default();
        let native_staking_store = NativeStakingStore::default();
        let stake_manager_store = StakeManagerStore::default();
        let start_block_string = self.config.clone().start_block;

        // wrapping around is case to shared across threads
        let shared_postgres_ask_store = Arc::new(RwLock::new(postgres_ask_store));
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
            shared_postgres_ask_store,
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
        shared_postgres_ask_store: Arc<RwLock<AskDatabase>>,
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

        let generator_registry = bindings::prover_manager::ProverManager::new(
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
        let shared_postgres_ask_data = Arc::clone(&shared_postgres_ask_store);

        let matching_engine_key_for_server = hex::decode(matching_engine_key.clone()).unwrap();
        let shared_matching_key = Arc::new(RwLock::new(matching_engine_key_for_server));
        let shared_matching_key_clone = Arc::clone(&shared_matching_key);

        let unhandled_logs = Arc::new(RwLock::new(vec![]));
        let matching_errors = Arc::new(RwLock::new(vec![]));

        let backup_in_progress = Arc::new(RwLock::new(false));

        let should_stop = Arc::new(AtomicBool::new(false));
        let stop_handle_clone = should_stop.clone();

        let mut handles = vec![];

        let stop_handle_clone1 = stop_handle_clone.clone();
        let stop_handle_clone2 = stop_handle_clone.clone();

        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            stop_handle_clone.store(true, Ordering::Release);
        });

        let server = MatchingEngineServer::new(
            shared_market_data,
            shared_postgres_ask_data,
            shared_parsed_block.clone(),
            shared_matching_key_clone,
            shared_entity_key_registry,
            shared_generator_data.clone(),
            shared_native_store.clone(),
            shared_symbiotic_staking_store.clone(),
            shared_key_store.clone(),
            shared_cost_store.clone(),
            shared_stake_manager_store.clone(),
            relayer_key_balance.clone(),
            should_stop.clone(),
            unhandled_logs.clone(),
            matching_errors.clone(),
        );

        let matching_engine_port = self.matching_engine_port;

        let server_handle: JoinHandle<Result<(), anyhow::Error>> = tokio::spawn(async move {
            match server.start_server(matching_engine_port, false).await {
                Ok(_) => Ok(()), // If successful, return Ok(()).
                Err(e) => {
                    log::error!("Server failed to start: {}", e); // Log the error.
                    stop_handle_clone1.store(true, Ordering::Release); // Signal shutdown.
                    Err(e.into()) // Propagate the error.
                }
            }
        });
        handles.push(server_handle);

        let confirmations = 5; // ideally this should be more
        let block_range = 20000; // Number of blocks to fetch logs from at once
        let should_stop_clone = should_stop.clone();

        let log_parser = LogParser::new(
            should_stop_clone,
            rpc_url,
            relayer_signer.clone(),
            shared_parsed_block.clone(),
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
            shared_postgres_ask_store.clone(),
            shared_generator_store,
            shared_market_store.clone(),
            shared_key_store.clone(),
            shared_cost_store.clone(),
            shared_symbiotic_staking_store.clone(),
            shared_native_store.clone(),
            shared_stake_manager_store.clone(),
            chain_id,
            unhandled_logs,
            matching_errors,
            backup_in_progress.clone(),
        );

        let parser = Arc::new(log_parser);

        let parser_handle: JoinHandle<Result<(), anyhow::Error>> = tokio::spawn(async move {
            match parser.parse().await {
                Ok(_) => Ok(()),
                Err(e) => {
                    log::error!("Parser failed: {}", e); // Log the error.
                    stop_handle_clone2.store(true, Ordering::Release); // Signal shutdown.
                    Err(e.into()) // Propagate the error.
                }
            }
        });

        handles.push(parser_handle);

        for handle in handles {
            let _ = handle.await;
        }

        println!("All tasks completed or shutdown.");

        Ok(())
    }
}
