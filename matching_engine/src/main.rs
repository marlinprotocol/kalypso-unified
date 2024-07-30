use ask::{LocalAskStore, MarketMetadataStore};
use dotenv::dotenv;
use ethers::prelude::*;
use generator::{GeneratorStore, KeyStore};
use jobs::cleanup::CleanupTool;
use jobs::parser::LogParser;
use jobs::server::MatchingEngineServer;
use secret_input_helpers::secret_inputs_helpers;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fs, str::FromStr, sync::Arc};
use tokio::sync::Mutex;

mod ask;
mod generator;
mod jobs;
mod log_processor;
mod middlewares;
mod routes;
mod utility;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MatchingEngineConfig {
    rpc_url: String,
    chain_id: String,
    matching_engine_key: String,
    relayer_private_key: String,
    proof_market_place: String,
    generator_registry: String,
    entity_registry: String,
    start_block: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let matching_engine_config_path =
        "../matching_engine_config/matching_engine_config.json".to_string();
    let alt_matching_engine_config_path =
        "./matching_engine_config/matching_engine_config.json".to_string();
    let file_content = fs::read_to_string(&matching_engine_config_path)
        .or_else(|_| fs::read_to_string(&alt_matching_engine_config_path))?;
    let config: MatchingEngineConfig = serde_json::from_str(&file_content)?;

    let local_ask_store = LocalAskStore::new();
    let generator_list_store = GeneratorStore::new();
    let key_list_store = KeyStore::new();
    let market_list_store = MarketMetadataStore::new();

    // wrapping around is case to shared across threads
    let shared_local_ask_store = Arc::new(Mutex::new(local_ask_store));
    let shared_generator_store = Arc::new(Mutex::new(generator_list_store));
    let shared_market_store = Arc::new(Mutex::new(market_list_store));
    let shared_key_store = Arc::new(Mutex::new(key_list_store));

    let rpc_url = config.rpc_url;
    let chain_id = config.chain_id;

    let matching_engine_key = config.matching_engine_key;
    let matching_engine_signer = matching_engine_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(U64::from_dec_str(&chain_id).unwrap().as_u64());

    let relayer_key = config.relayer_private_key;
    let relayer_signer = relayer_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(U64::from_dec_str(&chain_id).unwrap().as_u64());

    let start_block_string = config.start_block;

    log::info!(
        "matching engine address {:?}",
        matching_engine_signer.clone().address()
    );

    log::info!("relayer address {:?}", relayer_signer.clone().address());

    let provider_http = Provider::<Http>::try_from(&rpc_url)
        .unwrap()
        // .with_signer(matching_engine_signer.clone());
        .with_signer(relayer_signer.clone());
    let client = Arc::new(provider_http.clone());

    // Creating contract instance for proof market place
    let proof_market_place_var = config.proof_market_place;
    let proof_marketplace_address = Address::from_str(&proof_market_place_var).unwrap();

    let proof_marketplace = bindings::proof_marketplace::ProofMarketplace::new(
        proof_marketplace_address,
        client.clone(),
    );

    // Creating contract instance for generator registry
    let generator_registry_var = config.generator_registry;
    let generator_registry_address = Address::from_str(&generator_registry_var).unwrap();

    let generator_registry = bindings::generator_registry::GeneratorRegistry::new(
        generator_registry_address,
        client.clone(),
    );

    let entity_key_registry_var = config.entity_registry;
    let entity_key_registry_address = Address::from_str(&entity_key_registry_var).unwrap();

    let entity_key_registry = bindings::entity_key_registry::EntityKeyRegistry::new(
        entity_key_registry_address,
        client.clone(),
    );

    let shared_entity_key_registry = bindings::entity_key_registry::EntityKeyRegistry::new(
        entity_key_registry_address,
        client.clone(),
    );
    let shared_entity_key = Arc::new(Mutex::new(shared_entity_key_registry));
    let clone_shared_entity_key = Arc::clone(&shared_entity_key);

    let shared_parsed_store = Arc::new(Mutex::new(
        U64::from_dec_str(&start_block_string).expect("Unable to rad start_block"),
    ));
    let shared_parsed_block = Arc::clone(&shared_parsed_store);

    let shared_market_data = Arc::clone(&shared_market_store);
    let shared_generator_data = Arc::clone(&shared_generator_store);
    let shared_local_ask_data = Arc::clone(&shared_local_ask_store);

    let matching_engine_key_for_server = hex::decode(matching_engine_key.clone()).unwrap();
    let shared_matching_key = Arc::new(Mutex::new(matching_engine_key_for_server));
    let shared_matching_key_clone = Arc::clone(&shared_matching_key);

    let should_stop = Arc::new(AtomicBool::new(false));
    let stop_handle = should_stop.clone();

    let mut handles = vec![];

    let ctrl_c_handle = tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        stop_handle.store(true, Ordering::Release);
        Ok(())
    });
    handles.push(ctrl_c_handle);

    let server = MatchingEngineServer::new(
        shared_market_data,
        shared_local_ask_data,
        shared_parsed_block.clone(),
        shared_matching_key_clone,
        clone_shared_entity_key,
        shared_generator_data,
    );

    let server_handle = tokio::spawn(server.start_server());
    handles.push(server_handle);

    let confirmations = 10; // ideally this should be more
    let block_range = 20000; // Number of blocks to fetch logs from at once
    let parser = Arc::new(LogParser::new(
        should_stop.clone(),
        rpc_url,
        relayer_signer,
        shared_parsed_block,
        block_range.into(),
        confirmations.into(),
        proof_marketplace.clone(),
        generator_registry,
        entity_key_registry,
        matching_engine_key,
        shared_local_ask_store.clone(),
        shared_generator_store,
        shared_market_store,
        shared_key_store,
        chain_id,
    ));

    let parser_handle = tokio::spawn(async move {
        parser.parse().await.unwrap();
        Ok(())
    });

    handles.push(parser_handle);

    let cleanup_tool = CleanupTool::new(should_stop, shared_local_ask_store, proof_marketplace);
    let cleanup_handle = tokio::spawn(async move {
        cleanup_tool.ask_store_cleanup().await.unwrap();
        Ok(())
    });
    handles.push(cleanup_handle);

    for handle in handles {
        let _ = handle.await;
    }

    println!("All tasks completed or shutdown.");

    Ok(())
}
