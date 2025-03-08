use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::{prelude::*, r2d2::Pool};
use ethers::core::types::{Address, Bytes, H256, U256};
use std::cell::RefCell;
use std::{collections::HashMap, vec::Vec};

use crate::ask_lib::ask::LocalAsk;
use crate::ask_lib::ask_status::AskState;

use crate::generator_lib::{
    generator_store::{Generator, GeneratorInfoPerMarket},
    withdrawal_request::WithdrawlRequest,
};
use crate::schema::{
    ask_records, delegations, generator_markets, generators, slashing_records, token_trackers,
    withdrawal_requests,
};

/// Database-backed implementation for AskRecord.
pub struct AskDatabase {
    // conn: PgConnection,
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub private_store: PrivateInputStore,
}

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
    pub associated_stake_locks: Option<String>,
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
            associated_stake_locks: None,
        }
    }
}

impl AskDatabase {
    pub fn try_from_ask_record(&self, rec: AskRecord) -> Result<LocalAsk, String> {
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

/// Our Diesel-based store which uses a connection pool.
pub struct GeneratorDatabase {
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub cache: RefCell<HashMap<String, Vec<GeneratorInfoPerMarket>>>,
}

/// New record for the `generators` table.
#[derive(Debug, Queryable, Insertable, QueryableByName, Selectable)]
#[table_name = "generators"]
pub struct GeneratorRecord {
    pub address: String,                    // Primary key; must be present.
    pub reward_address: String,             // Required.
    pub total_native_stake: String,         // Serialized U256/TokenTracker as JSON or string.
    pub total_symbiotic_stake: String,      // Serialized U256/TokenTracker as JSON or string.
    pub sum_of_compute_allocations: String, // U256 as string.
    pub compute_consumed: String,           // U256 as string.
    pub native_stake_locked: String,        // Serialized TokenTracker.
    pub symbiotic_stake_locked: String,     // Serialized TokenTracker.
    pub active_market_places: String,       // U256 as string.
    pub declared_compute: String,           // U256 as string. (May be updated later.)
    pub intended_stake_util: String,        // U256 as string.
    pub intended_compute_util: String,      // U256 as string.
    pub generator_data: Vec<u8>,            // Bytea for binary data.
    pub active: bool,                       // Required.

    // These fields are **not present** in Generator
    pub earnings: String,         // U256 as string.
    pub kalypso_points: String,   // U256 as string.
    pub jobs_missed_counter: i32, // i32 counter.
}

/// New record for the `generator_markets` table.
#[derive(Debug, Queryable, Insertable, QueryableByName, Selectable, Clone)]
#[table_name = "generator_markets"]
pub struct GeneratorMarketRecord {
    pub generator_address: String, // e.g., format!("{:?}", generator_info.address)
    pub market_id: String,         // U256 as string
    pub compute_required_per_request: String, // U256 as string
    pub proof_generation_cost: String, // U256 as string
    pub proposed_time: String,     // U256 as string
    pub active_requests: String,   // U256 as string
    pub proofs_submitted: String,  // U256 as string
    pub proofs_slashed: String,    // U256 as string
    pub state: Option<String>,     // Optional state as a string
    pub earnings: String,          // U256 as string (per market earnings)
    pub kalypso_points: String,    // U256 as string (per market points)
}

/// New record for the `token_trackers` table.
/// Here, if `market_id` is None, the record applies to generator-wide data.
#[derive(Debug, Queryable, Insertable, QueryableByName, Selectable)]
#[table_name = "token_trackers"]
pub struct TokenTrackerRecord {
    pub generator_address: String,
    pub market_id: String,
    pub token_tracker: String,
}

/// New record for the `slashing_records` table.
#[derive(Debug, Queryable, Insertable, QueryableByName, Selectable)]
#[table_name = "slashing_records"]
pub struct DBSlashingRecord {
    pub generator_address: String,
    pub ask_id: String,
    pub slashing_block_number: String,
    pub market_id: String,
    pub slashing_tx: String,
    pub price_offered: String,
    pub expected_time: String,
    pub slashing_penalty: String, // We'll store a JSON representation of AddressTokenPair
    pub slashing_timestamp: String,
    pub source: String, // "Native" or "Symbiotic"
}

/// New record for the `delegations` table.
#[derive(Debug, Queryable, Insertable, QueryableByName, Selectable)]
#[table_name = "delegations"]
pub struct DelegationRecord {
    pub generator_address: String, // Generator address as string
    pub delegated_address: String, // Delegated address (H160 as string)
    pub delegated_amount: String,  // U256 as string
    pub source: String,            // Source (as string or serialized enum)
    pub operation: String,         // Operation (as string or serialized enum)
    pub block_number: String,      // U64 as string
    pub transaction_index: String, // U64 as string
    pub log_index: String,
    pub tx: String,
}

/// New record for the `withdrawal_requests` table.
#[derive(Debug, Queryable, Insertable, QueryableByName, Selectable)]
#[table_name = "withdrawal_requests"]
pub struct WithdrawalRequestRecord {
    pub generator_address: String, // Generator address as string
    pub account: String,           // Account (H160 as string)
    pub token: String,             // Token (H160 as string)
    pub amount: String,            // U256 as string
    pub request_index: String,     // U256 as string
    pub timestamp: String,         // U256 as string (or a proper timestamp type)
}

// Conversion from Generator (in-memory) to GeneratorRecord (db).
impl From<Generator> for GeneratorRecord {
    fn from(gen: Generator) -> Self {
        GeneratorRecord {
            address: gen.address.to_string(),
            reward_address: gen.reward_address.to_string(),
            total_native_stake: serde_json::to_string(&gen.total_native_stake)
                .expect("Serialization failed for total_native_stake"),
            total_symbiotic_stake: serde_json::to_string(&gen.total_symbiotic_stake)
                .expect("Serialization failed for total_symbiotic_stake"),
            sum_of_compute_allocations: gen.sum_of_compute_allocations.to_string(),
            compute_consumed: gen.compute_consumed.to_string(),
            native_stake_locked: serde_json::to_string(&gen.native_stake_locked)
                .expect("Serialization failed for native_stake_locked"),
            symbiotic_stake_locked: serde_json::to_string(&gen.symbiotic_stake_locked)
                .expect("Serialization failed for symbiotic_stake_locked"),
            active_market_places: gen.active_market_places.to_string(),
            declared_compute: gen.declared_compute.to_string(),
            intended_stake_util: gen.intended_stake_util.to_string(),
            intended_compute_util: gen.intended_compute_util.to_string(),
            generator_data: gen.generator_data.to_vec(), // Convert Bytes to Vec<u8>
            active: gen.active,
            // Set default values for DB-only fields.
            earnings: "0".to_string(),
            kalypso_points: "0".to_string(),
            jobs_missed_counter: 0,
        }
    }
}

// Conversion from GeneratorRecord (db) to Generator (in-memory).
impl From<GeneratorRecord> for Generator {
    fn from(rec: GeneratorRecord) -> Self {
        Generator {
            address: rec.address.parse().expect("Invalid address"),
            reward_address: rec.reward_address.parse().expect("Invalid reward address"),
            total_native_stake: serde_json::from_str(&rec.total_native_stake)
                .expect("Deserialization failed for total_native_stake"),
            total_symbiotic_stake: serde_json::from_str(&rec.total_symbiotic_stake)
                .expect("Deserialization failed for total_symbiotic_stake"),
            sum_of_compute_allocations: rec
                .sum_of_compute_allocations
                .parse()
                .expect("Invalid U256 for sum_of_compute_allocations"),
            compute_consumed: rec
                .compute_consumed
                .parse()
                .expect("Invalid U256 for compute_consumed"),
            native_stake_locked: serde_json::from_str(&rec.native_stake_locked)
                .expect("Deserialization failed for native_stake_locked"),
            symbiotic_stake_locked: serde_json::from_str(&rec.symbiotic_stake_locked)
                .expect("Deserialization failed for symbiotic_stake_locked"),
            active_market_places: rec
                .active_market_places
                .parse()
                .expect("Invalid U256 for active_market_places"),
            declared_compute: rec
                .declared_compute
                .parse()
                .expect("Invalid U256 for declared_compute"),
            intended_stake_util: rec
                .intended_stake_util
                .parse()
                .expect("Invalid U256 for intended_stake_util"),
            intended_compute_util: rec
                .intended_compute_util
                .parse()
                .expect("Invalid U256 for intended_compute_util"),
            generator_data: Bytes::from(rec.generator_data),
            active: rec.active,
        }
    }
}

// ================================================================
// Conversions for GeneratorInfoPerMarket
// ================================================================

impl From<GeneratorInfoPerMarket> for GeneratorMarketRecord {
    fn from(info: GeneratorInfoPerMarket) -> Self {
        GeneratorMarketRecord {
            generator_address: format!("{:?}", info.address),
            market_id: info.market_id.to_string(),
            compute_required_per_request: info.compute_required_per_request.to_string(),
            proof_generation_cost: info.proof_generation_cost.to_string(),
            proposed_time: info.proposed_time.to_string(),
            active_requests: info.active_requests.to_string(),
            proofs_submitted: info.proofs_submitted.to_string(),
            proofs_slashed: info.proofs_slashed.to_string(),
            state: format!("{:?}", info.state).into(),
            earnings: U256::zero().to_string(),
            kalypso_points: U256::zero().to_string(),
        }
    }
}

impl From<GeneratorMarketRecord> for GeneratorInfoPerMarket {
    fn from(rec: GeneratorMarketRecord) -> Self {
        GeneratorInfoPerMarket {
            address: rec.generator_address.parse().expect("Invalid address"),
            market_id: rec.market_id.parse().expect("Invalid market id"),
            compute_required_per_request: rec
                .compute_required_per_request
                .parse()
                .expect("Invalid U256"),
            proof_generation_cost: rec.proof_generation_cost.parse().expect("Invalid U256"),
            proposed_time: rec.proposed_time.parse().expect("Invalid U256"),
            active_requests: rec.active_requests.parse().expect("Invalid U256"),
            proofs_submitted: rec.proofs_submitted.parse().expect("Invalid U256"),
            proofs_slashed: rec.proofs_slashed.parse().expect("Invalid U256"),
            state: rec.state.and_then(|s| s.parse().ok()),
        }
    }
}

// ================================================================
// Conversions for WithdrawlRequest
// ================================================================

impl From<WithdrawlRequest> for WithdrawalRequestRecord {
    fn from(req: WithdrawlRequest) -> Self {
        WithdrawalRequestRecord {
            // The generator_address must be set by the caller (e.g., via context)
            generator_address: String::new(),
            account: format!("{:?}", req.account),
            token: format!("{:?}", req.token),
            amount: req.amount.to_string(),
            request_index: req.index.to_string(),
            timestamp: req.timestamp.to_string(),
        }
    }
}

impl From<WithdrawalRequestRecord> for WithdrawlRequest {
    fn from(rec: WithdrawalRequestRecord) -> Self {
        WithdrawlRequest {
            account: rec.account.parse().expect("Invalid account"),
            token: rec.token.parse().expect("Invalid token"),
            amount: rec.amount.parse().expect("Invalid amount"),
            index: rec.request_index.parse().expect("Invalid index"),
            timestamp: rec.timestamp.parse().expect("Invalid timestamp"),
        }
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
