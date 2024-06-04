use std::str::FromStr;
use std::sync::Arc;

use actix_web::web::Json;
use ecies::{PublicKey, SecretKey};
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::types::U256;
use reqwest::Response;
use std::env;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::model::{
    AddNewGenerator, GeneratorConfig, GeneratorConfigFile, GeneratorPublicKeys, RuntimeConfig,
    RuntimeConfigFile, SetupRequestBodyGeneratorConfig, SetupRequestBodyRuntimeConfig,
    SignAttestation, UpdateRuntimeConfig, ValidationResponse,
};

macro_rules! update_field {
    ($config:expr, $input:expr, $field:ident) => {
        if let Some(new_value) = &$input.$field {
            $config.$field = new_value.to_string();
        }
    };
}

macro_rules! update_u64_field {
    ($config:expr, $input:expr, $field:ident) => {
        if let Some(new_value) = $input.$field {
            $config.$field = new_value;
        }
    };
}

//Get ECIES public key
pub async fn get_public_keys_for_a_generator(
    generator_ecies_key: &String,
) -> Result<GeneratorPublicKeys, Box<dyn std::error::Error>> {
    let generator_public_key = hex::encode(generator_ecies_key.parse::<LocalWallet>()?.address());
    let formated_public_key = "0x".to_string() + &generator_public_key;

    let private_key = hex::decode(generator_ecies_key).unwrap();
    let private_key: &[u8; 32] = private_key.as_slice().try_into().unwrap();
    let sk = SecretKey::parse(private_key).unwrap();
    let public_key = PublicKey::from_secret_key(&sk);
    let public_key = public_key.serialize_compressed();
    let encoded_key = hex::encode(public_key);
    let formated_ecies_public_key = "0x".to_string() + &encoded_key;

    let public_keys = GeneratorPublicKeys {
        generator_ecies_public_key: formated_ecies_public_key,
        generator_public_key: formated_public_key,
    };

    Ok(public_keys)
}

//Generate Config file
pub async fn generate_config_file(
    generator_config_body: &Vec<SetupRequestBodyGeneratorConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create config_file_folder
    let folder_path = "../generator_config";
    if fs::metadata(&folder_path).await.is_ok() {
        log::info!("generator_config folder already exists!");
    } else {
        fs::create_dir(&folder_path)
            .await
            .expect("Unable to create new folder");
    }

    //Using the enclave secp secret for ecies private key
    let read_secp_private_key = fs::read("/app/secp.sec").await?;
    let secp_private_key = secp256k1::SecretKey::from_slice(&read_secp_private_key)
        .unwrap()
        .display_secret()
        .to_string();

    //Structure data
    let config_file_path = folder_path.to_string() + "/generator_config.json";
    let mut generator_config: Vec<GeneratorConfig> = Vec::new();
    for generator in generator_config_body {
        let generator_data = GeneratorConfig {
            address: generator.address.as_ref().unwrap().to_string(),
            data: generator.data.as_ref().unwrap().to_string(),
            ecies_private_key: secp_private_key.clone(),
            supported_markets: generator.supported_markets.as_ref().unwrap().to_vec(),
        };
        generator_config.push(generator_data);
    }

    let generator_config_file = GeneratorConfigFile { generator_config };

    //Generating the json config file
    let json_string = serde_json::to_string(&generator_config_file)?;
    let mut file = File::create(config_file_path).await?;
    tokio::io::AsyncWriteExt::write_all(&mut file, json_string.as_bytes()).await?;
    Ok(())
}

//Generate Runtime file
pub async fn generate_runtime_file(
    runtime_config_body: &SetupRequestBodyRuntimeConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create config_file_folder
    let folder_path = "../generator_config";
    if fs::metadata(&folder_path).await.is_ok() {
        log::info!("generator_config folder already exists!");
    } else {
        fs::create_dir(&folder_path)
            .await
            .expect("Unable to create new folder");
    }

    //Structure data
    let config_file_path = folder_path.to_string() + "/runtime_config.json";

    let runtime_config = RuntimeConfig {
        chain_id: runtime_config_body.chain_id.unwrap(),
        start_block: runtime_config_body.start_block.unwrap(),
        generator_registry: runtime_config_body
            .generator_registry
            .as_ref()
            .unwrap()
            .to_string(),
        params_path: "./params/".to_string(),
        private_key: runtime_config_body
            .private_key
            .as_ref()
            .unwrap()
            .to_string(),
        proof_market_place: runtime_config_body
            .proof_market_place
            .as_ref()
            .unwrap()
            .to_string(),
        ws_url: runtime_config_body.ws_url.as_ref().unwrap().to_string(),
        http_url: runtime_config_body.http_url.as_ref().unwrap().to_string(),
        payment_token: runtime_config_body
            .payment_token
            .as_ref()
            .unwrap()
            .to_string(),
        staking_token: runtime_config_body
            .staking_token
            .as_ref()
            .unwrap()
            .to_string(),
        attestation_verifier: runtime_config_body
            .attestation_verifier
            .as_ref()
            .unwrap()
            .to_string(),
        entity_registry: runtime_config_body
            .entity_registry
            .as_ref()
            .unwrap()
            .to_string(),
        markets: runtime_config_body.markets.clone(),
    };

    let generator_config_file = RuntimeConfigFile { runtime_config };

    //Generating the json config file
    let json_string = serde_json::to_string(&generator_config_file)?;
    let mut file = File::create(config_file_path).await?;
    tokio::io::AsyncWriteExt::write_all(&mut file, json_string.as_bytes()).await?;
    Ok(())
}

pub async fn update_runtime_config_with_new_data(
    json_input: &Json<UpdateRuntimeConfig>,
    mut config_file: RuntimeConfigFile,
) -> Result<RuntimeConfigFile, Box<dyn std::error::Error>> {
    let config = &mut config_file.runtime_config;

    update_u64_field!(config, json_input, chain_id);
    update_field!(config, json_input, ws_url);
    update_field!(config, json_input, http_url);
    update_field!(config, json_input, private_key);
    update_field!(config, json_input, proof_market_place);
    update_field!(config, json_input, generator_registry);
    update_u64_field!(config, json_input, start_block);
    update_field!(config, json_input, staking_token);
    update_field!(config, json_input, payment_token);
    update_field!(config, json_input, attestation_verifier);
    update_field!(config, json_input, entity_registry);
    if let Some(new_markets_data) = &json_input.markets {
        config_file.runtime_config.markets = new_markets_data.clone()
    }

    Ok(config_file)
}

//Read the runtime config file
pub async fn read_runtime_config_file() -> Result<RuntimeConfigFile, Box<dyn std::error::Error>> {
    let file = File::open("../generator_config/runtime_config.json").await?;
    let mut buf_reader = tokio::io::BufReader::new(file);

    let mut content = String::new();
    buf_reader.read_to_string(&mut content).await?;

    let generator_config_file: RuntimeConfigFile = serde_json::from_str(&content)?;
    Ok(generator_config_file)
}

//Update the runtime config file
pub async fn update_runtime_config_file(
    generator_config: RuntimeConfigFile,
) -> Result<(), std::io::Error> {
    let json_string = serde_json::to_string(&generator_config)?;
    let mut file = File::create("../generator_config/runtime_config.json").await?;
    tokio::io::AsyncWriteExt::write_all(&mut file, json_string.as_bytes()).await?;
    Ok(())
}

//Read the generator config file
pub async fn read_generator_config_file() -> Result<GeneratorConfigFile, Box<dyn std::error::Error>>
{
    let file = File::open("../generator_config/generator_config.json").await?;
    let mut buf_reader = tokio::io::BufReader::new(file);

    let mut content = String::new();
    buf_reader.read_to_string(&mut content).await?;

    let generator_config_file: GeneratorConfigFile = serde_json::from_str(&content)?;
    Ok(generator_config_file)
}

//Add a new generator to the generator config
pub async fn add_new_generator(
    json_input: &Json<AddNewGenerator>,
    mut config_file: GeneratorConfigFile,
) -> Result<GeneratorConfigFile, Box<dyn std::error::Error>> {
    //Updating the existing generator list
    let new_generator = &json_input.0;

    //Using the enclave secp secret for ecies private key
    let read_secp_private_key = fs::read("/app/secp.sec").await?;
    let secp_private_key = secp256k1::SecretKey::from_slice(&read_secp_private_key)
        .unwrap()
        .display_secret()
        .to_string();

    let new_generator_data = GeneratorConfig {
        address: new_generator.address.as_ref().unwrap().to_string(),
        data: new_generator.data.as_ref().unwrap().to_string(),
        ecies_private_key: secp_private_key,
        supported_markets: new_generator.supported_markets.as_ref().unwrap().to_vec(),
    };
    config_file.generator_config.push(new_generator_data);
    Ok(config_file)
}

//Update the generator config file
pub async fn update_generator_config_file(
    generator_config: GeneratorConfigFile,
) -> Result<(), std::io::Error> {
    let json_string = serde_json::to_string(&generator_config)?;
    let mut file = File::create("../generator_config/generator_config.json").await?;
    tokio::io::AsyncWriteExt::write_all(&mut file, json_string.as_bytes()).await?;
    Ok(())
}

//Check runtime address balance
pub async fn runtime_config_validation(
    private_key: &str,
    rpc_url: &str,
    chain_id: &i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let key = private_key;

    let signer = key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(U64::from_dec_str(&chain_id.to_string()).unwrap().as_u64());

    let provider = Provider::<Ws>::connect(rpc_url).await?.with_signer(signer);
    let gas_payer_address = provider.signer().address();
    let account_balance = provider.get_balance(gas_payer_address, None).await?;

    // Check if the balance is greater than 0.05 ETH
    if account_balance >= ethers::types::U256::from_dec_str("50000000000000000")? {
        Ok(true)
    } else {
        Ok(false)
    }
}

//Contract validation
pub async fn contract_validation() -> Result<ValidationResponse, Box<dyn std::error::Error>> {
    let config_file = read_generator_config_file().await?;
    let runtime_config = read_runtime_config_file().await?;
    let rpc_url = runtime_config.runtime_config.ws_url;
    let private_key = runtime_config.runtime_config.private_key;
    let chain_id = runtime_config.runtime_config.chain_id;
    let generator_registry_contract_address = runtime_config.runtime_config.generator_registry;
    let generator_config = config_file.generator_config;
    let entity_key_registry_address = runtime_config.runtime_config.entity_registry;

    let signer = private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(U64::from_dec_str(&chain_id.to_string()).unwrap().as_u64());

    let provider = Arc::new(Provider::<Ws>::connect(rpc_url).await?.with_signer(signer));
    let client = Arc::new(Arc::clone(&provider));

    let gas_payer_address = provider.signer().address();
    let account_balance = provider.get_balance(gas_payer_address, None).await?;

    log::info!("Trying to fetch account balance");
    // Check if the balance is greater than 0.05 ETH
    if account_balance < ethers::types::U256::from_dec_str("50000000000000000")? {
        let validation_message =
            "Runtime private_key doesn't have enough balance, minimum balance required is 0.05ETH"
                .to_string();
        return Ok(ValidationResponse {
            message: validation_message,
            status: false,
        });
    }
    log::info!("Generator balance fetched");

    let generator_registry_contract_address =
        Address::from_str(&generator_registry_contract_address)?;
    let generator_registry_contract = bindings::generator_registry::GeneratorRegistry::new(
        generator_registry_contract_address,
        Arc::clone(&client),
    );

    let entity_key_resitry_address = Address::from_str(&entity_key_registry_address)?;
    let entity_key_registry_contract = bindings::entity_key_registry::EntityKeyRegistry::new(
        entity_key_resitry_address,
        Arc::clone(&client),
    );

    for generator in generator_config {
        let converted_generator_address = Address::from_str(&generator.address)?;
        let market_id = &generator.supported_markets[0];
        log::info!("Trying to fetch the ECIES key");
        let skip_verification =
            env::var("SKIP_VERIFICATION").unwrap_or_else(|_| "false".to_string()) == "true";

        if !skip_verification {
            log::info!("Performing ecies key validation in the contracts");
            // Checking if the generator ECIES pub key is updated in the contracts
            let generator_ecies_pub_key = entity_key_registry_contract
                .pub_key(converted_generator_address, U256::from_dec_str(market_id)?)
                .await?;
            if generator_ecies_pub_key.len() < 2 {
                let validation_message = format!(
                    "ECIES key not udpated in the registry for generator : {}",
                    generator.address
                );
                return Ok(ValidationResponse {
                    message: validation_message,
                    status: false,
                });
            }
            log::info!("ECIES key is updated in the registry");
            let local_ecies_pub_key = get_public_keys_for_a_generator(&generator.ecies_private_key)
                .await?
                .generator_ecies_public_key;

            let mut prepended_key_array = [0u8; 65];
            prepended_key_array[0] = 0x04;
            prepended_key_array[1..].copy_from_slice(&generator_ecies_pub_key);
            let public_key = ecies::PublicKey::parse(&prepended_key_array).unwrap();
            let encoded_key = hex::encode(public_key.serialize_compressed());
            let contract_ecies_public_key = "0x".to_string() + &encoded_key;

            if local_ecies_pub_key != contract_ecies_public_key {
                let validation_message = format!("Local ECIES pub key does not match the the ecies pub key in registry for generator : {}",generator.address);
                return Ok(ValidationResponse {
                    message: validation_message,
                    status: false,
                });
            }
            log::info!("Local ECIES key is same as the one in the contract");
        }

        // Checking if generator has registered for the market provided in supported_markets vec
        for market in generator.supported_markets {
            let generator_data = generator_registry_contract
                .generator_info_per_market(
                    converted_generator_address,
                    U256::from_dec_str(&market)?,
                )
                .call()
                .await?;
            log::info!("generator address {}", converted_generator_address);
            log::info!("market {}", &market);
            dbg!(&generator_data);

            if generator_data.0 == 0 {
                let validation_message = format!(
                    "Generator {} is not registered for market {}. Please register and try again",
                    generator.address, market
                );
                return Ok(ValidationResponse {
                    message: validation_message.to_string(),
                    status: false,
                });
            }
            log::info!(
                "Generator {} is registered for market {}.",
                generator.address,
                market
            )
        }
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

//Benchmark proof generation. Return proof generation time in ms.
pub async fn benchmark(endpoint: String) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.get(endpoint).send().await?;
    Ok(res)
}
