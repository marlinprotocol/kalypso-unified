use actix_web::web::Data;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod ask_status;
pub mod chain_status;
pub mod decrypt_request;
pub mod get_priv_inputs;
pub mod market_info;

type EntityRegistryInstance = Data<
    Arc<
        Mutex<
            bindings::entity_key_registry::EntityKeyRegistry<
                SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
            >,
        >,
    >,
>;
