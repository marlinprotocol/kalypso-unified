use generator::generator::{start_non_confidential_proving_server, GeneratorTrait};
use generator::models::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let null_prover = NullProver::default();

    start_non_confidential_proving_server("0.0.0.0:3000", null_prover)
        .await
        .unwrap();

    Ok(())
}

use async_trait::async_trait;

#[derive(Default)]
struct NullProver;

#[async_trait]
impl GeneratorTrait for NullProver {
    async fn generate_proof(&self, _input: InputPayload) -> GenerateProofResponse {
        // Actual logic here
        unimplemented!()
    }

    async fn benchmark(&self) -> BenchmarkResponse {
        // Actual logic here
        unimplemented!()
    }
}
