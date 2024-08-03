use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InvalidInputPayload {
    ask_id: ethers::types::U256,
    public: Vec<u8>,
    secrets: Option<Secrets>,
}

impl InvalidInputPayload {
    pub fn only_ask_id(&self) -> U256 {
        self.ask_id
    }

    pub fn only_public_inputs(ask_id: U256, public: Vec<u8>) -> Self {
        InvalidInputPayload {
            ask_id,
            public,
            secrets: None,
        }
    }

    pub fn from_plain_secrets(ask_id: U256, public: Vec<u8>, secrets: Vec<u8>) -> Self {
        InvalidInputPayload {
            ask_id,
            public,
            secrets: Some(Secrets::PlainSecrets(secrets)),
        }
    }

    pub fn encrypted_secrets(
        ask_id: U256,
        public: Vec<u8>,
        encrypted_data: Vec<u8>,
        acl: Vec<u8>,
    ) -> Self {
        InvalidInputPayload {
            ask_id,
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
                    self.generate_random_u256(),
                )
            }
        }
    }

    fn generate_random_u256(&self) -> U256 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut random_bytes = [0u8; 32];
        rng.fill(&mut random_bytes);
        U256::from_big_endian(&random_bytes)
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
pub struct CheckInputResponse {
    pub valid: bool,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct EncryptedInputPayload {
    pub acl: Vec<u8>,
    pub encrypted_secrets: Vec<u8>,
    pub me_decryption_url: String,
    pub market_id: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct VerifyInputsAndProof {
    pub public_input: Option<Vec<u8>>,
    pub private_input: Option<Vec<u8>>,
    pub proof: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct VerifyInputAndProofResponse {
    pub is_input_and_proof_valid: bool,
}
