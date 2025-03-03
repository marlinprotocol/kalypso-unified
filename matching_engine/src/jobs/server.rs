use std::sync::Arc;
use std::time::Duration;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use ethers::types::{Log, U64};
use kalypso_helper::middlewares::request_limiter::ConcurrencyLimiter;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;

use crate::ask_lib::ask_store::{
    AskManagementRead, CompletedProofsManagement, MarketRequestCounters, ProofCounters,
    RequestorCounters, TimingOperations,
};

use crate::costs::CostStoreOperations;
use crate::generator_lib::key_store::KeyStoreOperations;
use crate::generator_lib::native_stake_store::NativeStakingOperations;
use crate::generator_lib::stake_manager_store::StakeManagerOperations;
use crate::generator_lib::symbiotic_stake_store::{
    OperatorStakeManagement, SlashResultManagement, TokenLockManagement, VaultSnapshotManagement,
};
use crate::generator_lib::traits::{
    GeneratorAdditionalQuery, GeneratorAvailability, GeneratorEarningsAndSlashing,
    GeneratorRegistration, JobMissedCounter, WithdrawalManagement,
};
use crate::market_metadata::MarketMetadataStoreRead;
use crate::routes::{get_core_scope, get_stats_scope, get_swagger, ui_scope};

type EntityRegistryInstance = Arc<
    RwLock<
        bindings::entity_key_registry::EntityKeyRegistry<
            SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        >,
    >,
>;

pub struct MatchingEngineServer<
    MS: MarketMetadataStoreRead,
    AS: AskManagementRead
        + CompletedProofsManagement
        + ProofCounters
        + RequestorCounters
        + TimingOperations
        + MarketRequestCounters,
    GS: GeneratorAdditionalQuery
        + GeneratorRegistration
        + GeneratorEarningsAndSlashing
        + GeneratorAvailability
        + WithdrawalManagement,
    NS: NativeStakingOperations,
    SS: TokenLockManagement + VaultSnapshotManagement + OperatorStakeManagement + SlashResultManagement,
    KS: KeyStoreOperations,
    SM: StakeManagerOperations,
    CS: CostStoreOperations,
> {
    shared_market_data: Arc<RwLock<MS>>,
    shared_local_ask_data: Arc<RwLock<AS>>,
    shared_parsed_block: Arc<RwLock<U64>>,
    shared_matching_key_clone: Arc<RwLock<Vec<u8>>>,
    shared_entity_key_registry: EntityRegistryInstance,
    shared_generator_data: Arc<RwLock<GS>>,
    shared_native_staking_data: Arc<RwLock<NS>>,
    shared_symbiotic_staking_data: Arc<RwLock<SS>>,
    shared_key_data: Arc<RwLock<KS>>,
    shared_cost_store_data: Arc<RwLock<CS>>,
    shared_stake_manager_store: Arc<RwLock<SM>>,
    relayer_key_balance: Arc<RwLock<ethers::types::U256>>,
    should_stop: Arc<AtomicBool>,
    shared_unhandled_logs: Arc<RwLock<Vec<Log>>>,
    shared_matching_errors: Arc<RwLock<Vec<String>>>,
}

impl<
        MS: MarketMetadataStoreRead + Send + Sync + 'static,
        AS: AskManagementRead
            + CompletedProofsManagement
            + ProofCounters
            + RequestorCounters
            + TimingOperations
            + MarketRequestCounters
            + Send
            + Sync
            + 'static,
        GS: GeneratorAdditionalQuery
            + GeneratorRegistration
            + GeneratorEarningsAndSlashing
            + GeneratorAvailability
            + JobMissedCounter
            + WithdrawalManagement
            + Send
            + Sync
            + 'static,
        NS: NativeStakingOperations + Send + Sync + 'static,
        SS: TokenLockManagement
            + VaultSnapshotManagement
            + OperatorStakeManagement
            + SlashResultManagement
            + Send
            + Sync
            + 'static,
        KS: KeyStoreOperations + Send + Sync + 'static,
        SM: StakeManagerOperations + Send + Sync + 'static,
        CS: CostStoreOperations + Send + Sync + 'static,
    > MatchingEngineServer<MS, AS, GS, NS, SS, KS, SM, CS>
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        shared_market_data: Arc<RwLock<MS>>,
        shared_local_ask_data: Arc<RwLock<AS>>,
        shared_parsed_block: Arc<RwLock<U64>>,
        shared_matching_key_clone: Arc<RwLock<Vec<u8>>>,
        shared_entity_key_registry: EntityRegistryInstance,
        shared_generator_data: Arc<RwLock<GS>>,
        shared_native_staking_data: Arc<RwLock<NS>>,
        shared_symbiotic_staking_data: Arc<RwLock<SS>>,
        shared_key_data: Arc<RwLock<KS>>,
        shared_cost_store_data: Arc<RwLock<CS>>,
        shared_stake_manager_store: Arc<RwLock<SM>>,
        relayer_key_balance: Arc<RwLock<ethers::types::U256>>,
        should_stop: Arc<AtomicBool>,
        shared_unhandled_logs: Arc<RwLock<Vec<Log>>>,
        shared_matching_errors: Arc<RwLock<Vec<String>>>,
    ) -> Self {
        MatchingEngineServer {
            shared_market_data,
            shared_local_ask_data,
            shared_parsed_block,
            shared_matching_key_clone,
            shared_entity_key_registry,
            shared_generator_data,
            shared_native_staking_data,
            shared_symbiotic_staking_data,
            shared_cost_store_data,
            shared_stake_manager_store,
            shared_key_data,
            relayer_key_balance,
            should_stop,
            shared_unhandled_logs,
            shared_matching_errors,
        }
    }

    pub async fn start_server(self, port: u16, enable_ssc: bool) -> anyhow::Result<()> {
        let server = HttpServer::new(move || {
            let max_requests = 15 as usize;

            let ui_request_concurrency = ConcurrencyLimiter::new(max_requests);
            let ui_rate_limiter = kalypso_helper::middlewares::ratelimiter::get_rate_limiter(
                Duration::from_secs(1),
                max_requests as u64,
            );

            let stats_request_concurrency = ConcurrencyLimiter::new(max_requests);
            let stats_rate_limiter = kalypso_helper::middlewares::ratelimiter::get_rate_limiter(
                Duration::from_secs(1),
                max_requests as u64,
            );

            let core_request_concurrency = ConcurrencyLimiter::new(max_requests);
            let core_rate_limiter = kalypso_helper::middlewares::ratelimiter::get_rate_limiter(
                Duration::from_secs(1),
                max_requests as u64,
            );

            #[cfg(not(feature = "matching_engine_enable_cors"))]
            let matching_engine_server = App::new();

            #[cfg(feature = "matching_engine_enable_cors")]
            let matching_engine_server =
                App::new().wrap(kalypso_helper::middlewares::dirty_cors::get_dirty_cors());

            matching_engine_server
                .app_data(Data::new(self.shared_market_data.clone()))
                .app_data(Data::new(self.shared_local_ask_data.clone()))
                .app_data(Data::new(self.shared_parsed_block.clone()))
                .app_data(Data::new(self.shared_matching_key_clone.clone()))
                .app_data(Data::new(self.shared_entity_key_registry.clone()))
                .app_data(Data::new(self.shared_generator_data.clone()))
                .app_data(Data::new(self.shared_native_staking_data.clone()))
                .app_data(Data::new(self.shared_symbiotic_staking_data.clone()))
                .app_data(Data::new(self.shared_cost_store_data.clone()))
                .app_data(Data::new(self.shared_stake_manager_store.clone()))
                .app_data(Data::new(self.shared_key_data.clone()))
                .app_data(Data::new(self.relayer_key_balance.clone()))
                .app_data(Data::new(self.shared_unhandled_logs.clone()))
                .app_data(Data::new(self.shared_matching_errors.clone()))
                .service(get_swagger())
                .service(
                    ui_scope::<MS, AS, GS, NS, SS, KS>()
                        .wrap(ui_request_concurrency)
                        .wrap(ui_rate_limiter),
                )
                .service(
                    get_stats_scope::<AS, GS, SS>()
                        .wrap(stats_request_concurrency)
                        .wrap(stats_rate_limiter),
                )
                .service(
                    get_core_scope()
                        .wrap(core_request_concurrency)
                        .wrap(core_rate_limiter),
                )
        });

        if enable_ssc {
            let tls_config = kalypso_helper::ssc::create_random_rustls_server_config();
            // Error handling for TLS configuration
            if let Err(err) = tls_config {
                log::error!("Failed to create TLS config: {}", err);
                self.should_stop.store(true, Ordering::Release);
                return Err(anyhow::Error::from(err));
            }

            let tls_config = tls_config.unwrap();

            // Bind the server using Rustls for HTTPS
            let server = server.bind_rustls(format!("0.0.0.0:{}", port), tls_config);
            if let Err(err) = server {
                log::error!("Failed to bind server with Rustls: {}", err);
                self.should_stop.store(true, Ordering::Release);
                return Err(anyhow::Error::from(err));
            }

            // Run the server and await
            server.unwrap().run().await?;
        } else {
            // Bind the server using plain HTTP
            let server = server.bind(format!("0.0.0.0:{}", port));
            if let Err(err) = server {
                log::error!("Failed to bind server with HTTP: {}", err);
                self.should_stop.store(true, Ordering::Release);
                return Err(anyhow::Error::from(err));
            }

            // Run the server and await
            server.unwrap().run().await?;
        }

        Ok(())
    }
}
