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
    gas_key_check_iter: usize,
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
            gas_key_check_iter: 500,
        }
    }

    pub async fn start_cleanup(
        self,
        check_gas_key_balance: bool,
        fast_clean: bool,
    ) -> anyhow::Result<()> {
        let mut iter = 0 as usize;
        loop {
            if fast_clean {
                log::debug!("fast cleanup initiated");
                thread::sleep(Duration::from_secs(4));
            } else {
                thread::sleep(Duration::from_secs(60));
            }

            if self.should_stop.load(Ordering::Acquire) {
                log::info!("Gracefully shutting down...");
                break;
            }

            iter += 1;
            if check_gas_key_balance && iter >= self.gas_key_check_iter {
                log::debug!("Checking gas key balance");
                let balance = match self
                    .proof_market_place
                    .client()
                    .get_balance(self.relayer_address, None)
                    .await
                {
                    Ok(data) => {
                        log::debug!("gas key balance: {}", data);
                        data
                    }
                    Err(_) => {
                        log::error!("Failed fetching gas key balance");
                        ethers::types::U256::zero()
                    }
                };
                {
                    *self.relayer_address_balance.write().await = balance;
                }
                iter = 0;
            }

            match self.ask_store.try_read() {
                Ok(store) => {
                    let completed_asks = store.get_cleanup_asks().result();
                    if completed_asks.is_none() {
                        continue;
                    }
                    let completed_asks = completed_asks.unwrap();
                    if !completed_asks.is_empty() {
                        match self.ask_store.try_write() {
                            Ok(mut ask_store) => {
                                for elem in completed_asks {
                                    log::info!("Removed Completed ask: {}", &elem.ask_id);
                                    ask_store.remove_ask_only_if_completed(&elem.ask_id);
                                }
                            }
                            Err(_) => {
                                log::error!("Failed to acquire write lock on ask_store, skipping this iteration");
                                continue;
                            }
                        }
                    }
                }
                Err(_) => {
                    log::debug!(
                        "Failed to acquire read lock on ask_store, skipping this iteration"
                    );
                    continue;
                }
            }
        }

        Ok(())
    }
}
