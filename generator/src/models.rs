use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InputPayload {
    public: Vec<u8>,
    secrets: Option<Secrets>,
}

impl InputPayload {
    pub fn only_public_inputs(public: Vec<u8>) -> Self {
        InputPayload {
            public,
            secrets: None,
        }
    }

    pub fn from_plain_secrets(public: Vec<u8>, secrets: Vec<u8>) -> Self {
        InputPayload {
            public,
            secrets: Some(Secrets::PlainSecrets(secrets)),
        }
    }

    pub fn from_encrypted_secrets(public: Vec<u8>, encrypted_data: Vec<u8>, acl: Vec<u8>) -> Self {
        InputPayload {
            public,
            secrets: Some(Secrets::EncryptedSecrets(EncryptedSecret {
                encrypted_data,
                acl,
            })),
        }
    }

    pub fn get_public(&self) -> Vec<u8> {
        self.public.clone()
    }

    pub fn get_plain_secrets(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match self.secrets.clone().unwrap() {
            Secrets::PlainSecrets(data) => Ok(data),
            Secrets::EncryptedSecrets(_) => return Err("Fetching Plain texts not supported".into()),
        }
    }

    pub fn get_plain_secrets_from_encrypted_secrets(
        &self,
        decryption_key: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match self.secrets.clone().unwrap() {
            Secrets::PlainSecrets(_) => return Err("Can't decrypt the plain text".into()),
            Secrets::EncryptedSecrets(data) => {
                return kalypso_helper::secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
                    &data.encrypted_data,
                    &data.acl,
                    &decryption_key,
                    U256::max_value(),
                )
            }
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
enum Secrets {
    PlainSecrets(Vec<u8>),
    EncryptedSecrets(EncryptedSecret),
}

#[derive(Serialize, Debug, Deserialize, Clone)]
struct EncryptedSecret {
    encrypted_data: Vec<u8>,
    acl: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct GenerateProofResponse {
    pub proof: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct TestResponse {
    pub data: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct BenchmarkResponse {
    pub data: String,
    pub time_in_ms: u128,
}
