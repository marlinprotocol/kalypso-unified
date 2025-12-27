use super::models::{KeyDatabase, KeyStoreRecord};
use crate::generator_lib::key_store::{Key, KeyStoreOperations};
use crate::schema::key_record;
use diesel::prelude::*;
use ethers::core::types::Address;
use ethers::types::Bytes;

impl KeyStoreOperations for KeyDatabase {
    fn insert(&mut self, address_val: Address, value: u64, key: Key) {
        let conn = &mut self.pool.get().expect("DB connection error");
        // Create the record from our Key.
        let record: KeyStoreRecord = key.into();
        // In case your in‑memory tuple key is (address, value) and value corresponds to key_index,
        // you may choose to ignore the `value` parameter if it’s already in `key.key_index`.
        diesel::insert_into(key_record::table)
            .values(&record)
            .execute(conn)
            .expect("Error inserting key into key_store");
    }

    fn get_by_address(&self, address_val: &Address, value: u64) -> Option<Key> {
        let conn = &mut self.pool.get().expect("DB connection error");

        key_record::table
            .filter(key_record::address.eq(address_val.to_string()))
            .filter(key_record::key_index.eq(value as i64))
            .first::<KeyStoreRecord>(conn)
            .optional()
            .expect("Error querying key_store")
            .map(|rec| rec.into())
    }

    fn remove_by_address(&mut self, address_val: &Address, value: u64) {
        let conn = &mut self.pool.get().expect("DB connection error");
        diesel::delete(
            key_record::table
                .filter(key_record::address.eq(address_val.to_string()))
                .filter(key_record::key_index.eq(value as i64)),
        )
        .execute(conn)
        .expect("Error deleting from key_store");
    }

    fn update_pub_key(&mut self, address_val: &Address, value: u64, new_pub_key: Option<Bytes>) {
        let conn = &mut self.pool.get().expect("DB connection error");
        diesel::update(
            key_record::table
                .filter(key_record::address.eq(address_val.to_string()))
                .filter(key_record::key_index.eq(value as i64)),
        )
        .set(key_record::ecies_pub_key.eq(new_pub_key.map(|b| b.to_vec())))
        .execute(conn)
        .expect("Error updating key_store");
    }
}
