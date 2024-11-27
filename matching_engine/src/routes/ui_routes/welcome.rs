use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};

use crate::ask_lib::ask_store::LocalAskStore;
use crate::generator_lib::generator_store::GeneratorStore;
use crate::generator_lib::native_stake_store::NativeStakingStore;
use crate::generator_lib::symbiotic_stake_store::SymbioticStakeStore;

use crate::market_metadata::MarketMetadataStore;
use crate::{models::WelcomeResponse, try_read_or_lock};
use actix_web::{web::Data, HttpResponse};

pub async fn welcome() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(WelcomeResponse {
        status: "welcome to ui routes!.".into(),
    }))
}

pub async fn get_dump(
    local_market_store: Data<Arc<RwLock<MarketMetadataStore>>>,
    local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    local_generator_store: Data<Arc<RwLock<GeneratorStore>>>,
    local_native_store: Data<Arc<RwLock<NativeStakingStore>>>,
    local_symbiotic_store: Data<Arc<RwLock<SymbioticStakeStore>>>,
) -> actix_web::Result<HttpResponse> {
    try_read_or_lock!(local_market_store, market_store);
    try_read_or_lock!(local_ask_store, ask_store);
    try_read_or_lock!(local_generator_store, generator_store);
    try_read_or_lock!(local_native_store, native_store);
    try_read_or_lock!(local_symbiotic_store, symbiotic_store);

    // Create a JSON object containing all the store data
    let dump = serde_json::json!({
        "market_metadata_store": &*market_store,
        "local_ask_store": &*ask_store,
        "generator_store": &*generator_store,
        "native_staking_store": &*native_store,
        "symbiotic_stake_store": &*symbiotic_store,
    });

    // Return the JSON response
    Ok(HttpResponse::Ok().json(dump))
}
