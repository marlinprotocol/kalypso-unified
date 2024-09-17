use serde_json::{json, Value};

use crate::{
    kalypso::{
        matching_engine_config_validation, read_matching_engine_config_file,
        update_matching_engine_config_file, update_matching_engine_config_with_new_data,
    },
    model::UpdateMatchingEngineConfig,
};

pub async fn _update_matching_engine_config_setup(
    jsonbody: &UpdateMatchingEngineConfig,
) -> anyhow::Result<Value> {
    let config_file_call = read_matching_engine_config_file().await;
    let config_file = match config_file_call {
        Ok(data) => data,
        Err(e) => return Err(anyhow::Error::msg(e.to_string())),
    };

    // Checking if the provided private_key has enough gas
    let private_key = match &jsonbody.relayer_private_key {
        Some(data) => data,
        None => &config_file.relayer_private_key,
    };
    let chain_id = match &jsonbody.chain_id {
        Some(data) => data,
        None => &config_file.chain_id,
    };
    let rpc_url = match &jsonbody.rpc_url {
        Some(data) => data,
        None => &config_file.rpc_url,
    };

    let validation_status = matching_engine_config_validation(private_key, rpc_url, chain_id).await;
    let validation_status_result = match validation_status {
        Ok(data) => data,
        Err(e) => return Err(anyhow::Error::msg(e.to_string())),
    };

    if !validation_status_result {
        return Err(anyhow::Error::msg("Matching engine private_key doesn't have enough balance, minimum balance required is 0.05ETH".to_string()));
    }

    //Updating the matching_engine config file
    let updated_matching_engine_config_data_call =
        update_matching_engine_config_with_new_data(jsonbody, config_file).await;
    let updated_matching_engine_config_data = match updated_matching_engine_config_data_call {
        Ok(data) => data,
        Err(e) => return Err(anyhow::Error::msg(e.to_string())),
    };

    let update_config_file =
        update_matching_engine_config_file(updated_matching_engine_config_data).await;
    match update_config_file {
        Ok(data) => data,
        Err(e) => return Err(anyhow::Error::msg(e.to_string())),
    }

    Ok(json!({}))
}
