use std::sync::Arc;
use std::time::Duration;

use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};
use ethers::types::U64;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

pub struct ListenerHealthCheckServer {
    shared_latest_block: Arc<Mutex<U64>>,
    service_name: Arc<Mutex<String>>,
    should_stop: Arc<AtomicBool>,
    shared_metrics: Arc<Mutex<kalypso_helper::prom_client::ListenerMetrics>>,
}

use kalypso_helper::common_handlers::__path_metrics_handler;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Prover Listener",
        description = "Prover Listener Runtime Info",
        version = if cfg!(feature = "mainnet") {
            "mainnet"
        } else {
            "beta"
        },
        license(name = "MIT License", url = "https://opensource.org/licenses/MIT")
    ),
    tags(
        (name = "Manage", description = "Check Listener State"),
    )
)]
#[openapi(paths(get_latest_block_number, metrics_handler))]
struct ApiDoc;

fn get_swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}

impl ListenerHealthCheckServer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_name: String,
        shared_latest_block: Arc<Mutex<U64>>,
        should_stop: Arc<AtomicBool>,
        shared_metrics: Arc<Mutex<kalypso_helper::prom_client::ListenerMetrics>>,
    ) -> Self {
        ListenerHealthCheckServer {
            shared_latest_block,
            service_name: Arc::new(Mutex::new(service_name)),
            should_stop,
            shared_metrics,
        }
    }

    pub async fn start_server(self, port: u16, enable_ssc: bool) -> anyhow::Result<()> {
        let server = HttpServer::new(move || {
            let rate_limiter = kalypso_helper::middlewares::ratelimiter::get_rate_limiter(
                Duration::from_secs(1),
                100 as u64,
            );
            App::new()
                .wrap(rate_limiter)
                .app_data(Data::new(self.shared_latest_block.clone()))
                .app_data(Data::new(self.service_name.clone()))
                .app_data(Data::new(self.shared_metrics.clone()))
                .service(get_swagger())
                .route("/getLatestBlock", web::get().to(get_latest_block_number))
                .route(
                    "/metrics",
                    web::get().to(kalypso_helper::common_handlers::metrics_handler),
                )
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

#[derive(Serialize, Debug, Clone, ToSchema)]
struct GetLatestBlockNumberResponse {
    pub service_name: String,
    pub block_number: String,
}

#[utoipa::path(
    get,
    path = "/getLatestBlock",
    responses(
        (status = 200, description = "Return the latest block till block parsed", body = GetLatestBlockNumberResponse),
    ),
    tag = "Manage"
)]
async fn get_latest_block_number(
    _shared_parsed_block: Data<Arc<Mutex<U64>>>,
    service_name: Data<Arc<Mutex<String>>>,
) -> actix_web::Result<HttpResponse> {
    let latest_parsed_block = _shared_parsed_block.lock().unwrap();
    let service_name = service_name.lock().unwrap();

    Ok(HttpResponse::Ok().json(GetLatestBlockNumberResponse {
        service_name: service_name.to_string(),
        block_number: latest_parsed_block.to_string(),
    }))
}
