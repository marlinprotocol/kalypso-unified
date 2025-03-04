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

#[cfg(test)]
mod tests {
    use crate::counters::median_counter::MedianCounter;
    use ethers::types::U256;

    #[test]
    fn test_empty_median() {
        let counter = MedianCounter::<U256>::new();
        assert!(counter.median_all().is_none());
        let key = U256::from(123);
        assert!(counter.median_by_key(&key).is_none());
    }

    #[test]
    fn test_single_element() {
        let mut counter = MedianCounter::<U256>::new();
        let key = U256::from(1);
        let value = U256::from(10);
        counter.insert(key, value);
        assert_eq!(counter.median_all(), Some(value));
        assert_eq!(counter.median_by_key(&key), Some(value));
    }

    #[test]
    fn test_global_median_odd() {
        let mut counter = MedianCounter::<U256>::new();
        let key = U256::from(1);
        // Insert an odd number of elements: [3, 1, 5] => sorted [1, 3, 5]
        let values = vec![U256::from(3), U256::from(1), U256::from(5)];
        for v in &values {
            counter.insert(key, *v);
        }
        // For odd count, median is the middle element (3)
        assert_eq!(counter.median_all(), Some(U256::from(3)));
    }

    #[test]
    fn test_global_median_even() {
        let mut counter = MedianCounter::<U256>::new();
        let key = U256::from(1);
        // Insert even number of elements: [3, 1, 5, 7] => sorted [1, 3, 5, 7]
        let values = vec![U256::from(3), U256::from(1), U256::from(5), U256::from(7)];
        for v in &values {
            counter.insert(key, *v);
        }
        // With even count, the implementation returns the top of the max_heap,
        // which represents the lower median (3)
        assert_eq!(counter.median_all(), Some(U256::from(3)));
    }

    #[test]
    fn test_median_by_key_multiple_keys() {
        let mut counter = MedianCounter::<U256>::new();
        // Use two separate keys
        let key1 = U256::from(1);
        let key2 = U256::from(2);

        // For key1, insert three elements: [4, 2, 6] => sorted [2, 4, 6]
        let values1 = vec![U256::from(4), U256::from(2), U256::from(6)];
        for v in &values1 {
            counter.insert(key1, *v);
        }
        // For key2, insert two elements: [3, 5] => sorted [3, 5], lower median = 3
        let values2 = vec![U256::from(3), U256::from(5)];
        for v in &values2 {
            counter.insert(key2, *v);
        }

        assert_eq!(counter.median_by_key(&key1), Some(U256::from(4)));
        assert_eq!(counter.median_by_key(&key2), Some(U256::from(3)));
    }

    #[test]
    fn test_duplicate_values() {
        let mut counter = MedianCounter::<U256>::new();
        let key = U256::from(10);
        // Insert duplicate values multiple times
        for _ in 0..5 {
            counter.insert(key, U256::from(42));
        }
        // The median should be 42 for both global and key-specific queries
        assert_eq!(counter.median_all(), Some(U256::from(42)));
        assert_eq!(counter.median_by_key(&key), Some(U256::from(42)));
    }

    #[test]
    fn test_nonexistent_key() {
        let mut counter = MedianCounter::<U256>::new();
        counter.insert(U256::from(1), U256::from(10));
        let nonexistent_key = U256::from(999);
        // Querying a key that hasn't been inserted should return None
        assert!(counter.median_by_key(&nonexistent_key).is_none());
    }

    #[test]
    fn test_median_mixed_insertion() {
        let mut counter = MedianCounter::<U256>::new();
        // Insert values for multiple keys in mixed order
        let key1 = U256::from(1);
        let key2 = U256::from(2);
        // For key1, values: [20, 5, 15] => sorted: [5, 15, 20], median is 15
        let values1 = vec![U256::from(20), U256::from(5), U256::from(15)];
        // For key2, values: [30, 25] => sorted: [25, 30], lower median is 25
        let values2 = vec![U256::from(30), U256::from(25)];
        for v in &values1 {
            counter.insert(key1, *v);
        }
        for v in &values2 {
            counter.insert(key2, *v);
        }
        // Global insertion gives [20, 5, 15, 30, 25] => sorted: [5, 15, 20, 25, 30], median is 20
        assert_eq!(counter.median_all(), Some(U256::from(20)));
        assert_eq!(counter.median_by_key(&key1), Some(U256::from(15)));
        assert_eq!(counter.median_by_key(&key2), Some(U256::from(25)));
    }

    #[test]
    fn test_median_mixed_insertion_2() {
        let mut counter = MedianCounter::<U256>::new();
        // Insert values for multiple keys in mixed order
        let key1 = U256::from(1);
        let values1 = vec![U256::from(1000000); 500];
        for v in &values1 {
            counter.insert(key1, *v);
        }

        let values2 = vec![U256::from(0); 200];
        for v in &values2 {
            counter.insert(key1, *v);
        }
        assert_eq!(counter.median_by_key(&key1), Some(U256::from(1000000)));

        let values3 = vec![U256::from(0); 2000];
        for v in &values3 {
            counter.insert(key1, *v);
        }
        assert_eq!(counter.median_by_key(&key1), Some(U256::from(0)));
    }
}
