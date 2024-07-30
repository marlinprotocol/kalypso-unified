pub mod cleanup;
pub mod parser;
pub mod server;

use ethers::prelude::*;
use k256::ecdsa::SigningKey;

pub type ProofMarketplaceInstance = bindings::proof_marketplace::ProofMarketplace<
    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
>;
