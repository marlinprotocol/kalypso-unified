pub mod models;

use generator_models::Request;
use generator_models::RequestType;

pub fn get_test_request() -> Request<()> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/api/test".into(),
    }
}

pub fn generate_input_request(
    secret_input_paylod: Option<models::InputPayload>,
) -> Request<models::InputPayload> {
    Request {
        request_type: RequestType::POST(secret_input_paylod.unwrap_or_else(|| {
            models::InputPayload {
                secrets: Some("secret".into()),
                public: "public".into(),
            }
        })),
        service_endpoint: "/api/checkInput".into(),
    }
}

pub fn generate_invalid_input_request(
    secret_input_paylod: Option<models::InvalidInputPayload>,
) -> Request<models::InvalidInputPayload> {
    Request {
        request_type: RequestType::POST(secret_input_paylod.unwrap_or_else(|| {
            models::InvalidInputPayload {
                ask_id: "1".into(),
                public: "public".into(),
                secrets: Some("secrets".into()),
            }
        })),
        service_endpoint: "/api/checkInput".into(),
    }
}
