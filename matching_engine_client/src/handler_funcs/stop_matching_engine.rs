use serde_json::{json, Value};

use crate::supervisord::stop_matching_engine;

pub fn _stop_matching_engine() -> anyhow::Result<Value> {
    let supervisord_response = match stop_matching_engine() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(
                "There was an error in stopping the matching_engine".to_string(),
            ));
        }
    };
    if !supervisord_response.status {
        return Err(anyhow::Error::msg(
            "supervisord_response.output".to_string(),
        ));
    }

    Ok(json!({}))
}
