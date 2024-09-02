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

use crate::ask::{self, LocalAskStore};

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

    pub async fn ask_store_cleanup(self, skip_relayer_balance_check: bool) -> anyhow::Result<()> {
        loop {
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
                    *self.relayer_address_balance.lock().await = balance;
                }
            }

            let mut ask_store = { self.ask_store.lock().await };

            // Removing the completed asks
            let completed_asks = ask_store.get_by_state(ask::AskState::Complete).result();

            if completed_asks.is_some() {
                for elem in completed_asks.unwrap() {
                    log::info!("Removed Completed ask:{}", &elem.ask_id);
                    ask_store.remove_by_ask_id(&elem.ask_id);
                }
            }
        }

        Ok(())
    }
}
