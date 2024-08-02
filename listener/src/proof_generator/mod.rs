use crate::job_creator::MarketDetails;
use bindings::proof_marketplace::{AskCreatedFilter, ProofMarketplace};
use bindings::shared_types::Ask;
use confidential_provers::ConfidentialProver;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use flate2::read::ZlibDecoder;

use kalypso_helper::secret_inputs_helpers::decrypt_data_with_ecies_and_aes;
use non_confidential_prover::NonConfidentialProver;
use prover::{Proof, Prover};
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use std::time::Instant;
use std::{thread, time::Duration};

mod confidential_provers;
mod non_confidential_prover;
pub mod prover;

type ProofMarketPlaceContractHttp =
    Arc<ProofMarketplace<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>;

#[derive(Clone)]
pub struct GenerateProofParams<'a> {
    pub ask_id: ethers::types::U256,
    pub new_acl: ethers::types::Bytes,
    pub proof_market_place_contract_http: ProofMarketPlaceContractHttp,
    pub ecies_private_key: &'a Option<ecies::SecretKey>,
    pub start_block: &'a U64,
    pub end_block: &'a U64,
    pub markets: &'a HashMap<String, MarketDetails>,
}

//Generating proof for the input
pub async fn generate_proof(
    generate_proof_params: GenerateProofParams<'_>,
) -> Result<Proof, Box<dyn std::error::Error>> {
    let (public_inputs, decoded_secret_input, market_id, parsed_ask_created_log, markets) =
        fetch_decoded_secret(generate_proof_params.clone())
            .await
            .unwrap();

    if !generate_proof_params
        .markets
        .contains_key(&market_id.to_string())
    {
        return Err("Market not being proven for".into());
    }

    if parsed_ask_created_log.has_private_inputs {
        // market with confidential inputs
        let generator_port = &markets.get(&market_id.to_string()).unwrap().port;
        let confidential_prover = ConfidentialProver::new(
            format!(
                "http://localhost:{}/api/checkInput",
                generator_port.clone().unwrap()
            ),
            format!(
                "http://localhost:{}/api/getAttestationForInvalidInputs",
                generator_port.clone().unwrap()
            ),
            format!(
                "http://localhost:{}/api/verifyInputsAndProof",
                generator_port.clone().unwrap()
            ),
            format!(
                "http://localhost:{}/api/generateProof",
                generator_port.clone().unwrap()
            ),
            parsed_ask_created_log.ask_id,
            public_inputs.into(),
            decoded_secret_input
                .expect("Unable to decode secret for confidential markets")
                .into(),
        );

        confidential_prover.get_proof().await
    } else {
        // market without confidential inputs
        let ivs_url = &markets.get(&market_id.to_string()).unwrap().ivs_url;
        let generator_url = &markets
            .get(&market_id.to_string())
            .unwrap()
            .prover_gateway_url;

        let non_confidential_prover = NonConfidentialProver::new(
            format!("{}/api/checkInput", ivs_url.clone().unwrap()),
            format!(
                "{}/api/getAttestationForInvalidInputs",
                ivs_url.clone().unwrap()
            ),
            format!("{}/api/verifyInputsAndProof", ivs_url.clone().unwrap()),
            generator_url.clone().unwrap().clone(),
            parsed_ask_created_log.ask_id,
            public_inputs.into(),
        );

        non_confidential_prover.get_proof().await
    }
}

type InputAndDecodedSecret<'a> = (
    Vec<u8>,
    Option<Vec<u8>>,
    U256,
    AskCreatedFilter,
    &'a HashMap<String, MarketDetails>,
);
async fn fetch_decoded_secret(
    generate_proof_params: GenerateProofParams<'_>,
) -> Result<InputAndDecodedSecret, Box<dyn std::error::Error>> {
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
            ecies_private_key.unwrap().serialize().as_ref(),
            market_id,
        );

        let result = result.unwrap();
        let mut decoder = ZlibDecoder::new(&result[..]);
        let mut decoded_secret_input: Vec<u8> = Vec::new();
        decoder.read_to_end(&mut decoded_secret_input).unwrap();
        Some(decoded_secret_input)
    } else {
        None
    };

    let ask_secret_fetch_time = fetching_ask_secret_timer_start.elapsed().as_millis();
    log::info!(
        "Took {} ms for fetching the secret inputs for the ask",
        ask_secret_fetch_time
    );

    Ok((
        list_of_ask.prover_data.to_vec(),
        decoded_secret_input,
        market_id,
        parsed_ask_created_log,
        markets,
    ))
}
