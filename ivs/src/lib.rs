pub mod models;
use ethers::core::types::U256;

use generator_models::Request;
use generator_models::RequestType;
use models::EncryptedInputPayload;

pub fn get_test_request() -> Request<()> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/api/test".into(),
    }
}

pub fn generate_check_input_request(
    secret_input_paylod: Option<models::SecretInputPayload>,
) -> Request<models::SecretInputPayload> {
    Request {
        request_type: RequestType::POST(secret_input_paylod.unwrap_or_else(|| {
            models::SecretInputPayload {
                secrets: "secret".into(),
            }
        })),
        service_endpoint: "/api/checkInput".into(),
    }
}

pub fn generate_get_attestation_for_invalid_input_request(
    ask_payload: Option<models::AskPayload>,
) -> Request<models::AskPayload> {
    Request {
        request_type: RequestType::POST(ask_payload.unwrap_or_else(|| models::AskPayload {
            ask: bindings::shared_types::Ask {
                market_id: U256::from_dec_str("1").unwrap(),
                reward: U256::from(1),
                expiry: U256::from(1),
                time_taken_for_proof_generation: U256::from(1),
                deadline: U256::from(1),
                refund_address: "0000dead0000dead0000dead0000dead0000dead".parse().unwrap(),
                prover_data: vec![1, 2, 3, 4].into(),
            },
            encrypted_secret: "encrypted secret".into(),
            acl: "acl".into(),
            ask_id: 1,
        })),
        service_endpoint: "/api/getAttestationForInvalidInputs".into(),
    }
}

pub fn generate_check_encrypted_inputs_request(
    encrypted_payload: Option<EncryptedInputPayload>,
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
