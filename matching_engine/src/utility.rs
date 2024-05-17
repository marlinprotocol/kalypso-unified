use ethers::{
    core::k256,
    crypto::{recover, signer::recoverable},
    utils::{keccak256, parse_address},
};

fn ecrecover_from_signature(signature: &str, message_hash: &[u8]) -> Option<k256::ecdsa::VerifyingKey> {
    // Parse the signature from a hex string
    let signature = Signature::from_str(signature).ok()?;

    // Use the `recover` function from ethers to recover the public key
    let recovered_key = recover(
        recoverable::Signature {
            r: signature.r(),
            s: signature.s(),
            v: signature.recovery_id().into(),
        },
        &message_hash,
    );

    // Return the result
    recovered_key.ok()
}


fn derive_address_from_public_key(public_key: &[u8]) -> String {
    // Step 1: Hash the public key using Keccak-256
    let hash = keccak256(public_key);
    
    // Step 2: Take the last 20 bytes of the hash
    let address_bytes = &hash[hash.len() - 20..];
    
    // Step 3: Convert the bytes to a hexadecimal string
    let address_hex = hex::encode(address_bytes);
    
    // Step 4: Prefix with "0x" to represent it as an Ethereum address
    let address = format!("0x{}", address_hex);
    
    address
}

// Derive Ethereum address from signature and message
pub fn derive_address_from_signature(signature: &str, message: &str) -> Option<String> {
    // Parse the signature from a hex string
    let signature = k256::ecdsa::Signature::from_bytes(&hex::decode(signature).ok()?);

    // Hash the message using Keccak-256
    let message_hash = keccak256(message.as_bytes());

    let recovered = signature.recover(message_hash)?;
    // Recover the public key from the signature
    // let recovered_key = recover(
    //     recoverable::Signature {
    //         r: signature.r(),
    //         s: signature.s(),
    //         v: signature.recovery_id().into(),
    //     },
    //     &message_hash,
    // ).ok()?;

    // // Derive the Ethereum address from the public key
    // let address_bytes = &keccak256(&recovered_key.to_bytes()[1..])[12..];
    // let address_hex = hex::encode(address_bytes);
    // let address = parse_address(&address_hex).ok()?;
    recovered
    
}