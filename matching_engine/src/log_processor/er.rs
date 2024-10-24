use crate::generator_lib::*;
use crate::log_processor::constants;
use ecies;
use ethers::prelude::{k256::ecdsa::SigningKey, *};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn process_entity_key_registry_logs(
    logs: Vec<Log>,
    entity_key_registry: bindings::entity_key_registry::EntityKeyRegistry<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
    key_store: &Arc<RwLock<key_store::KeyStore>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for log in &logs {
        if constants::TOPICS_TO_SKIP.get(&log.topics[0]).is_some() {
            log::warn!("standard topic to skip found, ignoring it");
            continue;
        }

        if entity_key_registry
            .decode_event::<bindings::entity_key_registry::EnclaveImageWhitelistedFilter>(
                "EnclaveImageWhitelisted",
                log.topics.clone(),
                log.clone().data,
            )
            .is_ok()
        {
            log::warn!("Skipped EnclaveImageWhitelisted event");
            continue;
        }

        if entity_key_registry
            .decode_event::<bindings::entity_key_registry::EnclaveImageRevokedFilter>(
                "EnclaveImageRevoked",
                log.topics.clone(),
                log.clone().data,
            )
            .is_ok()
        {
            log::warn!("Skipped EnclaveImageRevoked event");
            continue;
        }

        if entity_key_registry
            .decode_event::<bindings::entity_key_registry::EnclaveImageWhitelistedFilter>(
                "EnclaveImageWhitelisted",
                log.topics.clone(),
                log.clone().data,
            )
            .is_ok()
        {
            log::warn!("Skipped EnclaveImageWhitelisted event");
            continue;
        }

        if entity_key_registry
            .decode_event::<bindings::entity_key_registry::EnclaveKeyRevokedFilter>(
                "EnclaveKeyRevoked",
                log.topics.clone(),
                log.clone().data,
            )
            .is_ok()
        {
            log::warn!("Skipped EnclaveKeyRevoked event");
            continue;
        }

        if entity_key_registry
            .decode_event::<bindings::entity_key_registry::EnclaveKeyVerifiedFilter>(
                "EnclaveKeyVerified",
                log.topics.clone(),
                log.clone().data,
            )
            .is_ok()
        {
            log::warn!("Skipped EnclaveKeyVerified event");
            continue;
        }

        if let Ok(parsed_update_key_log) = entity_key_registry
            .decode_event::<bindings::entity_key_registry::UpdateKeyFilter>(
            "UpdateKey",
            log.topics.clone(),
            log.clone().data,
        ) {
            log::debug!("{:?}", parsed_update_key_log);
            let mut key_store = { key_store.write().await };
            let user = parsed_update_key_log.user;
            let key_index = parsed_update_key_log.key_index.as_u64();

            let generator_pub_key = entity_key_registry
                .pub_key(user, key_index.into())
                .call()
                .await
                .unwrap();

            let mut extended_pub_key = vec![0x04];
            extended_pub_key.extend_from_slice(&generator_pub_key);

            // Now, `extended_pub_key` is a 65-byte vector with `04` prepended.
            let pub_key_array: &[u8; 65] = extended_pub_key.as_slice().try_into().unwrap();
            let public_key = ecies::PublicKey::parse(pub_key_array);

            match public_key {
                Ok(generator_public_key) => {
                    log::debug!(
                        "Serialized uncompressed key: {}",
                        hex::encode(generator_public_key.serialize())
                    );
                    let get_user_key = key_store.get_by_address(&user, key_index);
                    match get_user_key {
                        Some(_) => {
                            key_store.update_pub_key(
                                &user,
                                key_index,
                                Some((*pub_key_array).into()),
                            );
                        }
                        None => {
                            let key =
                                key_store::Key::new(user, key_index, Some((*pub_key_array).into()));
                            key_store.insert(key.address, key_index, key);
                        }
                    }
                }
                Err(_err) => {
                    log::warn!("Couldn't derive ecies pub key for the generator {:?}", user);
                    let get_user_key = key_store.get_by_address(&user, key_index);
                    match get_user_key {
                        Some(_) => {
                            key_store.update_pub_key(&user, key_index, None);
                        }
                        None => {
                            let key = key_store::Key::new(user, key_index, None);
                            key_store.insert(user, key_index, key);
                        }
                    }
                }
            };

            continue;
        }

        if let Ok(parsed_remove_key_log) = entity_key_registry
            .decode_event::<bindings::entity_key_registry::RemoveKeyFilter>(
            "RemoveKey",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::debug!("{:?}", parsed_remove_key_log);
            let mut key_store = { key_store.write().await };
            let user = parsed_remove_key_log.user;
            let key_index = parsed_remove_key_log.key_index.as_u64();

            let get_user_key = key_store.get_by_address(&user, key_index);
            if get_user_key.is_some() {
                key_store.remove_by_address(&user, key_index);
            }
            continue;
        }

        if let Ok(initialized_logs) = entity_key_registry.decode_event_raw(
            "Initialized",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::warn!("Version: {:?}", initialized_logs);
            continue;
        }

        if let Ok(attestation_auther_logs) = entity_key_registry.decode_event_raw(
            "EnclaveImageAddedToFamily",
            log.topics.clone(),
            log.data.clone(),
        ) {
            log::warn!("Attestation Auther logs: {:?}", attestation_auther_logs);
            continue;
        }

        log::error!("Unhandled log in entity key registry {:?}", log);
        return Err("Unhandled log in entity key registry".into());
    }

    Ok(())
}
