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
use crate::Dump;

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

    let dump = Dump {
        market_metadata_store: market_store.clone(),
        local_ask_store: ask_store.clone(),
        generator_store: generator_store.clone(),
        native_staking_store: native_store.clone(),
        symbiotic_stake_store: symbiotic_store.clone(),
        cost_store: cost_store.clone(),
        key_store: key_store.clone(),
        stake_manager_store: stake_manager_store.clone(),
        parsed_block: parsed_block.clone(),
    };

    // Return the JSON response
    Ok(HttpResponse::MethodNotAllowed().json(dump))
}

pub async fn get_encrypted_dump(
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

    let dump = Dump {
        market_metadata_store: market_store.clone(),
        local_ask_store: ask_store.clone(),
        generator_store: generator_store.clone(),
        native_staking_store: native_store.clone(),
        symbiotic_stake_store: symbiotic_store.clone(),
        cost_store: cost_store.clone(),
        key_store: key_store.clone(),
        stake_manager_store: stake_manager_store.clone(),
        parsed_block: parsed_block.clone(),
    };

    let encrypted_dump = dump.create_encrypted_dump().await.unwrap();

    // Return the JSON response
    Ok(HttpResponse::Ok().json(encrypted_dump))
}

#[cfg(test)]
mod tests {
    use super::Dump;
    use serde_json;

    #[tokio::test]
    async fn test_encryption_and_decrytion() {
        // // fetch sample dump
        // let dump_path = "../matching_engine_config/dump.json".to_string();
        // let alt_dump_path = "./matching_engine_config/dump.json".to_string();
        // let file_content =
        //     fs::read_to_string(dump_path).or_else(|_| fs::read_to_string(alt_dump_path)).unwrap();
        // let dump: Dump = serde_json::from_str(&file_content).unwrap();

        // create default dump
        let dump = Dump::default();

        let encrypted_dump = dump.create_encrypted_dump().await.unwrap();

        let decrypted_dump = encrypted_dump.get_dump().unwrap();
        let decrypted_dump_str = serde_json::to_string(&decrypted_dump).unwrap();
        dbg!(decrypted_dump_str);
    }
}
