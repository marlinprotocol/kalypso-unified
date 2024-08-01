pub mod models;

use generator_models::Request;
use generator_models::RequestType;

pub fn get_test_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/api/test".into(),
        _marker: std::marker::PhantomData::<R>,
    }
}

pub fn generate_input_request<R>(
    secret_input_paylod: Option<models::InputPayload>,
) -> Request<models::InputPayload, R> {
    Request {
        request_type: RequestType::POST(secret_input_paylod.unwrap_or_else(|| {
            models::InputPayload {
                secrets: Some("secret".into()),
                public: "public".into(),
            }
        })),
        service_endpoint: "/api/checkInput".into(),
        _marker: std::marker::PhantomData::<R>,
    }
}

pub fn generate_invalid_input_request<R>(
    secret_input_paylod: Option<models::InvalidInputPayload>,
) -> Request<models::InvalidInputPayload, R> {
    Request {
        request_type: RequestType::POST(secret_input_paylod.unwrap_or_else(|| {
            models::InvalidInputPayload {
                ask_id: "1".into(),
                public: "public".into(),
                secrets: Some("secrets".into()),
            }
        })),
        service_endpoint: "/api/getAttestationForInvalidInputs".into(),
        _marker: std::marker::PhantomData::<R>,
    }
}

pub fn generate_check_encrypted_inputs_request<R>(
    encrypted_payload: Option<models::EncryptedInputPayload>,
) -> Request<models::EncryptedInputPayload, R> {
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
        _marker: std::marker::PhantomData::<R>,
    }
}

pub fn generate_verify_inputs_and_proof_request<R>(
    verify_input_and_secret_payload: Option<models::VerifyInputsAndProof>,
) -> Request<models::VerifyInputsAndProof, R> {
    Request {
        request_type: RequestType::POST(verify_input_and_secret_payload.unwrap_or_else(|| {
            models::VerifyInputsAndProof {
                proof: "proof".into(),
                public_input: Some("public_inputs".into()),
                private_input: vec![1, 2, 3, 4].into(),
            }
        })),
        service_endpoint: "/api/verifyInputsAndProof".into(),
        _marker: std::marker::PhantomData::<R>,
    }
}
