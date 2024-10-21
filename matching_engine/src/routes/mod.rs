use actix_web::web;
use actix_web::web::Data;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

mod ask_status;
mod chain_status;
mod decrypt_request;
mod get_priv_inputs;
mod market_info;
mod ui_routes;

type EntityRegistryInstance = Data<
    Arc<
        Mutex<
            bindings::entity_key_registry::EntityKeyRegistry<
                SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
            >,
        >,
    >,
>;

#[derive(Serialize)]
pub struct GetRequestResponse {
    encrypted_data: String,
}

pub fn ui_scope() -> actix_web::Scope {
    web::scope("/ui")
        .route("/welcome", web::get().to(ui_routes::welcome::welcome))
        .route(
            "/dashboard",
            web::get().to(ui_routes::dashboard::get_dashboard),
        )
        .route(
            "/generators",
            web::get().to(ui_routes::generators::get_generators_all),
        )
        .route(
            "/generator/{id}",
            web::get().to(ui_routes::single_generator::single_generator),
        )
        .route("/markets", web::get().to(ui_routes::markets::market_info))
}

pub fn get_stats_scope() -> actix_web::Scope {
    web::scope("/stats")
        .route("/welcome", web::get().to(chain_status::welcome))
        .route("/getStatus", web::get().to(ask_status::get_status))
        .route(
            "/getKeyBalance",
            web::get().to(chain_status::gas_key_balance),
        )
        .route(
            "/getAskStatus",
            web::post().to(ask_status::get_ask_status_askid),
        )
        .route(
            "/getProof",
            web::post().to(ask_status::get_ask_proof_by_ask_id),
        )
        .route(
            "/getPrivInput",
            web::post().to(get_priv_inputs::get_priv_input),
        )
        .route(
            "/getLatestBlock",
            web::get().to(chain_status::get_latest_block_number),
        )
        .route("/marketInfo", web::post().to(market_info::market_info))
        .route(
            "/marketStats/{marketId}",
            web::get().to(market_info::market_stats),
        )
}

pub fn get_core_scope() -> actix_web::Scope {
    web::scope("/app").route(
        "/decryptRequest",
        web::post().to(decrypt_request::decrypt_request),
    )
}
