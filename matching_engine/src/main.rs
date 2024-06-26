use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use ethers::prelude::{k256::ecdsa::SigningKey, *};
use itertools::Itertools;
use secret_input_helpers::secret_inputs_helpers;
use std::ops::{Add, Sub};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fs, str::FromStr, sync::Arc, thread, time::Duration};
use tokio::sync::Mutex;

mod ask;
// mod utility;
mod generator;
mod log_processor;
mod routes;
mod utility;

use tokio::runtime::Runtime;

use ask::{LocalAsk, LocalAskStore, MarketMetadataStore};
use generator::{GeneratorStore, KeyStore};

use crate::generator::GeneratorState;

use serde::{Deserialize, Serialize};

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

    let mut start_block: U64 = U64::from_dec_str(&start_block_string).unwrap();
    let parsed_block = start_block;

    let confirmations = 10; // ideally this should be more
    let block_range = 20000; // Number of blocks to fetch logs from at once

    let shared_parsed_store = Arc::new(Mutex::new(parsed_block));
    let shared_parsed_block = Arc::clone(&shared_parsed_store);

    let shared_market_data = Arc::clone(&shared_market_store);
    let shared_generator_data = Arc::clone(&shared_generator_store);
    let shared_local_ask_data = Arc::clone(&shared_local_ask_store);

    let matching_engine_key_for_server = hex::decode(matching_engine_key.clone()).unwrap();
    let shared_matching_key = Arc::new(Mutex::new(matching_engine_key_for_server));
    let shared_matching_key_clone = Arc::clone(&shared_matching_key);

    let server_handle = thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(async {
            HttpServer::new(move || {
                use actix_extensible_rate_limit::{
                    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
                    RateLimiter,
                };
                let backend = InMemoryBackend::builder().build();
                let input = SimpleInputFunctionBuilder::new(Duration::from_secs(1), 5)
                    .real_ip_key()
                    .build();
                let middleware = RateLimiter::builder(backend.clone(), input)
                    .add_headers()
                    .build();
                App::new()
                    .wrap(middleware)
                    .app_data(Data::new(shared_market_data.clone()))
                    .app_data(Data::new(shared_local_ask_data.clone()))
                    .app_data(Data::new(shared_parsed_block.clone()))
                    .app_data(Data::new(shared_matching_key_clone.clone()))
                    .app_data(Data::new(clone_shared_entity_key.clone()))
                    .app_data(Data::new(shared_generator_data.clone()))
                    .route("/welcome", web::get().to(routes::welcome)) // Route to welcome endpoint
                    .route("/getStatus", web::get().to(routes::get_status)) // Route to all ask status
                    .route(
                        "/getAskStatus",
                        web::post().to(routes::get_ask_status_askid),
                    ) // Provide specific ask status
                    .route("/getPrivInput", web::post().to(routes::get_priv_input)) // provide private inputs for a specific ask
                    .route("/decryptRequest", web::post().to(routes::decrypt_request)) // Return decrypted input
                    .route(
                        "/getLatestBlock",
                        web::get().to(routes::get_latest_block_number), // Returns the latest Block parsed so far
                    )
                    .route("/marketInfo", web::post().to(routes::market_info))
            })
            .bind("0.0.0.0:3000")?
            .run()
            .await
        });

        match result {
            Ok(_) => log::warn!("Server stopped successfully."),
            Err(e) => log::error!("Server error: {}", e),
        }
    });

    let should_stop = Arc::new(AtomicBool::new(false));
    let stop_handle = should_stop.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        stop_handle.store(true, Ordering::Release);
    });

    loop {
        if should_stop.load(Ordering::Acquire) {
            log::info!("Gracefully shutting down...");
            break;
        }

        let latest_block = provider_http.get_block_number().await.unwrap();
        let end_block = if start_block + block_range > latest_block {
            latest_block - 1
        } else {
            start_block + block_range - 1
        };

        if start_block + confirmations <= end_block {
            log::info!(
                "Processing blocks from {:?} to {:?}",
                start_block,
                end_block
            );

            let filter = Filter::default()
                .from_block(start_block)
                .to_block(end_block)
                .address(vec![
                    proof_marketplace_address,
                    generator_registry_address,
                    entity_key_registry_address,
                ]);

            let logs = provider_http.get_logs(&filter).await?;

            let grouped_logs = &logs
                .into_iter()
                .group_by(|log| log.block_number.unwrap_or_default());

            for (block_number, group) in grouped_logs {
                log::debug!("Processing block {}", block_number);
                for log in group {
                    log::debug!(
                        "Processing logs for block number: {:?}, log-index: {:?}",
                        block_number,
                        log.log_index
                    );

                    let matching_engine_key = hex::decode(matching_engine_key.clone()).unwrap();
                    // Process each log
                    if log.address.eq(&proof_marketplace_address) {
                        log_processor::pm::process_proof_market_place_logs(
                            vec![log],
                            proof_marketplace.clone(),
                            &shared_local_ask_store,
                            &shared_generator_store,
                            &shared_market_store,
                            &matching_engine_key,
                        )
                        .await?;

                        continue;
                    }

                    if log.address.eq(&generator_registry_address) {
                        log_processor::gr::process_generator_registry_logs(
                            vec![log],
                            generator_registry.clone(),
                            &shared_generator_store,
                        )
                        .await?;

                        continue;
                    }

                    if log.address.eq(&entity_key_registry_address) {
                        log_processor::er::process_entity_key_registry_logs(
                            vec![log],
                            entity_key_registry.clone(),
                            &shared_key_store,
                        )
                        .await?;
                        continue;
                    }

                    log::error!("Log of unknown contract found {:?}", log.address);
                    return Err("Unknown log".into());
                }

                log::debug!("Processed block {}", block_number);
            }

            start_block = end_block + 1;
            let mut latest_parsed_block = shared_parsed_store.lock().await;
            *latest_parsed_block = start_block;
            continue;
        }

        log::debug!("processed till {:?}. Waiting for new blocks", end_block);
        let mut ask_store = shared_local_ask_store.lock().await;

        // Cleaning the stale ask requests
        // let timeout = thread::spawn(move || {
        //     ask_store_cleanup(&shared_local_ask_store.clone(), proof_marketplace.clone());
        //     thread::sleep(Duration::from_secs(86400));
        // });

        log::debug!("Trying to fetch available asks");
        let available_asks = ask_store
            .get_by_state(ask::AskState::Create)
            .filter_by_flag(true)
            .result();
        log::debug!("Complete fetch available asks");

        if available_asks.is_none() {
            thread::sleep(Duration::from_millis(60));
            continue;
        }

        let available_asks = available_asks.unwrap();
        log::warn!("available asks: {}", available_asks.len());

        let mut task_list = vec![];
        for random_pending_ask in available_asks {
            log::debug!("Trying to fetch idle generators");
            let idle_generators = get_idle_generators(
                random_pending_ask.clone(),
                &shared_generator_store,
                &shared_market_store,
                &shared_key_store,
                random_pending_ask.reward,
            )
            .await;
            log::warn!("idle generators: {}", &idle_generators.len());
            log::debug!("Fetched idle generators");

            if !idle_generators.is_empty() {
                // assign task here
                let mut generator_store = shared_generator_store.lock().await;
                let key_store = shared_key_store.lock().await;
                let idle_generator =
                    generator::random_generator_selection(idle_generators).unwrap();

                let mut new_acl = Bytes::from_str("0x").unwrap().to_vec();
                if random_pending_ask.has_private_inputs {
                    let acl_data = random_pending_ask.secret_acl.clone().unwrap();
                    let matching_engine_key: Vec<u8> =
                        hex::decode(matching_engine_key.clone()).unwrap();

                    let cipher = secret_inputs_helpers::decrypt_ecies(
                        &matching_engine_key.to_vec(),
                        &acl_data,
                    )?;

                    let generator_ecies_pub_key = key_store
                        .get_by_address(&idle_generator.address, idle_generator.market_id.as_u64())
                        .unwrap()
                        .ecies_pub_key
                        .clone()
                        .unwrap()
                        .to_vec();
                    new_acl = secret_inputs_helpers::encrypt_ecies(
                        &generator_ecies_pub_key,
                        cipher.as_slice(),
                    )?;
                }

                // state confirmation
                let ask_state = proof_marketplace
                    .get_ask_state(random_pending_ask.ask_id)
                    .await
                    .unwrap();
                let ask_state = ask::get_ask_state(ask_state);

                if ask_state != ask::AskState::Create {
                    log::warn!(
                        "ask {:?}. {:?}. skipping it",
                        random_pending_ask.ask_id,
                        ask_state
                    );
                    ask_store.modify_state(&random_pending_ask.ask_id, ask_state);
                    continue;
                }

                let generator_state = generator_registry
                    .get_generator_state(idle_generator.address, idle_generator.market_id)
                    .await
                    .unwrap();
                let generator_state = generator::get_generator_state(generator_state.0);

                if generator_state != generator::GeneratorState::Joined
                    && generator_state != generator::GeneratorState::Wip
                {
                    log::warn!(
                        "Generator {:?}. {:?}",
                        idle_generator.address,
                        generator_state
                    );
                    generator_store.update_state(
                        &idle_generator.address,
                        &idle_generator.market_id,
                        generator_state,
                    );
                } else {
                    let market_store = shared_market_store.lock().await;
                    let slashing_penalty = market_store
                        .get_slashing_penalty_by_market_id(&idle_generator.market_id)
                        .unwrap();
                    let generator_global = generator_store
                        .get_by_address(&idle_generator.address)
                        .unwrap();
                    let remaining_stake = generator_global
                        .total_stake
                        .sub(generator_global.stake_locked.add(slashing_penalty));
                    let remaining_compute = generator_global.declared_compute.sub(
                        generator_global
                            .compute_consumed
                            .add(idle_generator.compute_required_per_request),
                    );
                    if remaining_stake.lt(&slashing_penalty)
                        || remaining_compute.lt(&idle_generator.compute_required_per_request)
                    {
                        generator_store.update_state(
                            &idle_generator.address,
                            &idle_generator.market_id,
                            GeneratorState::PendingConfirmation,
                        );
                    }
                }

                log::info!(
                    "Assigned ask: {} to generator: {}, at {:?}",
                    &random_pending_ask.ask_id,
                    &idle_generator.address,
                    // provider_http.get_block_number().await.unwrap()
                    std::time::Instant::now()
                );
                task_list.push((random_pending_ask, idle_generator.clone(), new_acl.clone()));

                // if task_list.len() >= 10 {
                //     break;
                // }
            } else {
                log::debug!(
                    "Can't find idle-generators for ask {:?}, market_id: {:?}",
                    random_pending_ask.ask_id,
                    random_pending_ask.market_id
                );
            }
        }

        match task_list.len() {
            0 => {
                log::warn!("No Matches");
                thread::sleep(Duration::from_millis(60));
            }
            _ => {
                let mut ask_ids = vec![];
                let mut generators = vec![];
                let mut new_acls = vec![];

                #[allow(unused_variables)]
                let mut index = 0;

                for pending_task in task_list {
                    let pending_ask = pending_task.0;
                    let idle_generator = pending_task.1;
                    let new_acl = pending_task.2;

                    ask_ids.push(pending_ask.ask_id);
                    generators.push(idle_generator.address);
                    new_acls.push(ethers::types::Bytes::from(new_acl));
                    index += 1;
                }

                let values = vec![
                    ethers::abi::Token::Array(
                        ask_ids
                            .clone()
                            .into_iter()
                            .map(ethers::abi::Token::Uint)
                            .collect(),
                    ),
                    ethers::abi::Token::Array(
                        generators
                            .clone()
                            .into_iter()
                            .map(ethers::abi::Token::Address)
                            .collect(),
                    ),
                    ethers::abi::Token::Array(
                        new_acls
                            .clone()
                            .into_iter()
                            .map(|v| ethers::abi::Token::Bytes(v.to_vec()))
                            .collect(),
                    ),
                ];

                let encoded = ethers::abi::encode(&values);
                let digest = ethers::utils::keccak256(encoded);

                let signature = matching_engine_signer
                    .sign_message(ethers::types::H256(digest))
                    .await?;
                println!("Signature: {:?}", signature);
                log::info!("Tx signed at {:?}", std::time::Instant::now());

                // todo!("create and broad cast tx");
                // // Assign batch task here
                let batch_relay_tx_pending = proof_marketplace.relay_batch_assign_tasks(
                    ask_ids.clone(),
                    generators.clone(),
                    new_acls.clone(),
                    ethers::types::Bytes::from_str(&signature.to_string()).unwrap(),
                );

                log::info!("Tx created at {:?}", std::time::Instant::now());

                let batch_relay_tx = batch_relay_tx_pending
                    .send()
                    .await
                    .unwrap()
                    .confirmations(10)
                    .await
                    .unwrap()
                    .unwrap();

                log::info!(
                    "Relayed {:?} requests tx: {:?}",
                    ask_ids.clone().len(),
                    batch_relay_tx.transaction_hash
                );

                log::info!("Tx mined at {:?}", std::time::Instant::now());
            }
        }

        if should_stop.load(Ordering::Acquire) {
            log::info!("Gracefully shutting down...");
            break;
        }

        // to avoid rate limit
        thread::sleep(Duration::from_millis(600));
        // timeout.join().unwrap();
    }

    server_handle.join().unwrap();

    Ok(())
}

async fn get_idle_generators(
    random_pending_ask: LocalAsk,
    generator_store: &Arc<Mutex<GeneratorStore>>,
    market_store: &Arc<Mutex<MarketMetadataStore>>,
    key_store: &Arc<Mutex<KeyStore>>,
    task_reward: U256,
) -> Vec<generator::GeneratorInfoPerMarket> {
    // Ensure Generator implements Clone
    let generator_store = generator_store.lock().await;
    let market_metadata_store = market_store.lock().await;
    let key_store = key_store.lock().await;
    let slashing_penalty = market_metadata_store
        .get_slashing_penalty_by_market_id(&random_pending_ask.market_id)
        .unwrap();
    let idle_generators = {
        let generator_query = {
            if random_pending_ask.has_private_inputs {
                generator_store.filter_by_has_private_inputs_support(
                    generator_store.filter_by_available_stake(
                        generator_store.filter_by_has_idle_compute(
                            generator_store
                                .query_by_state(GeneratorState::Joined)
                                .filter_by_market_id(random_pending_ask.market_id)
                                .filter_by_reward(task_reward),
                        ),
                        slashing_penalty,
                    ),
                    key_store,
                )
            } else {
                generator_store.filter_by_available_stake(
                    generator_store.filter_by_has_idle_compute(
                        generator_store
                            .query_by_state(GeneratorState::Joined)
                            .filter_by_market_id(random_pending_ask.market_id)
                            .filter_by_reward(task_reward),
                    ),
                    slashing_penalty,
                )
            }
        };

        let generators = generator_query.result();

        generator::idle_generator_selector(generators)
    };
    idle_generators
}

#[allow(unused)]
async fn ask_store_cleanup(
    ask_store: &Arc<Mutex<LocalAskStore>>,
    proof_market_place: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
) {
    let mut ask_store = ask_store.lock().await;

    // Removing the completed asks
    let completed_asks = ask_store.get_by_state(ask::AskState::Complete).result();

    if completed_asks.is_some() {
        for elem in completed_asks.unwrap() {
            ask_store.remove_by_ask_id(&elem.ask_id);
        }
    }

    // Checking and removing the assigned asks
    let assigned_asks = ask_store.get_by_state(ask::AskState::Assigned).result();

    if assigned_asks.is_some() {
        for elem in assigned_asks.unwrap() {
            let ask_state = proof_market_place.get_ask_state(elem.ask_id).await.unwrap();
            let ask_state = ask::get_ask_state(ask_state);
            if ask_state != ask::AskState::Assigned || ask_state != ask::AskState::Create {
                ask_store.remove_by_ask_id(&elem.ask_id);
            }
        }
    }

    // Checking and removing the idle asks
    let idle_asks = ask_store.get_by_state(ask::AskState::Create).result();

    if idle_asks.is_some() {
        for elem in idle_asks.unwrap() {
            let ask_state = proof_market_place.get_ask_state(elem.ask_id).await.unwrap();
            let ask_state = ask::get_ask_state(ask_state);
            if ask_state != ask::AskState::Assigned || ask_state != ask::AskState::Create {
                ask_store.remove_by_ask_id(&elem.ask_id);
            }
        }
    }
}
