use std::env;
use std::process::Command;

use crate::model::SupervisordResponse;

//Get matching_engine status
pub fn get_matching_engine_status() -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let matching_engine_path: String = env::var("MATCHING_ENGINE_PATH")?
        .parse::<String>()
        .expect("Please provide a valid matching_engine path");

    log::info!("Fetching matching_engine status..");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("status")
        .arg(matching_engine_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error fetching matching_engine status".to_string(),
            status: false,
        };
        return Ok(resp);
    }
    let log = String::from_utf8(output.stdout)?;
    if log.contains("Stopped") {
        let resp = SupervisordResponse {
            output: "Matching Engine hasn't been started yet.".to_string(),
            status: true,
        };
        return Ok(resp);
    }
    if log.contains("Exited") {
        let resp = SupervisordResponse {
            output: "Matching Engine has been stopped.".to_string(),
            status: true,
        };
        return Ok(resp);
    }
    if log.contains("Running") {
        let resp = SupervisordResponse {
            output: "Matching Engine has been started.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error fetching matching_engine status".to_string(),
        status: false,
    };
    Ok(resp)
}

//Start matching_engine
pub fn start_matching_engine() -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let matching_engine_path: String = env::var("MATCHING_ENGINE_PATH")?
        .parse::<String>()
        .expect("Please provide a valid matching_engine path");

    log::info!("Starting matching_engine...");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("start")
        .arg(matching_engine_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error starting the matching_engine".to_string(),
            status: false,
        };
        return Ok(resp);
    }

    let log = String::from_utf8(output.stdout)?;
    if log.contains("started") {
        let resp = SupervisordResponse {
            output: "Matching Engine started.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error starting the matching_engine".to_string(),
        status: false,
    };
    Ok(resp)
}

//Stop matching_engine
pub fn stop_matching_engine() -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let matching_engine_path: String = env::var("MATCHING_ENGINE_PATH")?
        .parse::<String>()
        .expect("Please provide a valid matching_engine path");
    log::info!("Stopping the matching_engine..");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("stop")
        .arg(matching_engine_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error stopping the matching_engine".to_string(),
            status: false,
        };
        return Ok(resp);
    }

    let log = String::from_utf8(output.stdout)?;
    if log.contains("stopped") {
        let resp = SupervisordResponse {
            output: "Matching Engine stopped.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error stopping the matching_engine".to_string(),
        status: false,
    };
    Ok(resp)
}
