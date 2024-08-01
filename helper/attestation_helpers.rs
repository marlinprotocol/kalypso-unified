use ethers::abi::Token;
use ethers::core::types::Bytes;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{H256, U256};
use ethers::utils::keccak256;
use secp256k1::SecretKey;

pub async fn convert_to_tee_attested_proofs<S>(
    public_inputs: Bytes,
    proof: Bytes,
    signer: S,
) -> Result<Bytes, Box<dyn std::error::Error>>
where
    S: Signer + Clone,
{
    let value = vec![
        Token::Bytes(public_inputs.to_vec()),
        Token::Bytes(proof.to_vec()),
    ];

    let encoded = ethers::abi::encode(&value);
    let digest = keccak256(encoded);

    let signature = signer.sign_message(H256(digest)).await.unwrap();

    let value = vec![
        Token::Bytes(public_inputs.to_vec()),
        Token::Bytes(proof.to_vec()),
        Token::Bytes(signature.to_vec()),
    ];
    let encoded = ethers::abi::encode(&value);
    Ok(encoded.into())
}

pub async fn convert_to_tee_attested_proofs_with_enclave_key(
    public_inputs: Bytes,
    proof: Bytes,
    secp_private_key: Vec<u8>,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    let secp_private_key = SecretKey::from_slice(&secp_private_key)
        .unwrap()
        .display_secret()
        .to_string();
    let signer_wallet = secp_private_key.parse::<LocalWallet>().unwrap();

    convert_to_tee_attested_proofs(public_inputs, proof, signer_wallet).await
}

pub async fn generate_invalid_input_attestation<S>(
    ask_id: U256,
    public_inputs: Bytes,
    signer: S,
) -> String
where
    S: Signer + Clone,
{
    let value = vec![Token::Uint(ask_id), Token::Bytes(public_inputs.to_vec())];
    let encoded = ethers::abi::encode(&value);
    let digest = keccak256(encoded);

    let signature = signer.sign_message(H256(digest)).await.unwrap();

    signature.to_string()
}

pub async fn generate_invalid_input_attestation_with_private_key(
    ask_id: U256,
    public_inputs: Bytes,
    secp_private_key: Vec<u8>,
) -> String {
    let secp_private_key = secp256k1::SecretKey::from_slice(&secp_private_key)
        .unwrap()
        .display_secret()
        .to_string();
    let signer_wallet = secp_private_key.parse::<LocalWallet>().unwrap();

    generate_invalid_input_attestation(ask_id, public_inputs, signer_wallet).await
}
