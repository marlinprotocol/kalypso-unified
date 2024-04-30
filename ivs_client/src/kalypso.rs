use std::str::FromStr;

use ecies::{PublicKey, SecretKey};
use ethers::{abi::Address, prelude::*};
use serde_bytes::ByteBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::model::{
    ApiGenerationResponse, ApiKeyFile, IvsConfig, IvsPublicKeys, SignAttestation, VerifyApiResponse,
};

//generating API key
pub async fn generate_api_key() -> Result<ApiGenerationResponse, Box<dyn std::error::Error>> {
    //Checking if the config folder is already generated, if not creating a new one
    let folder_path = "../ivs_config";
    if fs::metadata(&folder_path).await.is_ok() {
        log::info!("ivs_config folder already exists!");
    } else {
        fs::create_dir(&folder_path)
            .await
            .expect("Unable to create new folder");
    }

    //Checking if the API key was already generated
    if fs::metadata("../ivs_config/api_key.json").await.is_ok() {
        log::info!("api_key.json already exists!");
        return Ok(ApiGenerationResponse {
            api_key: "".to_string(),
            status: false,
            message: "API was already generated. It cannot be generated again".to_string(),
        });
    }

    let api_key = Uuid::new_v4();
    let api_key_file = ApiKeyFile {
        api_key: api_key.to_string(),
    };
    let json_string = serde_json::to_string(&api_key_file)?;
    let mut file = File::create("../ivs_config/api_key.json").await?;
    tokio::io::AsyncWriteExt::write_all(&mut file, json_string.as_bytes()).await?;
    Ok(ApiGenerationResponse {
        api_key: api_key.to_string(),
        status: true,
        message: "API key generated. Please save this API key somewhere safe, it cannot be generated again.".to_string(),
    })
}

//Verify api key
pub async fn verify_api_key(
    request_api_key: &str,
) -> Result<VerifyApiResponse, Box<dyn std::error::Error>> {
    //Checking if the API key was already generated
    if fs::metadata("../ivs_config/api_key.json").await.is_err() {
        return Ok(VerifyApiResponse {
            status: false,
            message: "api_key is not generated".to_string(),
        });
    }
    let file = File::open("../ivs_config/api_key.json").await?;
    let mut buf_reader = tokio::io::BufReader::new(file);

    let mut content = String::new();
    buf_reader.read_to_string(&mut content).await?;

    let api_key_data: ApiKeyFile = serde_json::from_str(&content)?;
    if api_key_data.api_key != request_api_key {
        return Ok(VerifyApiResponse {
            status: false,
            message: "Authentication failed, wrong API key provided".to_string(),
        });
    }
    Ok(VerifyApiResponse {
        status: true,
        message: "authenticated".to_string(),
    })
}

// Get ECIES public key
pub async fn get_public_keys_for_ivs() -> Result<IvsPublicKeys, Box<dyn std::error::Error>> {
    let ivs_private_key = read_ivs_config_file().await?.secp256k1_private_key;
    let ivs_public_key = hex::encode(ivs_private_key.parse::<LocalWallet>()?.address());
    let formated_public_key = "0x".to_string() + &ivs_public_key;

    let private_key = hex::decode(ivs_private_key).unwrap();
    let private_key: &[u8; 32] = private_key.as_slice().try_into().unwrap();
    let sk = SecretKey::parse(private_key).unwrap();
    let public_key = PublicKey::from_secret_key(&sk);
    let public_key = public_key.serialize_compressed();
    let encoded_key = hex::encode(public_key);
    let formated_ecies_public_key = "0x".to_string() + &encoded_key;

    let public_keys = IvsPublicKeys {
        ivs_public_key: formated_public_key,
        ivs_ecies_public_key: formated_ecies_public_key,
    };

    Ok(public_keys)
}

//Generate Config file
pub async fn generate_ivs_config_file() -> Result<(), Box<dyn std::error::Error>> {
    // Create config_file_folder
    let folder_path = "../ivs_config";
    if fs::metadata(&folder_path).await.is_ok() {
        log::info!("ivs_config folder already exists!");
    } else {
        fs::create_dir(&folder_path)
            .await
            .expect("Unable to create new folder");
    }

    //Structure data
    let config_file_path = folder_path.to_string() + "/ivs_config.json";

    // Get enclave private key
    let key_path = "app/secp.sec";
    let mut file = File::open(key_path).await?;
    let mut priv_key = [0; 32];
    file.read_exact(&mut priv_key).await?;

    let ivs_key = ByteBuf::from(priv_key);

    let ivs_config_file = IvsConfig {
        secp256k1_private_key: hex::encode(ivs_key),
    };

    //Generating the json config file
    let json_string = serde_json::to_string(&ivs_config_file)?;
    let mut file = File::create(config_file_path).await?;
    tokio::io::AsyncWriteExt::write_all(&mut file, json_string.as_bytes()).await?;
    Ok(())
}

//Read the runtime config file
pub async fn read_ivs_config_file() -> Result<IvsConfig, Box<dyn std::error::Error>> {
    let file = File::open("../ivs_config/ivs_config.json").await?;
    let mut buf_reader = tokio::io::BufReader::new(file);

    let mut content = String::new();
    buf_reader.read_to_string(&mut content).await?;

    let ivs_config_file: IvsConfig = serde_json::from_str(&content)?;
    Ok(ivs_config_file)
}

pub async fn sign_addy(address: &str) -> Result<Signature, Box<dyn std::error::Error>> {
    //Using the enclave secp secret for ecies private key
    let read_secp_private_key = fs::read("/app/secp.sec").await?;
    let secp_private_key = secp256k1::SecretKey::from_slice(&read_secp_private_key)
        .unwrap()
        .display_secret()
        .to_string();
    let signer = secp_private_key.parse::<LocalWallet>().unwrap();
    let values = vec![ethers::abi::Token::Address(Address::from_str(address)?)];
    let encoded = ethers::abi::encode(&values);
    let digest = ethers::utils::keccak256(encoded);
    let signature = signer.sign_message(ethers::types::H256(digest)).await?;
    Ok(signature)
}

pub async fn sign_attest(
    attestation: SignAttestation,
) -> Result<Signature, Box<dyn std::error::Error>> {
    // Using enclave private key for signature
    let secp_file = fs::read("/app/secp.sec").await?;
    let secp_private_key = secp256k1::SecretKey::from_slice(&secp_file)
        .unwrap()
        .display_secret()
        .to_string();
    fetch_signed_attestation(attestation, secp_private_key).await
}

pub async fn fetch_signed_attestation(
    attestation: SignAttestation,
    private_key: String,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let signer = private_key.parse::<LocalWallet>().unwrap();
    let attestation_bytes = attestation.attestation.unwrap();
    let attestation_string: Vec<&str> = attestation_bytes.split("x").collect();
    let attestation_decoded = hex::decode(attestation_string[1]).unwrap();
    let address = attestation.address.unwrap();
    let values = vec![
        ethers::abi::Token::Bytes(attestation_decoded),
        ethers::abi::Token::Address(Address::from_str(&address)?),
    ];

    let encoded = ethers::abi::encode(&values);
    let digest = ethers::utils::keccak256(encoded);
    let signature = signer.sign_message(ethers::types::H256(digest)).await?;
    log::info!("Signature {:?}", signature);
    Ok(signature)
}

#[cfg(test)]
mod tests {

    use super::fetch_signed_attestation;
    use crate::model::SignAttestation;
    use ethers::types::BigEndianHash;
    use serde_json::json;

    #[tokio::test]
    async fn check_sigs() {
        let ivs_attestation = "0x0000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000002a0000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000007d00000000000000000000000000000000000000000000000000000018deedd1b9f00000000000000000000000000000000000000000000000000000000000000419db220cfc95b1433119ffea7e4d71563cae9044306231ee93ede8282c74be1bd75b4c61dff7fd0cb1462ff27437ccf9473aeeed4d93e81f084d212be0188d31e1c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000409cd2daed35d622413c84a11f73365e1dd1da20e5de5aacb1e3a2918bcf8bc6d375deb5108971e8aeb41e4fc2f0e29c9502933cec45952d3f215144809356703c000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000";
        let expected_signature = "0x08b0a62a11f8d000ce2fe6f0dbab738cd2bde5757afee90b2ee7731a64f8ebb57e1b229065c7ae1c526547eb4eaa3dff5f8e33f84d06c1cccd1f4c083a996d7c1b";
        let recovery_address = "0x79D4B2d6a37078A221b361eA74A8AE7eB03DA3E4";
        let private_key = "0x7c802eab2cd5481827c227b22a4db1216ade08f2152a886cdfe9889fa0719c53";
        let address_to_sign = "0xFABB0ac9d68B0B445fB7357272Ff202C5651694a";

        let obtained_signature = fetch_signed_attestation(
            SignAttestation::new(ivs_attestation, address_to_sign),
            private_key.to_string(),
        )
        .await
        .unwrap();

        let json_signature = json!({
            "r": ethers::types::H256::from_uint(&obtained_signature.r),
            "s": ethers::types::H256::from_uint(&obtained_signature.s),
            "v": obtained_signature.v
        });

        let obtained_signature = "0x".to_owned() + &obtained_signature.to_string();

        dbg!(&obtained_signature);
        dbg!(&expected_signature);
        dbg!(json_signature);

        assert!(obtained_signature == expected_signature);
    }
}
