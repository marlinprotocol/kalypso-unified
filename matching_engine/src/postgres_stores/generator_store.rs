use diesel::prelude::*;
use crate::schema::generators;
use crate::db::DbPool;
use super::{Generator, GeneratorInfoPerMarket};

pub struct GeneratorDb {
    pub pool: DbPool,
}

impl GeneratorRegistration for GeneratorDb {
    fn register_generator(&mut self, generator: Generator) {
        let conn = &mut self.pool.get().unwrap();
        diesel::insert_into(generators::table)
            .values(&generator)
            .execute(conn)
            .expect("Error inserting generator");
    }

    fn register_generator_in_market(&mut self, generator_market: GeneratorInfoPerMarket) {
        // Similar logic for inserting generator into market-specific table
    }

    fn remove_by_address_and_market(&mut self, address: &Address, market_id: &U256) {
        let conn = &mut self.pool.get().unwrap();
        diesel::delete(
            generators::table
                .filter(generators::address.eq(address))
                .filter(generators::market_id.eq(market_id)),
        )
        .execute(conn)
        .expect("Error removing generator");
    }

    fn remove_by_address(&mut self, address: &Address) {
        let conn = &mut self.pool.get().unwrap();
        diesel::delete(generators::table.filter(generators::address.eq(address)))
            .execute(conn)
            .expect("Error removing generator by address");
    }

    fn get_by_address(&self, address: &Address) -> Option<Generator> {
        let conn = &mut self.pool.get().unwrap();
        generators::table
            .filter(generators::address.eq(address))
            .first::<Generator>(conn)
            .ok()
    }

    fn is_active(&self, generator_address: &Address) -> bool {
        let conn = &mut self.pool.get().unwrap();
        generators::table
            .filter(generators::address.eq(generator_address))
            .select(generators::is_active)
            .first::<bool>(conn)
            .unwrap_or(false)
    }
}

use crate::schema::generator_stakes;

impl GeneratorStakeComputeManagement for GeneratorDb {
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
        let conn = &mut self.pool.get().unwrap();
        diesel::insert_into(generator_stakes::table)
            .values((
                generator_stakes::generator_address.eq(generator_address),
                generator_stakes::token_address.eq(token_address),
                generator_stakes::amount.eq(amount),
                generator_stakes::block_number.eq(block_number),
                generator_stakes::transaction_index.eq(transaction_index),
                generator_stakes::log_index.eq(log_index),
                generator_stakes::tx.eq(tx),
                generator_stakes::source.eq(source as i32),
            ))
            .execute(conn)
            .expect("Failed to add extra stake");
    }

    fn update_intended_stake_util(&mut self, address: &Address, new_stake_util: U256) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_stakes::table.filter(generator_stakes::generator_address.eq(address)))
            .set(generator_stakes::stake_util.eq(new_stake_util))
            .execute(conn)
            .expect("Failed to update intended stake utilization");
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
        let conn = &mut self.pool.get().unwrap();
        diesel::delete(
            generator_stakes::table
                .filter(generator_stakes::generator_address.eq(generator_address))
                .filter(generator_stakes::token_address.eq(token_address))
                .filter(generator_stakes::amount.eq(amount))
                .filter(generator_stakes::block_number.eq(block_number)),
        )
        .execute(conn)
        .expect("Failed to remove stake");
    }

    fn update_reward_address(&mut self, address: &Address, new_reward_address: Address) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_stakes::table.filter(generator_stakes::generator_address.eq(address)))
            .set(generator_stakes::reward_address.eq(new_reward_address))
            .execute(conn)
            .expect("Failed to update reward address");
    }

    fn add_extra_compute(&mut self, address: &Address, compute: U256) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_stakes::table.filter(generator_stakes::generator_address.eq(address)))
            .set(generator_stakes::compute.eq(compute))
            .execute(conn)
            .expect("Failed to add extra compute");
    }

    fn update_intended_compute_util(&mut self, address: &Address, new_compute_util: U256) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_stakes::table.filter(generator_stakes::generator_address.eq(address)))
            .set(generator_stakes::compute_util.eq(new_compute_util))
            .execute(conn)
            .expect("Failed to update compute utilization");
    }

    fn remove_compute(&mut self, address: &Address, compute: U256) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_stakes::table.filter(generator_stakes::generator_address.eq(address)))
            .set(generator_stakes::compute.eq(generator_stakes::compute - compute))
            .execute(conn)
            .expect("Failed to remove compute");
    }
}

use crate::schema::generator_markets;

impl GeneratorMarketManagement for GeneratorDb {
    fn update_state(&mut self, address: &Address, market_id: &U256, new_state: GeneratorState) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_markets::table
            .filter(generator_markets::generator_address.eq(address))
            .filter(generator_markets::market_id.eq(market_id)))
            .set(generator_markets::state.eq(new_state as i32))
            .execute(conn)
            .expect("Failed to update generator state");
    }

    fn update_on_assigned_task(&mut self, address: &Address, market_id: &U256) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_markets::table
            .filter(generator_markets::generator_address.eq(address))
            .filter(generator_markets::market_id.eq(market_id)))
            .set(generator_markets::assigned_tasks.eq(generator_markets::assigned_tasks + 1))
            .execute(conn)
            .expect("Failed to update assigned tasks");
    }

    fn update_on_submit_proof(&mut self, address: &Address, market_id: &U256, earning: &U256, block_number: &U64) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_markets::table
            .filter(generator_markets::generator_address.eq(address))
            .filter(generator_markets::market_id.eq(market_id)))
            .set((
                generator_markets::earnings.eq(generator_markets::earnings + earning),
                generator_markets::block_number.eq(block_number),
            ))
            .execute(conn)
            .expect("Failed to update proof submission");
    }

    fn reduce_active_requests(&mut self, generator_address: &Address, market_id: &U256) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_markets::table
            .filter(generator_markets::generator_address.eq(generator_address))
            .filter(generator_markets::market_id.eq(market_id)))
            .set(generator_markets::active_requests.eq(generator_markets::active_requests - 1))
            .execute(conn)
            .expect("Failed to reduce active requests");
    }

    fn pause_assignments_across_all_markets(&mut self, address: &Address) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_markets::table
            .filter(generator_markets::generator_address.eq(address)))
            .set(generator_markets::is_paused.eq(true))
            .execute(conn)
            .expect("Failed to pause assignments");
    }

    fn resume_assignments_accross_all_markets(&mut self, address: &Address) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_markets::table
            .filter(generator_markets::generator_address.eq(address)))
            .set(generator_markets::is_paused.eq(false))
            .execute(conn)
            .expect("Failed to resume assignments");
    }
}

use crate::schema::slashing_records;

impl GeneratorSlashingManagement for GeneratorDb {
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
        let conn = &mut self.pool.get().unwrap();
        diesel::insert_into(slashing_records::table)
            .values((
                slashing_records::generator_address.eq(generator_address),
                slashing_records::ask_id.eq(ask_id),
                slashing_records::market_id.eq(market_id),
                slashing_records::native_tokens_slashed.eq(native_tokens_slashed),
                slashing_records::native_slashings.eq(native_slashings),
                slashing_records::symbiotic_tokens_slashed.eq(symbiotic_tokens_slashed),
                slashing_records::symbiotic_slashings.eq(symbiotic_slashings),
                slashing_records::slashing_tx.eq(slashing_tx),
                slashing_records::price_offered.eq(price_offered),
                slashing_records::deadline.eq(deadline),
                slashing_records::slashing_block_number.eq(slashing_block_number),
                slashing_records::slashing_timestamp.eq(slashing_timestamp),
            ))
            .execute(conn)
            .expect("Failed to record slashing event");
    }
}

use crate::schema::generator_locks;

impl GeneratorLockManagement for GeneratorDb {
    fn update_on_stake_locked(&mut self, generator_address: &Address, token_address: &Address, stake_locked: U256, source: Source) {
        let conn = &mut self.pool.get().unwrap();
        diesel::insert_into(generator_locks::table)
            .values((
                generator_locks::generator_address.eq(generator_address),
                generator_locks::token_address.eq(token_address),
                generator_locks::stake_locked.eq(stake_locked),
                generator_locks::source.eq(source as i32),
            ))
            .execute(conn)
            .expect("Failed to update stake locked");
    }

    fn update_on_stake_released(&mut self, generator_address: &Address, token_address: &Address, stake_released: U256, source: Source) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_locks::table
            .filter(generator_locks::generator_address.eq(generator_address))
            .filter(generator_locks::token_address.eq(token_address)))
            .set(generator_locks::stake_locked.eq(generator_locks::stake_locked - stake_released))
            .execute(conn)
            .expect("Failed to release stake");
    }

    fn update_on_compute_locked(&mut self, address: &Address, compute_locked: U256) {
        let conn = &mut self.pool.get().unwrap();
        diesel::insert_into(generator_locks::table)
            .values((
                generator_locks::generator_address.eq(address),
                generator_locks::compute_locked.eq(compute_locked),
            ))
            .execute(conn)
            .expect("Failed to lock compute");
    }

    fn update_on_compute_released(&mut self, address: &Address, compute_released: U256) {
        let conn = &mut self.pool.get().unwrap();
        diesel::update(generator_locks::table
            .filter(generator_locks::generator_address.eq(address)))
            .set(generator_locks::compute_locked.eq(generator_locks::compute_locked - compute_released))
            .execute(conn)
            .expect("Failed to release compute");
    }
}
