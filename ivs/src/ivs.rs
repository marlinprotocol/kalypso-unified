use actix_web::http::StatusCode;
use async_trait::async_trait;

use actix_web::web::Data;
use actix_web::{
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};

use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{LocalWallet, Signer, Wallet},
};
use generator_models::{
    generator::{benchmark_handler, proof_handler, GeneratorTrait},
    models::{self, InputPayload, TestResponse},
};

use crate::models::{
    CheckInputResponse, EncryptedInputPayload, InvalidInputPayload,
    SignInputsAndProofForNonConfidentialInput, VerifyInputAndProofResponse, VerifyInputsAndProof,
};

use std::fs;
use std::sync::{Arc, Mutex};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[async_trait]
pub trait IVSTrait: Send + Sync {
    async fn check_inputs(&self, input: InputPayload) -> CheckInputResponse;
    async fn check_inputs_and_proof(
        &self,
        inputs_and_proof: VerifyInputsAndProof,
    ) -> VerifyInputAndProofResponse;
}

#[utoipa::path(
    get,
    path = "/api/test",
    responses(
        (status = 200, description = "Check Server Connection", body = models::TestResponse),
    ),
    tag = "Manage"
)]
async fn test_handler() -> impl Responder {
    HttpResponse::Ok().json(TestResponse {
        data: "Running!".into(),
    })
}

#[utoipa::path(
    post,
    path = "/api/checkInput",
    request_body = InputPayload,
    responses(
        (status = 200, description = "Check Inputs are correct or not", body = CheckInputResponse),
    ),
    tag = "Manage"
)]
async fn input_handler<T: IVSTrait>(
    ivs: web::Data<T>,
    payload: Json<InputPayload>,
) -> impl Responder {
    let response = ivs.check_inputs(payload.into_inner()).await;
    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    post,
    path = "/api/getAttestationForInvalidInputs",
    responses(
        (status = 200, description = "Check Inputs are correct or not", body = models::GenerateProofResponse),
    ),
    tag = "Manage"
)]
async fn invalid_input_handler<T: IVSTrait>(
    ivs: web::Data<T>,
    payload: Json<InvalidInputPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let input = InputPayload::from_plain_secrets(
        payload.get_public(),
        payload.get_plain_secrets().unwrap(),
    );
    let check_input_response = ivs.check_inputs(input).await;

    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };
    let signer_wallet = get_signer(ecies_priv_key);

    if check_input_response.valid {
        return HttpResponse::Ok().json(check_input_response);
    }

    return HttpResponse::Ok()
        .json(generate_invalid_input_attestation(payload.0, signer_wallet).await);
}

#[utoipa::path(
    post,
    path = "/api/checkEncryptedInputs",
    request_body = EncryptedInputPayload,
    responses(
        (status = 200, description = "Check If Encrypted inputs are valid or not", body = CheckInputResponse),
    ),
    tag = "Manage"
)]
async fn encrypted_input_handler<T: IVSTrait>(
    ivs: web::Data<T>,
    payload: Json<EncryptedInputPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let (signature, ivs_pub_key) = {
        let message = &payload.market_id;
        let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };
        let signer_wallet = get_signer(ecies_priv_key);
        let digest = ethers::utils::keccak256(message.as_bytes());

        let read_secp_pub_key = fs::read("./app/secp.pub").unwrap();
        let mut modified_secp_pub_key = vec![0x04];
        modified_secp_pub_key.extend_from_slice(&read_secp_pub_key);
        let signature = signer_wallet
            .sign_hash(ethers::types::H256(digest))
            .expect("Failed signing market id for check encrypted inputs");
        (signature.to_string(), modified_secp_pub_key)
    };
    let decrypt_request_payload = matching_engine_models::models::DecryptRequest {
        market_id: payload.market_id.to_string(),
        private_input: hex::encode(payload.clone().encrypted_secrets),
        acl: hex::encode(payload.clone().acl),
        signature,
        ivs_pubkey: hex::encode(ivs_pub_key),
    };

    let client = reqwest::Client::new();
    let api_response = client
        .post(&payload.me_decryption_url)
        .json(&decrypt_request_payload)
        .send()
        .await
        .unwrap();

    if api_response.status().is_success() {
        let response_payload: matching_engine_models::models::GetRequestResponse =
            match api_response.json().await {
                Ok(data) => data,
                Err(err) => {
                    dbg!(err);
                    return kalypso_helper::response::response(
                        "Unable to get response from matching engine",
                        StatusCode::EXPECTATION_FAILED,
                        None,
                    );
                }
            };

        let encrypted_data = hex::decode(response_payload.encrypted_data).unwrap();
        let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };
        let decrypted_data =
            kalypso_helper::secret_inputs_helpers::decrypt_ecies(&ecies_priv_key, &encrypted_data)
                .unwrap();

        let decrypted_secret = {
            let decrypted_secret = String::from_utf8(decrypted_data.clone());
            if decrypted_secret.is_ok() {
                decrypted_data
            } else {
                let decompreseed_decrypted_data =
                    kalypso_helper::secret_inputs_helpers::flatten(&decrypted_data).unwrap();
                decompreseed_decrypted_data
            }
        };

        let input = InputPayload::from_plain_secrets(
            payload.0.public_inputs.unwrap_or(vec![]),
            decrypted_secret,
        );

        let check_input_response = ivs.check_inputs(input).await;

        return HttpResponse::Ok().json(check_input_response);
    } else {
        return kalypso_helper::response::response(
            "Could not fetch info from matching engine",
            StatusCode::FAILED_DEPENDENCY,
            None,
        );
    }
}

#[utoipa::path(
    post,
    path = "/api/verifyInputsAndProof",
    request_body = VerifyInputsAndProof,
    responses(
        (status = 200, description = "Check If proof is valid against inputs are valid or not", body = VerifyInputAndProofResponse),
    ),
    tag = "Manage"
)]
async fn verify_handler<T: IVSTrait>(
    ivs: web::Data<T>,
    payload: Json<VerifyInputsAndProof>,
) -> impl Responder {
    let response = ivs.check_inputs_and_proof(payload.into_inner()).await;
    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    post,
    path = "/api/signInputsAndProofForNonConfidentialInputs",
    request_body = SignInputsAndProofForNonConfidentialInput,
    responses(
        (status = 200, description = "Get signed inputs and proofs", body = models::GenerateProofResponse),
    ),
    tag = "Manage"
)]
async fn signed_inputs_handler<T: IVSTrait>(
    ivs: web::Data<T>,
    payload: Json<SignInputsAndProofForNonConfidentialInput>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let inputs_and_proof = VerifyInputsAndProof {
        public_input: Some(payload.clone().public_input),
        private_input: None,
        proof: payload.clone().proof,
    };

    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };
    let signer_wallet = get_signer(ecies_priv_key);

    let response = ivs.check_inputs_and_proof(inputs_and_proof).await;

    if response.is_input_and_proof_valid {
        return HttpResponse::Ok()
            .json(sign_inputs_and_proof(payload.clone(), signer_wallet).await);
    } else {
        return HttpResponse::Ok().json(response);
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Ivs",
        description = "APIs to interact with ivs",
        version = "beta",
        license(name = "MIT License", url = "https://opensource.org/licenses/MIT")
    ),
    tags(
        (name = "Manage", description = "interact with ivs"),
    )
)]
#[openapi(paths(
    test_handler,
    input_handler,
    invalid_input_handler,
    encrypted_input_handler,
    verify_handler,
    signed_inputs_handler
))]
struct IvsApiDoc;

fn get_ivs_swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", IvsApiDoc::openapi())
}
// -------------------------------------------------------------------------
// 3) START SERVER (WIRES UP ROUTES TO HANDLERS)
// -------------------------------------------------------------------------
pub async fn start_ivs_server<T: IVSTrait + 'static>(
    addr: &str,
    ivs_impl: T,
) -> std::io::Result<()> {
    let service_data = web::Data::new(ivs_impl);

    HttpServer::new(move || {
        App::new()
            .service(get_ivs_swagger())
            .app_data(service_data.clone())
            .route("/api/test", web::get().to(test_handler))
            .route("/api/checkInput", web::post().to(input_handler::<T>))
            .route(
                "/api/getAttestationForInvalidInputs",
                web::post().to(invalid_input_handler::<T>),
            )
            .route(
                "/api/checkEncryptedInputs",
                web::post().to(encrypted_input_handler::<T>),
            )
            .route(
                "/api/verifyInputsAndProof",
                web::post().to(verify_handler::<T>),
            )
            .route(
                "/api/signInputsAndProofForNonConfidentialInputs",
                web::post().to(signed_inputs_handler::<T>),
            )
    })
    .bind(addr)?
    .run()
    .await
}

use generator_models::generator::__path_benchmark_handler;
use generator_models::generator::__path_proof_handler;
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Confidential Prover",
        description = "APIs to interact with confidential prover",
        version = "beta",
        license(name = "MIT License", url = "https://opensource.org/licenses/MIT")
    ),
    tags(
        (name = "Manage", description = "interact with confidential prover"),
    )
)]
#[openapi(paths(
    test_handler,
    input_handler,
    invalid_input_handler,
    encrypted_input_handler,
    verify_handler,
    signed_inputs_handler,
    benchmark_handler,
    proof_handler
))]
struct ConfidentialProverDoc;

fn get_conf_prover_swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-docs/openapi.json", ConfidentialProverDoc::openapi())
}

pub async fn start_confidential_proving_server<ConfProver: GeneratorTrait + IVSTrait + 'static>(
    addr: &str,
    generator: ConfProver,
) -> std::io::Result<()> {
    let data = web::Data::new(generator);

    HttpServer::new(move || {
        App::new()
            .service(get_conf_prover_swagger())
            .app_data(data.clone())
            .route(
                "/api/benchmark",
                web::get().to(benchmark_handler::<ConfProver>),
            )
            .route(
                "/api/generateProof",
                web::post().to(proof_handler::<ConfProver>),
            )
            .route("/api/test", web::get().to(test_handler))
            .route(
                "/api/checkInput",
                web::post().to(input_handler::<ConfProver>),
            )
            .route(
                "/api/getAttestationForInvalidInputs",
                web::post().to(invalid_input_handler::<ConfProver>),
            )
            .route(
                "/api/checkEncryptedInputs",
                web::post().to(encrypted_input_handler::<ConfProver>),
            )
            .route(
                "/api/verifyInputsAndProof",
                web::post().to(verify_handler::<ConfProver>),
            )
            .route(
                "/api/signInputsAndProofForNonConfidentialInputs",
                web::post().to(signed_inputs_handler::<ConfProver>),
            )
    })
    .bind(addr)?
    .run()
    .await
}

async fn generate_invalid_input_attestation(
    payload: InvalidInputPayload,
    signer_wallet: Wallet<SigningKey>,
) -> models::GenerateProofResponse {
    let ask_id = payload.only_ask_id();
    let value = vec![
        ethers::abi::Token::Uint(ask_id.into()),
        ethers::abi::Token::Bytes(payload.get_public()),
    ];
    let encoded = ethers::abi::encode(&value);
    let digest = ethers::utils::keccak256(encoded);

    let signature = signer_wallet
        .sign_message(ethers::types::H256(digest))
        .await
        .unwrap();

    let response = models::GenerateProofResponse {
        proof: signature.to_vec(),
    };

    return response;
}

async fn sign_inputs_and_proof(
    payload: SignInputsAndProofForNonConfidentialInput,
    signer_wallet: Wallet<SigningKey>,
) -> models::GenerateProofResponse {
    let value = vec![
        ethers::abi::Token::Bytes(payload.public_input.into()),
        ethers::abi::Token::Bytes(payload.proof.into()),
    ];
    let encoded = ethers::abi::encode(&value);
    let digest = ethers::utils::keccak256(encoded);

    let signature = signer_wallet
        .sign_message(ethers::types::H256(digest))
        .await
        .unwrap();

    let response = models::GenerateProofResponse {
        proof: signature.to_vec(),
    };

    return response;
}

fn get_signer(ecies_priv_key: Vec<u8>) -> Wallet<SigningKey> {
    let secp_private_key = secp256k1::SecretKey::from_slice(&ecies_priv_key)
        .unwrap()
        .display_secret()
        .to_string();
    secp_private_key.parse::<LocalWallet>().unwrap()
}
