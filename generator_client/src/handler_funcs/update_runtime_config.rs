use serde_json::{json, Value};

use crate::{
    kalypso::{
        read_runtime_config_file, runtime_config_validation, update_runtime_config_file,
        update_runtime_config_with_new_data,
    },
    model::UpdateRuntimeConfig,
};
use std::io::ErrorKind;

pub async fn _udpate_runtime_config(jsonbody: UpdateRuntimeConfig) -> anyhow::Result<Value> {
    let config_file_call = read_runtime_config_file().await;
    let config_file = match config_file_call {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(
                "There was an issue while updating the generator config file".to_string(),
            ));
        }
    };

    // Checking if the provided private_key has enough gas
    let private_key = match &jsonbody.private_key {
        Some(data) => data,
        None => &config_file.runtime_config.private_key,
    };
    let chain_id = match &jsonbody.chain_id {
        Some(data) => data,
        None => &config_file.runtime_config.chain_id,
    };
    let ws_rpc_url = match &jsonbody.ws_url {
        Some(data) => data,
        None => &config_file.runtime_config.ws_url,
    };

    let validation_status = runtime_config_validation(private_key, ws_rpc_url, chain_id).await;
    let validation_status_result = match validation_status {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg("There was an issue while updating the runtime config, Please make sure if you are providing a new RPC url it is a valid one.".to_string()));
        }
    };
    if !validation_status_result {
        return Err(anyhow::Error::msg(
            "Runtime private_key doesn't have enough balance, minimum balance required is 0.05ETH"
                .to_string(),
        ));
    }

    //Updating the runtime config file
    let updated_runtime_config_data_call =
        update_runtime_config_with_new_data(&jsonbody, config_file).await;
    let updated_runtime_config_data = match updated_runtime_config_data_call {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(
                "There was an issue in updating the config file".to_string(),
            ));
        }
    };

    let update_config_file = update_runtime_config_file(updated_runtime_config_data).await;
    match update_config_file {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            if e.kind() == ErrorKind::NotFound {
                return Err(anyhow::Error::msg("There was an issue in updating the config file, since the config file was not found".to_string()));
            }
            return Err(anyhow::Error::msg(
                "There was an issue in updating the config file".to_string(),
            ));
        }
    }

    Ok(json!({}))
}
