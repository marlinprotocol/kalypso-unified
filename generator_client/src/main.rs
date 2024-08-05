use dotenv::dotenv;
use generator_client::client;
use tokio::fs;

use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "1500".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let enclave_key = fs::read("/app/secp.sec").await?;
    let server = client::GeneratorClient::new(hex::encode(enclave_key), port);

    server.start().await.unwrap();

    Ok(())
}
