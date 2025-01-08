use std::sync::Arc;

use ethers::prelude::{k256::ecdsa::SigningKey, *};
use tokio::sync::RwLock;

use crate::{generator_lib::stake_manager_store, log_processor::constants};

pub async fn process_staking_manager_log(
    log: &Log,
    staking_manager: &bindings::staking_manager::StakingManager<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
    stake_manager_store: &Arc<RwLock<stake_manager_store::StakeManagerStore>>,
    supported_native_staking_pool: Address,
    supported_symbiotic_staking_pool: Address,
    unhandled_logs: &Arc<RwLock<Vec<Log>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if constants::STAKING_MANAGER_TOPICS_SKIP
        .get(&log.topics[0])
        .is_some()
    {
        log::debug!("standard topic to skip found, ignoring it");
        return Ok(());
    }

    let mut stake_manager_store = { stake_manager_store.write().await };

    // typed event not working here
    // <bindings::staking_manager::StakingPoolAddedFilter>
    if let Ok(event_log) =
        staking_manager.decode_event_raw("StakingPoolAdded", log.topics.clone(), log.data.clone())
    {
        log::debug!("StakingPoolAdded Logs: {:?}", event_log);
        let staking_pool = event_log.get(0).unwrap().clone().into_address().unwrap();
        // let staking_pool = event_log.pool;
        log::warn!("Dynamic stake pool management is not supported yet");

        if staking_pool == supported_native_staking_pool
            || staking_pool == supported_symbiotic_staking_pool
        {
        } else {
            log::error!("unsupported pool by matching engine {:?}", log);
            return Err("unsupported pool by matching engine".into());
        }

        if stake_manager_store.exists(&staking_pool) {
            log::debug!("Staking Pool: {} already exists", staking_pool);
        }

        let existing_staking_pools = stake_manager_store.get_all();
        for staking_pool in existing_staking_pools {
            log::debug!("Existing Staking Pool: {}", staking_pool);
        }
        return Ok(());
    }

    if let Ok(event_log) = staking_manager
        .decode_event::<bindings::staking_manager::PoolRewardShareSetFilter>(
            "PoolRewardShareSet",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("PoolRewardShareSet Logs: {:?}", event_log);
        return Ok(());
    }

    // typed event not working
    // <bindings::staking_manager::ProofMarketplaceSetFilter>
    if let Ok(event_log) = staking_manager.decode_event_raw(
        "ProofMarketplaceSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("ProofMarketplaceSet Logs: {:?}", event_log);
        return Ok(());
    }

    // typed event not working
    // <bindings::staking_manager::FeeTokenSetFilter>
    if let Ok(event_log) =
        staking_manager.decode_event_raw("FeeTokenSet", log.topics.clone(), log.data.clone())
    {
        log::debug!("FeeTokenSet Logs: {:?}", event_log);
        return Ok(());
    }

    // typed event not working
    // <bindings::staking_manager::SymbioticStakingSetFilter>
    if let Ok(event_log) = staking_manager.decode_event_raw(
        "SymbioticStakingSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("SymbioticStakingSet Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = staking_manager
        .decode_event::<bindings::staking_manager::PoolEnabledSetFilter>(
            "PoolEnabledSet",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("PoolEnabledSet Logs: {:?}", event_log);
        // let staking_pool = event_log.get(0).unwrap().clone().into_address().unwrap();
        // let is_enabled = event_log.get(1).unwrap().clone().into_bool().unwrap();

        let staking_pool = event_log.pool;
        let is_enabled = event_log.enabled;

        log::debug!("Staking Pool: {} is_enabled: {}", staking_pool, is_enabled);

        if is_enabled {
            stake_manager_store.add(staking_pool);
        } else {
            stake_manager_store.remove(&staking_pool);
        }
        return Ok(());
    }

    if cfg!(feature = "skip_unknown_events") {
        log::warn!("{:?}", log);
        log::warn!("Unknown event noted and skipped");
        let mut unhandled_logs = { unhandled_logs.write().await };
        unhandled_logs.push(log.clone());
        return Ok(());
    }

    log::error!("unhandled log in staking manager {:?}", log);
    return Err("Unhandled log in staking manager".into());
}
