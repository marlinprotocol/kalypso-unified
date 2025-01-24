use async_trait::async_trait;

use actix_web::{
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use generator_models::{
    generator::{benchmark_handler, proof_handler, GeneratorTrait},
    models::{self, InputPayload},
};

use crate::models::{
    CheckInputResponse, EncryptedInputPayload, InvalidInputPayload,
    SignInputsAndProofForNonConfidentialInput, VerifyInputAndProofResponse, VerifyInputsAndProof,
};

#[async_trait]
pub trait IVSTrait: Send + Sync {
    async fn handle_test_request(&self) -> models::TestResponse;
    async fn handle_input_request(&self, input: models::InputPayload) -> models::TestResponse;
    async fn handle_invalid_input_request(
        &self,
        invalid_input: InvalidInputPayload,
    ) -> CheckInputResponse;
    async fn handle_generate_check_encrypted_inputs_request(
        &self,
        encrypted_payload: EncryptedInputPayload,
    ) -> CheckInputResponse;
    async fn handle_verify_inputs_and_proof_request(
        &self,
        verify_input_and_secret_payload: VerifyInputsAndProof,
    ) -> VerifyInputAndProofResponse;

    async fn handle_sign_inputs_and_proof_request(
        &self,
        verify_input_and_secret_payload: SignInputsAndProofForNonConfidentialInput,
    ) -> generator_models::models::GenerateProofResponse;
}

async fn test_handler<T: IVSTrait>(ivs: web::Data<T>) -> impl Responder {
    let response = ivs.handle_test_request().await;
    HttpResponse::Ok().json(response)
}

async fn input_handler<T: IVSTrait>(
    ivs: web::Data<T>,
    payload: Json<InputPayload>,
) -> impl Responder {
    let response = ivs.handle_input_request(payload.into_inner()).await;
    HttpResponse::Ok().json(response)
}

async fn invalid_input_handler<T: IVSTrait>(
    ivs: web::Data<T>,
    payload: Json<InvalidInputPayload>,
) -> impl Responder {
    let response = ivs.handle_invalid_input_request(payload.into_inner()).await;
    HttpResponse::Ok().json(response)
}

async fn encrypted_input_handler<T: IVSTrait>(
    ivs: web::Data<T>,
    payload: Json<EncryptedInputPayload>,
) -> impl Responder {
    let response = ivs
        .handle_generate_check_encrypted_inputs_request(payload.into_inner())
        .await;
    HttpResponse::Ok().json(response)
}

async fn verify_handler<T: IVSTrait>(
    ivs: web::Data<T>,
    payload: Json<VerifyInputsAndProof>,
) -> impl Responder {
    let response = ivs
        .handle_verify_inputs_and_proof_request(payload.into_inner())
        .await;
    HttpResponse::Ok().json(response)
}

async fn signed_inputs_handler<T: IVSTrait>(
    ivs: web::Data<T>,
    payload: Json<SignInputsAndProofForNonConfidentialInput>,
) -> impl Responder {
    let response = ivs
        .handle_sign_inputs_and_proof_request(payload.into_inner())
        .await;
    HttpResponse::Ok().json(response)
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
            .app_data(service_data.clone())
            .route("/api/test", web::get().to(test_handler::<T>))
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

pub async fn start_confidential_proving_server<ConfProver: GeneratorTrait + IVSTrait + 'static>(
    addr: &str,
    generator: ConfProver,
) -> std::io::Result<()> {
    let data = web::Data::new(generator);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route(
                "/api/benchmark",
                web::get().to(benchmark_handler::<ConfProver>),
            )
            .route(
                "/api/generateProof",
                web::post().to(proof_handler::<ConfProver>),
            )
            .route("/api/test", web::get().to(test_handler::<ConfProver>))
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
