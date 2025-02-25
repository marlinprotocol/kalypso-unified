use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::asks;

pub struct AskRepository {
    conn: PgConnection,
}

impl AskRepository {
    pub fn new(database_url: &str) -> Self {
        let conn = PgConnection::establish(database_url)
            .expect("Error connecting to database");
        Self { conn }
    }
}

impl AskManagementWrite for AskRepository {
    fn insert(&mut self, ask: LocalAsk) {
        let new_ask = (
            asks::id.eq(ask.id),
            asks::market_id.eq(ask.market_id),
            asks::generator.eq(ask.generator.map(|g| g.to_string())),
            asks::acl.eq(ask.acl),
            asks::state.eq(ask.state.to_string()),
            asks::deadline.eq(ask.deadline),
        );

        diesel::insert_into(asks::table)
            .values(&new_ask)
            .execute(&mut self.conn)
            .expect("Error inserting ask");
    }

    fn remove_ask_only_if_completed(&mut self, ask_id: &U256, reason: RemoveReason) {
        use crate::schema::asks::dsl::*;

        diesel::delete(asks)
            .filter(id.eq(ask_id))
            .filter(state.eq("COMPLETED"))
            .execute(&mut self.conn)
            .expect("Error removing ask");
    }

    fn modify_state(&mut self, ask_id: &U256, new_state: AskState) {
        use crate::schema::asks::dsl::*;

        diesel::update(asks)
            .filter(id.eq(ask_id))
            .set(state.eq(new_state.to_string()))
            .execute(&mut self.conn)
            .expect("Error modifying ask state");
    }

    fn update_ask_generator(&mut self, ask_id: &U256, new_generator: Option<Address>) {
        use crate::schema::asks::dsl::*;

        diesel::update(asks)
            .filter(id.eq(ask_id))
            .set(generator.eq(new_generator.map(|g| g.to_string())))
            .execute(&mut self.conn)
            .expect("Error updating generator");
    }

    fn update_ask_acl(&mut self, ask_id: &U256, new_acl: Option<Bytes>) {
        use crate::schema::asks::dsl::*;

        diesel::update(asks)
            .filter(id.eq(ask_id))
            .set(acl.eq(new_acl))
            .execute(&mut self.conn)
            .expect("Error updating ACL");
    }

    fn update_deadline(&mut self, ask_id: &U256, deadline_value: U256) {
        use crate::schema::asks::dsl::*;

        diesel::update(asks)
            .filter(id.eq(ask_id))
            .set(deadline.eq(deadline_value))
            .execute(&mut self.conn)
            .expect("Error updating deadline");
    }

    fn store_valid_proof(
        &mut self,
        ask_id: &U256,
        proof_value: Bytes,
        proof_time_value: U256,
        proof_cost_value: U256,
        proof_transaction_value: String,
    ) {
        use crate::schema::proofs::dsl::*;

        let new_proof = (
            ask_id.eq(ask_id),
            proof.eq(proof_value),
            proof_time.eq(proof_time_value),
            proof_cost.eq(proof_cost_value),
            proof_transaction.eq(proof_transaction_value),
        );

        diesel::insert_into(proofs)
            .values(&new_proof)
            .execute(&mut self.conn)
            .expect("Error storing proof");
    }

    fn note_invalid_inputs(&mut self, ask_id: &U256, proof_cost_value: U256, proof_transaction_value: String) {
        self.store_valid_proof(ask_id, vec![], U256::zero(), proof_cost_value, proof_transaction_value);
    }

    fn note_proof_denied(&mut self, ask_id: &U256, proof_transaction_value: String) {
        self.store_valid_proof(ask_id, vec![], U256::zero(), U256::zero(), proof_transaction_value);
    }
}


impl AskManagementRead for AskRepository {
    fn get_proving_time(&self, ask_id: &U256) -> Option<U256> {
        use crate::schema::proofs::dsl::*;
        proofs.filter(ask_id.eq(ask_id))
            .select(proof_time)
            .first::<U256>(&self.conn)
            .ok()
    }

    fn get_proving_cost(&self, ask_id: &U256) -> Option<U256> {
        use crate::schema::proofs::dsl::*;
        proofs.filter(ask_id.eq(ask_id))
            .select(proof_cost)
            .first::<U256>(&self.conn)
            .ok()
    }

    fn get_proof_transaction(&self, ask_id: &U256) -> Option<String> {
        use crate::schema::proofs::dsl::*;
        proofs.filter(ask_id.eq(ask_id))
            .select(proof_transaction)
            .first::<String>(&self.conn)
            .ok()
    }

    fn get_by_market_id(&self, market_id: &U256) -> AskQueryResult {
        use crate::schema::asks::dsl::*;
        asks.filter(market_id.eq(market_id))
            .load::<LocalAsk>(&self.conn)
            .expect("Error querying asks by market ID")
    }
}


impl RequestorCounters for AskRepository {
    fn total_requestor_count(&self) -> usize {
        use crate::schema::requestors::dsl::*;
        requestors.count().get_result::<i64>(&self.conn).unwrap_or(0) as usize
    }

    fn total_requestors_by_market_count(&self, market_id: &U256) -> usize {
        use crate::schema::requestors::dsl::*;
        requestors
            .filter(market_id.eq(market_id))
            .select(requestor_count)
            .first::<i32>(&self.conn)
            .unwrap_or(0) as usize
    }
}

impl ProofCounters for AskRepository {
    fn get_proof_count(&self, market_id: &U256) -> usize {
        use crate::schema::proof_counters::dsl::*;
        proof_counters
            .filter(market_id.eq(market_id))
            .select(proof_count)
            .first::<i32>(&self.conn)
            .unwrap_or(0) as usize
    }

    fn get_total_proof_count(&self) -> usize {
        use crate::schema::proof_counters::dsl::*;
        proof_counters.sum(proof_count).get_result::<Option<i64>>(&self.conn).unwrap_or(Some(0)).unwrap_or(0) as usize
    }
}

impl MarketRequestCounters for AskRepository {
    fn get_request_count_by_market_id(&self, market_id: &U256) -> usize {
        use crate::schema::requestors::dsl::*;
        requestors
            .filter(market_id.eq(market_id))
            .select(requestor_count)
            .first::<i32>(&self.conn)
            .unwrap_or(0) as usize
    }

    fn get_total_request_count(&self) -> usize {
        use crate::schema::requestors::dsl::*;
        requestors.sum(requestor_count).get_result::<Option<i64>>(&self.conn).unwrap_or(Some(0)).unwrap_or(0) as usize
    }
}


impl CompletedProofsManagement for AskRepository {
    fn get_failed_request_count_by_market_id(&self, market_id: &U256) -> usize {
        use crate::schema::proof_counters::dsl::*;
        proof_counters
            .filter(market_id.eq(market_id))
            .select(proof_count)
            .first::<i32>(&self.conn)
            .unwrap_or(0) as usize
    }

    fn get_failed_request_count(&self) -> usize {
        use crate::schema::proof_counters::dsl::*;
        proof_counters.sum(proof_count).get_result::<Option<i64>>(&self.conn).unwrap_or(Some(0)).unwrap_or(0) as usize
    }

    fn get_recent_completed_proofs(&self, n: usize) -> Vec<LocalAsk> {
        use crate::schema::asks::dsl::*;
        asks
            .filter(state.eq("COMPLETED"))
            .order(deadline.desc())
            .limit(n as i64)
            .load::<LocalAsk>(&self.conn)
            .expect("Error fetching recent completed proofs")
    }

    fn get_completed_proof_of_generator(
        &self,
        generator_addr: &Address,
        skip: usize,
        count: usize,
    ) -> Vec<LocalAsk> {
        use crate::schema::asks::dsl::*;
        asks
            .filter(state.eq("COMPLETED"))
            .filter(generator.eq(generator_addr.to_string()))
            .order(deadline.desc())
            .offset(skip as i64)
            .limit(count as i64)
            .load::<LocalAsk>(&self.conn)
            .expect("Error fetching completed proofs by generator")
    }

    fn get_completed_proofs_of_market(
        &self,
        market_id_value: &U256,
        skip: usize,
        count: usize,
    ) -> Vec<LocalAsk> {
        use crate::schema::asks::dsl::*;
        asks
            .filter(state.eq("COMPLETED"))
            .filter(market_id.eq(market_id_value))
            .order(deadline.desc())
            .offset(skip as i64)
            .limit(count as i64)
            .load::<LocalAsk>(&self.conn)
            .expect("Error fetching completed proofs by market")
    }
}


impl TimingOperations for AskRepository {
    fn get_proof_proof_cycle_completed_on(&self, ask_id: &U256) -> Option<U256> {
        use crate::schema::timing_operations::dsl::*;
        timing_operations
            .filter(ask_id.eq(ask_id))
            .select(proof_cycle_completed_on)
            .first::<Option<U256>>(&self.conn)
            .ok()
            .flatten()
    }

    fn update_proof_proof_cycle_completed_on(&mut self, ask_id: &U256, submitted_on: U256) {
        use crate::schema::timing_operations::dsl::*;
        diesel::update(timing_operations)
            .filter(ask_id.eq(ask_id))
            .set(proof_cycle_completed_on.eq(submitted_on))
            .execute(&mut self.conn)
            .expect("Error updating proof cycle completion timestamp");
    }

    fn get_job_completed_on_timestamp(&self, ask_id: &U256) -> Option<U256> {
        use crate::schema::timing_operations::dsl::*;
        timing_operations
            .filter(ask_id.eq(ask_id))
            .select(job_completed_on)
            .first::<Option<U256>>(&self.conn)
            .ok()
            .flatten()
    }

    fn update_job_completed_on_timestamp(&mut self, ask_id: &U256, completed_on_timestamp: U256) {
        use crate::schema::timing_operations::dsl::*;
        diesel::update(timing_operations)
            .filter(ask_id.eq(ask_id))
            .set(job_completed_on.eq(completed_on_timestamp))
            .execute(&mut self.conn)
            .expect("Error updating job completed timestamp");
    }

    fn get_job_matched_on_timestamp(&self, ask_id: &U256) -> Option<U256> {
        use crate::schema::timing_operations::dsl::*;
        timing_operations
            .filter(ask_id.eq(ask_id))
            .select(job_matched_on)
            .first::<Option<U256>>(&self.conn)
            .ok()
            .flatten()
    }

    fn update_job_matched_on_timestamp(&mut self, ask_id: &U256, matched_on_timestamp: U256) {
        use crate::schema::timing_operations::dsl::*;
        diesel::update(timing_operations)
            .filter(ask_id.eq(ask_id))
            .set(job_matched_on.eq(matched_on_timestamp))
            .execute(&mut self.conn)
            .expect("Error updating job matched timestamp");
    }

    fn get_job_created_on_timestamp(&self, ask_id: &U256) -> Option<U256> {
        use crate::schema::timing_operations::dsl::*;
        timing_operations
            .filter(ask_id.eq(ask_id))
            .select(job_created_on)
            .first::<Option<U256>>(&self.conn)
            .ok()
            .flatten()
    }

    fn update_job_created_on_timestamp(&mut self, ask_id: &U256, created_on_timestamp: U256) {
        use crate::schema::timing_operations::dsl::*;
        diesel::update(timing_operations)
            .filter(ask_id.eq(ask_id))
            .set(job_created_on.eq(created_on_timestamp))
            .execute(&mut self.conn)
            .expect("Error updating job created timestamp");
    }

    fn get_overall_proving_time(&self, ask_id: &U256) -> Option<U256> {
        use crate::schema::timing_operations::dsl::*;
        timing_operations
            .filter(ask_id.eq(ask_id))
            .select(proof_cycle_completed_on)
            .first::<Option<U256>>(&self.conn)
            .ok()
            .flatten()
    }
}
