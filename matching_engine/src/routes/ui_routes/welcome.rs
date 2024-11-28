use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

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

// Define the Dump struct
#[derive(Serialize)]
struct Dump<'a> {
    market_metadata_store: Option<&'a MarketMetadataStore>,
    local_ask_store: Option<&'a LocalAskStore>,
    generator_store: Option<&'a GeneratorStore>,
    native_staking_store: Option<&'a NativeStakingStore>,
    symbiotic_stake_store: Option<&'a SymbioticStakeStore>,
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

    let dump = Dump {
        market_metadata_store: Some(&*market_store),
        local_ask_store: Some(&*ask_store),
        generator_store: Some(&*generator_store),
        native_staking_store: Some(&*native_store),
        symbiotic_stake_store: Some(&*symbiotic_store),
    };

    // Return the JSON response
    Ok(HttpResponse::Ok().json(dump))
}
