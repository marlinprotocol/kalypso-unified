use std::io::ErrorKind;

use crate::kalypso::{
    contract_validation, generate_api_key, generate_matching_engine_config_file,
    get_matching_engine_ecies_public_key, get_matching_engine_public_key,
    matching_engine_config_validation, read_matching_engine_config_file, sign_addy, sign_attest,
    update_matching_engine_config_file, update_matching_engine_config_with_new_data,
};
use crate::middleware::api_auth;
use crate::model::{
    MatchingEngineConfigSetupRequestBody, MatchingEnginePublicKeys, SignAddress, SignAttestation,
    SupervisordResponse, UpdateMatchingEngineConfig,
};
use crate::response::response;
use crate::supervisord::{get_matching_engine_status, start_matching_engine, stop_matching_engine};
use actix_web::http::StatusCode;
use actix_web::{get, post, put, web, Responder};
use actix_web_lab::middleware::from_fn;
use ethers::types::BigEndianHash;
use serde_json::{json, Value};
use validator::Validate;

// Generate API key
#[post("/generateApiKey")]
async fn generate_api_key_handler() -> impl Responder {
    let api_key_response = match generate_api_key().await {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in generating the API key",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !api_key_response.status {
        return response(&api_key_response.message, StatusCode::UNAUTHORIZED, None);
    }

    response(
        &api_key_response.message,
        StatusCode::OK,
        Some(Value::String(api_key_response.api_key)),
    )
}

// Start matching_engine
#[post("/startMatchingEngine")]
async fn start_matching_engine_handler() -> impl Responder {
    //Smart contract checks
    let validation_status = contract_validation().await;
    let validation_status_result = match validation_status {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in validating",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !validation_status_result.status {
        return response(
            &validation_status_result.message,
            StatusCode::BAD_REQUEST,
            None,
        );
    }

    //Starting the matching_engine with supervisord ctl
    let supervisord_response = match start_matching_engine() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in starting the matching_engine",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !supervisord_response.status {
        return response(
            &supervisord_response.output,
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }
    response("Matching Engine started", StatusCode::OK, None)
}

//Stop matching_engine
#[post("/stopMatchingEngine")]
async fn stop_matching_engine_handler() -> impl Responder {
    let supervisord_response = match stop_matching_engine() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in stopping the matching_engine",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !supervisord_response.status {
        return response(
            &supervisord_response.output,
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }
    response("Matching Engine stopped", StatusCode::OK, None)
}

//Restart matching_engine
#[post("/restartMatchingEngine")]
async fn restart_matching_engine_handler() -> impl Responder {
    //Smart contract checks
    let validation_status = contract_validation().await;
    let validation_status_result = match validation_status {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in validating",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !validation_status_result.status {
        return response(
            &validation_status_result.message,
            StatusCode::BAD_REQUEST,
            None,
        );
    }

    // Stopping matching_engine
    let stop_supervisord_response = match stop_matching_engine() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in stopping the matching_engine",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !stop_supervisord_response.status {
        return response(
            &stop_supervisord_response.output,
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }

    //Starting matching_engine
    let start_supervisord_response = match start_matching_engine() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in starting the matching_engine",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !start_supervisord_response.status {
        return response(
            &start_supervisord_response.output,
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }
    response("Matching Engine restarted", StatusCode::OK, None)
}

// Get matching_engine status from the supervisord
#[get("/getMatchingEngineStatus")]
async fn get_matching_engine_status_handler() -> impl Responder {
    let supervisord_response: SupervisordResponse = match get_matching_engine_status() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in fetching the matching_engine status.",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !supervisord_response.status {
        return response(
            &supervisord_response.output,
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }
    response(
        "Matching Engine status fetched",
        StatusCode::OK,
        Some(Value::String(supervisord_response.output)),
    )
}

// Generate config setup
#[post("/matchingEngineConfigSetup")]
async fn generate_config_setup(
    jsonbody: web::Json<MatchingEngineConfigSetupRequestBody>,
) -> impl Responder {
    //Validating the main JSON body
    let matching_engine_config_body = &jsonbody;
    if let Err(err) = matching_engine_config_body.validate() {
        log::error!("{}", err);
        return response(
            "Invalid payload",
            StatusCode::BAD_REQUEST,
            Some(Value::String(err.to_string())),
        );
    }

    let private_key = matching_engine_config_body
        .relayer_private_key
        .as_ref()
        .unwrap();

    let chain_id = matching_engine_config_body.chain_id.as_ref().unwrap();

    let rpc_url = matching_engine_config_body.rpc_url.as_ref().unwrap();

    //Validating the matching_engine config to check if the matching_engine address has enough gas.
    let validation_status = matching_engine_config_validation(private_key, rpc_url, chain_id).await;
    let validation_status_result = match validation_status {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue while validating the request",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !validation_status_result {
        return response(
            "Matching engine private_key doesn't have enough balance, minimum balance required is 0.05ETH",
            StatusCode::BAD_REQUEST,
            None,
        );
    }

    //Generating the matching_engine config file
    let matching_engine_config_file =
        generate_matching_engine_config_file(matching_engine_config_body).await;
    match matching_engine_config_file {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in matching_engine setup",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    response("Config done", StatusCode::OK, None)
}

// Update matching_engine config
#[put("/updateMatchingEngineConfig")]
async fn update_matching_engine_config(
    jsonbody: web::Json<UpdateMatchingEngineConfig>,
) -> impl Responder {
    let json_input = &jsonbody;
    let config_file_call = read_matching_engine_config_file().await;
    let config_file = match config_file_call {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue while updating the matching_engine config file",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    // Checking if the provided private_key has enough gas
    let private_key = match &jsonbody.relayer_private_key {
        Some(data) => data,
        None => &config_file.relayer_private_key,
    };
    let chain_id = match &jsonbody.chain_id {
        Some(data) => data,
        None => &config_file.chain_id,
    };
    let rpc_url = match &jsonbody.rpc_url {
        Some(data) => data,
        None => &config_file.rpc_url,
    };

    let validation_status = matching_engine_config_validation(private_key, rpc_url, chain_id).await;
    let validation_status_result = match validation_status {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue while validating the request",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !validation_status_result {
        return response(
            "Matching engine private_key doesn't have enough balance, minimum balance required is 0.05ETH",
            StatusCode::BAD_REQUEST,
            None,
        );
    }

    //Updating the matching_engine config file
    let updated_matching_engine_config_data_call =
        update_matching_engine_config_with_new_data(json_input, config_file).await;
    let updated_matching_engine_config_data = match updated_matching_engine_config_data_call {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in updating the config file",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    let update_config_file =
        update_matching_engine_config_file(updated_matching_engine_config_data).await;
    match update_config_file {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            if e.kind() == ErrorKind::NotFound {
                return response(
                    "There was an issue in updating the config file, since the config file was not found",
                    StatusCode::NOT_FOUND,
                    None,
                );
            }
            return response(
                "There was an issue in updating the config file",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    }

    response("Matching Engine config updated", StatusCode::OK, None)
}

// Get matching_engine status from the supervisord
#[get("/getMatchingEnginePublicKeys")]
async fn get_matching_engine_public_keys() -> impl Responder {
    let matching_engine_public_key = match get_matching_engine_public_key().await {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in fetching matching engine public key",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    let matching_engine_ecies_public_key = match get_matching_engine_ecies_public_key().await {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in fetching matching engine ecies public key",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    let public_keys = MatchingEnginePublicKeys {
        matching_engine_public_key,
        matching_engine_ecies_public_key,
    };

    let converted_public_key = serde_json::to_value(public_keys).unwrap();

    response(
        "Matching Engine public keys fetched",
        StatusCode::OK,
        Some(converted_public_key),
    )
}

// Sign Address
#[post("/signAddress")]
async fn sign_address(jsonbody: web::Json<SignAddress>) -> impl Responder {
    //Validating inputs
    let json_input = &jsonbody.0;
    if let Err(err) = json_input.validate() {
        log::error!("{}", err);
        return response(
            "Invalid payload",
            StatusCode::BAD_REQUEST,
            Some(Value::String(err.to_string())),
        );
    }
    let addy_to_be_signed = json_input.address.as_ref().unwrap();
    let signed = sign_addy(&addy_to_be_signed).await.unwrap();
    let signature = json!({
        "r": ethers::types::H256::from_uint(&signed.r),
        "s": ethers::types::H256::from_uint(&signed.s),
        "v": signed.v
    });
    response("Address signed", StatusCode::OK, Some(signature))
}

// Sign Attestaion
#[post("/signAttestation")]
async fn sign_attestation(jsonbody: web::Json<SignAttestation>) -> impl Responder {
    // Validating inputs
    let json_input = &jsonbody.0;
    if let Err(err) = json_input.validate() {
        log::error!("{}", err);
        return response(
            "Invalid attestation",
            StatusCode::BAD_REQUEST,
            Some(Value::String(err.to_string())),
        );
    }
    let signed = sign_attest(jsonbody.0).await.unwrap();
    let signature = json!({
        "r": ethers::types::H256::from_uint(&signed.r),
        "s": ethers::types::H256::from_uint(&signed.s),
        "v": signed.v
    });
    response("Attestation signed", StatusCode::OK, Some(signature))
}

// Routes
pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(generate_api_key_handler)
        .service(start_matching_engine_handler)
        .service(stop_matching_engine_handler)
        .service(restart_matching_engine_handler)
        .service(get_matching_engine_status_handler)
        .service(generate_config_setup)
        .service(get_matching_engine_public_keys)
        .service(update_matching_engine_config)
        .service(sign_address)
        .service(sign_attestation);
    conf.service(scope);
}
