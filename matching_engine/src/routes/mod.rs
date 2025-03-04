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
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod ask_status;
mod chain_status;
mod decrypt_request;
mod get_priv_inputs;
mod market_info;
mod symbiotic_snapshot;
mod ui_routes;
mod unhandled_logs;

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

pub fn ui_scope<
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
        + WithdrawalManagement
        + JobMissedCounter
        + Send
        + Sync
        + 'static,
    NS: NativeStakingOperations + Send + Sync + 'static,
    SS: TokenLockManagement + Send + Sync + 'static,
    KS: KeyStoreOperations + Send + Sync + 'static,
>() -> actix_web::Scope {
    web::scope("/ui")
        .route("/welcome", web::get().to(ui_routes::welcome::welcome))
        .route(
            "/dashboard",
            web::get().to(ui_routes::dashboard::get_dashboard::<MS, AS, GS, NS, SS>),
        )
        .route(
            "/generators",
            web::get().to(ui_routes::generators::get_generators_all::<MS, GS, NS, SS>),
        )
        .route(
            "/generator/{id}",
            web::get().to(ui_routes::single_generator::single_generator::<MS, AS, GS, NS, SS, KS>),
        )
        .route(
            "/markets",
            web::get().to(ui_routes::markets::total_market_info::<MS, AS, GS, NS, SS>),
        )
        .route(
            "/market/{id}",
            web::get().to(ui_routes::single_market::single_market::<MS, AS, GS, NS, SS>),
        )
        .route(
            "/market_jobs/{id}",
            web::get().to(ui_routes::single_market::jobs::<AS>),
        )
        .route(
            "/withdrawals/{id}",
            web::get().to(ui_routes::single_generator::withdrawal_request::<GS>),
        )
}

pub fn get_stats_scope<
    AS: AskManagementRead + ProofCounters + TimingOperations + Send + Sync + 'static,
    GS: GeneratorAdditionalQuery
        + GeneratorAvailability
        + GeneratorRegistration
        + Send
        + Sync
        + 'static,
    SS: VaultSnapshotManagement
        + OperatorStakeManagement
        + SlashResultManagement
        + TokenLockManagement
        + Send
        + Sync
        + 'static,
    BS: LatestBlockStoreTrait + Send + Sync + 'static,
>() -> actix_web::Scope {
    web::scope("/stats")
        .route("/welcome", web::get().to(chain_status::welcome))
        .route("/getStatus", web::get().to(ask_status::get_status::<AS>))
        .route(
            "/getKeyBalance",
            web::get().to(chain_status::gas_key_balance),
        )
        .route(
            "/getAskStatus",
            web::post().to(ask_status::get_ask_status_askid::<AS>),
        )
        .route("/getAsk/{id}", web::get().to(ask_status::get_ask::<AS>))
        .route(
            "/getProof",
            web::post().to(ask_status::get_ask_proof_by_ask_id::<AS>),
        )
        .route(
            "/getPrivInput",
            web::post().to(get_priv_inputs::get_priv_input::<AS>),
        )
        .route(
            "/getLatestBlock",
            web::get().to(chain_status::get_latest_block_number::<BS>),
        )
        .route(
            "/marketInfo",
            web::post().to(market_info::market_info::<AS, GS>),
        )
        .route(
            "/marketStats/{marketId}",
            web::get().to(market_info::market_stats::<AS, GS>),
        )
        .route("/dump", web::get().to(ui_routes::welcome::get_dump))
        .route(
            "/encrypted_dump",
            web::get().to(ui_routes::welcome::get_encrypted_dump),
        )
        .route(
            "/unhandled_logs",
            web::get().to(unhandled_logs::get_unhandled_logs),
        )
        .route(
            "/matching_errors",
            web::get().to(unhandled_logs::get_matching_errors),
        )
        .route(
            "/symbiotic_snapshot",
            web::get().to(symbiotic_snapshot::get_snapshot::<SS>),
        )
}

use crate::ask_lib::ask_store::AskManagementRead;
use crate::ask_lib::ask_store::CompletedProofsManagement;
use crate::ask_lib::ask_store::MarketRequestCounters;
use crate::ask_lib::ask_store::ProofCounters;
use crate::ask_lib::ask_store::RequestorCounters;
use crate::ask_lib::ask_store::TimingOperations;
use crate::generator_lib::key_store::KeyStoreOperations;
use crate::generator_lib::native_stake_store::NativeStakingOperations;
use crate::generator_lib::symbiotic_stake_store::OperatorStakeManagement;
use crate::generator_lib::symbiotic_stake_store::SlashResultManagement;
use crate::generator_lib::symbiotic_stake_store::TokenLockManagement;
use crate::generator_lib::symbiotic_stake_store::VaultSnapshotManagement;
use crate::generator_lib::traits::GeneratorAdditionalQuery;
use crate::generator_lib::traits::GeneratorAvailability;
use crate::generator_lib::traits::GeneratorEarningsAndSlashing;
use crate::generator_lib::traits::GeneratorRegistration;
use crate::generator_lib::traits::JobMissedCounter;
use crate::generator_lib::traits::WithdrawalManagement;
use crate::latest_block_store::LatestBlockStoreTrait;
use crate::market_metadata::MarketMetadataStoreRead;
use crate::routes::ask_status::*;
use crate::routes::chain_status::*;
use crate::routes::market_info::*;
use crate::routes::symbiotic_snapshot::*;
use crate::routes::unhandled_logs::*;

use crate::routes::ui_routes::dashboard::*;
use crate::routes::ui_routes::generators::*;
use crate::routes::ui_routes::markets::*;
use crate::routes::ui_routes::single_generator::*;
use crate::routes::ui_routes::single_market::*;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Kalypso Indexer APIs",
        description = "APIs to interact with kalypo indexer",
        version = if cfg!(feature = "mainnet") {
            "mainnet"
        } else if cfg!(feature = "stagenet") {
            "stagenet"
        } else {
            "beta"
        },
        license(name = "MIT License", url = "https://opensource.org/licenses/MIT")
    ),
    tags(
        (name = "Manage", description = "Read the matching engine state"),
        (name = "UI", description = "Data Modelled to suit UI view"),
    )
)]
#[openapi(paths(
    welcome,
    gas_key_balance,
    get_latest_block_number,
    get_ask,
    get_ask_status_askid,
    get_ask_proof_by_ask_id,
    get_unhandled_logs,
    get_matching_errors,
    get_snapshot,
    market_stats,
    market_info,
    get_dashboard,
    get_generators_all,
    total_market_info,
    single_generator,
    withdrawal_request,
    single_market
))]
struct ApiDoc;

pub fn get_swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}

pub fn get_core_scope() -> actix_web::Scope {
    web::scope("") // "" means root scope
        .route(
            "/decryptRequest",
            web::post().to(decrypt_request::decrypt_request),
        )
}
