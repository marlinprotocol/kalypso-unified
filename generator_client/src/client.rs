use std::sync::{Arc, Mutex};

use crate::handler;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct GeneratorClient {
    enclave_key: Arc<Mutex<Vec<u8>>>,
    port: u16,
}

use handler::{
    __path_add_new_generator_config, __path_benchmark_generator,
    __path_fetch_generator_public_keys, __path_generate_config_setup,
    __path_generate_config_setup_encrypted, __path_get_program_status_handler,
    __path_remove_generator_from_config, __path_restart_program_handler,
    __path_start_program_handler, __path_stop_program_handler, __path_update_generator_config,
    __path_update_generator_config_encrypted, __path_update_runtime_config,
    __path_update_runtime_config_encrypted,
};

use helper::common_handlers::{
    __path_sign_address, __path_sign_address_encrypted, __path_sign_attestation,
    __path_sign_attestation_encrypted,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Kalypso Generator Client APIs",
        description = "APIs to interact with generator via client",
        version = if cfg!(feature = "mainnet") {
            "mainnet"
        } else {
            "beta"
        },
        license(name = "MIT License", url = "https://opensource.org/licenses/MIT"),
    ),
    tags(
        (name = "Not Tested", description = "These APIs are not tested and should not be used in any environment"),
        (name = "Secure", description = "These APIs are secure and should be used to load generator configurations"),
        (name = "Deprecated", description = "These APIs are deprecated and should not be used in any environment"),

    )
)]
#[openapi(paths(
    start_program_handler,
    stop_program_handler,
    restart_program_handler,
    get_program_status_handler,
    generate_config_setup,
    generate_config_setup_encrypted,
    update_runtime_config,
    update_runtime_config_encrypted,
    add_new_generator_config,
    remove_generator_from_config,
    update_generator_config,
    update_generator_config_encrypted,
    fetch_generator_public_keys,
    benchmark_generator,
    sign_address,
    sign_address_encrypted,
    sign_attestation,
    sign_attestation_encrypted
))]
struct ApiDoc;

fn get_swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
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
