pub mod ask_lib;
pub mod costs;
pub mod counters;
pub mod market_metadata;
pub mod models;
pub mod utility;

mod generator_lib;
mod jobs;
mod log_processor;
mod routes;

use ask_lib::ask_store::LocalAskStore;
use market_metadata::MarketMetadataStore;

use costs::CostStore;
use ethers::prelude::*;
use generator_lib::{generator_store::GeneratorStore, key_store::KeyStore};
use jobs::{cleanup::CleanupTool, parser::LogParser, server::MatchingEngineServer};
use models::{GetAskStatus, MarketInfo};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use service_check_helper::{Request, RequestType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

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
    pub start_block: String,
}

pub struct MatchingEngine {
    config: MatchingEngineConfig,
    matching_engine_port: u16,
}

impl MatchingEngine {
    pub fn from_config(config: MatchingEngineConfig) -> Self {
        Self {
            config,
            matching_engine_port: 3000,
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
        start_block: String,
    ) -> Self {
        let config: MatchingEngineConfig = MatchingEngineConfig {
            rpc_url,
            chain_id,
            matching_engine_key,
            relayer_private_key,
            proof_market_place,
            generator_registry,
            entity_registry,
            start_block,
        };

        Self::from_config(config)
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let local_ask_store = LocalAskStore::new();
        let generator_list_store = GeneratorStore::new();
        let key_list_store = KeyStore::new();
        let cost_store = CostStore::new();
        let market_list_store = MarketMetadataStore::new();

        // wrapping around is case to shared across threads
        let shared_local_ask_store = Arc::new(RwLock::new(local_ask_store));
        let shared_generator_store = Arc::new(RwLock::new(generator_list_store));
        let shared_market_store = Arc::new(RwLock::new(market_list_store));
        let shared_key_store = Arc::new(RwLock::new(key_list_store));
        let shared_cost_store = Arc::new(RwLock::new(cost_store));
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

        let start_block_string = self.config.clone().start_block;

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

        let shared_parsed_block_number_store = Arc::new(RwLock::new(
            U64::from_dec_str(&start_block_string).expect("Unable to rad start_block"),
        ));
        let shared_parsed_block = Arc::clone(&shared_parsed_block_number_store);

        let shared_market_data = Arc::clone(&shared_market_store);
        let shared_generator_data = Arc::clone(&shared_generator_store);
        let shared_local_ask_data = Arc::clone(&shared_local_ask_store);

        let matching_engine_key_for_server = hex::decode(matching_engine_key.clone()).unwrap();
        let shared_matching_key = Arc::new(RwLock::new(matching_engine_key_for_server));
        let shared_matching_key_clone = Arc::clone(&shared_matching_key);

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
            relayer_key_balance.clone(),
            should_stop.clone(),
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
        let parser = Arc::new(LogParser::new(
            should_stop_clone,
            rpc_url,
            relayer_signer.clone(),
            shared_parsed_block,
            block_range.into(),
            confirmations.into(),
            proof_marketplace.clone(),
            generator_registry,
            entity_key_registry,
            matching_engine_key,
            vec![], //TODO! fetch these slave keys using Oyster KMS
            shared_local_ask_store.clone(),
            shared_generator_store,
            shared_market_store.clone(),
            shared_key_store,
            shared_cost_store,
            chain_id,
        ));

        let parser_handle = tokio::spawn(async move {
            parser.parse().await.unwrap();
            Ok(())
        });

        handles.push(parser_handle);

        let cleanup_tool = CleanupTool::new(
            should_stop,
            shared_local_ask_store,
            proof_marketplace,
            relayer_signer.address(),
            relayer_key_balance,
        );
        let cleanup_handle = tokio::spawn(async move {
            match cleanup_tool.start_cleanup(true, true).await {
                _ => Ok(()),
            }
        });
        handles.push(cleanup_handle);

        for handle in handles {
            let _ = handle.await;
        }

        println!("All tasks completed or shutdown.");

        Ok(())
    }
}
