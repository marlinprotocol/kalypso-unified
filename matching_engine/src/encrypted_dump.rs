use crate::{dump::Dump, MatchingEngineConfig};
use ethers::prelude::*;
use kalypso_helper::secret_inputs_helpers;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct EncryptedDump {
    pub encrypted: Vec<u8>,
    pub acls: Vec<Vec<u8>>,
}

impl ToPlainDump for EncryptedDump {
    fn get_dump(&self) -> Result<Dump, Box<dyn std::error::Error>> {
        // Load matching engine configuration
        let config_path = "../matching_engine_config/matching_engine_config.json".to_string();
        let alt_config_path = "./matching_engine_config/matching_engine_config.json".to_string();
        let file_content =
            fs::read_to_string(config_path).or_else(|_| fs::read_to_string(alt_config_path))?;
        let config: MatchingEngineConfig = serde_json::from_str(&file_content)?;
        let me_private_key_vec = hex::decode(config.matching_engine_key)?;

        let encrypted_dump = self.clone().encrypted;
        let mut decrypted_dump: Dump = Dump::default();

        // let mut counter = 0;
        for acl in self.clone().acls {
            // counter = counter + 1;
            // println!("Loop {:?}, ACL {:?}", counter, acl);
            let decrypted = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
                &encrypted_dump,
                &acl,
                &me_private_key_vec,
                Some(U256::from(1)),
            );
            match decrypted {
                Ok(data) => {
                    // println!("OK, Loop {:?}", counter);
                    decrypted_dump = serde_json::from_slice(&data)?;
                    break;
                }
                Err(e) => {
                    // println!("Err, Loop {:?}", counter);
                    log::warn!("Error: ecies key mismatch {:?}", e);
                    continue;
                }
            }
        }
        Ok(decrypted_dump)
    }
}

pub trait ToPlainDump {
    fn get_dump(&self) -> Result<Dump, Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use crate::{dump::ToEncryptedDump, encrypted_dump::ToPlainDump};

    use super::Dump;
    use serde_json;

    #[tokio::test]
    async fn test_encryption_and_decrytion() {
        // create default dump
        let dump = Dump::default();

        let encrypted_dump = dump.create_encrypted_dump().await.unwrap();
        let decrypted_dump = encrypted_dump.get_dump().unwrap();
        let decrypted_dump_str = serde_json::to_string(&decrypted_dump).unwrap();
        dbg!(decrypted_dump_str);
    }
}
