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
        service_endpoint: "/api/getAttestationForInvalidInputs".into(),
    }
}

pub fn generate_check_encrypted_inputs_request(
    encrypted_payload: Option<models::EncryptedInputPayload>,
) -> Request<models::EncryptedInputPayload> {
    Request {
        request_type: RequestType::POST(encrypted_payload.unwrap_or_else(|| {
            models::EncryptedInputPayload {
                acl: "acl".into(),
                encrypted_secrets: "encrypted_secrets".into(),
                me_decryption_url: "matching_engine_decryption_url".into(),
                market_id: "market_id".into(),
            }
        })),
        service_endpoint: "/api/checkEncryptedInputs".into(),
    }
}

pub fn generate_verify_inputs_and_secrets_request(
    verify_input_and_secret_payload: Option<models::VerifyInputsAndSecrets>,
) -> Request<models::VerifyInputsAndSecrets> {
    Request {
        request_type: RequestType::POST(verify_input_and_secret_payload.unwrap_or_else(|| {
            models::VerifyInputsAndSecrets {
                public: "public".into(),
                secrets: Some("secrets".into()),
            }
        })),
        service_endpoint: "/api/verifyInputsAndSecrets".into(),
    }
}
