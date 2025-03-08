// // tests/test_postgres_generator_store.rs

// use diesel::prelude::*;
// use diesel::r2d2::{ConnectionManager, Pool};
// use diesel::PgConnection;
// use diesel::result::Error;
// use std::env;
// use ethers::core::types::{U256, U64, Address, Bytes, H256};


// use my_crate::{
//     DieselGeneratorStore, Generator, GeneratorInfoPerMarket, NewGenerator, NewGeneratorMarket,
//     GeneratorRegistration, GeneratorStakeComputeManagement, Operation, Source,
// };
// use my_crate::schema::{generators, generator_markets};

// // ----------------------------------------------------------------
// // Helpers
// // ----------------------------------------------------------------

// /// Initializes a connection pool using the TEST_DATABASE_URL environment variable.
// /// For testing we limit the pool size to 1.
// fn init_test_pool() -> Pool<ConnectionManager<PgConnection>> {
//     let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
//     let manager = ConnectionManager::<PgConnection>::new(database_url);
//     Pool::builder().max_size(1).build(manager).expect("Failed to create pool")
// }

// /// Returns a sample `Generator` instance for testing.
// fn sample_generator() -> Generator {
//     Generator {
//         address: "0x0000000000000000000000000000000000000001".parse().unwrap(),
//         reward_address: "0x0000000000000000000000000000000000000002".parse().unwrap(),
//         total_native_stake: U256::from(1000),
//         total_symbiotic_stake: U256::from(2000),
//         sum_of_compute_allocations: U256::from(3000),
//         compute_consumed: U256::from(0),
//         // For stake locked fields, assume a U256 value represented as a string.
//         native_stake_locked: U256::from(0),
//         symbiotic_stake_locked: U256::from(0),
//         active_market_places: U256::from(1),
//         declared_compute: U256::from(1),
//         intended_stake_util: U256::from(500),
//         intended_compute_util: U256::from(600),
//         generator_data: Bytes::from("some data"),
//         active: true,
//         earnings: U256::from(0),
//         kalypso_points: U256::from(0),
//         jobs_missed_counter: 0,
//     }
// }

// /// Returns a sample `GeneratorInfoPerMarket` instance for testing.
// fn sample_generator_market() -> GeneratorInfoPerMarket {
//     GeneratorInfoPerMarket {
//         address: "0x0000000000000000000000000000000000000001".parse().unwrap(),
//         market_id: U256::from(123),
//         compute_required_per_request: U256::from(100),
//         proof_generation_cost: U256::from(50),
//         proposed_time: U256::from(1000),
//         active_requests: U256::from(10),
//         proofs_submitted: U256::from(5),
//         proofs_slashed: U256::from(2),
//         state: None, // or Some(YourState::...)
//     }
// }

// // ----------------------------------------------------------------
// // Tests for GeneratorRegistration trait (implemented for DieselGeneratorStore)
// // ----------------------------------------------------------------

// #[test]
// fn test_register_generator() {
//     let pool = init_test_pool();
//     // Get a connection for transaction isolation.
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         // Create an instance of DieselGeneratorStore using the pool.
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         let gen = sample_generator();
//         store.register_generator(gen.clone());

//         // Query the generators table to check if the record was inserted.
//         use my_crate::schema::generators::dsl::*;
//         let result: NewGenerator = generators
//             .filter(address.eq(format!("{:?}", gen.address)))
//             .first(&conn)
//             .expect("Failed to fetch generator");
//         // You can compare fields – here we check that the reward address matches.
//         assert_eq!(result.reward_address, format!("{:?}", gen.reward_address));
//         Ok(())
//     });
// }

// #[test]
// fn test_register_generator_in_market() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };
//         let gen_market = sample_generator_market();
//         store.register_generator_in_market(gen_market.clone());

//         // Query the generator_markets table.
//         use my_crate::schema::generator_markets::dsl::*;
//         let result: NewGeneratorMarket = generator_markets
//             .filter(generator_address.eq(format!("{:?}", gen_market.address)))
//             .filter(market_id.eq(gen_market.market_id.to_string()))
//             .first(&conn)
//             .expect("Failed to fetch generator market");
//         // Compare one field to ensure insertion.
//         assert_eq!(result.compute_required_per_request, gen_market.compute_required_per_request.to_string());
//         Ok(())
//     });
// }

// #[test]
// fn test_remove_by_address_and_market() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // First, register a generator market.
//         let gen_market = sample_generator_market();
//         store.register_generator_in_market(gen_market.clone());

//         // Now remove it.
//         store.remove_by_address_and_market(&gen_market.address, &gen_market.market_id);

//         // Verify deletion.
//         use my_crate::schema::generator_markets::dsl::*;
//         let count: i64 = generator_markets
//             .filter(generator_address.eq(format!("{:?}", gen_market.address)))
//             .filter(market_id.eq(gen_market.market_id.to_string()))
//             .count()
//             .get_result(&conn)
//             .expect("Count query failed");
//         assert_eq!(count, 0);
//         Ok(())
//     });
// }

// #[test]
// fn test_remove_by_address() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Register a generator.
//         let gen = sample_generator();
//         store.register_generator(gen.clone());
//         // Also, register a market record for the same generator.
//         let gen_market = sample_generator_market();
//         store.register_generator_in_market(gen_market);

//         // Remove the generator by address.
//         store.remove_by_address(&gen.address);

//         // Check that the generator record is deleted.
//         use my_crate::schema::generators::dsl::*;
//         let count_gen: i64 = generators
//             .filter(address.eq(format!("{:?}", gen.address)))
//             .count()
//             .get_result(&conn)
//             .unwrap_or(0);
//         assert_eq!(count_gen, 0);

//         // Check that related generator_market records are also deleted.
//         use my_crate::schema::generator_markets::dsl::*;
//         let count_market: i64 = generator_markets
//             .filter(generator_address.eq(format!("{:?}", gen.address)))
//             .count()
//             .get_result(&conn)
//             .unwrap_or(0);
//         assert_eq!(count_market, 0);
//         Ok(())
//     });
// }

// #[test]
// fn test_get_by_address_and_is_active() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         let mut gen = sample_generator();
//         // Ensure active is true.
//         gen.active = true;
//         store.register_generator(gen.clone());

//         // Get by address.
//         let fetched = store.get_by_address(&gen.address);
//         assert!(fetched.is_some());
//         let fetched_gen = fetched.unwrap();
//         assert_eq!(fetched_gen.reward_address.to_string(), gen.reward_address.to_string());

//         // Test is_active.
//         let active = store.is_active(&gen.address);
//         assert!(active);
//         Ok(())
//     });
// }

// // ----------------------------------------------------------------
// // Tests for GeneratorStakeComputeManagement trait
// // ----------------------------------------------------------------

// #[test]
// fn test_add_extra_stake_and_update_reward_address() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         // For this test, we use DieselGeneratorStore.
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Register a generator first.
//         let mut gen = sample_generator();
//         // Set an initial reward address.
//         gen.reward_address = "0x0000000000000000000000000000000000000002".parse().unwrap();
//         store.register_generator(gen.clone());

//         // For add_extra_stake, we assume the function updates token trackers and inserts a delegation.
//         // Note: The add_extra_stake implementation uses a helper to update the tracker.
//         // For this test, we might assume that helper returns a new value.
//         // (If the helper isn’t implemented, you may need to adjust the test.)
//         let token_addr: Address = "0x0000000000000000000000000000000000000003".parse().unwrap();
//         let amount = U256::from(100);
//         let block_num = U64::from(50);
//         let tx_index = U64::from(1);
//         let log_index = U256::from(0);
//         let tx_hash = "0xabcdef".to_string();
//         // For the source, assume Source is an enum with variant Native.
//         store.add_extra_stake(
//             &gen.address,
//             &token_addr,
//             &amount,
//             block_num,
//             tx_index,
//             log_index,
//             tx_hash.clone(),
//             Source::Native,
//         );

//         // Verify that a delegation record was inserted.
//         use my_crate::schema::delegations::dsl::*;
//         let delegation_count: i64 = delegations
//             .filter(generator_address.eq(format!("{:?}", gen.address)))
//             .filter(delegated_address.eq(format!("{:?}", token_addr)))
//             .count()
//             .get_result(&conn)
//             .expect("Failed to count delegations");
//         assert!(delegation_count > 0, "Expected at least one delegation record");

//         // Test update_reward_address.
//         let new_reward: Address = "0x000000000000000000000000000000000000dead".parse().unwrap();
//         store.update_reward_address(&gen.address, new_reward);
//         // Verify update in generators table.
//         use my_crate::schema::generators::dsl::*;
//         let updated_reward: String = generators
//             .filter(address.eq(format!("{:?}", gen.address)))
//             .select(reward_address)
//             .first(&conn)
//             .expect("Failed to fetch updated reward address");
//         assert_eq!(updated_reward, format!("{:?}", new_reward));
//         Ok(())
//     });
// }

// fn test_add_extra_compute() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         // Create a DieselGeneratorStore using the pool.
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Register a generator first.
//         let mut gen = sample_generator();
//         // Ensure declared_compute is known.
//         // (For example, initial declared_compute = 100)
//         diesel::insert_into(generators::table)
//             .values(&gen.clone().into())  // assuming From<Generator> for NewGenerator exists
//             .execute(&conn)?;

//         // Call add_extra_compute to add, say, 50 extra compute.
//         let extra = U256::from(50);
//         store.add_extra_compute(&gen.address, extra);

//         // Query back the declared_compute value.
//         use my_crate::schema::generators::dsl::*;
//         let updated: String = generators
//             .filter(address.eq(format!("{:?}", gen.address)))
//             .select(declared_compute)
//             .first(&conn)?;
//         let updated_compute = U256::from_dec_str(&updated).unwrap();
//         assert_eq!(updated_compute, U256::from(150)); // 100 + 50
//         Ok(())
//     });
// }

// #[test]
// fn test_update_intended_compute_util() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         let mut gen = sample_generator();
//         diesel::insert_into(generators::table)
//             .values(&gen.clone().into())
//             .execute(&conn)?;

//         let new_util = U256::from(800);
//         store.update_intended_compute_util(&gen.address, new_util);

//         use my_crate::schema::generators::dsl::*;
//         let updated: String = generators
//             .filter(address.eq(format!("{:?}", gen.address)))
//             .select(intended_compute_util)
//             .first(&conn)?;
//         assert_eq!(updated, new_util.to_string());
//         Ok(())
//     });
// }

// #[test]
// fn test_remove_compute() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert a generator with declared_compute = 200.
//         let mut gen = sample_generator();
//         gen.declared_compute = U256::from(200);
//         diesel::insert_into(generators::table)
//             .values(&gen.clone().into())
//             .execute(&conn)?;

//         // Remove 50 compute.
//         store.remove_compute(&gen.address, U256::from(50));

//         use my_crate::schema::generators::dsl::*;
//         let updated: String = generators
//             .filter(address.eq(format!("{:?}", gen.address)))
//             .select(declared_compute)
//             .first(&conn)?;
//         let updated_compute = U256::from_dec_str(&updated).unwrap();
//         assert_eq!(updated_compute, U256::from(150)); // 200 - 50
//         Ok(())
//     });
// }

// // ----------------------------------------------------------------
// // Tests for GeneratorMarketManagement trait
// // ----------------------------------------------------------------

// #[test]
// fn test_update_state_market() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert a generator market record.
//         let gm = sample_generator_market();
//         diesel::insert_into(generator_markets::table)
//             .values(&gm.into())  // assuming From<GeneratorInfoPerMarket> for NewGeneratorMarket exists
//             .execute(&conn)?;

//         // Update state to a new value.
//         store.update_state(&gm.address, &gm.market_id, GeneratorState::Paused);
//         // Query back the state.
//         use my_crate::schema::generator_markets::dsl::*;
//         let updated: Option<String> = generator_markets
//             .filter(generator_address.eq(format!("{:?}", gm.address)))
//             .filter(market_id.eq(gm.market_id.to_string()))
//             .select(state)
//             .first(&conn)
//             .ok()
//             .flatten();
//         assert_eq!(updated, Some(GeneratorState::Paused.to_string()));
//         Ok(())
//     });
// }

// #[test]
// fn test_update_on_assigned_task() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert a generator market record with active_requests = 10.
//         let mut gm = sample_generator_market();
//         gm.active_requests = U256::from(10);
//         diesel::insert_into(generator_markets::table)
//             .values(&gm.clone().into())
//             .execute(&conn)?;

//         // Call update_on_assigned_task (which should increment active_requests by 1).
//         store.update_on_assigned_task(&gm.address, &gm.market_id);

//         use my_crate::schema::generator_markets::dsl::*;
//         let updated: String = generator_markets
//             .filter(generator_address.eq(format!("{:?}", gm.address)))
//             .filter(market_id.eq(gm.market_id.to_string()))
//             .select(active_requests)
//             .first(&conn)?;
//         let updated_requests = U256::from_dec_str(&updated).unwrap();
//         assert_eq!(updated_requests, U256::from(11));
//         Ok(())
//     });
// }

// #[test]
// fn test_update_on_submit_proof() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert a generator market record.
//         let mut gm = sample_generator_market();
//         // Set initial values.
//         gm.active_requests = U256::from(10);
//         gm.proofs_submitted = U256::from(2);
//         gm.earnings = U256::from(100);
//         gm.kalypso_points = U256::from(20);
//         diesel::insert_into(generator_markets::table)
//             .values(&gm.clone().into())
//             .execute(&conn)?;

//         // Also insert a generator record for overall updates.
//         let mut gen = sample_generator();
//         gen.earnings = U256::from(50);
//         gen.kalypso_points = U256::from(10);
//         diesel::insert_into(generators::table)
//             .values(&gen.clone().into())
//             .execute(&conn)?;

//         let earning = U256::from(25);
//         let block_num = U64::from(100);
//         // Assume get_points(block_number) returns, say, 5.
//         // (Ensure that your production get_points function is deterministic for tests.)
//         store.update_on_submit_proof(&gm.address, &gm.market_id, &earning, &block_num);

//         // Query generator_markets for updated fields.
//         use my_crate::schema::generator_markets::dsl::*;
//         let gm_updated: (String, String, String, String) = generator_markets
//             .filter(generator_address.eq(format!("{:?}", gm.address)))
//             .filter(market_id.eq(gm.market_id.to_string()))
//             .select((active_requests, proofs_submitted, earnings, kalypso_points))
//             .first(&conn)?;
//         let new_active_requests = U256::from_dec_str(&gm_updated.0).unwrap();
//         let new_proofs_submitted = U256::from_dec_str(&gm_updated.1).unwrap();
//         let new_market_earnings = U256::from_dec_str(&gm_updated.2).unwrap();
//         let new_market_kalypso_points = U256::from_dec_str(&gm_updated.3).unwrap();
//         // Expect active_requests decreased by 1 (10-1 = 9) and proofs_submitted increased by 1 (2+1 = 3).
//         assert_eq!(new_active_requests, U256::from(9));
//         assert_eq!(new_proofs_submitted, U256::from(3));
//         // Also, overall earnings and kalypso points in generator record should be updated.
//         use my_crate::schema::generators::dsl::*;
//         let gen_updated: (String, String) = generators
//             .filter(address.eq(format!("{:?}", gen.address)))
//             .select((earnings, kalypso_points))
//             .first(&conn)?;
//         let new_total_earnings = U256::from_dec_str(&gen_updated.0).unwrap();
//         let new_total_kalypso_points = U256::from_dec_str(&gen_updated.1).unwrap();
//         assert_eq!(new_total_earnings, U256::from(75)); // 50+25
//         // Assuming get_points returns 5.
//         assert_eq!(new_total_kalypso_points, U256::from(15)); // 10+5
//         Ok(())
//     });
// }

// #[test]
// fn test_reduce_active_requests() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert a generator market record with active_requests = 10 and proofs_slashed = 2.
//         let mut gm = sample_generator_market();
//         gm.active_requests = U256::from(10);
//         gm.proofs_slashed = U256::from(2);
//         diesel::insert_into(generator_markets::table)
//             .values(&gm.clone().into())
//             .execute(&conn)?;

//         // Call reduce_active_requests.
//         store.reduce_active_requests(&gm.address, &gm.market_id);

//         use my_crate::schema::generator_markets::dsl::*;
//         let updated: (String, String) = generator_markets
//             .filter(generator_address.eq(format!("{:?}", gm.address)))
//             .filter(market_id.eq(gm.market_id.to_string()))
//             .select((active_requests, proofs_slashed))
//             .first(&conn)?;
//         let new_active_requests = U256::from_dec_str(&updated.0).unwrap();
//         let new_proofs_slashed = U256::from_dec_str(&updated.1).unwrap();
//         // Expect active_requests decreased by 1 and proofs_slashed increased by 1.
//         assert_eq!(new_active_requests, U256::from(9));
//         assert_eq!(new_proofs_slashed, U256::from(3));
//         Ok(())
//     });
// }

// #[test]
// fn test_pause_and_resume_assignments_across_all_markets() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert two generator market records for the same generator.
//         let gm1 = sample_generator_market();
//         let mut gm2 = sample_generator_market();
//         gm2.market_id = U256::from(456);
//         diesel::insert_into(generator_markets::table)
//             .values(&gm1.clone().into())
//             .execute(&conn)?;
//         diesel::insert_into(generator_markets::table)
//             .values(&gm2.clone().into())
//             .execute(&conn)?;

//         // Pause assignments: state should be set to PendingConfirmation.
//         store.pause_assignments_across_all_markets(&gm1.address);
//         use my_crate::schema::generator_markets::dsl::*;
//         let states: Vec<Option<String>> = generator_markets
//             .filter(generator_address.eq(format!("{:?}", gm1.address)))
//             .select(state)
//             .load(&conn)?;
//         for st in states {
//             assert_eq!(st, Some(GeneratorState::PendingConfirmation.to_string()));
//         }

//         // Resume assignments: state should be set to Joined.
//         store.resume_assignments_accross_all_markets(&gm1.address);
//         let states: Vec<Option<String>> = generator_markets
//             .filter(generator_address.eq(format!("{:?}", gm1.address)))
//             .select(state)
//             .load(&conn)?;
//         for st in states {
//             assert_eq!(st, Some(GeneratorState::Joined.to_string()));
//         }
//         Ok(())
//     });
// }

// // ----------------------------------------------------------------
// // Tests for GeneratorLockManagement trait (compute locked/released)
// // ----------------------------------------------------------------

// #[test]
// fn test_update_on_compute_locked() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert a generator with compute_consumed = 50.
//         let mut gen = sample_generator();
//         gen.compute_consumed = U256::from(50);
//         diesel::insert_into(generators::table)
//             .values(&gen.clone().into())
//             .execute(&conn)?;

//         // Call update_on_compute_locked to add 30.
//         store.update_on_compute_locked(&gen.address, U256::from(30));
//         use my_crate::schema::generators::dsl::*;
//         let updated: String = generators
//             .filter(address.eq(format!("{:?}", gen.address)))
//             .select(compute_consumed)
//             .first(&conn)?;
//         let new_compute = U256::from_dec_str(&updated).unwrap();
//         assert_eq!(new_compute, U256::from(80)); // 50 + 30
//         Ok(())
//     });
// }

// #[test]
// fn test_update_on_compute_released() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert a generator with compute_consumed = 80.
//         let mut gen = sample_generator();
//         gen.compute_consumed = U256::from(80);
//         diesel::insert_into(generators::table)
//             .values(&gen.clone().into())
//             .execute(&conn)?;

//         // Call update_on_compute_released to subtract 20.
//         store.update_on_compute_released(&gen.address, U256::from(20));
//         use my_crate::schema::generators::dsl::*;
//         let updated: String = generators
//             .filter(address.eq(format!("{:?}", gen.address)))
//             .select(compute_consumed)
//             .first(&conn)?;
//         let new_compute = U256::from_dec_str(&updated).unwrap();
//         assert_eq!(new_compute, U256::from(60)); // 80 - 20
//         Ok(())
//     });
// }

// // ----------------------------------------------------------------
// // Tests for GeneratorMetadata trait
// // ----------------------------------------------------------------

// #[test]
// fn test_update_generator_metadata() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert a generator with initial generator_data.
//         let mut gen = sample_generator();
//         gen.generator_data = Bytes::from("old metadata");
//         diesel::insert_into(generators::table)
//             .values(&gen.clone().into())
//             .execute(&conn)?;

//         // Update the metadata.
//         let new_meta = Bytes::from("new metadata");
//         store.update_generator_metadata(gen.address, new_meta.clone());

//         use my_crate::schema::generators::dsl::*;
//         let updated: Vec<u8> = generators
//             .filter(address.eq(format!("{:?}", gen.address)))
//             .select(generator_data)
//             .first(&conn)?;
//         assert_eq!(updated, new_meta.to_vec());
//         Ok(())
//     });
// }

// // ----------------------------------------------------------------
// // Tests for JobMissedCounter trait
// // ----------------------------------------------------------------

// #[test]
// fn test_job_missed_counter() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let mut store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert a generator with jobs_missed_counter = 0.
//         let mut gen = sample_generator();
//         gen.jobs_missed_counter = 0;
//         diesel::insert_into(generators::table)
//             .values(&gen.clone().into())
//             .execute(&conn)?;

//         // Increment the counter.
//         store.count_job_missed_by_generator(gen.address, std::time::SystemTime::now());
//         // Verify via get_job_missed_count.
//         let count = store.get_job_missed_count(&gen.address);
//         assert_eq!(count, 1);
//         Ok(())
//     });
// }

// // ----------------------------------------------------------------
// // Tests for GeneratorAdditionalQuery trait
// // ----------------------------------------------------------------

// #[test]
// fn test_get_by_address_and_market() {
//     let pool = init_test_pool();
//     let conn = pool.get().unwrap();
//     conn.test_transaction::<_, Error, _>(|| {
//         let store = DieselGeneratorStore { pool: pool.clone() };

//         // Insert a generator market record.
//         let gm = sample_generator_market();
//         diesel::insert_into(generator_markets::table)
//             .values(&gm.clone().into())
//             .execute(&conn)?;

//         // Retrieve by address and market.
//         let fetched = store.get_by_address_and_market(&gm.address, &gm.market_id);
//         assert!(fetched.is_some());
//         let fetched_market = fetched.unwrap();
//         assert_eq!(fetched_market.market_id, gm.market_id);
//         Ok(())
//     });
// }