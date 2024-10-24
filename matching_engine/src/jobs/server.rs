use std::sync::Arc;
use std::time::Duration;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use ethers::types::U64;
use kalypso_helper::middlewares::request_limiter::ConcurrencyLimiter;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;

use crate::market_metadata::MarketMetadataStore;
use crate::routes::{get_core_scope, get_stats_scope, ui_scope};
use crate::{ask_lib::ask_store::LocalAskStore, generator_lib::generator_store::GeneratorStore};

type EntityRegistryInstance = Arc<
    RwLock<
        bindings::entity_key_registry::EntityKeyRegistry<
            SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        >,
    >,
>;

pub struct MatchingEngineServer {
    shared_market_data: Arc<RwLock<MarketMetadataStore>>,
    shared_local_ask_data: Arc<RwLock<LocalAskStore>>,
    shared_parsed_block: Arc<RwLock<U64>>,
    shared_matching_key_clone: Arc<RwLock<Vec<u8>>>,
    shared_entity_key_registry: EntityRegistryInstance,
    shared_generator_data: Arc<RwLock<GeneratorStore>>,
    relayer_key_balance: Arc<RwLock<ethers::types::U256>>,
    should_stop: Arc<AtomicBool>,
}

impl MatchingEngineServer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        shared_market_data: Arc<RwLock<MarketMetadataStore>>,
        shared_local_ask_data: Arc<RwLock<LocalAskStore>>,
        shared_parsed_block: Arc<RwLock<U64>>,
        shared_matching_key_clone: Arc<RwLock<Vec<u8>>>,
        shared_entity_key_registry: EntityRegistryInstance,
        shared_generator_data: Arc<RwLock<GeneratorStore>>,
        relayer_key_balance: Arc<RwLock<ethers::types::U256>>,
        should_stop: Arc<AtomicBool>,
    ) -> Self {
        MatchingEngineServer {
            shared_market_data,
            shared_local_ask_data,
            shared_parsed_block,
            shared_matching_key_clone,
            shared_entity_key_registry,
            shared_generator_data,
            relayer_key_balance,
            should_stop,
        }
    }

    pub async fn start_server(self, port: u16, enable_ssc: bool) -> anyhow::Result<()> {
        let server = HttpServer::new(move || {
            let ui_request_concurrency = ConcurrencyLimiter::new(2);
            let ui_rate_limiter = kalypso_helper::middlewares::ratelimiter::get_rate_limiter(
                Duration::from_secs(1),
                10 as u64,
            );

            let stats_request_concurrency = ConcurrencyLimiter::new(2);
            let stats_rate_limiter = kalypso_helper::middlewares::ratelimiter::get_rate_limiter(
                Duration::from_secs(1),
                10 as u64,
            );

            let core_request_concurrency = ConcurrencyLimiter::new(10);
            let core_rate_limiter = kalypso_helper::middlewares::ratelimiter::get_rate_limiter(
                Duration::from_secs(1),
                10 as u64,
            );

            #[cfg(not(feature = "matching_engine_enable_cors"))]
            let matching_engine_server = App::new();

            #[cfg(feature = "matching_engine_enable_cors")]
            let matching_engine_server = App::new().wrap(kalypso_helper::middlewares::dirty_cors::get_dirty_cors());


            matching_engine_server
                .app_data(Data::new(self.shared_market_data.clone()))
                .app_data(Data::new(self.shared_local_ask_data.clone()))
                .app_data(Data::new(self.shared_parsed_block.clone()))
                .app_data(Data::new(self.shared_matching_key_clone.clone()))
                .app_data(Data::new(self.shared_entity_key_registry.clone()))
                .app_data(Data::new(self.shared_generator_data.clone()))
                .app_data(Data::new(self.relayer_key_balance.clone()))
                .service(
                    ui_scope()
                        .wrap(ui_request_concurrency)
                        .wrap(ui_rate_limiter),
                )
                .service(
                    get_stats_scope()
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
