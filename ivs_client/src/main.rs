mod handler;
mod kalypso;
mod model;
mod supervisord;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

use std::env;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| panic!("PORT must be provided in the .env file"))
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let enable_ssc: bool = env::var("ENABLE_SSC")
        .ok()
        .map(|val| val == "true" || val == "1")
        .unwrap_or(false);

    let server = HttpServer::new(move || App::new().configure(handler::routes));

    if enable_ssc {
        let tls_config = helper::ssc::create_random_rustls_server_config();
        // Error handling for TLS configuration
        if let Err(err) = tls_config {
            log::error!("Failed to create TLS config: {}", err);
            return Err(anyhow::Error::from(err));
        }

        let tls_config = tls_config.unwrap();

        // Bind the server using Rustls for HTTPS
        let server = server.bind_rustls(format!("0.0.0.0:{}", &port), tls_config);
        if let Err(err) = server {
            log::error!("Failed to bind server with Rustls: {}", err);
            return Err(anyhow::Error::from(err));
        }

        // Run the server and await
        server.unwrap().run().await?;
    } else {
        // Bind the server using plain HTTP
        let server = server.bind(format!("0.0.0.0:{}", &port));
        if let Err(err) = server {
            log::error!("Failed to bind server with HTTP: {}", err);
            return Err(anyhow::Error::from(err));
        }

        // Run the server and await
        server.unwrap().run().await?;
    }

    Ok(())
}
