pub mod models;
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::Serialize;

pub enum RequestType<T> {
    GET,
    POST(T),
}

pub struct Request<T> {
    pub request_type: RequestType<T>,
    pub service_endpoint: String,
}

#[async_trait]
pub trait Run {
    async fn execute(&self, server_url: &str) -> Option<StatusCode>;
    fn name(&self) -> String;
}

#[async_trait]
impl<T> Run for Request<T>
where
    T: Serialize + Send + Sync,
{
    async fn execute(&self, server_url: &str) -> Option<StatusCode> {
        let client = Client::new();
        let url = format!("{}{}", server_url, self.service_endpoint);

        let response = match &self.request_type {
            RequestType::GET => client.get(&url).send().await,
            RequestType::POST(payload) => client.post(&url).json(payload).send().await,
        };

        match response {
            Ok(data) => Some(data.status()),
            Err(_) => None,
        }
    }

    fn name(&self) -> String {
        match self.request_type {
            RequestType::GET => {
                format!("GET {}", self.service_endpoint.clone())
            }
            RequestType::POST(_) => {
                format!("POST {}", self.service_endpoint.clone())
            }
        }
    }
}

pub struct ServiceChecker {
    pub server_url: String,
    pub services: Vec<Box<dyn Run>>,
}

impl ServiceChecker {
    pub async fn check_all_services(&self) {
        for service in &self.services {
            let status_code = service.execute(&self.server_url).await;
            match status_code {
                Some(code) => {
                    println!("Service: {}. Status Code: {}", service.name(), code);
                }
                None => {
                    println!("Service: {}, Status Code: (Service Error)", service.name());
                }
            }
        }
    }
}

// ------------------------------------------- ###### -------------------------------------------//
pub fn get_test_request() -> Request<()> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/api/test".into(),
    }
}

pub fn get_benchmark_request() -> Request<()> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/api/benchmark".into(),
    }
}

pub fn generate_proof_request(
    input_payload: Option<models::InputPayload>,
) -> Request<models::InputPayload> {
    Request {
        request_type: RequestType::POST(input_payload.unwrap_or_else(|| models::InputPayload {
            public: "public".into(),
            secrets: Some("secrets".into()),
        })),
        service_endpoint: "/api/generateProof".into(),
    }
}
