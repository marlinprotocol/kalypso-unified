use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use ethers::types::U256;
use crate::schema::cost_store;

/// Structure representing the CostStore.
pub struct CostStore {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait]
impl CostStoreOperations for CostStore {
    /// Creates a new CostStore with a database connection pool.
    fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        Self { pool }
    }

    /// Inserts or updates a key-value pair in the database.
    async fn upsert(&mut self, key: u8, value: U256) {
        use crate::schema::cost_store::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(cost_store)
            .values((key.eq(key as i16), value.eq(diesel::dsl::sql::<diesel::sql_types::Numeric>(&value.to_string()))))
            .on_conflict(key)
            .do_update()
            .set(value.eq(diesel::dsl::sql::<diesel::sql_types::Numeric>(&value.to_string())))
            .execute(connection)
            .expect("Failed to upsert cost store");
    }

    /// Removes a key-value pair by key and returns the deleted value.
    async fn remove(&mut self, key: u8) -> Option<U256> {
        use crate::schema::cost_store::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");
        let deleted_value: Option<String> = diesel::delete(cost_store.filter(key.eq(key as i16)))
            .returning(value)
            .get_result::<String>(connection)
            .ok();

        deleted_value.and_then(|val| U256::from_dec_str(&val).ok())
    }

    /// Retrieves the value associated with a key.
    async fn get(&self, key: u8) -> Option<U256> {
        use crate::schema::cost_store::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");
        let stored_value: Option<String> = cost_store
            .filter(key.eq(key as i16))
            .select(value)
            .first::<String>(connection)
            .ok();

        stored_value.and_then(|val| U256::from_dec_str(&val).ok())
    }
}
