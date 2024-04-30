use crate::kalypso::{
    generate_api_key, generate_ivs_config_file, get_public_keys_for_ivs, sign_addy, sign_attest,
};
use crate::middleware::api_auth;
use crate::model::{SignAddress, SignAttestation, SupervisordResponse};
use crate::response::response;
use crate::supervisord::{get_ivs_status, start_ivs, stop_ivs};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, Responder};
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

// Start input verifier
#[post("/startInputVerifier")]
async fn start_input_verifier_handler() -> impl Responder {
    //Starting the input verifier with supervisord ctl
    let supervisord_response = match start_ivs() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in starting the input verifier",
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
    response("Input verifier started", StatusCode::OK, None)
}

//Stop input verifier
#[post("/stopInputVerifier")]
async fn stop_input_verifier_handler() -> impl Responder {
    let supervisord_response = match stop_ivs() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in stopping the input verifier",
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
    response("Input verifier stopped", StatusCode::OK, None)
}

//Restart input verifier
#[post("/restartInputVerifier")]
async fn restart_input_verifier_handler() -> impl Responder {
    // Stopping input verifier
    let stop_supervisord_response = match stop_ivs() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in stopping the input verifier",
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

    //Starting generator
    let start_supervisord_response = match start_ivs() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in starting the input verifier",
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
    response("Input verifier restarted", StatusCode::OK, None)
}

// Get input verifier status from the supervisord
#[get("/getInputVerifierStatus")]
async fn get_input_verifier_status_handler() -> impl Responder {
    let supervisord_response: SupervisordResponse = match get_ivs_status() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in fetching the input verifier status.",
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
        "Input verifier status fetched",
        StatusCode::OK,
        Some(Value::String(supervisord_response.output)),
    )
}

// Generate config setup
#[post("/generateConfigSetup")]
async fn generate_config_setup() -> impl Responder {
    //Generating the ivs config file
    let generate_config_file = generate_ivs_config_file().await;
    match generate_config_file {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in ivs setup",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    response("Config done", StatusCode::OK, None)
}

#[post("/fetchInputVerifierPublicKeys")]
async fn fetch_input_verifier_public_keys() -> impl Responder {
    let ivs_public_keys = match get_public_keys_for_ivs().await {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in fetching input verifier public keys",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    let converted_public_keys = serde_json::to_value(ivs_public_keys).unwrap();

    response(
        "Input verifier public keys fetched",
        StatusCode::OK,
        Some(converted_public_keys),
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
        .service(start_input_verifier_handler)
        .service(stop_input_verifier_handler)
        .service(restart_input_verifier_handler)
        .service(get_input_verifier_status_handler)
        .service(generate_config_setup)
        .service(fetch_input_verifier_public_keys)
        .service(sign_address)
        .service(sign_attestation);
    conf.service(scope);
}
