use bytes::Bytes;
use ethers::types::U256;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AttestationVerifierResponse {
    pub signature: String,
    pub secp256k1_public: String,
    pub pcr0: String,
    pub pcr1: String,
    pub pcr2: String,
    pub timestamp: usize,
}

pub fn utility_url(base_url: &str, path: &str) -> String {
    format!("{}{}", base_url, path)
}

pub async fn build_attestation(
    base_url: &str,
    print_logs: bool,
) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, Box<dyn Error>> {
    let attestation_end_point = utility_url(base_url, "/attestation/raw");

    build_attestation_raw(&attestation_end_point, print_logs).await
}

pub async fn build_attestation_raw(
    attestation_end_point: &str,
    print_logs: bool,
) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, Box<dyn Error>> {
    if print_logs {
        println!("build attestation {}", attestation_end_point);
    }

    let client = Client::new();
    let response = client.get(attestation_end_point).send().await?;

    // Check if the response status is successful (2xx)
    if !response.status().is_success() {
        println!("status code: {}", response.status());
        return Err("failed building the attestation".into());
    }

    // Get the response body as a stream of bytes
    let stream = response.bytes_stream();

    Ok(stream)
}

pub async fn build_attestation_vec(
    attestation_end_point: &str,
    print_logs: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let attestation_stream = build_attestation(attestation_end_point, print_logs)
        .await
        .map_err(|e| format!("Failed Building Attestations: {}", e))?;

    let attestation_data: Vec<u8> = attestation_stream
        .fold(Vec::new(), |mut acc, item| async {
            match item {
                Ok(bytes) => {
                    acc.extend_from_slice(&bytes);
                    acc
                }
                Err(e) => {
                    println!("Error while receiving data: {}", e);
                    acc
                }
            }
        })
        .await;

    Ok(attestation_data)
}

pub async fn build_attestation_vec_raw(
    base_url: &str,
    print_logs: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let attestation_stream = build_attestation_raw(base_url, print_logs)
        .await
        .map_err(|e| format!("Failed Building Attestations: {}", e))?;

    let attestation_data: Vec<u8> = attestation_stream
        .fold(Vec::new(), |mut acc, item| async {
            match item {
                Ok(bytes) => {
                    acc.extend_from_slice(&bytes);
                    acc
                }
                Err(e) => {
                    println!("Error while receiving data: {}", e);
                    acc
                }
            }
        })
        .await;

    Ok(attestation_data)
}

// Function to get attestation by sending attestation_data to the verifier
pub async fn get_verified_attestation(
    verifier_url: &str,
    attestation_data: Vec<u8>,
    print_logs: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Construct the verify endpoint URL
    let verify_endpoint = utility_url(verifier_url, "/verify/raw");

    if print_logs {
        println!("Sending attestation data to {}", verify_endpoint);
    }

    let client = Client::new();
    let response = client
        .post(&verify_endpoint)
        .header("Content-Type", "application/octet-stream")
        .body(attestation_data)
        .send()
        .await?;

    if !response.status().is_success() {
        if print_logs {
            println!(
                "Attestation verifier responded with status: {}",
                response.status()
            );
        }
        return Err("Failed to verify attestation".into());
    }

    let verifier_response: AttestationVerifierResponse = response.json().await?;

    if print_logs {
        println!("Fetched attestation successfully");
        println!("Verifier response: {:?}", verifier_response);
    }

    // Extract and process secp256k1_public
    let ecies_pubkey = format!("0x{}", verifier_response.secp256k1_public);
    if ecies_pubkey.len() != 130 {
        return Err("secp pub key length incorrect".into());
    }

    // Decode hex strings to bytes
    let signature_bytes = hex::decode(&verifier_response.signature.trim_start_matches("0x"))?;
    let pcr0_bytes = hex::decode(&verifier_response.pcr0.trim_start_matches("0x"))?;
    let pcr1_bytes = hex::decode(&verifier_response.pcr1.trim_start_matches("0x"))?;
    let pcr2_bytes = hex::decode(&verifier_response.pcr2.trim_start_matches("0x"))?;

    let timestamp_u256 = U256::from(verifier_response.timestamp);
    let signature_vec = signature_bytes;
    let ecies_pubkey_vec =
        hex::decode(&verifier_response.secp256k1_public.trim_start_matches("0x"))?;

    let pcr0_vec = pcr0_bytes;
    let pcr1_vec = pcr1_bytes;
    let pcr2_vec = pcr2_bytes;

    let encoded = ethers::abi::encode(&[
        ethers::abi::Token::Bytes(signature_vec.into()),
        ethers::abi::Token::Bytes(ecies_pubkey_vec.clone().into()),
        ethers::abi::Token::Bytes(pcr0_vec.into()),
        ethers::abi::Token::Bytes(pcr1_vec.into()),
        ethers::abi::Token::Bytes(pcr2_vec.into()),
        ethers::abi::Token::Uint(timestamp_u256),
    ]);

    Ok(encoded)
}

pub async fn verify_attestation(
    verifier_url: &str,
    attestation_data: Vec<u8>,
    print_logs: bool,
) -> Result<AttestationVerifierResponse, Box<dyn Error>> {
    let verify_endpoint = utility_url(verifier_url, "/verify/raw");

    if print_logs {
        println!("Sending attestation data to {}", verify_endpoint);
    }

    let client = Client::new();
    let response = client
        .post(&verify_endpoint)
        .header("Content-Type", "application/octet-stream")
        .body(attestation_data)
        .send()
        .await?;

    if !response.status().is_success() {
        if print_logs {
            println!(
                "Attestation verifier responded with status: {}",
                response.status()
            );
        }
        return Err("Failed to verify attestation".into());
    }

    let verifier_response: AttestationVerifierResponse = response.json().await?;

    Ok(verifier_response)
}

use std::collections::BTreeMap;

use aws_nitro_enclaves_cose::{crypto::Openssl, CoseSign1};
use serde_cbor::{self, value, value::Value};

pub fn parse_attestation_doc(
    attestation_doc: &[u8],
) -> Result<(CoseSign1, BTreeMap<Value, Value>, AttestationData), AttestationError> {
    let cosesign1 = CoseSign1::from_bytes(&attestation_doc)
        .map_err(|e| AttestationError::ParseFailed(format!("cose: {e}")))?;
    let payload = cosesign1
        .get_payload::<Openssl>(None)
        .map_err(|e| AttestationError::ParseFailed(format!("cose payload: {e}")))?;
    let cbor = serde_cbor::from_slice::<Value>(&payload)
        .map_err(|e| AttestationError::ParseFailed(format!("cbor: {e}")))?;
    let attestation_doc = value::from_value::<BTreeMap<Value, Value>>(cbor.clone())
        .map_err(|e| AttestationError::ParseFailed(format!("doc: {e}")))?;

    let another_attestation_doc = value::from_value::<AttestationData>(cbor)
        .map_err(|e| AttestationError::ParseFailed(format!("doc: {e}")))?;

    Ok((cosesign1, attestation_doc, another_attestation_doc))
}

pub fn verify_with_timestamp(
    attestation_doc_cbor: Vec<u8>,
    pcrs: [[u8; 48]; 3],
    timestamp: usize,
) -> Result<Vec<u8>, AttestationError> {
    oyster::verify_with_timestamp(attestation_doc_cbor, pcrs, timestamp)
}

pub fn verify(
    attestation_doc_cbor: Vec<u8>,
    pcrs: [[u8; 48]; 3],
    max_age: usize,
) -> Result<Vec<u8>, AttestationError> {
    oyster::verify(attestation_doc_cbor, pcrs, max_age)
}

pub fn get_pubkey_from_attestation(attestation_doc: Vec<u8>) -> Result<Vec<u8>, AttestationError> {
    Ok(oyster::decode_attestation(attestation_doc)?.public_key)
}

#[cfg(test)]
mod tests {
    use super::{build_attestation_vec, parse_attestation_doc, verify_attestation};

    #[tokio::test]
    async fn test_verified_attestation_with_verifier() {
        let result = build_attestation_vec("http://3.110.146.109:1500", false).await;

        assert!(
            result.is_ok(),
            "Expected Ok(_), got Err({:?})",
            result.err()
        );

        if let Ok(attestation_vec) = result {
            assert!(
                !attestation_vec.is_empty(),
                "Attestation vector should not be empty"
            );

            let verified_result =
                verify_attestation("http://13.201.207.60:1400", attestation_vec, false).await;

            assert!(
                verified_result.is_ok(),
                "Expected Ok(_), got Err({:?})",
                verified_result.err()
            );

            match serde_json::to_string_pretty(&verified_result.unwrap()) {
                Ok(json_string) => {
                    // Step 3: Print the JSON string
                    println!("Verified Attestation as JSON:\n{}", json_string);
                }
                Err(e) => {
                    eprintln!("Error serializing Verified Attestation: {}", e);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_pcrs_decode() {
        let result = build_attestation_vec("http://3.110.146.109:1500", false).await;

        assert!(
            result.is_ok(),
            "Expected Ok(_), got Err({:?})",
            result.err()
        );

        if let Ok(attestation_vec) = result {
            assert!(
                !attestation_vec.is_empty(),
                "Attestation vector should not be empty"
            );
            let parsed_result = parse_attestation_doc(&attestation_vec);
            assert!(parsed_result.is_ok(), "Expected Ok(_)");

            let (_, _, doc) = parsed_result.unwrap();

            match serde_json::to_string_pretty(&doc.to_attestation_response()) {
                Ok(json_string) => {
                    // Step 3: Print the JSON string
                    println!("AttestationResponse as JSON:\n{}", json_string);
                }
                Err(e) => {
                    eprintln!("Error serializing AttestationResponse: {}", e);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_attestation_decode() {
        let attestation_vec = hex::decode("8444a1013822a0591305a9696d6f64756c655f69647827692d30616563356332363863326134643630342d656e633031393532306535643836306466653666646967657374665348413338346974696d657374616d701b0000019520ebcf126470637273b00058300c828b625aa780030861feb37942db2b611a2904ee90d726228f5cf42095ad5d65b571adce7aa6eb77dd79bf6090619d0158303dc2602d18944028b4705c2b46c5d6efd73cba3c58d09deccc073075c68a4ebac36e5368eb0921c7b4c699f4ae03a1e5025830acca98eacd413c4aea449d98c0c1cee92c8c420f7cdd6cbad1da0f9066cd027de036a5677b51d876928e86298330dc7c035830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000045830c41f7f918c40e1aa86409878244f7560be9397b30e200fddf74895e7e6279b7255e82a1d04dd3629c39395f2562e57210558300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000658300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000758300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000858300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000958300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006b63657274696669636174655902813082027d30820203a0030201020210019520e5d860dfe60000000067b680e0300a06082a8648ce3d04030330818f310b30090603550406130255533113301106035504080c0a57617368696e67746f6e3110300e06035504070c0753656174746c65310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753313a303806035504030c31692d30616563356332363863326134643630342e61702d736f7574682d312e6177732e6e6974726f2d656e636c61766573301e170d3235303232303031303934395a170d3235303232303034303935325a308194310b30090603550406130255533113301106035504080c0a57617368696e67746f6e3110300e06035504070c0753656174746c65310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753313f303d06035504030c36692d30616563356332363863326134643630342d656e63303139353230653564383630646665362e61702d736f7574682d312e6177733076301006072a8648ce3d020106052b8104002203620004fa7531335f2a3761e20219ac879dcd5191625b32c1e76d2e35ee427752c402d6bd3ac802670f2da2b393e12529449debac67876db606df20e08a96be4f43749922b808956a7ae0e360ea1d3a0bdf852f8c7f3087a9d3fcf1f8f137522d498d7aa31d301b300c0603551d130101ff04023000300b0603551d0f0404030206c0300a06082a8648ce3d040303036800306502305044e8f5725d21c59a45bf47e67ba8aa4f6dbed3ae5eb7a24e739ad1d0de25d37213a3a9d480c58f278c102f717e568f0231009f09c31558c9357ac21aa8112e717a90906d79323a6f559423eba4eea7e9e51ee48080118176456f36e6b2ead28ce83b68636162756e646c65845902153082021130820196a003020102021100f93175681b90afe11d46ccb4e4e7f856300a06082a8648ce3d0403033049310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753311b301906035504030c126177732e6e6974726f2d656e636c61766573301e170d3139313032383133323830355a170d3439313032383134323830355a3049310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753311b301906035504030c126177732e6e6974726f2d656e636c617665733076301006072a8648ce3d020106052b8104002203620004fc0254eba608c1f36870e29ada90be46383292736e894bfff672d989444b5051e534a4b1f6dbe3c0bc581a32b7b176070ede12d69a3fea211b66e752cf7dd1dd095f6f1370f4170843d9dc100121e4cf63012809664487c9796284304dc53ff4a3423040300f0603551d130101ff040530030101ff301d0603551d0e041604149025b50dd90547e796c396fa729dcf99a9df4b96300e0603551d0f0101ff040403020186300a06082a8648ce3d0403030369003066023100a37f2f91a1c9bd5ee7b8627c1698d255038e1f0343f95b63a9628c3d39809545a11ebcbf2e3b55d8aeee71b4c3d6adf3023100a2f39b1605b27028a5dd4ba069b5016e65b4fbde8fe0061d6a53197f9cdaf5d943bc61fc2beb03cb6fee8d2302f3dff65902c4308202c030820246a003020102021100e765410eace7b2af1b50037ca6f1d142300a06082a8648ce3d0403033049310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753311b301906035504030c126177732e6e6974726f2d656e636c61766573301e170d3235303231343231333631345a170d3235303330363232333631345a3065310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c034157533137303506035504030c2e636266306536323639383264313265362e61702d736f7574682d312e6177732e6e6974726f2d656e636c617665733076301006072a8648ce3d020106052b81040022036200047bcbc3a06d5bfd56dc292da24a6a000f0164eb506dd1c32327d734b0a681ebe0bdf2cdb1c7d6bd5dd643ee76554f99a7dfee909c9388c4884c4336e44b1d3b40eed583ca5ef9cf075c3982d86284487a10928ecc5641988e14efa7153b3252c9a381d53081d230120603551d130101ff040830060101ff020102301f0603551d230418301680149025b50dd90547e796c396fa729dcf99a9df4b96301d0603551d0e041604140b60d43b98a9eb9125b1db25c8c0e3e91d7da7b4300e0603551d0f0101ff040403020186306c0603551d1f046530633061a05fa05d865b687474703a2f2f6177732d6e6974726f2d656e636c617665732d63726c2e73332e616d617a6f6e6177732e636f6d2f63726c2f61623439363063632d376436332d343262642d396539662d3539333338636236376638342e63726c300a06082a8648ce3d0403030368003065023047bdfa14b0380cea2feabc41905e5e6e7f2837d379a5b739b23a817f3c0ac9d2f330b3d8355c1e4e49e4ef243ed2f6fa023100d82583fcd2967af57205dc6fda4ae99ad43303f30bdb146f3089e2a7432f2be42be32f3eca63ad7e4eaff1377faef1f959031d308203193082029fa00302010202110087788f29910bfd7bd3db57f3e7b359cf300a06082a8648ce3d0403033065310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c034157533137303506035504030c2e636266306536323639383264313265362e61702d736f7574682d312e6177732e6e6974726f2d656e636c61766573301e170d3235303231393138343834335a170d3235303232353132343834325a30818a313d303b06035504030c34356335646439303939366138326635322e7a6f6e616c2e61702d736f7574682d312e6177732e6e6974726f2d656e636c61766573310c300a060355040b0c03415753310f300d060355040a0c06416d617a6f6e310b3009060355040613025553310b300906035504080c0257413110300e06035504070c0753656174746c653076301006072a8648ce3d020106052b81040022036200045cf65c76ac361cecb72f79179c5425502abe9f6125c8c4ae2834363815cdfdc84109c1c1f303ce218d81c1d35bfd75e769e040b36a95ae9bca9e654ca53cddc916777e6132da939d0994c15a636c7a8ce16390893a0fa0b5dbe37ba550227b80a381ec3081e930120603551d130101ff040830060101ff020101301f0603551d230418301680140b60d43b98a9eb9125b1db25c8c0e3e91d7da7b4301d0603551d0e04160414bc9d6085864db379d56355479a89ec02cd678401300e0603551d0f0101ff0404030201863081820603551d1f047b30793077a075a0738671687474703a2f2f63726c2d61702d736f7574682d312d6177732d6e6974726f2d656e636c617665732e73332e61702d736f7574682d312e616d617a6f6e6177732e636f6d2f63726c2f62613939393664312d336232392d343335392d626138372d6365336137613730316335642e63726c300a06082a8648ce3d0403030368003065023100ce1d08a96e2ba9e1b173ddde4b300e54b0061ef248b5d9adc25eddc13925977bb85159fac6011ee06f357b243757c0f40230095f35d41186b43398fea08267550887a848b6dc126c9851aef5e1f0336638940c5352c3c8ef1c2fe8052e6177c690fa5902c5308202c130820246a00302010202145ace1e8c85cf0214805e469d4b8884f1a2862b81300a06082a8648ce3d04030330818a313d303b06035504030c34356335646439303939366138326635322e7a6f6e616c2e61702d736f7574682d312e6177732e6e6974726f2d656e636c61766573310c300a060355040b0c03415753310f300d060355040a0c06416d617a6f6e310b3009060355040613025553310b300906035504080c0257413110300e06035504070c0753656174746c65301e170d3235303232303031303631395a170d3235303232313031303631395a30818f310b30090603550406130255533113301106035504080c0a57617368696e67746f6e3110300e06035504070c0753656174746c65310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753313a303806035504030c31692d30616563356332363863326134643630342e61702d736f7574682d312e6177732e6e6974726f2d656e636c617665733076301006072a8648ce3d020106052b810400220362000430e2b64a098288bf26d5d4e7f7bed963c826da7490ec831a12fc254608bc8333ecd87d33e7d18e1e7a6120de56f4a787ec62a039f846ae77cadf598e03bceaa07c47c415f82e00ebd42ff52df46f3cecfd5f3e55a97a8791a1829ac3b5a6aadaa366306430120603551d130101ff040830060101ff020100300e0603551d0f0101ff040403020204301d0603551d0e04160414337cb30298c8a7928aa486859391013705276963301f0603551d23041830168014bc9d6085864db379d56355479a89ec02cd678401300a06082a8648ce3d0403030369003066023100c6b836c18661994fdebb47a8bbcb3205062430a77c1e963e1850a1a2e960934ea106e577c4eadb361cc9d0e2f3e8addd0231008558446d752a66529f582e77d634f32c8bc7bb602f73628467aa027c10b00d0d48cfb73fcb35b9003c093879776fb1896a7075626c69635f6b6579584037b0d0329a0d42304435f8b2e365e48b911a928a6897ebe00cd622280cd6915d1f380a70056e811f4aaa315c491bbdefcf35f33980670fc5ddc437172911ec0b69757365725f646174615901b87b22636f6e646974696f6e223a7b22636861696e223a312c22636f6e646974696f6e54797065223a22636f6e7472616374222c22636f6e747261637441646472657373223a22307835633237363134373730343136324538414137433644333136613531324435443644383834624363222c2266756e6374696f6e416269223a7b22696e70757473223a5b7b22696e7465726e616c54797065223a2261646472657373222c226e616d65223a22222c2274797065223a2261646472657373227d5d2c226e616d65223a2269735665726966696564222c226f757470757473223a5b7b22696e7465726e616c54797065223a22626f6f6c222c226e616d65223a22222c2274797065223a22626f6f6c227d5d2c2273746174654d75746162696c697479223a2276696577222c2274797065223a2266756e6374696f6e227d2c226d6574686f64223a2269735665726966696564222c22706172616d6574657273223a5b223a7573657241646472657373225d2c2272657475726e56616c756554657374223a7b22636f6d70617261746f72223a223d3d222c2276616c7565223a747275657d7d2c2276657273696f6e223a22312e302e30227d656e6f6e6365f658604251bd0ac99dfaae492acd1221eed173b02f095f813b125e8c9cb2f64f0fc8c2e2d71352866136222aafeee8cb6c26c96b1d0fb0192b2ab0e88fd3a7bf93f52ff20b67039268c57e6cabd80fd4b86f52e4d1b3d07ac25824656ac6af613b0f6d").unwrap_or_default();
        let parsed_result = parse_attestation_doc(&attestation_vec);
        assert!(parsed_result.is_ok(), "Expected Ok(_)");

        let (_, _, doc) = parsed_result.unwrap();

        match serde_json::to_string_pretty(&doc.to_attestation_response()) {
            Ok(json_string) => {
                // Step 3: Print the JSON string
                println!("AttestationResponse as JSON:\n{}", json_string);
            }
            Err(e) => {
                eprintln!("Error serializing AttestationResponse: {}", e);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttestationData {
    pcrs: Option<BTreeMap<u32, Value>>,
    nonce: Option<Value>,
    digest: Option<Value>,
    cabundle: Option<Value>,
    module_id: Option<Value>,
    timestamp: Option<Value>,
    user_data: Option<Value>,
    public_key: Option<Value>,
    certificate: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttestationResponse {
    pcrs: Option<Vec<String>>,
    nonce: Option<String>,
    digest: Option<String>,
    cabundle: Option<String>,
    module_id: Option<String>,
    timestamp: Option<String>,
    user_data: Option<String>,
    public_key: Option<String>,
    certificate: Option<String>,
}

impl AttestationData {
    pub fn to_attestation_response(&self) -> AttestationResponse {
        // Process PCRs by converting each value with our helper.
        let pcrs = self.pcrs.as_ref().map(|map| {
            map.values()
                .map(|value| to_option_string_inner(value).unwrap_or_default())
                .collect::<Vec<_>>()
        });

        // For nonce, module_id, timestamp, user_data, and public_key we use the helper.
        let nonce = to_option_string(&self.nonce);
        let module_id = to_option_string(&self.module_id);
        let timestamp = to_option_string(&self.timestamp);
        let public_key = to_option_string(&self.public_key);

        // For userdata we are assuming utf8 data
        let user_data = self.user_data.as_ref().and_then(|ud| match ud {
            serde_cbor::Value::Bytes(bytes) => {
                std::str::from_utf8(bytes).ok().map(|s| s.to_string())
            }
            serde_cbor::Value::Text(s) => Some(s.clone()),
            _ => to_option_string_inner(&ud),
        });

        // For digest we expect a text value.
        let digest = self.digest.as_ref().and_then(|d| {
            if let serde_cbor::Value::Text(val) = d {
                Some(val.clone())
            } else {
                to_option_string_inner(d)
            }
        });

        // For cabundle we process it separately.
        let cabundle = self.cabundle.as_ref().and_then(|cb| {
            process_cabundle(cb)
                .ok()
                .or_else(|| to_option_string_inner(cb))
        });

        // Certificate gets special handling: if it is bytes we convert from DER to PEM.
        let certificate = self.certificate.as_ref().and_then(|cert| {
            match cert {
                serde_cbor::Value::Bytes(val) => Some(der_to_pem(val)),
                _ => to_option_string_inner(cert), // fallback conversion if not bytes
            }
        });

        AttestationResponse {
            pcrs,
            nonce,
            digest,
            cabundle,
            module_id,
            timestamp,
            user_data,
            public_key,
            certificate,
        }
    }
}

fn process_cabundle(cabundle: &Value) -> Result<String, String> {
    match cabundle {
        Value::Array(cert_values) => {
            let mut pem_bundle = String::new();

            for cert_value in cert_values {
                match cert_value {
                    Value::Bytes(der_bytes) => {
                        let pem = der_to_pem(der_bytes);
                        pem_bundle.push_str(&pem);
                        pem_bundle.push('\n'); // Ensure separation between certificates
                    }
                    _ => return Err("Expected each CA bundle element to be Bytes".to_string()),
                }
            }

            Ok(pem_bundle)
        }
        _ => Err("Expected cabundle to be an Array".to_string()),
    }
}

use pem::{encode as pem_encode, Pem};

use crate::oyster::{self, AttestationError};

fn der_to_pem(der_bytes: &[u8]) -> String {
    let pem = Pem {
        tag: String::from("CERTIFICATE"),
        contents: der_bytes.to_vec(),
    };
    pem_encode(&pem)
}

/// Helper function that converts a reference to a `serde_cbor::Value` into an Option<String>
fn to_option_string_inner(value: &serde_cbor::Value) -> Option<String> {
    match value {
        serde_cbor::Value::Bytes(vec) => Some(hex::encode(vec)),
        serde_cbor::Value::Null => None,
        serde_cbor::Value::Bool(b) => Some(b.to_string()),
        serde_cbor::Value::Integer(i) => Some(i.to_string()),
        serde_cbor::Value::Float(f) => Some(f.to_string()),
        serde_cbor::Value::Text(s) => Some(s.clone()),
        serde_cbor::Value::Array(arr) => {
            // Recursively convert each element.
            let converted: Option<Vec<String>> = arr.iter().map(to_option_string_inner).collect();
            converted.map(|vec| format!("[{}]", vec.join(", ")))
        }
        serde_cbor::Value::Map(map) => {
            let mut pairs = Vec::new();
            for (key, val) in map.iter() {
                let key_str = match key {
                    serde_cbor::Value::Text(s) => s.clone(),
                    _ => format!("{:?}", key), // Use Debug formatting for non-text keys.
                };
                let val_str = to_option_string_inner(val)?;
                pairs.push(format!("{}: {}", key_str, val_str));
            }
            Some(format!("{{{}}}", pairs.join(", ")))
        }
        serde_cbor::Value::Tag(tag, inner) => {
            to_option_string_inner(inner).map(|s| format!("Tag {}: {}", tag, s))
        }
        _ => None,
    }
}

/// Public function that accepts a reference to an Option<serde_cbor::Value>.
/// If the value is Some, it converts it using the helper function;
/// if it's None, it returns None.
pub fn to_option_string(value: &Option<serde_cbor::Value>) -> Option<String> {
    value.as_ref().and_then(to_option_string_inner)
}
