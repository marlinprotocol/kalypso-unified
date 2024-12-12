use ethers::types::U64;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::ask_lib::ask_store::LocalAskStore;
use crate::costs::CostStore;
use crate::generator_lib::generator_store::GeneratorStore;
use crate::generator_lib::key_store::KeyStore;
use crate::generator_lib::native_stake_store::NativeStakingStore;
use crate::generator_lib::stake_manager_store::StakeManagerStore;
use crate::generator_lib::symbiotic_stake_store::SymbioticStakeStore;
use crate::market_metadata::MarketMetadataStore;

use crate::DumpSend;
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
    local_cost_store: Data<Arc<RwLock<CostStore>>>,
    local_key_store: Data<Arc<RwLock<KeyStore>>>,
    local_stake_manager_store: Data<Arc<RwLock<StakeManagerStore>>>,
    local_parsed_block: Data<Arc<RwLock<U64>>>,
) -> actix_web::Result<HttpResponse> {
    try_read_or_lock!(local_market_store, market_store);
    try_read_or_lock!(local_ask_store, ask_store);
    try_read_or_lock!(local_generator_store, generator_store);
    try_read_or_lock!(local_native_store, native_store);
    try_read_or_lock!(local_symbiotic_store, symbiotic_store);
    try_read_or_lock!(local_cost_store, cost_store);
    try_read_or_lock!(local_key_store, key_store);
    try_read_or_lock!(local_stake_manager_store, stake_manager_store);
    try_read_or_lock!(local_parsed_block, parsed_block);

    let dump = DumpSend {
        market_metadata_store: Some(&*market_store),
        local_ask_store: Some(&*ask_store),
        generator_store: Some(&*generator_store),
        native_staking_store: Some(&*native_store),
        symbiotic_stake_store: Some(&*symbiotic_store),
        cost_store: Some(&*cost_store),
        key_store: Some(&*key_store),
        stake_manager_store: Some(&*stake_manager_store),
        parsed_block: Some(&*parsed_block),
    };

    // Return the JSON response
    Ok(HttpResponse::Ok().json(dump))
}
