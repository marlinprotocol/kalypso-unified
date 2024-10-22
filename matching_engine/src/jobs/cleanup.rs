use ethers::{providers::Middleware, types::Address};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use tokio::sync::RwLock;

use crate::ask_lib::ask_store::LocalAskStore;

use super::ProofMarketplaceInstance;

pub struct CleanupTool {
    should_stop: Arc<AtomicBool>,
    ask_store: Arc<RwLock<LocalAskStore>>,
    #[allow(unused)]
    proof_market_place: ProofMarketplaceInstance, // will be used latter for more finely cleaning up services,
    relayer_address: Address,
    relayer_address_balance: Arc<RwLock<ethers::types::U256>>,
}

impl CleanupTool {
    pub fn new(
        should_stop: Arc<AtomicBool>,
        ask_store: Arc<RwLock<LocalAskStore>>,
        proof_market_place: ProofMarketplaceInstance,
        relayer_address: Address,
        relayer_address_balance: Arc<RwLock<ethers::types::U256>>,
    ) -> Self {
        CleanupTool {
            should_stop,
            ask_store,
            proof_market_place,
            relayer_address,
            relayer_address_balance,
        }
    }

    pub async fn start_cleanup(
        self,
        skip_relayer_balance_check: bool,
        slow_cleanup: bool,
    ) -> anyhow::Result<()> {
        loop {
            if slow_cleanup {
                thread::sleep(Duration::from_secs(30));
            } else {
                thread::sleep(Duration::from_secs(1));
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
                    *self.relayer_address_balance.write().await = balance;
                }
            }

            if let Some(completed_asks) = self.ask_store.read().await.get_cleanup_asks().result() {
                if completed_asks.len() > 0 {
                    let mut ask_store = self.ask_store.write().await;
                    for elem in completed_asks {
                        log::info!("Removed Completed ask: {}", &elem.ask_id);
                        ask_store.remove_ask_only_if_completed(&elem.ask_id);
                    }
                }
            }
        }

        Ok(())
    }
}
