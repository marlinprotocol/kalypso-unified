use crate::generator_lib;
use crate::generator_lib::delegation::{Delegation, Operation, Source};
use crate::generator_lib::generator_query::GeneratorQueryResult;
use crate::generator_lib::generator_state::GeneratorState;
use crate::generator_lib::generator_store::{Generator, GeneratorInfoPerMarket};
use crate::generator_lib::key_store::KeyStoreOperations;
use crate::generator_lib::points::get_points;
use crate::generator_lib::withdrawal_request::WithdrawlRequest;
use crate::schema::{
    delegations, generator_markets, generators, slashing_records, token_trackers,
    withdrawal_requests,
};
use crate::utility::{AddressTokenPair, TokenTracker};
use diesel::prelude::*;
use diesel::result::Error;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use tokio::sync::RwLockReadGuard;

use ethers::core::types::{Address, Bytes, U256, U64};

use crate::postgres_stores::models::*;
// use crate::postgres_stores::generator_query2::GeneratorQueryResult2;

use crate::generator_lib::traits::{
    GeneratorAdditionalQuery, GeneratorAvailability, GeneratorEarningsAndSlashing, GeneratorFilter,
    GeneratorKeyStoreFilterInterfaceTrait, GeneratorLockManagement, GeneratorMarketManagement,
    GeneratorMetadata, GeneratorQuery, GeneratorRegistration, GeneratorSlashingManagement,
    GeneratorStakeComputeManagement, JobMissedCounter, WithdrawalManagement,
};

/// -------------------------
/// Generator Registration
/// -------------------------
impl GeneratorRegistration for GeneratorDatabase {
    fn register_generator(&mut self, generator: Generator) {
        // Convert the domain generator to an insertable record.
        let new_gen: GeneratorRecord = generator.into();
        let conn = &mut self.pool.get().expect("DB connection error");
        diesel::insert_into(generators::table)
            .values(&new_gen)
            .execute(conn)
            .expect("Error inserting generator");
    }

    fn register_generator_in_market(&mut self, generator_market: GeneratorInfoPerMarket) {
        let new_market: GeneratorMarketRecord = generator_market.into();
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
        diesel::delete(
            generator_markets::table.filter(generator_markets::generator_address.eq(addr_str)),
        )
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
        log_index: U256,
        tx: String,
        source: Source,
    ) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let gen_addr_str = generator_address.to_string();

        // Update the extra stake for the generator.
        // Depending on source, update the appropriate token tracker.
        match source {
            Source::Native => {
                // Fetch the current total_native_stake.
                let rec: GeneratorRecord = generators::table
                    .filter(generators::address.eq(&gen_addr_str))
                    .first(conn)
                    .expect("Error fetching generator record");
                // Deserialize the stored tracker.
                let mut tracker: TokenTracker = serde_json::from_str(&rec.total_native_stake)
                    .expect("Error parsing total_native_stake");
                // Add the extra stake.
                tracker.add_token(token_address, amount);
                // Serialize the updated tracker.
                let updated_tracker = serde_json::to_string(&tracker)
                    .expect("Error serializing updated native stake");
                // Update the generator record.
                diesel::update(generators::table.filter(generators::address.eq(&gen_addr_str)))
                    .set(generators::total_native_stake.eq(updated_tracker))
                    .execute(conn)
                    .expect("Error updating native stake");
            }
            Source::Symbiotic => {
                let rec: GeneratorRecord = generators::table
                    .filter(generators::address.eq(&gen_addr_str))
                    .first(conn)
                    .expect("Error fetching generator record");
                let mut tracker: TokenTracker = serde_json::from_str(&rec.total_symbiotic_stake)
                    .expect("Error parsing total_symbiotic_stake");
                tracker.add_token(token_address, amount);
                let updated_tracker = serde_json::to_string(&tracker)
                    .expect("Error serializing updated symbiotic stake");
                diesel::update(generators::table.filter(generators::address.eq(&gen_addr_str)))
                    .set(generators::total_symbiotic_stake.eq(updated_tracker))
                    .execute(conn)
                    .expect("Error updating symbiotic stake");
            }
        }

        // Insert the delegation record.
        let new_delegation = DelegationRecord {
            generator_address: gen_addr_str.clone(),
            delegated_address: token_address.to_string(),
            delegated_amount: amount.to_string(),
            source: match source {
                Source::Native => "Native".to_string(),
                Source::Symbiotic => "Symbiotic".to_string(),
            },
            operation: Operation::Delegate.to_string(),
            block_number: block_number.to_string(),
            transaction_index: transaction_index.to_string(),
            log_index: log_index.to_string(),
            tx: tx.clone(),
        };

        diesel::insert_into(delegations::table)
            .values(&new_delegation)
            .execute(conn)
            .expect("Error inserting delegation record");
    }

    fn update_intended_stake_util(&mut self, address: &Address, new_stake_util: U256) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = address.to_string();

        diesel::update(generators::table.filter(generators::address.eq(&addr_str)))
            .set(generators::intended_stake_util.eq(new_stake_util.to_string()))
            .execute(conn)
            .expect("Error updating intended stake util");
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
        let conn = &mut self.pool.get().expect("DB connection error");
        let gen_addr_str = generator_address.to_string();

        // Update the token tracker by subtracting the given amount.
        match source {
            Source::Native => {
                let rec: GeneratorRecord = generators::table
                    .filter(generators::address.eq(&gen_addr_str))
                    .first(conn)
                    .expect("Error fetching generator record");
                let mut tracker: TokenTracker = serde_json::from_str(&rec.total_native_stake)
                    .expect("Error parsing total_native_stake");
                tracker
                    .sub_token(token_address, amount)
                    .expect("Error subtracting token from native stake");
                let updated_tracker = serde_json::to_string(&tracker)
                    .expect("Error serializing updated native stake");
                diesel::update(generators::table.filter(generators::address.eq(&gen_addr_str)))
                    .set(generators::total_native_stake.eq(updated_tracker))
                    .execute(conn)
                    .expect("Error updating native stake");
            }
            Source::Symbiotic => {
                let rec: GeneratorRecord = generators::table
                    .filter(generators::address.eq(&gen_addr_str))
                    .first(conn)
                    .expect("Error fetching generator record");
                let mut tracker: TokenTracker = serde_json::from_str(&rec.total_symbiotic_stake)
                    .expect("Error parsing total_symbiotic_stake");
                tracker
                    .sub_token(token_address, amount)
                    .expect("Error subtracting token from symbiotic stake");
                let updated_tracker = serde_json::to_string(&tracker)
                    .expect("Error serializing updated symbiotic stake");
                diesel::update(generators::table.filter(generators::address.eq(&gen_addr_str)))
                    .set(generators::total_symbiotic_stake.eq(updated_tracker))
                    .execute(conn)
                    .expect("Error updating symbiotic stake");
            }
        }

        // Insert the delegation record for the removal.
        let new_delegation = DelegationRecord {
            generator_address: gen_addr_str.clone(),
            delegated_address: token_address.to_string(),
            delegated_amount: amount.to_string(),
            source: match source {
                Source::Native => "Native".to_string(),
                Source::Symbiotic => "Symbiotic".to_string(),
            },
            operation: operation.to_string(), // Assuming Operation implements ToString.
            block_number: block_number.to_string(),
            transaction_index: transaction_index.to_string(),
            log_index: log_index.to_string(),
            tx: tx.clone(),
        };

        diesel::insert_into(delegations::table)
            .values(&new_delegation)
            .execute(conn)
            .expect("Error inserting delegation record");
    }

    fn update_reward_address(&mut self, address: &Address, new_reward_address: Address) {
        // Convert addresses to the string representations stored in the DB.
        let addr_str = format!("{:?}", address);
        let new_reward_addr_str = format!("{:?}", new_reward_address);
        let conn = &mut self.pool.get().expect("DB connection error");

        // Update the reward_address field in the generators table.
        diesel::update(generators::table.filter(generators::address.eq(addr_str)))
            .set(generators::reward_address.eq(new_reward_addr_str))
            .execute(conn)
            .expect("Failed to update reward address");
    }

    fn add_extra_compute(&mut self, generator_addr: &Address, extra_compute: U256) {
        // Convert the generator address to the string representation used in the DB.
        let addr_str = format!("{:?}", generator_addr);
        let conn = &mut self.pool.get().expect("DB connection error");
        conn.transaction::<(), Error, _>(|conn| {
            // Fetch the current declared_compute value (stored as text).
            let current_declared_compute: String = generators::table
                .filter(generators::address.eq(&addr_str))
                .select(generators::declared_compute)
                .first(conn)?;

            // Parse the current value to U256. If parsing fails, default to zero.
            let current_compute =
                U256::from_dec_str(&current_declared_compute).unwrap_or_else(|_| U256::zero());

            // Add the extra compute.
            let new_compute = current_compute + extra_compute;

            // Update the generator record with the new value (as a string).
            diesel::update(generators::table.filter(generators::address.eq(&addr_str)))
                .set(generators::declared_compute.eq(new_compute.to_string()))
                .execute(conn)?;

            Ok(())
        })
        .expect("Failed to update extra compute");
    }

    fn update_intended_compute_util(&mut self, generator_addr: &Address, new_compute_util: U256) {
        // Convert the generator address to the string representation stored in the DB.
        let addr_str = format!("{:?}", generator_addr);
        let conn = &mut self.pool.get().expect("DB connection error");
        // Convert U256 to string since the DB column is Text.
        let new_compute_util_str = new_compute_util.to_string();

        // Update the intended_compute_util field in the generators table.
        diesel::update(generators::table.filter(generators::address.eq(addr_str)))
            .set(generators::intended_compute_util.eq(new_compute_util_str))
            .execute(conn)
            .expect("Failed to update intended compute util");
    }

    fn remove_compute(&mut self, generator_addr: &Address, compute: U256) {
        // Convert the generator address to the string representation used in the DB.
        let addr_str = format!("{:?}", generator_addr);
        let conn = &mut self.pool.get().expect("DB connection error");

        conn.transaction::<(), Error, _>(|conn| {
            // Fetch the current declared_compute value from the DB (stored as text).
            let current_compute_str: String = generators::table
                .filter(generators::address.eq(&addr_str))
                .select(generators::declared_compute)
                .first(conn)?;

            // Parse the string into a U256
            let current_compute = U256::from_dec_str(&current_compute_str).unwrap();

            // Subtract the specified compute.
            let new_compute = current_compute - compute;

            // Update the declared_compute field with the new value.
            diesel::update(generators::table.filter(generators::address.eq(&addr_str)))
                .set(generators::declared_compute.eq(new_compute.to_string()))
                .execute(conn)?;

            Ok(())
        })
        .expect("Failed to update declared_compute");
    }
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

        diesel::update(
            generator_markets::table
                .filter(generator_markets::generator_address.eq(&addr_str))
                .filter(generator_markets::market_id.eq(&market_id_str)),
        )
        .set(generator_markets::state.eq(Some(new_state_str)))
        .execute(conn)
        .expect("Failed to update generator market state");
    }

    fn update_on_assigned_task(&mut self, address: &Address, market_id: &U256) {
        // Convert the address and market_id to string representations as stored in the DB.
        let conn = &mut self.pool.get().expect("DB connection error");
        let address_str = format!("{:?}", address);
        let market_id_str = market_id.to_string();

        // First, fetch the current record for this generator/market.
        let record: Option<GeneratorMarketRecord> = generator_markets::table
            .filter(generator_markets::generator_address.eq(&address_str))
            .filter(generator_markets::market_id.eq(&market_id_str))
            .select(GeneratorMarketRecord::as_select())
            .first::<GeneratorMarketRecord>(conn)
            .optional()
            .expect("Error loading generator market record");

        if let Some(rec) = record {
            // Parse the current active_requests value from a String to U256.
            let current_active_requests = rec
                .active_requests
                .parse::<U256>()
                .unwrap_or_else(|_| U256::zero());
            let new_active_requests = current_active_requests + U256::one();
            let new_active_requests_str = new_active_requests.to_string();

            // Update the record in the database.
            diesel::update(
                generator_markets::table
                    .filter(generator_markets::generator_address.eq(&address_str))
                    .filter(generator_markets::market_id.eq(&market_id_str)),
            )
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
        conn.transaction::<(), Error, _>(|conn| {
            // --- Update the generator_markets record ---
            // Fetch the existing generator market record.
            let gen_market_opt = generator_markets::table
                .filter(generator_markets::generator_address.eq(&address_str))
                .filter(generator_markets::market_id.eq(&market_id_str))
                .select(GeneratorMarketRecord::as_select())
                .first::<GeneratorMarketRecord>(conn)
                .optional()?;

            if let Some(gen_market) = gen_market_opt {
                // Parse the text fields into U256. (Adjust parsing as needed.)
                let current_active_requests =
                    U256::from_dec_str(&gen_market.active_requests).unwrap_or(U256::zero());
                let current_proofs_submitted =
                    U256::from_dec_str(&gen_market.proofs_submitted).unwrap_or(U256::zero());
                let current_market_earnings =
                    U256::from_dec_str(&gen_market.earnings).unwrap_or(U256::zero());
                let current_market_kalypso_points =
                    U256::from_dec_str(&gen_market.kalypso_points).unwrap_or(U256::zero());

                // Calculate new values.
                let new_active_requests = current_active_requests - U256::one();
                let new_proofs_submitted = current_proofs_submitted + U256::one();
                let new_market_earnings = current_market_earnings + *earning;
                let new_market_kalypso_points =
                    current_market_kalypso_points + kalypso_points_per_proof;

                // Update the generator_markets record.
                diesel::update(
                    generator_markets::table
                        .filter(generator_markets::generator_address.eq(&address_str))
                        .filter(generator_markets::market_id.eq(&market_id_str)),
                )
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
                let current_total_earnings =
                    U256::from_dec_str(&gen.earnings).unwrap_or(U256::zero());
                let current_total_kalypso_points =
                    U256::from_dec_str(&gen.kalypso_points).unwrap_or(U256::zero());

                let new_total_earnings = current_total_earnings + *earning;
                let new_total_kalypso_points =
                    current_total_kalypso_points + kalypso_points_per_proof;

                diesel::update(generators::table.filter(generators::address.eq(&address_str)))
                    .set((
                        generators::earnings.eq(new_total_earnings.to_string()),
                        generators::kalypso_points.eq(new_total_kalypso_points.to_string()),
                    ))
                    .execute(conn)?;
            }

            Ok(())
        })
        .expect("Transaction failed updating on submit proof");
    }

    fn reduce_active_requests(&mut self, generator_addr: &Address, market: &U256) {
        // Convert address and market to the string representations stored in the DB.
        let conn = &mut self.pool.get().expect("DB connection error");
        let address_str = format!("{:?}", generator_addr);
        let market_id_str = market.to_string();

        conn.transaction::<(), Error, _>(|conn| {
            // Fetch the current generator market record.
            let record_opt: Option<GeneratorMarketRecord> = generator_markets::table
                .filter(generator_markets::generator_address.eq(&address_str))
                .filter(generator_markets::market_id.eq(&market_id_str))
                .select(GeneratorMarketRecord::as_select())
                .first::<GeneratorMarketRecord>(conn)
                .optional()?;

            if let Some(record) = record_opt {
                // Parse current active_requests and proofs_slashed from their text representations.
                let current_active_requests =
                    U256::from_dec_str(&record.active_requests).unwrap_or(U256::zero());
                let current_proofs_slashed =
                    U256::from_dec_str(&record.proofs_slashed).unwrap_or(U256::zero());

                // Compute new values.
                let new_active_requests = current_active_requests - U256::one();
                let new_proofs_slashed = current_proofs_slashed + U256::one();

                // Update the record.
                diesel::update(
                    generator_markets::table
                        .filter(generator_markets::generator_address.eq(&address_str))
                        .filter(generator_markets::market_id.eq(&market_id_str)),
                )
                .set((
                    generator_markets::active_requests.eq(new_active_requests.to_string()),
                    generator_markets::proofs_slashed.eq(new_proofs_slashed.to_string()),
                ))
                .execute(conn)?;
            }
            Ok(())
        })
        .expect("Transaction failed updating reduce_active_requests");
    }

    fn pause_assignments_across_all_markets(&mut self, generator_addr: &Address) {
        // Convert the generator address to the string representation as stored in the DB.
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = format!("{:?}", generator_addr);
        let pending_state = GeneratorState::PendingConfirmation.to_string();

        // Update all rows in the generator_markets table for this generator,
        // setting their state to PendingConfirmation.
        diesel::update(
            generator_markets::table.filter(generator_markets::generator_address.eq(&addr_str)),
        )
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
        diesel::update(
            generator_markets::table.filter(generator_markets::generator_address.eq(&addr_str)),
        )
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
        // Get a DB connection once.
        let conn = &mut self.pool.get().expect("DB connection error");

        // Helper: Convert generator and market identifiers to strings.
        let gen_addr_str = generator_address.to_string();
        let market_str = market_id.to_string();

        // Process Native Slashings
        for (token, slashing) in native_tokens_slashed.iter().zip(native_slashings.iter()) {
            // --- Update token_trackers (aggregated slashings per (generator, market, token)) ---
            // Try to fetch an existing record.
            let existing: Option<TokenTrackerRecord> = token_trackers::table
                .filter(token_trackers::generator_address.eq(&gen_addr_str))
                .filter(token_trackers::market_id.eq(&market_str))
                .select(TokenTrackerRecord::as_select())
                .first(conn)
                .optional()
                .expect("Error querying token tracker");

            let new_tracker: TokenTracker = if let Some(rec) = existing {
                let mut current: TokenTracker = serde_json::from_str(&rec.token_tracker)
                    .expect("Failed to parse total_native_stake");
                current.add_token(token, slashing);
                current
            } else {
                let mut current = TokenTracker::new(); // Create a new TokenTracker if none exists
                current.add_token(token, slashing); // Add the slashing amount
                current
            };

            // Upsert the token tracker record.
            diesel::insert_into(token_trackers::table)
                .values((
                    token_trackers::generator_address.eq(&gen_addr_str),
                    token_trackers::market_id.eq(&market_str),
                    token_trackers::token_tracker.eq(serde_json::to_string(&new_tracker)
                        .expect("Error serializing token tracker")),
                ))
                .on_conflict((token_trackers::generator_address, token_trackers::market_id))
                .do_update()
                .set(token_trackers::token_tracker.eq(
                    serde_json::to_string(&new_tracker).expect("Error serializing token tracker"),
                ))
                .execute(conn)
                .expect("Error upserting token tracker (native)");

            // --- Insert slashing record for Native ---
            let slashing_penalty = serde_json::to_string(&(token.clone(), slashing.to_string()))
                .expect("Error serializing slashing penalty");
            let new_slashing_record = DBSlashingRecord {
                generator_address: gen_addr_str.clone(),
                ask_id: ask_id.to_string(),
                slashing_block_number: slashing_block_number.to_string(),
                market_id: market_str.clone(),
                slashing_tx: slashing_tx.clone(),
                price_offered: price_offered.to_string(),
                expected_time: deadline.to_string(),
                slashing_penalty,
                slashing_timestamp: slashing_timestamp.to_string(),
                source: "Native".to_string(),
            };

            diesel::insert_into(slashing_records::table)
                .values(&new_slashing_record)
                .execute(conn)
                .expect("Error inserting native slashing record");
        }

        // Process Symbiotic Slashings
        for (token, slashing) in symbiotic_tokens_slashed
            .iter()
            .zip(symbiotic_slashings.iter())
        {
            let existing: Option<TokenTrackerRecord> = token_trackers::table
                .filter(token_trackers::generator_address.eq(&gen_addr_str))
                .filter(token_trackers::market_id.eq(&market_str))
                .select(TokenTrackerRecord::as_select())
                .first(conn)
                .optional()
                .expect("Error querying token tracker");

            let new_tracker: TokenTracker = if let Some(rec) = existing {
                let mut current: TokenTracker = serde_json::from_str(&rec.token_tracker)
                    .expect("Failed to parse total_native_stake");
                current.add_token(token, slashing);
                current
            } else {
                let mut current = TokenTracker::new(); // Create a new TokenTracker if none exists
                current.add_token(token, slashing); // Add the slashing amount
                current
            };

            // Upsert the token tracker record.
            diesel::insert_into(token_trackers::table)
                .values((
                    token_trackers::generator_address.eq(&gen_addr_str),
                    token_trackers::market_id.eq(&market_str),
                    token_trackers::token_tracker.eq(serde_json::to_string(&new_tracker)
                        .expect("Error serializing token tracker")),
                ))
                .on_conflict((token_trackers::generator_address, token_trackers::market_id))
                .do_update()
                .set(token_trackers::token_tracker.eq(
                    serde_json::to_string(&new_tracker).expect("Error serializing token tracker"),
                ))
                .execute(conn)
                .expect("Error upserting token tracker (symbiotic)");

            let slashing_penalty = serde_json::to_string(&(token.clone(), slashing.to_string()))
                .expect("Error serializing slashing penalty (symbiotic)");
            let new_slashing_record = DBSlashingRecord {
                generator_address: gen_addr_str.clone(),
                ask_id: ask_id.to_string(),
                slashing_block_number: slashing_block_number.to_string(),
                market_id: market_str.clone(),
                slashing_tx: slashing_tx.clone(),
                price_offered: price_offered.to_string(),
                expected_time: deadline.to_string(),
                slashing_penalty,
                slashing_timestamp: slashing_timestamp.to_string(),
                source: "Symbiotic".to_string(),
            };

            diesel::insert_into(slashing_records::table)
                .values(&new_slashing_record)
                .execute(conn)
                .expect("Error inserting symbiotic slashing record");
        }
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
        // Get a DB connection.
        let conn = &mut self.pool.get().expect("DB connection error");

        // Query the generator record.
        let gen_rec: GeneratorRecord = generators::table
            .filter(generators::address.eq(generator_address.to_string()))
            .first(conn)
            .expect("Error fetching generator record");

        // Update the appropriate TokenTracker field.
        let new_tracker_str = match source {
            Source::Native => {
                // Deserialize the stored string into a TokenTracker.
                let mut tracker: TokenTracker = serde_json::from_str(&gen_rec.native_stake_locked)
                    .expect("Error parsing native_stake_locked");
                // Add the token amount.
                tracker.add_token(token_address, &stake_locked);
                // Serialize it back.
                serde_json::to_string(&tracker).expect("Error serializing native_stake_locked")
            }
            Source::Symbiotic => {
                let mut tracker: TokenTracker =
                    serde_json::from_str(&gen_rec.symbiotic_stake_locked)
                        .expect("Error parsing symbiotic_stake_locked");
                tracker.add_token(token_address, &stake_locked);
                serde_json::to_string(&tracker).expect("Error serializing symbiotic_stake_locked")
            }
        };

        // Update the corresponding column.
        match source {
            Source::Native => {
                diesel::update(
                    generators::table.filter(generators::address.eq(generator_address.to_string())),
                )
                .set(generators::native_stake_locked.eq(new_tracker_str))
                .execute(conn)
                .expect("Error updating native_stake_locked");
            }
            Source::Symbiotic => {
                diesel::update(
                    generators::table.filter(generators::address.eq(generator_address.to_string())),
                )
                .set(generators::symbiotic_stake_locked.eq(new_tracker_str))
                .execute(conn)
                .expect("Error updating symbiotic_stake_locked");
            }
        }
    }

    fn update_on_stake_released(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_released: U256,
        source: Source,
    ) {
        let conn = &mut self.pool.get().expect("DB connection error");

        let gen_rec: GeneratorRecord = generators::table
            .filter(generators::address.eq(generator_address.to_string()))
            .first(conn)
            .expect("Error fetching generator record");

        let new_tracker_str = match source {
            Source::Native => {
                let mut tracker: TokenTracker = serde_json::from_str(&gen_rec.native_stake_locked)
                    .expect("Error parsing native_stake_locked");
                // Subtract the token amount (assuming sub_token returns a Result).
                tracker
                    .sub_token(token_address, &stake_released)
                    .expect("Error subtracting token from native_stake_locked");
                serde_json::to_string(&tracker).expect("Error serializing native_stake_locked")
            }
            Source::Symbiotic => {
                let mut tracker: TokenTracker =
                    serde_json::from_str(&gen_rec.symbiotic_stake_locked)
                        .expect("Error parsing symbiotic_stake_locked");
                tracker
                    .sub_token(token_address, &stake_released)
                    .expect("Error subtracting token from symbiotic_stake_locked");
                serde_json::to_string(&tracker).expect("Error serializing symbiotic_stake_locked")
            }
        };

        match source {
            Source::Native => {
                diesel::update(
                    generators::table.filter(generators::address.eq(generator_address.to_string())),
                )
                .set(generators::native_stake_locked.eq(new_tracker_str))
                .execute(conn)
                .expect("Error updating native_stake_locked");
            }
            Source::Symbiotic => {
                diesel::update(
                    generators::table.filter(generators::address.eq(generator_address.to_string())),
                )
                .set(generators::symbiotic_stake_locked.eq(new_tracker_str))
                .execute(conn)
                .expect("Error updating symbiotic_stake_locked");
            }
        }
    }

    fn update_on_compute_locked(&mut self, generator_addr: &Address, compute_locked: U256) {
        // Convert the generator address to the string representation used in the DB.
        let conn = &mut self.pool.get().expect("DB connection error");

        // Fetch the current compute_consumed.
        let gen_rec: GeneratorRecord = generators::table
            .filter(generators::address.eq(generator_addr.to_string()))
            .first(conn)
            .expect("Error fetching generator record");

        // Parse the stored compute_consumed value.
        let current_compute: U256 = gen_rec
            .compute_consumed
            .parse()
            .expect("Invalid compute_consumed value");
        let new_compute = current_compute + compute_locked;

        // Update the compute_consumed field.
        diesel::update(
            generators::table.filter(generators::address.eq(generator_addr.to_string())),
        )
        .set(generators::compute_consumed.eq(new_compute.to_string()))
        .execute(conn)
        .expect("Error updating compute_consumed");
    }

    fn update_on_compute_released(&mut self, generator_addr: &Address, compute_released: U256) {
        let conn = &mut self.pool.get().expect("DB connection error");

        let gen_rec: GeneratorRecord = generators::table
            .filter(generators::address.eq(generator_addr.to_string()))
            .first(conn)
            .expect("Error fetching generator record");

        let current_compute: U256 = gen_rec
            .compute_consumed
            .parse()
            .expect("Invalid compute_consumed value");
        let new_compute = current_compute - compute_released;

        diesel::update(
            generators::table.filter(generators::address.eq(generator_addr.to_string())),
        )
        .set(generators::compute_consumed.eq(new_compute.to_string()))
        .execute(conn)
        .expect("Error updating compute_consumed");
    }
}

/// -------------------------
/// Generator Availability
/// -------------------------
impl GeneratorAvailability for GeneratorDatabase {
    fn get_available_compute(&self, addr: Address) -> Option<U256> {
        // Convert the Address to the string format you use in the DB.
        let addr_str = format!("{:?}", addr);
        let conn = &mut self.pool.get().ok()?;

        // Query the overall generator info from the generators table.
        let gen_rec: Option<GeneratorRecord> = generators::table
            .filter(generators::address.eq(addr.to_string()))
            .first(conn)
            .optional()
            .expect("Error querying generator record");

        // If found, convert declared_compute and compute_consumed from strings to U256.
        gen_rec.and_then(|rec| {
            let declared = U256::from_dec_str(&rec.declared_compute).ok()?;
            let consumed = U256::from_dec_str(&rec.compute_consumed).ok()?;
            Some(declared - consumed)
        })
    }

    fn get_available_native_stake(&self, gen_addr: &Address) -> Option<TokenTracker> {
        let addr_str = format!("{:?}", gen_addr);
        let mut conn = self.pool.get().ok()?;

        let record: Option<GeneratorRecord> = generators::table
            .filter(generators::address.eq(addr_str))
            .first(&mut conn)
            .optional()
            .expect("Error loading generator record");

        record.and_then(|rec| {
            // Deserialize the JSON strings into TokenTracker.
            let total: TokenTracker = serde_json::from_str(&rec.total_native_stake).ok()?;
            let locked: TokenTracker = serde_json::from_str(&rec.native_stake_locked).ok()?;
            Some(total - locked)
        })
    }

    fn get_available_symbiotic_stake(&self, gen_addr: &Address) -> Option<TokenTracker> {
        let addr_str = format!("{:?}", gen_addr);
        let mut conn = self.pool.get().ok()?;

        let record: Option<GeneratorRecord> = generators::table
            .filter(generators::address.eq(addr_str))
            .first(&mut conn)
            .optional()
            .expect("Error loading generator record");

        record.and_then(|rec| {
            let total: TokenTracker = serde_json::from_str(&rec.total_symbiotic_stake).ok()?;
            let locked: TokenTracker = serde_json::from_str(&rec.symbiotic_stake_locked).ok()?;
            Some(total - locked)
        })
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

    // Helper to get cached results or load from DB.
    fn query_by_market_id_and_only_active(&self, market_id: &U256) -> GeneratorQueryResult {
        let market_id_str = market_id.to_string();

        // No cache entry; run the query.
        let conn = &mut self.pool.get().expect("Failed to get connection from pool");

        let results: Vec<(GeneratorMarketRecord, GeneratorRecord)> = generator_markets::table
            .inner_join(
                generators::table.on(generators::address.eq(generator_markets::generator_address)),
            )
            .filter(generator_markets::market_id.eq(&market_id_str))
            .filter(generators::active.eq(true))
            .select((
                GeneratorMarketRecord::as_select(),
                GeneratorRecord::as_select(),
            ))
            .load::<(GeneratorMarketRecord, GeneratorRecord)>(conn)
            .expect("Error loading generator markets");

        // Convert each DB record into an owned GeneratorInfoPerMarket.
        let owned_infos: Vec<GeneratorInfoPerMarket> = results
            .into_iter()
            .map(|(market_rec, _)| GeneratorInfoPerMarket::from(market_rec))
            .collect();

        // Store in the cache.
        self.cache
            .borrow_mut()
            .insert(market_id_str.clone(), owned_infos);

        // Return references into the cache.
        let cached = self.cache.borrow();
        let cached_result = cached.get(&market_id_str).unwrap();
        let refs: Vec<GeneratorInfoPerMarket> = cached_result.iter().cloned().collect();

        GeneratorQueryResult::new(refs)
        // GeneratorQueryResult::new(Vec::new())
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
        let generator_array = generator_query.result();
        let mut filtered: Vec<GeneratorInfoPerMarket> = Vec::new();

        for elem in generator_array {
            // Get a DB connection.
            let conn = &mut self.pool.get().expect("DB connection error");

            // Query the overall generator info from the generators table.
            let gen_rec: Option<GeneratorRecord> = generators::table
                .filter(generators::address.eq(elem.address.to_string()))
                .first(conn)
                .optional()
                .expect("Error querying generator record");

            if let Some(rec) = gen_rec {
                // Parse the declared and consumed compute.
                let declared: U256 = rec.declared_compute.parse().expect("Invalid U256");
                let consumed: U256 = rec.compute_consumed.parse().expect("Invalid U256");
                let idle_compute = declared.sub(consumed);
                // If idle compute >= the per-market required compute...
                if idle_compute.ge(&elem.compute_required_per_request) {
                    // Then query the generator market record.
                    let market_rec: Option<GeneratorMarketRecord> = generator_markets::table
                        .filter(generator_markets::generator_address.eq(elem.address.to_string()))
                        .filter(generator_markets::market_id.eq(elem.market_id.to_string()))
                        .select(GeneratorMarketRecord::as_select())
                        .first::<GeneratorMarketRecord>(conn)
                        .optional()
                        .expect("Error querying generator market record");
                    if let Some(market_rec) = market_rec {
                        // Convert DB record to domain object.
                        let info = GeneratorInfoPerMarket::from(market_rec);
                        filtered.push(info);
                    }
                }
            }
        }
        log::debug!("Generator with idle compute: {}", filtered.len());
        GeneratorQueryResult::new(filtered)
        // GeneratorQueryResult::new(Vec::new())
    }

    fn filter_by_available_native_stake(
        &self,
        generator_query: GeneratorQueryResult,
        min_stake: Vec<AddressTokenPair>,
    ) -> GeneratorQueryResult {
        let generator_array = generator_query.result();
        let mut filtered: Vec<GeneratorInfoPerMarket> = Vec::new();

        for elem in generator_array {
            let conn = &mut self.pool.get().expect("DB connection error");

            // Query the generator record to obtain native stake details.
            let gen_rec: Option<GeneratorRecord> = generators::table
                .filter(generators::address.eq(elem.address.to_string()))
                .first(conn)
                .optional()
                .expect("Error querying generator record");

            if let Some(rec) = gen_rec {
                let total: TokenTracker = serde_json::from_str(&rec.total_native_stake)
                    .expect("Failed to parse total_native_stake");
                let locked: TokenTracker = serde_json::from_str(&rec.native_stake_locked)
                    .expect("Failed to parse native_stake_locked");
                let remaining = total.sub(locked);

                log::debug!(
                    "Generator: {} remaining native stake: {:?}",
                    elem.address.to_string(),
                    remaining
                );
                log::debug!("Required native stake pairs: {:?}", min_stake);

                // Check if at least one of the required stake conditions is met.
                let is_valid = min_stake
                    .iter()
                    .any(|pair| remaining.has_more_than_or_eq(pair));
                if is_valid {
                    // Retrieve the generator market record.
                    let market_rec: Option<GeneratorMarketRecord> = generator_markets::table
                        .filter(generator_markets::generator_address.eq(elem.address.to_string()))
                        .filter(generator_markets::market_id.eq(elem.market_id.to_string()))
                        .select(GeneratorMarketRecord::as_select())
                        .first::<GeneratorMarketRecord>(conn)
                        .optional()
                        .expect("Error querying generator market record");
                    if let Some(market_rec) = market_rec {
                        let info = GeneratorInfoPerMarket::from(market_rec);
                        filtered.push(info);
                    }
                }
            }
        }
        log::debug!(
            "Number of generators available with native stake {:?} = {}",
            min_stake,
            filtered.len()
        );
        GeneratorQueryResult::new(filtered)
        // GeneratorQueryResult::new(Vec::new())
    }

    fn filter_by_available_symbiotic_stake(
        &self,
        generator_query: GeneratorQueryResult,
        min_stake: Vec<AddressTokenPair>,
    ) -> GeneratorQueryResult {
        let generator_array = generator_query.result();
        let mut filtered: Vec<GeneratorInfoPerMarket> = Vec::new();

        for elem in generator_array {
            let conn = &mut self.pool.get().expect("DB connection error");

            // Query the generator record to obtain symbiotic stake details.
            let gen_rec: Option<GeneratorRecord> = generators::table
                .filter(generators::address.eq(elem.address.to_string()))
                .first(conn)
                .optional()
                .expect("Error querying generator record");

            if let Some(rec) = gen_rec {
                let total: TokenTracker = serde_json::from_str(&rec.total_symbiotic_stake)
                    .expect("Failed to parse total_symbiotic_stake");
                let locked: TokenTracker = serde_json::from_str(&rec.symbiotic_stake_locked)
                    .expect("Failed to parse total_symbiotic_stake");
                let remaining = total.sub(locked);

                log::debug!(
                    "Generator: {} remaining symbiotic stake: {:?}",
                    elem.address.to_string(),
                    remaining
                );
                log::debug!("Required symbiotic stake pairs: {:?}", min_stake);

                let is_valid = min_stake
                    .iter()
                    .any(|pair| remaining.has_more_than_or_eq(pair));
                if is_valid {
                    let market_rec: Option<GeneratorMarketRecord> = generator_markets::table
                        .filter(generator_markets::generator_address.eq(elem.address.to_string()))
                        .filter(generator_markets::market_id.eq(elem.market_id.to_string()))
                        .select(GeneratorMarketRecord::as_select())
                        .first::<GeneratorMarketRecord>(conn)
                        .optional()
                        .expect("Error querying generator market record");
                    if let Some(market_rec) = market_rec {
                        let info = GeneratorInfoPerMarket::from(market_rec);
                        filtered.push(info.clone());
                    }
                }
            }
        }
        log::debug!(
            "Number of generators available with symbiotic stake {:?} = {}",
            min_stake,
            filtered.len()
        );
        GeneratorQueryResult::new(filtered)
        // GeneratorQueryResult::new(Vec::new())
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
        // Get the original vector of domain generator info.
        let generator_array = generator_query.result();

        // Create a vector to collect the filtered results.
        let mut filtered: Vec<GeneratorInfoPerMarket> = Vec::new();

        for elem in generator_array {
            // Check the key store for ECIES public key support.
            if key_store
                .get_by_address(&elem.address, elem.market_id.as_u64())
                .is_some()
            {
                // Obtain a database connection.
                let conn = &mut self.pool.get().expect("DB connection error");

                // Query the generator_markets table for a matching record.
                let result: Option<GeneratorMarketRecord> = generator_markets::table
                    .filter(generator_markets::generator_address.eq(elem.address.to_string()))
                    .filter(generator_markets::market_id.eq(elem.market_id.to_string()))
                    .select(GeneratorMarketRecord::as_select())
                    .first::<GeneratorMarketRecord>(conn)
                    .optional()
                    .expect("Error querying generator market record");

                if let Some(record) = result {
                    // Convert the DB record back into a domain object.
                    filtered.push(GeneratorInfoPerMarket::from(record.clone()));
                }
            }
        }

        GeneratorQueryResult::new(filtered)
        // GeneratorQueryResult::new(Vec::new())
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
        // Convert the domain request into a DB record.
        let mut record: WithdrawalRequestRecord = withdrawal_request.into();
        // Set the generator_address using the provided operator_address.
        record.generator_address = operator_address.to_string();

        let conn = &mut self.pool.get().expect("DB connection error");

        diesel::insert_into(withdrawal_requests::table)
            .values(&record)
            .execute(conn)
            .expect("Error inserting withdrawal request");
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
        // Convert the domain request into a DB record.
        let record: WithdrawalRequestRecord = withdrawal_request.into();

        let conn = &mut self.pool.get().expect("DB connection error");

        // Build a query filtering by the operator's address and all fields from the record.
        let num_deleted = diesel::delete(
            withdrawal_requests::table
                .filter(withdrawal_requests::generator_address.eq(operator_address.to_string()))
                .filter(withdrawal_requests::account.eq(record.account))
                .filter(withdrawal_requests::token.eq(record.token))
                .filter(withdrawal_requests::amount.eq(record.amount))
                .filter(withdrawal_requests::request_index.eq(record.request_index))
                .filter(withdrawal_requests::timestamp.eq(record.timestamp)),
        )
        .execute(conn)
        .expect("Error deleting withdrawal request");

        num_deleted > 0
    }
}

/// -------------------------
/// Generator Metadata
/// -------------------------
impl GeneratorMetadata for GeneratorDatabase {
    fn update_generator_metadata(&mut self, generator_addr: Address, generator_meta_data: Bytes) {
        // Get a DB connection from the pool.
        let conn = &mut self.pool.get().expect("DB connection error");

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
        generator_address: Address,
        _timestamp: std::time::SystemTime,
    ) {
        // Get a DB connection from the pool.
        let conn = &mut self.pool.get().expect("DB connection error");

        // Update the record matching the generator's address by incrementing jobs_missed_counter.
        diesel::update(
            generators::table.filter(generators::address.eq(generator_address.to_string())),
        )
        .set(generators::jobs_missed_counter.eq(generators::jobs_missed_counter + 1))
        .execute(conn)
        .expect("Error updating jobs missed counter");
    }

    fn get_job_missed_count(&self, generator_address: &Address) -> usize {
        // Get a DB connection from the pool.
        let conn = &mut self.pool.get().expect("DB connection error");

        // Query the jobs_missed_counter for the record matching the generator's address.
        let count: i32 = generators::table
            .filter(generators::address.eq(generator_address.to_string()))
            .select(generators::jobs_missed_counter)
            .first(conn)
            .expect("Error loading job missed count");

        count as usize
    }
}

/// -------------------------
/// Generator Additional Query
/// -------------------------
impl GeneratorAdditionalQuery for GeneratorDatabase {
    fn all_generators_address(&self) -> Vec<Address> {
        // Query the generators table and return all addresses.
        let conn = &mut self.pool.get().expect("DB connection error");

        // Query all addresses as Strings from the generators table.
        let address_strings: Vec<String> = generators::table
            .select(generators::address)
            .load(conn)
            .expect("Error loading generator addresses");

        // Convert each String to an Address.
        address_strings
            .into_iter()
            .map(|s| s.parse().expect("Invalid address format"))
            .collect()
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
        let conn = &mut self.pool.get().expect("DB connection error");

        let address_str = format!("{:?}", address);
        let market_id_str = market.to_string();

        // Query the generator_markets table for a record matching the address and market_id.
        let result = generator_markets::table
            .filter(generator_markets::generator_address.eq(address_str))
            .filter(generator_markets::market_id.eq(market_id_str))
            .select(GeneratorMarketRecord::as_select())
            .first::<GeneratorMarketRecord>(conn)
            .optional() // Returns Ok(Some(record)) if found, Ok(None) if not found.
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
