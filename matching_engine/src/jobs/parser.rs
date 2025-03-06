use crate::ask_lib::ask::LocalAsk;
use crate::ask_lib::ask_store::{
    AskManagementRead, AskManagementWrite, CompletedProofsManagement, MarketRequestCounters,
    ProofCounters, ProofMarketStakeLockManagement, RequestorCounters, TimingOperations,
};
use crate::costs::CostStoreOperations;
use crate::generator_lib::generator_state::GeneratorState;
use crate::generator_lib::native_stake_store::NativeStakingOperations;
use crate::generator_lib::stake_manager_store::StakeManagerOperations;
use crate::generator_lib::traits::{
    GeneratorAdditionalQuery, GeneratorAvailability, GeneratorEarningsAndSlashing, GeneratorFilter,
    GeneratorKeyStoreFilterInterfaceTrait, GeneratorLockManagement, GeneratorMarketManagement,
    GeneratorMetadata, GeneratorQuery, GeneratorRegistration, GeneratorSlashingManagement,
    GeneratorStakeComputeManagement, JobMissedCounter, WithdrawalManagement,
};
use crate::generator_lib::{generator_helper, generator_store};

use crate::generator_lib::symbiotic_stake_store::{
    OperatorStakeManagement, SlashResultManagement, TokenLockManagement, VaultSnapshotManagement,
};
use crate::latest_block_store::LatestBlockStoreTrait;
use crate::market_metadata::{MarketMetadataStoreRead, MarketMetadataStoreWrite};
use anyhow::Result;
use ethers::prelude::*;
use k256::ecdsa::SigningKey;
use std::collections::HashMap;

use crate::generator_lib::key_store::KeyStoreOperations;
use crate::log_processor;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::RwLock;

type EntityRegistryInstance = bindings::entity_key_registry::EntityKeyRegistry<
    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
>;

type ProofMarketplaceInstance = bindings::proof_marketplace::ProofMarketplace<
    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
>;

type GeneratorRegistryInstance =
    bindings::prover_manager::ProverManager<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

type SymbioticStakingInstance = bindings::symbiotic_staking::SymbioticStaking<
    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
>;

type NativeStakingInstance =
    bindings::native_staking::NativeStaking<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

type StakingManagerInstance =
    bindings::staking_manager::StakingManager<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

pub struct LogParser<
    AS: AskManagementRead
        + AskManagementWrite
        + RequestorCounters
        + ProofCounters
        + MarketRequestCounters
        + CompletedProofsManagement
        + TimingOperations
        + ProofMarketStakeLockManagement,
    GS: GeneratorRegistration
        + GeneratorStakeComputeManagement
        + GeneratorMarketManagement
        + GeneratorSlashingManagement
        + GeneratorLockManagement
        + GeneratorAvailability
        + GeneratorMetadata
        + GeneratorQuery
        + GeneratorFilter
        + GeneratorKeyStoreFilterInterfaceTrait<KS>
        + GeneratorEarningsAndSlashing
        + WithdrawalManagement
        + JobMissedCounter
        + GeneratorAdditionalQuery,
    MS: MarketMetadataStoreRead + MarketMetadataStoreWrite,
    KS: KeyStoreOperations,
    CS: CostStoreOperations,
    SS: OperatorStakeManagement + TokenLockManagement + VaultSnapshotManagement + SlashResultManagement,
    NS: NativeStakingOperations,
    SM: StakeManagerOperations,
    BS: LatestBlockStoreTrait,
> {
    should_stop: Arc<AtomicBool>,
    start_block: Arc<RwLock<BS>>,
    block_range: U64,
    confirmations: U64,
    proof_marketplace: ProofMarketplaceInstance,
    generator_registry: GeneratorRegistryInstance,
    entity_registry: EntityRegistryInstance,
    symbiotic_staking: SymbioticStakingInstance,
    native_staking: NativeStakingInstance,
    staking_manager: StakingManagerInstance,
    provider_http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    matching_engine_key: Vec<u8>,
    matching_engine_slave_keys: Vec<Vec<u8>>,
    shared_local_ask_store: Arc<RwLock<AS>>,
    shared_generator_store: Arc<RwLock<GS>>,
    shared_market_store: Arc<RwLock<MS>>,
    shared_key_store: Arc<RwLock<KS>>,
    shared_cost_store: Arc<RwLock<CS>>,
    shared_symbiotic_stake_store: Arc<RwLock<SS>>,
    shared_native_stake_store: Arc<RwLock<NS>>,
    shared_stake_manager_store: Arc<RwLock<SM>>,
    #[allow(unused)]
    chain_id: String,
    #[allow(unused)]
    max_tasks_size: usize,
    rpc_url: String,
    unhandled_logs: Arc<RwLock<Vec<Log>>>,
    #[allow(unused)]
    matching_errors: Arc<RwLock<Vec<String>>>,
    backup_in_progress: Arc<RwLock<bool>>,
}

impl<
        AS: AskManagementRead
            + AskManagementWrite
            + RequestorCounters
            + ProofCounters
            + MarketRequestCounters
            + CompletedProofsManagement
            + TimingOperations
            + ProofMarketStakeLockManagement,
        GS: GeneratorRegistration
            + GeneratorStakeComputeManagement
            + GeneratorMarketManagement
            + GeneratorSlashingManagement
            + GeneratorLockManagement
            + GeneratorAvailability
            + GeneratorMetadata
            + GeneratorQuery
            + GeneratorFilter
            + GeneratorKeyStoreFilterInterfaceTrait<KS>
            + GeneratorEarningsAndSlashing
            + WithdrawalManagement
            + JobMissedCounter
            + GeneratorAdditionalQuery,
        MS: MarketMetadataStoreRead + MarketMetadataStoreWrite,
        KS: KeyStoreOperations,
        CS: CostStoreOperations,
        SS: OperatorStakeManagement
            + TokenLockManagement
            + VaultSnapshotManagement
            + SlashResultManagement,
        NS: NativeStakingOperations,
        SM: StakeManagerOperations,
        BS: LatestBlockStoreTrait,
    > LogParser<AS, GS, MS, KS, CS, SS, NS, SM, BS>
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        should_stop: Arc<AtomicBool>,
        rpc_url: String,
        relayer_signer: Wallet<SigningKey>,
        start_block: Arc<RwLock<BS>>,
        block_range: U64,
        confirmations: U64,
        proof_marketplace: ProofMarketplaceInstance,
        generator_registry: GeneratorRegistryInstance,
        entity_registry: EntityRegistryInstance,
        symbiotic_staking: SymbioticStakingInstance,
        native_staking: NativeStakingInstance,
        staking_manager: StakingManagerInstance,
        matching_engine_key: String,
        matching_engine_slave_keys: Vec<String>,
        shared_local_ask_store: Arc<RwLock<AS>>,
        shared_generator_store: Arc<RwLock<GS>>,
        shared_market_store: Arc<RwLock<MS>>,
        shared_key_store: Arc<RwLock<KS>>,
        shared_cost_store: Arc<RwLock<CS>>,
        shared_symbiotic_stake_store: Arc<RwLock<SS>>,
        shared_native_stake_store: Arc<RwLock<NS>>,
        shared_stake_manager_store: Arc<RwLock<SM>>,
        chain_id: String,
        unhandled_logs: Arc<RwLock<Vec<Log>>>,
        matching_errors: Arc<RwLock<Vec<String>>>,
        backup_in_progress: Arc<RwLock<bool>>,
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
            symbiotic_staking,
            native_staking,
            staking_manager,
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
            shared_symbiotic_stake_store,
            shared_native_stake_store,
            shared_stake_manager_store,
            chain_id,
            max_tasks_size: 10, // TODO: dynamically adjust latter
            rpc_url,
            unhandled_logs,
            matching_errors,
            backup_in_progress,
        }
    }

    pub async fn parse(&self) -> anyhow::Result<()> {
        let mut matches_upto: Option<U64> = None;

        loop {
            if self.should_stop.load(Ordering::Acquire) {
                log::info!("Gracefully shutting down...");
                break;
            }

            if let Ok(back_going_on) = self.backup_in_progress.try_read() {
                if *back_going_on {
                    log::debug!("Backup in progress, sleeping parser");
                    // manually dropping it to avoid continue write starvation during backups
                    drop(back_going_on);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            }
            // if not read lock, let parser run as it is

            let (mut start_block, end_block) = match self
                .get_start_end_block()
                .await
                .map_err(|e| format!("Failed to get start and end block: {}", e.to_string()))
            {
                Ok(data) => data,
                Err(e) => {
                    log::warn!("Could fetch start_block and end_block, pausing ME. {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            if let Some(_matches_upto) = matches_upto.filter(|&m| m == end_block) {
                log::warn!(
                    "All matches made up to {}. Waiting for a few seconds",
                    _matches_upto
                );
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }

            if start_block + self.confirmations <= end_block {
                log::info!(
                    "Processing blocks from {:?} to {:?}. ==> Range: {:?}",
                    start_block,
                    end_block,
                    end_block - start_block
                );

                let proof_marketplace_address = self.proof_marketplace.address();
                let generator_registry_address = self.generator_registry.address();
                let entity_key_registry_address = self.entity_registry.address();
                let native_staking_address = self.native_staking.address();
                let symbiotic_staking_address = self.symbiotic_staking.address();
                let staking_manager_address = self.staking_manager.address();

                let filter = Filter::default()
                    .from_block(start_block)
                    .to_block(end_block)
                    .address(vec![
                        proof_marketplace_address,
                        generator_registry_address,
                        entity_key_registry_address,
                        native_staking_address,
                        symbiotic_staking_address,
                        staking_manager_address,
                    ]);

                let logs = match self.provider_http.get_logs(&filter).await {
                    Ok(data) => data,
                    Err(err) => {
                        log::error!("Error fetching logs, sleeping the thread to avoid rate limit");
                        log::error!("{}", err);
                        tokio::time::sleep(Duration::from_secs(5)).await;
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
                                    log,
                                    &self.proof_marketplace,
                                    &self.shared_local_ask_store,
                                    &self.shared_generator_store,
                                    &self.shared_market_store,
                                    &self.shared_cost_store,
                                    &self.shared_native_stake_store,
                                    &self.shared_symbiotic_stake_store,
                                    &self.matching_engine_key,
                                    &self.matching_engine_slave_keys,
                                    &self.rpc_url,
                                    &self.unhandled_logs,
                                )
                                .await
                                .unwrap();

                                continue;
                            }
                            if log.address.eq(&generator_registry_address) {
                                log_processor::gr::process_generator_registry_logs(
                                    log,
                                    &self.generator_registry,
                                    &self.shared_generator_store,
                                    &self.rpc_url,
                                    &self.unhandled_logs,
                                )
                                .await
                                .unwrap();

                                continue;
                            }

                            if log.address.eq(&entity_key_registry_address) {
                                log_processor::er::process_entity_key_registry_logs(
                                    log,
                                    &self.entity_registry,
                                    &self.shared_key_store,
                                    &self.unhandled_logs,
                                )
                                .await
                                .unwrap();
                                continue;
                            }

                            if log.address.eq(&native_staking_address) {
                                log_processor::ns::process_native_staking_logs(
                                    log,
                                    &self.native_staking,
                                    &self.shared_generator_store,
                                    &self.shared_native_stake_store,
                                    &self.shared_local_ask_store,
                                    &self.rpc_url,
                                    &self.unhandled_logs,
                                )
                                .await
                                .unwrap();
                                continue;
                            }

                            if log.address.eq(&symbiotic_staking_address) {
                                log_processor::ss::process_symbiotic_staking_logs(
                                    log,
                                    &self.symbiotic_staking,
                                    &self.shared_generator_store,
                                    &self.shared_symbiotic_stake_store,
                                    &self.shared_local_ask_store,
                                    &self.rpc_url,
                                    &self.unhandled_logs,
                                )
                                .await
                                .unwrap();
                                continue;
                            }

                            if log.address.eq(&staking_manager_address) {
                                log_processor::sm::process_staking_manager_log(
                                    log,
                                    &self.staking_manager,
                                    &self.shared_stake_manager_store,
                                    native_staking_address,
                                    symbiotic_staking_address,
                                    &self.unhandled_logs,
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
                self.start_block.write().await.set_latest_block(start_block);
                continue;
            }

            matches_upto = match self
                .create_match(end_block)
                .await
                .map_err(|e| format!("Failed to create match: {}", e.to_string()))
            {
                Ok(upto) => {
                    log::info!("Completed match assignment upto: {}", upto);

                    Some(upto)
                }
                Err(err) => {
                    log::error!("{}", err);
                    log::error!("Match Creation Failed, retyring in couple of seconds");
                    tokio::time::sleep(Duration::from_secs(4)).await;
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
                tokio::time::sleep(Duration::from_secs(5)).await;
                return Err("Failed fetching latest block number".into());
            }
        };
        let start_block = self.start_block.read().await.get_latest_block();
        let end_block = if start_block + self.block_range > latest_block {
            latest_block - 1
        } else {
            start_block + self.block_range - 1
        };

        Ok((start_block, end_block))
    }

    async fn create_match(&self, end_block: U64) -> Result<U64, Box<dyn std::error::Error>> {
        use crate::ask_lib::ask_status::get_ask_state;
        use crate::ask_lib::ask_status::AskState;
        use crate::utility::TokenTracker;
        use kalypso_helper::secret_inputs_helpers;
        use std::ops::Sub;
        use std::str::FromStr;

        log::debug!("processed till {:?}. Waiting for new blocks", end_block);
        let ask_store = { self.shared_local_ask_store.read().await };
        let generator_store = { self.shared_generator_store.read().await };

        log::debug!("Trying to fetch available asks");
        let available_asks = ask_store
            .get_by_ask_state_except_complete(AskState::Create)
            .filter_by_flag(true)
            .result();

        drop(ask_store); // dropping here manually because, in next step may require to obtain a write lock
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
                let available_stake = {
                    let available_native_stake = generator_store
                        .get_available_native_stake(&_generator)
                        .unwrap_or_default();
                    let available_symbiotic_stake = generator_store
                        .get_available_symbiotic_stake(&_generator)
                        .unwrap_or_default();

                    let mut available_stake = TokenTracker::new();

                    if self
                        .shared_stake_manager_store
                        .read()
                        .await
                        .exists(&self.native_staking.address())
                    {
                        available_stake += available_native_stake;
                    }

                    if self
                        .shared_stake_manager_store
                        .read()
                        .await
                        .exists(&self.symbiotic_staking.address())
                    {
                        available_stake += available_symbiotic_stake;
                    }
                    available_stake
                };
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
            log::debug!(
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
                    &self.shared_native_stake_store,
                    &self.shared_symbiotic_stake_store,
                )
                .await;

            if idle_generators.is_empty() {
                log::warn!(
                    "Can't find idle-generators for ask {:?}, market_id: {:?}",
                    random_pending_ask.ask_id,
                    random_pending_ask.market_id
                );
                continue;
            }

            log::debug!("idle generators: {}", &idle_generators.len());

            let key_store = { self.shared_key_store.read().await };

            #[cfg(feature = "use_time_and_price_weighted_matching")]
            let idle_generator = {
                let generator_store = { self.shared_generator_store.read().await };
                let mut missed_jobs = HashMap::new();
                for idle_generator in idle_generators.iter() {
                    let missed =
                        generator_store.get_job_missed_count_in_window(&idle_generator.address);
                    missed_jobs.insert(idle_generator.address, missed);
                }
                generator_helper::weighted_time_cost_random_selection(idle_generators, missed_jobs)
                    .unwrap()
            };

            #[cfg(not(feature = "use_time_and_price_weighted_matching"))]
            let idle_generator =
                generator_helper::random_generator_selection(idle_generators).unwrap();

            if let Some(&cached_compute_value) = cached_compute.get(&idle_generator.address) {
                log::debug!(
                    "Generator: {}, Compute available: {}, vs compute required: {}",
                    idle_generator.address,
                    cached_compute_value,
                    idle_generator.compute_required_per_request
                );
                if idle_generator.compute_required_per_request > cached_compute_value {
                    log::warn!(
                        "Possible insuff compute if ask: {} is assigned, so skipping it",
                        random_pending_ask.ask_id
                    );
                    log::warn!(
                        "Will try assigning ask: {} in next iteration to any generator",
                        random_pending_ask.ask_id
                    );
                    continue;
                } else {
                    cached_compute.insert(
                        idle_generator.address,
                        cached_compute_value.sub(idle_generator.compute_required_per_request),
                    );
                }
            }

            if let Some(cached_stake_value) = cached_stake.get(&idle_generator.address) {
                let stash_required = {
                    self.shared_native_stake_store
                        .read()
                        .await
                        .tokens_to_lock()
                        .await
                        .clone()
                        + self
                            .shared_symbiotic_stake_store
                            .read()
                            .await
                            .tokens_to_lock()
                            .clone()
                };

                log::debug!(
                    "Generator: {}, Stash available: {}, vs stash required: {}",
                    idle_generator.address,
                    cached_stake_value.to_string(),
                    stash_required
                );

                let address_token_pairs = stash_required.to_address_token_pair();

                if cached_stake_value.has_more_than_or_eq_in_at_least_one(&address_token_pairs) {
                    if let Some(selected_token) =
                        cached_stake_value.select_one_random_stakable_pair(&address_token_pairs)
                    {
                        let updated_cached_stake = cached_stake_value
                            .clone()
                            .sub(TokenTracker::from_address_token_pair(selected_token));

                        log::debug!(
                            "Updated cached stake of generator: {} = {}",
                            idle_generator.address,
                            updated_cached_stake
                        );
                        cached_stake.insert(idle_generator.address, updated_cached_stake);
                    }
                } else {
                    log::warn!(
                        "Possible insuff stash if ask: {} is assigned, so skipping it",
                        random_pending_ask.ask_id
                    );
                    log::warn!(
                        "Will try assigning ask: {} in next iteration to any generator",
                        random_pending_ask.ask_id
                    );
                    continue;
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
                .get_bid_state(random_pending_ask.ask_id)
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
                    let mut ask_store = self.shared_local_ask_store.write().await;
                    ask_store.modify_state(&random_pending_ask.ask_id, ask_state);
                    drop(ask_store);
                }
                continue;
            }

            log::info!(
                "Assigned ask: {} to generator: {:?}, at {:#?}",
                &random_pending_ask.ask_id,
                &idle_generator.address,
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
        } else {
            log::info!("Trying to assign {} asks", task_list.len());
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

        #[cfg(not(feature = "disable_match_creation"))]
        {
            use kalypso_helper::try_read_contract_error_log;
            let mut batch_relay_tx_pending = self.proof_marketplace.relay_batch_assign_tasks(
                ask_ids.clone(),
                generators.clone(),
                new_acls.clone(),
                ethers::types::Bytes::from_str(&signature.to_string()).unwrap(),
            );

            log::debug!("Tx created at {:?}", std::time::Instant::now());

            if cfg!(feature = "force_transactions") {
                batch_relay_tx_pending = batch_relay_tx_pending.gas(10_000_000);
            }

            let batch_relay_tx =
                match batch_relay_tx_pending
                    .send()
                    .await
                    .map_err(|e: ContractError<_>| {
                        log::error!("========================\n");
                        try_read_contract_error_log!(
                            e,
                            bindings::proof_marketplace::ProofMarketplaceErrors,
                            "ProofMarketplace"
                        );

                        try_read_contract_error_log!(
                            e,
                            bindings::entity_key_registry::EntityKeyRegistryErrors,
                            "EntityKeyRegistry"
                        );

                        try_read_contract_error_log!(
                            e,
                            bindings::native_staking::NativeStakingErrors,
                            "NativeStaking"
                        );

                        try_read_contract_error_log!(
                            e,
                            bindings::symbiotic_staking::SymbioticStakingErrors,
                            "SymbioticStaking"
                        );

                        try_read_contract_error_log!(
                            e,
                            bindings::staking_manager::StakingManagerErrors,
                            "StakeManager"
                        );

                        try_read_contract_error_log!(
                            e,
                            bindings::error::ErrorErrors,
                            "OtherErrors"
                        );
                        format!("Failed to send transaction: {}", e)
                    }) {
                    Ok(data) => data.confirmations(10),
                    Err(err) => {
                        log::error!("{}", err);
                        log::error!("failed sending the transaction");
                        if let Ok(mut errors) = self.matching_errors.try_write() {
                            errors.push(err.to_string());
                        } else {
                            log::warn!(
                                "Could not acquire lock on matching_errors to record the error."
                            );
                        }

                        tokio::time::sleep(Duration::from_secs(2)).await;
                        return Err("Failed creating matching".into());
                    }
                };

            let batch_relay_tx = batch_relay_tx.await.unwrap().unwrap();

            log::info!(
                "Relayed {:?} requests tx: {:?}",
                ask_ids.clone().len(),
                batch_relay_tx.transaction_hash
            );
        }

        if self.should_stop.load(Ordering::Acquire) {
            log::info!("Gracefully shutting down...");
            return Err("Stopped Match Making".into());
        }
        Ok(end_block)
    }

    async fn get_idle_generators(
        &self,
        random_pending_ask: LocalAsk,
        generator_store: &Arc<RwLock<GS>>,
        _: &Arc<RwLock<MS>>,
        key_store: &Arc<RwLock<KS>>,
        task_reward: U256,
        native_staking_store: &Arc<RwLock<NS>>,
        symbiotic_staking_store: &Arc<RwLock<SS>>,
    ) -> Vec<generator_store::GeneratorInfoPerMarket> {
        // Ensure Generator implements Clone

        let generator_store = generator_store.read().await;
        let key_store = key_store.read().await;
        let native_staking_store = native_staking_store.read().await;
        let symbiotic_staking_store = symbiotic_staking_store.read().await;

        let native_stake_requirements = native_staking_store
            .tokens_to_lock()
            .await
            .to_address_token_pair();
        let symbiotic_stake_requirements = symbiotic_staking_store
            .tokens_to_lock()
            .to_address_token_pair();

        log::debug!(
            "Required Native Stake for Job: {}",
            random_pending_ask.ask_id
        );
        log::debug!("Job {} Reward: {}", random_pending_ask.ask_id, task_reward);

        let idle_generators = {
            let generator_query = {
                if random_pending_ask.has_private_inputs {
                    let generator_query = generator_store
                        .query_by_market_id_and_only_active(&random_pending_ask.market_id)
                        .filter_by_state(vec![GeneratorState::Joined, GeneratorState::Wip])
                        .filter_by_reward(task_reward)
                        .filter_by_time(random_pending_ask.time_requested_for_proof_generation);

                    let generator_with_idle_compute =
                        generator_store.filter_by_has_idle_compute(generator_query);

                    let generator_with_available_native_stake = generator_store
                        .filter_by_available_native_stake(
                            generator_with_idle_compute,
                            native_stake_requirements,
                        );

                    let generator_with_available_stake = generator_store
                        .filter_by_available_symbiotic_stake(
                            generator_with_available_native_stake,
                            symbiotic_stake_requirements,
                        );

                    generator_store.filter_by_has_private_inputs_support(
                        generator_with_available_stake,
                        key_store,
                    )
                } else {
                    let generator_query = generator_store
                        .query_by_market_id_and_only_active(&random_pending_ask.market_id)
                        .filter_by_state(vec![GeneratorState::Joined, GeneratorState::Wip])
                        .filter_by_reward(task_reward)
                        .filter_by_time(random_pending_ask.time_requested_for_proof_generation);

                    let generator_with_idle_compute =
                        generator_store.filter_by_has_idle_compute(generator_query);

                    let generator_with_available_native_stake = generator_store
                        .filter_by_available_native_stake(
                            generator_with_idle_compute,
                            native_stake_requirements,
                        );

                    generator_store.filter_by_available_symbiotic_stake(
                        generator_with_available_native_stake,
                        symbiotic_stake_requirements,
                    )
                }
            };

            let generators = generator_query.result();

            log::debug!(
                "Final Number of elligible generators found: {}",
                generators.len()
            );
            generator_helper::select_idle_generators(generators)
        };
        idle_generators
    }
}
