use std::future::Future;
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

    // Get cached data or recompute it if the cache is outdated
    pub async fn get_or_recompute<F, Fut>(&mut self, timeout: Duration, compute: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        // Check if the data needs to be recomputed
        if self.data.is_none() || self.last_update.elapsed() > timeout {
            // Await the asynchronous compute function
            let new_data = compute().await;
            self.data = Some(new_data.clone());
            self.last_update = Instant::now();
        }

        // Return the cached or newly computed data
        self.data.clone().unwrap()
    }
}
