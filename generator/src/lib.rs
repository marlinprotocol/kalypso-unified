pub mod models;
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

pub enum RequestType<T> {
    GET,
    POST(T),
}

pub struct Request<T, R> {
    pub request_type: RequestType<T>,
    pub service_endpoint: String,

    pub _marker: std::marker::PhantomData<R>,
}

#[async_trait]
pub trait Run {
    async fn execute(&self, server_url: &str) -> Option<(StatusCode, bool, String)>;
    fn name(&self) -> String;
}

#[async_trait]
impl<T, R> Run for Request<T, R>
where
    T: Serialize + Send + Sync,
    R: DeserializeOwned + Send + Sync,
{
    async fn execute(&self, server_url: &str) -> Option<(StatusCode, bool, String)> {
        let client = Client::new();
        let url = format!("{}{}", server_url, self.service_endpoint);

        let response = match &self.request_type {
            RequestType::GET => client.get(&url).send().await,
            RequestType::POST(payload) => client.post(&url).json(payload).send().await,
        };

        match response {
            Ok(data) => {
                let status = data.status();
                match data.json::<R>().await {
                    Ok(_) => Some((status, true, "".into())),
                    Err(_) => Some((status, false, std::any::type_name::<R>().into())),
                }
            }
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
                Some((code, is_type_ok, expected_type_name)) => {
                    if is_type_ok {
                        println!("Service: {}. Status Code: {}", service.name(), code);
                    } else {
                        println!(
                            "Service: {}. Status Code: {}. But Expected Response Type: {}",
                            service.name(),
                            code,
                            expected_type_name
                        );
                    }
                }
                None => {
                    println!(
                        "Service: {}, Status Code: (Service Error, Fixed Needed in API)",
                        service.name()
                    );
                }
            }
        }
    }
}

// ------------------------------------------- ###### -------------------------------------------//
pub fn get_test_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/api/test".into(),
        _marker: std::marker::PhantomData::<R>,
    }
}

pub fn get_benchmark_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/api/benchmark".into(),
        _marker: std::marker::PhantomData::<R>,
    }
}

pub fn generate_proof_request<R>(
    input_payload: Option<models::InputPayload>,
) -> Request<models::InputPayload, R> {
    Request {
        request_type: RequestType::POST(input_payload.unwrap_or_else(|| models::InputPayload {
            public: "public".into(),
            secrets: Some("secrets".into()),
        })),
        service_endpoint: "/api/generateProof".into(),
        _marker: std::marker::PhantomData::<R>,
    }
}
