use std::sync::Arc;

use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};
use ethers::types::U64;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;

pub struct ListenerHealthCheckServer {
    shared_latest_block: Arc<Mutex<U64>>,
    should_stop: Arc<AtomicBool>,
}

impl ListenerHealthCheckServer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(shared_latest_block: Arc<Mutex<U64>>, should_stop: Arc<AtomicBool>) -> Self {
        ListenerHealthCheckServer {
            shared_latest_block,
            should_stop,
        }
    }

    pub async fn start_server(self, port: u16, enable_ssc: bool) -> anyhow::Result<()> {
        let server = HttpServer::new(move || {
            App::new()
                .wrap(kalypso_helper::middlewares::ratelimiter::get_rate_limiter())
                .app_data(Data::new(self.shared_latest_block.clone()))
                .route("/getLatestBlock", web::get().to(get_latest_block_number))
        });

        if enable_ssc {
            let tls_config = kalypso_helper::ssc::create_random_rustls_server_config();
            // Error handling for TLS configuration
            if let Err(err) = tls_config {
                log::error!("Failed to create TLS config: {}", err);
                self.should_stop.store(true, Ordering::Release);
                return Err(anyhow::Error::from(err));
            }

            let tls_config = tls_config.unwrap();

            // Bind the server using Rustls for HTTPS
            let server = server.bind_rustls(format!("0.0.0.0:{}", port), tls_config);
            if let Err(err) = server {
                log::error!("Failed to bind server with Rustls: {}", err);
                self.should_stop.store(true, Ordering::Release);
                return Err(anyhow::Error::from(err));
            }

            // Run the server and await
            server.unwrap().run().await?;
        } else {
            // Bind the server using plain HTTP
            let server = server.bind(format!("0.0.0.0:{}", port));
            if let Err(err) = server {
                log::error!("Failed to bind server with HTTP: {}", err);
                self.should_stop.store(true, Ordering::Release);
                return Err(anyhow::Error::from(err));
            }

            // Run the server and await
            server.unwrap().run().await?;
        }

        Ok(())
    }
}

async fn get_latest_block_number(
    _shared_parsed_block: Data<Arc<Mutex<U64>>>,
) -> actix_web::Result<HttpResponse> {
    let latest_parsed_block = _shared_parsed_block.lock().await;

    #[derive(Serialize, Debug, Clone)]
    struct GetLatestBlockNumberResponse {
        pub block_number: String,
    }

    Ok(HttpResponse::Ok().json(GetLatestBlockNumberResponse {
        block_number: latest_parsed_block.to_string(),
    }))
}
