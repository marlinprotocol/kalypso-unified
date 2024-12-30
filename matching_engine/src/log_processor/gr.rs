use ethers::prelude::{k256::ecdsa::SigningKey, *};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::generator_lib::*;
use crate::log_processor::constants;
use crate::utility::TokenTracker;

pub async fn process_generator_registry_logs(
    log: &Log,
    genertor_registry: &bindings::generator_registry::GeneratorRegistry<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
    generator_store: &Arc<RwLock<generator_store::GeneratorStore>>,
    rpc_url: &str,
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

        log::debug!("During registration initial stake is assumed to be 0");

        let generator_data = genertor_registry
            .generator_registry(address)
            .call()
            .await
            .unwrap();

        let generator = generator_store::Generator {
            address,
            reward_address: generator_data.0,
            total_native_stake: TokenTracker::new(),
            total_symbiotic_stake: TokenTracker::new(),
            sum_of_compute_allocations: 0.into(),
            compute_consumed: 0.into(),
            native_stake_locked: TokenTracker::new(),
            symbiotic_stake_locked: TokenTracker::new(),
            active_market_places: 0.into(),
            declared_compute: compute,
            intended_stake_util: 1000000000000000000_i64.into(),
            intended_compute_util: 1000000000000000000_i64.into(),
            generator_data: generator_data.6,
            active: true,
        };

        generator_store.register_generator(generator.clone());
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
        generator_store.register_generator_in_market(generator_market);
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

        log::debug!("Add Stake is now handled in native staking");
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

        log::debug!(
            "RequestStakeDecrease is not processed using native_staking::StakeWithdrawalRequested"
        );
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

        log::debug!("Request stake decrese in no processed in native_stake::StakeWithdrawn");

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
        log::debug!(
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
        log::debug!(
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
        log::debug!("Stake Lock Imposed is now Handled in native_staking::StakeLocked and symbiotic_staking::StakeLocked separately");
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
        log::debug!("Stake Lock Released in native_staking::StakeUnlocked and symbiotic_stake::StakeUnlocked separately");
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
        log::debug!("Stake Slashed: {:?}", stake_slash_logs);
        log::debug!("Stake slash is now handled in ns::JobSlashed and ss::JobSlashed separately");
        return Ok(());
    }

    if let Ok(symbiotic_complete_snapshot_log) = genertor_registry.decode_event_raw(
        "SymbioticCompleteSnapshot",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!(
            "Processing SymbioticCompleteSnapshot: {:?}",
            symbiotic_complete_snapshot_log
        );
        log::debug!(
            "SymbioticCompleteSnapshot is now processed using symbiotic::SnapshotConfirmed"
        );
        return Ok(());
    }

    // ------ custom patches being handled here --------------------- //
    let provider_http = Provider::<Http>::try_from(rpc_url).unwrap();

    let client = Arc::new(provider_http.clone());

    let gr_update_generator_data_patch =
        binding_patches::GeneratorRegistryPatch::new(genertor_registry.address(), client);

    if let Ok(update_generator_metadata_log) = gr_update_generator_data_patch.decode_event_raw(
        "GeneratorDataUpdated",
        log.topics.clone(),
        log.data.clone(),
    ) {
        let generator_bytes = update_generator_metadata_log.get(0).unwrap();
        let generator_address = generator_bytes.clone().into_address().unwrap();

        let generator_bytes = update_generator_metadata_log.get(1).unwrap();
        let generator_meta_data = generator_bytes.clone().into_bytes().unwrap().to_vec();

        generator_store.update_generator_metadata(generator_address, generator_meta_data.into());
        return Ok(());
    }

    if let Ok(intend_to_reduce_stake_logs) = gr_update_generator_data_patch.decode_event_raw(
        "IntendToReduceStake",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!(
            "Intend to reduce stake logs {:?}",
            intend_to_reduce_stake_logs
        );

        log::debug!("IntendToReduceStake is not processed here");
        return Ok(());
    }

    if cfg!(feature = "skip_unknown_events") {
        log::warn!("{:?}", log);
        log::warn!("Unknown event noted and skipped");
        return Ok(());
    }

    log::error!("unhandled log in generator registry {:?}", log);
    return Err("Unhandled log in generator registry".into());
}
