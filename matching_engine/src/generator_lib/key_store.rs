use ethers::core::types::Address;
use ethers::prelude::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone)]
pub struct Key {
    pub address: Address,
    key_index: u64,
    ecies_pub_key: Option<Bytes>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct KeyInfo {
    pub address: String,
    key_index: String,
    ecies_pub_key: Option<String>,
}

impl Key {
    pub fn to_key_info(&self) -> KeyInfo {
        let ecies_pub_key = {
            if self.ecies_pub_key.is_none() {
                None
            } else {
                Some(hex::encode(&self.ecies_pub_key().unwrap().to_vec()))
            }
        };
        KeyInfo {
            address: format!("{:?}", self.address),
            key_index: self.key_index.to_string(),
            ecies_pub_key,
        }
    }
}

impl Key {
    pub fn ecies_pub_key(&self) -> Option<Bytes> {
        self.ecies_pub_key.clone()
    }

    pub fn new(address: Address, key_index: u64, ecies_pub_key: Option<Bytes>) -> Self {
        Self {
            address,
            key_index,
            ecies_pub_key,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct KeyStore {
    keys: HashMap<(Address, u64), Key>, // Using u64 as a stand-in for uint256.
}

impl KeyStore {
    pub fn new() -> Self {
        KeyStore {
            keys: HashMap::new(),
        }
    }

    pub fn insert(&mut self, address: Address, value: u64, key: Key) {
        self.keys.insert((address, value), key);
    }

    pub fn get_by_address(&self, address: &Address, value: u64) -> Option<Key> {
        self.keys.get(&(*address, value)).cloned()
    }

    pub fn remove_by_address(&mut self, address: &Address, value: u64) {
        self.keys.remove(&(*address, value));
    }

    pub fn update_pub_key(&mut self, address: &Address, value: u64, new_pub_key: Option<Bytes>) {
        if let Some(key) = self.keys.get_mut(&(*address, value)) {
            key.ecies_pub_key = new_pub_key;
        }
    }
}

// Implement custom Serialize for KeyStore
impl Serialize for KeyStore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert the HashMap into a Vec<Key>
        let keys_vec: Vec<&Key> = self.keys.values().collect();

        // Serialize the keys_vec under the "keys" field
        let mut state = serializer.serialize_struct("KeyStore", 1)?;
        state.serialize_field("keys", &keys_vec)?;
        state.end()
    }
}

// Implement custom Deserialize for KeyStore
impl<'de> Deserialize<'de> for KeyStore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyStoreVisitor;

        impl<'de> Visitor<'de> for KeyStoreVisitor {
            type Value = KeyStore;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct KeyStore")
            }

            fn visit_map<V>(self, mut map: V) -> Result<KeyStore, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut keys = None;

                while let Some(field) = map.next_key::<String>()? {
                    if field == "keys" {
                        if keys.is_some() {
                            return Err(de::Error::duplicate_field("keys"));
                        }
                        let keys_vec = map.next_value::<Vec<Key>>()?;
                        let mut keys_map = HashMap::new();
                        for key in keys_vec {
                            // Use (address, key_index) as the key in the HashMap
                            keys_map.insert((key.address.clone(), key.key_index), key);
                        }
                        keys = Some(keys_map);
                    } else {
                        return Err(de::Error::unknown_field(&field, &["keys"]));
                    }
                }

                let keys = keys.ok_or_else(|| de::Error::missing_field("keys"))?;
                Ok(KeyStore { keys })
            }
        }

        const FIELDS: &'static [&'static str] = &["keys"];
        deserializer.deserialize_struct("KeyStore", FIELDS, KeyStoreVisitor)
    }
}
