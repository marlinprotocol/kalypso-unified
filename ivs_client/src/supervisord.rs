use std::env;
use std::process::Command;

use crate::model::SupervisordResponse;

//Get ivs status
pub fn get_ivs_status() -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let ivs_path: String = env::var("IVS_PATH")?
        .parse::<String>()
        .expect("Please provide a valid ivs path");

    log::info!("Fetching ivs status..");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("status")
        .arg(ivs_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error fetching input verifier status".to_string(),
            status: false,
        };
        return Ok(resp);
    }
    let log = String::from_utf8(output.stdout)?;
    if log.contains("Stopped") {
        let resp = SupervisordResponse {
            output: "Input verifier hasn't been started yet.".to_string(),
            status: true,
        };
        return Ok(resp);
    }
    if log.contains("Exited") {
        let resp = SupervisordResponse {
            output: "Input verifier has been stopped.".to_string(),
            status: true,
        };
        return Ok(resp);
    }
    if log.contains("Running") {
        let resp = SupervisordResponse {
            output: "Input verifier has been started.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error fetching input verifier status".to_string(),
        status: false,
    };
    Ok(resp)
}

// Start ivs
pub fn start_ivs() -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let ivs_path: String = env::var("IVS_PATH")?
        .parse::<String>()
        .expect("Please provide a valid input verifier path");

    log::info!("Starting input verifier...");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("start")
        .arg(ivs_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error starting the input verifier".to_string(),
            status: false,
        };
        return Ok(resp);
    }

    let log = String::from_utf8(output.stdout)?;
    if log.contains("started") {
        let resp = SupervisordResponse {
            output: "Input verifier started.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error starting the input verifier".to_string(),
        status: false,
    };
    Ok(resp)
}

// Stop ivs
pub fn stop_ivs() -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let ivs_path: String = env::var("IVS_PATH")?
        .parse::<String>()
        .expect("Please provide a valid input verifier path");
    log::info!("Stopping the input verifier..");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("stop")
        .arg(ivs_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error stopping the input verifier".to_string(),
            status: false,
        };
        return Ok(resp);
    }

    let log = String::from_utf8(output.stdout)?;
    if log.contains("stopped") {
        let resp = SupervisordResponse {
            output: "Input verifier stopped.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error stopping the input verifier".to_string(),
        status: false,
    };
    Ok(resp)
}
