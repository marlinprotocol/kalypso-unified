use im::HashSet;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

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
            .par_iter()
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
