use ethers::{providers::Middleware, types::Address};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use tokio::sync::Mutex;

use crate::ask_lib::ask_store::LocalAskStore;

use super::ProofMarketplaceInstance;

pub struct CleanupTool {
    should_stop: Arc<AtomicBool>,
    ask_store: Arc<Mutex<LocalAskStore>>,
    #[allow(unused)]
    proof_market_place: ProofMarketplaceInstance, // will be used latter for more finely cleaning up services,
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

    pub async fn ask_store_cleanup(
        self,
        skip_relayer_balance_check: bool,
        slow_cleanup: bool,
    ) -> anyhow::Result<()> {
        loop {
            if slow_cleanup {
                thread::sleep(Duration::from_secs(30));
            } else {
                thread::sleep(Duration::from_secs(2));
            }

            if self.should_stop.load(Ordering::Acquire) {
                log::info!("Gracefully shutting down...");
                break;
            }

            if skip_relayer_balance_check {
                let balance = match self
                    .proof_market_place
                    .client()
                    .get_balance(self.relayer_address, None)
                    .await
                {
                    Ok(data) => data,
                    Err(_) => ethers::types::U256::zero(),
                };
                {
                    let mut relayer_balance_lock = self.relayer_address_balance.lock().await;
                    *relayer_balance_lock = balance;

                    // Explicitly drop the lock after updating the balance
                    drop(relayer_balance_lock);
                }
            }

            {
                let mut ask_store = { self.ask_store.lock().await };

                // Removing the completed asks
                let completed_asks = ask_store.get_cleanup_asks().result();

                if completed_asks.is_some() {
                    for elem in completed_asks.unwrap() {
                        log::info!("Removed Completed ask:{}", &elem.ask_id);
                        ask_store.remove_ask_only_if_completed(&elem.ask_id);
                    }
                }

                drop(ask_store);
            }
        }

        Ok(())
    }
}
