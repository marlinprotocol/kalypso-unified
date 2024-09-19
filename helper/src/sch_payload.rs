use crate::secret_inputs_helpers::{decrypt_ecies, encrypt_ecies};
use anyhow::Context;
use ethers::abi::{encode, Token};
use ethers::core::utils::keccak256;
use ethers::prelude::*;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct SCHPayload {
    request: Vec<u8>,
    response_key: Vec<u8>,
    signature: Vec<u8>,
}

pub trait ToPayload<T> {
    fn to_payload(&self, ecies_priv_key: &Vec<u8>) -> anyhow::Result<T>;
}

impl<T> ToPayload<T> for SCHPayload
where
    T: for<'de> Deserialize<'de>,
{
    fn to_payload(&self, ecies_priv_key: &Vec<u8>) -> anyhow::Result<T> {
        let sch_payload = self.verify()?.decrypt(ecies_priv_key)?;
        let request_str = std::str::from_utf8(&sch_payload.request)
            .context("Failed to convert request to UTF-8 string")?;

        let payload: T = serde_json::from_str(request_str)
            .context("Failed to deserialize request into payload")?;

        Ok(payload)
    }
}

impl SCHPayload {
    fn verify(&self) -> anyhow::Result<Self> {
        let response_key = self.response_key.clone();
        let request = self.request.clone();

        let tokens = vec![
            Token::Bytes(request.clone()),
            Token::Bytes(response_key.clone()),
        ];

        let encoded = encode(&tokens);

        let digest = keccak256(&encoded);

        let signature = match Signature::try_from(&self.signature[..]) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        let recovered_address = match signature.recover(ethers::utils::hash_message(digest)) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        let hash = keccak256(&response_key[1..]);
        let expected_address = H160::from_slice(&hash[12..]).into();

        if recovered_address == expected_address {
            Ok(self.clone())
        } else {
            Err(anyhow::Error::msg(
                "Signature verification failed.".to_string(),
            ))
        }
    }

    fn decrypt(&self, ecies_private_key: &[u8]) -> anyhow::Result<Self> {
        let request = match decrypt_ecies(ecies_private_key, &self.request) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        Ok(SCHPayload {
            request,
            response_key: self.response_key.clone(),
            signature: self.signature.clone(),
        })
    }
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct SCHResponse {
    response: Vec<u8>,
    salt: Vec<u8>,
    signature: Vec<u8>,
}

pub trait ToSchResponse {
    fn to_sch_response(
        &self,
        response_payload: Value,
        enclave_key: Vec<u8>,
    ) -> impl std::future::Future<Output = anyhow::Result<SCHResponse>> + Send;
}

impl ToSchResponse for SCHPayload {
    async fn to_sch_response(
        &self,
        response_payload: Value,
        enclave_key: Vec<u8>,
    ) -> anyhow::Result<SCHResponse> {
        let json_str = serde_json::to_string(&response_payload)
            .context("Failed to serialize response payload to JSON")?;

        let utf8_bytes = json_str.as_bytes().to_vec();

        let encrypted_data = match encrypt_ecies(&self.response_key, &utf8_bytes) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        let salt = {
            let mut salt = vec![0u8; 64];
            OsRng.fill_bytes(&mut salt);
            salt
        };

        let values = vec![
            ethers::abi::Token::Bytes(encrypted_data.clone()),
            ethers::abi::Token::Bytes(salt.clone()),
        ];

        let encoded = ethers::abi::encode(&values);
        let digest = ethers::utils::keccak256(encoded);

        let enclave_key = hex::encode(&enclave_key);
        let enclave_signer = enclave_key.parse::<LocalWallet>().unwrap();

        let signature = match enclave_signer
            .sign_message(ethers::types::H256(digest))
            .await
        {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        Ok(SCHResponse {
            response: encrypted_data,
            salt,
            signature: signature.to_vec(),
        })
    }
}
