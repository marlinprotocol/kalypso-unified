use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use ethers::types::U64;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;

use crate::middlewares;
use crate::routes;
use crate::{
    ask::{LocalAskStore, MarketMetadataStore},
    generator_lib::generator_store::GeneratorStore,
};

type EntityRegistryInstance = Arc<
    Mutex<
        bindings::entity_key_registry::EntityKeyRegistry<
            SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        >,
    >,
>;

pub struct MatchingEngineServer {
    shared_market_data: Arc<Mutex<MarketMetadataStore>>,
    shared_local_ask_data: Arc<Mutex<LocalAskStore>>,
    shared_parsed_block: Arc<Mutex<U64>>,
    shared_matching_key_clone: Arc<Mutex<Vec<u8>>>,
    shared_entity_key_registry: EntityRegistryInstance,
    shared_generator_data: Arc<Mutex<GeneratorStore>>,
    relayer_key_balance: Arc<Mutex<ethers::types::U256>>,
    should_stop: Arc<AtomicBool>,
}

impl MatchingEngineServer {
    pub fn new(
        shared_market_data: Arc<Mutex<MarketMetadataStore>>,
        shared_local_ask_data: Arc<Mutex<LocalAskStore>>,
        shared_parsed_block: Arc<Mutex<U64>>,
        shared_matching_key_clone: Arc<Mutex<Vec<u8>>>,
        shared_entity_key_registry: EntityRegistryInstance,
        shared_generator_data: Arc<Mutex<GeneratorStore>>,
        relayer_key_balance: Arc<Mutex<ethers::types::U256>>,
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
            App::new()
                .wrap(middlewares::ratelimiter::get_rate_limiter())
                .app_data(Data::new(self.shared_market_data.clone()))
                .app_data(Data::new(self.shared_local_ask_data.clone()))
                .app_data(Data::new(self.shared_parsed_block.clone()))
                .app_data(Data::new(self.shared_matching_key_clone.clone()))
                .app_data(Data::new(self.shared_entity_key_registry.clone()))
                .app_data(Data::new(self.shared_generator_data.clone()))
                .app_data(Data::new(self.relayer_key_balance.clone()))
                .route("/welcome", web::get().to(routes::chain_status::welcome))
                .route("/getStatus", web::get().to(routes::ask_status::get_status))
                .route(
                    "/getKeyBalance",
                    web::get().to(routes::chain_status::gas_key_balance),
                )
                .route(
                    "/getAskStatus",
                    web::post().to(routes::ask_status::get_ask_status_askid),
                )
                .route(
                    "/getProof",
                    web::post().to(routes::ask_status::get_ask_proof_by_ask_id),
                )
                .route(
                    "/getPrivInput",
                    web::post().to(routes::get_priv_inputs::get_priv_input),
                )
                .route(
                    "/decryptRequest",
                    web::post().to(routes::decrypt_request::decrypt_request),
                )
                .route(
                    "/getLatestBlock",
                    web::get().to(routes::chain_status::get_latest_block_number),
                )
                .route(
                    "/marketInfo",
                    web::post().to(routes::market_info::market_info),
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
