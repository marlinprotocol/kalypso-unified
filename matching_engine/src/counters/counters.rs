use im::HashSet;
use serde::Deserialize;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default, Clone)]
pub struct GenericCounters<TKey, TValue> {
    total_values: HashSet<TValue>, // Tracks all unique values (e.g., requestors) across all keys
    key_wise: HashMap<TKey, HashSet<TValue>>, // Maps keys (e.g., markets) to sets of values (e.g., requestors)
}

impl<TKey, TValue> GenericCounters<TKey, TValue>
where
    TKey: Eq + Hash + Clone + Send + Sync, // Key type must implement Eq, Hash, and Clone for use in HashMap
    TValue: Eq + Hash + Clone + Send + Sync, // Value type must implement Eq, Hash, and Clone for use in HashSet
{
    pub fn new() -> Self {
        GenericCounters {
            total_values: HashSet::new(),
            key_wise: HashMap::new(),
        }
    }

    // Insert a value for a specific key
    pub fn insert(&mut self, key: TKey, value: TValue) {
        // Insert the value into the global set of values
        self.total_values.insert(value.clone());

        // Insert the value into the key-specific set of values
        let key_entry = self.key_wise.entry(key).or_insert_with(HashSet::new);
        key_entry.insert(value);
    }

    // Remove a value from a specific key
    #[allow(unused)]
    pub fn remove(&mut self, key: TKey, value: &TValue) {
        // Remove the value from the key-specific set
        if let Some(key_entry) = self.key_wise.get_mut(&key) {
            key_entry.remove(value);

            // If the key set is empty, remove the key entry entirely
            if key_entry.is_empty() {
                self.key_wise.remove(&key);
            }
        }

        // Optionally: Remove the value from the global set only if they are no longer in any key
        if !self.key_wise.values().any(|values| values.contains(value)) {
            self.total_values.remove(value);
        }
    }

    // Get the total number of unique values across all keys
    pub fn total_count(&self) -> usize {
        self.total_values.len()
    }

    // Get the number of values for a specific key
    pub fn key_count(&self, key: &TKey) -> usize {
        self.key_wise.get(key).map_or(0, |set| set.len())
    }

    // Get the list of values associated with a specific key
    #[allow(unused)]
    pub fn get_values_by_key(&self, key: &TKey) -> Option<&HashSet<TValue>> {
        self.key_wise.get(key)
    }

    // Get all keys that a value is associated with
    #[allow(unused)]
    pub fn get_keys_by_value(&self, value: &TValue) -> Vec<TKey> {
        self.key_wise
            .iter()
            .filter_map(|(key, values)| {
                if values.contains(value) {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<TKey, TValue> Serialize for GenericCounters<TKey, TValue>
where
    TKey: Eq + Hash + Clone + Send + Sync + Serialize,
    TValue: Eq + Hash + Clone + Send + Sync + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GenericCounters", 2)?;

        // Serialize total_values as Vec<TValue>
        let total_values_vec: Vec<&TValue> = self.total_values.iter().collect();
        state.serialize_field("total_values", &total_values_vec)?;

        // Serialize key_wise as HashMap<TKey, Vec<TValue>>
        let key_wise_map: HashMap<&TKey, Vec<&TValue>> = self
            .key_wise
            .iter()
            .map(|(k, v)| (k, v.iter().collect()))
            .collect();

        state.serialize_field("key_wise", &key_wise_map)?;

        state.end()
    }
}

use serde::de::Deserializer;
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl<'de, TKey, TValue> Deserialize<'de> for GenericCounters<TKey, TValue>
where
    TKey: Eq + Hash + Clone + Send + Sync + Deserialize<'de>,
    TValue: Eq + Hash + Clone + Send + Sync + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct GenericCountersVisitor<TKey, TValue> {
            marker: std::marker::PhantomData<fn() -> GenericCounters<TKey, TValue>>,
        }

        impl<'de, TKey, TValue> Visitor<'de> for GenericCountersVisitor<TKey, TValue>
        where
            TKey: Eq + Hash + Clone + Send + Sync + Deserialize<'de>,
            TValue: Eq + Hash + Clone + Send + Sync + Deserialize<'de>,
        {
            type Value = GenericCounters<TKey, TValue>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct GenericCounters")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut total_values = None;
                let mut key_wise = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "total_values" => {
                            if total_values.is_some() {
                                return Err(de::Error::duplicate_field("total_values"));
                            }
                            let values: Vec<TValue> = map.next_value()?;
                            total_values = Some(values.into_iter().collect());
                        }
                        "key_wise" => {
                            if key_wise.is_some() {
                                return Err(de::Error::duplicate_field("key_wise"));
                            }
                            let map_data: HashMap<TKey, Vec<TValue>> = map.next_value()?;
                            let key_wise_converted = map_data
                                .into_iter()
                                .map(|(k, v)| (k, v.into_iter().collect()))
                                .collect();
                            key_wise = Some(key_wise_converted);
                        }
                        _ => {
                            let _: de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                let total_values =
                    total_values.ok_or_else(|| de::Error::missing_field("total_values"))?;
                let key_wise = key_wise.ok_or_else(|| de::Error::missing_field("key_wise"))?;
                Ok(GenericCounters {
                    total_values,
                    key_wise,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["total_values", "key_wise"];
        deserializer.deserialize_struct(
            "GenericCounters",
            FIELDS,
            GenericCountersVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}
