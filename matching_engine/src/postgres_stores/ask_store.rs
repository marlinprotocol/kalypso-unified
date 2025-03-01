use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::QueryResult;
use ethers::core::types::{U256, Address, Bytes, H256};
use std::vec::Vec;
use crate::schema::ask_records::dsl::*;
// use crate::models::AskRecord;

// The DB record does NOT include the private inputs.
#[derive(Debug, Queryable, Insertable)]
#[table_name = "asks"]
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
    pub proving_time_taken: Option<Vec<u8>>,
    pub proving_cost_taken: Option<Vec<u8>>,
    pub proof_transaction: Option<Vec<u8>>,
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
    store: Mutex<HashMap<U256, AskPrivateInputs>>,
}

impl PrivateInputStore {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }

    pub fn insert(&self, ask_id: U256, inputs: AskPrivateInputs) {
        self.store.lock().unwrap().insert(ask_id, inputs);
    }

    pub fn get(&self, ask_id: &U256) -> Option<AskPrivateInputs> {
        self.store.lock().unwrap().get(ask_id).cloned()
    }
}


// Helper conversion functions.
fn u256_to_bytes(val: U256) -> Vec<u8> {
    val.to_fixed_bytes().to_vec()
}

fn bytes_to_u256(b: &[u8]) -> U256 {
    U256::from_big_endian(b)
}

impl From<LocalAsk> for AskRecord {
    fn from(ask: LocalAsk) -> Self {
        AskRecord {
            ask_id: u256_to_bytes(ask.ask_id),
            market_id: u256_to_bytes(ask.market_id),
            reward: u256_to_bytes(ask.reward),
            expiry: u256_to_bytes(ask.expiry),
            deadline: u256_to_bytes(ask.deadline),
            time_requested_for_proof_generation: u256_to_bytes(ask.time_requested_for_proof_generation),
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


impl TryFrom<AskRecord> for LocalAsk {
    type Error = String;
    fn try_from(rec: AskRecord) -> Result<Self, Self::Error> {
        Ok(LocalAsk {
            ask_id: bytes_to_u256(&rec.ask_id),
            market_id: bytes_to_u256(&rec.market_id),
            reward: bytes_to_u256(&rec.reward),
            expiry: bytes_to_u256(&rec.expiry),
            deadline: bytes_to_u256(&rec.deadline),
            time_requested_for_proof_generation: bytes_to_u256(&rec.time_requested_for_proof_generation),
            prover_refund_address: Address::from_slice(&rec.prover_refund_address),
            prover_data: Bytes::from(rec.prover_data),
            has_private_inputs: rec.has_private_inputs,
            state: rec.state.map(|v| match v.get(0) {
                Some(&0) => AskState::Pending,
                Some(&1) => AskState::Completed,
                Some(&2) => AskState::Failed,
                _ => AskState::Pending,
            }),
            generator: rec.generator.map(|b| Address::from_slice(&b)),
            invalid_secret_flag: rec.invalid_secret_flag,
            created_on: bytes_to_u256(&rec.created_on),
            created_on_l1: bytes_to_u256(&rec.created_on_l1),
            create_transaction: H256::from_slice(&rec.create_transaction),
        })
    }
}

/// Database-backed implementation.
pub struct AskDatabase {
    conn: PgConnection,
    private_store: PrivateInputStore,
}
/// Implement AskManagementWrite for AskDatabase.
impl AskManagementWrite for AskDatabase {
    
    fn insert(&mut self, ask: LocalAsk) {
        // Convert to AskRecord, which omits private inputs.
        let record: AskRecord = ask.clone().into();
        
        // Insert non-sensitive fields into the DB.
        diesel::insert_into(ask_records::table)
            .values(&record)
            .execute(&self.conn)
            .expect("Failed to insert ask");

        // If there are private inputs, store them in the private input store.
        if ask.has_private_inputs {
            let private_inputs = AskPrivateInputs {
                secret_data: ask.secret_data.map(|s| s.to_vec()),
                secret_acl: ask.secret_acl.map(|s| s.to_vec()),
            };
            self.private_store.insert(ask.ask_id, private_inputs);
        }
    }
    

    fn remove_ask_only_if_completed(&mut self, id: &U256, _reason: RemoveReason) {
        // Only delete if the ask's state is Completed.
        let id_bytes = u256_to_bytes(*id);
        // Assuming Completed is represented as 1.
        // diesel::delete(ask_records.filter(ask_id.eq(id_bytes))
        //     .filter(state.eq(vec![AskState::Completed as u8])))
        //     .execute(&self.conn)
        //     .expect("Failed to delete ask");
    }

    fn modify_state(&mut self, id: &U256, new_state: AskState) {
        let id_bytes = u256_to_bytes(*id);
        let new_state_bytes = vec![new_state as u8];
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(state.eq(new_state_bytes))
            .execute(&self.conn)
            .expect("Failed to update state");
    }

    fn update_ask_generator(&mut self, id: &U256, new_generator: Option<Address>) {
        let id_bytes = u256_to_bytes(*id);
        let gen_bytes = new_generator.map(|g| g.as_bytes().to_vec());
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(generator.eq(gen_bytes))
            .execute(&self.conn)
            .expect("Failed to update generator");
    }

    fn update_ask_acl(&self, id: &U256, new_acl: Option<Bytes>) {
        let mut store = self.private_store.store.lock().unwrap();

        if let Some(entry) = store.get_mut(id) {
            entry.secret_acl = new_acl.map(|b| b.to_vec());
        } else {
            store.insert(
                *id,
                AskPrivateInputs {
                    secret_data: None, // Keeping secret_data unchanged
                    secret_acl: new_acl.map(|b| b.to_vec()),
                },
            );
        }
    }

    fn update_deadline(&mut self, id: &U256, new_deadline: U256) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(deadline.eq(u256_to_bytes(new_deadline)))
            .execute(&self.conn)
            .expect("Failed to update deadline");
    }

    fn store_valid_proof(
        &mut self,
        id: &U256,
        new_proof: Bytes,
        new_proof_time: U256,
        new_proof_cost: U256,
        new_proof_transaction: String,
    ) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set((
                proof.eq(Some(new_proof.to_vec())),
                proving_time_taken.eq(Some(u256_to_bytes(new_proof_time))),
                proving_cost_taken.eq(Some(u256_to_bytes(new_proof_cost))),
                proof_transaction.eq(Some(new_proof_transaction)),
            ))
            .execute(&self.conn)
            .expect("Failed to store valid proof");
    }

    fn note_invalid_inputs(&mut self, id: &U256, new_proof_cost: U256, new_proof_transaction: String) {
        // implement
    }

    fn note_proof_denied(&mut self, id: &U256, new_proof_transaction: String) {
        // implement    
    }
}

impl AskManagementRead for AskDatabase {
    fn get_proving_time(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proving_time_taken)
            .first::<Option<Vec<u8>>>(&self.conn)
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn get_proving_cost(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proving_cost_taken)
            .first::<Option<Vec<u8>>>(&self.conn)
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn get_proof_transaction(&self, id: &U256) -> Option<String> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proof_transaction)
            .first::<Option<String>>(&self.conn)
            .ok()
            .flatten()
    }

    fn get_proof_by_ask_id(&self, id: &U256) -> Option<Proof> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proof)
            .first::<Option<Vec<u8>>>(&self.conn)
            .ok()
            .and_then(|opt| opt.map(|b| Proof { data: Bytes::from(b) }))
    }

    fn get_by_market_id(&self, market: &U256) -> AskQueryResult {
        let market_bytes = u256_to_bytes(*market);
        let results = ask_records.filter(market_id.eq(market_bytes))
            .load::<AskRecord>(&self.conn)
            .expect("Failed to load asks by market");
        results.into_iter().filter_map(|rec| LocalAsk::try_from(rec).ok()).collect()
    }

    fn get_by_ask_state_except_complete(&self, st: AskState) -> AskQueryResult {
        // Assuming Completed is represented as 1.
        let complete_state = vec![AskState::Completed as u8];
        let target_state = vec![st as u8];
        let results = ask_records
            .filter(state.eq(target_state))
            .filter(state.ne(complete_state))
            .load::<AskRecord>(&self.conn)
            .expect("Failed to load asks by state");
        results.into_iter().filter_map(|rec| LocalAsk::try_from(rec).ok()).collect()
    }

    fn get_cleanup_asks(&self) -> AskQueryResult {
        // Stub: Return asks with deadlines in the past.
        // You’d normally compare deadline with the current timestamp.
        let results = ask_records
            .filter(deadline.lt(u256_to_bytes(U256::zero())))
            .load::<AskRecord>(&self.conn)
            .expect("Failed to load cleanup asks");
        results.into_iter().filter_map(|rec| LocalAsk::try_from(rec).ok()).collect()
    }

    fn get_by_ask_id(&self, id: &U256) -> Option<LocalAsk> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .first::<AskRecord>(&self.conn)
            .ok()
            .and_then(|rec| LocalAsk::try_from(rec).ok())
    }

    fn get_ask_status(&self) -> LocalAskStatus {
        // Example: count asks by state.
        let all_asks = ask_records.load::<AskRecord>(&self.conn)
            .expect("Failed to load asks");
        let mut status = LocalAskStatus::default();
        status.total = all_asks.len();
        for rec in all_asks {
            if let Some(s) = rec.state.and_then(|v| v.get(0).cloned()) {
                match s {
                    x if x == (AskState::Pending as u8) => status.pending += 1,
                    x if x == (AskState::Completed as u8) => status.completed += 1,
                    x if x == (AskState::Failed as u8) => status.failed += 1,
                    _ => {}
                }
            }
        }
        status
    }
}

impl RequestorCounters for AskDatabase {
    fn total_requestor_count(&self) -> usize {
        // Count distinct requestor addresses (using prover_refund_address).
        ask_records.select(prover_refund_address)
            .distinct()
            .load::<Vec<u8>>(&self.conn)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    fn total_requestors_by_market_count(&self, market: &U256) -> usize {
        let market_bytes = u256_to_bytes(*market);
        ask_records.filter(market_id.eq(market_bytes))
            .select(prover_refund_address)
            .distinct()
            .load::<Vec<u8>>(&self.conn)
            .map(|v| v.len())
            .unwrap_or(0)
    }
}

impl ProofCounters for AskDatabase {
    fn get_proof_count(&self, market: &U256) -> usize {
        let market_bytes = u256_to_bytes(*market);
        ask_records.filter(market_id.eq(market_bytes))
            .filter(proof.is_not_null())
            .count()
            .get_result::<i64>(&self.conn)
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }

    fn get_total_proof_count(&self) -> usize {
        ask_records.filter(proof.is_not_null())
            .count()
            .get_result::<i64>(&self.conn)
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }
}


impl MarketRequestCounters for AskDatabase {
    fn get_request_count_by_market_id(&self, market: &U256) -> usize {
        let market_bytes = u256_to_bytes(*market);
        ask_records.filter(market_id.eq(market_bytes))
            .count()
            .get_result::<i64>(&self.conn)
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }

    fn get_total_request_count(&self) -> usize {
        ask_records.count()
            .get_result::<i64>(&self.conn)
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }
}


impl CompletedProofsManagement for AskDatabase {
    fn get_failed_request_count_by_market_id(&self, market: &U256) -> usize {
        let market_bytes = u256_to_bytes(*market);
        // Assuming Failed state is represented as 2.
        ask_records.filter(market_id.eq(market_bytes))
            .filter(state.eq(vec![AskState::Failed as u8]))
            .count()
            .get_result::<i64>(&self.conn)
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }

    fn get_failed_request_count(&self) -> usize {
        ask_records.filter(state.eq(vec![AskState::Failed as u8]))
            .count()
            .get_result::<i64>(&self.conn)
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }

    fn get_recent_completed_proofs(&self, n: usize) -> AskQueryResult {
        // Assuming Completed state is represented as 1 and ordering by job_completed_on_timestamp descending.
        let results = ask_records.filter(state.eq(vec![AskState::Completed as u8]))
            .order(job_completed_on_timestamp.desc().nulls_last())
            .limit(n as i64)
            .load::<AskRecord>(&self.conn)
            .expect("Failed to load recent completed proofs");
        results.into_iter().filter_map(|rec| LocalAsk::try_from(rec).ok()).collect()
    }

    fn get_completed_proof_of_generator(&self, gen: &Address, skip: usize, count: usize) -> AskQueryResult {
        let gen_bytes = gen.as_bytes().to_vec();
        let results = ask_records.filter(generator.eq(Some(gen_bytes)))
            .filter(state.eq(vec![AskState::Completed as u8]))
            .offset(skip as i64)
            .limit(count as i64)
            .load::<AskRecord>(&self.conn)
            .expect("Failed to load completed proofs for generator");
        results.into_iter().filter_map(|rec| LocalAsk::try_from(rec).ok()).collect()
    }

    fn get_completed_proofs_of_market(&self, market: &U256, skip: usize, count: usize) -> AskQueryResult {
        let market_bytes = u256_to_bytes(*market);
        let results = ask_records.filter(market_id.eq(market_bytes))
            .filter(state.eq(vec![AskState::Completed as u8]))
            .offset(skip as i64)
            .limit(count as i64)
            .load::<AskRecord>(&self.conn)
            .expect("Failed to load completed proofs for market");
        results.into_iter().filter_map(|rec| LocalAsk::try_from(rec).ok()).collect()
    }
}


impl TimingOperations for AskDatabase {
    fn get_proof_proof_cycle_completed_on(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proof_cycle_completed_on)
            .first::<Option<Vec<u8>>>(&self.conn)
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn update_proof_proof_cycle_completed_on(&mut self, id: &U256, submitted_on: U256) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(proof_cycle_completed_on.eq(Some(u256_to_bytes(submitted_on))))
            .execute(&self.conn)
            .expect("Failed to update proof cycle timestamp");
    }

    fn get_job_completed_on_timestamp(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(job_completed_on_timestamp)
            .first::<Option<Vec<u8>>>(&self.conn)
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn update_job_completed_on_timestamp(&mut self, id: &U256, ts: U256) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(job_completed_on_timestamp.eq(Some(u256_to_bytes(ts))))
            .execute(&self.conn)
            .expect("Failed to update job completed timestamp");
    }

    fn get_job_matched_on_timestamp(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(job_matched_on_timestamp)
            .first::<Option<Vec<u8>>>(&self.conn)
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn update_job_matched_on_timestamp(&mut self, id: &U256, ts: U256) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(job_matched_on_timestamp.eq(Some(u256_to_bytes(ts))))
            .execute(&self.conn)
            .expect("Failed to update job matched timestamp");
    }

    fn get_job_created_on_timestamp(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(job_created_on_timestamp)
            .first::<Option<Vec<u8>>>(&self.conn)
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn update_job_created_on_timestamp(&mut self, id: &U256, ts: U256) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(job_created_on_timestamp.eq(Some(u256_to_bytes(ts))))
            .execute(&self.conn)
            .expect("Failed to update job created timestamp");
    }

    fn get_overall_proving_time(&self, id: &U256) -> Option<U256> {
        // In this example, we assume the overall proving time is stored directly.
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proving_time_taken)
            .first::<Option<Vec<u8>>>(&self.conn)
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }
}


