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
    proof_market_place: ProofMarketplaceInstance,
}

impl CleanupTool {
    pub fn new(
        should_stop: Arc<AtomicBool>,
        ask_store: Arc<Mutex<LocalAskStore>>,
        proof_market_place: ProofMarketplaceInstance,
    ) -> Self {
        CleanupTool {
            should_stop,
            ask_store,
            proof_market_place,
        }
    }

    pub async fn ask_store_cleanup(self) -> anyhow::Result<()> {
        loop {
            thread::sleep(Duration::from_secs(20));
            if self.should_stop.load(Ordering::Acquire) {
                log::info!("Gracefully shutting down...");
                break;
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
