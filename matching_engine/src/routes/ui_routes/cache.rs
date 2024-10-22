use tokio::time::{Duration, Instant};

#[derive(Clone)]
pub struct CachedResponse<T> {
    data: Option<T>,
    last_update: Instant,
}

impl<T: Clone> CachedResponse<T> {
    // Create a new CachedResponse
    pub fn new() -> Self {
        CachedResponse {
            data: None,
            last_update: Instant::now(),
        }
    }

    // Check if the cached response is still valid
    pub fn get_if_valid(&self, timeout: Duration) -> Option<T> {
        if self.last_update.elapsed() <= timeout {
            self.data.clone() // Return the cached data if valid
        } else {
            None // Cache is outdated
        }
    }

    // Store a newly computed value in the cache
    pub fn store(&mut self, new_data: T) {
        self.data = Some(new_data);
        self.last_update = Instant::now();
    }

    // Remove cached data
    #[allow(unused)]
    pub fn clear(&mut self) {
        self.data = None;
    }
}
