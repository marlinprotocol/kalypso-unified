use ethers::abi::{encode, Token};
use ethers::core::rand;
use ethers::core::utils::hex::FromHex;
use ethers::types::{Address, Signature, SignatureError, H160, U256};
use ethers::utils::keccak256;
use hex::decode;
use im::HashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};
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

pub fn random_u256() -> U256 {
    // Define lower and upper bounds as u128
    const LOWER_BOUND: u128 = 10u128.pow(18);
    const UPPER_BOUND: u128 = 10u128.pow(22);

    // Calculate the range
    let range = UPPER_BOUND - LOWER_BOUND;

    // Initialize the random number generator
    let mut rng = rand::thread_rng();

    // Generate a random u128 within [0, range)
    let rand_u128: u128 = rng.gen_range(0..range);

    // Shift the random number into the desired range
    let result_u128 = rand_u128 + LOWER_BOUND;

    // Convert u128 to U256
    let result = U256::from(result_u128);

    result
}

pub fn random_usize() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=256)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenAmount {
    pub token: String,
    pub amount: String,
}

pub type AddressTokenPair = (Address, U256);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct TokenTracker {
    tokens: HashMap<Address, U256>,
}

impl TokenTracker {
    pub fn new() -> Self {
        TokenTracker {
            tokens: HashMap::new(),
        }
    }

    pub fn from_address_string_and_dec_string(
        addresses: Vec<String>,
        values: Vec<String>,
    ) -> Result<Self, String> {
        // Ensure both vectors have the same length
        if addresses.len() != values.len() {
            return Err(format!(
                "Mismatched number of addresses ({}) and values ({}).",
                addresses.len(),
                values.len()
            ));
        }

        let mut token_tracker = TokenTracker::new();

        // Iterate over both addresses and values
        for (address_str, value_str) in addresses.iter().zip(values.iter()) {
            // Parse the address from a hex string
            let address = address_str
                .parse()
                .map_err(|_| format!("Invalid address format: {}", address_str))?;

            // Parse the value from a decimal string
            let amount = U256::from_dec_str(value_str)
                .map_err(|_| format!("Invalid decimal value format: {}", value_str))?;

            // Add the token to the tracker
            token_tracker.add_token(&address, &amount);
        }

        Ok(token_tracker)
    }
}

impl TokenTracker {
    pub fn to_address_token_pair(&self) -> Vec<AddressTokenPair> {
        self.tokens
            .iter()
            .map(|(address, amount)| (*address, *amount)) // Create AddressTokenPair tuples
            .collect() // Collect into a Vec<AddressTokenPair>
    }

    pub fn to_token_amount(&self) -> Vec<TokenAmount> {
        self.tokens
            .iter()
            .map(|(token, amount)| TokenAmount {
                token: address_to_string(token),
                amount: amount.to_string(),
            })
            .collect()
    }

    pub fn add_token(&mut self, token: &Address, amount: &U256) {
        let entry = self.tokens.entry(*token).or_insert(U256::zero());
        *entry += *amount; // Increment the token amount
    }

    pub fn sub_token(&mut self, token: &Address, amount: &U256) -> Result<(), String> {
        if let Some(entry) = self.tokens.get_mut(token) {
            if *entry >= *amount {
                *entry -= *amount; // Decrement the token amount
                Ok(())
            } else {
                Err(format!("Insufficient balance for token: {:?}", token))
            }
        } else {
            Err(format!("Token not found: {:?}", token))
        }
    }

    // Subtract tokens from the tracker, but set balance to 0 in case of underflow
    pub fn sub_token_saturating(&mut self, token: &Address, amount: &U256) {
        if let Some(entry) = self.tokens.get_mut(token) {
            if *entry >= *amount {
                *entry -= *amount; // Normal subtraction
            } else {
                *entry = U256::zero(); // Set to 0 if subtraction would underflow
            }
        }
    }
}

use std::ops::{Add, AddAssign, Sub};

// Implement Add trait for TokenTracker (TokenTracker + TokenTracker)
impl Add for TokenTracker {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self.clone();

        // Iterate over the other TokenTracker's tokens
        for (address, amount) in other.tokens.iter() {
            result.add_token(address, amount); // Use the updated add_token method
        }

        result
    }
}

// Implement Sub trait for TokenTracker (TokenTracker - TokenTracker)
impl Sub for TokenTracker {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self.clone();

        // Iterate over the other TokenTracker's tokens
        for (address, amount) in other.tokens.iter() {
            result.sub_token_saturating(address, amount); // Use the updated sub_token_saturating method
        }

        result
    }
}

impl TokenTracker {
    // Check if the tracker has more or equal amount than the given address-token pair
    pub fn has_more_than_or_eq(&self, address_token_pair: &AddressTokenPair) -> bool {
        let (address, amount) = address_token_pair;
        if let Some(current_amount) = self.tokens.get(address) {
            return current_amount >= amount;
        }
        false
    }

    // Check if the tracker has less or equal amount than the given address-token pair
    pub fn has_less_than_or_eq(&self, address_token_pair: &AddressTokenPair) -> bool {
        let (address, amount) = address_token_pair;
        if let Some(current_amount) = self.tokens.get(address) {
            return current_amount <= amount;
        }
        false
    }
}

impl TokenTracker {
    fn sorted_tokens(&self) -> Vec<(Address, U256)> {
        let mut tokens_vec: Vec<(Address, U256)> =
            self.tokens.iter().map(|(k, v)| (*k, *v)).collect();
        // Sort the vector first by address, then by value
        tokens_vec.sort_by(|a, b| {
            let addr_cmp = a.0.cmp(&b.0); // Compare addresses
            if addr_cmp == Ordering::Equal {
                a.1.cmp(&b.1) // If addresses are equal, compare values
            } else {
                addr_cmp
            }
        });
        tokens_vec
    }
}

use std::cmp::Ordering;
// Implement the Ord and PartialOrd traits for TokenTracker
impl Ord for TokenTracker {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sorted_tokens().cmp(&other.sorted_tokens())
    }
}

impl PartialOrd for TokenTracker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use std::fmt;
impl fmt::Display for TokenTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (token, amount) in &self.tokens {
            writeln!(f, "{:?}: {}", token, amount)?;
        }
        Ok(())
    }
}

impl AddAssign for TokenTracker {
    fn add_assign(&mut self, other: Self) {
        for (address, amount) in other.tokens {
            self.add_token(&address, &amount); // Use the add_token method to add tokens
        }
    }
}

pub const POND: &str = "POND";
pub const TEST_TOKEN_ADDRESS_STRING: &str = "0x9999888899998888999988889999888899998888";

use once_cell::sync::Lazy;
pub static TEST_TOKEN_ADDRESS: Lazy<Address> =
    Lazy::new(|| TEST_TOKEN_ADDRESS_STRING.parse::<Address>().unwrap());
