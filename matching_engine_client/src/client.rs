use actix_web::web::Data;
use actix_web::{App, HttpServer};
use std::sync::{Arc, Mutex};

use crate::handler;

pub struct MatchingEngineClient {
    enclave_key: Arc<Mutex<Vec<u8>>>,
    port: u16,
}

impl MatchingEngineClient {
    pub fn new(enclave_key: String, port: u16) -> Self {
        let enclave_key = hex::decode(enclave_key).unwrap();
        let enclave_key = Arc::new(Mutex::new(enclave_key));
        MatchingEngineClient { enclave_key, port }
    }

    pub async fn start(self) -> anyhow::Result<()> {
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(self.enclave_key.clone()))
                .configure(handler::routes)
        })
        .bind(("0.0.0.0", self.port))
        .unwrap_or_else(|_| panic!("Can not bind to {}", &self.port))
        .run()
        .await
        .unwrap();

        Ok(())
    }
}
