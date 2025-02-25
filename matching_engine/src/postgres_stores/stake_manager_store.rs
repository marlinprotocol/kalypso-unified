use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use ethers::types::Address;
use async_trait::async_trait;
use crate::schema::stake_pools;

/// Represents a stake pool entry in the database.
#[derive(Debug, Queryable, Insertable)]
#[table_name = "stake_pools"]
pub struct StakePool {
    pub address: Vec<u8>, // Store Address as a byte array
}

/// A PostgreSQL-backed implementation of StakeManagerOperations.
pub struct StakeManagerStore {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl StakeManagerStore {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        Self { pool }
    }
}

pub trait StakeManagerOperations {
    fn add(&mut self, address: Address) -> bool;
    fn remove(&mut self, address: &Address) -> bool;
    fn exists(&self, address: &Address) -> bool;
    fn get_all(&self) -> Vec<Address>;
}

impl StakeManagerOperations for StakeManagerStore {
    fn add(&mut self, address: Address) -> bool {
        use crate::schema::stake_pools::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");

        let new_entry = StakePool {
            address: address.as_bytes().to_vec(),
        };

        let inserted = diesel::insert_into(stake_pools)
            .values(&new_entry)
            .on_conflict_do_nothing()
            .execute(connection)
            .expect("Failed to insert stake pool");

        inserted > 0
    }

    fn remove(&mut self, address: &Address) -> bool {
        use crate::schema::stake_pools::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");

        let deleted = diesel::delete(stake_pools.filter(address.eq(address.as_bytes().to_vec())))
            .execute(connection)
            .expect("Failed to remove stake pool");

        deleted > 0
    }

    fn exists(&self, address: &Address) -> bool {
        use crate::schema::stake_pools::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");

        let count: i64 = stake_pools
            .filter(address.eq(address.as_bytes().to_vec()))
            .count()
            .get_result(connection)
            .unwrap_or(0);

        count > 0
    }

    fn get_all(&self) -> Vec<Address> {
        use crate::schema::stake_pools::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");

        let results = stake_pools
            .select(address)
            .load::<Vec<u8>>(connection)
            .expect("Failed to fetch stake pools");

        results.into_iter().map(|bytes| Address::from_slice(&bytes)).collect()
    }
}
