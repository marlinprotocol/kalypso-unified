// tests/test_helpers.rs

use ethers::core::types::{U256, Address, Bytes, H256};
use crate::LocalAsk;
use crate::tests::AskState; // Assuming you define AskState in your tests module

pub fn sample_local_ask() -> LocalAsk {
    LocalAsk {
        ask_id: U256::from(1),
        market_id: U256::from(100),
        reward: U256::from(500),
        expiry: U256::from(1000),
        deadline: U256::from(2000),
        time_requested_for_proof_generation: U256::from(3000),
        prover_refund_address: "0x0000000000000000000000000000000000000001".parse().unwrap(),
        prover_data: Bytes::from("data"),
        has_private_inputs: false,
        secret_data: None,
        secret_acl: None,
        state: Some(AskState::Pending),
        generator: None,
        invalid_secret_flag: false,
        created_on: U256::from(4000),
        created_on_l1: U256::from(5000),
        create_transaction: "0x0000000000000000000000000000000000000000000000000000000000000001".parse().unwrap(),
    }
}

// You can add more helper functions for other common values as needed.
