use ethers::types::Address;

use crate::{generator_lib::stake_manager_store::StakeManagerOperations, schema::stake_manager_record};

use super::models::{StakeManagerRecord, StakeManagerStoreDB};

use diesel::prelude::*;
use diesel::dsl::exists;

impl StakeManagerOperations for StakeManagerStoreDB {
    /// Inserts a new pool address into the DB.
    /// Returns true if the address was not already present.
    fn add(&mut self, pool_address: Address) -> bool {
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = pool_address.to_string();
        let new_record = StakeManagerRecord { address: addr_str.clone() };

        // Use insert_into with an on_conflict clause to ignore duplicates.
        let result = diesel::insert_into(stake_manager_record::table)
            .values(&new_record)
            .on_conflict_do_nothing()
            .execute(conn)
            .expect("Error inserting enabled pool");

        // If one row was affected, the record was inserted; otherwise it already existed.
        result == 1
    }

    /// Removes an address from the DB.
    /// Returns true if a record was deleted.
    fn remove(&mut self, pool_address: &Address) -> bool {
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = pool_address.to_string();
        let num_deleted = diesel::delete(stake_manager_record::table.filter(stake_manager_record::address.eq(addr_str)))
            .execute(conn)
            .expect("Error deleting enabled pool");
        num_deleted > 0
    }

    /// Checks if a pool address exists in the DB.
    fn exists(&self, pool_address: &Address) -> bool {
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = pool_address.to_string();
        let exists: bool = diesel::select(exists(stake_manager_record::table.filter(stake_manager_record::address.eq(addr_str))))
            .get_result(conn)
            .expect("Error checking enabled pool existence");
        exists
    }

    /// Retrieves all enabled pool addresses as a vector.
    fn get_all(&self) -> Vec<Address> {
        let conn = &mut self.pool.get().expect("DB connection error");
        let records: Vec<StakeManagerRecord> = stake_manager_record::table
            .load(conn)
            .expect("Error loading enabled pools");
        // Parse the stored string back into an Address.
        records.into_iter()
            .map(|r| r.address.parse().expect("Invalid address in DB"))
            .collect()
    }
}