use ethers::abi::{encode, Address, Token};

#[cfg(feature = "use_l1_block_numbers")]
use ethers::abi::AbiParser;

use ethers::core::rand::seq::SliceRandom;
use ethers::core::rand::{self, thread_rng};
use ethers::core::utils::hex::FromHex;

use ethers::prelude::*;

#[cfg(feature = "use_l1_block_numbers")]
use ethers::types::{Address as OtherAddress, Bytes, TransactionRequest};

use ethers::types::{Signature, SignatureError, H160, U256};

use ethers::utils::keccak256;
use hex::decode;
use im::HashMap;
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use utoipa::ToSchema;

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

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct TokenAmount {
    /// Address of the token
    pub token: String,
    /// Amount of tokens
    pub amount: String,
}

pub type AddressTokenPair = (Address, U256);

pub fn address_token_pair_to_token_amount(pair: AddressTokenPair) -> TokenAmount {
    TokenAmount {
        token: address_to_string(&pair.0),
        amount: pair.1.to_string(),
    }
}

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

    pub fn from_address_token_pair(pair: AddressTokenPair) -> Self {
        let mut token_tracker = TokenTracker::new();

        token_tracker.add_token(&pair.0, &pair.1);

        token_tracker
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

    #[deprecated(note = "Use `add_token` instead")]
    pub fn replace_token(&mut self, token: &Address, amount: &U256) {
        log::debug!("Replace token value is being called in TokenTracker, avoid using this function and modify the app's code");
        let entry = self.tokens.entry(*token).or_insert(U256::zero());
        *entry = *amount; // replace with  whatever value provided.
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

    #[allow(unused)]
    #[deprecated(
        note = "Use `sub_token` instead. If whole indexer is right, sub_token should work"
    )]
    pub fn sub_token_saturating(&mut self, token: &Address, amount: &U256) {
        log::debug!("using sub_token_saturation in TokenTracker. User sub_token instead");
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
            if result.tokens.contains_key(address) {
                // unwrapping to find the error if any
                result.sub_token(address, amount).unwrap();
            }
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

    pub fn has_more_than_or_eq_across_multiple(
        &self,
        address_token_pairs: &Vec<AddressTokenPair>,
    ) -> bool {
        // Iterate over each pair and ensure all satisfy the `has_more_than_or_eq` condition
        address_token_pairs
            .iter()
            .all(|pair| self.has_more_than_or_eq(pair))
    }

    pub fn has_more_than_or_eq_in_at_least_one(
        &self,
        address_token_pairs: &Vec<AddressTokenPair>,
    ) -> bool {
        // Iterate over each pair and check if any satisfy the `has_more_than_or_eq` condition
        address_token_pairs
            .iter()
            .any(|pair| self.has_more_than_or_eq(pair))
    }

    pub fn select_one_random_stakable_pair(
        &self,
        address_token_pairs: &Vec<AddressTokenPair>,
    ) -> Option<AddressTokenPair> {
        // Collect all pairs where the tracker has less than the specified amount
        let less_pairs: Vec<&AddressTokenPair> = address_token_pairs
            .iter()
            .filter(|pair| self.has_more_than_or_eq(pair))
            .collect();

        // If no pairs satisfy the condition, return None
        if less_pairs.is_empty() {
            return None;
        }

        // Initialize the random number generator
        let mut rng = thread_rng();

        // Select a random pair from the filtered list and clone it
        less_pairs.choose(&mut rng).cloned().cloned() // Ensure AddressTokenPair implements `Clone`
    }

    // Check if the tracker has less or equal amount than the given address-token pair
    pub fn has_less_than_or_eq(&self, address_token_pair: &AddressTokenPair) -> bool {
        let (address, amount) = address_token_pair;
        if let Some(current_amount) = self.tokens.get(address) {
            return current_amount <= amount;
        }
        false
    }

    pub fn has_less_than_or_eq_across_multiple(
        &self,
        address_token_pairs: &Vec<AddressTokenPair>,
    ) -> bool {
        // Iterate over each pair and ensure all satisfy the `has_less_than_or_eq` condition
        address_token_pairs
            .iter()
            .all(|pair| self.has_less_than_or_eq(pair))
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

#[cfg(any(feature = "stagenet", feature = "beta"))]
pub const USDC_TOKEN_STRING: &str = "0x8230d71d809718132C2054704F5E3aF1b86B669C";

#[cfg(all(feature = "mainnet", not(any(feature = "stagenet", feature = "beta"))))]
pub const USDC_TOKEN_STRING: &str = "0xaf88d065e77c8cC2239327C5EDb3A432268e5831";

#[cfg(not(feature = "add_timestamp_to_asks"))]
pub async fn get_timestamp_from_l2block_number(_: &str, _: &U256) -> Option<U256> {
    None
}

#[cfg(feature = "add_timestamp_to_asks")]
pub async fn get_timestamp_from_l2block_number(rpc_url: &str, l2_block_num: &U256) -> Option<U256> {
    get_block_timestamp(rpc_url, l2_block_num).await
}

pub async fn get_block_timestamp(rpc_url: &str, block_num: &U256) -> Option<U256> {
    let provider = match Provider::<Http>::try_from(rpc_url) {
        Ok(data) => data,
        _ => return None,
    };

    let timestamp = {
        let block = provider.get_block(block_num.as_u64()).await;
        if block.is_err() {
            None
        } else {
            let block = block.unwrap();
            if block.is_none() {
                None
            } else {
                let block = block.unwrap();
                Some(block.timestamp)
            }
        }
    };

    timestamp
}

#[cfg(not(feature = "use_l1_block_numbers"))]
pub async fn get_l1_block_from_l2_block(
    _: &str,
    l2_block_num: U256,
) -> Result<U256, Box<dyn Error>> {
    Ok(l2_block_num)
}

#[cfg(feature = "use_l1_block_numbers")]
pub async fn get_l1_block_from_l2_block(rpc_url: &str, l2_block_num: U256) -> Option<U256> {
    // Connect to Arbitrum's L2 endpoint
    let provider = Provider::<Http>::try_from(rpc_url).ok()?;

    // Define ABI for blockL1Num function
    let abi = AbiParser::default()
        .parse(&["function blockL1Num(uint64 l2BlockNum) view returns (uint256)"])
        .ok()?;

    // Retrieve the function
    let function = abi.function("blockL1Num").ok()?;

    // Encode the data for the function call
    let data = function.encode_input(&[Token::Uint(l2_block_num)]).ok()?;

    // NodeInterface special address for the call
    let node_interface_address = "0x00000000000000000000000000000000000000c8"
        .parse::<OtherAddress>()
        .ok()?;

    let data_bytes = Bytes::from(data);

    // Create a TransactionRequest
    let tx_request = TransactionRequest::new()
        .to(node_interface_address)
        .data(data_bytes);

    // Call the function via the provider
    let result = provider.call(&tx_request.into(), None).await.ok()?;

    // Decode the result
    let decoded_result: U256 = function
        .decode_output(&result)
        .ok()?
        .get(0)
        .cloned()?
        .into_uint()?;

    Some(decoded_result)
}

use once_cell::sync::Lazy;
pub static USDC_TOKEN: Lazy<Address> = Lazy::new(|| USDC_TOKEN_STRING.parse::<Address>().unwrap());

impl TokenTracker {
    pub fn get_balance(&self, address: &Address) -> U256 {
        let balance = self.tokens.get(address);
        if balance.is_none() {
            return 0.into();
        }

        return balance.unwrap().clone();
    }
}

pub fn convert_to_option_string(timestamp: Option<U256>) -> Option<String> {
    timestamp.and_then(|ts| {
        if ts == U256::zero() {
            None
        } else {
            Some(ts.to_string())
        }
    })
}

impl TokenTracker {
    pub fn force_set(&mut self, token: Address, amount: U256) {
        self.tokens.insert(token, amount);
    }

    pub fn force_remove(&mut self, token: Address) {
        self.tokens.remove(&token);
    }
}

/// Serialize a `HashMap<U256, V>` by converting `U256` keys to hexadecimal strings.
pub fn serialize_hashmap_u256<V, S>(
    map: &HashMap<U256, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    V: Serialize,
    S: Serializer,
{
    // Convert each U256 key to a hexadecimal string with "0x" prefix.
    let string_map: HashMap<String, &V> =
        map.iter().map(|(k, v)| (format!("{:#x}", k), v)).collect();
    string_map.serialize(serializer)
}

pub fn serialize_u256_map<V, S>(map: &HashMap<U256, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    V: Serialize,
    S: Serializer,
{
    let string_map: HashMap<String, &V> =
        map.iter().map(|(k, v)| (format!("{:#x}", k), v)).collect();
    string_map.serialize(serializer)
}

// Custom deserialization function
pub fn deserialize_u256_map<'de, V, D>(deserializer: D) -> Result<HashMap<U256, V>, D::Error>
where
    V: Deserialize<'de> + std::clone::Clone,
    D: Deserializer<'de>,
{
    // Deserialize into a temporary `HashMap<String, V>`.
    let string_map: HashMap<String, V> = HashMap::deserialize(deserializer)?;
    let mut map = HashMap::new();

    for (k, v) in string_map {
        // Remove the "0x" prefix if present.
        let key_str = k.strip_prefix("0x").unwrap_or(&k);
        // Parse the hexadecimal string back into a `U256`.
        let u256 = U256::from_str_radix(key_str, 16).map_err(serde::de::Error::custom)?;
        map.insert(u256, v);
    }

    Ok(map)
}

use std::time::{Duration, SystemTime, UNIX_EPOCH};
pub fn u256_to_system_time(timestamp: U256) -> SystemTime {
    // Convert U256 to u64. Ensure that your timestamp really fits in u64!
    let timestamp_secs = timestamp.as_u64();
    UNIX_EPOCH + Duration::from_secs(timestamp_secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_block_number_to_system_time() {
        let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
        let block_timestamp =
            get_block_timestamp(rpc_url, &U256::from_dec_str("127755465").unwrap())
                .await
                .unwrap_or_default();

        println!("{}", block_timestamp);
        let missed_at_time = u256_to_system_time(block_timestamp);

        println!("{:?}", missed_at_time);
    }
}
