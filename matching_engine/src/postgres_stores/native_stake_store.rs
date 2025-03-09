// use async_trait::async_trait;
// use diesel::prelude::*;
// use diesel::insert_into;
// use crate::schema::native_staking_store::dsl::*;
// use ethers::types::Address;
// use serde_json;
// use std::str::FromStr;


// impl NativeStakingOperations for NativeStakingStoreDB {
//     async fn set_lock_token(&mut self, token: Address, amount: U256) {
//         let pool = self.pool.clone();
//         // Move token and amount into the closure.
//         tokio::task::spawn_blocking(move || {
//             let conn = &mut pool.get().expect("DB connection error");

//             // Try to load the single row with id = 1.
//             let record_opt: Option<NativeStakingRecord> = native_staking_store
//                 .filter(id.eq(1))
//                 .first(conn)
//                 .optional()
//                 .expect("Error querying native staking store");

//             // Deserialize or create a new tracker.
//             let mut tracker = if let Some(record) = record_opt {
//                 serde_json::from_str::<TokenTracker>(&record.tokens_to_lock)
//                     .unwrap_or_else(|_| TokenTracker::new())
//             } else {
//                 TokenTracker::new()
//             };

//             // Set the lock token.
//             tracker.force_set(token, amount);

//             // Serialize updated tracker.
//             let new_tokens = serde_json::to_string(&tracker)
//                 .expect("Error serializing token tracker");

//             // Upsert: update if exists, insert otherwise.
//             if record_opt.is_some() {
//                 diesel::update(native_staking_store.filter(id.eq(1)))
//                     .set(tokens_to_lock.eq(new_tokens))
//                     .execute(conn)
//                     .expect("Error updating native staking store");
//             } else {
//                 let new_record = NativeStakingRecord {
//                     id: 1,
//                     tokens_to_lock: new_tokens,
//                 };
//                 insert_into(native_staking_store)
//                     .values(&new_record)
//                     .execute(conn)
//                     .expect("Error inserting native staking store record");
//             }
//         }).await.expect("spawn_blocking failed");
//     }

//     async fn remove_lock_token(&mut self, token: Address) {
//         let pool = self.pool.clone();
//         tokio::task::spawn_blocking(move || {
//             let conn = &mut pool.get().expect("DB connection error");

//             // Load existing record with id = 1.
//             let record_opt: Option<NativeStakingRecord> = native_staking_store
//                 .filter(id.eq(1))
//                 .first(conn)
//                 .optional()
//                 .expect("Error querying native staking store");

//             if let Some(record) = record_opt {
//                 let mut tracker: TokenTracker = serde_json::from_str(&record.tokens_to_lock)
//                     .unwrap_or_else(|_| TokenTracker::new());
//                 // Remove the token.
//                 tracker.force_remove(token);

//                 let new_tokens = serde_json::to_string(&tracker)
//                     .expect("Error serializing token tracker");

//                 diesel::update(native_staking_store.filter(id.eq(1)))
//                     .set(tokens_to_lock.eq(new_tokens))
//                     .execute(conn)
//                     .expect("Error updating native staking store");
//             }
//         }).await.expect("spawn_blocking failed");
//     }

//     async fn tokens_to_lock(&self) -> TokenTracker {
//         let pool = self.pool.clone();
//         tokio::task::spawn_blocking(move || {
//             let conn = &mut pool.get().expect("DB connection error");
//             let record_opt: Option<NativeStakingRecord> = native_staking_store
//                 .filter(id.eq(1))
//                 .first(conn)
//                 .optional()
//                 .expect("Error querying native staking store");
//             if let Some(record) = record_opt {
//                 serde_json::from_str(&record.tokens_to_lock)
//                     .unwrap_or_else(|_| TokenTracker::new())
//             } else {
//                 TokenTracker::new()
//             }
//         }).await.expect("spawn_blocking failed")
//     }
// }
