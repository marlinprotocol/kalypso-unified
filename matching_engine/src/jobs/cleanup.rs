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

use crate::{
    ask_lib::{ask_status::AskState, ask_store::LocalAskStore},
    market_metadata::MarketMetadataStore,
};

use super::ProofMarketplaceInstance;

pub struct CleanupTool {
    should_stop: Arc<AtomicBool>,
    market_meta_store: Arc<Mutex<MarketMetadataStore>>,
    ask_store: Arc<Mutex<LocalAskStore>>,
    #[allow(unused)]
    proof_market_place: ProofMarketplaceInstance, // will be used latter for more finely cleaning up services,
    relayer_address: Address,
    relayer_address_balance: Arc<Mutex<ethers::types::U256>>,
}

impl CleanupTool {
    pub fn new(
        should_stop: Arc<AtomicBool>,
        market_meta_store: Arc<Mutex<MarketMetadataStore>>,
        ask_store: Arc<Mutex<LocalAskStore>>,
        proof_market_place: ProofMarketplaceInstance,
        relayer_address: Address,
        relayer_address_balance: Arc<Mutex<ethers::types::U256>>,
    ) -> Self {
        CleanupTool {
            should_stop,
            market_meta_store,
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
                thread::sleep(Duration::from_secs(10));
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
                    *self.relayer_address_balance.lock().await = balance;
                }
            }

            let mut ask_store = { self.ask_store.lock().await };

            let market_meta_store = { self.market_meta_store.lock().await };
            let all_markets = market_meta_store.get_all_markets();

            for market in all_markets {
                if let Some(asks) = ask_store.get_by_market_id(&market.market_id).result() {
                    for ask in asks {
                        if let Some(state) = ask.state {
                            if state == AskState::Complete {
                                log::info!("Removed Completed ask:{}", &ask.ask_id);
                                ask_store.remove_ask_from_market_index_only(&ask.ask_id);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
