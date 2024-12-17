use std::sync::Arc;

use ethers::prelude::{k256::ecdsa::SigningKey, *};
use tokio::sync::RwLock;

use crate::{
    ask_lib::ask_store,
    generator_lib::{
        delegation,
        generator_store::{self, WithdrawlRequest},
        native_stake_store,
    },
    log_processor::constants,
    utility::{get_l1_block_from_l2_block, tx_to_string},
};

pub async fn process_native_staking_logs(
    log: &Log,
    native_staking: &bindings::native_staking::NativeStaking<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
    generator_store: &Arc<RwLock<generator_store::GeneratorStore>>,
    native_store: &Arc<RwLock<native_stake_store::NativeStakingStore>>,
    #[allow(unused)] ask_store: &Arc<RwLock<ask_store::LocalAskStore>>,
    rpc_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if constants::NATIVE_STAKING_TOPICS_SKIP
        .get(&log.topics[0])
        .is_some()
    {
        log::debug!("standard topic to skip found, ignoring it");
        return Ok(());
    }
    if let Ok(stake_manager_set_log) =
        native_staking.decode_event_raw("StakingManagerSet", log.topics.clone(), log.data.clone())
    {
        log::debug!("Staking Manager Set Logs: {:?}", stake_manager_set_log);
        return Ok(());
    }

    let mut native_store = { native_store.write().await };

    if let Ok(event_log) =
        native_staking.decode_event_raw("StakeTokenAdded", log.topics.clone(), log.data.clone())
    {
        log::debug!("StakeTokenAdded Logs: {:?}", event_log);
        let token = event_log.get(0).unwrap().clone().into_address().unwrap();
        let weight = event_log.get(1).unwrap().clone().into_uint().unwrap();

        log::debug!("Added token: {} with weight: {}", token, weight);
        native_store.set_lock_token(token, U256::zero());
        return Ok(());
    }

    if let Ok(event_log) =
        native_staking.decode_event_raw("StakeTokenRemoved", log.topics.clone(), log.data.clone())
    {
        log::debug!("StakeTokenRemoved Logs: {:?}", event_log);
        let token = event_log.get(0).unwrap().clone().into_address().unwrap();

        log::debug!("Removed token: {}", token);
        native_store.remove_lock_token(token);
        return Ok(());
    }

    if let Ok(event_log) =
        native_staking.decode_event_raw("AmountToLockSet", log.topics.clone(), log.data.clone())
    {
        log::debug!("AmountToLockSet Logs: {:?}", event_log);
        let token = event_log.get(0).unwrap().clone().into_address().unwrap();
        let amount = event_log.get(1).unwrap().clone().into_uint().unwrap();

        log::debug!("AmountToLockSet: Token: {}  Amount: {}", token, amount);
        native_store.set_lock_token(token, amount);
        return Ok(());
    }

    let mut generator_store = { generator_store.write().await };

    if let Ok(added_stake_log) = native_staking
        .decode_event::<bindings::native_staking::StakedFilter>(
            "Staked",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!(
            "Native stake Added. Generator: {}",
            added_stake_log.operator
        );

        let address = added_stake_log.operator;
        let amount = added_stake_log.amount;
        let token_address = added_stake_log.token;

        let block_l2: U256 = log.block_number.unwrap().as_u64().into();
        let block_l1: U256 = get_l1_block_from_l2_block(rpc_url, block_l2)
            .await
            .unwrap_or_default();

        generator_store.add_extra_stake(
            &address,
            &token_address,
            &amount,
            U64::from(block_l1.as_u64()),
            log.transaction_index.unwrap(),
            log.log_index.unwrap(),
            tx_to_string(&log.transaction_hash.unwrap()),
            delegation::Source::Native,
        );

        return Ok(());
    }

    if let Ok(request_stake_decrease_log) = native_staking
        .decode_event::<bindings::native_staking::StakeWithdrawalRequestedFilter>(
        "StakeWithdrawalRequested",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!(
            "Request stake decrease for Generator: {:?}",
            request_stake_decrease_log.operator
        );

        let address = request_stake_decrease_log.operator;
        let account = request_stake_decrease_log.account;
        let index = request_stake_decrease_log.index;
        let token = request_stake_decrease_log.token;
        let amount = request_stake_decrease_log.amount;

        log::warn!("pausing all assignments across all markets");
        log::warn!("will be unpaused once the request if fully withdrawn");

        let withdrawal_request_time = match native_staking
            .withdrawal_requests(account, address, index)
            .call()
            .await
        {
            Ok(data) => data.2,
            Err(err) => {
                log::error!("Failed Querying withdrawal request timestamp: {}", err);
                0.into()
            }
        };

        generator_store.pause_assignments_across_all_markets(&address);

        log::warn!("Setting new utilization to same value");
        let new_utilization = 1000000000000000000_i64.into();
        generator_store.update_intended_stake_util(&address, new_utilization);
        generator_store.insert_withdrawal_request(
            &address,
            WithdrawlRequest {
                account,
                index,
                token,
                amount,
                timestamp: withdrawal_request_time,
            },
        );
        return Ok(());
    }

    if let Ok(remove_stake_log) = native_staking
        .decode_event::<bindings::native_staking::StakeWithdrawnFilter>(
            "StakeWithdrawn",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!(
            "Remove stake for Generator: {:?}",
            remove_stake_log.operator
        );

        let address = remove_stake_log.operator;
        let amount = remove_stake_log.amount;
        let token_address = remove_stake_log.token;
        let account = remove_stake_log.account;
        let index = remove_stake_log.index;

        let block_l2: U256 = log.block_number.unwrap().as_u64().into();
        let block_l1: U256 = get_l1_block_from_l2_block(rpc_url, block_l2)
            .await
            .unwrap_or_default();

        let withdrawal_request_time = match native_staking
            .withdrawal_requests(account, address, index)
            .call()
            .await
        {
            Ok(data) => data.2,
            Err(err) => {
                log::error!("Failed Querying withdrawal request timestamp: {}", err);
                0.into()
            }
        };

        generator_store.remove_stake(
            &address,
            &token_address,
            &amount,
            U64::from(block_l1.as_u64()),
            log.transaction_index.unwrap(),
            log.log_index.unwrap(),
            tx_to_string(&log.transaction_hash.unwrap()),
            delegation::Operation::UnDelegate,
            delegation::Source::Native,
        );
        generator_store.resume_assignments_accross_all_markets(&address);
        generator_store.update_intended_stake_util(&address, 1000000000000000000_i64.into());
        generator_store.remove_withdrawal_request(
            &address,
            WithdrawlRequest {
                account,
                index,
                token: token_address,
                amount,
                timestamp: withdrawal_request_time,
            },
        );

        return Ok(());
    }

    if let Ok(stake_lock_logs) = native_staking
        .decode_event::<bindings::native_staking::StakeLockedFilter>(
            "StakeLocked",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("Stake Locked: {:?}", stake_lock_logs);
        let address = stake_lock_logs.operator;
        let stake_locked = stake_lock_logs.amount;
        let token_address = stake_lock_logs.token;

        generator_store.update_on_stake_locked(
            &address,
            &token_address,
            stake_locked,
            delegation::Source::Native,
        );
        return Ok(());
    }

    if let Ok(stake_lock_logs) = native_staking
        .decode_event::<bindings::native_staking::StakeUnlockedFilter>(
            "StakeUnlocked",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("Stake Lock Released: {:?}", stake_lock_logs);
        let address = stake_lock_logs.operator;
        let stake_released = stake_lock_logs.amount;
        let token_address = stake_lock_logs.token;
        generator_store.update_on_stake_released(
            &address,
            &token_address,
            stake_released,
            delegation::Source::Native,
        );
        return Ok(());
    }

    if let Ok(stake_slash_logs) = native_staking
        .decode_event::<bindings::native_staking::JobSlashedFilter>(
            "JobSlashed",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::warn!("Job/Stake Slashed: {:?}", stake_slash_logs);
        let address = stake_slash_logs.operator;
        let stake_slashed = stake_slash_logs.amount;
        let token_address = stake_slash_logs.token;

        let block_l2: U256 = log.block_number.unwrap().as_u64().into();
        let block_l1: U256 = get_l1_block_from_l2_block(rpc_url, block_l2)
            .await
            .unwrap_or_default();

        let stake_slashed = if cfg!(feature = "override_native_slashing_to_zero") {
            log::warn!("Native slashing is disabled, however an 0 entry is still noted");
            0.into()
        } else {
            stake_slashed
        };

        generator_store.remove_stake(
            &address,
            &token_address,
            &stake_slashed,
            U64::from(block_l1.as_u64()),
            log.transaction_index.unwrap(),
            log.log_index.unwrap(),
            tx_to_string(&log.transaction_hash.unwrap()),
            delegation::Operation::Slash,
            delegation::Source::Native,
        );

        // No need to generate this if there are dummy logs enabled
        #[cfg(not(feature = "generate_dummy_slash_logs"))]
        {
            use crate::utility::get_timestamp_from_l2block_number;

            let ask_id = stake_slash_logs.job_id; //job _id and ask_id are same
            let ask = { ask_store.read().await.get_by_ask_id(&ask_id).unwrap() };

            let market_id = ask.market_id;
            let reward = ask.reward;
            let deadline = ask.deadline;

            let (native_slashing_tokens, native_slashings): (Vec<Address>, Vec<U256>) =
                { (vec![token_address], vec![stake_slashed]) };

            // only notes the slashing entry, doesn't update stake
            generator_store.note_entry_slashing(
                &address,
                &ask_id,
                &market_id,
                native_slashing_tokens,
                native_slashings,
                vec![],
                vec![],
                tx_to_string(&log.transaction_hash.unwrap()),
                &reward,
                &deadline,
                &U64::from(block_l1.as_u64()),
                &get_timestamp_from_l2block_number(rpc_url, &block_l1)
                    .await
                    .unwrap_or_default(),
            );
        }
        return Ok(());
    }

    if cfg!(feature = "skip_unknown_events") {
        log::warn!("{:?}", log);
        log::warn!("Unknown event noted and skipped");
        return Ok(());
    }

    log::error!("unhandled log in native staking {:?}", log);
    return Err("Unhandled log in native staking".into());
}
