use bindings::proof_marketplace as pmp;
use ethers::prelude::*;
use ethers::types::U256;
use ethers::{abi::Address, providers::Provider};
use kalypso_helper::custom_logger::CustomLogger;
use openssl::rand::rand_bytes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::proof_generator::GenerateProofParams;
use crate::{ask, generator_store, proof_generator};
use warp::Filter;

#[derive(Debug, Serialize, Deserialize)]
struct GeneratorConfigModel {
    address: String,
    ecies_private_key: Option<String>,
    data: Option<String>,
    supported_markets: Vec<String>,
    staked_amount: Option<U256>,
    min_reward: Option<U256>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    generator_config: Vec<GeneratorConfigModel>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct MarketDetails {
    pub port: Option<String>,
    pub ivs_url: Option<String>,
    pub prover_gateway_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RuntimeConfigModel {
    ws_url: Option<String>,
    http_url: String,
    private_key: String,
    proof_market_place: String,
    generator_registry: String,
    start_block: u64,
    chain_id: u64,
    markets: HashMap<String, MarketDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    runtime_config: RuntimeConfigModel,
}

pub struct JobCreator {
    config: Config,
    runtime_config: RuntimeConfig,
    #[allow(unused)]
    log_storage: Option<Arc<Mutex<Vec<String>>>>,
}

impl JobCreator {
    pub fn new(config: Config, runtime_config: RuntimeConfig, enable_logging_server: bool) -> Self {
        Self::initialize(config, runtime_config, enable_logging_server)
    }

    fn initialize(
        config: Config,
        runtime_config: RuntimeConfig,
        enable_logging_server: bool,
    ) -> Self {
        if enable_logging_server {
            let log_storage = Arc::new(Mutex::new(Vec::new()));
            let storage_clone = log_storage.clone();

            log::set_boxed_logger(Box::new(CustomLogger::new(storage_clone))).unwrap();
            log::set_max_level(log::LevelFilter::Info);

            tokio::spawn(Self::start_logging_server(log_storage.clone()));
            Self {
                config,
                runtime_config,
                log_storage: Some(log_storage),
            }
        } else {
            Self {
                config,
                runtime_config,
                log_storage: None,
            }
        }
    }

    fn start_logging_server(
        log_storage: Arc<Mutex<Vec<String>>>,
    ) -> impl std::future::Future<Output = ()> + Send {
        let log_route = warp::path::end().map(move || {
            let logs = log_storage.lock().unwrap();
            warp::reply::json(&*logs)
        });

        let server = warp::serve(log_route).run(([127, 0, 0, 1], 9999));

        Box::pin(server)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn simple_listener_for_non_confidential_prover(
        generator_address: String,
        supported_market_dec_string: String,
        http_rpc_url: String,
        gas_key: String,
        proof_market_place: String,
        generator_registry: String,
        start_block: u64,
        chain_id: u64,
        prover_gateway_url: String,
        ivs_url: String,
        enable_logging_server: bool,
    ) -> Self {
        let generator_config_models = vec![GeneratorConfigModel {
            address: generator_address,
            ecies_private_key: None,
            data: None,
            supported_markets: vec![supported_market_dec_string.clone()],
            staked_amount: None,
            min_reward: None,
        }];
        let config = Config {
            generator_config: generator_config_models,
        };

        let runtime_config_model = RuntimeConfigModel {
            ws_url: None,
            http_url: http_rpc_url,
            private_key: gas_key,
            proof_market_place,
            generator_registry,
            start_block,
            chain_id,
            markets: {
                let mut markets = HashMap::new();
                markets.insert(
                    supported_market_dec_string,
                    MarketDetails {
                        port: None,
                        ivs_url: Some(ivs_url),
                        prover_gateway_url: Some(prover_gateway_url),
                    },
                );
                markets
            },
        };

        let runtime_config = RuntimeConfig {
            runtime_config: runtime_config_model,
        };

        Self::initialize(config, runtime_config, enable_logging_server)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn simple_listener_for_confidential_prover(
        generator_address: String,
        ecies_private_key: String,
        supported_market_dec_string: String,
        http_rpc_url: String,
        gas_key: String,
        proof_market_place: String,
        generator_registry: String,
        start_block: u64,
        chain_id: u64,
        prover_port: String,
        enable_logging_server: bool,
    ) -> Self {
        let generator_config_models = vec![GeneratorConfigModel {
            address: generator_address,
            ecies_private_key: Some(ecies_private_key),
            data: None,
            supported_markets: vec![supported_market_dec_string.clone()],
            staked_amount: None,
            min_reward: None,
        }];
        let config = Config {
            generator_config: generator_config_models,
        };

        let runtime_config_model = RuntimeConfigModel {
            ws_url: None,
            http_url: http_rpc_url,
            private_key: gas_key,
            proof_market_place,
            generator_registry,
            start_block,
            chain_id,
            markets: {
                let mut markets = HashMap::new();
                markets.insert(
                    supported_market_dec_string,
                    MarketDetails {
                        port: Some(prover_port),
                        ivs_url: None,
                        prover_gateway_url: None,
                    },
                );
                markets
            },
        };

        let runtime_config = RuntimeConfig {
            runtime_config: runtime_config_model,
        };

        Self::initialize(config, runtime_config, enable_logging_server)
    }

    pub fn from_config_paths(
        generator_config_path: &str,
        runtime_config_path: &str,
        enable_logging_server: bool,
    ) -> anyhow::Result<Self> {
        let file_content = std::fs::read_to_string(generator_config_path)?;
        let config: Config = serde_json::from_str(&file_content)?;

        let file_content = std::fs::read_to_string(runtime_config_path)?;
        let runtime_config: RuntimeConfig = serde_json::from_str(&file_content)?;

        Ok(Self::initialize(
            config,
            runtime_config,
            enable_logging_server,
        ))
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let runtime_config = &self.runtime_config.runtime_config;

        let key = runtime_config.private_key.clone();
        let chain_id = runtime_config.chain_id;

        let http_url = runtime_config.http_url.clone();
        let proof_market_place_var = runtime_config.proof_market_place.clone();
        let markets = Arc::new(runtime_config.markets.clone());

        let signer = key.parse::<LocalWallet>().unwrap().with_chain_id(chain_id);
        let signer_address = signer.address();
        log::info!("Gas payers address : {:?}", signer.address());

        let provider_http = Provider::<Http>::connect(&http_url)
            .await
            .with_signer(signer.clone());

        let client_http = Arc::new(provider_http.clone());

        let wallet_nonce = provider_http
            .get_transaction_count(signer_address, None)
            .await?
            .as_u64();
        log::info!("Wallet nonce : {}", wallet_nonce);

        let proof_marketplace_address = Address::from_str(&proof_market_place_var)?;
        let proof_marketplace_http = Arc::new(pmp::ProofMarketplace::new(
            proof_marketplace_address,
            Arc::clone(&client_http),
        ));

        let submitter_pmp = Arc::new(tokio::sync::Mutex::new(pmp::ProofMarketplace::new(
            proof_marketplace_address,
            Arc::clone(&client_http),
        )));

        let mut key_store = generator_store::GeneratorStore::new();
        for config in &self.config.generator_config {
            let generator_address = match Address::from_str(&config.address) {
                Ok(value) => value,
                Err(_) => {
                    log::error!("Invalid Address for generator provided");
                    continue;
                }
            };

            let existing_generator_data = key_store.get_generator(&generator_address);
            if existing_generator_data.is_some() {
                return Err(anyhow::anyhow!(
                    "Generator Address mentioned twice in the network"
                ));
            }

            let (ecies_secret, ecies_public) = match &config.ecies_private_key {
                Some(private_key) => {
                    let mut original_message = vec![0; 32]; // for example, 32 bytes
                    rand_bytes(&mut original_message).expect("Failed to generate random bytes");

                    let private_key = hex::decode(private_key).unwrap();
                    let ecies_secret_key = ecies::SecretKey::parse_slice(&private_key).unwrap();
                    let ecies_public_key = ecies::PublicKey::from_secret_key(&ecies_secret_key);

                    let public_key =
                        ecies::PublicKey::from_secret_key(&ecies_secret_key).serialize();
                    let encrypted_message = match ecies::encrypt(&public_key, &original_message) {
                        Ok(value) => value,
                        Err(_) => {
                            log::error!("Unable to encrypt message using public key");
                            continue;
                        }
                    };

                    let decrypted_message = match ecies::decrypt(&private_key, &encrypted_message) {
                        Ok(value) => value,
                        Err(_) => {
                            log::error!("Unable to decrypt message using private key");
                            continue;
                        }
                    };

                    if original_message != decrypted_message {
                        log::error!("The public and private keys do not match!");
                        continue;
                    }

                    (Some(ecies_secret_key), Some(ecies_public_key))
                }
                _ => (None, None),
            };

            let mut supported_markets: Vec<U256> = vec![];

            for market in config.supported_markets.clone().into_iter() {
                let market_temp = U256::from_dec_str(&market).unwrap();
                supported_markets.push(market_temp);
            }

            let generator = generator_store::Generator {
                address: generator_address,
                supported_market_ids: supported_markets,
                ecies_priv_key: ecies_secret,
                ecies_pub_key: ecies_public,
            };
            key_store.add_generator(generator);
        }

        log::info!("Total number of generators {:?}", key_store.count());

        let block_to_use = client_http
            .provider()
            .get_block_number()
            .await
            .unwrap_or(4180050.into());
        let runtime_start_block =
            U64::from_dec_str(&runtime_config.start_block.to_string()).unwrap_or(block_to_use);
        let mut start_block = runtime_start_block;

        let blocks_at_once = 10000;

        let should_stop = Arc::new(AtomicBool::new(false));
        let stop_handle = should_stop.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            stop_handle.store(true, Ordering::Release);
        });

        let thread_count = Arc::new(AtomicUsize::new(0));
        let max_thread_count = 20;

        loop {
            if should_stop.load(Ordering::Acquire) {
                log::info!("Gracefully shutting down...");
                break;
            }

            if thread_count.load(Ordering::SeqCst) >= max_thread_count {
                thread::sleep(Duration::from_millis(60));
                log::warn!(
                    "Stopped proof generation as {} proof generations in progress",
                    max_thread_count
                );
                continue;
            }

            let latest_block = match provider_http.get_block_number().await {
                Ok(data) => data,
                Err(err) => {
                    log::error!("{}", err);
                    thread::sleep(Duration::from_millis(1000));
                    continue;
                }
            };

            let end = if start_block + blocks_at_once > latest_block {
                thread::sleep(Duration::from_millis(2000)); // to reduce calls on eth_latestBlock
                latest_block - 1
            } else {
                start_block + blocks_at_once - 1
            };

            if start_block > end {
                thread::sleep(Duration::from_millis(2000));
                continue;
            }

            log::info!(
                "Searching for TASKs from Block {} to {}...",
                start_block,
                end
            );

            let filter = proof_marketplace_http
                .task_created_filter()
                .filter
                .from_block(start_block)
                .to_block(end);

            let logs = match client_http.provider().get_logs(&filter).await {
                Ok(data) => data,
                Err(_) => {
                    log::error!("Sleeping the thread for logs to avoid rate limit");
                    thread::sleep(Duration::from_millis(2000));
                    continue;
                }
            };

            for log in logs {
                let event = proof_marketplace_http.decode_event::<pmp::TaskCreatedFilter>(
                    "TaskCreated",
                    log.topics,
                    log.data,
                )?;
                let generator = match key_store.get_generator(&event.generator) {
                    Some(gen) => {
                        let ask_details: &(pmp::Ask, u8, H160, H160) =
                            &proof_marketplace_http.list_of_ask(event.ask_id).await?;

                        log::debug!("Generator Data (on polling): {:?}", &gen);
                        if gen.supported_market_ids.contains(&ask_details.0.market_id) {
                            gen
                        } else {
                            log::debug!("Skipping ask: {:?}, because Generator: {:?} doesn't support Market: {:?}", event.ask_id, gen.address, ask_details.0.market_id);
                            continue;
                        }
                    }
                    None => {
                        log::debug!(
                            "Skipping ask: {:?}, because it is not assigned to my generators",
                            event.ask_id
                        );
                        continue;
                    }
                };

                let ask_state = &proof_marketplace_http.get_ask_state(event.ask_id).await?;
                let ask_state = ask::get_ask_state(*ask_state);
                log::info!("Ask: {} state: {:?}", event.ask_id, ask_state);
                if ask_state == ask::AskState::Assigned {
                    log::info!(
                        "Need to generate proof (polling) for ASK ID : {}",
                        event.ask_id
                    );
                    let gen_ecies_private_key = generator.ecies_priv_key;

                    let proof_market_place_clone_http = Arc::clone(&proof_marketplace_http);
                    let submitter_pmp_clone_http = Arc::clone(&submitter_pmp);
                    let markets_clone = Arc::clone(&markets);
                    // code inside thread starts here

                    thread_count.fetch_add(1, Ordering::SeqCst);
                    let thread_count_clone = Arc::clone(&thread_count);

                    tokio::spawn(async move {
                        log::warn!("Spin up new thread from proof generation calls");
                        let generate_proof_args = GenerateProofParams {
                            ask_id: event.ask_id,
                            new_acl: event.new_acl,
                            proof_market_place_contract_http: proof_market_place_clone_http,
                            ecies_private_key: &gen_ecies_private_key,
                            start_block: &runtime_start_block,
                            end_block: &latest_block,
                            markets: &markets_clone,
                        };

                        let proof = match proof_generator::generate_proof(generate_proof_args).await
                        {
                            Ok(proof) => proof,
                            Err(err) => return log::error!("{}", err),
                        };

                        log::info!("{:?}", &proof);

                        let proof_transaction = match proof {
                            crate::proof_generator::prover::Proof::ValidProof(proof) => {
                                log::info!("Submitting proof on-chain...");
                                submitter_pmp_clone_http
                                    .lock()
                                    .await
                                    .submit_proof(event.ask_id, proof)
                                    .send()
                                    .await
                                    .unwrap()
                                    .await
                                    .unwrap()
                            }
                            crate::proof_generator::prover::Proof::InvalidProof(
                                invalid_proof_signature,
                            ) => {
                                log::info!("Submitting signature on-chain...");
                                submitter_pmp_clone_http
                                    .lock()
                                    .await
                                    .submit_proof_for_invalid_inputs(
                                        event.ask_id,
                                        invalid_proof_signature,
                                    )
                                    .send()
                                    .await
                                    .unwrap()
                                    .await
                                    .unwrap()
                            }
                        };

                        match proof_transaction {
                            Some(tx_data) => {
                                log::info!(
                                    "Submitted proof for OLD ask with id : {} via transaction {:?}",
                                    event.ask_id,
                                    tx_data.transaction_hash
                                );
                            }
                            None => {
                                log::error!(
                                    "Error in submitting proof for ASK ID : {}",
                                    event.ask_id
                                );
                            }
                        }

                        thread_count_clone.fetch_sub(1, Ordering::SeqCst);
                        // code inside thread ends here
                    });
                }
            }

            start_block = end + 1;
        }
        Ok(())
    }
}
