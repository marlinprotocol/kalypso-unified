use serde_json::{json, Value};

use crate::{
    kalypso::contract_validation,
    supervisord::{start_matching_engine, stop_matching_engine},
};

pub async fn _restart_matching_engine() -> anyhow::Result<Value> {
    let validation_status = contract_validation().await;
    let validation_status_result = match validation_status {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(
                "There was an issue in validating".to_string(),
            ));
        }
    };
    if !validation_status_result.status {
        return Err(anyhow::Error::msg(
            validation_status_result.message.to_string(),
        ));
    }

    // Stopping matching_engine
    let stop_supervisord_response = match stop_matching_engine() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(e.to_string()));
        }
    };
    if !stop_supervisord_response.status {
        return Err(anyhow::Error::msg(
            "stop_supervisord_response.output".to_string(),
        ));
    }

    //Starting matching_engine
    let start_supervisord_response = match start_matching_engine() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(
                "There was an error in starting the matching_engine".to_string(),
            ));
        }
    };
    if !start_supervisord_response.status {
        return Err(anyhow::Error::msg(
            start_supervisord_response.output.to_string(),
        ));
    }

    Ok(json!({}))
}
