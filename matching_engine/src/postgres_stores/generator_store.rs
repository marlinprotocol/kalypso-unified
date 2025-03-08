use diesel::Insertable;
use crate::generator_lib;
use crate::generator_lib::delegation::{Operation, Source};
use crate::generator_lib::generator_query::GeneratorQueryResult;
use crate::generator_lib::generator_state::GeneratorState;
use crate::generator_lib::generator_store::{Generator, GeneratorInfoPerMarket};
use crate::generator_lib::points::get_points;
use crate::schema::{
    generators, generator_markets, token_trackers, slashing_records, delegations, withdrawal_requests,
};
use crate::utility::TokenTracker;
use diesel::prelude::*;
use diesel::result::Error;

use ethers::core::types::{Address, U256, U64, Bytes};
use std::time::SystemTime;
use tokio::sync::RwLockReadGuard;

use crate::postgres_stores::models::*;

use crate::generator_lib::traits::{
    GeneratorRegistration, GeneratorStakeComputeManagement, GeneratorMarketManagement,
    GeneratorSlashingManagement, GeneratorLockManagement, GeneratorAvailability, GeneratorQuery,
    GeneratorFilter, GeneratorKeyStoreFilterInterfaceTrait, GeneratorEarningsAndSlashing,
    WithdrawalManagement, GeneratorMetadata, JobMissedCounter, GeneratorAdditionalQuery,
};


/// -------------------------
/// Generator Registration
/// -------------------------
impl GeneratorRegistration for GeneratorDatabase {
    fn register_generator(&mut self, generator: Generator) {
        // Convert the domain generator to an insertable record.
        let new_gen = generator.into();
        let conn = &mut self.pool.get().expect("DB connection error");
        diesel::insert_into(generators::table)
            .values(&new_gen)
            .execute(conn)
            .expect("Error inserting generator");
    }

    fn register_generator_in_market(&mut self, generator_market: GeneratorInfoPerMarket) {
        let new_market = generator_market.into();
        let conn = &mut self.pool.get().expect("DB connection error");
        diesel::insert_into(generator_markets::table)
            .values(&new_market)
            .execute(conn)
            .expect("Error inserting generator market");
    }

    fn remove_by_address_and_market(&mut self, address: &Address, market_id: &U256) {
        let conn = &mut self.pool.get().expect("DB connection error");
        // Convert address and market_id to strings (per your mapping).
        let addr_str = format!("{:?}", address);
        let market_str = market_id.to_string();
        diesel::delete(
            generator_markets::table
                .filter(generator_markets::generator_address.eq(addr_str))
                .filter(generator_markets::market_id.eq(market_str)),
        )
        .execute(conn)
        .expect("Error deleting generator market");
    }

    fn remove_by_address(&mut self, address: &Address) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = format!("{:?}", address);
        // Remove from generators table.
        diesel::delete(generators::table.filter(generators::address.eq(addr_str.clone())))
            .execute(conn)
            .expect("Error deleting generator");
        // Cascade-delete related records (markets, etc.) as needed.
        diesel::delete(generator_markets::table.filter(generator_markets::generator_address.eq(addr_str)))
            .execute(conn)
            .expect("Error deleting generator markets");
    }

    fn get_by_address(&self, address: &Address) -> Option<Generator> {
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = format!("{:?}", address);
        let rec = generators::table
            .filter(generators::address.eq(addr_str))
            .first::<GeneratorRecord>(conn)
            .ok()?;
        Some(rec.into())
    }

    fn is_active(&self, generator_address: &Address) -> bool {
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = format!("{:?}", generator_address);
        generators::table
            .filter(generators::address.eq(addr_str))
            .select(generators::active)
            .first::<bool>(conn)
            .unwrap_or(false)
    }
}

/// -------------------------
/// Generator Stake & Compute Management
/// -------------------------
impl GeneratorStakeComputeManagement for GeneratorDatabase {
    fn add_extra_stake(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        amount: &U256,
        block_number: U64,
        transaction_index: U64,
        log_index: U256, // not stored in delegations table per schema, but available if needed
        tx: String,      // same as above
        source: Source,
    ) {
        // Convert addresses and amount to strings.
        let gen_addr_str = format!("{:?}", generator_address);
        let token_addr_str = format!("{:?}", token_address);
        let amount_str = amount.to_string();
        let conn = &mut self.pool.get().expect("DB connection error");
        // Run all operations in a single transaction.
        
        // 1. Update the generator's token tracker.
        // Determine which column to update based on the source.
        let (current_tracker_column, update_column) = match source {
            Source::Native => (generators::total_native_stake, generators::total_native_stake),
            Source::Symbiotic => (generators::total_symbiotic_stake, generators::total_symbiotic_stake),
        };

        // Fetch the current token tracker from the generators table.
        let current_tracker: String = generators::table
            .filter(generators::address.eq(&gen_addr_str))
            .select(current_tracker_column)
            .first(conn)?;

        // Use a helper function to update the tracker (e.g., updating a JSON map).
        // let new_tracker = update_token_tracker(&current_tracker, &token_addr_str, amount)
        //     .expect("Failed to update token tracker");

        // check token additions and subs

        // Update the corresponding column in the generators table.
        // diesel::update(generators::table.filter(generators::address.eq(&gen_addr_str)))
        //     .set(update_column.eq(new_tracker))
        //     .execute(conn)?;

            // 2. Insert a new delegation record.
            // Prepare a new delegation record. This struct must correspond to the `delegations` table.
        let new_delegation = DelegationRecord {
            generator_address: gen_addr_str.clone(),
            delegated_address: token_addr_str.clone(),
            delegated_amount: amount_str,
            source: source.to_string(),
            operation: Operation::Delegate.to_string(),
            block_number: block_number.to_string(),
            transaction_index: transaction_index.to_string(),
            // If your delegations table/schema supports log_index and tx, add them here.
        };

        diesel::insert_into(delegations::table)
            .values(&new_delegation)
            .execute(conn)?;


        

    }
}

    fn update_intended_stake_util(&mut self, address: &Address, new_stake_util: U256) {
        // Update the 'intended_stake_util' column in the generators table.
        unimplemented!("Update intended stake util");
    }

    fn remove_stake(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        amount: &U256,
        block_number: U64,
        transaction_index: U64,
        log_index: U256,
        tx: String,
        operation: Operation,
        source: Source,
    ) {
        // Update token tracker by reducing the stake.
        unimplemented!("Remove stake from the DB");
    }

    fn update_reward_address(&mut self, address: &Address, new_reward_address: Address) {

        // Convert addresses to the string representations stored in the DB.
        let addr_str = format!("{:?}", generator_addr);
        let new_reward_addr_str = format!("{:?}", new_reward_addr);

        // Update the reward_address field in the generators table.
        diesel::update(generators.filter(address.eq(addr_str)))
            .set(reward_address.eq(new_reward_addr_str))
            .execute(&self.conn)
            .expect("Failed to update reward address");

    }

    fn add_extra_compute(&mut self, generator_addr: &Address, extra_compute: U256) {
        // Convert the generator address to the string representation used in the DB.
        let addr_str = format!("{:?}", generator_addr);

        self.conn.transaction::<(), Error, _>(|| {
            // Fetch the current declared_compute value (stored as text).
            let current_declared_compute: String = generators
                .filter(address.eq(&addr_str))
                .select(declared_compute)
                .first(&self.conn)?;

            // Parse the current value to U256. If parsing fails, default to zero.
            let current_compute = U256::from_dec_str(&current_declared_compute)
                .unwrap_or_else(|_| U256::zero());

            // Add the extra compute.
            let new_compute = current_compute + extra_compute;

            // Update the generator record with the new value (as a string).
            diesel::update(generators.filter(address.eq(&addr_str)))
                .set(declared_compute.eq(new_compute.to_string()))
                .execute(&self.conn)?;

            Ok(())
        }).expect("Failed to update extra compute");
    }

    fn update_intended_compute_util(&mut self, generator_addr: &Address, new_compute_util: U256) {
        // Convert the generator address to the string representation stored in the DB.
        let addr_str = format!("{:?}", generator_addr);

        // Convert U256 to string since the DB column is Text.
        let new_compute_util_str = new_compute_util.to_string();

        // Update the intended_compute_util field in the generators table.
        diesel::update(generators.filter(address.eq(addr_str)))
            .set(intended_compute_util.eq(new_compute_util_str))
            .execute(&self.conn)
            .expect("Failed to update intended compute util");
    }

    fn remove_compute(&mut self, generator_addr: &Address, compute: U256) {
        // Convert the generator address to the string representation used in the DB.
        let addr_str = format!("{:?}", generator_addr);

        self.conn.transaction::<(), Error, _>(|| {
            // Fetch the current declared_compute value from the DB (stored as text).
            let current_compute_str: String = generators
                .filter(address.eq(&addr_str))
                .select(declared_compute)
                .first(&self.conn)?;

            // Parse the string into a U256
            let current_compute = U256::from_dec_str(&current_compute_str)
                .unwrap();

            // Subtract the specified compute.
            let new_compute = current_compute - compute;

            // Update the declared_compute field with the new value.
            diesel::update(generators.filter(address.eq(&addr_str)))
                .set(declared_compute.eq(new_compute.to_string()))
                .execute(&self.conn)?;

            Ok(())
        }).expect("Failed to update declared_compute");
    }


/// -------------------------
/// Generator Market Management
/// -------------------------
impl GeneratorMarketManagement for GeneratorDatabase {
    fn update_state(&mut self, generator_addr: &Address, market: &U256, new_state: GeneratorState) {
        // Convert the address and market_id into the string representations as stored in the DB.
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = format!("{:?}", generator_addr);
        let market_id_str = market.to_string();
        let new_state_str = new_state.to_string();

        diesel::update(generator_markets::table.filter(generator_markets::generator_address.eq(&addr_str))
            .filter(generator_markets::market_id.eq(&market_id_str)))
            .set(generator_markets::state.eq(Some(new_state_str)))
            .execute(conn)
            .expect("Failed to update generator market state");
    }

    fn update_on_assigned_task(&mut self, address: &Address, market_id: &U256) {

        // Convert the address and market_id to string representations as stored in the DB.
        let conn = &mut self.pool.get().expect("DB connection error");
        let address_str = format!("{:?}", address);
        let market_id_str = GeneratorMarketRecord::market.to_string();

        // First, fetch the current record for this generator/market.
        let record: Option<GeneratorMarketRecord> = generator_markets::table
            .filter(generator_markets::generator_address.eq(&address_str))
            .filter(generator_markets::market_id.eq(&market_id_str))
            .first::<GeneratorMarketRecord>(conn)
            .optional()
            .expect("Error loading generator market record");

        if let Some(rec) = record {
            // Parse the current active_requests value from a String to U256.
            let conn = &mut self.pool.get().expect("DB connection error");
            let current_active_requests = rec
                .active_requests
                .parse::<U256>()
                .unwrap_or_else(|_| U256::zero());
            let new_active_requests = current_active_requests + U256::one();
            let new_active_requests_str = new_active_requests.to_string();

            // Update the record in the database.
            diesel::update(generator_markets::table.filter(generator_markets::generator_address.eq(&address_str))
                .filter(market_id.eq(&market_id_str)))
                .set(generator_markets::active_requests.eq(new_active_requests_str))
                .execute(conn)
                .expect("Failed to update active_requests");
        }

    }

    fn update_on_submit_proof(
            &mut self,
            address: &Address,
            market: &U256,
            earning: &U256,
            block_number: &U64,
        ) {
            // Calculate kalypso points for this proof.
            let conn = &mut self.pool.get().expect("DB connection error");
            let kalypso_points_per_proof = get_points(block_number.as_u64());

            // Convert address and market into their stored string representations.
            let address_str = format!("{:?}", address);
            let market_id_str = market.to_string();

            // Execute all updates within a single transaction.
            conn.transaction::<(), Error, _>(|| {
                // --- Update the generator_markets record ---
                // Fetch the existing generator market record.
                let gen_market_opt = generator_markets::table
                    .filter(generator_markets::generator_address.eq(&address_str))
                    .filter(generator_markets::market_id.eq(&market_id_str))
                    .first::<GeneratorMarketRecord>(conn)
                    .optional()?;

                if let Some(gen_market) = gen_market_opt {
                    // Parse the text fields into U256. (Adjust parsing as needed.)
                    let current_active_requests = U256::from_dec_str(&gen_market.active_requests)
                        .unwrap_or(U256::zero());
                    let current_proofs_submitted = U256::from_dec_str(&gen_market.proofs_submitted)
                        .unwrap_or(U256::zero());
                    let current_market_earnings = U256::from_dec_str(&gen_market.earnings)
                        .unwrap_or(U256::zero());
                    let current_market_kalypso_points = U256::from_dec_str(&gen_market.kalypso_points)
                        .unwrap_or(U256::zero());

                    // Calculate new values.
                    let new_active_requests = current_active_requests - U256::one();
                    let new_proofs_submitted = current_proofs_submitted + U256::one();
                    let new_market_earnings = current_market_earnings + *earning;
                    let new_market_kalypso_points = current_market_kalypso_points + kalypso_points_per_proof;

                    // Update the generator_markets record.
                    diesel::update(generator_markets::table.filter(generator_markets::generator_address.eq(&address_str))
                        .filter(generator_markets::market_id.eq(&market_id_str)))
                        .set((
                            generator_markets::active_requests.eq(new_active_requests.to_string()),
                            generator_markets::proofs_submitted.eq(new_proofs_submitted.to_string()),
                            generator_markets::earnings.eq(new_market_earnings.to_string()),
                            generator_markets::kalypso_points.eq(new_market_kalypso_points.to_string()),
                        ))
                        .execute(conn)?;
                }

                let gen_opt = generators::table
                    .filter(generators::address.eq(&address_str))
                    .first::<GeneratorRecord>(conn)
                    .optional()?;

                if let Some(gen) = gen_opt {
                    let current_total_earnings = U256::from_dec_str(&gen.earnings)
                        .unwrap_or(U256::zero());
                    let current_total_kalypso_points = U256::from_dec_str(&gen.kalypso_points)
                        .unwrap_or(U256::zero());

                    let new_total_earnings = current_total_earnings + *earning;
                    let new_total_kalypso_points = current_total_kalypso_points + kalypso_points_per_proof;

                    diesel::update(generators::table.filter(generators::address.eq(&address_str)))
                        .set((
                            generators::earnings.eq(new_total_earnings.to_string()),
                            generators::kalypso_points.eq(new_total_kalypso_points.to_string()),
                        ))
                        .execute(conn)?;
                }

            Ok(())
        }).expect("Transaction failed updating on submit proof");

    }


    fn reduce_active_requests(&mut self, generator_addr: &Address, market: &U256) {
        // Convert address and market to the string representations stored in the DB.
        let conn = &mut self.pool.get().expect("DB connection error");
        let address_str = format!("{:?}", generator_addr);
        let market_id_str = market.to_string();

        conn.transaction::<(), Error, _>(|| {
            // Fetch the current generator market record.
            let record_opt = generator_markets::table
            .filter(generator_markets::generator_address.eq(&address_str))
            .filter(generator_markets::market_id.eq(&market_id_str))
                .first::<GeneratorMarketRecord>(conn)
                .optional()?;

            if let Some(record) = record_opt {
                // Parse current active_requests and proofs_slashed from their text representations.
                let current_active_requests = U256::from_dec_str(&record.active_requests)
                    .unwrap_or(U256::zero());
                let current_proofs_slashed = U256::from_dec_str(&record.proofs_slashed)
                    .unwrap_or(U256::zero());

                // Compute new values.
                let new_active_requests = current_active_requests - U256::one();
                let new_proofs_slashed = current_proofs_slashed + U256::one();

                // Update the record.
                diesel::update(generator_markets::table.filter(generator_markets::generator_address.eq(&address_str))
                    .filter(generator_markets::market_id.eq(&market_id_str)))
                    .set((
                        generator_markets::active_requests.eq(new_active_requests.to_string()),
                        generator_markets::proofs_slashed.eq(new_proofs_slashed.to_string()),
                    ))
                    .execute(conn)?;
            }
            Ok(())
        }).expect("Transaction failed updating reduce_active_requests");
    }

    fn pause_assignments_across_all_markets(&mut self, generator_addr: &Address) {
        // Convert the generator address to the string representation as stored in the DB.
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = format!("{:?}", generator_addr);
        let pending_state = GeneratorState::PendingConfirmation.to_string();

        // Update all rows in the generator_markets table for this generator,
        // setting their state to PendingConfirmation.
        diesel::update(generator_markets::table.filter(generator_markets::generator_address.eq(&addr_str)))
            .set(generator_markets::state.eq(Some(pending_state)))
            .execute(conn)
            .expect("Failed to update state for all markets");
    }

    fn resume_assignments_accross_all_markets(&mut self, generator_addr: &Address) {
        // Convert the generator address to the string representation stored in the DB.
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = format!("{:?}", generator_addr);
        let joined_state = GeneratorState::Joined.to_string();

        // Update all rows in the generator_markets table for this generator,
        // setting their state to "Joined"
        diesel::update(generator_markets::table.filter(generator_markets::generator_address.eq(&addr_str)))
            .set(generator_markets::state.eq(Some(joined_state)))
            .execute(conn)
            .expect("Failed to update state to Joined for all markets");
    }
}

/// -------------------------
/// Generator Slashing Management
/// -------------------------
impl GeneratorSlashingManagement for GeneratorDatabase {
    fn note_entry_slashing(
        &mut self,
        generator_address: &Address,
        ask_id: &U256,
        market_id: &U256,
        native_tokens_slashed: Vec<Address>,
        native_slashings: Vec<U256>,
        symbiotic_tokens_slashed: Vec<Address>,
        symbiotic_slashings: Vec<U256>,
        slashing_tx: String,
        price_offered: &U256,
        deadline: &U256,
        slashing_block_number: &U64,
        slashing_timestamp: &U256,
    ) {
        // Insert a new record into the slashing_records table and update token_trackers.
        unimplemented!("Record a slashing event");
    }
}

/// -------------------------
/// Generator Lock Management
/// -------------------------
impl GeneratorLockManagement for GeneratorDatabase {
    fn update_on_stake_locked(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_locked: U256,
        source: Source,
    ) {
        // Update locked stake (native or symbiotic) in the generators table.
        unimplemented!("Update stake locked");
    }

    fn update_on_stake_released(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_released: U256,
        source: Source,
    ) {
        unimplemented!("Update stake released");
    }

    fn update_on_compute_locked(&mut self, generator_addr: &Address, compute_locked: U256) {
        // Convert the generator address to the string representation used in the DB.
        let addr_str = format!("{:?}", generator_addr);

        self.conn.transaction::<(), Error, _>(|| {
            // Fetch the current compute_consumed value from the DB (stored as text).
            let current_compute_str: String = generators
                .filter(address.eq(&addr_str))
                .select(compute_consumed)
                .first(&self.conn)?;

            // Parse the current compute value; default to zero if parsing fails.
            let current_compute = U256::from_dec_str(&current_compute_str)
                .unwrap();
            // Add the new compute_locked amount.
            let new_compute = current_compute + compute_locked;

            // Update the compute_consumed field with the new value (as a string).
            diesel::update(generators.filter(address.eq(&addr_str)))
                .set(compute_consumed.eq(new_compute.to_string()))
                .execute(&self.conn)?;

            Ok(())
        })
        .expect("Failed to update compute_consumed");
    }

    fn update_on_compute_released(&mut self, generator_addr: &Address, compute_released: U256) {
        // Convert the generator address to the string representation used in the DB.
        let addr_str = format!("{:?}", generator_addr);

        self.conn.transaction::<(), Error, _>(|| {
            // Fetch the current compute_consumed value from the DB (stored as text).
            let current_compute_str: String = generators
                .filter(address.eq(&addr_str))
                .select(compute_consumed)
                .first(&self.conn)?;

            // Parse the current compute value; default to zero if parsing fails.
            let current_compute = U256::from_dec_str(&current_compute_str)
                .unwrap();
            // Add the new compute_locked amount.
            let new_compute = current_compute - compute_locked;

            // Update the compute_consumed field with the new value (as a string).
            diesel::update(generators.filter(address.eq(&addr_str)))
                .set(compute_consumed.eq(new_compute.to_string()))
                .execute(&self.conn)?;

            Ok(())
        })
        .expect("Failed to update compute_consumed");
    }
}

/// -------------------------
/// Generator Availability
/// -------------------------
impl GeneratorAvailability for GeneratorDatabase {
    fn get_available_compute(&self, address: Address) -> Option<U256> {
        // Calculate available compute (e.g., declared minus locked).
        unimplemented!("Compute available compute");
    }

    fn get_available_native_stake(&self, generator_address: &Address) -> Option<TokenTracker> {
        // Retrieve and compute available native stake.
        unimplemented!("Compute available native stake");
    }

    fn get_available_symbiotic_stake(&self, generator_address: &Address) -> Option<TokenTracker> {
        unimplemented!("Compute available symbiotic stake");
    }

    // UI
    fn get_native_stake_locked(&self, generator_address: &Address) -> Option<TokenTracker> {
        unimplemented!("Fetch native stake locked");
    }

    // UI
    fn get_symbiotic_stake_locked(&self, generator_address: &Address) -> Option<TokenTracker> {
        unimplemented!("Fetch symbiotic stake locked");
    }

    // UI
    fn get_all_by_market_id(&self, market_id: &U256) -> Vec<GeneratorInfoPerMarket> {
        // Query generator_markets table for the specified market.
        unimplemented!("Fetch all generators by market id");
    }
}

/// -------------------------
/// Generator Query
/// -------------------------
impl GeneratorQuery for GeneratorDatabase {
    // not used
    fn query(&self) -> GeneratorQueryResult {
        // Return a result representing all generators.
        unimplemented!("Query all generators");
    }

    fn query_by_market_id_and_only_active(&self, market_id: &U256) -> GeneratorQueryResult {
        unimplemented!("Query active generators by market id");
    }

    // not used
    fn query_by_states(&self, states: Vec<GeneratorState>) -> GeneratorQueryResult {
        unimplemented!("Query generators by state");
    }

    // not used
    fn query_by_address(&self, address: Address) -> GeneratorQueryResult {
        unimplemented!("Query generator by address");
    }
}

/// -------------------------
/// Generator Filter
/// -------------------------
impl GeneratorFilter for GeneratorDatabase {
    fn filter_by_has_idle_compute(
        &self,
        generator_query: GeneratorQueryResult,
    ) -> GeneratorQueryResult {
        // Filter out generators that have no idle compute.
        unimplemented!("Filter by idle compute");
    }

    fn filter_by_available_native_stake(
        &self,
        generator_query: GeneratorQueryResult,
        min_stake: Vec<(Address, U256)>,
    ) -> GeneratorQueryResult {
        unimplemented!("Filter by available native stake");
    }

    fn filter_by_available_symbiotic_stake(
        &self,
        generator_query: GeneratorQueryResult,
        min_stake: Vec<(Address, U256)>,
    ) -> GeneratorQueryResult {
        unimplemented!("Filter by available symbiotic stake");
    }
}

/// -------------------------
/// Generator KeyStore Filter Interface
/// -------------------------
impl<KS: KeyStoreOperations> GeneratorKeyStoreFilterInterfaceTrait<KS> for GeneratorDatabase {
    fn filter_by_has_private_inputs_support(
        &self,
        generator_query: GeneratorQueryResult,
        key_store: RwLockReadGuard<'_, KS>,
    ) -> GeneratorQueryResult {
        // Use the provided key store to filter generators that support private inputs.
        unimplemented!("Filter by private inputs support");
    }
}

/// -------------------------
/// Generator Earnings and Slashing
/// -------------------------
impl GeneratorEarningsAndSlashing for GeneratorDatabase {
    // all UI routes
    fn get_total_earning(&self, address: &Address) -> Option<U256> {
        // Query the generators table for total earnings.
        unimplemented!("Get total earnings");
    }

    fn get_earning_per_market(&self, address: &Address, market_id: &U256) -> Option<U256> {
        // Query generator_markets table for per-market earnings.
        unimplemented!("Get per-market earnings");
    }

    fn get_kalypso_points(&self, address: &Address) -> Option<U256> {
        unimplemented!("Get total kalypso points");
    }

    fn get_kalypso_points_per_market(&self, address: &Address, market_id: &U256) -> Option<U256> {
        unimplemented!("Get kalypso points per market");
    }

    fn get_total_slashing(&self, generator_address: &Address) -> Option<TokenTracker> {
        // Aggregate generator-wide slashings from token_trackers table.
        unimplemented!("Get total slashing");
    }

    fn get_slashing_per_generator_per_market(
        &self,
        address: &Address,
        market_id: &U256,
    ) -> Option<TokenTracker> {
        // Query token_trackers table for slashings specific to a market.
        unimplemented!("Get slashing per market");
    }

    fn get_slashing_records(&self, address: &Address) -> Vec<generator_lib::SlashingRecord> {
        // Query slashing_records table for records related to the generator.
        unimplemented!("Get slashing records");
    }
}

/// -------------------------
/// Withdrawal Management
/// -------------------------
impl WithdrawalManagement for GeneratorDatabase {
    fn insert_withdrawal_request(
        &mut self,
        operator_address: &Address,
        withdrawal_request: WithdrawlRequest,
    ) {
        // Convert and insert into withdrawal_requests table.
        unimplemented!("Insert withdrawal request");
    }

    // ui route
    fn get_withdrawl_requests(&self, operator_address: &Address) -> Vec<WithdrawlRequest> {
        unimplemented!("Get withdrawal requests");
    }

    fn remove_withdrawal_request(
        &mut self,
        operator_address: &Address,
        withdrawal_request: WithdrawlRequest,
    ) -> bool {
        unimplemented!("Remove withdrawal request")
    }
}

/// -------------------------
/// Generator Metadata
/// -------------------------
impl GeneratorMetadata for GeneratorDatabase {
    fn update_generator_metadata(&mut self, generator_addr: Address, generator_meta_data: Bytes) {
        // Convert the generator address to the string representation stored in the DB.
        let addr_str = format!("{:?}", generator_addr);

        diesel::update(generators::table.filter(generators::address.eq(addr_str)))
            .set(generators::generator_data.eq(generator_meta_data.to_vec()))
            .execute(conn)
            .expect("Failed to update generator metadata");
    }
}

/// -------------------------
/// Job Missed Counter
/// -------------------------
impl JobMissedCounter for GeneratorDatabase {
    fn count_job_missed_by_generator(
        &mut self,
        generator_addr: Address,
        _timestamp: SystemTime, // timestamp is not stored in this DB version
    ) {
        // Convert the generator address to the string representation stored in the DB.
        let addr_str = format!("{:?}", generator_addr);

        // Increment the jobs_missed_counter by one.
        // Diesel supports arithmetic expressions on columns if the types implement the operators.
        diesel::update(GeneratorRecord::table.filter(GeneratorRecord::address.eq(&addr_str)))
            .set(generators::jobs_missed_counter.eq(jobs_missed_counter + 1))
            .execute(conn)
            .expect("Failed to update jobs missed counter");
    }

    fn get_job_missed_count(&self, generator_addr: &Address) -> usize {
        let addr_str = format!("{:?}", generator_addr);
        // Query the current jobs_missed_counter value.
        GeneratorRecord::table
            .filter(GeneratorRecord::address.eq(&addr_str))
            .select(GeneratorRecord::jobs_missed_counter)
            .first::<i32>(conn)
            .expect("Failed to fetch jobs missed counter") as usize
    }
}

/// -------------------------
/// Generator Additional Query
/// -------------------------
impl GeneratorAdditionalQuery for GeneratorDatabase {
    fn all_generators_address(&self) -> Vec<Address> {
        // Query the generators table and return all addresses.
        unimplemented!("Return all generator addresses");
    }

    // UI
    fn get_delegations(
        &self,
        generator_address: &Address,
        operations: Vec<Operation>,
        skip: Option<usize>,
        count: Option<usize>,
    ) -> Vec<Delegation> {
        // Query the delegations table filtering by generator_address and operations.
        unimplemented!("Return delegations");
    }

    //UI
    fn total_native_stake_accross_all_generators(&self) -> TokenTracker {
        // Aggregate native stake from all generators.
        unimplemented!("Aggregate total native stake");
    }

    // UI
    fn total_symbiotic_stake_across_all_generators(&self) -> TokenTracker {
        unimplemented!("Aggregate total symbiotic stake");
    }

    fn get_by_address_and_market(
        &self,
        address: &Address,
        market: &U256,
    ) -> Option<GeneratorInfoPerMarket> {
        // Convert the address and market_id into the string representations as stored in the DB.
        let address_str = format!("{:?}", address);
        let market_id_str = market.to_string();

        // Query the generator_markets table for a record matching the address and market_id.
        let result = generator_markets
            .filter(generator_address.eq(address_str))
            .filter(market_id.eq(market_id_str))
            .first::<GeneratorMarketRecord>(&self.conn)
            .optional()  // Returns Ok(Some(record)) if found, Ok(None) if not found.
            .expect("Error fetching generator market record");

        // Convert the record to GeneratorInfoPerMarket if it exists.
        result.map(GeneratorInfoPerMarket::from)
    }

    // UI
    fn get_all_markets_of_generator(&self, address: &Address) -> Vec<GeneratorInfoPerMarket> {
        // Query the generator_markets table filtering by generator_address.
        unimplemented!("Get all markets of generator");
    }
}