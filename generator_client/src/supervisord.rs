use std::env;
use std::process::Command;

use crate::model::SupervisordResponse;

//Get program status
pub fn get_program_status(
    program_name: String,
) -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let program_path: String = program_name;

    log::info!("Fetching program status..");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("status")
        .arg(program_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error fetching program status".to_string(),
            status: false,
        };
        return Ok(resp);
    }
    let log = String::from_utf8(output.stdout)?;
    if log.contains("Stopped") {
        let resp = SupervisordResponse {
            output: "Program hasn't been started yet.".to_string(),
            status: true,
        };
        return Ok(resp);
    }
    if log.contains("Exited") {
        let resp = SupervisordResponse {
            output: "Program has been stopped.".to_string(),
            status: true,
        };
        return Ok(resp);
    }
    if log.contains("Running") {
        let resp = SupervisordResponse {
            output: "Program has been started.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error fetching program status".to_string(),
        status: false,
    };
    Ok(resp)
}

//Start program
pub fn start_program(
    program_name: String,
) -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let program_path: String = program_name;

    log::info!("Starting program...");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("start")
        .arg(program_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error starting the program".to_string(),
            status: false,
        };
        return Ok(resp);
    }

    let log = String::from_utf8(output.stdout)?;
    if log.contains("started") {
        let resp = SupervisordResponse {
            output: "Program started.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error starting the program".to_string(),
        status: false,
    };
    Ok(resp)
}

//Stop program
pub fn stop_program(
    program_name: String,
) -> Result<SupervisordResponse, Box<dyn std::error::Error>> {
    let supervisord_path: String = env::var("SUPERVISORD_PATH")?
        .parse::<String>()
        .expect("Please provide a valid supervisord path");

    let program_path: String = program_name;
    log::info!("Stopping the program..");
    let output = Command::new(supervisord_path)
        .arg("ctl")
        .arg("stop")
        .arg(program_path)
        .output()?;

    if !output.status.success() {
        let error_log = String::from_utf8(output.stderr)?;
        log::error!("{:#?}", error_log);
        let resp = SupervisordResponse {
            output: "Error stopping the program".to_string(),
            status: false,
        };
        return Ok(resp);
    }

    let log = String::from_utf8(output.stdout)?;
    if log.contains("stopped") {
        let resp = SupervisordResponse {
            output: "Program stopped.".to_string(),
            status: true,
        };
        return Ok(resp);
    }

    let resp = SupervisordResponse {
        output: "Error stopping the program".to_string(),
        status: false,
    };
    Ok(resp)
}
