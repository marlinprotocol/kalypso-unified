use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use ethers::types::{Address, U256};
use crate::utility::TokenTracker;
use crate::schema::token_locks;

/// Represents a token lock entry in the database.
#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "token_locks"]
pub struct TokenLock {
    pub token: Vec<u8>, // Store Address as a byte array
    pub amount: String,  // Store U256 as a String to preserve precision
}

/// A PostgreSQL-backed implementation of NativeStakingOperations.
pub struct NativeStakingStore {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl NativeStakingStore {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        Self { pool }
    }
}

#[async_trait]
pub trait NativeStakingOperations {
    async fn set_lock_token(&mut self, token: Address, amount: U256);
    async fn remove_lock_token(&mut self, token: Address);
    async fn tokens_to_lock(&self) -> TokenTracker;
}

#[async_trait]
impl NativeStakingOperations for NativeStakingStore {
    async fn set_lock_token(&mut self, token: Address, amount: U256) {
        use crate::schema::token_locks::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");

        let new_lock = TokenLock {
            token: token.as_bytes().to_vec(),
            amount: amount.to_string(), // Convert U256 to String
        };

        diesel::insert_into(token_locks)
            .values(&new_lock)
            .on_conflict(token)
            .do_update()
            .set(amount.eq(new_lock.amount))
            .execute(connection)
            .expect("Failed to insert or update token lock");
    }

    async fn remove_lock_token(&mut self, token: Address) {
        use crate::schema::token_locks::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");

        diesel::delete(token_locks.filter(token.eq(token.as_bytes().to_vec())))
            .execute(connection)
            .expect("Failed to delete token lock");
    }

    async fn tokens_to_lock(&self) -> TokenTracker {
        use crate::schema::token_locks::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");

        let results = token_locks
            .load::<TokenLock>(connection)
            .expect("Failed to fetch locked tokens");

        let mut tracker = TokenTracker::new();

        for lock in results {
            let token_address = Address::from_slice(&lock.token);
            let amount = U256::from_dec_str(&lock.amount).expect("Failed to parse U256 amount");

            tracker.insert(token_address, amount);
        }

        tracker
    }
}
