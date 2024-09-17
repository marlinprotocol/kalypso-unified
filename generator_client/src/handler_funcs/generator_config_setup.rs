use serde_json::{json, Value};
use validator::Validate;

use crate::{
    kalypso::{generate_config_file, generate_runtime_file, runtime_config_validation},
    model::GeneratorConfigSetupRequestBody,
};

pub async fn _generator_config_setup(
    json_input: &GeneratorConfigSetupRequestBody,
    ecies_priv_key: Vec<u8>,
) -> anyhow::Result<Value> {
    //Validating the generator config
    let generator_config_body = json_input.generator_config.as_ref().unwrap();
    if generator_config_body.len() > 1 {
        return Err(anyhow::Error::msg(
            "Only one generator supported".to_string(),
        ));
    }
    for generator in generator_config_body {
        if generator.supported_markets.as_ref().unwrap().len() > 1 {
            return Err(anyhow::Error::msg(
                "Only one market is supported for every generator".to_string(),
            ));
        }
        if let Err(err) = generator.validate() {
            log::error!("{}", err);
            return Err(anyhow::Error::msg(
                "Invalid Payload: Generator Setup Config".to_string(),
            ));
        }
    }

    //Validating the runtime config
    let runtime_config_body = json_input.runtime_config.as_ref().unwrap();
    if let Err(err) = runtime_config_body.validate() {
        log::error!("{}", err);
        return Err(anyhow::Error::msg(
            "Invalid Payload: Runtime Config".to_string(),
        ));
    }

    let private_key = runtime_config_body.private_key.as_ref().unwrap();

    let chain_id = runtime_config_body.chain_id.as_ref().unwrap();

    let rpc_url = runtime_config_body.ws_url.as_ref().unwrap();

    //Validating the runtime config to check if the runtime address has enough gas.
    let validation_status = runtime_config_validation(private_key, rpc_url, chain_id).await;
    let validation_status_result = match validation_status {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(
                "There was an issue while validating the request".to_string(),
            ));
        }
    };
    if !validation_status_result {
        return Err(anyhow::Error::msg(
            "Runtime private_key doesn't have enough balance, minimum balance required is 0.05ETH"
                .to_string(),
        ));
    }

    let ecies_priv_key = { hex::encode(ecies_priv_key) };

    //Generating the generator config file
    let generate_config_file = generate_config_file(generator_config_body, ecies_priv_key).await;
    match generate_config_file {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(
                "There was an issue in generator setup".to_string(),
            ));
        }
    };

    //Generating the runtime config file
    let runtime_config_file = generate_runtime_file(runtime_config_body).await;
    match runtime_config_file {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(
                "There was an issue in runtime setup".to_string(),
            ));
        }
    };

    Ok(json!({}))
}
