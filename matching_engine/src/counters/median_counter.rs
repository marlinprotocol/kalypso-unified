use std::cmp::Reverse;
use std::collections::BinaryHeap;

use im::HashMap;

use ethers::types::U256;
use serde::{Deserialize, Serialize}; // To use for the min-heap

// Struct to store two heaps for median calculation
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct MedianTracker<TValue>
where
    TValue: std::cmp::Ord,
{
    min_heap: BinaryHeap<Reverse<TValue>>, // Min-heap to store larger half (smallest on top)
    max_heap: BinaryHeap<TValue>,          // Max-heap to store smaller half (largest on top)
}

impl<TValue> MedianTracker<TValue>
where
    TValue: Ord + Clone, // Require Ord to maintain heap order and Clone for value extraction
{
    pub fn new() -> Self {
        MedianTracker {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    // Insert a value and maintain the balance between the heaps
    pub fn insert(&mut self, value: TValue) {
        if self.max_heap.is_empty() || value <= *self.max_heap.peek().unwrap() {
            self.max_heap.push(value);
        } else {
            self.min_heap.push(Reverse(value));
        }

        // Balance the heaps (make sure max_heap has equal or one more element than min_heap)
        if self.max_heap.len() > self.min_heap.len() + 1 {
            let moved_value = self.max_heap.pop().unwrap();
            self.min_heap.push(Reverse(moved_value));
        } else if self.min_heap.len() > self.max_heap.len() {
            let moved_value = self.min_heap.pop().unwrap().0;
            self.max_heap.push(moved_value);
        }
    }

    // Get the median
    pub fn median(&self) -> Option<TValue> {
        if self.max_heap.is_empty() {
            return None;
        }

        if self.max_heap.len() > self.min_heap.len() {
            // Odd number of elements, median is the top of the max-heap
            Some(self.max_heap.peek().unwrap().clone())
        } else {
            // Even number of elements, median is the average of the two tops
            // Here we return the top of the max-heap, you can modify it to suit your case
            Some(self.max_heap.peek().unwrap().clone())
        }
    }
}

use crate::utility::deserialize_u256_map;
use crate::utility::serialize_u256_map;

// Main struct to handle per-key and global median tracking
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct MedianCounter<TValue>
where
    TValue: std::cmp::Ord + std::clone::Clone,
{
    #[serde(
        serialize_with = "serialize_u256_map",
        deserialize_with = "deserialize_u256_map"
    )]
    key_wise: HashMap<U256, MedianTracker<TValue>>, // Per-key median trackers
    global_tracker: MedianTracker<TValue>, // Global median tracker
}

impl<TValue> MedianCounter<TValue>
where
    TValue: Ord + Clone, // Value must be sortable and clonable
{
    pub fn new() -> Self {
        MedianCounter {
            key_wise: HashMap::new(),
            global_tracker: MedianTracker::new(),
        }
    }

    // Insert a value for a specific key and update the global tracker
    pub fn insert(&mut self, key: U256, value: TValue) {
        // Insert into the global tracker
        self.global_tracker.insert(value.clone());

        // Insert into the key-specific tracker
        let key_entry = self.key_wise.entry(key).or_insert_with(MedianTracker::new);
        key_entry.insert(value);
    }

    // Get the global median
    pub fn median_all(&self) -> Option<TValue> {
        self.global_tracker.median()
    }

    // Get the median for a specific key
    pub fn median_by_key(&self, key: &U256) -> Option<TValue> {
        self.key_wise.get(key).and_then(|tracker| tracker.median())
    }
}
