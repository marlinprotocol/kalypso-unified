pub mod models;

use generator_models::Request;
use generator_models::RequestType;
use reqwest::StatusCode;

pub fn get_test_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/api/test".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Checks server reach".into(),
    }
}

pub fn generate_input_request<R>(
    secret_input_paylod: Option<generator_models::models::InputPayload>,
    expected_status_code: StatusCode,
    info: String,
) -> Request<generator_models::models::InputPayload, R> {
    Request {
        request_type: RequestType::POST(secret_input_paylod.unwrap_or_else(|| {
            generator_models::models::InputPayload::from_plain_secrets(
                "public".into(),
                "secrets".into(),
            )
        })),
        service_endpoint: "/api/checkInput".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code,
        info,
    }
}

pub fn generate_invalid_input_request<R>(
    secret_input_paylod: Option<models::InvalidInputPayload>,
    expected_status_code: StatusCode,
    info: String,
) -> Request<models::InvalidInputPayload, R> {
    Request {
        request_type: RequestType::POST(secret_input_paylod.unwrap_or_else(|| {
            models::InvalidInputPayload::from_plain_secrets(
                "1".into(),
                "public".into(),
                "secrets".into(),
            )
        })),
        service_endpoint: "/api/getAttestationForInvalidInputs".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code,
        info,
    }
}

pub fn generate_check_encrypted_inputs_request<R>(
    encrypted_payload: Option<models::EncryptedInputPayload>,
    expected_status_code: StatusCode,
    info: String,
) -> Request<models::EncryptedInputPayload, R> {
    Request {
        request_type: RequestType::POST(encrypted_payload.unwrap_or_else(|| {
            models::EncryptedInputPayload {
                acl: "acl".into(),
                encrypted_secrets: "encrypted_secrets".into(),
                me_decryption_url: "matching_engine_decryption_url".into(),
                market_id: "market_id".into(),
                public_inputs: Some(vec![1, 2, 3, 4]),
            }
        })),
        service_endpoint: "/api/checkEncryptedInputs".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code,
        info,
    }
}

pub fn generate_verify_inputs_and_proof_request<R>(
    verify_input_and_secret_payload: Option<models::VerifyInputsAndProof>,
    expected_status_code: StatusCode,
    info: String,
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
        expected_status_code,
        info,
    }
}

pub fn generate_signed_inputs_and_proof_request<R>(
    verify_input_and_secret_payload: Option<models::SignInputsAndProofForNonConfidentialInput>,
    expected_status_code: StatusCode,
    info: String,
) -> Request<models::SignInputsAndProofForNonConfidentialInput, R> {
    Request {
        request_type: RequestType::POST(verify_input_and_secret_payload.unwrap_or_else(|| {
            models::SignInputsAndProofForNonConfidentialInput {
                proof: "proof".into(),
                public_input: "public_inputs".into(),
            }
        })),
        service_endpoint: "/api/signInputsAndProofForNonConfidentialInputs".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code,
        info,
    }
}
