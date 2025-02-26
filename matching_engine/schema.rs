// table! {
//     asks (id) {
//         id -> Numeric,
//         market_id -> Numeric,
//         generator -> Nullable<Varchar>,
//         acl -> Nullable<Bytea>,
//         state -> Varchar,
//         deadline -> Numeric,
//         proof -> Nullable<Bytea>,
//         proof_time -> Nullable<Numeric>,
//         proof_cost -> Nullable<Numeric>,
//         proof_transaction -> Nullable<Varchar>,
//     }
// }

// table! {
//     proofs (ask_id) {
//         ask_id -> Numeric,
//         proof -> Bytea,
//         proof_time -> Numeric,
//         proof_cost -> Numeric,
//         proof_transaction -> Varchar,
//     }
// }

// table! {
//     requestors (market_id) {
//         market_id -> Numeric,
//         requestor_count -> Integer,
//     }
// }

// table! {
//     proof_counters (market_id) {
//         market_id -> Numeric,
//         proof_count -> Integer,
//     }
// }

// table! {
//     timing_operations (ask_id) {
//         ask_id -> Numeric,
//         proof_cycle_completed_on -> Nullable<Numeric>,
//         job_completed_on -> Nullable<Numeric>,
//         job_matched_on -> Nullable<Numeric>,
//         job_created_on -> Nullable<Numeric>,
//     }
// }
diesel::table! {
    ask_records (ask_id) {
        ask_id -> Binary,
        market_id -> Binary,
        reward -> Binary,
        expiry -> Binary,
        deadline -> Binary,
        time_requested_for_proof_generation -> Binary,
        prover_refund_address -> Binary,
        prover_data -> Binary,
        has_private_inputs -> Bool,
        secret_data -> Nullable<Binary>,
        secret_acl -> Nullable<Binary>,
        state -> Nullable<Binary>,
        generator -> Nullable<Binary>,
        invalid_secret_flag -> Bool,
        created_on -> Binary,
        created_on_l1 -> Binary,
        create_transaction -> Binary,

        proof -> Nullable<Binary>,
        proving_time_taken -> Nullable<Binary>,
        proving_cost_taken -> Nullable<Binary>,
        proof_transaction -> Nullable<Text>,

        proof_cycle_completed_on -> Nullable<Binary>,
        job_created_on_timestamp -> Nullable<Binary>,
        job_matched_on_timestamp -> Nullable<Binary>,
        job_completed_on_timestamp -> Nullable<Binary>,
    }
}


table! {
    cost_store (key) {
        key -> Int2,
        value -> Numeric,
    }
}

table! {
    key_store (address, key_index) {
        address -> Bytea,
        key_index -> Int8,
        ecies_pub_key -> Nullable<Bytea>,
    }
}

table! {
    token_locks (token) {
        token -> Bytea,
        amount -> Numeric,
    }
}

table! {
    stake_pools (address) {
        address -> Bytea,
    }
}

table! {
    operator_stakes (operator, token_address) {
        operator -> Bytea,
        token_address -> Bytea,
        absolute_stake -> Numeric,
    }
}

table! {
    token_locks (token) {
        token -> Bytea,
        amount -> Numeric,
    }
}

table! {
    vault_snapshots (captured_timestamp, index) {
        captured_timestamp -> Numeric,
        index -> Numeric,
        snapshot_data -> Bytea, // Store VaultSnapshot as serialized data
    }
}

table! {
    slash_results (captured_timestamp, index) {
        captured_timestamp -> Numeric,
        index -> Numeric,
        result_data -> Bytea, // Store SlashResult as serialized data
    }
}


// schema.rs
diesel::table! {
    generators (id) {
        id -> Int4,
        address -> Varchar,
        market_id -> Nullable<Varchar>,
        state -> Int4,
        reward_address -> Varchar,
        compute_power -> Numeric,
    }
}

diesel::table! {
    stakes (id) {
        id -> Int4,
        generator_address -> Varchar,
        token_address -> Varchar,
        amount -> Numeric,
        block_number -> Int8,
        transaction_index -> Int8,
        log_index -> Int8,
        tx_hash -> Varchar,
        source -> Int4,
    }
}

diesel::table! {
    withdrawals (id) {
        id -> Int4,
        operator_address -> Varchar,
        amount -> Numeric,
        requested_at -> Timestamp,
    }
}


joinable!(proofs -> asks (ask_id));
joinable!(proof_counters -> requestors (market_id));
allow_tables_to_appear_in_same_query!(asks, proofs, requestors, proof_counters, timing_operations);
