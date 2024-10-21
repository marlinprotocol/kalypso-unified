use ethers::abi::{encode, Token};
use ethers::core::utils::hex::FromHex;
use ethers::types::{Address, Signature, SignatureError, H160, U256};
use ethers::utils::keccak256;
use hex::decode;
use std::error::Error;

pub fn address_to_string(address: &Address) -> String {
    format!("0x{}", hex::encode(address.as_bytes()))
}

pub fn tx_to_string(tx: &ethers::types::H256) -> String {
    format!("0x{}", hex::encode(tx.as_bytes()))
}

pub fn bytes_to_string(data: &ethers::types::Bytes) -> String {
    format!("0x{}", hex::encode(data))
}

// Derive Ethereum address from signature and message
pub fn derive_address_from_signature(
    signature_str: &str,
    message: &str,
) -> Result<H160, SignatureError> {
    // Parse the signature from a hex string
    let signature =
        string_to_signature(signature_str).expect("Failed to convert hex string to Signature");

    let message_hash = keccak256(message);

    signature.recover(message_hash)
}

fn string_to_signature(sig_str: &str) -> Result<Signature, Box<dyn Error>> {
    // Ensure the string is the correct length (130 characters: 64 for r, 64 for s, 2 for v)
    if sig_str.len() != 130 {
        return Err("Invalid signature length".into());
    }

    // Split the signature string into r, s, and v components
    let r_str = &sig_str[0..64];
    let s_str = &sig_str[64..128];
    let v_str = &sig_str[128..130];

    // Parse the hex strings into bytes
    let r = <[u8; 32]>::from_hex(r_str)?;
    let s = <[u8; 32]>::from_hex(s_str)?;
    let v = u64::from_str_radix(v_str, 16)?;

    // Create the Signature struct
    let signature = Signature {
        r: r.into(),
        s: s.into(),
        v,
    };

    Ok(signature)
}

pub fn ivs_family_id(market_id: &str) -> [u8; 32] {
    let market_id = U256::from_dec_str(market_id).expect("Invalid market_id");
    let tokens = vec![Token::String("ivs".to_string()), Token::Uint(market_id)];
    let encoded = encode(&tokens);

    keccak256(encoded)
}

pub fn public_key_to_address(public_key_hex: &str) -> Result<H160, hex::FromHexError> {
    // Decode the public key from hex string to byte array
    let public_key_bytes = decode(public_key_hex)?;

    // Ensure the public key is 65 bytes (uncompressed format)
    assert_eq!(
        public_key_bytes.len(),
        65,
        "Public key should be 65 bytes in uncompressed format"
    );

    // Keccak256 hash of the public key (skipping the first byte which is 0x04)
    let hash = keccak256(&public_key_bytes[1..]);

    // Take the last 20 bytes of the hash to get the address
    Ok(H160::from_slice(&hash[12..]))
}
