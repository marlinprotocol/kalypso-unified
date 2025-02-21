use std::sync::Arc;

use ethers::prelude::{k256::ecdsa::SigningKey, *};
use tokio::sync::RwLock;

use crate::{
    ask_lib::ask_store::{self, AskManagementRead},
    generator_lib::{
        delegation, generator_store,
        symbiotic_stake_store::{self, SlashResult, VaultSnapshot},
    },
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
    #[allow(unused)] ask_store: &Arc<RwLock<ask_store::LocalAskStore>>,
    rpc_url: &str,
    unhandled_logs: &Arc<RwLock<Vec<Log>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // let provider_http = Provider::<Http>::try_from(rpc_url).unwrap();
    // let client = Arc::new(provider_http.clone());

    if constants::SYMBIOTIC_STAKING_TOPICS_SKIP
        .get(&log.topics[0])
        .is_some()
    {
        log::debug!("standard topic to skip found, ignoring it");
        return Ok(());
    }

    // typed event not working
    // <bindings::symbiotic_staking::StakingManagerSetFilter>
    if let Ok(stake_manager_set_log) = symbiotic_staking.decode_event_raw(
        "StakingManagerSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("Staking Manager Set Logs: {:?}", stake_manager_set_log);
        return Ok(());
    }

    // typed event not working
    // <bindings::symbiotic_staking::ProofMarketplaceSetFilter>
    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "ProofMarketplaceSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("ProofMarketplaceSet Logs: {:?}", event_log);
        return Ok(());
    }

    // typed event not working
    // <bindings::symbiotic_staking::RewardDistributorSetFilter>
    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "RewardDistributorSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("RewardDistributorSet Logs: {:?}", event_log);
        return Ok(());
    }

    // typed event not working
    // <bindings::symbiotic_staking::FeeRewardTokenSetFilter>
    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "FeeRewardTokenSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("FeeRewardTokenSet Logs: {:?}", event_log);
        return Ok(());
    }

    let mut symbiotic_stake_store = { symbiotic_stake_store.write().await };

    //  typed event not working
    // <bindings::symbiotic_staking::StakeTokenAddedFilter>
    if let Ok(event_log) =
        symbiotic_staking.decode_event_raw("StakeTokenAdded", log.topics.clone(), log.data.clone())
    {
        log::debug!("StakeTokenAdded Logs: {:?}", event_log);
        let token = event_log.get(0).unwrap().clone().into_address().unwrap();
        let weight = event_log.get(1).unwrap().clone().into_uint().unwrap();

        // let token = event_log.token;
        // let weight = event_log.weight;

        log::debug!("Added token: {} with weight: {}", token, weight);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking
        .decode_event::<bindings::symbiotic_staking::StakeTokenRemovedFilter>(
            "StakeTokenRemoved",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("StakeTokenRemoved Logs: {:?}", event_log);
        // let token = event_log.get(0).unwrap().clone().into_address().unwrap();
        let token = event_log.token;

        log::debug!("Removed token: {}", token);
        symbiotic_stake_store.remove_lock_token(token);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking
        .decode_event::<bindings::symbiotic_staking::AmountToLockSetFilter>(
            "AmountToLockSet",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("AmountToLockSet Logs: {:?}", event_log);
        // let token = event_log.get(0).unwrap().clone().into_address().unwrap();
        // let amount = event_log.get(1).unwrap().clone().into_uint().unwrap();

        let token = event_log.token;
        let amount = event_log.amount;

        log::debug!("AmountToLockSet: Token: {}  Amount: {}", token, amount);
        symbiotic_stake_store.set_lock_token(token, amount);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking
        .decode_event::<bindings::symbiotic_staking::StakeTokenSelectionWeightSetFilter>(
        "StakeTokenSelectionWeightSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("StakeTokenSelectionWeightSet Logs: {:?}", event_log);
        return Ok(());
    }

    //typed event not working here
    // <bindings::symbiotic_staking::BaseTransmitterComissionRateSetFilter>
    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "BaseTransmitterComissionRateSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("BaseTransmitterComissionRateSet Logs: {:?}", event_log);
        return Ok(());
    }

    // type event not working here
    // <bindings::symbiotic_staking::SubmissionCooldownSetFilter>
    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "SubmissionCooldownSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("SubmissionCooldownSet Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking
        .decode_event::<bindings::symbiotic_staking::EnclaveImageAddedFilter>(
            "EnclaveImageAdded",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("EnclaveImageAdded Logs: {:?}", event_log);
        return Ok(());
    }

    // typed event not working
    // <bindings::symbiotic_staking::AttestationVerifierSetFilter>
    if let Ok(event_log) = symbiotic_staking.decode_event_raw(
        "AttestationVerifierSet",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("AttestationVerifierSet Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking
        .decode_event::<bindings::symbiotic_staking::EnclaveImageRemovedFilter>(
            "EnclaveImageRemoved",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("EnclaveImageRemoved Logs: {:?}", event_log);
        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking
        .decode_event::<bindings::symbiotic_staking::VaultSnapshotSubmittedFilter>(
        "VaultSnapshotSubmitted",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("VaultSnapshotSubmitted Logs: {:?}", event_log);
        let (
            transmitter,
            captured_timestamp,
            index,
            num_of_transactions,
            image_id,
            snapshot_data,
            proof,
        ) = {
            // let transmitter = event_log.get(0).unwrap().clone().into_address().unwrap();
            // let capture_time = event_log.get(1).unwrap().clone().into_uint().unwrap();
            // let index = event_log.get(2).unwrap().clone().into_uint().unwrap();
            // let num_of_transactions = event_log.get(3).unwrap().clone().into_uint().unwrap();
            // let image_id = event_log
            //     .get(4)
            //     .unwrap()
            //     .clone()
            //     .into_fixed_bytes()
            //     .unwrap();
            // let snapshot_data = event_log.get(5).unwrap().clone().into_bytes().unwrap();
            // let proof = event_log.get(6).unwrap().clone().into_bytes().unwrap();

            let transmitter = event_log.transmitter;
            let capture_time = event_log.capture_timestamp;
            let index = event_log.index;
            let num_of_transactions = event_log.num_of_txs;
            let image_id = event_log.image_id.into();
            let snapshot_data = event_log.vault_snapshot_data;
            let proof = event_log.proof;

            (
                transmitter,
                capture_time,
                index,
                num_of_transactions,
                image_id,
                snapshot_data,
                proof,
            )
        };

        symbiotic_stake_store.store_vault_snapshot(
            captured_timestamp,
            index,
            VaultSnapshot {
                transmitter,
                index,
                captured_timestamp,
                num_of_transactions,
                image_id,
                snapshot_data: snapshot_data.to_vec(),
                proof: proof.to_vec(),
            },
        );

        return Ok(());
    }

    if let Ok(event_log) = symbiotic_staking
        .decode_event::<bindings::symbiotic_staking::SlashResultSubmittedFilter>(
            "SlashResultSubmitted",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("SlashResultSubmitted Logs: {:?}", event_log);
        let (
            transmitter,
            captured_timestamp,
            index,
            num_of_transactions,
            image_id,
            slash_data,
            proof,
        ) = {
            // let transmitter = event_log.get(0).unwrap().clone().into_address().unwrap();
            // let capture_time = event_log.get(1).unwrap().clone().into_uint().unwrap();
            // let index = event_log.get(2).unwrap().clone().into_uint().unwrap();
            // let num_of_transactions = event_log.get(3).unwrap().clone().into_uint().unwrap();
            // let image_id = event_log
            //     .get(4)
            //     .unwrap()
            //     .clone()
            //     .into_fixed_bytes()
            //     .unwrap();
            // let slash_result = event_log.get(5).unwrap().clone().into_bytes().unwrap();
            // let proof = event_log.get(6).unwrap().clone().into_bytes().unwrap();

            let transmitter = event_log.transmitter;
            let capture_time = event_log.capture_timestamp;
            let index = event_log.index;
            let num_of_transactions = event_log.num_of_txs;
            let image_id = event_log.image_id.into();
            let slash_result = event_log.slash_result_data;
            let proof = event_log.proof;

            (
                transmitter,
                capture_time,
                index,
                num_of_transactions,
                image_id,
                slash_result,
                proof,
            )
        };

        symbiotic_stake_store.store_slash_result(
            captured_timestamp,
            index,
            SlashResult {
                transmitter,
                index,
                captured_timestamp,
                num_of_transactions,
                image_id,
                slash_data: slash_data.to_vec(),
                proof: proof.to_vec(),
            },
        );

        return Ok(());
    }

    let mut generator_store = { generator_store.write().await };

    if let Ok(symbiotic_complete_snapshot_log) =
        symbiotic_staking.decode_event::<bindings::symbiotic_staking::SnapshotConfirmedFilter>(
            "SnapshotConfirmed",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("Processing SnapshotConfirmed");

        // let transmitter = symbiotic_complete_snapshot_log
        //     .get(0)
        //     .unwrap()
        //     .clone()
        //     .into_address()
        //     .unwrap();
        // let confirmed_timestamp = symbiotic_complete_snapshot_log
        // .get(1)
        // .unwrap()
        // .clone()
        // .into_uint()
        // .unwrap();

        let transmitter = symbiotic_complete_snapshot_log.transmitter;
        let confirmed_timestamp = symbiotic_complete_snapshot_log.confirmed_timestamp;

        log::debug!("Transmitter: {:?}", transmitter);
        let vault_snapshots = symbiotic_stake_store.get_all_vault_snapshots(confirmed_timestamp);
        let slash_results = symbiotic_stake_store.get_all_slash_results(confirmed_timestamp);

        if slash_results.len() == 0 && vault_snapshots.len() == 0 {
            log::warn!("No snapshots found with timestamp: {}", confirmed_timestamp);
            return Ok(());
        }

        // when snapshot is confirmed, clear all the operator data before it.
        symbiotic_stake_store.clean_operators();

        for vault_snapshot in vault_snapshots.iter() {
            log::debug!("{:?}", vault_snapshot.clone().decode_vault_snapshot());
            for decoded_vault_snapshot in vault_snapshot.clone().decode_vault_snapshot().iter() {
                symbiotic_stake_store.note_down_stake(
                    &decoded_vault_snapshot.operator,
                    &decoded_vault_snapshot.stake_token,
                    &decoded_vault_snapshot.amount,
                );
            }
        }

        for slash_result in slash_results.iter() {
            log::debug!("{:?}", slash_result.clone().decode_slash_result());
        }

        let all_generators = generator_store.all_generators_address();

        let (known_tokens, _): (Vec<Address>, Vec<U256>) = {
            symbiotic_stake_store
                .tokens_to_lock
                .clone()
                .to_address_token_pair()
                .into_iter()
                .unzip()
        };
        for stake_token in known_tokens {
            for operator in all_generators.clone().into_iter() {
                // at this point symbiotic snapshot is already cleanup, and refreshed.
                let vault_snapshot_amount =
                    symbiotic_stake_store.get_latest_stake_info(&operator, &stake_token);

                log::debug!(
                    "operator:{:?}, snapshot token:{:?}, amount: {:?}",
                    &operator,
                    &stake_token,
                    vault_snapshot_amount.to_string()
                );

                let last_stored_staking_info = generator_store
                    .get_by_address(&operator)
                    .unwrap_or_default()
                    .total_symbiotic_stake
                    .get_balance(&stake_token);

                if vault_snapshot_amount.gt(&last_stored_staking_info) {
                    log::debug!(
                        "Operator: {:?} Token:{:?} existing stake {:?}",
                        &operator,
                        &stake_token,
                        last_stored_staking_info
                    );
                    log::debug!(
                        "Operator: {:?} Token:{:?} new stake {:?}",
                        &operator,
                        &stake_token,
                        vault_snapshot_amount
                    );
                    log::debug!(
                        "Operator: {:?} Token:{:?} recevied extra stake {:?}",
                        &operator,
                        &stake_token,
                        vault_snapshot_amount - last_stored_staking_info
                    );
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
                    log::debug!(
                        "Operator: {:?} Token:{:?} existing stake {:?}",
                        &operator,
                        &stake_token,
                        last_stored_staking_info
                    );
                    log::debug!(
                        "Operator: {:?} Token:{:?} new stake {:?}",
                        &operator,
                        &stake_token,
                        vault_snapshot_amount
                    );
                    log::debug!(
                        "Operator: {:?} Token:{:?} removed stake {:?}",
                        &operator,
                        &stake_token,
                        last_stored_staking_info - vault_snapshot_amount
                    );

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
        let address = stake_lock_logs.prover;
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
        let address = stake_lock_logs.prover;
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
        .decode_event::<bindings::symbiotic_staking::TaskSlashedFilter>(
            "TaskSlashed",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::warn!("Job/Stake Slashed: {:?}", stake_slash_logs);
        let address = stake_slash_logs.prover;
        let stake_slashed = stake_slash_logs.amount;
        let token_address = stake_slash_logs.token;

        let block_l2: U256 = log.block_number.unwrap().as_u64().into();
        let block_l1: U256 = get_l1_block_from_l2_block(rpc_url, block_l2)
            .await
            .unwrap_or_default();

        let stake_slashed = if cfg!(feature = "override_symbiotic_slashing_to_zero") {
            log::warn!("Symbiotic slashing is disabled, however an 0 entry is still noted");
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

        #[cfg(not(feature = "generate_dummy_slash_logs"))]
        {
            use crate::utility::get_timestamp_from_l2block_number;

            let ask_id = stake_slash_logs.bid_id; //job _id and ask_id are same
            let ask = { ask_store.read().await.get_by_ask_id(&ask_id).unwrap() };

            let market_id = ask.market_id;
            let reward = ask.reward;
            let deadline = ask.deadline;

            let (symbiotic_slashing_tokens, symbiotic_slashings): (Vec<Address>, Vec<U256>) =
                { (vec![token_address], vec![stake_slashed]) };

            // only notes the slashing entry, doesn't update stake
            generator_store.note_entry_slashing(
                &address,
                &ask_id,
                &market_id,
                vec![],
                vec![],
                symbiotic_slashing_tokens,
                symbiotic_slashings,
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
        let mut unhandled_logs = { unhandled_logs.write().await };
        unhandled_logs.push(log.clone());
        return Ok(());
    }

    log::error!("unhandled log in symbiotic staking {:?}", log);
    return Err("Unhandled log in symbiotic staking".into());
}
