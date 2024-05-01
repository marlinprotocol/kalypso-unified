use std::io::ErrorKind;

use crate::kalypso::{
    add_new_generator, benchmark, contract_validation, generate_config_file, generate_runtime_file,
    get_public_keys_for_a_generator, read_generator_config_file, read_runtime_config_file,
    runtime_config_validation, sign_addy, sign_attest, update_generator_config_file,
    update_runtime_config_file, update_runtime_config_with_new_data,
};
use crate::model::{
    AddNewGenerator, GeneratorConfigSetupRequestBody, GetGeneratorPublicKeys, RemoveGenerator,
    SignAddress, SignAttestation, SupervisordInputBody, SupervisordResponse, UpdateGeneratorConfig,
    UpdateRuntimeConfig,
};
use crate::response::response;
use crate::supervisord::{get_program_status, start_program, stop_program};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, Responder};
use ethers::types::BigEndianHash;
use serde::Deserialize;
use serde_json::{json, Value};
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
    let signed = sign_addy(addy_to_be_signed).await.unwrap();
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
            let proof_generation_time = response_unwrapped.data.to_string().replace("\"", "");
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
        .service(update_runtime_config)
        // .service(add_new_generator_config)
        // .service(remove_generator_from_config)
        .service(update_generator_config)
        .service(fetch_generator_public_keys)
        .service(sign_address)
        .service(sign_attestation)
        .service(benchmark_generator);
    conf.service(scope);
}
