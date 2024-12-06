use crate::ask_lib::ask::LocalAsk;
use crate::ask_lib::ask_status::AskState;
use crate::ask_lib::ask_store::LocalAskStore;
use crate::costs::CostStore;
use crate::utility::get_l1_block_from_l2_block;
use crate::utility::get_timestamp_from_l2block_number;
use crate::utility::tx_to_string;
use ethers::prelude::{k256::ecdsa::SigningKey, *};
use im::HashSet;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::generator_lib::*;
use crate::market_metadata::*;
use kalypso_helper::secret_inputs_helpers;

use bindings::proof_marketplace as pmp;

use super::constants;

#[allow(clippy::too_many_arguments)]
pub async fn process_proof_market_place_logs(
    log: &Log,
    proof_market_place: &pmp::ProofMarketplace<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
    local_ask_store: &Arc<RwLock<LocalAskStore>>,
    generator_store: &Arc<RwLock<generator_store::GeneratorStore>>,
    market_store: &Arc<RwLock<MarketMetadataStore>>,
    cost_store: &Arc<RwLock<CostStore>>,
    _native_store: &Arc<RwLock<native_stake_store::NativeStakingStore>>,
    _symbiotic_stake_store: &Arc<RwLock<symbiotic_stake_store::SymbioticStakeStore>>,
    matching_engine_key: &[u8],
    matchin_engine_slave_keys: &Vec<Vec<u8>>,
    rpc_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if constants::PROOF_MARKET_TOPICS_SKIP
        .get(&log.topics[0])
        .is_some()
    {
        log::debug!("standard topic to skip found, ignoring it");
        return Ok(());
    }

    if let Ok(min_proving_time_log) = proof_market_place
        .decode_event::<pmp::UpdateMinProvingTimeFilter>(
            "UpdateMinProvingTime",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("Min Proving Time as market creation param changed");
        log::debug!(
            "Secret: {}, New Min Time: {}",
            min_proving_time_log.secret_type,
            min_proving_time_log.new_proving_time
        );
        return Ok(());
    }

    if let Ok(upgraded_logs) = proof_market_place.decode_event::<pmp::UpgradedFilter>(
        "Upgraded",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("Implementation {:?}", upgraded_logs.implementation);
        return Ok(());
    }

    if let Ok(paused_logs) = proof_market_place.decode_event::<pmp::PausedFilter>(
        "Paused",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("Paused {:?}", paused_logs.account);
        return Ok(());
    }

    if let Ok(unpaused_logs) = proof_market_place.decode_event::<pmp::UnpausedFilter>(
        "Unpaused",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("Unpaused {:?}", unpaused_logs.account);
        return Ok(());
    }

    if let Ok(parsed_ask_created_log) = proof_market_place.decode_event::<pmp::AskCreatedFilter>(
        "AskCreated",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("{:?}", parsed_ask_created_log);
        let mut local_ask_store = { local_ask_store.write().await };
        let ask_data: (pmp::Ask, _, _, _) = proof_market_place
            .list_of_ask(parsed_ask_created_log.ask_id)
            .call()
            .await
            .unwrap();

        let created_on: U256 = log.block_number.unwrap().as_u64().into();
        let created_on_l1: U256 = get_l1_block_from_l2_block(rpc_url, created_on)
            .await
            .unwrap_or_default();

        let mut ask_to_store = LocalAsk {
            ask_id: parsed_ask_created_log.ask_id,
            market_id: ask_data.0.market_id,
            reward: ask_data.0.reward,
            expiry: ask_data.0.expiry,
            deadline: ask_data.0.deadline,
            prover_refund_address: ask_data.0.refund_address,
            prover_data: ask_data.0.prover_data,
            has_private_inputs: parsed_ask_created_log.has_private_inputs,
            secret_data: None,
            secret_acl: None,
            state: Some(AskState::Create),
            generator: None,
            invalid_secret_flag: false,
            created_on: created_on_l1,
            time_requested_for_proof_generation: ask_data.0.time_taken_for_proof_generation,
        };

        if parsed_ask_created_log.has_private_inputs {
            ask_to_store.secret_data = Some(parsed_ask_created_log.secret_data);
            ask_to_store.secret_acl = Some(parsed_ask_created_log.acl);
            let secret_inputs = &ask_to_store.clone().secret_data.unwrap();
            let acl = &ask_to_store.clone().secret_acl.unwrap();

            let mut decrypted_secret_data = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
                secret_inputs,
                acl,
                matching_engine_key,
                Some(ask_to_store.market_id),
            );

            if decrypted_secret_data.is_err() {
                for slave_key in matchin_engine_slave_keys {
                    decrypted_secret_data = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
                        secret_inputs,
                        acl,
                        slave_key,
                        Some(ask_to_store.market_id),
                    );

                    if decrypted_secret_data.is_ok() {
                        break;
                    }
                }
            }

            if decrypted_secret_data.is_ok() {
                ask_to_store.invalid_secret_flag = true;
                log::debug!(
                    "Stored private ask with AskId {:?} to store",
                    parsed_ask_created_log.ask_id
                );
            } else {
                ask_to_store.state = Some(AskState::InvalidSecret).to_owned();
                log::warn!(
                    "Stored ask with AskId {:?} to store but couldn't infer private inputs",
                    parsed_ask_created_log.ask_id
                );
            }
            local_ask_store.insert(ask_to_store.to_owned());
        } else {
            ask_to_store.invalid_secret_flag = true;
            log::debug!(
                "Stored {:?} ask to store, Market: {}",
                parsed_ask_created_log.ask_id,
                &ask_to_store.market_id
            );
            local_ask_store.insert(ask_to_store.to_owned());
        }

        local_ask_store.update_job_created_on_timestamp(
            &parsed_ask_created_log.ask_id,
            get_timestamp_from_l2block_number(rpc_url, &created_on)
                .await
                .unwrap_or_default(),
        );
        return Ok(());
    }

    if let Ok(parsed_task_created_log) = proof_market_place.decode_event::<pmp::TaskCreatedFilter>(
        "TaskCreated",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!("{:?}", parsed_task_created_log);
        let mut local_ask_store = { local_ask_store.write().await };

        let ask_id = parsed_task_created_log.ask_id;
        let generator = parsed_task_created_log.generator;
        let new_acl = parsed_task_created_log.new_acl;

        local_ask_store.update_ask_generator(&ask_id, Some(generator));
        local_ask_store.update_ask_acl(&ask_id, Some(new_acl));

        local_ask_store.modify_state(&ask_id, AskState::Assigned);

        local_ask_store.update_job_matched_on_timestamp(
            &ask_id,
            get_timestamp_from_l2block_number(rpc_url, &log.block_number.unwrap().as_u64().into())
                .await
                .unwrap_or_default(),
        );

        let market_id = { local_ask_store.get_by_ask_id(&ask_id).unwrap().market_id };

        generator_store
            .write()
            .await
            .update_on_assigned_task(&generator.into(), &market_id);

        return Ok(());
    }

    if let Ok(parsed_proof_created_log) = proof_market_place
        .decode_event::<pmp::ProofCreatedFilter>(
            "ProofCreated",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("{:?}", parsed_proof_created_log);
        let mut local_ask_store = { local_ask_store.write().await };

        let ask_id = parsed_proof_created_log.ask_id;
        let proof = parsed_proof_created_log.proof;

        let proof_cycle_completed_on: U256 = log.block_number.unwrap().as_u64().into();
        let proof_cycle_completed_on_l1: U256 =
            get_l1_block_from_l2_block(rpc_url, proof_cycle_completed_on)
                .await
                .unwrap_or_default();

        local_ask_store.update_proof_proof_cycle_completed_on(&ask_id, proof_cycle_completed_on_l1);
        local_ask_store.modify_state(&ask_id, AskState::Complete);

        local_ask_store.update_job_completed_on_timestamp(
            &ask_id,
            get_timestamp_from_l2block_number(rpc_url, &proof_cycle_completed_on)
                .await
                .unwrap_or_default(),
        );

        let (generator_address, market_id) = {
            let data = local_ask_store.get_by_ask_id(&ask_id).unwrap();
            let generator_address = data.generator.unwrap().into();
            let market_id = data.market_id;
            (generator_address, market_id)
        };

        let proof_generator_cost = generator_store
            .read()
            .await
            .get_by_address_and_market(&generator_address, &market_id)
            .map_or(U256::from(0), |generator_info| {
                generator_info.proof_generation_cost.clone()
            });

        let created_on = { local_ask_store.get_by_ask_id(&ask_id).unwrap().created_on };
        let proof_time = proof_cycle_completed_on_l1.saturating_sub(created_on);

        local_ask_store.store_valid_proof(
            &ask_id,
            proof,
            proof_time,
            proof_generator_cost,
            tx_to_string(&log.transaction_hash.unwrap()),
        );
        local_ask_store.remove_ask_only_if_completed(&ask_id);

        {
            generator_store.write().await.update_on_submit_proof(
                &generator_address,
                &market_id,
                &proof_generator_cost,
                &proof_cycle_completed_on_l1.as_u64().into(),
            );
        }

        {
            market_store.write().await.note_proof_submission_stats(
                &market_id,
                proof_time,
                proof_generator_cost,
            );
        }
        return Ok(());
    }

    if let Ok(new_market_place) = proof_market_place.decode_event_raw(
        "MarketplaceCreated",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::warn!("Decode Raw Event is being used to detect MarketplaceCreated");
        log::warn!("fix this to ensure that you receive all errors during compilation");
        log::info!(
            "New market place has been registered {:?}",
            new_market_place
        );

        let market_id_bytes = new_market_place.first().unwrap();
        let market_id = market_id_bytes.clone().into_uint().unwrap();

        let market = proof_market_place
            .market_data(market_id)
            .call()
            .await
            .unwrap();

        let market = MarketMetadata {
            market_id,
            verifier: market.0,
            prover_images: {
                let mut s = HashSet::new();
                s.insert(market.1.into());
                s
            },
            activation_block: market.3,
            ivs_images: {
                let mut s = HashSet::new();
                s.insert(market.4.into());
                s
            },
            metadata: market.6,
        };

        {
            market_store.write().await.insert(market.clone());
        }

        log::debug!("Market added to store: {:?}", market.market_id);

        return Ok(());
    }

    if let Ok(prover_image_added_log) = proof_market_place
        .decode_event::<pmp::AddExtraProverImageFilter>(
            "AddExtraProverImage",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!(
            "Added prover image: {} to marketplace: {}",
            hex::encode(prover_image_added_log.image_id),
            prover_image_added_log.market_id
        );
        {
            market_store.write().await.add_prover_image(
                prover_image_added_log.market_id,
                prover_image_added_log.image_id.into(),
            );
        }
        return Ok(());
    }

    if let Ok(prover_image_removed_log) = proof_market_place
        .decode_event::<pmp::RemoveExtraProverImageFilter>(
            "RemoveExtraProverImage",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!(
            "Removed prover image: {} from marketplace: {}",
            hex::encode(prover_image_removed_log.image_id),
            prover_image_removed_log.market_id
        );

        {
            market_store.write().await.remove_prover_image(
                prover_image_removed_log.market_id,
                prover_image_removed_log.image_id.into(),
            );
        }
        return Ok(());
    }

    if let Ok(ivs_image_added_log) = proof_market_place.decode_event::<pmp::AddExtraIVSImageFilter>(
        "AddExtraIVSImage",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::debug!(
            "Add ivs image: {} to marketplace: {}",
            hex::encode(ivs_image_added_log.image_id),
            ivs_image_added_log.market_id
        );

        {
            market_store.write().await.add_ivs_image(
                ivs_image_added_log.market_id,
                ivs_image_added_log.image_id.into(),
            );
        }

        return Ok(());
    }

    if let Ok(ivs_image_removed_log) = proof_market_place
        .decode_event::<pmp::RemoveExtraIVSImageFilter>(
            "RemoveExtraIVSImage",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!(
            "Removed ivs image: {} from marketplace: {}",
            hex::encode(ivs_image_removed_log.image_id),
            ivs_image_removed_log.market_id
        );

        {
            market_store.write().await.remove_ivs_image(
                ivs_image_removed_log.market_id,
                ivs_image_removed_log.image_id.into(),
            );
        }

        return Ok(());
    }

    if let Ok(ask_cancelled_log) =
        proof_market_place.decode_event_raw("AskCancelled", log.topics.clone(), log.data.clone())
    {
        let ask_id = ask_cancelled_log
            .get(0)
            .unwrap()
            .clone()
            .into_uint()
            .unwrap();

        log::debug!("Ask has been cancelled {:?}", ask_id);
        let mut local_ask_store = { local_ask_store.write().await };

        let proof_cycle_completed_on: U256 = log.block_number.unwrap().as_u64().into();
        let proof_cycle_completed_on_l1: U256 =
            get_l1_block_from_l2_block(rpc_url, proof_cycle_completed_on)
                .await
                .unwrap_or_default();

        local_ask_store.update_proof_proof_cycle_completed_on(&ask_id, proof_cycle_completed_on_l1);
        local_ask_store.modify_state(&ask_id, AskState::Complete);
        local_ask_store.remove_ask_only_if_completed(&ask_id);
        return Ok(());
    }

    if let Ok(ask_not_generated) = proof_market_place.decode_event_raw(
        "ProofNotGenerated",
        log.topics.clone(),
        log.data.clone(),
    ) {
        let ask_id = ask_not_generated
            .get(0)
            .unwrap()
            .clone()
            .into_uint()
            .unwrap();

        log::warn!("Decode Raw Event is being used to detect ProofNotGenerated");
        log::warn!("Avoid it get error during compilation itself");
        log::warn!(
            "Ask's proof not generated {:?}. Generator is likely slashed",
            ask_id
        );
        let mut local_ask_store = { local_ask_store.write().await };

        let proof_cycle_completed_on: U256 = log.block_number.unwrap().as_u64().into();
        let proof_cycle_completed_on_l1: U256 =
            get_l1_block_from_l2_block(rpc_url, proof_cycle_completed_on)
                .await
                .unwrap_or_default();

        local_ask_store.update_proof_proof_cycle_completed_on(&ask_id, proof_cycle_completed_on_l1);
        local_ask_store.modify_state(&ask_id, AskState::Complete);
        local_ask_store.note_proof_denied(&ask_id, tx_to_string(&log.transaction_hash.unwrap()));

        log::debug!("Proof not Generated: update generator state");
        let (generator_address, _) = {
            let data = local_ask_store.get_by_ask_id(&ask_id).unwrap();
            let generator_address: Address = data.generator.unwrap().into();
            let market_id = data.market_id;
            (generator_address, market_id)
        };

        log::debug!("Proof not Generated: update on slashing penalty");

        let ask = local_ask_store.get_by_ask_id(&ask_id).unwrap();
        local_ask_store.remove_ask_only_if_completed(&ask_id);

        let mut generator_store = generator_store.write().await;

        generator_store.reduce_active_requests(&generator_address, &ask.market_id);

        log::warn!("Complete Proof not Generated");
        return Ok(());
    }

    if let Ok(invalid_inputs_detected_log) = proof_market_place.decode_event_raw(
        "InvalidInputsDetected",
        log.topics.clone(),
        log.data.clone(),
    ) {
        log::warn!("Decode Raw Event is being used to detect InvalidInputsDetected");
        log::warn!("fix this to ensure that you receive all errors during compilation");

        let ask_id = invalid_inputs_detected_log
            .get(0)
            .unwrap()
            .clone()
            .into_uint()
            .unwrap();

        log::warn!(
            "Ask's inputs were wrong {:?}. Submitted attestation for invalid input",
            ask_id
        );
        let mut local_ask_store = { local_ask_store.write().await };

        let proof_cycle_completed_on: U256 = log.block_number.unwrap().as_u64().into();
        let proof_cycle_completed_on_l1: U256 =
            get_l1_block_from_l2_block(rpc_url, proof_cycle_completed_on)
                .await
                .unwrap_or_default();
        local_ask_store.update_proof_proof_cycle_completed_on(&ask_id, proof_cycle_completed_on_l1);
        local_ask_store.modify_state(&ask_id, AskState::Complete);
        local_ask_store.note_invalid_inputs(&ask_id, tx_to_string(&log.transaction_hash.unwrap()));

        local_ask_store.update_job_completed_on_timestamp(
            &ask_id,
            get_timestamp_from_l2block_number(rpc_url, &proof_cycle_completed_on)
                .await
                .unwrap_or_default(),
        );

        let (generator_address, market_id) = {
            let data = local_ask_store.get_by_ask_id(&ask_id).unwrap();
            let generator_address = data.generator.unwrap().into();
            let market_id = data.market_id;
            (generator_address, market_id)
        };

        local_ask_store.remove_ask_only_if_completed(&ask_id);

        {
            let proof_generator_cost = generator_store
                .read()
                .await
                .get_by_address_and_market(&generator_address, &market_id)
                .map_or(U256::from(0), |generator_info| {
                    generator_info.proof_generation_cost.clone()
                });

            generator_store.write().await.update_on_submit_proof(
                &generator_address,
                &market_id,
                &proof_generator_cost,
                &proof_cycle_completed_on_l1.as_u64().into(),
            );
        }
        log::warn!("Complete: invalid input attestation event operation");
        return Ok(());
    }

    if let Ok(update_cost_per_byte_log) = proof_market_place
        .decode_event::<pmp::UpdateCostPerBytesFilter>(
            "UpdateCostPerBytes",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        let secret_type = update_cost_per_byte_log.secret_type;
        let cost_per_byte = update_cost_per_byte_log.cost_per_input_bytes;

        cost_store.write().await.upsert(secret_type, cost_per_byte);

        log::info!(
            "Cost per input byte changed to {:?} for input {:?}",
            cost_per_byte,
            secret_type
        );
        return Ok(());
    }

    if let Ok(operator_reward_log) = proof_market_place
        .decode_event::<pmp::OperatorFeeRewardAddedFilter>(
            "OperatorFeeRewardAdded",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("Operator Fee Reward Added: {}", operator_reward_log);
        return Ok(());
    }

    if let Ok(transmitter_reward_log) = proof_market_place
        .decode_event::<pmp::TransmitterFeeRewardAddedFilter>(
            "TransmitterFeeRewardAdded",
            log.topics.clone(),
            log.data.clone(),
        )
    {
        log::debug!("Transmitter Fee Reward Added: {}", transmitter_reward_log);
        return Ok(());
    }

    // ------ custom patches being handled here --------------------- //
    let provider_http = Provider::<Http>::try_from(rpc_url).unwrap();

    let client = Arc::new(provider_http.clone());

    let pmp_update_marketmetadata_patch = binding_patches::UpdateProofMarketplaceMetadataPatch::new(
        proof_market_place.address(),
        client,
    );

    if let Ok(update_market_metadata_log) = pmp_update_marketmetadata_patch.decode_event_raw(
        "MarketMetadataUpdated",
        log.topics.clone(),
        log.data.clone(),
    ) {
        let market_id_bytes = update_market_metadata_log.get(0).unwrap();
        let market_id = market_id_bytes.clone().into_uint().unwrap();

        let metadata_bytes = update_market_metadata_log.get(1).unwrap();
        let metadata = metadata_bytes.clone().into_bytes().unwrap().to_vec();

        {
            market_store
                .write()
                .await
                .update_marketmeta_bytes(market_id, metadata.into());
        }
        return Ok(());
    }

    if cfg!(feature = "skip_unknown_events") {
        log::warn!("{:?}", log);
        log::warn!("Unknown event noted and skipped");
        return Ok(());
    }

    log::error!("unhandled log in proof market place {:?}", log);
    return Err("Unhandled log in proof market place".into());
}
