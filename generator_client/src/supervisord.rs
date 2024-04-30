use std::env;
use std::process::Command;

use crate::model::SupervisordResponse;

//Get generator status
pub fn get_generator_status() -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let generator_path: String = env::var("GENERATOR_PATH")?
        .parse::<String>()
        .expect("Please provide a valid generator path");

    log::info!("Fetching generator status..");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("status")
        .arg(generator_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error fetching generator status".to_string(),
            status: false,
        };
        return Ok(resp);
    }
    let log = String::from_utf8(output.stdout)?;
    if log.contains("Stopped") {
        let resp = SupervisordResponse {
            output: "Generator hasn't been started yet.".to_string(),
            status: true,
        };
        return Ok(resp);
    }
    if log.contains("Exited") {
        let resp = SupervisordResponse {
            output: "Generator has been stopped.".to_string(),
            status: true,
        };
        return Ok(resp);
    }
    if log.contains("Running") {
        let resp = SupervisordResponse {
            output: "Generator has been started.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error fetching generator status".to_string(),
        status: false,
    };
    Ok(resp)
}

//Start generator
pub fn start_generator() -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let generator_path: String = env::var("GENERATOR_PATH")?
        .parse::<String>()
        .expect("Please provide a valid generator path");

    log::info!("Starting generator...");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("start")
        .arg(generator_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error starting the generator".to_string(),
            status: false,
        };
        return Ok(resp);
    }

    let log = String::from_utf8(output.stdout)?;
    if log.contains("started") {
        let resp = SupervisordResponse {
            output: "Generator started.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error starting the generator".to_string(),
        status: false,
    };
    Ok(resp)
}

//Stop generator
pub fn stop_generator() -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let generator_path: String = env::var("GENERATOR_PATH")?
        .parse::<String>()
        .expect("Please provide a valid generator path");
    log::info!("Stopping the generator..");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("stop")
        .arg(generator_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error stopping the generator".to_string(),
            status: false,
        };
        return Ok(resp);
    }

    let log = String::from_utf8(output.stdout)?;
    if log.contains("stopped") {
        let resp = SupervisordResponse {
            output: "Generator stopped.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error stopping the generator".to_string(),
        status: false,
    };
    Ok(resp)
}
