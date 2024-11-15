use ethers::prelude::{k256::ecdsa::SigningKey, *};

use crate::log_processor::constants;

pub async fn process_staking_manager_log(
    log: &Log,
    staking_manager: &bindings::staking_manager::StakingManager<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    if constants::STAKING_MANAGER_TOPICS_SKIP
        .get(&log.topics[0])
        .is_some()
    {
        log::debug!("standard topic to skip found, ignoring it");
        return Ok(());
    }

    if let Ok(event_log) =
        staking_manager.decode_event_raw("StakingPoolAdded", log.topics.clone(), log.data.clone())
    {
        log::debug!("StakingPoolAdded Logs: {:?}", event_log);
        let staking_pool = event_log.get(0).unwrap().clone().into_address().unwrap();
        log::warn!("Staking Pool Added: {}", staking_pool);
        log::warn!("Dynamic stake pool management is not supported yet");
        return Ok(());
    }

    if let Ok(event_log) =
        staking_manager.decode_event_raw("PoolRewardShareSet", log.topics.clone(), log.data.clone())
    {
        log::debug!("PoolRewardShareSet Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) =
        staking_manager.decode_event_raw("PoolEnabledSet", log.topics.clone(), log.data.clone())
    {
        log::debug!("PoolEnabledSet Logs: {:?}", event_log);
        let staking_pool = event_log.get(0).unwrap().clone().into_address().unwrap();
        let is_enabled = event_log.get(1).unwrap().clone().into_bool().unwrap();
        log::warn!("Staking Pool: {} is_enabled: {}", staking_pool, is_enabled);

        return Ok(());
    }

    log::error!("unhandled log in staking manager {:?}", log);
    return Err("Unhandled log in staking manager".into());
}
