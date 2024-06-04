use ethers::prelude::{k256::ecdsa::SigningKey, *};

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::ask::*;
use crate::generator::*;
use crate::secret_inputs_helpers;

use bindings::proof_marketplace as pmp;

use super::constants;

pub async fn process_proof_market_place_logs(
    logs: Vec<Log>,
    proof_market_place: pmp::ProofMarketplace<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    local_ask_store: &Arc<Mutex<LocalAskStore>>,
    generator_store: &Arc<Mutex<GeneratorStore>>,
    market_store: &Arc<Mutex<MarketMetadataStore>>,
    matching_engine_key: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut local_ask_store = local_ask_store.lock().await;
    let mut generator_store = generator_store.lock().await;
    let mut market_store = market_store.lock().await;
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
                proving_time: ask_data.0.time_taken_for_proof_generation,
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

            // [matching_engine/src/log_processor/pm.rs:63:17] ask_to_store = LocalAsk {
            //     ask_id: 220,
            //     market_id: 6,
            //     reward: 14500000000000000000,
            //     expiry: 100050812875,
            //     proving_time: 100000000000,
            //     deadline: 0,
            //     prover_refund_address: 0x2f3f64c69b2954ce2f85d1f92a4151bfc71c78ea,
            //     prover_data: Bytes(0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000003f616c656f31726e3633366739346d783371716866376d37396e736e65336c6c7634647173323537303779687763726b393270306b7772633971653339327767000000000000000000000000000000000000000000000000000000000000000004337536340000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000043175363400000000000000000000000000000000000000000000000000000000),
            //     has_private_inputs: false,
            //     secret_data: None,
            //     secret_acl: None,
            //     state: Some(
            //         The ask was created,
            //     ),
            //     generator: None,
            //     invalid_secret_flag: false,
            // }

            if parsed_ask_created_log.has_private_inputs {
                ask_to_store.secret_data = Some(parsed_ask_created_log.secret_data);
                ask_to_store.secret_acl = Some(parsed_ask_created_log.acl);
                let secret_inputs = &ask_to_store.clone().secret_data.unwrap();
                let acl = &ask_to_store.clone().secret_acl.unwrap();

                let decrypted_secret_data = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
                    secret_inputs,
                    acl,
                    matching_engine_key,
                    ask_to_store.market_id,
                );

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
            generator_store.update_state(
                &ask_data.3,
                &ask_data.0.market_id,
                GeneratorState::Joined,
            );

            let slashing_penalty = market_store
                .get_slashing_penalty_by_market_id(&ask_data.0.market_id)
                .unwrap();
            generator_store.update_on_assigned_task(
                &ask_data.3,
                &ask_data.0.market_id,
                slashing_penalty,
            );

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

            let ask_data: (pmp::Ask, u8, H160, H160) =
                proof_market_place.list_of_ask(ask_id).call().await.unwrap();

            local_ask_store.modify_state(&ask_id, AskState::Complete);
            let generator_address = ask_data.3;
            generator_store.update_state(
                &generator_address,
                &ask_data.0.market_id,
                GeneratorState::Joined,
            );

            let slashing_penalty = market_store
                .get_slashing_penalty_by_market_id(&ask_data.0.market_id)
                .unwrap();
            generator_store.update_on_submit_proof(
                &generator_address,
                &ask_data.0.market_id,
                slashing_penalty,
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

            let market_id_bytes = new_market_place.get(0).unwrap();
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
                let ask_id_bytes = ask_not_generated.get(0).unwrap();
                ask_id_bytes.clone().into_uint().unwrap()
            };
            log::warn!(
                "Ask's proof not generated {:?}. Generator is likely slashed",
                ask_id
            );

            local_ask_store.modify_state(&ask_id, AskState::Complete);

            let ask_data: (pmp::Ask, u8, H160, H160) =
                proof_market_place.list_of_ask(ask_id).call().await.unwrap();

            log::debug!("Proof not Generated: update generator state");
            let generator_address = ask_data.3;
            generator_store.update_state(
                &generator_address,
                &ask_data.0.market_id,
                GeneratorState::Joined,
            );

            log::debug!("Proof not Generated: update on slashing penalty");
            let slashing_penalty = market_store
                .get_slashing_penalty_by_market_id(&ask_data.0.market_id)
                .unwrap();
            generator_store.update_on_slashing(
                &generator_address,
                &ask_data.0.market_id,
                slashing_penalty,
            );

            log::warn!("Complete Proof not Generated");
            continue;
        }

        if let Ok(invalid_inputs_detected_log) = proof_market_place.decode_event_raw(
            "InvalidInputsDetected",
            log.topics.clone(),
            log.data.clone(),
        ) {
            let ask_id_bytes = invalid_inputs_detected_log.get(0).unwrap();
            let ask_id = ask_id_bytes.clone().into_uint().unwrap();

            log::warn!(
                "Ask's inputs were wrong {:?}. Submiting proof of invalid input",
                ask_id
            );
            local_ask_store.modify_state(&ask_id, AskState::Complete);

            let ask_data: (pmp::Ask, u8, H160, H160) =
                proof_market_place.list_of_ask(ask_id).call().await.unwrap();

            let generator_address = ask_data.3;
            generator_store.update_state(
                &generator_address,
                &ask_data.0.market_id,
                GeneratorState::Joined,
            );

            let slashing_penalty = market_store
                .get_slashing_penalty_by_market_id(&ask_data.0.market_id)
                .unwrap();
            generator_store.update_on_submit_proof(
                &generator_address,
                &ask_data.0.market_id,
                slashing_penalty,
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
