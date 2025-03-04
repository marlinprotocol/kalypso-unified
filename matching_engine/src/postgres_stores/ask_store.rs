
use diesel::prelude::*;
use ethers::core::types::{U256, Address, Bytes};
use std::vec::Vec;
use crate::ask_lib::ask_query::AskQueryResult;
use crate::ask_lib::ask_status::{AskState, LocalAskStatus};
use crate::ask_lib::ask_store::{AskManagementRead, AskManagementWrite, CompletedProofsManagement, MarketRequestCounters, ProofCounters, ProofMarketStakeLockManagement, RequestorCounters, TimingOperations};
use crate::ask_lib::{self, AssociatedStakeLock, Proof, RemoveReason};
use crate::ask_lib::ask::LocalAsk;
use crate::schema::ask_records::dsl::*;
use crate::postgres_stores::models::*;
use crate::utility::TokenTracker;


/// Implement AskManagementWrite for AskDatabase.
impl AskManagementWrite for AskDatabase {

    fn insert(&mut self, ask: LocalAsk) {
        // Convert to AskRecord, which omits private inputs.
        let record: AskRecord = ask.clone().into();

        // Insert non-sensitive fields into the DB.
        diesel::insert_into(ask_records)
            .values(&record)
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
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
        // don't do anything
    }

    fn modify_state(&mut self, id: &U256, new_state: AskState) {
        let id_bytes = u256_to_bytes(*id);
        let new_state_bytes = vec![new_state as u8];
        
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(state.eq(new_state_bytes))
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to update state");
    }

    fn update_ask_generator(&mut self, id: &U256, new_generator: Option<Address>) {
        let id_bytes = u256_to_bytes(*id);
        let gen_bytes = new_generator.map(|g| g.as_bytes().to_vec());
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(generator.eq(gen_bytes))
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to update generator");
    }

    fn update_ask_acl(&mut self, id: &U256, new_acl: Option<Bytes>) {
        let store = &mut self.private_store.store;

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
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
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
                // Set proof_type to "ValidProof" when proof is valid.
                proof_type.eq(Some("ValidProof".to_string())),
            ))
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to store valid proof");
    }

    fn note_invalid_inputs(&mut self, id: &U256, new_proof_cost: U256, new_proof_transaction: String) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set((
                // Update the cost and transaction details.
                proving_cost_taken.eq(Some(u256_to_bytes(new_proof_cost))),
                proof_transaction.eq(Some(new_proof_transaction)),
                // Set proof_type to "InvalidInputAttestation" when inputs are invalid.
                proof_type.eq(Some("InvalidInputAttestation".to_string())),
            ))
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to note invalid inputs");
    
    }

    fn note_proof_denied(&mut self, id: &U256, new_proof_transaction: String) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set((
                proof_transaction.eq(Some(new_proof_transaction)),
                // Set proof_type to "FailedProofGeneration" to indicate the proof was denied.
                proof_type.eq(Some("FailedProofGeneration".to_string())),
            ))
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to note proof denied"); 
    }
}

impl AskManagementRead for AskDatabase {
    fn get_proving_time(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proving_time_taken)
            .first::<Option<Vec<u8>>>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn get_proving_cost(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proving_cost_taken)
            .first::<Option<Vec<u8>>>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn get_proof_transaction(&self, id: &U256) -> Option<String> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proof_transaction)
            .first::<Option<String>>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .ok()
            .flatten()
    }

    fn get_proof_by_ask_id(&self, id: &U256) -> Option<Proof> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proof)
            .first::<Option<Vec<u8>>>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .ok()
            .and_then(|opt| opt.map(|b| Proof::from(ask_lib::Proof::ValidProof(Bytes::from(b)))))
    }

    fn get_by_market_id(&self, market: &U256) -> AskQueryResult {
        let market_bytes = u256_to_bytes(*market);
        let results = ask_records.filter(market_id.eq(market_bytes))
            .select(AskRecord::as_select())
            .load::<AskRecord>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to load asks by market");
        AskQueryResult { asks: Some(results.into_iter().filter_map(|rec| self.try_from_ask_record(rec).ok()).collect()) }
    }

    fn get_by_ask_state_except_complete(&self, st: AskState) -> AskQueryResult {
        // Assuming Completed is represented as 1.
        let complete_state = vec![AskState::Complete as u8];
        let target_state = vec![st as u8];
        let results = ask_records
                    .filter(state.eq(target_state))
                    .filter(state.ne(complete_state))
                    .select(AskRecord::as_select())
                    .load::<AskRecord>(&mut self.pool.get().expect("Failed to get connection from pool"))
                    .expect("Failed to load asks by state");
        AskQueryResult { asks: Some(results.into_iter().filter_map(|rec| self.try_from_ask_record(rec).ok()).collect()) }
    }

    fn get_cleanup_asks(&self) -> AskQueryResult {
        let results = ask_records
            .filter(deadline.lt(u256_to_bytes(U256::zero())))
            .select(AskRecord::as_select())
            .load::<AskRecord>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to load cleanup asks");
        // Call the conversion method on each record.
        AskQueryResult {
            asks: Some(results.into_iter()
                .filter_map(|rec| self.try_from_ask_record(rec).ok())
                .collect())
        }
    }

    fn get_by_ask_id(&self, id: &U256) -> Option<LocalAsk> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(AskRecord::as_select())
            .first::<AskRecord>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .ok()
            .and_then(|rec| self.try_from_ask_record(rec).ok())
    }

    fn get_ask_status(&self) -> LocalAskStatus {
        // Example: count asks by state.
        let all_asks = ask_records.select(AskRecord::as_select()).load::<AskRecord>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to load asks");
        let mut status = LocalAskStatus{
            created: 0,
            unassigned: 0,
            assigned: 0,
            completed: 0,
            deadline_crossed: 0,
            invalid_secret: 0,
        };
        
        for rec in all_asks {
            if let Some(s) = rec.state.and_then(|v| v.get(0).cloned()) {
                match s {
                    // x if x == (AskState::Null as u8) => status.null += 1,
                    x if x == (AskState::Create as u8) => status.created += 1,
                    x if x == (AskState::UnAssigned as u8) => status.unassigned += 1,
                    x if x == (AskState::Assigned as u8) => status.assigned += 1,
                    x if x == (AskState::Complete as u8) => status.completed += 1,
                    x if x == (AskState::DeadlineCrossed as u8) => status.deadline_crossed += 1,
                    x if x == (AskState::InvalidSecret as u8) => status.invalid_secret += 1,
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
            .load::<Vec<u8>>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .map(|v| v.len())
            .unwrap_or(0)
    }

    fn total_requestors_by_market_count(&self, market: &U256) -> usize {
        let market_bytes = u256_to_bytes(*market);
        ask_records.filter(market_id.eq(market_bytes))
            .select(prover_refund_address)
            .distinct()
            .load::<Vec<u8>>(&mut self.pool.get().expect("Failed to get connection from pool"))
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
            .get_result::<i64>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }

    fn get_total_proof_count(&self) -> usize {
        ask_records.filter(proof.is_not_null())
            .count()
            .get_result::<i64>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }
}

impl MarketRequestCounters for AskDatabase {
    fn get_request_count_by_market_id(&self, market: &U256) -> usize {
        let market_bytes = u256_to_bytes(*market);
        ask_records.filter(market_id.eq(market_bytes))
            .count()
            .get_result::<i64>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }

    fn get_total_request_count(&self) -> usize {
        ask_records.count()
            .get_result::<i64>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }
}

impl CompletedProofsManagement for AskDatabase {
    fn get_failed_request_count_by_market_id(&self, market: &U256) -> usize {
        let market_bytes = u256_to_bytes(*market);
        // Assuming Failed state is represented as 2.
        ask_records.filter(market_id.eq(market_bytes))
            .filter(state.eq(vec![AskState::InvalidSecret as u8]).or(state.eq(vec![AskState::DeadlineCrossed as u8])))
            .count()
            .get_result::<i64>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }

    fn get_failed_request_count(&self) -> usize {
        ask_records.filter(state.eq(vec![AskState::InvalidSecret as u8]).or(state.eq(vec![AskState::DeadlineCrossed as u8])))
            .count()
            .get_result::<i64>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .map(|cnt| cnt as usize)
            .unwrap_or(0)
    }

    fn get_recent_completed_proofs(&self, n: usize) -> Vec<LocalAsk> {
        // let results = ask_records.filter(state.eq(vec![AskState::Complete as u8]))
        //     .order(job_completed_on_timestamp.desc().nulls_last())
        //     .limit(n as i64)
        //     .load::<AskRecord>(&mut self.pool.get().expect("Failed to get connection from pool"))
        //     .expect("Failed to load recent completed proofs");
        // results.into_iter().filter_map(|rec| LocalAsk::try_from(rec).ok()).collect()
        return Vec::new();
    }

    fn get_completed_proof_of_generator(&self, gen: &Address, skip: usize, count: usize) -> Vec<LocalAsk> {
        // let gen_bytes = gen.as_bytes().to_vec();
        // let results = ask_records.filter(generator.eq(Some(gen_bytes)))
        //     .filter(state.eq(vec![AskState::Complete as u8]))
        //     .offset(skip as i64)
        //     .limit(count as i64)
        //     .load::<AskRecord>(&mut self.pool.get().expect("Failed to get connection from pool"))
        //     .expect("Failed to load completed proofs for generator");
        // results.into_iter().filter_map(|rec| LocalAsk::try_from(rec).ok()).collect()
        return Vec::new();
    }

    fn get_completed_proofs_of_market(&self, market: &U256, skip: usize, count: usize) -> Vec<LocalAsk> {
        // let market_bytes = u256_to_bytes(*market);
        // let results = ask_records.filter(market_id.eq(market_bytes))
        //     .filter(state.eq(vec![AskState::Complete as u8]))
        //     .offset(skip as i64)
        //     .limit(count as i64)
        //     .load::<AskRecord>(&mut self.pool.get().expect("Failed to get connection from pool"))
        //     .expect("Failed to load completed proofs for market");
        // results.into_iter().filter_map(|rec| LocalAsk::try_from(rec).ok()).collect()
        return Vec::new();
    }
}

impl TimingOperations for AskDatabase {
    fn get_proof_proof_cycle_completed_on(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proof_cycle_completed_on)
            .first::<Option<Vec<u8>>>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn update_proof_proof_cycle_completed_on(&mut self, id: &U256, submitted_on: U256) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(proof_cycle_completed_on.eq(Some(u256_to_bytes(submitted_on))))
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to update proof cycle timestamp");
    }

    fn get_job_completed_on_timestamp(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(job_completed_on_timestamp)
            .first::<Option<Vec<u8>>>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn update_job_completed_on_timestamp(&mut self, id: &U256, ts: U256) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(job_completed_on_timestamp.eq(Some(u256_to_bytes(ts))))
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to update job completed timestamp");
    }

    fn get_job_matched_on_timestamp(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(job_matched_on_timestamp)
            .first::<Option<Vec<u8>>>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn update_job_matched_on_timestamp(&mut self, id: &U256, ts: U256) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(job_matched_on_timestamp.eq(Some(u256_to_bytes(ts))))
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to update job matched timestamp");
    }

    fn get_job_created_on_timestamp(&self, id: &U256) -> Option<U256> {
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(job_created_on_timestamp)
            .first::<Option<Vec<u8>>>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }

    fn update_job_created_on_timestamp(&mut self, id: &U256, ts: U256) {
        let id_bytes = u256_to_bytes(*id);
        diesel::update(ask_records.filter(ask_id.eq(id_bytes)))
            .set(job_created_on_timestamp.eq(Some(u256_to_bytes(ts))))
            .execute(&mut self.pool.get().expect("Failed to get connection from pool"))
            .expect("Failed to update job created timestamp");
    }

    fn get_overall_proving_time(&self, id: &U256) -> Option<U256> {
        // In this example, we assume the overall proving time is stored directly.
        let id_bytes = u256_to_bytes(*id);
        ask_records.filter(ask_id.eq(id_bytes))
            .select(proving_time_taken)
            .first::<Option<Vec<u8>>>(&mut self.pool.get().expect("Failed to get connection from pool"))
            .ok()
            .and_then(|opt| opt.map(|b| bytes_to_u256(&b)))
    }
}

impl ProofMarketStakeLockManagement for AskDatabase {
    fn get_associated_stake_lock(&self, ask_id_db: &U256) -> Option<AssociatedStakeLock>{
        // unimplemented
        None
    }
    
    fn add_associated_native_stake_lock(&mut self, ask_id_db: &U256, stake_locked: TokenTracker){
        // unimplemented
    }

    fn add_associated_symbiotic_stake_lock(&mut self, ask_id_db: &U256, stake_locked: TokenTracker){
        // unimplemented
    }

    fn delete_all_associated_stake_locks(&mut self, ask_id_db: &U256){
        //unimplemented
    }
}