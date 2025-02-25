use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use ethers::types::{Address, Bytes};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use async_trait::async_trait;
use crate::schema::key_store;

/// Represents a key entry in the database.
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[table_name = "key_store"]
pub struct Key {
    pub address: Vec<u8>, // Store Address as a byte array
    pub key_index: i64,
    pub ecies_pub_key: Option<Vec<u8>>,
}

impl Key {
    pub fn to_key_info(&self) -> KeyInfo {
        KeyInfo {
            address: format!("0x{}", hex::encode(&self.address)),
            key_index: self.key_index.to_string(),
            ecies_pub_key: self.ecies_pub_key.as_ref().map(|pk| hex::encode(pk)),
        }
    }
}

/// A structured format for returning key information.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct KeyInfo {
    pub address: String,
    pub key_index: String,
    pub ecies_pub_key: Option<String>,
}

/// KeyStore structure using Diesel and PostgreSQL.
pub struct KeyStore {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl KeyStore {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        Self { pool }
    }
}

#[async_trait]
pub trait KeyStoreOperations {
    async fn insert(&mut self, address: Address, value: u64, key: Key);
    async fn get_by_address(&self, address: &Address, value: u64) -> Option<Key>;
    async fn remove_by_address(&mut self, address: &Address, value: u64);
    async fn update_pub_key(&mut self, address: &Address, value: u64, new_pub_key: Option<Bytes>);
}

#[async_trait]
impl KeyStoreOperations for KeyStore {
    async fn insert(&mut self, address: Address, value: u64, key: Key) {
        use crate::schema::key_store::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");

        let new_key = Key {
            address: address.as_bytes().to_vec(),
            key_index: value as i64,
            ecies_pub_key: key.ecies_pub_key.clone().map(|b| b.to_vec()),
        };

        diesel::insert_into(key_store)
            .values(&new_key)
            .on_conflict((address, key_index))
            .do_update()
            .set(ecies_pub_key.eq(new_key.ecies_pub_key))
            .execute(connection)
            .expect("Failed to insert or update key");
    }

    async fn get_by_address(&self, address: &Address, value: u64) -> Option<Key> {
        use crate::schema::key_store::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");
        key_store
            .filter(address.eq(address.as_bytes().to_vec()))
            .filter(key_index.eq(value as i64))
            .first::<Key>(connection)
            .ok()
    }

    async fn remove_by_address(&mut self, address: &Address, value: u64) {
        use crate::schema::key_store::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");
        diesel::delete(key_store.filter(address.eq(address.as_bytes().to_vec())).filter(key_index.eq(value as i64)))
            .execute(connection)
            .expect("Failed to delete key");
    }

    async fn update_pub_key(&mut self, address: &Address, value: u64, new_pub_key: Option<Bytes>) {
        use crate::schema::key_store::dsl::*;

        let connection = &mut self.pool.get().expect("Failed to get DB connection");

        diesel::update(key_store.filter(address.eq(address.as_bytes().to_vec())).filter(key_index.eq(value as i64)))
            .set(ecies_pub_key.eq(new_pub_key.map(|b| b.to_vec())))
            .execute(connection)
            .expect("Failed to update ECIES public key");
    }
}
