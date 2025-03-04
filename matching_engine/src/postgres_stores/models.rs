use crate::ask_lib::ask_status::AskState;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::{prelude::*, r2d2::Pool};
use ethers::core::types::{Address, Bytes, H256, U256};
use std::{collections::HashMap, vec::Vec};

use crate::ask_lib::ask::LocalAsk;
use crate::schema::ask_records;

#[derive(Debug, Queryable, Insertable, QueryableByName, Selectable)]
#[table_name = "ask_records"]
pub struct AskRecord {
    pub ask_id: Vec<u8>,
    pub market_id: Vec<u8>,
    pub reward: Vec<u8>,
    pub expiry: Vec<u8>,
    pub deadline: Vec<u8>,
    pub time_requested_for_proof_generation: Vec<u8>,
    pub prover_refund_address: Vec<u8>,
    pub prover_data: Vec<u8>,
    pub has_private_inputs: bool,
    // Omit secret_data and secret_acl from the DB.
    pub state: Option<Vec<u8>>,
    pub generator: Option<Vec<u8>>,
    pub invalid_secret_flag: bool,
    pub created_on: Vec<u8>,
    pub created_on_l1: Vec<u8>,
    pub create_transaction: Vec<u8>,
    // Other fields like proofs, timestamps are also stored here.
    pub proof: Option<Vec<u8>>,
    pub proof_type: Option<String>,
    pub proving_time_taken: Option<Vec<u8>>,
    pub proving_cost_taken: Option<Vec<u8>>,
    pub proof_transaction: Option<String>,
    pub proof_cycle_completed_on: Option<Vec<u8>>,
    pub job_created_on_timestamp: Option<Vec<u8>>,
    pub job_matched_on_timestamp: Option<Vec<u8>>,
    pub job_completed_on_timestamp: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct AskPrivateInputs {
    pub secret_data: Option<Vec<u8>>,
    pub secret_acl: Option<Vec<u8>>,
}

pub struct PrivateInputStore {
    pub store: HashMap<U256, AskPrivateInputs>,
}

impl PrivateInputStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn insert(&mut self, ask_id_val: U256, inputs: AskPrivateInputs) {
        self.store.insert(ask_id_val, inputs);
    }

    pub fn get(&self, ask_id_val: &U256) -> Option<AskPrivateInputs> {
        self.store.get(ask_id_val).cloned()
    }
}

// Helper conversion functions.
pub fn u256_to_bytes(val: U256) -> Vec<u8> {
    let mut bytes = [0u8; 32];
    val.to_big_endian(&mut bytes);
    bytes.to_vec()
}

pub fn bytes_to_u256(b: &[u8]) -> U256 {
    U256::from_big_endian(b)
}

/// Database-backed implementation.
pub struct AskDatabase {
    // conn: PgConnection,
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub private_store: PrivateInputStore,
}

impl From<LocalAsk> for AskRecord {
    fn from(ask: LocalAsk) -> Self {
        AskRecord {
            ask_id: u256_to_bytes(ask.ask_id),
            market_id: u256_to_bytes(ask.market_id),
            reward: u256_to_bytes(ask.reward),
            expiry: u256_to_bytes(ask.expiry),
            deadline: u256_to_bytes(ask.deadline),
            time_requested_for_proof_generation: u256_to_bytes(
                ask.time_requested_for_proof_generation,
            ),
            prover_refund_address: ask.prover_refund_address.as_bytes().to_vec(),
            prover_data: ask.prover_data.to_vec(),
            has_private_inputs: ask.has_private_inputs,
            state: ask.state.as_ref().map(|s| vec![*s as u8]),
            generator: ask.generator.map(|g| g.as_bytes().to_vec()),
            invalid_secret_flag: ask.invalid_secret_flag,
            created_on: u256_to_bytes(ask.created_on),
            created_on_l1: u256_to_bytes(ask.created_on_l1),
            create_transaction: ask.create_transaction.as_bytes().to_vec(),

            proof: None,
            proof_type: None,
            proving_time_taken: None,
            proving_cost_taken: None,
            proof_transaction: None,
            proof_cycle_completed_on: None,
            job_created_on_timestamp: None,
            job_matched_on_timestamp: None,
            job_completed_on_timestamp: None,
        }
    }
}

impl AskDatabase {
    pub fn try_from_ask_record(&self, rec: AskRecord) -> Result<LocalAsk, String> {
        // let private_inputs = self.private_store.get(&bytes_to_u256(&rec.ask_id)).unwrap_or(None);
        let private_inputs = self
            .private_store
            .get(&bytes_to_u256(&rec.ask_id))
            .unwrap_or(AskPrivateInputs {
                secret_data: None,
                secret_acl: None,
            });
        Ok(LocalAsk {
            secret_acl: private_inputs.secret_acl.map(Bytes::from),
            secret_data: private_inputs.secret_data.map(Bytes::from),
            ask_id: bytes_to_u256(&rec.ask_id),
            market_id: bytes_to_u256(&rec.market_id),
            reward: bytes_to_u256(&rec.reward),
            expiry: bytes_to_u256(&rec.expiry),
            deadline: bytes_to_u256(&rec.deadline),
            time_requested_for_proof_generation: bytes_to_u256(
                &rec.time_requested_for_proof_generation,
            ),
            prover_refund_address: Address::from_slice(&rec.prover_refund_address),
            prover_data: Bytes::from(rec.prover_data),
            has_private_inputs: rec.has_private_inputs,
            state: rec.state.map(|v| match v.get(0) {
                Some(&0) => AskState::Null,
                Some(&1) => AskState::Create,
                Some(&2) => AskState::UnAssigned,
                Some(&3) => AskState::Assigned,
                Some(&4) => AskState::Complete,
                Some(&5) => AskState::DeadlineCrossed,
                Some(&6) => AskState::InvalidSecret,
                _ => AskState::Create,
            }),
            generator: rec.generator.map(|b| Address::from_slice(&b)),
            invalid_secret_flag: rec.invalid_secret_flag,
            created_on: bytes_to_u256(&rec.created_on),
            created_on_l1: bytes_to_u256(&rec.created_on_l1),
            create_transaction: H256::from_slice(&rec.create_transaction),
            // add proof here (not needed yet)
        })
    }
}
