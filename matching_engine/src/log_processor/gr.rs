use ethers::prelude::{k256::ecdsa::SigningKey, *};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{generator::*, log_processor::constants};

pub async fn process_generator_registry_logs(
    logs: Vec<Log>,
    genertor_registry: bindings::generator_registry::GeneratorRegistry<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
    generator_store: &Arc<Mutex<GeneratorStore>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut generator_store = generator_store.lock().await;
    for log in &logs {
        if constants::TOPICS_TO_SKIP.get(&log.topics[0]).is_some() {
            log::warn!("standard topic to skip found, ignoring it");
            continue;
        }

        if let Ok(parsed_registered_generator_log) = genertor_registry.decode_event_raw(
            "RegisteredGenerator",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::info!(
                "Registered generator {:?} to store",
                parsed_registered_generator_log
            );

            let generator_bytes = parsed_registered_generator_log.get(0).unwrap();
            let compute_bytes = parsed_registered_generator_log.get(1).unwrap();
            let stake_bytes = parsed_registered_generator_log.get(2).unwrap();
            let address = generator_bytes.clone().into_address().unwrap();
            let compute = compute_bytes.clone().into_uint().unwrap();
            let stake = stake_bytes.clone().into_uint().unwrap();

            let generator_data = genertor_registry
                .generator_registry(address)
                .call()
                .await
                .unwrap();

            let generator = Generator {
                address,
                reward_address: generator_data.0,
                total_stake: stake,
                sum_of_compute_allocations: 0.into(),
                compute_consumed: 0.into(),
                stake_locked: 0.into(),
                active_market_places: 0.into(),
                declared_compute: compute,
                intended_stake_util: 1000000000000000000_i64.into(),
                intended_compute_util: 1000000000000000000_i64.into(),
                generator_data: Some(generator_data.9),
            };

            generator_store.insert(generator.clone());
            log::debug!("Generator registered {:?}", address.clone());

            continue;
        }

        if let Ok(parsed_deregistered_generator_log) = genertor_registry.decode_event_raw(
            "DeregisteredGenerator",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::debug!(
                "Deregistering Generator: {:?}",
                parsed_deregistered_generator_log.get(0).unwrap()
            );
            let address = parsed_deregistered_generator_log
                .get(0)
                .unwrap()
                .clone()
                .into_address()
                .unwrap();

            generator_store.remove_by_address(&address);
            continue;
        }

        if let Ok(generator_reward_address_change_log) = genertor_registry.decode_event_raw(
            "ChangedGeneratorRewardAddress",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::debug!(
                "Generator: {:?}, new reward address: {:?}",
                generator_reward_address_change_log.get(0).unwrap(),
                generator_reward_address_change_log.get(1).unwrap()
            );

            let address = generator_reward_address_change_log
                .get(0)
                .unwrap()
                .clone()
                .into_address()
                .unwrap();
            let reward_address = generator_reward_address_change_log
                .get(1)
                .unwrap()
                .clone()
                .into_address()
                .unwrap();

            generator_store.update_reward_address(&address, reward_address);
            continue;
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

            let total_stake = generator_store
                .get_by_address(&address)
                .unwrap()
                .total_stake;

            let generator_market = GeneratorInfoPerMarket {
                address: parsed_joined_market_place_log.generator,
                market_id: parsed_joined_market_place_log.market_id,
                total_stake,
                compute_required_per_request: parsed_joined_market_place_log.compute_allocation,
                proof_generation_cost: generator_market_data.2,
                proposed_time: generator_market_data.3,
                active_requests: 0.into(),
                proofs_submitted: 0.into(),
                state: Some(GeneratorState::Joined),
            };
            generator_store.insert_markets(generator_market);
            continue;
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

            generator_store.update_state(&address, &market_id, GeneratorState::RequestedForExit);
            continue;
        }

        if let Ok(parsed_left_market_place_log) =
            genertor_registry.decode_event::<bindings::generator_registry::LeftMarketplaceFilter>(
                "LeftMarketplace",
                log.topics.clone(),
                log.data.clone(),
            )
        {
            log::debug!(
                "Generator: {:?}, left Market_ID: {:?}",
                parsed_left_market_place_log.generator,
                parsed_left_market_place_log.market_id
            );
            let address = parsed_left_market_place_log.generator;
            let market_id = parsed_left_market_place_log.market_id;

            generator_store.remove_by_address_and_market(&address, &market_id);
            continue;
        }

        if let Ok(add_ivs_key_log) = genertor_registry
            .decode_event::<bindings::generator_registry::AddIvsKeyFilter>(
                "AddIvsKey",
                log.topics.clone(),
                log.data.clone(),
            )
        {
            log::warn!(
                "Ivs key signer: {:?}, market id: {:?}",
                add_ivs_key_log.signer,
                add_ivs_key_log.market_id
            );
            continue;
        }

        if let Ok(added_stake_log) = genertor_registry
            .decode_event::<bindings::generator_registry::AddedStakeFilter>(
                "AddedStake",
                log.topics.clone(),
                log.data.clone(),
            )
        {
            log::debug!("Added stake to Generator: {:?}", added_stake_log.generator);
            let address = added_stake_log.generator;
            let amount = added_stake_log.amount;

            generator_store.add_extra_stake(&address, &amount);

            continue;
        }

        if let Ok(request_stake_decrease_log) =
            genertor_registry
                .decode_event::<bindings::generator_registry::RequestStakeDecreaseFilter>(
                    "RequestStakeDecrease",
                    log.topics.clone(),
                    log.data.clone(),
                )
        {
            log::debug!(
                "Request stake decrease for Generator: {:?}",
                request_stake_decrease_log.generator
            );

            let address = request_stake_decrease_log.generator;
            let new_utilization = request_stake_decrease_log.intended_utilization;

            generator_store.update_intended_stake_util(&address, new_utilization);
            continue;
        }

        if let Ok(remove_stake_log) = genertor_registry
            .decode_event::<bindings::generator_registry::RemovedStakeFilter>(
            "RemovedStake",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::debug!(
                "Remove stake for Generator: {:?}",
                remove_stake_log.generator
            );

            let address = remove_stake_log.generator;
            let amount = remove_stake_log.amount;

            generator_store.remove_stake(&address, &amount);
            generator_store.update_intended_stake_util(&address, 1000000000000000000_i64.into());

            continue;
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
            continue;
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

            generator_store.update_intended_compute_util(&address, new_utilization);
            continue;
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
            generator_store.update_intended_compute_util(&address, 1000000000000000000_i64.into());
            continue;
        }

        if let Ok(upgraded_logs) =
            genertor_registry.decode_event_raw("Upgraded", log.topics.clone(), log.data.clone())
        {
            log::debug!("Upgraded: {:?}", upgraded_logs);
            continue;
        }

        if let Ok(initialized_logs) =
            genertor_registry.decode_event_raw("Initialized", log.topics.clone(), log.data.clone())
        {
            log::warn!("Version: {:?}", initialized_logs);
            continue;
        }

        if let Ok(role_granted_logs) = genertor_registry
            .decode_event::<bindings::generator_registry::RoleGrantedFilter>(
            "RoleGranted",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::warn!(
                "Role: {:?}, Sender: {:?}",
                hex::encode(role_granted_logs.role),
                role_granted_logs.sender
            );
            continue;
        }

        if let Ok(role_revoked_logs) = genertor_registry
            .decode_event::<bindings::generator_registry::RoleRevokedFilter>(
            "RoleRevoked",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::warn!(
                "Role: {:?}, Sender: {:?}",
                hex::encode(role_revoked_logs.role),
                role_revoked_logs.sender
            );
            continue;
        }

        if let Ok(role_admin_changed_logs) = genertor_registry
            .decode_event::<bindings::generator_registry::RoleAdminChangedFilter>(
            "RoleAdminChanged",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::warn!(
                "New Admin: {:?}, Old Admin: {:?}, Role: {:?}",
                role_admin_changed_logs.new_admin_role,
                role_admin_changed_logs.previous_admin_role,
                role_admin_changed_logs.role
            );
            continue;
        }

        if let Ok(stake_lock_logs) = genertor_registry
            .decode_event::<bindings::generator_registry::StakeLockImposedFilter>(
            "StakeLockImposed",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::warn!("Stake Lock Imposed: {:?}", stake_lock_logs);
            continue;
        }

        if let Ok(compute_lock_logs) = genertor_registry
            .decode_event::<bindings::generator_registry::ComputeLockImposedFilter>(
            "ComputeLockImposed",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::warn!("Compute Lock Imposed: {:?}", compute_lock_logs);
            continue;
        }

        if let Ok(stake_lock_logs) = genertor_registry
            .decode_event::<bindings::generator_registry::StakeLockReleasedFilter>(
            "StakeLockReleased",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::warn!("Stake Lock Released: {:?}", stake_lock_logs);
            continue;
        }

        if let Ok(compute_lock_logs) = genertor_registry
            .decode_event::<bindings::generator_registry::ComputeLockReleasedFilter>(
            "ComputeLockReleased",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::warn!("Compute Lock Released: {:?}", compute_lock_logs);
            continue;
        }

        log::warn!("unhandled log in generator registry {:?}", log);
        return Err("Unhandled log in generator registry".into());
    }
    Ok(())
}
