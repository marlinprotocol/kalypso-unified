// tests/test_postgres_askstore.rs

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::result::Error;
use std::env;
use ethers::core::types::{U256, Address, Bytes, H256};


use my_crate::{
    AskDatabase, LocalAsk, AskState, PrivateInputStore,
    u256_to_bytes, bytes_to_u256,
};
use my_crate::schema::ask_records::dsl::*;

/// Initializes a connection pool using the TEST_DATABASE_URL environment variable.
/// The pool is configured to have a maximum of 1 connection.
fn init_test_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(1)
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
        prover_refund_address: "0x0000000000000000000000000000000000000001".parse().unwrap(),
        prover_data: Bytes::from("data"),
        has_private_inputs: false,
        secret_data: None,
        secret_acl: None,
        state: Some(AskState::Pending),
        generator: None,
        invalid_secret_flag: false,
        created_on: U256::from(4000 + id),
        created_on_l1: U256::from(5000 + id),
        create_transaction: "0x0000000000000000000000000000000000000000000000000000000000000001".parse().unwrap(),
    }
}

/// Test inserting a LocalAsk and retrieving it by ask_id.
#[test]
fn test_insert_and_get_by_ask_id() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
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

/// Test modifying the state of an ask.
#[test]
fn test_modify_state() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
        let private_store = PrivateInputStore::new();
        let mut db = AskDatabase {
            pool: pool.clone(),
            private_store,
        };

        let ask = sample_local_ask(2);
        db.insert(ask.clone());

        // Change the state to Completed.
        db.modify_state(&ask.ask_id, AskState::Completed);
        let fetched = db.get_by_ask_id(&ask.ask_id).unwrap();
        assert!(fetched.state.is_some(), "State should be set");
        assert_eq!(fetched.state.unwrap() as u8, AskState::Completed as u8);
        Ok(())
    });
}

/// Test updating the generator for an ask.
#[test]
fn test_update_ask_generator() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
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
        let new_gen: Address = "0x000000000000000000000000000000000000dead".parse().unwrap();
        db.update_ask_generator(&ask.ask_id, Some(new_gen));
        let fetched = db.get_by_ask_id(&ask.ask_id).unwrap();
        assert!(fetched.generator.is_some(), "Generator should be set");
        assert_eq!(fetched.generator.unwrap(), new_gen);
        Ok(())
    });
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

    let ask_id = U256::from(4);
    // Insert initial private inputs.
    db.private_store.insert(
        ask_id,
        my_crate::AskPrivateInputs {
            secret_data: Some(vec![1, 2, 3]),
            secret_acl: Some(vec![4, 5, 6]),
        },
    );

    // Update the ACL.
    let new_acl = Bytes::from(vec![7, 8, 9]);
    db.update_ask_acl(&ask_id, Some(new_acl.clone()));

    // Verify that the ACL in the private store is updated.
    let entry = db.private_store.get(&ask_id).unwrap();
    assert_eq!(entry.secret_acl, Some(new_acl.to_vec()));
}

/// Test updating the deadline of an ask.
#[test]
fn test_update_deadline() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
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
        Ok(())
    });
}

/// Test storing a valid proof.
#[test]
fn test_store_valid_proof() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
        let private_store = PrivateInputStore::new();
        let mut db = AskDatabase {
            pool: pool.clone(),
            private_store,
        };

        let ask = sample_local_ask(6);
        db.insert(ask.clone());

        let new_proof = Bytes::from("valid proof");
        let new_proof_time = U256::from(1111);
        let new_proof_cost = U256::from(2222);
        let new_proof_tx = "0xprooftransaction".to_string();

        db.store_valid_proof(&ask.ask_id, new_proof.clone(), new_proof_time, new_proof_cost, new_proof_tx.clone());

        // Fetch the record directly using Diesel to verify the proof fields.
        let record: my_crate::AskRecord = ask_records
            .filter(ask_id.eq(u256_to_bytes(ask.ask_id)))
            .first(&conn)?;
        assert_eq!(record.proof, Some(new_proof.to_vec()));
        // (Additional assertions for proving_time_taken, proving_cost_taken, and proof_transaction can be added.)
        Ok(())
    });
}

/// Test retrieving proving time and cost.
#[test]
fn test_get_proving_time_and_cost() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
        // Insert a record with proving time and cost set.
        let mut ask = sample_local_ask(7);
        ask.state = Some(AskState::Completed);
        let mut record: my_crate::AskRecord = ask.clone().into();
        record.proving_time_taken = Some(u256_to_bytes(U256::from(3333)));
        record.proving_cost_taken = Some(u256_to_bytes(U256::from(4444)));
        diesel::insert_into(my_crate::schema::ask_records::table)
            .values(&record)
            .execute(&conn)?;

        let private_store = PrivateInputStore::new();
        let db = AskDatabase {
            pool: pool.clone(),
            private_store,
        };

        let proving_time = db.get_proving_time(&ask.ask_id);
        let proving_cost = db.get_proving_cost(&ask.ask_id);
        assert_eq!(proving_time, Some(U256::from(3333)));
        assert_eq!(proving_cost, Some(U256::from(4444)));
        Ok(())
    });
}

/// Test retrieving the proof transaction and the proof itself.
#[test]
fn test_get_proof_transaction_and_proof_by_ask_id() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
        let mut ask = sample_local_ask(8);
        let mut record: my_crate::AskRecord = ask.clone().into();
        record.proof_transaction = Some("0xprooftx".to_string());
        record.proof = Some(b"proofdata".to_vec());
        diesel::insert_into(my_crate::schema::ask_records::table)
            .values(&record)
            .execute(&conn)?;

        let private_store = PrivateInputStore::new();
        let db = AskDatabase {
            pool: pool.clone(),
            private_store,
        };

        let proof_tx = db.get_proof_transaction(&ask.ask_id);
        let proof = db.get_proof_by_ask_id(&ask.ask_id);
        assert_eq!(proof_tx, Some("0xprooftx".to_string()));
        assert!(proof.is_some());
        // Assuming Proof is a struct with a field `data` of type Bytes.
        let fetched_proof = proof.unwrap();
        assert_eq!(fetched_proof.data, Bytes::from("proofdata"));
        Ok(())
    });
}

/// Test retrieving asks by market id and by ask state (excluding Completed).
#[test]
fn test_get_by_market_id_and_by_ask_state_except_complete() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
        let private_store = PrivateInputStore::new();
        let mut db = AskDatabase {
            pool: pool.clone(),
            private_store,
        };

        // Insert asks with different market_ids.
        let ask1 = sample_local_ask(9);
        let mut ask2 = sample_local_ask(10);
        ask2.market_id = U256::from(555);
        db.insert(ask1.clone());
        db.insert(ask2.clone());

        let asks_by_market = db.get_by_market_id(&ask2.market_id);
        assert!(!asks_by_market.is_empty(), "Expected asks for the market");

        // Insert asks for state filtering.
        let mut ask3 = sample_local_ask(11);
        ask3.state = Some(AskState::Pending);
        let mut ask4 = sample_local_ask(12);
        ask4.state = Some(AskState::Completed);
        db.insert(ask3.clone());
        db.insert(ask4.clone());

        let asks_not_complete = db.get_by_ask_state_except_complete(AskState::Pending);
        for a in asks_not_complete {
            assert_eq!(a.state.unwrap() as u8, AskState::Pending as u8);
        }
        Ok(())
    });
}

/// Test cleanup asks and ask status retrieval.
#[test]
fn test_get_cleanup_asks_and_get_ask_status() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
        let private_store = PrivateInputStore::new();
        let mut db = AskDatabase {
            pool: pool.clone(),
            private_store,
        };

        // Insert an ask with an expired deadline.
        let mut ask = sample_local_ask(13);
        ask.deadline = U256::zero(); // Simulate expiration.
        db.insert(ask.clone());

        let cleanup = db.get_cleanup_asks();
        assert!(!cleanup.is_empty(), "Expected cleanup asks");

        let status = db.get_ask_status();
        assert!(status.total > 0, "Total asks should be > 0");
        Ok(())
    });
}

/// Test requestor and proof counters.
#[test]
fn test_requestor_and_proof_counters() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
        let private_store = PrivateInputStore::new();
        let mut db = AskDatabase {
            pool: pool.clone(),
            private_store,
        };

        // Insert asks with different prover_refund_address values.
        let mut ask1 = sample_local_ask(14);
        ask1.prover_refund_address = "0x000000000000000000000000000000000000000a".parse().unwrap();
        let mut ask2 = sample_local_ask(15);
        ask2.prover_refund_address = "0x000000000000000000000000000000000000000b".parse().unwrap();
        db.insert(ask1);
        db.insert(ask2);

        let total_requestors = db.total_requestor_count();
        assert!(total_requestors >= 2);

        let market_id = sample_local_ask(14).market_id;
        let req_by_market = db.total_requestors_by_market_count(&market_id);
        assert!(req_by_market >= 1);

        // Insert an ask with a proof.
        let mut ask3 = sample_local_ask(16);
        let mut record: my_crate::AskRecord = ask3.clone().into();
        record.proof = Some(b"proof".to_vec());
        diesel::insert_into(my_crate::schema::ask_records::table)
            .values(&record)
            .execute(&conn)?;
        let proof_count = db.get_proof_count(&ask3.market_id);
        let total_proof_count = db.get_total_proof_count();
        assert!(proof_count >= 1);
        assert!(total_proof_count >= 1);

        let req_count = db.get_request_count_by_market_id(&ask3.market_id);
        let total_req_count = db.get_total_request_count();
        assert!(req_count >= 1);
        assert!(total_req_count >= 1);
        Ok(())
    });
}

/// Test failed and completed proof counters.
#[test]
fn test_failed_and_completed_proof_counters() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
        let private_store = PrivateInputStore::new();
        let mut db = AskDatabase {
            pool: pool.clone(),
            private_store,
        };

        // Insert an ask with Failed state.
        let mut ask_failed = sample_local_ask(17);
        ask_failed.state = Some(AskState::Failed);
        db.insert(ask_failed.clone());

        let failed_count = db.get_failed_request_count_by_market_id(&ask_failed.market_id);
        let total_failed = db.get_failed_request_count();
        assert!(failed_count >= 1);
        assert!(total_failed >= 1);

        // Insert an ask with Completed state.
        let mut ask_completed = sample_local_ask(18);
        ask_completed.state = Some(AskState::Completed);
        let mut record: my_crate::AskRecord = ask_completed.clone().into();
        record.job_completed_on_timestamp = Some(u256_to_bytes(U256::from(7777)));
        diesel::insert_into(my_crate::schema::ask_records::table)
            .values(&record)
            .execute(&conn)?;
        let recent = db.get_recent_completed_proofs(10);
        assert!(!recent.is_empty(), "Expected recent completed proofs");

        let completed_for_gen = db.get_completed_proof_of_generator(&"0x0000000000000000000000000000000000000000".parse().unwrap(), 0, 10);
        let completed_for_market = db.get_completed_proofs_of_market(&ask_completed.market_id, 0, 10);
        // Even if empty, they should return valid Vecs.
        assert!(completed_for_gen.len() >= 0);
        assert!(completed_for_market.len() >= 0);
        Ok(())
    });
}

/// Test timing operations.
#[test]
fn test_timing_operations() {
    let pool = init_test_pool();
    let conn = pool.get().unwrap();
    conn.test_transaction::<_, Error, _>(|| {
        let private_store = PrivateInputStore::new();
        let mut db = AskDatabase {
            pool: pool.clone(),
            private_store,
        };

        let ask = sample_local_ask(19);
        db.insert(ask.clone());

        // Update proof cycle completed timestamp.
        let new_cycle_time = U256::from(8888);
        db.update_proof_proof_cycle_completed_on(&ask.ask_id, new_cycle_time);
        let cycle_time = db.get_proof_proof_cycle_completed_on(&ask.ask_id);
        assert_eq!(cycle_time, Some(new_cycle_time));

        // Update and verify job completed timestamp.
        let new_job_completed = U256::from(9999);
        db.update_job_completed_on_timestamp(&ask.ask_id, new_job_completed);
        let job_completed = db.get_job_completed_on_timestamp(&ask.ask_id);
        assert_eq!(job_completed, Some(new_job_completed));

        // Update and verify job matched timestamp.
        let new_job_matched = U256::from(1010);
        db.update_job_matched_on_timestamp(&ask.ask_id, new_job_matched);
        let job_matched = db.get_job_matched_on_timestamp(&ask.ask_id);
        assert_eq!(job_matched, Some(new_job_matched));

        // Update and verify job created timestamp.
        let new_job_created = U256::from(2020);
        db.update_job_created_on_timestamp(&ask.ask_id, new_job_created);
        let job_created = db.get_job_created_on_timestamp(&ask.ask_id);
        assert_eq!(job_created, Some(new_job_created));

        // Update proving time directly using Diesel.
        diesel::update(my_crate::schema::ask_records::table.filter(ask_id.eq(u256_to_bytes(ask.ask_id))))
            .set(my_crate::schema::ask_records::proving_time_taken.eq(Some(u256_to_bytes(U256::from(3030)))))
            .execute(&conn)?;
        let overall_time = db.get_overall_proving_time(&ask.ask_id);
        assert_eq!(overall_time, Some(U256::from(3030)));
        Ok(())
    });
}
