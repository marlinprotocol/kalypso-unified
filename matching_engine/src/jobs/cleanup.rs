use ethers::{providers::Middleware, types::Address};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::Mutex;
use tokio::time::interval;

use crate::ask_lib::ask_store::LocalAskStore;

use super::ProofMarketplaceInstance;

pub struct CleanupTool {
    should_stop: Arc<AtomicBool>,
    ask_store: Arc<Mutex<LocalAskStore>>,
    #[allow(unused)]
    proof_market_place: ProofMarketplaceInstance, // will be used later for more finely cleaning up services,
    relayer_address: Address,
    relayer_address_balance: Arc<Mutex<ethers::types::U256>>,
}

impl CleanupTool {
    pub fn new(
        should_stop: Arc<AtomicBool>,
        ask_store: Arc<Mutex<LocalAskStore>>,
        proof_market_place: ProofMarketplaceInstance,
        relayer_address: Address,
        relayer_address_balance: Arc<Mutex<ethers::types::U256>>,
    ) -> Self {
        CleanupTool {
            should_stop,
            ask_store,
            proof_market_place,
            relayer_address,
            relayer_address_balance,
        }
    }

    /// Starts the cleanup task. This function spawns a background task that periodically
    /// cleans up the ask store and updates the relayer balance.
    pub fn start_cleanup(
        self,
        skip_relayer_balance_check: bool,
        slow_cleanup: bool,
    ) -> tokio::task::JoinHandle<anyhow::Result<()>> {
        tokio::spawn(async move {
            // Determine the interval duration based on the `slow_cleanup` flag
            let cleanup_interval = if slow_cleanup {
                Duration::from_secs(30)
            } else {
                Duration::from_secs(2)
            };
            let mut interval = interval(cleanup_interval);

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Perform the cleanup operations

                        if skip_relayer_balance_check {
                            // Update the relayer's balance
                            let balance = match self
                                .proof_market_place
                                .client()
                                .get_balance(self.relayer_address, None)
                                .await
                            {
                                Ok(data) => data,
                                Err(err) => {
                                    log::error!("Failed to get balance: {:?}", err);
                                    ethers::types::U256::zero()
                                },
                            };
                            {
                                let mut relayer_balance_lock = self.relayer_address_balance.lock().await;
                                *relayer_balance_lock = balance;
                                // Lock is automatically dropped here
                            }
                        }

                        // Clean up completed asks
                        {
                            let mut ask_store = self.ask_store.lock().await;
                            if let Some(completed_asks) = ask_store.get_cleanup_asks().result() {
                                for elem in completed_asks {
                                    log::info!("Removed Completed ask: {}", &elem.ask_id);
                                    ask_store.remove_ask_only_if_completed(&elem.ask_id);
                                }
                            }
                            // Lock is automatically dropped here
                        }
                    }

                    // Check if a shutdown has been requested
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        if self.should_stop.load(Ordering::Acquire) {
                            log::info!("Gracefully shutting down cleanup task...");
                            break;
                        }
                    }
                }

                // Optional: Yield to allow other tasks to run
                tokio::task::yield_now().await;
            }

            Ok(())
        })
    }
}
