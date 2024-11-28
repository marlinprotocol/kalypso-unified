use std::sync::Arc;

use ethers::prelude::{k256::ecdsa::SigningKey, *};
use tokio::sync::RwLock;

use crate::{
    generator_lib::{delegation, generator_store, symbiotic_stake_store},
    log_processor::constants,
    utility::{get_l1_block_from_l2_block, tx_to_string},
};

pub async fn process_symbiotic_staking_logs(
    log: &Log,
    symbiotic_staking: &bindings::symbiotic_staking::SymbioticStaking<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
    generator_store: &Arc<RwLock<generator_store::GeneratorStore>>,
    symbiotic_stake_store: &Arc<RwLock<symbiotic_stake_store::SymbioticStakeStore>>,
    rpc_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if constants::SYMBIOTIC_STAKING_TOPICS_SKIP
        .get(&log.topics[0])
        .is_some()
    {
        log::warn!("standard topic to skip found, ignoring it");
        return Ok(());
    }

    if let Ok(stake_manager_set_log) = symbiotic_staking.decode_event_raw(
        "StakingManagerSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("Staking Manager Set Logs: {:?}", stake_manager_set_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "ProofMarketplaceSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("ProofMarketplaceSet Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "RewardDistributorSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("RewardDistributorSet Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "FeeRewardTokenSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("FeeRewardTokenSet Logs: {:?}", event_log);
        return Ok(());
    }

    let mut symbiotic_stake_store = { symbiotic_stake_store.write().await };

    if let Ok(event_log) =
        symbiotic_staking.decode_event_raw("StakeTokenAdded", log.topics.clone(), log.data.clone())
    {
        log::debug!("StakeTokenAdded Logs: {:?}", event_log);
        let token = event_log.get(0).unwrap().clone().into_address().unwrap();
        let weight = event_log.get(1).unwrap().clone().into_uint().unwrap();

        log::debug!("Added token: {} with weight: {}", token, weight);
        symbiotic_stake_store.set_lock_token(token, U256::zero());
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "StakeTokenRemoved",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("StakeTokenRemoved Logs: {:?}", event_log);
        let token = event_log.get(0).unwrap().clone().into_address().unwrap();

        log::debug!("Removed token: {}", token);
        symbiotic_stake_store.remove_lock_token(token);
        return Ok(());
    }

    if let Ok(event_log) =
        symbiotic_staking.decode_event_raw("AmountToLockSet", log.topics.clone(), log.data.clone())
    {
        log::debug!("AmountToLockSet Logs: {:?}", event_log);
        log::debug!("AmountToLockSet Logs: {:?}", event_log);
        let token = event_log.get(0).unwrap().clone().into_address().unwrap();
        let amount = event_log.get(1).unwrap().clone().into_uint().unwrap();

        log::debug!("AmountToLockSet: Token: {}  Amount: {}", token, amount);
        symbiotic_stake_store.set_lock_token(token, amount);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "BaseTransmitterComissionRateSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("BaseTransmitterComissionRateSet Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "SubmissionCooldownSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("SubmissionCooldownSet Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "EnclaveImageAdded",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("EnclaveImageAdded Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "AttestationVerifierUpdated",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("AttestationVerifierUpdated Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "EnclaveImageRemoved",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("EnclaveImageRemoved Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "VaultSnapshotSubmitted",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("VaultSnapshotSubmitted Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "SlashResultSubmitted",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("SlashResultSubmitted Logs: {:?}", event_log);
        return Ok(());
    }

    let mut generator_store = { generator_store.write().await };

    if let Ok(symbiotic_complete_snapshot_log) = symbiotic_staking.decode_event_raw(
        "SnapshotConfirmed",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("Processing SnapshotConfirmed");

        let capture_timestamp = {
            let transmitter_token = symbiotic_complete_snapshot_log.get(0).unwrap();
            let transmitter = transmitter_token.clone().into_address().unwrap();
            log::debug!("Transmitter: {}", transmitter);

            let capture_timestamp_token = symbiotic_complete_snapshot_log.get(1).unwrap();
            let capture_timestamp = capture_timestamp_token.clone().into_uint().unwrap();
            capture_timestamp
        };

        let (known_tokens, _): (Vec<Address>, Vec<U256>) = {
            symbiotic_stake_store
                .tokens_to_lock
                .clone()
                .to_address_token_pair()
                .into_iter()
                .unzip()
        };

        let all_generators = generator_store.all_generators_address();

        for stake_token in known_tokens {
            for operator in all_generators.clone().into_iter() {
                // if this fails, system breaks. TODO
                let vault_snapshot_amount = symbiotic_staking
                    .get_operator_stake_amount_at(capture_timestamp, stake_token, operator)
                    .call()
                    .await
                    .unwrap();

                log::debug!(
                    "operator:{}, snapshot token:{}, amount: {}",
                    &operator,
                    &stake_token,
                    vault_snapshot_amount.to_string()
                );

                // before updating in symbiotic, do these steps
                let last_stored_staking_info =
                    symbiotic_stake_store.get_latest_stake_info(&operator, &stake_token);

                if vault_snapshot_amount.gt(&last_stored_staking_info) {
                    generator_store.add_extra_stake(
                        &operator,
                        &stake_token,
                        &(vault_snapshot_amount - last_stored_staking_info),
                        log.block_number.unwrap(),
                        log.transaction_index.unwrap(),
                        log.log_index.unwrap(),
                        tx_to_string(&log.transaction_hash.unwrap()),
                        delegation::Source::Symbiotic,
                    );
                } else if vault_snapshot_amount.lt(&last_stored_staking_info) {
                    generator_store.remove_stake(
                        &operator,
                        &stake_token,
                        &(last_stored_staking_info - vault_snapshot_amount),
                        log.block_number.unwrap(),
                        log.transaction_index.unwrap(),
                        log.log_index.unwrap(),
                        tx_to_string(&log.transaction_hash.unwrap()),
                        delegation::Operation::UnDelegate,
                        delegation::Source::Symbiotic,
                    );
                } else {
                    log::debug!("No change in symbiotic stake noticed");
                }

                symbiotic_stake_store.upsert_stake(&operator, &stake_token, &vault_snapshot_amount);
            }
        }

        return Ok(());
    }

    if let Ok(stake_lock_logs) = symbiotic_staking
        .decode_event::<bindings::symbiotic_staking::StakeLockedFilter>(
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
            delegation::Source::Symbiotic,
        );
        return Ok(());
    }

    if let Ok(stake_lock_logs) = symbiotic_staking
        .decode_event::<bindings::symbiotic_staking::StakeUnlockedFilter>(
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
            delegation::Source::Symbiotic,
        );
        return Ok(());
    }

    if let Ok(stake_slash_logs) = symbiotic_staking
        .decode_event::<bindings::symbiotic_staking::JobSlashedFilter>(
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

        let stake_slashed = if cfg!(feature = "disable_symbiotic_slashing") {
            log::warn!(
                "Symbiotic slashing is disabled, however an 0 undelegation entry is still noted"
            );
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
            delegation::Source::Symbiotic,
        );
        return Ok(());
    }

    if cfg!(feature = "skip_unknown_events") {
        log::warn!("{:?}", log);
        log::warn!("Unknown event noted and skipped");
        return Ok(());
    }

    log::error!("unhandled log in symbiotic staking {:?}", log);
    return Err("Unhandled log in symbiotic staking".into());
}
