use anyhow::Context;
use ethers::abi::Token;
use ethers::core::utils::keccak256;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};

use crate::secret_inputs_helpers::decrypt_ecies;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct EncryptedResponse {
    pub response: Vec<u8>,
    pub salt: Vec<u8>,
    pub signature: Vec<u8>,
}

impl EncryptedResponse {
    fn verify(&self, sender_pubkey_key: &Vec<u8>) -> anyhow::Result<Self> {
        let response = self.response.clone();
        let salt = self.salt.clone();

        let tokens = vec![Token::Bytes(response.clone()), Token::Bytes(salt.clone())];

        let encoded = ethers::abi::encode(&tokens);

        let digest = ethers::utils::keccak256(&encoded);

        let signature = match Signature::try_from(&self.signature[..]) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        let recovered_address = match signature.recover(ethers::utils::hash_message(digest)) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        let hash = keccak256(&sender_pubkey_key[1..]);
        let expected_address = H160::from_slice(&hash[12..]).into();

        if recovered_address == expected_address {
            Ok(self.clone())
        } else {
            Err(anyhow::Error::msg(
                "Signature verification failed.".to_string(),
            ))
        }
    }

    fn decrypt(&self, receiver_ecies_private_key: &[u8]) -> anyhow::Result<Vec<u8>> {
        let decrypted_response = match decrypt_ecies(receiver_ecies_private_key, &self.response) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::Error::msg(e.to_string())),
        };

        Ok(decrypted_response)
    }
}

pub trait ToVerifiedAndDecryptedResponse<T> {
    fn to_payload(&self, senders_pub_key: &Vec<u8>, ecies_priv_key: &Vec<u8>) -> anyhow::Result<T>;
}

impl<T> ToVerifiedAndDecryptedResponse<T> for EncryptedResponse
where
    T: for<'de> Deserialize<'de>,
{
    fn to_payload(
        &self,
        senders_pub_key: &Vec<u8>,
        receiver_ecies_priv_key: &Vec<u8>,
    ) -> anyhow::Result<T> {
        let result = self
            .verify(senders_pub_key)?
            .decrypt(receiver_ecies_priv_key)?;
        let request_str =
            std::str::from_utf8(&result).context("Failed to convert request to UTF-8 string")?;

        let payload: T = serde_json::from_str(request_str)
            .context("Failed to deserialize request into payload")?;

        Ok(payload)
    }
}
