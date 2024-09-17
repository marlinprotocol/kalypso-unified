use std::io::ErrorKind;
use std::sync::{Arc, Mutex};

use crate::handler_funcs::generator_config_setup::_generator_config_setup;
use crate::handler_funcs::update_generator_config::_update_generator_config;
use crate::handler_funcs::update_runtime_config::_udpate_runtime_config;
use crate::kalypso::{
    add_new_generator, benchmark, contract_validation, get_public_keys_for_a_generator,
    read_generator_config_file, read_runtime_config_file, update_generator_config_file,
};
use crate::model::{
    AddNewGenerator, GeneratorConfigSetupRequestBody, GetGeneratorPublicKeys, RemoveGenerator,
    SupervisordInputBody, SupervisordResponse, UpdateGeneratorConfig, UpdateRuntimeConfig,
};
use crate::supervisord::{get_program_status, start_program, stop_program};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{delete, get, post, put, web, Responder};
use helper::common_handlers::{SCHPayload, ToPayload};
use helper::response::response;
use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

#[derive(Deserialize)]
struct ProgramName {
    program_name: String,
}

#[derive(Deserialize)]
struct BenchmarkParams {
    market_id: String,
}

#[derive(Deserialize, Debug)]
struct BenchmarkResponse {
    message: String,
    data: Value,
}

// Start a program on supervisord
#[post("/startProgram")]
async fn start_program_handler(jsonbody: web::Json<SupervisordInputBody>) -> impl Responder {
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

    //Starting the program with supervisord ctl
    let supervisord_response =
        match start_program(json_input.program_name.as_ref().unwrap().to_string()) {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", e);
                return response(
                    "There was an error in starting the program",
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
    response("Program started", StatusCode::OK, None)
}

//Stop a program running on supervisord
#[post("/stopProgram")]
async fn stop_program_handler(jsonbody: web::Json<SupervisordInputBody>) -> impl Responder {
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
    let supervisord_response =
        match stop_program(json_input.program_name.as_ref().unwrap().to_string()) {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", e);
                return response(
                    "There was an error in stopping the program",
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
    response("Program stopped", StatusCode::OK, None)
}

//Restart a program running on supervisord
#[post("/restartProgram")]
async fn restart_program_handler(jsonbody: web::Json<SupervisordInputBody>) -> impl Responder {
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

    // Stopping program
    let stop_supervisord_response =
        match stop_program(json_input.program_name.as_ref().unwrap().to_string()) {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", e);
                return response(
                    "There was an error in stopping the program",
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

    //Starting program
    let start_supervisord_response =
        match start_program(json_input.program_name.as_ref().unwrap().to_string()) {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", e);
                return response(
                    "There was an error in starting the program",
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
    response("Program restarted", StatusCode::OK, None)
}

// Get program status from the supervisord
#[get("/getProgramStatus")]
async fn get_program_status_handler(program_name: web::Query<ProgramName>) -> impl Responder {
    //Validating the main JSON body
    let program_name = &program_name.program_name;
    if program_name.is_empty() {
        return response(
            "Invalid program name, please provide program name in the request query params.",
            StatusCode::BAD_REQUEST,
            None,
        );
    }
    let supervisord_response: SupervisordResponse =
        match get_program_status(program_name.to_string()) {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", e);
                return response(
                    "There was an error in fetching the program status.",
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
        "Program status fetched",
        StatusCode::OK,
        Some(Value::String(supervisord_response.output)),
    )
}

// Generate config setup
#[post("/generatorConfigSetup")]
async fn generate_config_setup(
    jsonbody: web::Json<GeneratorConfigSetupRequestBody>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
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
    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };
    let result = _generator_config_setup(json_input, ecies_priv_key).await;

    if result.is_ok() {
        response("Config done", StatusCode::OK, None)
    } else {
        return response(
            result.unwrap_err().to_string().as_ref(),
            StatusCode::BAD_REQUEST,
            None,
        );
    }
}

// Generate config setup
#[post("/generatorConfigSetupEncrypted")]
async fn generate_config_setup_encrypted(
    jsonbody: web::Json<SCHPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    //Validating the main JSON body
    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };

    let generate_config_setup_request_body: GeneratorConfigSetupRequestBody =
        match jsonbody.0.to_payload(&ecies_priv_key) {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", &e.to_string());
                return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
            }
        };
    let result = _generator_config_setup(&generate_config_setup_request_body, ecies_priv_key).await;

    if result.is_ok() {
        response("Config done", StatusCode::OK, None)
    } else {
        return response(
            result.unwrap_err().to_string().as_ref(),
            StatusCode::BAD_REQUEST,
            None,
        );
    }
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

    let result = _udpate_runtime_config(json_input.0.clone()).await;

    if result.is_ok() {
        response("Runtime config updated", StatusCode::OK, None)
    } else {
        return response(
            result.unwrap_err().to_string().as_ref(),
            StatusCode::BAD_REQUEST,
            None,
        );
    }
}

// Update runtime config encrypted
#[put("/updateRuntimeConfigEncrypted")]
async fn update_runtime_config_encrypted(
    jsonbody: web::Json<SCHPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };

    let update_runtime_config_request_body: UpdateRuntimeConfig =
        match jsonbody.0.to_payload(&ecies_priv_key) {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", &e.to_string());
                return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
            }
        };
    let result = _udpate_runtime_config(update_runtime_config_request_body.clone()).await;

    if result.is_ok() {
        response("Runtime config updated", StatusCode::OK, None)
    } else {
        return response(
            result.unwrap_err().to_string().as_ref(),
            StatusCode::BAD_REQUEST,
            None,
        );
    }
}

#[post("/addNewGenerator")]
async fn add_new_generator_config(
    jsonbody: web::Json<AddNewGenerator>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
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

    let ecies_priv_key = {
        let key = ecies_priv_key.lock().unwrap().clone();
        hex::encode(key)
    };

    let updated_generator_config_data_call =
        add_new_generator(json_input, ecies_priv_key, config_file).await;
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

    let result = _update_generator_config(&json_input).await;

    if result.is_ok() {
        response("Generator config updated", StatusCode::OK, None)
    } else {
        return response(
            result.unwrap_err().to_string().as_ref(),
            StatusCode::BAD_REQUEST,
            None,
        );
    }
}

// Update generator config Encrypted
#[put("/updateGeneratorConfigEncrypted")]
async fn update_generator_config_encrypted(
    jsonbody: web::Json<SCHPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };

    let update_generator_config_request_body: UpdateGeneratorConfig =
        match jsonbody.0.to_payload(&ecies_priv_key) {
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", &e.to_string());
                return response(&e.to_string(), StatusCode::BAD_REQUEST, None);
            }
        };

    let result = _update_generator_config(&update_generator_config_request_body).await;

    if result.is_ok() {
        response("Generator config updated", StatusCode::OK, None)
    } else {
        return response(
            result.unwrap_err().to_string().as_ref(),
            StatusCode::BAD_REQUEST,
            None,
        );
    }
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

// Get program status from the supervisord
#[get("/benchmark")]
async fn benchmark_generator(benchmark_params: web::Query<BenchmarkParams>) -> impl Responder {
    let runtime_config_file = match read_runtime_config_file().await {
        Ok(config_file) => config_file,
        Err(err) => {
            log::error!("{}", err);
            return response(
                "There was an error reading the config file, Please make sure you have setup the generator config file.",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    let markets = runtime_config_file.runtime_config.markets;

    let market_id = &benchmark_params.market_id;

    if markets.contains_key(&market_id.to_string()) {
        let generator_port = &markets.get(&market_id.to_string()).unwrap().port;

        let benchmark_endpoint = format!("http://localhost:{}/api/benchmark", generator_port);

        let benchmark_result = match benchmark(benchmark_endpoint).await {
            Ok(benchmark_res) => benchmark_res,
            Err(err) => {
                log::error!("{}", err);
                return response(
                    &format!(
                        "There was an issue while benchmarking the generator for market ID :{} ",
                        market_id
                    ),
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                );
            }
        };

        if benchmark_result.status().is_server_error() {
            let response_unwrapped: BenchmarkResponse = benchmark_result.json().await.unwrap();
            let res_message = response_unwrapped.message;
            return response(&res_message, StatusCode::INTERNAL_SERVER_ERROR, None);
        }

        if benchmark_result.status().is_success() {
            let response_unwrapped: BenchmarkResponse = benchmark_result.json().await.unwrap();
            let proof_generation_time = response_unwrapped.data.to_string().replace('"', "");
            let response_message = format!("Proof generated in {}ms", proof_generation_time);
            return response(&response_message, StatusCode::OK, None);
        }

        return response(
            "There was an issue while fetching benchmark result from the generator",
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }

    response(
        &format!("No generator found for market ID :{} ", market_id),
        StatusCode::BAD_REQUEST,
        None,
    )
}

// Routes
pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(start_program_handler)
        .service(stop_program_handler)
        .service(restart_program_handler)
        .service(get_program_status_handler)
        .service(generate_config_setup)
        .service(generate_config_setup_encrypted)
        .service(update_runtime_config)
        .service(update_runtime_config_encrypted)
        .service(update_generator_config)
        .service(fetch_generator_public_keys)
        .service(benchmark_generator)
        .service(helper::common_handlers::sign_address)
        .service(helper::common_handlers::sign_attestation)
        .service(helper::common_handlers::sign_address_encrypted)
        .service(helper::common_handlers::sign_attestation_encrypted);
    conf.service(scope);
}
