use actix_web::{
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use async_trait::async_trait;

use crate::models::{BenchmarkResponse, GenerateProofResponse, InputPayload, TestResponse};

#[async_trait]
pub trait GeneratorTrait: Send + Sync {
    async fn handle_test_request(&self) -> TestResponse;
    async fn handle_benchmark_request(&self) -> BenchmarkResponse;
    async fn handle_proof_request(&self, input: InputPayload) -> GenerateProofResponse;
}

async fn test_handler<T: GeneratorTrait>(generator: web::Data<T>) -> impl Responder {
    let result = generator.handle_test_request().await;
    HttpResponse::Ok().json(result)
}

pub async fn benchmark_handler<T: GeneratorTrait>(generator: web::Data<T>) -> impl Responder {
    let result = generator.handle_benchmark_request().await;
    HttpResponse::Ok().json(result)
}

pub async fn proof_handler<T: GeneratorTrait>(
    generator: web::Data<T>,
    input: Json<InputPayload>,
) -> impl Responder {
    let result = generator.handle_proof_request(input.into_inner()).await;
    HttpResponse::Ok().json(result)
}

// -----------------
// 4) START SERVER FUNCTION
// -----------------
pub async fn start_non_confidential_proving_server<T: GeneratorTrait + 'static>(
    addr: &str,
    generator: T,
) -> std::io::Result<()> {
    let data = web::Data::new(generator);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/api/test", web::get().to(test_handler::<T>))
            .route("/api/benchmark", web::get().to(benchmark_handler::<T>))
            .route("/api/generateProof", web::post().to(proof_handler::<T>))
    })
    .bind(addr)?
    .run()
    .await
}
