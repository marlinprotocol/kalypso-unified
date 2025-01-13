use actix_web::web::Data;
use actix_web::HttpResponse;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::generator_lib::symbiotic_stake_store::SymbioticStakeStore;
use crate::models::WelcomeResponse;
use crate::utility::address_to_string;

pub async fn get_snapshot(
    _local_symbiotic_store: Data<Arc<RwLock<SymbioticStakeStore>>>,
) -> actix_web::Result<HttpResponse> {
    let local_symbiotic_store = {
        match _local_symbiotic_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let operators: Vec<_> = local_symbiotic_store
        .operators
        .iter()
        .map(|(addr, tracker)| {
            json!({
                "address": address_to_string(addr),
                "tokens": tracker.to_token_amount()
            })
        })
        .collect();

    let vault_snapshots: Vec<_> = local_symbiotic_store
        .vault_snapshots
        .iter()
        .map(|(timestamp, snapshot)| {
            let snapshot: Vec<_> = snapshot
                .iter()
                .map(|(index, snap)| {
                    json!({
                        "index": index.to_string(),
                        "snapshot": {
                            "transmitter": address_to_string(&snap.transmitter),
                            "index": snap.index.to_string(),
                            "captured_timestamp": snap.captured_timestamp.to_string(),
                            "num_of_transactions": snap.num_of_transactions.to_string(),
                            "image_id": hex::encode(&snap.image_id),
                            "snapshot_data": hex::encode(&snap.snapshot_data),
                            "proof": hex::encode(&snap.proof),
                        }
                    })
                })
                .collect();

            json!({
                "timestamp": timestamp.to_string(),
                "snapshot": snapshot

            })
        })
        .collect();

    let slash_results: Vec<_> = local_symbiotic_store
        .slash_results
        .iter()
        .map(|(timestamp, slash_result)| {
            let slash_result: Vec<_> = slash_result
                .iter()
                .map(|(index, slash)| {
                    json!({
                        "index": index.to_string(),
                        "slash": {
                            "transmitter": address_to_string(&slash.transmitter),
                            "index": slash.index.to_string(),
                            "captured_timestamp": slash.captured_timestamp.to_string(),
                            "num_of_transactions": slash.num_of_transactions.to_string(),
                            "image_id": hex::encode(&slash.image_id),
                            "slash_data": hex::encode(&slash.slash_data),
                            "proof": hex::encode(&slash.proof),
                        }
                    })
                })
                .collect();
            json!({
                "timestamp": timestamp.to_string(),
                "slash_result": slash_result
            })
        })
        .collect();

    let vault_snapshot_indexes: Vec<_> = local_symbiotic_store
        .vault_snapshot_indexes
        .iter()
        .map(|a| a.to_string())
        .collect();

    let slash_result_indexes: Vec<_> = local_symbiotic_store
        .slash_result_indexes
        .iter()
        .map(|a| a.to_string())
        .collect();

    let tokens_to_lock = local_symbiotic_store.tokens_to_lock.to_token_amount();

    return Ok(HttpResponse::Ok().json(json!({
        "operators": operators,
        "vault_snapshots": vault_snapshots,
        "slash_results": slash_results,
        "vault_snapshot_indexes": vault_snapshot_indexes,
        "slash_result_indexes": slash_result_indexes,
        "tokens_to_lock": tokens_to_lock
    })));
}
