use ethers::core::types::Address;
use ethers::prelude::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::generator_lib::key_store::{Key, KeyStoreOperations};

#[derive(Debug, Clone)]
pub struct KeyStore {
    keys: HashMap<(Address, u64), Key>, // Using u64 as a stand-in for uint256.
}

impl KeyStore {
    fn new() -> Self {
        KeyStore {
            keys: HashMap::new(),
        }
    }
}

impl Default for KeyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyStoreOperations for KeyStore {
    fn insert(&mut self, address: Address, value: u64, key: Key) {
        self.keys.insert((address, value), key);
    }

    fn get_by_address(&self, address: &Address, value: u64) -> Option<Key> {
        self.keys.get(&(*address, value)).cloned()
    }

    fn remove_by_address(&mut self, address: &Address, value: u64) {
        self.keys.remove(&(*address, value));
    }

    fn update_pub_key(&mut self, address: &Address, value: u64, new_pub_key: Option<Bytes>) {
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
