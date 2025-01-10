use std::str::FromStr;

use crate::sch_response::EncryptedResponse;
use crate::secret_inputs_helpers::{decrypt_ecies, encrypt_ecies, get_uncompressed_ecies_pubkey};
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

pub async fn prepare_sch_payload<T>(
    data: T,
    server_pub_key: &[u8],
    self_priv_key: &[u8; 32],
    chain_id: &U64,
) -> Result<SCHPayload, Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let json_str = serde_json::to_string(&data)?;

    let utf8_bytes = json_str.as_bytes();

    let encrypted_data = encrypt_ecies(server_pub_key, utf8_bytes)?;

    let request = encrypted_data.clone();

    let response_key = get_uncompressed_ecies_pubkey(self_priv_key);

    let tokens = vec![
        Token::Bytes(encrypted_data.clone()),
        Token::Bytes(response_key.clone()),
    ];
    let encoded = ethers::abi::encode(&tokens);
    let digest = ethers::utils::keccak256(encoded);

    let self_priv_key = hex::encode(self_priv_key);
    let signer = self_priv_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id.as_u64());

    let signature = match signer.sign_message(ethers::types::H256(digest)).await {
        Ok(data) => data,
        Err(err) => {
            log::error!("{}", err);
            return Err("Failed generating signature".into());
        }
    };

    let signature_bytes = ethers::types::Bytes::from_str(&signature.to_string())?;
    let sch_payload = SCHPayload {
        request,
        response_key,
        signature: signature_bytes.to_vec(),
    };

    Ok(sch_payload)
}

pub trait ToPayload<T> {
    fn to_payload(&self, ecies_priv_key: &Vec<u8>) -> anyhow::Result<T>;
}

impl<T> ToPayload<T> for SCHPayload
where
    T: for<'de> Deserialize<'de>,
{
    fn to_payload(&self, receiver_ecies_priv_key: &Vec<u8>) -> anyhow::Result<T> {
        let sch_payload = self.verify()?.decrypt(receiver_ecies_priv_key)?;
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

pub trait GenerateEncryptedResponse {
    fn to_encrypted_response(
        &self,
        response_payload: &Value,
        enclave_key: &Vec<u8>,
    ) -> impl std::future::Future<Output = anyhow::Result<EncryptedResponse>> + Send;
}

impl GenerateEncryptedResponse for SCHPayload {
    async fn to_encrypted_response(
        &self,
        response_payload: &Value,
        senders_priv_key: &Vec<u8>, // used to sign the response for confirmations
    ) -> anyhow::Result<EncryptedResponse> {
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

        let enclave_key = hex::encode(&senders_priv_key);
        let enclave_signer = enclave_key.parse::<LocalWallet>()?;

        let signature = match enclave_signer
            .sign_message(ethers::types::H256(digest))
            .await
        {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        Ok(EncryptedResponse {
            response: encrypted_data,
            salt,
            signature: signature.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use ethers::{core::rand, types::U64};
    use rand::rngs::OsRng;
    use serde_json::{json, Value};

    use crate::{
        sch_request::{GenerateEncryptedResponse, ToPayload},
        sch_response::ToVerifiedAndDecryptedResponse,
    };

    use super::prepare_sch_payload;

    #[tokio::test]
    async fn sim_client_to_enclave_payload_response_transfers() {
        let mut rng = OsRng;
        let client_priv_key = ecies::SecretKey::random(&mut rng);
        let _client_pub_key = ecies::PublicKey::from_secret_key(&client_priv_key).serialize();

        let enclave_priv_key = ecies::SecretKey::random(&mut rng);
        let enclave_pub_key = ecies::PublicKey::from_secret_key(&enclave_priv_key).serialize();

        // any chain id is fine
        let chain_id = U64::from_str_radix("11213", 10).unwrap();

        // some json to send
        let nested_json_to_send = json!({
            "person": {
                "name": "Bob",
                "details": {
                    "age": 25,
                    "hobbies": ["reading", "gaming", "hiking"]
                }
            },
            "active": true
        });

        dbg!(&nested_json_to_send);

        // step 1: any client will prepare payload, provided enclave key is verified
        let payload = prepare_sch_payload(
            nested_json_to_send.clone(),
            &enclave_pub_key,
            &client_priv_key.serialize(),
            &chain_id,
        )
        .await
        .unwrap();

        // step 2: server receives and decrypts it
        let payload_seen_by_server: Value = payload
            .to_payload(&enclave_priv_key.serialize().into())
            .unwrap();

        assert_eq!(payload_seen_by_server, nested_json_to_send);

        // step 3: servers send some data as response to client.
        let plain_response_sent_by_server = json!({
            "message": "I have seen the data",
            "come": [
                "collect", "tokens", "here"
            ]
        });

        let response_to_client = payload
            .to_encrypted_response(
                &plain_response_sent_by_server,
                &enclave_priv_key.serialize().into(),
            )
            .await
            .unwrap();

        // step 4: client receives response and reads it.
        let payload_seen_by_client: Value = response_to_client
            .to_payload(&enclave_pub_key.into(), &client_priv_key.serialize().into())
            .unwrap();

        assert_eq!(payload_seen_by_client, plain_response_sent_by_server);
    }
}
