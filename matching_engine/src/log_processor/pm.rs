use crate::ask_lib::ask::LocalAsk;
use crate::ask_lib::ask_status::AskState;
use crate::ask_lib::ask_store::LocalAskStore;
use crate::costs::CostStore;
use crate::utility::tx_to_string;
use ethers::prelude::{k256::ecdsa::SigningKey, *};
use generator_store::SlashingRecord;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::generator_lib::*;
use crate::market_metadata::*;
use kalypso_helper::secret_inputs_helpers;

use bindings::proof_marketplace as pmp;

use super::constants;

#[allow(clippy::too_many_arguments)]
pub async fn process_proof_market_place_logs(
    logs: Vec<Log>,
    proof_market_place: pmp::ProofMarketplace<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    local_ask_store: &Arc<Mutex<LocalAskStore>>,
    generator_store: &Arc<Mutex<generator_store::GeneratorStore>>,
    market_store: &Arc<Mutex<MarketMetadataStore>>,
    cost_store: &Arc<Mutex<CostStore>>,
    matching_engine_key: &[u8],
    matchin_engine_slave_keys: &Vec<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut local_ask_store = local_ask_store.lock().await;
    let mut generator_store = generator_store.lock().await;
    let mut market_store = market_store.lock().await;
    let mut cost_store = cost_store.lock().await;

    for log in &logs {
        if constants::TOPICS_TO_SKIP.get(&log.topics[0]).is_some() {
            log::warn!("standard topic to skip found, ignoring it");
            continue;
        }

        if let Ok(parsed_ask_created_log) = proof_market_place
            .decode_event::<pmp::AskCreatedFilter>(
                "AskCreated",
                log.topics.clone(),
                log.data.clone(),
            )
        {
            log::debug!("{:?}", parsed_ask_created_log);
            let ask_data: (pmp::Ask, _, _, _) = proof_market_place
                .list_of_ask(parsed_ask_created_log.ask_id)
                .call()
                .await
                .unwrap();

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
            };

            if parsed_ask_created_log.has_private_inputs {
                ask_to_store.secret_data = Some(parsed_ask_created_log.secret_data);
                ask_to_store.secret_acl = Some(parsed_ask_created_log.acl);
                let secret_inputs = &ask_to_store.clone().secret_data.unwrap();
                let acl = &ask_to_store.clone().secret_acl.unwrap();

                let mut decrypted_secret_data =
                    secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
                        secret_inputs,
                        acl,
                        matching_engine_key,
                        Some(ask_to_store.market_id),
                    );

                if decrypted_secret_data.is_err() {
                    // try with slave keys
                    for slave_key in matchin_engine_slave_keys {
                        decrypted_secret_data =
                            secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
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
                    log::info!(
                        "Stored private ask with AskId {:?} to store",
                        parsed_ask_created_log.ask_id
                    );
                } else {
                    ask_to_store.state = Some(AskState::InvalidSecret);
                    log::error!(
                        "Stored ask with AskId {:?} to store but flagged = false",
                        parsed_ask_created_log.ask_id
                    );
                }
                local_ask_store.insert(ask_to_store);
            } else {
                ask_to_store.invalid_secret_flag = true;
                // repeated, try to modify if-else logic and work!..
                log::info!(
                    "Stored {:?} ask to store, Market: {}",
                    parsed_ask_created_log.ask_id,
                    &ask_to_store.market_id
                );
                local_ask_store.insert(ask_to_store);
            }
            continue;
        }

        if let Ok(parsed_task_created_log) = proof_market_place
            .decode_event::<pmp::TaskCreatedFilter>(
                "TaskCreated",
                log.topics.clone(),
                log.data.clone(),
            )
        {
            log::debug!("{:?}", parsed_task_created_log);

            let ask_id = parsed_task_created_log.ask_id;
            let generator = parsed_task_created_log.generator;
            let new_acl = parsed_task_created_log.new_acl;

            local_ask_store.update_ask_generator(&ask_id, Some(generator));
            local_ask_store.update_ask_acl(&ask_id, Some(new_acl));

            let ask_data: (pmp::Ask, u8, H160, H160) =
                proof_market_place.list_of_ask(ask_id).call().await.unwrap();

            local_ask_store.modify_state(&ask_id, AskState::Assigned);
            generator_store.update_on_assigned_task(&ask_data.3, &ask_data.0.market_id);

            continue;
        }

        if let Ok(parsed_proof_created_log) = proof_market_place
            .decode_event::<pmp::ProofCreatedFilter>(
                "ProofCreated",
                log.topics.clone(),
                log.data.clone(),
            )
        {
            log::debug!("{:?}", parsed_proof_created_log);

            let ask_id = parsed_proof_created_log.ask_id;
            let proof = parsed_proof_created_log.proof;

            let ask_data: (pmp::Ask, u8, H160, H160) =
                proof_market_place.list_of_ask(ask_id).call().await.unwrap();

            local_ask_store.modify_state(&ask_id, AskState::Complete);
            let generator_address = ask_data.3;
            let proof_generator_cost = generator_store
                .get_by_address_and_market(&generator_address, &ask_data.0.market_id)
                .map_or(U256::from(0), |generator_info| {
                    generator_info.proof_generation_cost.clone()
                });

            let proof_submitted_on: U256 = log.block_number.unwrap().as_u64().into();
            let proof_time = proof_submitted_on.saturating_sub(ask_data.0.deadline);

            local_ask_store.store_valid_proof(
                &ask_id,
                proof,
                proof_time,
                proof_generator_cost,
                tx_to_string(&log.transaction_hash.unwrap()),
            );
            generator_store.update_on_submit_proof(
                &generator_address,
                &ask_data.0.market_id,
                &proof_generator_cost,
            );

            market_store.note_proof_submission_stats(
                &ask_data.0.market_id,
                proof_time,
                proof_generator_cost,
            );
            continue;
        }

        if let Ok(new_market_place) = proof_market_place.decode_event_raw(
            "MarketplaceCreated",
            log.topics.clone(),
            log.data.clone(),
        ) {
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
                prover_image_id: market.1,
                slashing_penalty: market.2,
                activation_block: market.3,
                ivs_image_id: market.4,
                metadata: market.6,
            };

            market_store.insert(market.clone());
            // let verification_url = market_store.decode_market_verification_url_by_id(&market_id);

            log::debug!("Market added to store: {:?}", market.market_id);

            continue;
        }

        if let Ok(prover_image_added_log) = proof_market_place
            .decode_event::<pmp::AddExtraProverImageFilter>(
                "AddExtraProverImage",
                log.topics.clone(),
                log.data.clone(),
            )
        {
            log::info!(
                "Added prover image: {} to marketplace: {}",
                hex::encode(prover_image_added_log.image_id),
                prover_image_added_log.market_id
            );
            log::warn!("Not indexing adding prover image to market right now");
            continue;
        }

        if let Ok(prover_image_remove_log) = proof_market_place
            .decode_event::<pmp::AddExtraIVSImageFilter>(
                "AddExtraIVSImage",
                log.topics.clone(),
                log.data.clone(),
            )
        {
            log::info!(
                "Removed prover image: {} from marketplace: {}",
                hex::encode(prover_image_remove_log.image_id),
                prover_image_remove_log.market_id
            );
            log::warn!("Not indexing removing prover image from market right now");
            continue;
        }

        if let Ok(ask_cancelled_log) = proof_market_place.decode_event::<pmp::AskCancelledFilter>(
            "AskCancelled",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::debug!("Ask has been cancelled {:?}", ask_cancelled_log.ask_id);
            local_ask_store.modify_state(&ask_cancelled_log.ask_id, AskState::Complete);
            continue;
        }

        if let Ok(ask_not_generated) = proof_market_place.decode_event_raw(
            "ProofNotGenerated",
            log.topics.clone(),
            log.data.clone(),
        ) {
            let ask_id = {
                let ask_id_bytes = ask_not_generated.first().unwrap();
                ask_id_bytes.clone().into_uint().unwrap()
            };
            log::warn!(
                "Ask's proof not generated {:?}. Generator is likely slashed",
                ask_id
            );

            local_ask_store.modify_state(&ask_id, AskState::Complete);
            local_ask_store.note_invalid_proof(&ask_id);

            let ask_data: (pmp::Ask, u8, H160, H160) =
                proof_market_place.list_of_ask(ask_id).call().await.unwrap();

            log::debug!("Proof not Generated: update generator state");
            let generator_address = ask_data.3;

            let market_data = market_store.get_market_by_market_id(&ask_data.0.market_id);
            log::debug!("Proof not Generated: update on slashing penalty");

            let ask = local_ask_store.get_by_ask_id(&ask_id).unwrap();

            generator_store.update_on_slashing(
                &generator_address,
                &ask_data.0.market_id,
                &market_data.unwrap().slashing_penalty,
                SlashingRecord {
                    ask_id,
                    market_id: ask.market_id,
                    slashing_tx: tx_to_string(&log.transaction_hash.unwrap()),
                    price_offered: ask.reward,
                    expected_time: ask.deadline,
                    slashing_penalty: market_data.unwrap().slashing_penalty,
                },
            );

            log::warn!("Complete Proof not Generated");
            continue;
        }

        if let Ok(invalid_inputs_detected_log) = proof_market_place.decode_event_raw(
            "InvalidInputsDetected",
            log.topics.clone(),
            log.data.clone(),
        ) {
            let ask_id_bytes = invalid_inputs_detected_log.first().unwrap();
            let ask_id = ask_id_bytes.clone().into_uint().unwrap();

            log::warn!(
                "Ask's inputs were wrong {:?}. Submiting proof of invalid input",
                ask_id
            );
            local_ask_store.modify_state(&ask_id, AskState::Complete);

            let ask_data: (pmp::Ask, u8, H160, H160) =
                proof_market_place.list_of_ask(ask_id).call().await.unwrap();

            let generator_address = ask_data.3;

            let proof_generator_cost = generator_store
                .get_by_address_and_market(&generator_address, &ask_data.0.market_id)
                .map_or(U256::from(0), |generator_info| {
                    generator_info.proof_generation_cost.clone()
                });

            generator_store.update_on_submit_proof(
                &generator_address,
                &ask_data.0.market_id,
                &proof_generator_cost,
            );
            log::warn!("Complete invalid input proof submitted");
            continue;
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

            cost_store.upsert(secret_type, cost_per_byte);

            log::info!(
                "Cost per input byte changed to {:?} for input {:?}",
                cost_per_byte,
                secret_type
            );
            continue;
        }

        if let Ok(upgraded_logs) = proof_market_place.decode_event::<pmp::UpgradedFilter>(
            "Upgraded",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::debug!("Implementation {:?}", upgraded_logs.implementation);
            continue;
        }

        if let Ok(paused_logs) = proof_market_place.decode_event::<pmp::PausedFilter>(
            "Paused",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::debug!("Paused {:?}", paused_logs.account);
            continue;
        }

        if let Ok(unpaused_logs) = proof_market_place.decode_event::<pmp::UnpausedFilter>(
            "Unpaused",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::debug!("Unpaused {:?}", unpaused_logs.account);
            continue;
        }

        if let Ok(initialized_logs) =
            proof_market_place.decode_event_raw("Initialized", log.topics.clone(), log.data.clone())
        {
            log::warn!("Version: {:?}", initialized_logs);
            continue;
        }

        log::warn!("unhandled log in proof market place {:?}", log);
        return Err("Unhandled log in proof market place".into());
    }
    Ok(())
}
