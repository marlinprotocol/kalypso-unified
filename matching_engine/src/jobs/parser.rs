use anyhow::Result;
use ethers::prelude::*;
use k256::ecdsa::SigningKey;
use secret_input_helpers::secret_inputs_helpers;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use tokio::sync::Mutex;

use crate::{
    ask::{self, LocalAsk, LocalAskStore, MarketMetadataStore},
    generator::{self, GeneratorState, GeneratorStore, KeyStore},
    log_processor,
};

type EntityRegistryInstance = bindings::entity_key_registry::EntityKeyRegistry<
    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
>;

type ProofMarketplaceInstance = bindings::proof_marketplace::ProofMarketplace<
    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
>;

type GeneratorRegistryInstance = bindings::generator_registry::GeneratorRegistry<
    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
>;

pub struct LogParser {
    should_stop: Arc<AtomicBool>,
    start_block: Arc<Mutex<U64>>,
    block_range: U64,
    confirmations: U64,
    proof_marketplace: ProofMarketplaceInstance,
    generator_registry: GeneratorRegistryInstance,
    entity_registry: EntityRegistryInstance,
    provider_http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    matching_engine_key: Vec<u8>,
    shared_local_ask_store: Arc<Mutex<LocalAskStore>>,
    shared_generator_store: Arc<Mutex<GeneratorStore>>,
    shared_market_store: Arc<Mutex<MarketMetadataStore>>,
    shared_key_store: Arc<Mutex<KeyStore>>,
    chain_id: String,
}

impl LogParser {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        should_stop: Arc<AtomicBool>,
        rpc_url: String,
        relayer_signer: Wallet<SigningKey>,
        start_block: Arc<Mutex<U64>>,
        block_range: U64,
        confirmations: U64,
        proof_marketplace: ProofMarketplaceInstance,
        generator_registry: GeneratorRegistryInstance,
        entity_registry: EntityRegistryInstance,
        matching_engine_key: String,
        shared_local_ask_store: Arc<Mutex<LocalAskStore>>,
        shared_generator_store: Arc<Mutex<GeneratorStore>>,
        shared_market_store: Arc<Mutex<MarketMetadataStore>>,
        shared_key_store: Arc<Mutex<KeyStore>>,
        chain_id: String,
    ) -> Self {
        let provider_http = Provider::<Http>::try_from(&rpc_url)
            .unwrap()
            .with_signer(relayer_signer.clone());
        let provider_http = Arc::new(provider_http);

        LogParser {
            should_stop,
            start_block,
            block_range,
            confirmations,
            proof_marketplace,
            generator_registry,
            entity_registry,
            provider_http,
            matching_engine_key: hex::decode(matching_engine_key).unwrap(),
            shared_local_ask_store,
            shared_generator_store,
            shared_market_store,
            shared_key_store,
            chain_id,
        }
    }

    pub async fn parse(&self) -> anyhow::Result<()> {
        loop {
            if self.should_stop.load(Ordering::Acquire) {
                log::info!("Gracefully shutting down...");
                break;
            }

            let (mut start_block, end_block) = match self.get_start_end_block().await {
                Ok(data) => data,
                Err(_) => continue,
            };
            if start_block + self.confirmations <= end_block {
                log::info!(
                    "Processing blocks from {:?} to {:?}",
                    start_block,
                    end_block
                );

                let proof_marketplace_address = self.proof_marketplace.address();
                let generator_registry_address = self.generator_registry.address();
                let entity_key_registry_address = self.entity_registry.address();

                let filter = Filter::default()
                    .from_block(start_block)
                    .to_block(end_block)
                    .address(vec![
                        proof_marketplace_address,
                        generator_registry_address,
                        entity_key_registry_address,
                    ]);

                let logs = match self.provider_http.get_logs(&filter).await {
                    Ok(data) => data,
                    _ => {
                        log::error!("Sleeping the thread for logs to avoid rate limit");
                        thread::sleep(Duration::from_millis(5000));
                        continue;
                    }
                };

                let mut grouped_logs: HashMap<U64, Vec<Log>> = HashMap::new();

                for log in logs {
                    let block_number = log.block_number.unwrap_or_default();
                    grouped_logs.entry(block_number).or_default().push(log);
                }

                let mut sorted_block_numbers: Vec<U64> = grouped_logs.keys().cloned().collect();
                sorted_block_numbers.sort();

                for block_number in sorted_block_numbers {
                    log::debug!("Processing block {}", block_number);
                    if let Some(group) = grouped_logs.get(&block_number) {
                        for log in group {
                            log::debug!(
                                "Processing logs for block number: {:?}, log-index: {:?}",
                                block_number,
                                log.log_index
                            );

                            if log.address.eq(&proof_marketplace_address) {
                                log_processor::pm::process_proof_market_place_logs(
                                    vec![log.clone()],
                                    self.proof_marketplace.clone(),
                                    &self.shared_local_ask_store,
                                    &self.shared_generator_store,
                                    &self.shared_market_store,
                                    &self.matching_engine_key,
                                )
                                .await
                                .unwrap();

                                continue;
                            }
                            if log.address.eq(&generator_registry_address) {
                                log_processor::gr::process_generator_registry_logs(
                                    vec![log.clone()],
                                    self.generator_registry.clone(),
                                    &self.shared_generator_store,
                                )
                                .await
                                .unwrap();

                                continue;
                            }

                            if log.address.eq(&entity_key_registry_address) {
                                log_processor::er::process_entity_key_registry_logs(
                                    vec![log.clone()],
                                    self.entity_registry.clone(),
                                    &self.shared_key_store,
                                )
                                .await
                                .unwrap();
                                continue;
                            }

                            log::error!("Log of unknown contract found {:?}", log.address);
                            return Err(anyhow::anyhow!("Unknown log"));
                        }
                    }
                    log::debug!("Processed block {}", block_number);
                }

                start_block = end_block + 1;
                *self.start_block.lock().await = start_block;
                continue;
            }
            match self.create_match(end_block).await {
                Ok(_) => continue,
                Err(_) => break,
            }
        }
        Ok(())
    }

    async fn get_start_end_block(&self) -> Result<(U64, U64), Box<dyn std::error::Error>> {
        let latest_block = match self.provider_http.get_block_number().await {
            Ok(data) => data,
            _ => {
                log::error!("Sleeping the thread for latest block fetch to avoid rate limit");
                thread::sleep(Duration::from_millis(5000));
                return Err("Failed fetching latest block number".into());
            }
        };
        let start_block = { *self.start_block.lock().await };
        let end_block = if start_block + self.block_range > latest_block {
            latest_block - 1
        } else {
            start_block + self.block_range - 1
        };

        Ok((start_block, end_block))
    }

    async fn create_match(&self, end_block: U64) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("processed till {:?}. Waiting for new blocks", end_block);
        let mut ask_store = { self.shared_local_ask_store.lock().await };

        log::debug!("Trying to fetch available asks");
        let available_asks = ask_store
            .get_by_state(ask::AskState::Create)
            .filter_by_flag(true)
            .result();
        log::debug!("Complete fetch available asks");

        if available_asks.is_none() {
            thread::sleep(Duration::from_millis(60));
            return Ok(());
        }

        let available_asks = available_asks.unwrap();
        log::warn!("available asks: {}", available_asks.len());

        let mut task_list = vec![];

        for random_pending_ask in available_asks {
            log::debug!("Trying to fetch idle generators");
            let idle_generators = self
                .get_idle_generators(
                    random_pending_ask.clone(),
                    &self.shared_generator_store,
                    &self.shared_market_store,
                    &self.shared_key_store,
                    random_pending_ask.reward,
                )
                .await;
            log::warn!("idle generators: {}", &idle_generators.len());
            log::debug!("Fetched idle generators");

            if !idle_generators.is_empty() {
                let mut generator_store = { self.shared_generator_store.lock().await };
                let key_store = { self.shared_key_store.lock().await };
                let idle_generator =
                    generator::random_generator_selection(idle_generators).unwrap();

                let mut new_acl = Bytes::from_str("0x").unwrap().to_vec();

                if random_pending_ask.has_private_inputs {
                    let acl_data = random_pending_ask.secret_acl.clone().unwrap();

                    let cipher = secret_inputs_helpers::decrypt_ecies(
                        &self.matching_engine_key.to_vec(),
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
                let ask_state = match self
                    .proof_marketplace
                    .get_ask_state(random_pending_ask.ask_id)
                    .await
                {
                    Ok(data) => data,
                    _ => {
                        log::error!(
                            "Skipping ask {} because no status received from chain state",
                            random_pending_ask.ask_id
                        );
                        thread::sleep(Duration::from_millis(5000));
                        continue;
                    }
                };
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

                let generator_state = match self
                    .generator_registry
                    .get_generator_state(idle_generator.address, idle_generator.market_id)
                    .await
                {
                    Ok(data) => data,
                    _ => {
                        log::error!("Skipping ask {} because no generator {}-market-{} status received from chain state", random_pending_ask.ask_id, idle_generator.address, idle_generator.market_id);
                        thread::sleep(Duration::from_millis(5000));
                        continue;
                    }
                };
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
                    let market_store = { self.shared_market_store.lock().await };
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
            } else {
                log::debug!(
                    "Can't find idle-generators for ask {:?}, market_id: {:?}",
                    random_pending_ask.ask_id,
                    random_pending_ask.market_id
                );
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

                    for pending_task in task_list.clone() {
                        let pending_ask = pending_task.0;
                        let idle_generator = pending_task.1;
                        let new_acl = pending_task.2;

                        ask_ids.push(pending_ask.ask_id);
                        generators.push(idle_generator.address);
                        new_acls.push(ethers::types::Bytes::from(new_acl));
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

                    let matching_engine_key = hex::encode(&self.matching_engine_key);
                    let matching_engine_signer = matching_engine_key
                        .parse::<LocalWallet>()
                        .unwrap()
                        .with_chain_id(U64::from_dec_str(&self.chain_id).unwrap().as_u64());

                    let signature = matching_engine_signer
                        .sign_message(ethers::types::H256(digest))
                        .await?;
                    println!("Signature: {:?}", signature);
                    log::info!("Tx signed at {:?}", std::time::Instant::now());

                    // todo!("create and broad cast tx");
                    // // Assign batch task here
                    let batch_relay_tx_pending = self.proof_marketplace.relay_batch_assign_tasks(
                        ask_ids.clone(),
                        generators.clone(),
                        new_acls.clone(),
                        ethers::types::Bytes::from_str(&signature.to_string()).unwrap(),
                    );

                    log::info!("Tx created at {:?}", std::time::Instant::now());

                    let batch_relay_tx = match batch_relay_tx_pending.send().await {
                        Ok(data) => data.confirmations(10),
                        _ => {
                            log::error!("failed sending the transaction");
                            thread::sleep(Duration::from_millis(5000));
                            continue;
                        }
                    };

                    let batch_relay_tx = batch_relay_tx.await.unwrap().unwrap();

                    log::info!(
                        "Relayed {:?} requests tx: {:?}",
                        ask_ids.clone().len(),
                        batch_relay_tx.transaction_hash
                    );

                    log::info!("Tx mined at {:?}", std::time::Instant::now());
                }
            }
        }

        if self.should_stop.load(Ordering::Acquire) {
            log::info!("Gracefully shutting down...");
            return Err("Stopped Match Making".into());
        }

        // to avoid rate limit
        thread::sleep(Duration::from_millis(600));
        // timeout.join().unwrap();

        Ok(())
    }

    async fn get_idle_generators(
        &self,
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
}
