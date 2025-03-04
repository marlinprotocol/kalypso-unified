use actix_web::{
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use async_trait::async_trait;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::{BenchmarkResponse, GenerateProofResponse, InputPayload, TestResponse};

#[async_trait]
pub trait GeneratorTrait: Send + Sync {
    async fn generate_proof(&self, input: InputPayload) -> GenerateProofResponse;
    async fn benchmark(&self) -> BenchmarkResponse;
}

#[utoipa::path(
    get,
    path = "/api/test",
    responses(
        (status = 200, description = "Returns 200 if server is reachable", body = TestResponse),
    ),
    tag = "Manage",
    description = "API to test if server is running"
)]
async fn test_handler() -> impl Responder {
    HttpResponse::Ok().json(TestResponse {
        data: "Generator is running!".into(),
    })
}

#[utoipa::path(
    get,
    path = "/api/benchmark",
    responses(
        (status = 200, description = "Returns 200 and Benchmark Response", body = BenchmarkResponse),
    ),
    tag = "Manage",
    description = "API to benchmark proving time of the server"
)]
pub async fn benchmark_handler<T: GeneratorTrait>(generator: web::Data<T>) -> impl Responder {
    let result = generator.benchmark().await;
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    path = "/api/generateProof",
    request_body = InputPayload,
    responses(
        (status = 200, description = "Response on proof generation", body = GenerateProofResponse),
    ),
    tag = "Manage",
    description = "API to fetch the proof from the server"
)]
pub async fn proof_handler<T: GeneratorTrait>(
    generator: web::Data<T>,
    input: Json<InputPayload>,
) -> impl Responder {
    let result = generator.generate_proof(input.into_inner()).await;
    HttpResponse::Ok().json(result)
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Generator",
        description = "APIs to interact with generator",
        version = if cfg!(feature = "mainnet") {
            "mainnet"
        } else {
            "beta"
        },
        license(name = "MIT License", url = "https://opensource.org/licenses/MIT")
    ),
    tags(
        (name = "Manage", description = "interact with generator"),
    )
)]
#[openapi(paths(proof_handler, benchmark_handler, test_handler))]
struct ApiDoc;

fn get_swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
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
            .service(get_swagger())
            .app_data(data.clone())
            .route("/api/test", web::get().to(test_handler))
            .route("/api/benchmark", web::get().to(benchmark_handler::<T>))
            .route("/api/generateProof", web::post().to(proof_handler::<T>))
    })
    .bind(addr)?
    .run()
    .await
}
