use serde_json::{json, Value};

use crate::{kalypso::contract_validation, supervisord::start_matching_engine};

pub async fn _start_matching_engine() -> anyhow::Result<Value> {
    //Smart contract checks
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

    //Starting the matching_engine with supervisord ctl
    let supervisord_response = match start_matching_engine() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(
                "There was an error in starting the matching_engine".to_string(),
            ));
        }
    };
    if !supervisord_response.status {
        return Err(anyhow::Error::msg(supervisord_response.output.to_string()));
    }

    Ok(json!({}))
}
