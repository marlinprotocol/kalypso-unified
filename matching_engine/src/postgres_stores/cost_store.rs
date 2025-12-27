use async_trait::async_trait;
use diesel::prelude::*;
use ethers::types::U256;
// use crate::models::CostRecord;
use crate::{costs::CostStoreOperations, schema::cost_record};

use super::models::{bytes_to_u256, u256_to_bytes, CostDatabase};

#[async_trait]
impl CostStoreOperations for CostDatabase {
    /// Inserts or updates a key-value pair in the database.
    async fn upsert(&mut self, key: u8, value: U256) {
        let conn = &mut self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(cost_record::table)
            .values((
                cost_record::key.eq(key as i16),
                cost_record::value.eq(u256_to_bytes(value)),
            ))
            .on_conflict(cost_record::key)
            .do_update()
            .set(cost_record::value.eq(u256_to_bytes(value)))
            .execute(conn)
            .expect("Failed to upsert cost store");
    }

    /// Removes a key-value pair by key and returns the deleted value.
    async fn remove(&mut self, key: u8) -> Option<U256> {
        let conn = &mut self.pool.get().expect("Failed to get DB connection");
        let deleted_value: Option<Vec<u8>> =
            diesel::delete(cost_record::table.filter(cost_record::key.eq(key as i16)))
                .returning(cost_record::value)
                .get_result::<Vec<u8>>(conn)
                .ok();

        deleted_value.and_then(|val| Some(bytes_to_u256(&val)))
    }

    /// Retrieves the value associated with a key.
    async fn get(&self, key: u8) -> Option<U256> {
        let conn = &mut self.pool.get().expect("Failed to get DB connection");
        let stored_value: Option<Vec<u8>> = cost_record::table
            .filter(cost_record::key.eq(key as i16))
            .select(cost_record::value)
            .first::<Vec<u8>>(conn)
            .ok();

        stored_value.and_then(|val| Some(bytes_to_u256(&val)))
    }

    #[doc = " Creates a new CostStore."]
    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}
