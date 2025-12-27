// tests/test_postgres_askstore.rs

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::PgConnection;
use dotenv::dotenv;
use ethers::core::types::{Address, Bytes, U256};
use matching_engine::ask_lib::ask_store::{AskManagementRead, AskManagementWrite};
use std::env;

use matching_engine::ask_lib::ask::LocalAsk;
use matching_engine::ask_lib::ask_status::AskState;
use matching_engine::postgres_stores::models::*;
use matching_engine::schema::ask_records::dsl::*;

/// Initializes a connection pool using the TEST_DATABASE_URL environment variable.
/// The pool is configured to have a maximum of 1 connection.
fn init_test_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool")
}

/// Returns a sample `LocalAsk` with default values, parameterized by the given id.
/// This helper lets all tests use consistent initial data.
fn sample_local_ask(id: u64) -> LocalAsk {
    LocalAsk {
        ask_id: U256::from(id),
        market_id: U256::from(100 + id),
        reward: U256::from(500 + id),
        expiry: U256::from(1000 + id),
        deadline: U256::from(2000 + id),
        time_requested_for_proof_generation: U256::from(3000 + id),
        prover_refund_address: "0x0000000000000000000000000000000000000001"
            .parse()
            .unwrap(),
        prover_data: Bytes::from(Vec::from("data".as_bytes())),
        has_private_inputs: false,
        secret_data: None,
        secret_acl: None,
        state: Some(AskState::Null),
        generator: None,
        invalid_secret_flag: false,
        created_on: U256::from(4000 + id),
        created_on_l1: U256::from(5000 + id),
        create_transaction: "0x0000000000000000000000000000000000000000000000000000000000000001"
            .parse()
            .unwrap(),
    }
}

/// Test database connection
#[test]
fn test_connection() {
    dotenv().ok(); // Load .env file if present
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    match PgConnection::establish(&database_url) {
        Ok(_) => println!("Database connection successful!"),
        Err(e) => eprintln!("Database connection failed: {:?}", e),
    }
}

/// Test inserting a LocalAsk and retrieving it by ask_id.
#[test]
fn test_insert_and_get_by_ask_id() {
    let pool = init_test_pool();
    let mut conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|_| {
        let private_store = PrivateInputStore::new();
        let mut db = AskDatabase {
            pool: pool.clone(),
            private_store,
        };

        let ask = sample_local_ask(1);
        db.insert(ask.clone());

        let fetched = db.get_by_ask_id(&ask.ask_id);
        assert!(fetched.is_some(), "Expected to retrieve an ask");
        let fetched_ask = fetched.unwrap();
        assert_eq!(fetched_ask.ask_id, ask.ask_id);
        Ok(())
    });
}

#[test]
fn test_insert_and_get_by_ask_id1() {
    let pool = init_test_pool();
    let private_store = PrivateInputStore::new();
    let mut db = AskDatabase {
        pool: pool.clone(),
        private_store,
    };

    let ask = sample_local_ask(1);
    db.insert(ask.clone());

    // Verify the record was inserted.
    let fetched = db.get_by_ask_id(&ask.ask_id);
    assert!(fetched.is_some(), "Expected to retrieve an ask");
    let fetched_ask = fetched.unwrap();
    assert_eq!(fetched_ask.ask_id, ask.ask_id);

    // Cleanup: delete the inserted record.
    {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::delete(ask_records)
            .execute(&mut conn)
            .expect("Failed to delete inserted record");
    }

    // Verify the record was deleted.
    let fetched_after_delete = db.get_by_ask_id(&ask.ask_id);
    assert!(
        fetched_after_delete.is_none(),
        "Expected no record after deletion"
    );
}

// Test modifying the state of an ask.
#[test]
fn test_modify_state() {
    let pool = init_test_pool();
    let private_store = PrivateInputStore::new();
    let mut db = AskDatabase {
        pool: pool.clone(),
        private_store,
    };

    let ask = sample_local_ask(2);
    db.insert(ask.clone());

    // Change the state to Completed.
    db.modify_state(&ask.ask_id, AskState::Complete);
    let fetched = db.get_by_ask_id(&ask.ask_id).unwrap();
    assert!(fetched.state.is_some(), "State should be set");
    assert_eq!(fetched.state.unwrap() as u8, AskState::Complete as u8);
    // Cleanup: delete the inserted record.
    {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::delete(ask_records)
            .execute(&mut conn)
            .expect("Failed to delete inserted record");
    }

    // Verify the record was deleted.
    let fetched_after_delete = db.get_by_ask_id(&ask.ask_id);
    assert!(
        fetched_after_delete.is_none(),
        "Expected no record after deletion"
    );
}

// /// Test updating the generator for an ask.
#[test]
fn test_update_ask_generator() {
    let pool = init_test_pool();
    let private_store = PrivateInputStore::new();
    let mut db = AskDatabase {
        pool: pool.clone(),
        private_store,
    };

    let mut ask = sample_local_ask(3);
    // Initially, no generator is set.
    ask.generator = None;
    db.insert(ask.clone());

    // Update generator address.
    let new_gen: Address = "0x000000000000000000000000000000000000dead"
        .parse()
        .unwrap();
    db.update_ask_generator(&ask.ask_id, Some(new_gen));
    let fetched = db.get_by_ask_id(&ask.ask_id).unwrap();
    assert!(fetched.generator.is_some(), "Generator should be set");
    assert_eq!(fetched.generator.unwrap(), new_gen);

    // Cleanup: delete the inserted record.
    {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::delete(ask_records)
            .execute(&mut conn)
            .expect("Failed to delete inserted record");
    }

    // Verify the record was deleted.
    let fetched_after_delete = db.get_by_ask_id(&ask.ask_id);
    assert!(
        fetched_after_delete.is_none(),
        "Expected no record after deletion"
    );
}

/// Test updating the ask ACL in the private store.
#[test]
fn test_update_ask_acl() {
    let pool = init_test_pool();
    // This test uses only the in-memory private store.
    let private_store = PrivateInputStore::new();
    let mut db = AskDatabase {
        pool: pool.clone(),
        private_store,
    };

    let _ask_id = U256::from(4);
    // Insert initial private inputs.
    db.private_store.insert(
        _ask_id,
        AskPrivateInputs {
            secret_data: Some(vec![1, 2, 3]),
            secret_acl: Some(vec![4, 5, 6]),
        },
    );

    // Update the ACL.
    let new_acl = Bytes::from(vec![7, 8, 9]);
    db.update_ask_acl(&_ask_id, Some(new_acl.clone()));

    // Verify that the ACL in the private store is updated.
    let entry = db.private_store.get(&_ask_id).unwrap();
    assert_eq!(entry.secret_acl, Some(new_acl.to_vec()));
}

/// Test updating the deadline of an ask.
#[test]
fn test_update_deadline() {
    let pool = init_test_pool();
    let private_store = PrivateInputStore::new();
    let mut db = AskDatabase {
        pool: pool.clone(),
        private_store,
    };

    let ask = sample_local_ask(5);
    db.insert(ask.clone());

    let new_deadline = U256::from(9999);
    db.update_deadline(&ask.ask_id, new_deadline);
    let fetched = db.get_by_ask_id(&ask.ask_id).unwrap();
    assert_eq!(fetched.deadline, new_deadline);

    {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::delete(ask_records)
            .execute(&mut conn)
            .expect("Failed to delete inserted record");
    }

    // Verify the record was deleted.
    let fetched_after_delete = db.get_by_ask_id(&ask.ask_id);
    assert!(
        fetched_after_delete.is_none(),
        "Expected no record after deletion"
    );
}

/// Test storing a valid proof.
#[test]
fn test_store_valid_proof() {
    let pool = init_test_pool();
    let private_store = PrivateInputStore::new();
    let mut db = AskDatabase {
        pool: pool.clone(),
        private_store,
    };

    let ask = sample_local_ask(6);
    db.insert(ask.clone());

    let new_proof = Bytes::from(Vec::from("valid proof".as_bytes()));
    let new_proof_time = U256::from(1111);
    let new_proof_cost = U256::from(2222);
    let new_proof_tx = "0xprooftransaction".to_string();

    db.store_valid_proof(
        &ask.ask_id,
        new_proof.clone(),
        new_proof_time,
        new_proof_cost,
        new_proof_tx.clone(),
    );

    // proof fetching will require modification in the model.rs file

    // Fetch the record directly using Diesel to verify the proof fields.
    {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::delete(ask_records)
            .execute(&mut conn)
            .expect("Failed to delete inserted record");
    }

    // Verify the record was deleted.
    let fetched_after_delete = db.get_by_ask_id(&ask.ask_id);
    assert!(
        fetched_after_delete.is_none(),
        "Expected no record after deletion"
    );
    // (Additional assertions for proving_time_taken, proving_cost_taken, and proof_transaction can be added.)
}
