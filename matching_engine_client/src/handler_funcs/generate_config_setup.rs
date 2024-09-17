use serde_json::{json, Value};

use crate::{
    kalypso::{generate_matching_engine_config_file, matching_engine_config_validation},
    model::MatchingEngineConfigSetupRequestBody,
};

pub async fn _generate_config_setup(
    matching_engine_config_body: &MatchingEngineConfigSetupRequestBody,
    ecies_priv_key: Vec<u8>,
) -> anyhow::Result<Value> {
    let chain_id = matching_engine_config_body.chain_id.as_ref().unwrap();

    let rpc_url = matching_engine_config_body.rpc_url.as_ref().unwrap();

    let private_key = matching_engine_config_body
        .relayer_private_key
        .as_ref()
        .unwrap();

    //Validating the matching_engine config to check if the matching_engine address has enough gas.
    let validation_status = matching_engine_config_validation(private_key, rpc_url, chain_id).await;
    let validation_status_result = match validation_status {
        Ok(data) => data,
        Err(e) => return Err(anyhow::Error::msg(e.to_string())),
    };

    if !validation_status_result {
        return Err(anyhow::Error::msg("Matching engine private_key doesn't have enough balance, minimum balance required is 0.05ETH".to_string()));
    }

    let ecies_priv_key = hex::encode(ecies_priv_key);

    //Generating the matching_engine config file
    let matching_engine_config_file =
        generate_matching_engine_config_file(matching_engine_config_body, ecies_priv_key).await;
    match matching_engine_config_file {
        Ok(data) => data,
        Err(e) => return Err(anyhow::Error::msg(e.to_string())),
    };

    Ok(json!({}))
}
