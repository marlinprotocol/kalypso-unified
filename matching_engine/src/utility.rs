use ethers::core::utils::hex::FromHex;
use ethers::types::{Signature, SignatureError, H160};
use ethers::utils::keccak256;
use hex::decode;
use std::error::Error;

// fn ecrecover_from_signature(signature: &str, message_hash: &[u8]) -> Option<k256::ecdsa::VerifyingKey> {
//     // Parse the signature from a hex string
//     let signature = Signature::from_str(signature).ok()?;

//     // Use the `recover` function from ethers to recover the public key
//     let recovered_key = recover(
//         recoverable::Signature {
//             r: signature.r(),
//             s: signature.s(),
//             v: signature.recovery_id().into(),
//         },
//         &message_hash,
//     );

//     // Return the result
//     recovered_key.ok()
// }

// fn derive_address_from_public_key(public_key: &[u8]) -> String {
//     // Step 1: Hash the public key using Keccak-256
//     let hash = keccak256(public_key);

//     // Step 2: Take the last 20 bytes of the hash
//     let address_bytes = &hash[hash.len() - 20..];

//     // Step 3: Convert the bytes to a hexadecimal string
//     let address_hex = hex::encode(address_bytes);

//     // Step 4: Prefix with "0x" to represent it as an Ethereum address
//     let address = format!("0x{}", address_hex);

//     address
// }

// Derive Ethereum address from signature and message
pub fn derive_address_from_signature(
    signature_str: &str,
    message: &str,
) -> Result<H160, SignatureError> {
    // Parse the signature from a hex string
    let signature =
        string_to_signature(signature_str).expect("Failed to convert hex string to Signature");

    let message_hash = keccak256(&message);

    let recovered = signature.recover(message_hash);

    recovered
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
        v: v as u64,
    };

    Ok(signature)
}

pub fn ivs_family_id(market_id: &str) -> [u8; 32] {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"ivs");
    bytes.extend_from_slice(market_id.as_bytes());
    keccak256(bytes)
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
