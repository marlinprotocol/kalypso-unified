use generator_models::generator::GeneratorTrait;
use generator_models::models::*;
use ivs::ivs::{start_confidential_proving_server, IVSTrait};
use ivs::models::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let null_conf_prover = NullConfProver::default();
    start_confidential_proving_server("0.0.0.0:3000", null_conf_prover, vec![])
        .await
        .unwrap();

    Ok(())
}

use async_trait::async_trait;

#[derive(Default)]
struct NullConfProver;

#[async_trait]
impl GeneratorTrait for NullConfProver {
    async fn generate_proof(&self, _input: InputPayload) -> GenerateProofResponse {
        // Actual logic here
        unimplemented!()
    }

    async fn benchmark(&self) -> BenchmarkResponse {
        // Actual logic here
        unimplemented!()
    }
}

#[async_trait]
impl IVSTrait for NullConfProver {
    async fn check_inputs(&self, _input: InputPayload) -> CheckInputResponse {
        // Actual logic here
        unimplemented!()
    }
    async fn check_inputs_and_proof(
        &self,
        _input: VerifyInputsAndProof,
    ) -> VerifyInputAndProofResponse {
        unimplemented!()
    }
}
