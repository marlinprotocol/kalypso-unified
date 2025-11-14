use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use ethers::types::{Address, U256};
use crate::schema::operator_stakes;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "operator_stakes"]
pub struct OperatorStake {
    pub operator: Vec<u8>,
    pub token_address: Vec<u8>,
    pub absolute_stake: String, // Store U256 as a numeric string
}

pub struct OperatorStakeStore {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl OperatorStakeStore {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        Self { pool }
    }
}

pub trait OperatorStakeManagement {
    fn note_down_stake(&mut self, operator: &Address, token_address: &Address, absolute_stake: &U256);
    fn get_latest_stake_info(&self, operator: &Address, token_address: &Address) -> U256;
    fn clean_operators(&mut self);
}

impl OperatorStakeManagement for OperatorStakeStore {
    fn note_down_stake(&mut self, operator: &Address, token_address: &Address, absolute_stake: &U256) {
        use crate::schema::operator_stakes::dsl::*;
        let connection = &mut self.pool.get().expect("DB connection failed");

        let new_stake = OperatorStake {
            operator: operator.as_bytes().to_vec(),
            token_address: token_address.as_bytes().to_vec(),
            absolute_stake: absolute_stake.to_string(),
        };

        diesel::insert_into(operator_stakes)
            .values(&new_stake)
            .on_conflict((operator, token_address))
            .do_update()
            .set(absolute_stake.eq(new_stake.absolute_stake))
            .execute(connection)
            .expect("Failed to insert/update stake");
    }

    fn get_latest_stake_info(&self, operator: &Address, token_address: &Address) -> U256 {
        use crate::schema::operator_stakes::dsl::*;
        let connection = &mut self.pool.get().expect("DB connection failed");

        let result: Option<String> = operator_stakes
            .filter(operator.eq(operator.as_bytes().to_vec()))
            .filter(token_address.eq(token_address.as_bytes().to_vec()))
            .select(absolute_stake)
            .first(connection)
            .ok();

        result.map(|s| U256::from_dec_str(&s).unwrap()).unwrap_or(U256::zero())
    }

    fn clean_operators(&mut self) {
        use crate::schema::operator_stakes::dsl::*;
        let connection = &mut self.pool.get().expect("DB connection failed");

        diesel::delete(operator_stakes)
            .execute(connection)
            .expect("Failed to clear operators");
    }
}


use crate::schema::token_locks;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "token_locks"]
pub struct TokenLock {
    pub token: Vec<u8>,
    pub amount: String,
}

pub trait TokenLockManagement {
    fn set_lock_token(&mut self, token: Address, amount: U256);
    fn remove_lock_token(&mut self, token: Address);
}

impl TokenLockManagement for OperatorStakeStore {
    fn set_lock_token(&mut self, token: Address, amount: U256) {
        use crate::schema::token_locks::dsl::*;
        let connection = &mut self.pool.get().expect("DB connection failed");

        let lock_entry = TokenLock {
            token: token.as_bytes().to_vec(),
            amount: amount.to_string(),
        };

        diesel::insert_into(token_locks)
            .values(&lock_entry)
            .on_conflict(token)
            .do_update()
            .set(amount.eq(lock_entry.amount))
            .execute(connection)
            .expect("Failed to set lock token");
    }

    fn remove_lock_token(&mut self, token: Address) {
        use crate::schema::token_locks::dsl::*;
        let connection = &mut self.pool.get().expect("DB connection failed");

        diesel::delete(token_locks.filter(token.eq(token.as_bytes().to_vec())))
            .execute(connection)
            .expect("Failed to remove lock token");
    }
}

use crate::schema::vault_snapshots;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "vault_snapshots"]
pub struct VaultSnapshotEntry {
    pub captured_timestamp: String,
    pub index: String,
    pub snapshot_data: Vec<u8>, // Store serialized snapshot
}

pub trait VaultSnapshotManagement {
    fn store_vault_snapshot(&mut self, captured_timestamp: U256, index: U256, snapshot: VaultSnapshot);
    fn get_vault_snapshot(&self, captured_timestamp: U256, index: U256) -> Option<VaultSnapshot>;
}

impl VaultSnapshotManagement for OperatorStakeStore {
    fn store_vault_snapshot(&mut self, captured_timestamp: U256, index: U256, snapshot: VaultSnapshot) {
        use crate::schema::vault_snapshots::dsl::*;
        let connection = &mut self.pool.get().expect("DB connection failed");

        let serialized_snapshot = bincode::serialize(&snapshot).expect("Failed to serialize snapshot");

        let snapshot_entry = VaultSnapshotEntry {
            captured_timestamp: captured_timestamp.to_string(),
            index: index.to_string(),
            snapshot_data: serialized_snapshot,
        };

        diesel::insert_into(vault_snapshots)
            .values(&snapshot_entry)
            .on_conflict((captured_timestamp, index))
            .do_update()
            .set(snapshot_data.eq(snapshot_entry.snapshot_data))
            .execute(connection)
            .expect("Failed to store vault snapshot");
    }

    fn get_vault_snapshot(&self, captured_timestamp: U256, index: U256) -> Option<VaultSnapshot> {
        use crate::schema::vault_snapshots::dsl::*;
        let connection = &mut self.pool.get().expect("DB connection failed");

        let result: Option<Vec<u8>> = vault_snapshots
            .filter(captured_timestamp.eq(captured_timestamp.to_string()))
            .filter(index.eq(index.to_string()))
            .select(snapshot_data)
            .first(connection)
            .ok();

        result.map(|data| bincode::deserialize(&data).expect("Failed to deserialize snapshot"))
    }
}

use crate::schema::slash_results;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "slash_results"]
pub struct SlashResultEntry {
    pub captured_timestamp: String,
    pub index: String,
    pub result_data: Vec<u8>,
}

pub trait SlashResultManagement {
    fn store_slash_result(&mut self, captured_timestamp: U256, index: U256, result: SlashResult);
}

impl SlashResultManagement for OperatorStakeStore {
    fn store_slash_result(&mut self, captured_timestamp: U256, index: U256, result: SlashResult) {
        use crate::schema::slash_results::dsl::*;
        let connection = &mut self.pool.get().expect("DB connection failed");

        let serialized_result = bincode::serialize(&result).expect("Failed to serialize slash result");

        let result_entry = SlashResultEntry {
            captured_timestamp: captured_timestamp.to_string(),
            index: index.to_string(),
            result_data: serialized_result,
        };

        diesel::insert_into(slash_results)
            .values(&result_entry)
            .on_conflict((captured_timestamp, index))
            .do_update()
            .set(result_data.eq(result_entry.result_data))
            .execute(connection)
            .expect("Failed to store slash result");
    }
}
