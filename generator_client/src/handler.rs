use std::io::ErrorKind;

use crate::kalypso::{
    add_new_generator, contract_validation, generate_api_key, generate_config_file,
    generate_runtime_file, get_public_keys_for_a_generator, read_generator_config_file,
    read_runtime_config_file, runtime_config_validation, sign_addy, update_generator_config_file,
    update_runtime_config_file, update_runtime_config_with_new_data,
};
use crate::middleware::api_auth;
use crate::model::{
    AddNewGenerator, GeneratorConfigSetupRequestBody, GetGeneratorPublicKeys, RemoveGenerator,
    SignAddress, SupervisordResponse, UpdateGeneratorConfig, UpdateRuntimeConfig,
};
use crate::response::response;
use crate::supervisord::{get_generator_status, start_generator, stop_generator};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, Responder};
use actix_web_lab::middleware::from_fn;
use serde_json::Value;
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

// Start generator
#[post("/startGenerator")]
async fn start_generator_handler() -> impl Responder {
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

    //Starting the generator with supervisord ctl
    let supervisord_response = match start_generator() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in starting the generator",
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
    response("Generator started", StatusCode::OK, None)
}

//Stop generator
#[post("/stopGenerator")]
async fn stop_generator_handler() -> impl Responder {
    let supervisord_response = match stop_generator() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in stopping the generator",
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
    response("Generator stopped", StatusCode::OK, None)
}

//Restart generator
#[post("/restartGenerator")]
async fn restart_generator_handler() -> impl Responder {
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

    // Stopping generator
    let stop_supervisord_response = match stop_generator() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in stopping the generator",
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
    let start_supervisord_response = match start_generator() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in starting the generator",
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
    response("Generator restarted", StatusCode::OK, None)
}

// Get generator status from the supervisord
#[get("/getGeneratorStatus")]
async fn get_generator_status_handler() -> impl Responder {
    let supervisord_response: SupervisordResponse = match get_generator_status() {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an error in fetching the generator status.",
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
        "Generator status fetched",
        StatusCode::OK,
        Some(Value::String(supervisord_response.output)),
    )
}

// Generate config setup
#[post("/generatorConfigSetup")]
async fn generate_config_setup(
    jsonbody: web::Json<GeneratorConfigSetupRequestBody>,
) -> impl Responder {
    //Validating the main JSON body
    let json_input = &jsonbody;
    if let Err(err) = json_input.validate() {
        log::error!("{}", err);
        return response(
            "Invalid payload",
            StatusCode::BAD_REQUEST,
            Some(Value::String(err.to_string())),
        );
    }

    //Validating the generator config
    let generator_config_body = json_input.0.generator_config.as_ref().unwrap();
    if generator_config_body.len() > 1 {
        return response(
            "Only one generator supported",
            StatusCode::BAD_REQUEST,
            None,
        );
    }
    for generator in generator_config_body {
        if generator.supported_markets.as_ref().unwrap().len() > 1 {
            return response(
                "Only one market is supported for every generator",
                StatusCode::BAD_REQUEST,
                None,
            );
        }
        if let Err(err) = generator.validate() {
            log::error!("{}", err);
            return response(
                "Invalid payload",
                StatusCode::BAD_REQUEST,
                Some(Value::String(err.to_string())),
            );
        }
    }

    //Validating the runtime config
    let runtime_config_body = json_input.0.runtime_config.as_ref().unwrap();
    if let Err(err) = runtime_config_body.validate() {
        log::error!("{}", err);
        return response(
            "Invalid payload",
            StatusCode::BAD_REQUEST,
            Some(Value::String(err.to_string())),
        );
    }

    let private_key = runtime_config_body.private_key.as_ref().unwrap();

    let chain_id = runtime_config_body.chain_id.as_ref().unwrap();

    let rpc_url = runtime_config_body.ws_url.as_ref().unwrap();

    //Validating the runtime config to check if the runtime address has enough gas.
    let validation_status = runtime_config_validation(private_key, rpc_url, chain_id).await;
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
            "Runtime private_key doesn't have enough balance, minimum balance required is 0.05ETH",
            StatusCode::BAD_REQUEST,
            None,
        );
    }

    //Generating the generator config file
    let generate_config_file = generate_config_file(generator_config_body).await;
    match generate_config_file {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in generator setup",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    //Generating the runtime config file
    let runtime_config_file = generate_runtime_file(runtime_config_body).await;
    match runtime_config_file {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in runtime setup",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    response("Config done", StatusCode::OK, None)
}

// Update runtime config
#[put("/updateRuntimeConfig")]
async fn update_runtime_config(jsonbody: web::Json<UpdateRuntimeConfig>) -> impl Responder {
    let json_input = &jsonbody;
    if let Err(err) = json_input.validate() {
        log::error!("{}", err);
        return response(
            "Invalid payload",
            StatusCode::BAD_REQUEST,
            Some(Value::String(err.to_string())),
        );
    }
    let config_file_call = read_runtime_config_file().await;
    let config_file = match config_file_call {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue while updating the generator config file",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    // Checking if the provided private_key has enough gas
    let private_key = match &jsonbody.private_key {
        Some(data) => data,
        None => &config_file.runtime_config.private_key,
    };
    let chain_id = match &jsonbody.chain_id {
        Some(data) => data,
        None => &config_file.runtime_config.chain_id,
    };
    let ws_rpc_url = match &jsonbody.ws_url {
        Some(data) => data,
        None => &config_file.runtime_config.ws_url,
    };

    let validation_status = runtime_config_validation(private_key, ws_rpc_url, chain_id).await;
    let validation_status_result = match validation_status {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue while updating the runtime config, Please make sure if you are providing a new RPC url it is a valid one.",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    if !validation_status_result {
        return response(
            "Runtime private_key doesn't have enough balance, minimum balance required is 0.05ETH",
            StatusCode::BAD_REQUEST,
            None,
        );
    }

    //Updating the runtime config file
    let updated_runtime_config_data_call =
        update_runtime_config_with_new_data(json_input, config_file).await;
    let updated_runtime_config_data = match updated_runtime_config_data_call {
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

    let update_config_file = update_runtime_config_file(updated_runtime_config_data).await;
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

    response("Runtime config updated", StatusCode::OK, None)
}

// Update runtime config
#[post("/addNewGenerator")]
async fn add_new_generator_config(jsonbody: web::Json<AddNewGenerator>) -> impl Responder {
    let json_input = &jsonbody;
    if let Err(err) = json_input.validate() {
        log::error!("{}", err);
        return response(
            "Invalid payload",
            StatusCode::BAD_REQUEST,
            Some(Value::String(err.to_string())),
        );
    }
    let generator_address_to_be_added = json_input.0.address.as_ref().unwrap();
    let config_file_call = read_generator_config_file().await;
    let config_file = match config_file_call {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue while updating the generator config file",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    //If generator with the requested address already exists then send response
    if config_file
        .generator_config
        .iter()
        .any(|x| &x.address == generator_address_to_be_added)
    {
        return response(
            "Generator with the provided address already exists.",
            StatusCode::BAD_REQUEST,
            None,
        );
    }

    let updated_generator_config_data_call = add_new_generator(json_input, config_file).await;
    let updated_generator_config_data = match updated_generator_config_data_call {
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

    let update_config_file = update_generator_config_file(updated_generator_config_data).await;
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
    response("Generator config updated", StatusCode::OK, None)
}

// Remove generator from the config file
#[delete("/removeGenerator")]
async fn remove_generator_from_config(jsonbody: web::Json<RemoveGenerator>) -> impl Responder {
    let json_input = jsonbody.0;
    if let Err(err) = json_input.validate() {
        log::error!("{}", err);
        return response(
            "Invalid payload",
            StatusCode::BAD_REQUEST,
            Some(Value::String(err.to_string())),
        );
    }
    let generator_address_to_be_removed = &json_input.address.unwrap();
    let config_file_call = read_generator_config_file().await;
    let mut config_file = match config_file_call {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue while updating the generator config file",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    let finding_generator_index = config_file
        .generator_config
        .iter()
        .position(|x| &x.address == generator_address_to_be_removed);

    let generator_index = match finding_generator_index {
        Some(data) => data,
        None => {
            return response(
                "No generator found for the provided address",
                StatusCode::NOT_FOUND,
                None,
            );
        }
    };

    config_file.generator_config.remove(generator_index);
    let update_config_file = update_generator_config_file(config_file).await;
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
    response("Removed Generator", StatusCode::OK, None)
}

// Update generator config
#[put("/updateGeneratorConfig")]
async fn update_generator_config(jsonbody: web::Json<UpdateGeneratorConfig>) -> impl Responder {
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

    if json_input.supported_markets.as_ref().unwrap().len() > 1 {
        return response(
            "Only one market is supported for every generator",
            StatusCode::BAD_REQUEST,
            None,
        );
    }

    let config_file_call = read_generator_config_file().await;
    let mut config_file = match config_file_call {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue while updating the generator config file",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    //Finding the generator to update
    let generator_to_be_updated = json_input.address.as_ref().unwrap();
    let finding_generator_index = config_file
        .generator_config
        .iter()
        .position(|x| &x.address == generator_to_be_updated);

    let generator_index = match finding_generator_index {
        Some(data) => data,
        None => {
            return response(
                "No generator found for the provided address",
                StatusCode::NOT_FOUND,
                None,
            );
        }
    };

    //Checking the input for changes
    if let Some(new_supported_markets) = &json_input.supported_markets {
        config_file.generator_config[generator_index].supported_markets =
            new_supported_markets.to_vec()
    }
    if let Some(new_data) = &json_input.data {
        config_file.generator_config[generator_index].data = new_data.to_string()
    }

    //Updating the config file
    let update_config_file = update_generator_config_file(config_file).await;
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
    response("Generator config updated", StatusCode::OK, None)
}

// Update generator config
#[post("/fetchGeneratorPublicKeys")]
async fn fetch_generator_public_keys(
    jsonbody: web::Json<GetGeneratorPublicKeys>,
) -> impl Responder {
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
    let config_file_call = read_generator_config_file().await;
    let config_file = match config_file_call {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue while fetching the generator public key.",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    //Finding the generator to update
    let generator_to_be_updated = json_input.generator_address.as_ref().unwrap();
    let finding_generator_index = config_file
        .generator_config
        .iter()
        .position(|x| &x.address == generator_to_be_updated);

    let generator_index = match finding_generator_index {
        Some(data) => data,
        None => {
            return response(
                "No generator found for the provided address",
                StatusCode::NOT_FOUND,
                None,
            );
        }
    };

    let generator_ecies_key = &config_file.generator_config[generator_index].ecies_private_key;

    let generator_public_keys = match get_public_keys_for_a_generator(generator_ecies_key).await {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return response(
                "There was an issue in fetching generator public keys",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    let converted_public_keys = serde_json::to_value(generator_public_keys).unwrap();

    response(
        "Generators public keys fetched",
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
    let signature = serde_json::to_value(sign_addy(&addy_to_be_signed).await.unwrap()).unwrap();
    response("Address signed", StatusCode::OK, Some(signature))
}

// Routes
pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .wrap(from_fn(api_auth))
        .service(generate_api_key_handler)
        .service(start_generator_handler)
        .service(stop_generator_handler)
        .service(restart_generator_handler)
        .service(get_generator_status_handler)
        .service(generate_config_setup)
        .service(update_runtime_config)
        // .service(add_new_generator_config)
        // .service(remove_generator_from_config)
        .service(update_generator_config)
        .service(fetch_generator_public_keys)
        .service(sign_address);
    conf.service(scope);
}
