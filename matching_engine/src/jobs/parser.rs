use crate::ask_lib::ask_status::{get_ask_state, AskState};
use crate::ask_lib::ask_store::LocalAskStore;
use crate::costs::CostStore;
use crate::generator_lib::generator_store;
use crate::market_metadata::MarketMetadataStore;
use anyhow::Result;
use ethers::prelude::*;
use k256::ecdsa::SigningKey;
use kalypso_helper::secret_inputs_helpers;
use std::collections::HashMap;
use std::ops::Sub;

use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use tokio::sync::RwLock;

use crate::log_processor;
use crate::{
    ask_lib::ask::LocalAsk,
    generator_lib::{
        generator_helper, generator_state::GeneratorState, generator_store::GeneratorStore,
        key_store::KeyStore,
    },
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
    start_block: Arc<RwLock<U64>>,
    block_range: U64,
    confirmations: U64,
    proof_marketplace: ProofMarketplaceInstance,
    generator_registry: GeneratorRegistryInstance,
    entity_registry: EntityRegistryInstance,
    provider_http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    matching_engine_key: Vec<u8>,
    matching_engine_slave_keys: Vec<Vec<u8>>,
    shared_local_ask_store: Arc<RwLock<LocalAskStore>>,
    shared_generator_store: Arc<RwLock<GeneratorStore>>,
    shared_market_store: Arc<RwLock<MarketMetadataStore>>,
    shared_key_store: Arc<RwLock<KeyStore>>,
    shared_cost_store: Arc<RwLock<CostStore>>,
    chain_id: String,
    max_tasks_size: usize,
}

impl LogParser {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        should_stop: Arc<AtomicBool>,
        rpc_url: String,
        relayer_signer: Wallet<SigningKey>,
        start_block: Arc<RwLock<U64>>,
        block_range: U64,
        confirmations: U64,
        proof_marketplace: ProofMarketplaceInstance,
        generator_registry: GeneratorRegistryInstance,
        entity_registry: EntityRegistryInstance,
        matching_engine_key: String,
        matching_engine_slave_keys: Vec<String>,
        shared_local_ask_store: Arc<RwLock<LocalAskStore>>,
        shared_generator_store: Arc<RwLock<GeneratorStore>>,
        shared_market_store: Arc<RwLock<MarketMetadataStore>>,
        shared_key_store: Arc<RwLock<KeyStore>>,
        shared_cost_store: Arc<RwLock<CostStore>>,
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
            matching_engine_slave_keys: matching_engine_slave_keys
                .into_iter()
                .map(|s| hex::decode(s).unwrap())
                .collect(),
            shared_local_ask_store,
            shared_generator_store,
            shared_market_store,
            shared_key_store,
            shared_cost_store,
            chain_id,
            max_tasks_size: 10, // TODO: dynamically adjust latter
        }
    }

    pub async fn parse(&self) -> anyhow::Result<()> {
        let mut matches_upto: Option<U64> = None;
        loop {
            if self.should_stop.load(Ordering::Acquire) {
                log::info!("Gracefully shutting down...");
                break;
            }

            let (mut start_block, end_block) = match self.get_start_end_block().await {
                Ok(data) => data,
                Err(_) => {
                    log::warn!("Could fetch start_block and end_block, pausing ME");
                    thread::sleep(Duration::from_secs(5));
                    continue;
                }
            };

            if let Some(matches_upto) = matches_upto.filter(|&m| m == end_block) {
                log::warn!(
                    "All matches made up to {}. Waiting for a few seconds",
                    matches_upto
                );
                thread::sleep(Duration::from_secs(5));
                continue;
            }

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
                    Err(err) => {
                        log::error!("Error fetching logs, sleeping the thread to avoid rate limit");
                        log::error!("{}", err);
                        thread::sleep(Duration::from_secs(5));
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
                                    &self.shared_cost_store,
                                    &self.matching_engine_key,
                                    &self.matching_engine_slave_keys,
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
                *self.start_block.write().await = start_block;
                continue;
            }
            
            matches_upto = match self.create_match(end_block).await {
                Ok(upto) => {
                    log::info!("Completed match assignment upto: {}", upto);
                    Some(upto)
                }
                Err(err) => {
                    log::error!("{}", err);
                    log::error!("Match Creation Failed, retyring in couple of seconds");
                    thread::sleep(Duration::from_secs(4));
                    None
                }
            };
        }
        Ok(())
    }

    async fn get_start_end_block(&self) -> Result<(U64, U64), Box<dyn std::error::Error>> {
        let latest_block = match self.provider_http.get_block_number().await {
            Ok(data) => data,
            Err(err) => {
                log::error!("Failed fetching the latest block, sleeping to avoid rate limit");
                log::error!("{}", err);
                thread::sleep(Duration::from_secs(5));
                return Err("Failed fetching latest block number".into());
            }
        };
        let start_block = { *self.start_block.read().await };
        let end_block = if start_block + self.block_range > latest_block {
            latest_block - 1
        } else {
            start_block + self.block_range - 1
        };

        Ok((start_block, end_block))
    }

    #[cfg(feature = "disable_match_creation")]
    async fn create_match(&self, end_block: U64) -> Result<U64, Box<dyn std::error::Error>> {
        thread::sleep(Duration::from_secs(5)); // just to mimic match creation time and may be free resource for else where
        Ok(end_block)
    }

    #[cfg(not(feature = "disable_match_creation"))]
    async fn create_match(&self, end_block: U64) -> Result<U64, Box<dyn std::error::Error>> {
        log::debug!("processed till {:?}. Waiting for new blocks", end_block);
        let ask_store = { self.shared_local_ask_store.read().await };
        let generator_store = { self.shared_generator_store.read().await };

        log::debug!("Trying to fetch available asks");
        let available_asks = ask_store
            .get_by_ask_state_except_complete(AskState::Create)
            .filter_by_flag(true)
            .result();
        log::debug!("Complete fetch available asks");

        if available_asks.is_none() {
            return Ok(end_block);
        }

        let available_asks = available_asks.unwrap();
        if available_asks.is_empty() {
            return Ok(end_block);
        }
        log::warn!("available asks: {}", available_asks.len());

        let mut task_list = vec![];

        let all_generators = { generator_store.all_generators_address() };
        let mut cached_stake = {
            let mut m = HashMap::new();
            for _generator in all_generators.clone() {
                let available_stake = generator_store
                    .get_available_stake(_generator.into())
                    .unwrap();
                m.insert(_generator, available_stake);
            }
            m
        };

        let mut cached_compute = {
            let mut m = HashMap::new();
            for _generator in all_generators.clone() {
                let available_compute = generator_store.get_available_compute(_generator).unwrap();
                m.insert(_generator, available_compute);
            }
            m
        };

        for random_pending_ask in available_asks {
            log::info!(
                "Finding matching generator for ask: {}",
                random_pending_ask.ask_id
            );
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

            if idle_generators.is_empty() {
                log::warn!(
                    "Can't find idle-generators for ask {:?}, market_id: {:?}",
                    random_pending_ask.ask_id,
                    random_pending_ask.market_id
                );
                continue;
            }

            let key_store = { self.shared_key_store.read().await };
            let idle_generator =
                generator_helper::random_generator_selection(idle_generators).unwrap();

            if let Some(&cached_compute_value) = cached_compute.get(&idle_generator.address) {
                log::info!(
                    "Generator: {}, Compute available: {}, vs compute required: {}",
                    idle_generator.address,
                    cached_compute_value,
                    idle_generator.compute_required_per_request
                );
                if idle_generator.compute_required_per_request > cached_compute_value {
                    log::warn!(
                        "Possible insuff compute if ask: {} is assigned, hence skipping",
                        random_pending_ask.ask_id
                    );
                } else {
                    cached_compute.insert(
                        idle_generator.address,
                        cached_compute_value.sub(idle_generator.compute_required_per_request),
                    );
                }
            }

            if let Some(cached_stake_value) = cached_stake.get(&idle_generator.address).cloned() {
                let market_id = random_pending_ask.market_id;
                let stash_required = {
                    self.shared_market_store
                        .try_read()
                        .unwrap()
                        .get_market_by_market_id(&market_id)
                        .unwrap()
                        .slashing_penalty
                };

                log::info!(
                    "Generator: {}, Stash available: {}, vs stash required: {}",
                    idle_generator.address,
                    cached_stake_value.to_string(),
                    stash_required
                );

                if stash_required > cached_stake_value {
                    log::warn!(
                        "Possible insuff stash if ask: {} is assigned, hence skipping",
                        random_pending_ask.ask_id
                    );
                } else {
                    cached_stake.insert(
                        idle_generator.address,
                        cached_stake_value.sub(stash_required),
                    );
                }
            }

            let new_acl = if random_pending_ask.has_private_inputs {
                let acl_data = random_pending_ask.secret_acl.clone().unwrap();

                let mut cipher = secret_inputs_helpers::decrypt_ecies(
                    &self.matching_engine_key.to_vec(),
                    &acl_data,
                );

                // one of the key will surely dipher it, or else the ask would already have been flagged
                if cipher.is_err() {
                    for slave_key in &self.matching_engine_slave_keys {
                        cipher = secret_inputs_helpers::decrypt_ecies(slave_key, &acl_data);
                    }
                }

                let generator_ecies_pub_key = key_store
                    .get_by_address(&idle_generator.address, idle_generator.market_id.as_u64())
                    .unwrap()
                    .ecies_pub_key()
                    .clone()
                    .unwrap()
                    .to_vec();
                secret_inputs_helpers::encrypt_ecies(&generator_ecies_pub_key, cipher?.as_slice())?
            } else {
                Bytes::from_str("0x").unwrap().to_vec()
            };

            // state confirmation
            let ask_state = match self
                .proof_marketplace
                .get_ask_state(random_pending_ask.ask_id)
                .await
            {
                Ok(data) => data,
                Err(err) => {
                    log::error!("{}", err);
                    log::error!(
                        "Skipping ask {} because no status received from chain state",
                        random_pending_ask.ask_id
                    );
                    return Err("No ask status received from chain".into());
                }
            };

            let ask_state = get_ask_state(ask_state);
            log::info!("ask: {} -- {:?}", random_pending_ask.ask_id, ask_state);
            if ask_state != AskState::Create {
                log::warn!(
                    "ask {:?}. {:?}. skipping it",
                    random_pending_ask.ask_id,
                    ask_state
                );
                {
                    // previous ref of ask store won't work because it was readonly, create a write only one that drops here only.
                    self.shared_local_ask_store
                        .try_write()
                        .unwrap()
                        .modify_state(&random_pending_ask.ask_id, ask_state);
                }
                continue;
            }

            log::info!(
                "Assigned ask: {} to generator: {}, at {:?}",
                &random_pending_ask.ask_id,
                &idle_generator.address,
                // provider_http.get_block_number().await.unwrap()
                std::time::Instant::now()
            );
            task_list.push((random_pending_ask, idle_generator, new_acl));

            if task_list.len() >= self.max_tasks_size {
                break;
            }
        }

        if task_list.len().eq(&0) {
            log::warn!("No Matches");
            return Ok(end_block);
        }

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

        let signature = match matching_engine_signer
            .sign_message(ethers::types::H256(digest))
            .await
        {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}", err);
                return Err("Failed generating signature".into());
            }
        };
        log::debug!("Signature: {:?}", signature);
        log::debug!("Tx signed at {:?}", std::time::Instant::now());

        // todo!("create and broad cast tx");
        // // Assign batch task here
        let batch_relay_tx_pending = self.proof_marketplace.relay_batch_assign_tasks(
            ask_ids.clone(),
            generators.clone(),
            new_acls.clone(),
            ethers::types::Bytes::from_str(&signature.to_string()).unwrap(),
        );

        log::debug!("Tx created at {:?}", std::time::Instant::now());

        let batch_relay_tx = match batch_relay_tx_pending.send().await {
            Ok(data) => data.confirmations(10),
            Err(err) => {
                log::error!("{}", err);
                log::error!("failed sending the transaction");
                thread::sleep(Duration::from_secs(2));
                return Err("Failed creating matching".into());
            }
        };

        let batch_relay_tx = batch_relay_tx.await.unwrap().unwrap();

        log::info!(
            "Relayed {:?} requests tx: {:?}",
            ask_ids.clone().len(),
            batch_relay_tx.transaction_hash
        );

        if self.should_stop.load(Ordering::Acquire) {
            log::info!("Gracefully shutting down...");
            return Err("Stopped Match Making".into());
        }
        Ok(end_block)
    }

    #[cfg(not(feature = "disable_match_creation"))]
    async fn get_idle_generators(
        &self,
        random_pending_ask: LocalAsk,
        generator_store: &Arc<RwLock<GeneratorStore>>,
        market_store: &Arc<RwLock<MarketMetadataStore>>,
        key_store: &Arc<RwLock<KeyStore>>,
        task_reward: U256,
    ) -> Vec<generator_store::GeneratorInfoPerMarket> {
        // Ensure Generator implements Clone
        let generator_store = generator_store.read().await;
        let market_metadata_store = market_store.read().await;
        let key_store = key_store.read().await;
        let slashing_penalty = market_metadata_store
            .get_slashing_penalty_by_market_id(&random_pending_ask.market_id)
            .unwrap();
        let idle_generators = {
            let generator_query = {
                if random_pending_ask.has_private_inputs {
                    let generator_query = generator_store
                        .query_by_market_id(&random_pending_ask.market_id)
                        .filter_by_state(vec![GeneratorState::Joined, GeneratorState::Wip])
                        .filter_by_reward(task_reward);

                    let generator_with_idle_compute =
                        generator_store.filter_by_has_idle_compute(generator_query);

                    let generator_with_available_stake = generator_store
                        .filter_by_available_stake(generator_with_idle_compute, slashing_penalty);
                    generator_store.filter_by_has_private_inputs_support(
                        generator_with_available_stake,
                        key_store,
                    )
                } else {
                    let generator_query = generator_store
                        .query_by_market_id(&random_pending_ask.market_id)
                        .filter_by_state(vec![GeneratorState::Joined, GeneratorState::Wip])
                        .filter_by_reward(task_reward);

                    let generator_with_idle_compute =
                        generator_store.filter_by_has_idle_compute(generator_query);

                    generator_store
                        .filter_by_available_stake(generator_with_idle_compute, slashing_penalty)
                }
            };

            let generators = generator_query.result();

            generator_helper::select_idle_generators(generators)
        };
        idle_generators
    }
}
