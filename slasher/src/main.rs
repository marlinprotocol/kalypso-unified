use std::str::FromStr;
use std::{fs, sync::Arc};

use anyhow::Result;
use dotenv::dotenv;
use ethers::prelude::*;
use k256::ecdsa::SigningKey;
use matching_engine_helpers::ask_lib::ask_status::AskState;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

use kalypso_helper::try_read_contract_error_log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let slashing_config = "../matching_engine_config/matching_engine_config.json".to_string();
    let alt_slashing_config = "./matching_engine_config/matching_engine_config.json".to_string();
    let file_content = fs::read_to_string(&slashing_config)
        .or_else(|_| fs::read_to_string(&alt_slashing_config))?;

    let config: SlashingInstanceConfig = serde_json::from_str(&file_content)?;
    // slashing_config and matching_engine config are same, hence using same config...

    let indexer_url = std::env::var("INDEXER_URL").expect(&format!("{} is not set", "INDEXER_URL"));
    let slashing_instance = SlashingInstance::new(indexer_url.as_ref(), config);

    let _ = slashing_instance.run().await?;
    Ok(())
}

type ProofMarketplaceInstance = bindings::proof_marketplace::ProofMarketplace<
    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
>;
pub struct SlashingInstance {
    indexer_url: String,
    client: Client,
    #[allow(unused)]
    config: SlashingInstanceConfig,
    proof_marketplace: ProofMarketplaceInstance,
    #[allow(unused)]
    reward_address: Address,

    delay_in_ms: Duration,
    confirmations: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlashingInstanceConfig {
    pub proof_market_place: String,
    pub rpc_url: String,
    #[serde(alias = "relayer_private_key")]
    pub slasher_key: String,
    pub chain_id: String,
    pub delay_in_time_ms: Option<u64>,
    pub confirmations: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Operator {
    address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Market {
    market_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MarketQueryResult {
    result: Vec<Market>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OperatorQueryResult {
    result: Vec<Operator>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Request {
    ask_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SingleOperatorQueryResult {
    active_jobs_list: Vec<Request>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SingleMarketQueryResult {
    unmatched_jobs: Vec<Request>,
}

impl SlashingInstance {
    fn new(indexer_url: &str, config: SlashingInstanceConfig) -> Self {
        let slasher_key = config.clone().slasher_key;
        let slasher_key = slasher_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(U64::from_dec_str(&config.chain_id).unwrap().as_u64());

        let provider_http = Provider::<Http>::try_from(&config.rpc_url)
            .unwrap()
            .with_signer(slasher_key.clone());

        let client = Arc::new(provider_http.clone());

        // Creating contract instance for proof market place
        let proof_market_place_var = config.clone().proof_market_place;
        let proof_marketplace_address = Address::from_str(&proof_market_place_var).unwrap();

        let proof_marketplace = bindings::proof_marketplace::ProofMarketplace::new(
            proof_marketplace_address,
            client.clone(),
        );

        Self {
            indexer_url: indexer_url.into(),
            client: Client::new(),
            config: config.clone(),
            proof_marketplace,
            reward_address: slasher_key.address(),
            delay_in_ms: Duration::from_millis(config.delay_in_time_ms.unwrap_or(100)),
            confirmations: config.confirmations.unwrap_or(10),
        }
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let operators = match self.get_generators().await {
                Ok(data) => data,
                Err(err) => {
                    log::error!("{}", err);
                    vec![]
                }
            };

            log::debug!("Found {} operators", operators.len());

            for operator in operators.iter() {
                log::debug!(
                    "Searching active requests of operator: {:?}",
                    operator.address
                );
                sleep(self.delay_in_ms).await;
                let active_requests = self.active_requests(operator.address.clone()).await?;

                log::debug!(
                    "Found {} active requests for operator: {}",
                    active_requests.len(),
                    operator.address
                );
                for active_request in active_requests.iter() {
                    log::debug!(
                        "Trying to slash request: {} for operator {:?}",
                        active_request.ask_id,
                        operator.address
                    );
                    sleep(self.delay_in_ms).await;
                    self._handle_request(active_request).await;
                }
            }
            sleep(self.delay_in_ms).await;

            let markets = match self.get_markets().await {
                Ok(data) => data,
                Err(err) => {
                    log::error!("{}", err);
                    vec![]
                }
            };

            log::debug!("Found {} markets", markets.len());

            for market in markets.iter() {
                log::debug!(
                    "Searching expired requests for market: {}",
                    market.market_id
                );
                sleep(self.delay_in_ms).await;
                let expired_requests = self.expired_requests(market.market_id.clone()).await?;
                log::debug!(
                    "Found {} expired requests in market: {}",
                    expired_requests.len(),
                    market.market_id
                );
                for expired_request in expired_requests.iter() {
                    log::debug!(
                        "Trying to cancel expired request: {} in the market: {}",
                        expired_request.ask_id,
                        market.market_id
                    );
                    sleep(self.delay_in_ms).await;
                    self._handle_request(expired_request).await;
                }
            }

            sleep(self.delay_in_ms).await;
        }
    }

    async fn _handle_request(&self, request: &Request) {
        log::debug!("Found Active Request with ask_id: {}", request.ask_id);
        let ask_id = match U256::from_dec_str(&request.ask_id) {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}", err.to_string());
                log::error!("Invalid Ask ID received");
                return;
            }
        };
        let ask_state = match self.proof_marketplace.get_bid_state(ask_id).await {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}", err.to_string());
                log::error!("Failed Fetching Ask State");
                return;
            }
        };

        let ask_state = matching_engine_helpers::ask_lib::ask_status::get_ask_state(ask_state);

        if ask_state == AskState::DeadlineCrossed || ask_state == AskState::Assigned {
            log::warn!(
                "Request with ask_id: {}, State: {:?}",
                request.ask_id,
                ask_state
            );

            let mut slashing_transaction = self.proof_marketplace.refund_fees(vec![ask_id]);

            if cfg!(feature = "force_transactions") {
                slashing_transaction = slashing_transaction.gas(10_000_000);
            }

            let slashing_transaction =
                match slashing_transaction
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
                            bindings::error::ErrorErrors,
                            "OtherErrors"
                        );
                        format!("Failed to send transaction: {}", e)
                    }) {
                    Ok(data) => data.confirmations(self.confirmations),
                    Err(err) => {
                        log::error!("{}", err);
                        log::error!("failed sending the transaction");
                        return;
                    }
                };

            let slashing_transaction = match slashing_transaction.await {
                Ok(data) => data,
                Err(err) => {
                    log::error!("{}", err.to_string());
                    log::error!("Failed broadcasting transaction");
                    return;
                }
            };

            let slashing_transaction = match slashing_transaction {
                Some(data) => data,
                _ => {
                    log::warn!("Broadcasted transaction, but failed getting receipt");
                    return;
                }
            };

            log::info!(
                "Slashed Ask: {}, tx: {:?}",
                ask_id,
                slashing_transaction.transaction_hash
            );
        }

        if ask_state == AskState::UnAssigned {
            log::warn!(
                "Request with ask_id: {}, State: {:?}",
                request.ask_id,
                ask_state
            );

            let mut cancellation_transaction = self.proof_marketplace.cancel_bid(ask_id);

            if cfg!(feature = "force_transactions") {
                cancellation_transaction = cancellation_transaction.gas(10_000_000);
            }

            let cancellation_transaction =
                match cancellation_transaction
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
                            bindings::error::ErrorErrors,
                            "OtherErrors"
                        );
                        format!("Failed to send transaction: {}", e)
                    }) {
                    Ok(data) => data.confirmations(self.confirmations),
                    Err(err) => {
                        log::error!("{}", err);
                        log::error!("failed sending the transaction");
                        return;
                    }
                };

            let cancellation_transaction = match cancellation_transaction.await {
                Ok(data) => data,
                Err(err) => {
                    log::error!("{}", err.to_string());
                    log::error!("Failed broadcasting transaction");
                    return;
                }
            };

            let cancellation_transaction = match cancellation_transaction {
                Some(data) => data,
                _ => {
                    log::warn!("Broadcasted transaction, but failed getting receipt");
                    return;
                }
            };

            log::info!(
                "Cancelled Ask: {}, tx: {:?}",
                ask_id,
                cancellation_transaction.transaction_hash
            );
        }
    }

    async fn active_requests(&self, operator_address: String) -> Result<Vec<Request>> {
        let url = format!("{}/ui/generator/{}", self.indexer_url, operator_address);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let operator_query_result: SingleOperatorQueryResult = response.json().await?;
            return Ok(operator_query_result.active_jobs_list);
        } else {
            log::error!(
                "Failed to fetch generator/{}: HTTP {}",
                operator_address,
                response.status()
            );
            return Ok(vec![]);
        }
    }

    async fn get_generators(&self) -> Result<Vec<Operator>> {
        let url = format!("{}/ui/generators", self.indexer_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let operators: OperatorQueryResult = response.json().await?;
            return Ok(operators.result);
        } else {
            log::error!("Failed to fetch generators: HTTP {}", response.status());
            return Ok(vec![]);
        }
    }

    async fn get_markets(&self) -> Result<Vec<Market>> {
        let url = format!("{}/ui/markets", self.indexer_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let markets: MarketQueryResult = response.json().await?;
            return Ok(markets.result);
        } else {
            log::error!("Failed to fetch generators: HTTP {}", response.status());
            return Ok(vec![]);
        }
    }

    async fn expired_requests(&self, market_id: String) -> Result<Vec<Request>> {
        let url = format!("{}/ui/market/{}", self.indexer_url, market_id);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let market_query_result: SingleMarketQueryResult = response.json().await?;
            return Ok(market_query_result.unmatched_jobs);
        } else {
            log::error!(
                "Failed to fetch market/{}: HTTP {}",
                market_id,
                response.status()
            );
            return Ok(vec![]);
        }
    }
}
