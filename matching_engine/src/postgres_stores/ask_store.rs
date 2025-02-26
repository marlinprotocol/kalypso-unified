// use diesel::prelude::*;
// use diesel::pg::PgConnection;
// use crate::schema::asks;

// pub struct AskRepository {
//     conn: PgConnection,
// }

// impl AskRepository {
//     pub fn new(database_url: &str) -> Self {
//         let conn = PgConnection::establish(database_url)
//             .expect("Error connecting to database");
//         Self { conn }
//     }
// }

// impl AskManagementWrite for AskRepository {
//     fn insert(&mut self, ask: LocalAsk) {
//         let new_ask = (
//             asks::id.eq(ask.id),
//             asks::market_id.eq(ask.market_id),
//             asks::generator.eq(ask.generator.map(|g| g.to_string())),
//             asks::acl.eq(ask.acl),
//             asks::state.eq(ask.state.to_string()),
//             asks::deadline.eq(ask.deadline),
//         );

//         diesel::insert_into(asks::table)
//             .values(&new_ask)
//             .execute(&mut self.conn)
//             .expect("Error inserting ask");
//     }

//     fn remove_ask_only_if_completed(&mut self, ask_id: &U256, reason: RemoveReason) {
//         use crate::schema::asks::dsl::*;

//         diesel::delete(asks)
//             .filter(id.eq(ask_id))
//             .filter(state.eq("COMPLETED"))
//             .execute(&mut self.conn)
//             .expect("Error removing ask");
//     }

//     fn modify_state(&mut self, ask_id: &U256, new_state: AskState) {
//         use crate::schema::asks::dsl::*;

//         diesel::update(asks)
//             .filter(id.eq(ask_id))
//             .set(state.eq(new_state.to_string()))
//             .execute(&mut self.conn)
//             .expect("Error modifying ask state");
//     }

//     fn update_ask_generator(&mut self, ask_id: &U256, new_generator: Option<Address>) {
//         use crate::schema::asks::dsl::*;

//         diesel::update(asks)
//             .filter(id.eq(ask_id))
//             .set(generator.eq(new_generator.map(|g| g.to_string())))
//             .execute(&mut self.conn)
//             .expect("Error updating generator");
//     }

//     fn update_ask_acl(&mut self, ask_id: &U256, new_acl: Option<Bytes>) {
//         use crate::schema::asks::dsl::*;

//         diesel::update(asks)
//             .filter(id.eq(ask_id))
//             .set(acl.eq(new_acl))
//             .execute(&mut self.conn)
//             .expect("Error updating ACL");
//     }

//     fn update_deadline(&mut self, ask_id: &U256, deadline_value: U256) {
//         use crate::schema::asks::dsl::*;

//         diesel::update(asks)
//             .filter(id.eq(ask_id))
//             .set(deadline.eq(deadline_value))
//             .execute(&mut self.conn)
//             .expect("Error updating deadline");
//     }

//     fn store_valid_proof(
//         &mut self,
//         ask_id: &U256,
//         proof_value: Bytes,
//         proof_time_value: U256,
//         proof_cost_value: U256,
//         proof_transaction_value: String,
//     ) {
//         use crate::schema::proofs::dsl::*;

//         let new_proof = (
//             ask_id.eq(ask_id),
//             proof.eq(proof_value),
//             proof_time.eq(proof_time_value),
//             proof_cost.eq(proof_cost_value),
//             proof_transaction.eq(proof_transaction_value),
//         );

//         diesel::insert_into(proofs)
//             .values(&new_proof)
//             .execute(&mut self.conn)
//             .expect("Error storing proof");
//     }

//     fn note_invalid_inputs(&mut self, ask_id: &U256, proof_cost_value: U256, proof_transaction_value: String) {
//         self.store_valid_proof(ask_id, vec![], U256::zero(), proof_cost_value, proof_transaction_value);
//     }

//     fn note_proof_denied(&mut self, ask_id: &U256, proof_transaction_value: String) {
//         self.store_valid_proof(ask_id, vec![], U256::zero(), U256::zero(), proof_transaction_value);
//     }
// }


// impl AskManagementRead for AskRepository {
//     fn get_proving_time(&self, ask_id: &U256) -> Option<U256> {
//         use crate::schema::proofs::dsl::*;
//         proofs.filter(ask_id.eq(ask_id))
//             .select(proof_time)
//             .first::<U256>(&self.conn)
//             .ok()
//     }

//     fn get_proving_cost(&self, ask_id: &U256) -> Option<U256> {
//         use crate::schema::proofs::dsl::*;
//         proofs.filter(ask_id.eq(ask_id))
//             .select(proof_cost)
//             .first::<U256>(&self.conn)
//             .ok()
//     }

//     fn get_proof_transaction(&self, ask_id: &U256) -> Option<String> {
//         use crate::schema::proofs::dsl::*;
//         proofs.filter(ask_id.eq(ask_id))
//             .select(proof_transaction)
//             .first::<String>(&self.conn)
//             .ok()
//     }

//     fn get_by_market_id(&self, market_id: &U256) -> AskQueryResult {
//         use crate::schema::asks::dsl::*;
//         asks.filter(market_id.eq(market_id))
//             .load::<LocalAsk>(&self.conn)
//             .expect("Error querying asks by market ID")
//     }
// }


// impl RequestorCounters for AskRepository {
//     fn total_requestor_count(&self) -> usize {
//         use crate::schema::requestors::dsl::*;
//         requestors.count().get_result::<i64>(&self.conn).unwrap_or(0) as usize
//     }

//     fn total_requestors_by_market_count(&self, market_id: &U256) -> usize {
//         use crate::schema::requestors::dsl::*;
//         requestors
//             .filter(market_id.eq(market_id))
//             .select(requestor_count)
//             .first::<i32>(&self.conn)
//             .unwrap_or(0) as usize
//     }
// }

// impl ProofCounters for AskRepository {
//     fn get_proof_count(&self, market_id: &U256) -> usize {
//         use crate::schema::proof_counters::dsl::*;
//         proof_counters
//             .filter(market_id.eq(market_id))
//             .select(proof_count)
//             .first::<i32>(&self.conn)
//             .unwrap_or(0) as usize
//     }

//     fn get_total_proof_count(&self) -> usize {
//         use crate::schema::proof_counters::dsl::*;
//         proof_counters.sum(proof_count).get_result::<Option<i64>>(&self.conn).unwrap_or(Some(0)).unwrap_or(0) as usize
//     }
// }

// impl MarketRequestCounters for AskRepository {
//     fn get_request_count_by_market_id(&self, market_id: &U256) -> usize {
//         use crate::schema::requestors::dsl::*;
//         requestors
//             .filter(market_id.eq(market_id))
//             .select(requestor_count)
//             .first::<i32>(&self.conn)
//             .unwrap_or(0) as usize
//     }

//     fn get_total_request_count(&self) -> usize {
//         use crate::schema::requestors::dsl::*;
//         requestors.sum(requestor_count).get_result::<Option<i64>>(&self.conn).unwrap_or(Some(0)).unwrap_or(0) as usize
//     }
// }


// impl CompletedProofsManagement for AskRepository {
//     fn get_failed_request_count_by_market_id(&self, market_id: &U256) -> usize {
//         use crate::schema::proof_counters::dsl::*;
//         proof_counters
//             .filter(market_id.eq(market_id))
//             .select(proof_count)
//             .first::<i32>(&self.conn)
//             .unwrap_or(0) as usize
//     }

//     fn get_failed_request_count(&self) -> usize {
//         use crate::schema::proof_counters::dsl::*;
//         proof_counters.sum(proof_count).get_result::<Option<i64>>(&self.conn).unwrap_or(Some(0)).unwrap_or(0) as usize
//     }

//     fn get_recent_completed_proofs(&self, n: usize) -> Vec<LocalAsk> {
//         use crate::schema::asks::dsl::*;
//         asks
//             .filter(state.eq("COMPLETED"))
//             .order(deadline.desc())
//             .limit(n as i64)
//             .load::<LocalAsk>(&self.conn)
//             .expect("Error fetching recent completed proofs")
//     }

//     fn get_completed_proof_of_generator(
//         &self,
//         generator_addr: &Address,
//         skip: usize,
//         count: usize,
//     ) -> Vec<LocalAsk> {
//         use crate::schema::asks::dsl::*;
//         asks
//             .filter(state.eq("COMPLETED"))
//             .filter(generator.eq(generator_addr.to_string()))
//             .order(deadline.desc())
//             .offset(skip as i64)
//             .limit(count as i64)
//             .load::<LocalAsk>(&self.conn)
//             .expect("Error fetching completed proofs by generator")
//     }

//     fn get_completed_proofs_of_market(
//         &self,
//         market_id_value: &U256,
//         skip: usize,
//         count: usize,
//     ) -> Vec<LocalAsk> {
//         use crate::schema::asks::dsl::*;
//         asks
//             .filter(state.eq("COMPLETED"))
//             .filter(market_id.eq(market_id_value))
//             .order(deadline.desc())
//             .offset(skip as i64)
//             .limit(count as i64)
//             .load::<LocalAsk>(&self.conn)
//             .expect("Error fetching completed proofs by market")
//     }
// }


// impl TimingOperations for AskRepository {
//     fn get_proof_proof_cycle_completed_on(&self, ask_id: &U256) -> Option<U256> {
//         use crate::schema::timing_operations::dsl::*;
//         timing_operations
//             .filter(ask_id.eq(ask_id))
//             .select(proof_cycle_completed_on)
//             .first::<Option<U256>>(&self.conn)
//             .ok()
//             .flatten()
//     }

//     fn update_proof_proof_cycle_completed_on(&mut self, ask_id: &U256, submitted_on: U256) {
//         use crate::schema::timing_operations::dsl::*;
//         diesel::update(timing_operations)
//             .filter(ask_id.eq(ask_id))
//             .set(proof_cycle_completed_on.eq(submitted_on))
//             .execute(&mut self.conn)
//             .expect("Error updating proof cycle completion timestamp");
//     }

//     fn get_job_completed_on_timestamp(&self, ask_id: &U256) -> Option<U256> {
//         use crate::schema::timing_operations::dsl::*;
//         timing_operations
//             .filter(ask_id.eq(ask_id))
//             .select(job_completed_on)
//             .first::<Option<U256>>(&self.conn)
//             .ok()
//             .flatten()
//     }

//     fn update_job_completed_on_timestamp(&mut self, ask_id: &U256, completed_on_timestamp: U256) {
//         use crate::schema::timing_operations::dsl::*;
//         diesel::update(timing_operations)
//             .filter(ask_id.eq(ask_id))
//             .set(job_completed_on.eq(completed_on_timestamp))
//             .execute(&mut self.conn)
//             .expect("Error updating job completed timestamp");
//     }

//     fn get_job_matched_on_timestamp(&self, ask_id: &U256) -> Option<U256> {
//         use crate::schema::timing_operations::dsl::*;
//         timing_operations
//             .filter(ask_id.eq(ask_id))
//             .select(job_matched_on)
//             .first::<Option<U256>>(&self.conn)
//             .ok()
//             .flatten()
//     }

//     fn update_job_matched_on_timestamp(&mut self, ask_id: &U256, matched_on_timestamp: U256) {
//         use crate::schema::timing_operations::dsl::*;
//         diesel::update(timing_operations)
//             .filter(ask_id.eq(ask_id))
//             .set(job_matched_on.eq(matched_on_timestamp))
//             .execute(&mut self.conn)
//             .expect("Error updating job matched timestamp");
//     }

//     fn get_job_created_on_timestamp(&self, ask_id: &U256) -> Option<U256> {
//         use crate::schema::timing_operations::dsl::*;
//         timing_operations
//             .filter(ask_id.eq(ask_id))
//             .select(job_created_on)
//             .first::<Option<U256>>(&self.conn)
//             .ok()
//             .flatten()
//     }

//     fn update_job_created_on_timestamp(&mut self, ask_id: &U256, created_on_timestamp: U256) {
//         use crate::schema::timing_operations::dsl::*;
//         diesel::update(timing_operations)
//             .filter(ask_id.eq(ask_id))
//             .set(job_created_on.eq(created_on_timestamp))
//             .execute(&mut self.conn)
//             .expect("Error updating job created timestamp");
//     }

//     fn get_overall_proving_time(&self, ask_id: &U256) -> Option<U256> {
//         use crate::schema::timing_operations::dsl::*;
//         timing_operations
//             .filter(ask_id.eq(ask_id))
//             .select(proof_cycle_completed_on)
//             .first::<Option<U256>>(&self.conn)
//             .ok()
//             .flatten()
//     }
// }

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::QueryResult;
use ethers::core::types::{U256, Address, Bytes, H256};
use std::vec::Vec;
use crate::schema::ask_records::dsl::*;
use crate::models::AskRecord;

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
            secret_data: ask.secret_data.map(|s| s.to_vec()),
            secret_acl: ask.secret_acl.map(|s| s.to_vec()),
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
            secret_data: rec.secret_data.map(Bytes::from),
            secret_acl: rec.secret_acl.map(Bytes::from),
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
    pub conn: PgConnection,
}

/// Implement AskManagementWrite for AskDatabase.
impl AskManagementWrite for AskDatabase {
    fn insert(&mut self, ask: LocalAsk) {
        let record: AskRecord = ask.into();
        diesel::insert_into(ask_records)
            .values(&record)
            .execute(&self.conn)
            .expect("Failed to insert ask");
    }

    fn remove_ask_only_if_completed(&mut self, id: &U256, _reason: RemoveReason) {
        // Only delete if the ask's state is Completed.
        let id_bytes = u256_to_bytes(*id);
        // Assuming Completed is represented as 1.
        diesel::delete(ask_records.filter(ask_id.eq(id_bytes))
            .filter(state.eq(vec![AskState::Completed as u8])))
            .execute(&self.conn)
            .expect("Failed to delete ask");
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

    fn update_ask_acl(&mut self, id: &U256, new_acl: Option<Bytes>) {
        let id_bytes = u256_to_bytes(*id);
        let acl_bytes = new_acl.map(|b| b.to_vec());
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(secret_acl.eq(acl_bytes))
            .execute(&self.conn)
            .expect("Failed to update ACL");
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
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set((
                proving_cost_taken.eq(Some(u256_to_bytes(new_proof_cost))),
                proof_transaction.eq(Some(new_proof_transaction)),
            ))
            .execute(&self.conn)
            .expect("Failed to note invalid inputs");
    }

    fn note_proof_denied(&mut self, id: &U256, new_proof_transaction: String) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(proof_transaction.eq(Some(new_proof_transaction)))
            .execute(&self.conn)
            .expect("Failed to note proof denied");
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


