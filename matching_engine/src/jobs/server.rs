use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use ethers::types::U64;
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
    ) -> Self {
        MatchingEngineServer {
            shared_market_data,
            shared_local_ask_data,
            shared_parsed_block,
            shared_matching_key_clone,
            shared_entity_key_registry,
            shared_generator_data,
            relayer_key_balance,
        }
    }

    pub async fn start_server(self) -> anyhow::Result<()> {
        HttpServer::new(move || {
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
                ) // Provide specific ask status
                .route(
                    "/getPrivInput",
                    web::post().to(routes::get_priv_inputs::get_priv_input),
                ) // provide private inputs for a specific ask
                .route(
                    "/decryptRequest",
                    web::post().to(routes::decrypt_request::decrypt_request),
                ) // Return decrypted input
                .route(
                    "/getLatestBlock",
                    web::get().to(routes::chain_status::get_latest_block_number), // Returns the latest Block parsed so far
                )
                .route(
                    "/marketInfo",
                    web::post().to(routes::market_info::market_info),
                )
        })
        .bind("0.0.0.0:3000")
        .unwrap()
        .run()
        .await
        .unwrap();

        Ok(())
    }
}
