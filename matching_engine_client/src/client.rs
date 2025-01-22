use actix_web::web::Data;
use actix_web::{App, HttpServer};
use std::sync::{Arc, Mutex};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handler;

pub struct MatchingEngineClient {
    enclave_key: Arc<Mutex<Vec<u8>>>,
    port: u16,
}

use crate::handler::{
    __path_generate_config_setup, __path_generate_config_setup_encrypted,
    __path_get_matching_engine_public_keys, __path_get_matching_engine_status_handler,
    __path_restart_matching_engine_handler, __path_restart_matching_engine_handler_encrypted,
    __path_start_matching_engine_handler, __path_start_matching_engine_handler_encrypted,
    __path_test_handler, __path_update_matching_engine_config,
    __path_update_matching_engine_config_encrypted,
};

use helper::common_handlers::{
    __path_sign_address, __path_sign_address_encrypted, __path_sign_attestation,
    __path_sign_attestation_encrypted,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Kalypso Matching Engine Client APIs",
        description = "APIs to interact with matching engine via client",
        version = "beta",
        license(name = "MIT License", url = "https://opensource.org/licenses/MIT")
    ),
    tags(
        (name = "Manage", description = "Manage Matching Engine"),
        (name = "Deprecated", description = "Unsecure communication with client. May lead to loss of funds"),
        (name = "Secure", description = "Uses nitro verification attestation derived security to communicate with enclave. Currently only possible via kalypso-cli")
    )
)]
#[openapi(paths(
    test_handler,
    start_matching_engine_handler,
    start_matching_engine_handler_encrypted,
    restart_matching_engine_handler,
    restart_matching_engine_handler_encrypted,
    get_matching_engine_status_handler,
    generate_config_setup,
    generate_config_setup_encrypted,
    get_matching_engine_public_keys,
    update_matching_engine_config,
    update_matching_engine_config_encrypted,
    sign_address,
    sign_address_encrypted,
    sign_attestation,
    sign_attestation_encrypted
))]
struct ApiDoc;

fn get_swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}

impl MatchingEngineClient {
    pub fn new(enclave_key: String, port: u16) -> Self {
        let enclave_key = hex::decode(enclave_key).unwrap();
        let enclave_key = Arc::new(Mutex::new(enclave_key));
        MatchingEngineClient { enclave_key, port }
    }

    pub async fn start(self, enable_ssc: bool) -> anyhow::Result<()> {
        let server = HttpServer::new(move || {
            App::new()
                .app_data(Data::new(self.enclave_key.clone()))
                .service(get_swagger())
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
