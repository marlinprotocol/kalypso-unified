use crate::MarketDetails;
use bindings::proof_marketplace::ProofMarketplace;
use bindings::shared_types::Ask;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use flate2::read::ZlibDecoder;
use reqwest::Response;
use secret_input_helpers::secret_inputs_helpers::{
    decrypt_data_with_ecies_and_aes, decrypt_ecies, encrypt_ecies,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use std::time::Instant;
use std::{thread, time::Duration};

// type ProofMarketPlaceContractWs =
//     Arc<ProofMarketplace<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>>;

type ProofMarketPlaceContractHttp =
    Arc<ProofMarketplace<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>;

pub struct GenerateProofParams<'a> {
    pub ask_id: ethers::types::U256,
    pub new_acl: ethers::types::Bytes,
    pub proof_market_place_contract_http: ProofMarketPlaceContractHttp,
    pub ecies_private_key: &'a [u8],
    pub start_block: &'a U64,
    pub end_block: &'a U64,
    pub markets: &'a HashMap<String, MarketDetails>,
}

#[derive(Debug, Clone)]
pub enum Proof {
    ValidProof(Bytes),
    InvalidProof(Bytes),
}

#[derive(Deserialize, Debug)]
struct ProofGenerationResponse {
    message: String,
    data: Bytes,
}

// Define the response format struct
#[derive(Deserialize)]
struct IvsResponse {
    signature: String,
    ask_id: u64,
}

#[derive(Deserialize, Debug)]
struct ApiDataResponse {
    ivs_ecies_public_key: String,
}

#[derive(Deserialize, Debug)]
struct IvsPublicKeyResponse {
    data: ApiDataResponse,
}

//Generating proof for the input
pub async fn generate_proof(
    generate_proof_params: GenerateProofParams<'_>,
) -> Result<Proof, Box<dyn std::error::Error>> {
    let GenerateProofParams {
        proof_market_place_contract_http,
        ask_id,
        start_block,
        end_block,
        ecies_private_key,
        new_acl,
        markets,
    } = generate_proof_params;
    let client = proof_market_place_contract_http.client();
    let list_of_ask: &Ask = &proof_market_place_contract_http
        .list_of_ask(ask_id)
        .await?
        .0;
    let market_id = list_of_ask.market_id;

    let fetching_ask_secret_timer_start = Instant::now();
    log::info!(
        "Finding the logs from block {}, for ask: {}",
        start_block,
        ask_id
    );

    // Looking for ask in a batch range of 9999 blocks, done to avoid the 10000 block limit.
    let mut end_block = *end_block;
    let start_block = *start_block;
    let mut ask_log: Vec<ethers::core::types::Log> = vec![];

    let blocks_at_once = 9999;
    while start_block <= end_block {
        let begin = if end_block >= start_block + blocks_at_once {
            end_block - blocks_at_once
        } else {
            start_block
        };

        // Fetching ask Transaction hash
        let ask_event_filter = &proof_market_place_contract_http
            .ask_created_filter()
            .filter
            .from_block(begin)
            .to_block(end_block)
            .topic1(ask_id);

        let logs = client.get_logs(ask_event_filter).await?;
        log::info!("Logs found: {:?}", logs.len());
        if logs.len() == 1 {
            ask_log = logs;
            break;
        }
        thread::sleep(Duration::from_millis(2000)); // to reduce calls and avoid rate limit (will be problem only for old requests, not new ones)
        end_block = begin - 1;
    }

    let parsed_ask_created_log = proof_market_place_contract_http
        .decode_event::<bindings::proof_marketplace::AskCreatedFilter>(
            "AskCreated",
            ask_log[0].topics.clone(),
            ask_log[0].data.clone(),
        )
        .unwrap();

    //Checking if the ask has a secret provided
    let decoded_secret_input = if parsed_ask_created_log.has_private_inputs {
        log::info!("Secret input found");
        // Handling compressed secret inputs
        let encrypted_secret_input = parsed_ask_created_log.secret_data.to_vec();

        let result = decrypt_data_with_ecies_and_aes(
            &encrypted_secret_input,
            &new_acl,
            ecies_private_key,
            market_id,
        );

        result.unwrap()
    } else {
        Vec::new()
    };

    let ask_secret_fetch_time = fetching_ask_secret_timer_start.elapsed().as_millis();
    log::info!(
        "Took {} ms for fetching the secret inputs for the ask",
        ask_secret_fetch_time
    );

    let mut decoder = ZlibDecoder::new(&decoded_secret_input[..]);
    let mut decoded_secret_input: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut decoded_secret_input).unwrap();

    // PRIVATE MARKET
    if markets.contains_key(&market_id.to_string()) && parsed_ask_created_log.has_private_inputs {
        let generator_port = &markets.get(&market_id.to_string()).unwrap().port;
        log::info!(
            "Forwarding inputs for market ID : {:#?} to the generator running on port {:#?}",
            market_id.to_string(),
            generator_port
        );
        let proof_generation_response = generate_proof_from_generator(
            list_of_ask.clone(),
            decoded_secret_input.clone(),
            generator_port.to_string(),
            ask_id.as_u64(),
        )
        .await?;
        if proof_generation_response.status().is_success() {
            let proof_response: ProofGenerationResponse = proof_generation_response.json().await?;
            log::info!("{:#?}", proof_response.message);
            return Ok(Proof::ValidProof(proof_response.data));
        }
        // else if proof_generation_response.status() == 400
        else {
            let proof_response: ProofGenerationResponse = proof_generation_response.json().await?;
            log::info!(
                "Error message from the generator : {}",
                proof_response.message
            );
            let signature = &proof_response.data.to_string()[2..];
            log::info!("Signature : {}", signature);
            let sign_bytes = hex::decode(signature)?;
            return Ok(Proof::InvalidProof(sign_bytes.into()));
        }
    }
    // PUBLIC MARKET
    else if markets.contains_key(&market_id.to_string())
        && !parsed_ask_created_log.has_private_inputs
    {
        let generator_port = &markets.get(&market_id.to_string()).unwrap().port;
        let ivs_url = &markets.get(&market_id.to_string()).unwrap().ivs_url;
        log::info!(
            "Forwarding inputs for market ID : {:#?} to the generator running on port {:#?}",
            market_id.to_string(),
            generator_port
        );
        let proof_generation_response = generate_proof_from_generator(
            list_of_ask.clone(),
            decoded_secret_input.clone(),
            generator_port.to_string(),
            ask_id.as_u64(),
        )
        .await?;
        if proof_generation_response.status().is_success() {
            let proof_response: ProofGenerationResponse = proof_generation_response.json().await?;
            log::info!("{:#?}", proof_response.message);
            return Ok(Proof::ValidProof(proof_response.data));
        }
        log::info!("Proof generated is not valid");

        let fetch_ivs_public_key = fetch_ivs_public_keys(ivs_url.to_string()).await?;

        let fetch_ivs_public_key_response_body: IvsPublicKeyResponse =
            fetch_ivs_public_key.json().await?;

        let trimmed_ecies_key = &fetch_ivs_public_key_response_body.data.ivs_ecies_public_key[2..];

        log::info!("IVS public key data : {:#?}", trimmed_ecies_key);

        let decoded_ecies = &hex::decode(trimmed_ecies_key)?;
        log::info!("Decoded the ecies key");

        let cipher = decrypt_ecies(ecies_private_key, &new_acl)?;
        log::info!("Cipher generated");
        let final_acl = encrypt_ecies(decoded_ecies, cipher.as_slice())?;
        log::info!("Final ACL generated");
        let encrypted_secret_input = parsed_ask_created_log.secret_data.to_vec();
        log::info!("Encrypted secret input fetched, fetching signature next");
        let invalid_proof_signature = get_proof_for_invalid_request(
            ivs_url.to_string(),
            ask_id.as_u64(),
            hex::encode(encrypted_secret_input),
            hex::encode(final_acl),
        )
        .await?;
        let response_data: IvsResponse = invalid_proof_signature.json().await?;
        log::info!(
            "Submitting signature for invalid inputs for ASK ID : {}",
            response_data.ask_id
        );
        let signature = &response_data.signature[2..];
        log::info!("Signature : {}", signature);
        let sign_bytes = hex::decode(signature).unwrap();
        return Ok(Proof::InvalidProof(sign_bytes.into()));
    }

    Err("Circuit not implemented".into())
}

//Get signature for invalid input from IVS
async fn get_proof_for_invalid_request(
    ivs_url: String,
    ask_id: u64,
    secret: String,
    acl: String,
) -> Result<Response, Box<dyn std::error::Error>> {
    log::info!("Inside the get proof for invalid request");
    #[derive(Serialize)]
    struct Payload {
        pub ask_id: u64,
        pub acl: String,
        pub encrypted_secret: String,
    }

    // Create a client instance
    let client = reqwest::Client::new();

    // Create the payload
    let payload = Payload {
        ask_id,
        acl,
        encrypted_secret: secret,
    };
    log::info!("Payload generated");

    let ivs_endpoint_to_fetch_signature = format!("{}:3030/checkInputWithSignature", ivs_url);

    // Make the POST request
    let res = client
        .post(ivs_endpoint_to_fetch_signature)
        .json(&payload)
        .send()
        .await?;
    Ok(res)
}

//Forward input to proof generator
async fn generate_proof_from_generator(
    ask: Ask,
    private_input: Vec<u8>,
    port: String,
    ask_id: u64,
) -> Result<Response, Box<dyn std::error::Error>> {
    #[derive(Serialize, Debug)]
    struct Payload {
        ask: Ask,
        private_input: Vec<u8>,
        ask_id: u64,
    }

    let payload = Payload {
        ask,
        private_input,
        ask_id,
    };

    // Create a client instance
    let client = reqwest::Client::new();

    let endpoint = format!("http://0.0.0.0:{}/api/generateProof", port);
    log::info!("endpoint : {}", endpoint);
    // Make the POST request
    let res = client.post(endpoint).json(&payload).send().await?;
    Ok(res)
}

//Fetch IVS public keys
async fn fetch_ivs_public_keys(ivs_url: String) -> Result<Response, Box<dyn std::error::Error>> {
    // Create a client instance
    let client = reqwest::Client::new();

    let fetch_ivs_public_key_endpoint =
        format!("{}:5000/api/fetchInputVerifierPublicKeys", ivs_url);
    // Make the POST request
    let res = client.post(fetch_ivs_public_key_endpoint).send().await?;
    Ok(res)
}
