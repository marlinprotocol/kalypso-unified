pub mod models;
use async_trait::async_trait;
use ethers::core::k256::sha2::{Digest, Sha256};
use reqwest::{Client, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

pub enum RequestType<T: Serialize> {
    GET,
    POST(T),
}

pub struct Request<T: Serialize, R> {
    pub request_type: RequestType<T>,
    pub service_endpoint: String,
    pub expected_status_code: reqwest::StatusCode,
    pub info: String,

    pub _marker: std::marker::PhantomData<R>,
}

impl<T, R> Request<T, R>
where
    T: Serialize,
{
    fn hash_payload(&self) -> Option<String> {
        if let RequestType::POST(ref payload) = self.request_type {
            // Serialize the payload to JSON and compute its hash
            let serialized_payload = serde_json::to_string(payload).ok()?;
            let mut hasher = Sha256::new();
            hasher.update(serialized_payload);
            Some(format!("{:x}", hasher.finalize()))
        } else {
            None
        }
    }
}

#[async_trait]
pub trait Run {
    async fn execute(&self, server_url: &str) -> Option<(StatusCode, bool, String)>;
    fn name(&self) -> String;
    fn request_info(&self) -> String;
    fn expected_status_code(&self) -> reqwest::StatusCode;
    fn hash_payload(&self) -> Option<String>;
    fn info(&self) -> String;
}

#[async_trait]
impl<T, R> Run for Request<T, R>
where
    T: Serialize + Send + Sync,
    R: Serialize + DeserializeOwned + Send + Sync,
{
    fn info(&self) -> String {
        self.info.clone()
    }

    fn hash_payload(&self) -> Option<String> {
        self.hash_payload()
    }

    fn expected_status_code(&self) -> reqwest::StatusCode {
        self.expected_status_code
    }

    fn request_info(&self) -> String {
        match &self.request_type {
            RequestType::GET => format!(
                "GET {} - Expected Status Code: {}",
                self.service_endpoint, self.expected_status_code
            ),
            RequestType::POST(_) => format!(
                "POST {} - Expected Status Code: {}",
                self.service_endpoint, self.expected_status_code
            ),
        }
    }

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
            println!("\nService: {}", service.name());
            println!("Info: {}", service.info());

            // Print the payload hash for POST requests
            if let Some(hash) = service.hash_payload() {
                println!("Payload Hash: {}", hash);
            }

            match status_code {
                Some((code, is_type_ok, expected_type_name)) => {
                    println!("Status Code: {}", code);

                    if !is_type_ok {
                        println!("Mismatch Detected:");
                        println!("  Expected Response Type: {}", expected_type_name);
                    }

                    if !is_type_ok || code != service.expected_status_code() {
                        println!("Expectation:");
                        println!("  {}", service.request_info());
                    }
                }
                None => {
                    println!("Status Code: Service Error (Fix Needed in API)");
                    println!("Expectation:");
                    println!("  {}", service.request_info());
                }
            }

            println!();
        }
    }
}

// ------------------------------------------- ###### -------------------------------------------//
pub fn get_test_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/api/test".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Checks server reach".into(),
    }
}

pub fn get_benchmark_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/api/benchmark".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Generate benchmark info for the prover".into(),
    }
}

pub fn generate_proof_request<R>(
    input_payload: Option<models::InputPayload>,
    expected_status_code: StatusCode,
    info: String,
) -> Request<models::InputPayload, R> {
    Request {
        request_type: RequestType::POST(input_payload.unwrap_or_else(|| {
            models::InputPayload::from_plain_secrets("public".into(), "secrets".into())
        })),
        service_endpoint: "/api/generateProof".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code,
        info,
    }
}
