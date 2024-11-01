use ethers::prelude::{k256::ecdsa::SigningKey, *};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::generator_lib::*;
use crate::log_processor::constants;
use crate::utility::{tx_to_string, TokenTracker, TEST_TOKEN_ADDRESS_ONE, TEST_TOKEN_ADDRESS_TWO};

pub async fn process_generator_registry_logs(
    log: &Log,
    genertor_registry: &bindings::generator_registry::GeneratorRegistry<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
    symbiotic_staking: &bindings::symbiotic_staking::SymbioticStaking<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
    generator_store: &Arc<RwLock<generator_store::GeneratorStore>>,
    symbiotic_stake_store: &Arc<RwLock<symbiotic_stake_store::SymbioticStakeStore>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if constants::GENERATOR_REGISTRY_TOPICS_SKIP
        .get(&log.topics[0])
        .is_some()
    {
        log::debug!("standard topic to skip found, ignoring it");
        return Ok(());
    }

    if let Ok(add_ivs_key_log) = genertor_registry
        .decode_event::<bindings::generator_registry::AddIvsKeyFilter>(
            "AddIvsKey",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!(
            "Ivs key signer: {:?}, market id: {:?}",
            add_ivs_key_log.signer,
            add_ivs_key_log.market_id
        );
        return Ok(());
    }

    // using here, as above events don' require any write lock
    let mut generator_store = { generator_store.write().await };

    if let Ok(parsed_registered_generator_log) =
        genertor_registry.decode_event::<bindings::generator_registry::RegisteredGeneratorFilter>(
            "RegisteredGenerator",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::info!(
            "Registered generator {:?} to store",
            parsed_registered_generator_log
        );

        let address = parsed_registered_generator_log.generator.into();
        let compute = parsed_registered_generator_log.initial_compute.into();

        log::warn!("During registration initial stake is assumed to be 0");

        let generator_data = genertor_registry
            .generator_registry(address)
            .call()
            .await
            .unwrap();

        let generator = generator_store::Generator {
            address,
            reward_address: generator_data.0,
            total_stake: TokenTracker::new(),
            sum_of_compute_allocations: 0.into(),
            compute_consumed: 0.into(),
            stake_locked: TokenTracker::new(),
            active_market_places: 0.into(),
            declared_compute: compute,
            intended_stake_util: 1000000000000000000_i64.into(),
            intended_compute_util: 1000000000000000000_i64.into(),
            generator_data: generator_data.6,
        };

        generator_store.insert(generator.clone());
        log::debug!("Generator registered {:?}", address.clone());

        return Ok(());
    }

    if let Ok(parsed_deregistered_generator_log) = genertor_registry.decode_event_raw(
        "DeregisteredGenerator",
        log.topics.clone(),
        log.data.clone(),
    ) {
        let generator_address = {
            let generator_address_token = parsed_deregistered_generator_log.first().unwrap();
            let generator_address = generator_address_token.clone().into_address().unwrap();
            generator_address
        };

        log::debug!("Deregistering Generator: {:?}", generator_address);
        let address = generator_address.into();

        generator_store.remove_by_address(&address);
        return Ok(());
    }

    if let Ok(generator_reward_address_change_log) =
        genertor_registry
            .decode_event::<bindings::generator_registry::ChangedGeneratorRewardAddressFilter>(
                "ChangedGeneratorRewardAddress",
                log.topics.clone(),
                log.data.clone(),
            )
    {
        log::debug!(
            "Generator: {:?}, new reward address: {:?}",
            generator_reward_address_change_log.generator,
            generator_reward_address_change_log.new_reward_address
        );

        let address = generator_reward_address_change_log.generator.into();
        let reward_address = generator_reward_address_change_log
            .new_reward_address
            .into();

        generator_store.update_reward_address(&address, reward_address);
        return Ok(());
    }

    if let Ok(parsed_joined_market_place_log) =
        genertor_registry.decode_event::<bindings::generator_registry::JoinedMarketplaceFilter>(
            "JoinedMarketplace",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::info!(
            "Generator: {:?}, joined Market_ID {:?}",
            parsed_joined_market_place_log.generator,
            parsed_joined_market_place_log.market_id
        );
        let address = parsed_joined_market_place_log.generator;
        let market_id = parsed_joined_market_place_log.market_id;

        let generator_market_data = genertor_registry
            .generator_info_per_market(address, market_id)
            .call()
            .await
            .unwrap();

        let generator_market = generator_store::GeneratorInfoPerMarket {
            address: parsed_joined_market_place_log.generator,
            market_id: parsed_joined_market_place_log.market_id,
            compute_required_per_request: parsed_joined_market_place_log.compute_allocation,
            proof_generation_cost: generator_market_data.2,
            proposed_time: generator_market_data.3,
            active_requests: 0.into(),
            proofs_submitted: 0.into(),
            proofs_slashed: 0.into(),
            state: Some(generator_state::GeneratorState::Joined),
        };
        generator_store.insert_markets(generator_market);
        return Ok(());
    }

    if let Ok(parsed_requested_for_exit_log) =
        genertor_registry
            .decode_event::<bindings::generator_registry::RequestExitMarketplaceFilter>(
                "RequestExitMarketplace",
                log.topics.clone(),
                log.data.clone(),
            )
    {
        log::debug!(
            "Generator: {:?}, request for exit from Market_ID: {:?}",
            parsed_requested_for_exit_log.generator,
            parsed_requested_for_exit_log.market_id
        );

        let address = parsed_requested_for_exit_log.generator;
        let market_id = parsed_requested_for_exit_log.market_id;

        generator_store.update_state(
            &address,
            &market_id,
            generator_state::GeneratorState::RequestedForExit,
        );
        return Ok(());
    }

    if let Ok(parsed_left_market_place_log) = genertor_registry
        .decode_event::<bindings::generator_registry::LeftMarketplaceFilter>(
        "LeftMarketplace",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!(
            "Generator: {:?}, left Market_ID: {:?}",
            parsed_left_market_place_log.generator,
            parsed_left_market_place_log.market_id
        );
        let address = parsed_left_market_place_log.generator;
        let market_id = parsed_left_market_place_log.market_id;

        generator_store.remove_by_address_and_market(&address, &market_id);
        return Ok(());
    }

    if let Ok(added_stake_log) = genertor_registry
        .decode_event::<bindings::generator_registry::AddedStakeFilter>(
            "AddedStake",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!(
            "Added stake to Generator: {:?}",
            added_stake_log.generator_address
        );
        let address = added_stake_log.generator_address;
        let amount = added_stake_log.amount;
        let token_address = added_stake_log.token;

        generator_store.add_extra_stake(
            &address,
            &token_address,
            &amount,
            log.block_number.unwrap(),
            log.transaction_index.unwrap(),
            log.log_index.unwrap(),
            tx_to_string(&log.transaction_hash.unwrap()),
            delegation::Source::Native,
        );

        return Ok(());
    }

    if let Ok(request_stake_decrease_log) = genertor_registry
        .decode_event::<bindings::generator_registry::RequestStakeDecreaseFilter>(
        "RequestStakeDecrease",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!(
            "Request stake decrease for Generator: {:?}",
            request_stake_decrease_log.generator_address
        );

        let address = request_stake_decrease_log.generator_address;

        log::warn!("pausing all assignments across all markets");
        log::warn!("will be unpaused once the request if fully withdrawn");

        generator_store.pause_assignments_across_all_markets(&address);

        log::warn!("Setting new utilization to same value");
        let new_utilization = 1000000000000000000_i64.into();
        generator_store.update_intended_stake_util(&address, new_utilization);
        return Ok(());
    }

    if let Ok(remove_stake_log) = genertor_registry
        .decode_event::<bindings::generator_registry::RemovedStakeFilter>(
            "RemovedStake",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!(
            "Remove stake for Generator: {:?}",
            remove_stake_log.generator_address
        );

        let address = remove_stake_log.generator_address;
        let amount = remove_stake_log.amount;
        let token_address = remove_stake_log.token;

        generator_store.remove_stake(
            &address,
            &token_address,
            &amount,
            log.block_number.unwrap(),
            log.transaction_index.unwrap(),
            log.log_index.unwrap(),
            tx_to_string(&log.transaction_hash.unwrap()),
            delegation::Operation::UnDelegate,
            delegation::Source::Native,
        );
        generator_store.resume_assignments_accross_all_markets(&address);
        generator_store.update_intended_stake_util(&address, 1000000000000000000_i64.into());

        return Ok(());
    }

    if let Ok(increase_compute_log) = genertor_registry
        .decode_event::<bindings::generator_registry::IncreasedComputeFilter>(
        "IncreasedCompute",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!(
            "Increase compute for Generator: {:?}",
            increase_compute_log.generator
        );
        let address = increase_compute_log.generator;
        let compute = increase_compute_log.compute;

        generator_store.add_extra_compute(&address, compute);
        return Ok(());
    }

    if let Ok(request_compute_decrease_log) =
        genertor_registry
            .decode_event::<bindings::generator_registry::RequestComputeDecreaseFilter>(
                "RequestComputeDecrease",
                log.topics.clone(),
                log.data.clone(),
            )
    {
        log::info!(
            "Request compute decrease for Generator: {:?}",
            request_compute_decrease_log.generator
        );

        let address = request_compute_decrease_log.generator;
        let new_utilization = request_compute_decrease_log.intended_utilization;

        generator_store.pause_assignments_across_all_markets(&address);
        generator_store.update_intended_compute_util(&address, new_utilization);
        return Ok(());
    }

    if let Ok(decrease_compute_log) = genertor_registry
        .decode_event::<bindings::generator_registry::DecreaseComputeFilter>(
        "DecreaseCompute",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::info!(
            "Compute decrease for Generator: {:?} to : {:?}",
            decrease_compute_log.generator,
            decrease_compute_log.compute
        );

        let address = decrease_compute_log.generator;
        let compute = decrease_compute_log.compute;

        generator_store.remove_compute(&address, compute);
        generator_store.resume_assignments_accross_all_markets(&address);
        generator_store.update_intended_compute_util(&address, 1000000000000000000_i64.into());
        return Ok(());
    }

    if let Ok(stake_lock_logs) = genertor_registry
        .decode_event::<bindings::generator_registry::StakeLockImposedFilter>(
        "StakeLockImposed",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("Stake Lock Imposed: {:?}", stake_lock_logs);
        let address = stake_lock_logs.generator_address;
        let stake_locked = stake_lock_logs.stake;
        let token_address = stake_lock_logs.token;

        generator_store.update_on_stake_locked(&address, &token_address, stake_locked);
        return Ok(());
    }

    if let Ok(compute_lock_logs) = genertor_registry
        .decode_event::<bindings::generator_registry::ComputeLockImposedFilter>(
        "ComputeLockImposed",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("Compute Lock Imposed: {:?}", compute_lock_logs);
        let address = compute_lock_logs.generator;
        let compute_locked = compute_lock_logs.compute;
        generator_store.update_on_compute_locked(&address, compute_locked);
        return Ok(());
    }

    if let Ok(stake_lock_logs) = genertor_registry
        .decode_event::<bindings::generator_registry::StakeLockReleasedFilter>(
        "StakeLockReleased",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("Stake Lock Released: {:?}", stake_lock_logs);
        let address = stake_lock_logs.generator_address;
        let stake_released = stake_lock_logs.stake;
        let token_address = stake_lock_logs.token;
        generator_store.update_on_stake_released(&address, &token_address, stake_released);
        return Ok(());
    }

    if let Ok(compute_lock_logs) = genertor_registry
        .decode_event::<bindings::generator_registry::ComputeLockReleasedFilter>(
        "ComputeLockReleased",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("Compute Lock Released: {:?}", compute_lock_logs);
        let address = compute_lock_logs.generator;
        let compute_released = compute_lock_logs.compute;
        generator_store.update_on_compute_released(&address, compute_released);
        return Ok(());
    }

    if let Ok(stake_slash_logs) = genertor_registry
        .decode_event::<bindings::generator_registry::StakeSlashedFilter>(
            "StakeSlashed",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::warn!("Stake Slashed: {:?}", stake_slash_logs);
        let address = stake_slash_logs.generator_address;
        let stake_slashed = stake_slash_logs.stake;
        let token_address = stake_slash_logs.token;

        generator_store.remove_stake(
            &address,
            &token_address,
            &stake_slashed,
            log.block_number.unwrap(),
            log.transaction_index.unwrap(),
            log.log_index.unwrap(),
            tx_to_string(&log.transaction_hash.unwrap()),
            delegation::Operation::Slash,
            delegation::Source::Native,
        );
        return Ok(());
    }

    if let Ok(symbiotic_complete_snapshot_log) =
        symbiotic_staking
            .decode_event::<bindings::generator_registry::SymbioticCompleteSnapshotFilter>(
                "SymbioticCompleteSnapshot",
                log.topics.clone(),
                log.data.clone(),
            )
    {
        let mut symbiotic_stake_store = { symbiotic_stake_store.write().await };

        let captured_timestamp = symbiotic_complete_snapshot_log.capture_timestamp;
        let known_tokens: Vec<Address> = vec![
            TEST_TOKEN_ADDRESS_ONE.clone(),
            TEST_TOKEN_ADDRESS_TWO.clone(),
        ];
        let all_generators = generator_store.all_generators_address();

        for stake_token in known_tokens {
            for operator in all_generators.clone().into_iter() {
                // if this fails, system breaks. TODO
                log::error!("latest vault snapshot amount is being fetched as place holder");
                log::error!(
                    "matches won't work unless captured_timestamp: {} is used",
                    captured_timestamp
                );
                log::error!("ask team to make Symbiotic.operatorStakeAmount public");

                let vault_snapshot_amount = symbiotic_staking
                    .get_operator_stake_amount(stake_token, operator)
                    .call()
                    .await
                    .unwrap();

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

    log::error!("unhandled log in generator registry {:?}", log);
    return Err("Unhandled log in generator registry".into());
}
