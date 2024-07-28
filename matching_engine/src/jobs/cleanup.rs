use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::Wallet,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::ask::{self, LocalAskStore};

#[allow(unused)]
pub async fn ask_store_cleanup(
    ask_store: &Arc<Mutex<LocalAskStore>>,
    proof_market_place: bindings::proof_marketplace::ProofMarketplace<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    >,
) {
    let mut ask_store = ask_store.lock().await;

    // Removing the completed asks
    let completed_asks = ask_store.get_by_state(ask::AskState::Complete).result();

    if completed_asks.is_some() {
        for elem in completed_asks.unwrap() {
            ask_store.remove_by_ask_id(&elem.ask_id);
        }
    }

    // Checking and removing the assigned asks
    let assigned_asks = ask_store.get_by_state(ask::AskState::Assigned).result();

    if assigned_asks.is_some() {
        for elem in assigned_asks.unwrap() {
            let ask_state = proof_market_place.get_ask_state(elem.ask_id).await.unwrap();
            let ask_state = ask::get_ask_state(ask_state);
            if ask_state != ask::AskState::Assigned || ask_state != ask::AskState::Create {
                ask_store.remove_by_ask_id(&elem.ask_id);
            }
        }
    }

    // Checking and removing the idle asks
    let idle_asks = ask_store.get_by_state(ask::AskState::Create).result();

    if idle_asks.is_some() {
        for elem in idle_asks.unwrap() {
            let ask_state = proof_market_place.get_ask_state(elem.ask_id).await.unwrap();
            let ask_state = ask::get_ask_state(ask_state);
            if ask_state != ask::AskState::Assigned || ask_state != ask::AskState::Create {
                ask_store.remove_by_ask_id(&elem.ask_id);
            }
        }
    }
}
