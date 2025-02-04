use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(bound(
    serialize = "K: Eq + std::hash::Hash + Serialize",
    deserialize = "K: Eq + std::hash::Hash + for<'a> Deserialize<'a>"
))]
pub struct EntryCounter<K> {
    entries: HashMap<K, VecDeque<SystemTime>>,
    retention: Duration,
}

impl<K> EntryCounter<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    /// Create a new EntryCounter with the specified retention period.
    pub fn new(retention: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            retention,
        }
    }

    /// Record an event for the given key at the specified timestamp.
    /// This method prunes stale events for the key before inserting.
    pub fn add(&mut self, key: K, timestamp: SystemTime) {
        self.prune_old(&key);
        self.entries
            .entry(key)
            .or_insert_with(VecDeque::new)
            .push_back(timestamp);
    }

    /// Remove events for the given key that are older than the retention period.
    fn prune_old(&mut self, key: &K) {
        if let Some(queue) = self.entries.get_mut(key) {
            let now = SystemTime::now();
            while let Some(&t) = queue.front() {
                if now.duration_since(t).unwrap_or(Duration::ZERO) > self.retention {
                    queue.pop_front();
                } else {
                    break;
                }
            }
        }
    }

    /// Returns the number of events for the given key that are still within the retention period.
    /// This method is read-only.
    pub fn count(&self, key: &K) -> usize {
        if let Some(queue) = self.entries.get(key) {
            let now = SystemTime::now();
            queue
                .iter()
                .filter(|&&t| now.duration_since(t).unwrap_or(Duration::ZERO) <= self.retention)
                .count()
        } else {
            0
        }
    }

    /// Returns the number of events for the given key that have occurred within the specified window.
    /// This method is read-only.
    pub fn count_within(&self, key: &K, window: Duration) -> usize {
        if let Some(queue) = self.entries.get(key) {
            let now = SystemTime::now();
            queue
                .iter()
                .filter(|&&t| now.duration_since(t).unwrap_or(Duration::ZERO) <= window)
                .count()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_counter_basic() {
        // Create a counter with a retention period of 2 hours.
        let retention = Duration::from_secs(2 * 3600);
        let mut counter = EntryCounter::new(retention);
        let key = "test_key".to_string();

        let now = SystemTime::now();
        counter.add(key.clone(), now);

        // The count should be 1.
        assert_eq!(counter.count(&key), 1);
        assert_eq!(counter.count_within(&key, Duration::from_secs(3600)), 1);

        // Add a backdated event from 30 minutes ago.
        let past = now - Duration::from_secs(1800);
        counter.add(key.clone(), past);

        // Both events are within the retention period.
        assert_eq!(counter.count(&key), 2);

        // Count events within a 20‑minute window: only the event at "now" qualifies.
        assert_eq!(counter.count_within(&key, Duration::from_secs(20 * 60)), 1);
    }

    #[test]
    fn test_pruning_of_old_events() {
        // Set a short retention period (10 seconds) for testing.
        let retention = Duration::from_secs(10);
        let mut counter = EntryCounter::new(retention);
        let key = "prune_key".to_string();
        let now = SystemTime::now();

        // Insert an event that occurred 11 seconds ago.
        // It should be pruned immediately when adding a new event.
        let old_event = now - Duration::from_secs(11);
        counter.add(key.clone(), old_event);

        // Insert a current event.
        counter.add(key.clone(), now);

        // Only the current event should count.
        assert_eq!(counter.count(&key), 1);
    }
}
