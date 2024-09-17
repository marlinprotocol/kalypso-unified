use crate::handler_funcs::generate_config_setup::_generate_config_setup;
use crate::handler_funcs::update_matching_engine_config::_update_matching_engine_config_setup;
use crate::kalypso::{
    contract_validation, get_matching_engine_ecies_public_key, get_matching_engine_public_key,
};

use crate::model::{
    MatchingEngineConfigSetupRequestBody, MatchingEnginePublicKeys, SupervisordResponse,
    UpdateMatchingEngineConfig,
};
use crate::supervisord::{get_matching_engine_status, start_matching_engine, stop_matching_engine};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{get, post, put, web, Responder};
use helper::common_handlers::{SCHPayload, ToPayload, ToSchResponse};
use helper::response::response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use validator::Validate;

#[derive(Deserialize, Serialize)]
struct EmptyPayload {}

// Start matching_engine
#[post("/startMatchingEngine")]
async fn start_matching_engine_handler(_payload: web::Json<EmptyPayload>) -> impl Responder {
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
async fn stop_matching_engine_handler(_payload: web::Json<EmptyPayload>) -> impl Responder {
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
async fn restart_matching_engine_handler(_payload: web::Json<EmptyPayload>) -> impl Responder {
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
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
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

    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };

    let result = _generate_config_setup(matching_engine_config_body, ecies_priv_key).await;

    if result.is_ok() {
        return response("Config done", StatusCode::OK, None);
    } else {
        return response(
            result.unwrap_err().to_string().as_ref(),
            StatusCode::BAD_REQUEST,
            None,
        );
    }
}

// Generate config setup encrypted
#[post("/matchingEngineConfigSetupEncrypted")]
async fn generate_config_setup_encrypted(
    jsonbody: web::Json<SCHPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };

    let matching_engine_config_body: MatchingEngineConfigSetupRequestBody =
        match jsonbody.0.to_payload(&ecies_priv_key) {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", &e.to_string());
                return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
            }
        };

    let result = _generate_config_setup(&matching_engine_config_body, ecies_priv_key.clone()).await;

    if result.is_ok() {
        let result = result.unwrap();
        let sch_response = match jsonbody.0.to_sch_response(result, ecies_priv_key).await {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", &e.to_string());
                return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
            }
        };
        return response(
            "Config done",
            StatusCode::OK,
            Some(serde_json::to_value(&sch_response).unwrap()),
        );
    } else {
        return response(
            result.unwrap_err().to_string().as_ref(),
            StatusCode::BAD_REQUEST,
            None,
        );
    }
}

// Update matching_engine config
#[put("/updateMatchingEngineConfig")]
async fn update_matching_engine_config(
    jsonbody: web::Json<UpdateMatchingEngineConfig>,
) -> impl Responder {
    let json_input = &jsonbody;

    let result = _update_matching_engine_config_setup(json_input).await;

    if result.is_ok() {
        response("Matching Engine config updated", StatusCode::OK, None)
    } else {
        return response(
            result.unwrap_err().to_string().as_ref(),
            StatusCode::BAD_REQUEST,
            None,
        );
    }
}

// Update matching_engine config encrypted
#[put("/updateMatchingEngineConfigEncrypted")]
async fn update_matching_engine_config_encrypted(
    jsonbody: web::Json<SCHPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };

    let update_matching_engine_config_json: UpdateMatchingEngineConfig =
        match jsonbody.0.to_payload(&ecies_priv_key) {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", &e.to_string());
                return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
            }
        };

    let result = _update_matching_engine_config_setup(&update_matching_engine_config_json).await;

    if result.is_ok() {
        let result = result.unwrap();
        let sch_response = match jsonbody.0.to_sch_response(result, ecies_priv_key).await {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", &e.to_string());
                return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
            }
        };
        return response(
            "Matching Engine config updated",
            StatusCode::OK,
            Some(serde_json::to_value(&sch_response).unwrap()),
        );
    } else {
        return response(
            result.unwrap_err().to_string().as_ref(),
            StatusCode::BAD_REQUEST,
            None,
        );
    }
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

// Routes
pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(start_matching_engine_handler)
        .service(stop_matching_engine_handler)
        .service(restart_matching_engine_handler)
        .service(get_matching_engine_status_handler)
        .service(generate_config_setup)
        .service(generate_config_setup_encrypted)
        .service(get_matching_engine_public_keys)
        .service(update_matching_engine_config)
        .service(update_matching_engine_config_encrypted)
        .service(helper::common_handlers::sign_address)
        .service(helper::common_handlers::sign_attestation)
        .service(helper::common_handlers::sign_address_encrypted)
        .service(helper::common_handlers::sign_attestation_encrypted);

    conf.service(scope);
}
