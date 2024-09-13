use std::sync::{Arc, Mutex};

use crate::handler;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

pub struct GeneratorClient {
    enclave_key: Arc<Mutex<Vec<u8>>>,
    port: u16,
}

impl GeneratorClient {
    pub fn new(enclave_key: String, port: u16) -> Self {
        let enclave_key = hex::decode(enclave_key).unwrap();
        let enclave_key = Arc::new(Mutex::new(enclave_key));
        GeneratorClient { enclave_key, port }
    }

    pub async fn start(self, enable_ssc: bool) -> anyhow::Result<()> {
        let server = HttpServer::new(move || {
            App::new()
                .app_data(Data::new(self.enclave_key.clone()))
                .configure(handler::routes)
        });

        if enable_ssc {
            let tls_config = helper::ssc::create_random_rustls_server_config();
            // Error handling for TLS configuration
            if let Err(err) = tls_config {
                log::error!("Failed to create TLS config: {}", err);
                return Err(anyhow::Error::from(err));
            }

            let tls_config = tls_config.unwrap();

            // Bind the server using Rustls for HTTPS
            let server = server.bind_rustls(format!("0.0.0.0:{}", self.port), tls_config);
            if let Err(err) = server {
                log::error!("Failed to bind server with Rustls: {}", err);
                return Err(anyhow::Error::from(err));
            }

            // Run the server and await
            server.unwrap().run().await?;
        } else {
            // Bind the server using plain HTTP
            let server = server.bind(format!("0.0.0.0:{}", self.port));
            if let Err(err) = server {
                log::error!("Failed to bind server with HTTP: {}", err);
                return Err(anyhow::Error::from(err));
            }

            // Run the server and await
            server.unwrap().run().await?;
        }

        Ok(())
    }
}
