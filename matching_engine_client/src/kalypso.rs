use std::str::FromStr;
use std::sync::Arc;

use actix_web::web::Json;
use ecies::{PublicKey, SecretKey};
use ethers::prelude::*;
use ethers::providers::Provider;
use serde_bytes::ByteBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::model::{
    ApiGenerationResponse, ApiKeyFile, MatchingEngineConfig, MatchingEngineConfigSetupRequestBody,
    SignAttestation, UpdateMatchingEngineConfig, ValidationResponse, VerifyApiResponse,
};

macro_rules! update_field {
    ($config:expr, $input:expr, $field:ident) => {
        if let Some(new_value) = &$input.$field {
            $config.$field = new_value.to_string();
        }
    };
}

//generating API key
pub async fn generate_api_key() -> Result<ApiGenerationResponse, Box<dyn std::error::Error>> {
    //Checking if the config folder is already generated, if not creating a new one
    let folder_path = "../matching_engine_config";
    if fs::metadata(&folder_path).await.is_ok() {
        log::info!("matching_engine_config folder already exists!");
    } else {
        fs::create_dir(&folder_path)
            .await
            .expect("Unable to create new folder");
    }

    //Checking if the API key was already generated
    if fs::metadata("../matching_engine_config/api_key.json")
        .await
        .is_ok()
    {
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
    let mut file = File::create("../matching_engine_config/api_key.json").await?;
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
    if fs::metadata("../matching_engine_config/api_key.json")
        .await
        .is_err()
    {
        return Ok(VerifyApiResponse {
            status: false,
            message: "api_key is not generated".to_string(),
        });
    }
    let file = File::open("../matching_engine_config/api_key.json").await?;
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

//Generate matching engine config file
pub async fn generate_matching_engine_config_file(
    me_config_body: &MatchingEngineConfigSetupRequestBody,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create config_file_folder
    let folder_path = "../matching_engine_config";
    if fs::metadata(&folder_path).await.is_ok() {
        log::info!("matching_engine_config folder already exists!");
    } else {
        fs::create_dir(&folder_path)
            .await
            .expect("Unable to create new folder");
    }

    // Generate a random private key
    // Get enclave private key
    let key_path = "app/secp.sec";
    let mut file = File::open(key_path).await?;
    let mut priv_key = [0u8; 32];
    file.read_exact(&mut priv_key).await?;

    let matching_engine_key = ByteBuf::from(priv_key);

    //Structure data
    let config_file_path = folder_path.to_string() + "/matching_engine_config.json";

    let matching_engine_config_file = MatchingEngineConfig {
        chain_id: me_config_body.chain_id.as_ref().unwrap().to_string(),
        start_block: me_config_body.start_block.as_ref().unwrap().to_string(),
        generator_registry: me_config_body
            .generator_registry
            .as_ref()
            .unwrap()
            .to_string(),
        matching_engine_key: hex::encode(matching_engine_key),
        proof_market_place: me_config_body
            .proof_market_place
            .as_ref()
            .unwrap()
            .to_string(),
        rpc_url: me_config_body.rpc_url.as_ref().unwrap().to_string(),
        relayer_private_key: me_config_body
            .relayer_private_key
            .as_ref()
            .unwrap()
            .to_string(),
        attestation_verifier: me_config_body
            .attestation_verifier
            .as_ref()
            .unwrap()
            .to_string(),
        entity_registry: me_config_body.entity_registry.as_ref().unwrap().to_string(),
        payment_token: me_config_body.payment_token.as_ref().unwrap().to_string(),
        platform_token: me_config_body.platform_token.as_ref().unwrap().to_string(),
    };

    //Generating the json config file
    let json_string = serde_json::to_string(&matching_engine_config_file)?;
    let mut file = File::create(config_file_path).await?;
    tokio::io::AsyncWriteExt::write_all(&mut file, json_string.as_bytes()).await?;
    Ok(())
}

pub async fn update_matching_engine_config_with_new_data(
    json_input: &Json<UpdateMatchingEngineConfig>,
    mut config_file: MatchingEngineConfig,
) -> Result<MatchingEngineConfig, Box<dyn std::error::Error>> {
    let config = &mut config_file;

    update_field!(config, json_input, chain_id);
    update_field!(config, json_input, start_block);
    update_field!(config, json_input, rpc_url);
    update_field!(config, json_input, relayer_private_key);
    update_field!(config, json_input, proof_market_place);
    update_field!(config, json_input, generator_registry);
    update_field!(config, json_input, platform_token);
    update_field!(config, json_input, payment_token);
    update_field!(config, json_input, attestation_verifier);
    update_field!(config, json_input, entity_registry);

    Ok(config_file)
}

//Read the matching_engine config file
pub async fn read_matching_engine_config_file(
) -> Result<MatchingEngineConfig, Box<dyn std::error::Error>> {
    let file = File::open("../matching_engine_config/matching_engine_config.json").await?;
    let mut buf_reader = tokio::io::BufReader::new(file);

    let mut content = String::new();
    buf_reader.read_to_string(&mut content).await?;

    let matching_engine_config_file: MatchingEngineConfig = serde_json::from_str(&content)?;
    Ok(matching_engine_config_file)
}

//Update the matching_engine config file
pub async fn update_matching_engine_config_file(
    matching_engine_config: MatchingEngineConfig,
) -> Result<(), std::io::Error> {
    let json_string = serde_json::to_string(&matching_engine_config)?;
    let mut file = File::create("../matching_engine_config/matching_engine_config.json").await?;
    tokio::io::AsyncWriteExt::write_all(&mut file, json_string.as_bytes()).await?;
    Ok(())
}

//Check matching engine address balance
pub async fn matching_engine_config_validation(
    private_key: &str,
    rpc_url: &str,
    chain_id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let key = private_key;
    let chain_id = chain_id;
    let rpc_url = rpc_url;

    let signer = key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(U64::from_dec_str(&chain_id.to_string()).unwrap().as_u64());

    let provider_http = Provider::<Http>::try_from(rpc_url)
        .unwrap()
        .with_signer(signer.clone());
    let gas_payer_address = signer.address();
    let account_balance = provider_http.get_balance(gas_payer_address, None).await?;

    // Check if the balance is greater than 0.05 ETH
    if account_balance >= ethers::types::U256::from_dec_str("50000000000000000")? {
        Ok(true)
    } else {
        Ok(false)
    }
}

//Fetch the matching engine public key
pub async fn get_matching_engine_public_key() -> Result<String, Box<dyn std::error::Error>> {
    let matching_engine_private_key = read_matching_engine_config_file()
        .await?
        .matching_engine_key;
    let matching_engine_public_key = hex::encode(
        matching_engine_private_key
            .parse::<LocalWallet>()?
            .address(),
    );
    let formated_me_public_key = "0x".to_string() + &matching_engine_public_key;
    Ok(formated_me_public_key)
}

//Fetch the matching engine ecies public key
pub async fn get_matching_engine_ecies_public_key() -> Result<String, Box<dyn std::error::Error>> {
    let matching_engine_private_key = read_matching_engine_config_file()
        .await?
        .matching_engine_key;
    let private_key = hex::decode(matching_engine_private_key).unwrap();
    let private_key: &[u8; 32] = private_key.as_slice().try_into().unwrap();
    let sk = SecretKey::parse(private_key).unwrap();
    let public_key = PublicKey::from_secret_key(&sk);
    let public_key = public_key.serialize_compressed();
    let encoded_key = hex::encode(public_key);
    let formated_ecies_public_key = "0x".to_string() + &encoded_key;
    Ok(formated_ecies_public_key)
}

//Contract validation
pub async fn contract_validation() -> Result<ValidationResponse, Box<dyn std::error::Error>> {
    let matching_engine_config = read_matching_engine_config_file().await?;
    let rpc_url = matching_engine_config.rpc_url;
    let proof_market_place_address = matching_engine_config.proof_market_place;
    let entity_key_registry = matching_engine_config.entity_registry;
    let private_key = matching_engine_config.relayer_private_key;
    let chain_id = matching_engine_config.chain_id;

    let signer = private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(U64::from_dec_str(&chain_id.to_string()).unwrap().as_u64());

    let proof_market_place_address = Address::from_str(&proof_market_place_address)?;

    let provider_http = Provider::<Http>::try_from(rpc_url)?.with_signer(signer.clone());
    let client = Arc::new(provider_http.clone());

    // // Check if the matching engine key has a role
    // let proof_market_place_contract = pmp::ProofMarketplace::new(
    //     proof_market_place_address,
    //     client.clone(),
    // );

    // Check if the ecies key is updated in the contract
    let entity_key_resitry_address = Address::from_str(&entity_key_registry)?;

    let entity_key_registry_contract = bindings::entity_key_registry::EntityKeyRegistry::new(
        entity_key_resitry_address,
        client.clone(),
    );

    // let converted_matching_engine_address = Address::from_str(&matching_engine_key)?;

    // // Check if the matching engine key has a role
    // let proof_market_place_contract =
    //     pmp::ProofMarketplace::new(proof_market_place_address, client.clone());

    /*
     * TODO: better to check if ENTITY_KEY_REGISTRY.allowOnlyVerified(signer, matchingEngineImageId)
     * figure out a way to fetch matching engine image id here using commented snippet above
     */

    let matching_eninge_ecies_pub_key = entity_key_registry_contract
        .pub_key(proof_market_place_address, 0.into())
        .await
        .unwrap();
    log::info!("me_public_ecies_key : {matching_eninge_ecies_pub_key}");
    if matching_eninge_ecies_pub_key.len() < 2 {
        let validation_message =
            "Matching engine ecies key not udpated in the registry".to_string();
        return Ok(ValidationResponse {
            message: validation_message,
            status: false,
        });
    }

    let local_ecies_pub_key = get_matching_engine_ecies_public_key().await?;

    let mut prepended_key_array = [0u8; 65];
    prepended_key_array[0] = 0x04;
    prepended_key_array[1..].copy_from_slice(&matching_eninge_ecies_pub_key);
    let public_key = ecies::PublicKey::parse(&prepended_key_array).unwrap();
    let encoded_key = hex::encode(public_key.serialize_compressed());
    let contract_ecies_public_key = "0x".to_string() + &encoded_key;

    if local_ecies_pub_key != contract_ecies_public_key {
        let validation_message =
            "Matching engine ecies pub key does not match the the ecies pub key in registry."
                .to_string();
        return Ok(ValidationResponse {
            message: validation_message,
            status: false,
        });
    }

    // Check if the balance is greater than 0.05 ETH
    let gas_payer_address = signer.address();
    let account_balance = provider_http.get_balance(gas_payer_address, None).await?;

    if account_balance < ethers::types::U256::from_dec_str("50000000000000000")? {
        let validation_message =
            "Matching engine relayer_key doesn't have enough balance, minimum balance required is 0.05ETH"
                .to_string();
        return Ok(ValidationResponse {
            message: validation_message,
            status: false,
        });
    }

    Ok(ValidationResponse {
        message: "Smart contract validation done".to_string(),
        status: true,
    })
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
    let signer = secp_private_key.parse::<LocalWallet>().unwrap();
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
    Ok(signature)
}
