table! {
    asks (id) {
        id -> Numeric,
        market_id -> Numeric,
        generator -> Nullable<Varchar>,
        acl -> Nullable<Bytea>,
        state -> Varchar,
        deadline -> Numeric,
        proof -> Nullable<Bytea>,
        proof_time -> Nullable<Numeric>,
        proof_cost -> Nullable<Numeric>,
        proof_transaction -> Nullable<Varchar>,
    }
}

table! {
    proofs (ask_id) {
        ask_id -> Numeric,
        proof -> Bytea,
        proof_time -> Numeric,
        proof_cost -> Numeric,
        proof_transaction -> Varchar,
    }
}

table! {
    requestors (market_id) {
        market_id -> Numeric,
        requestor_count -> Integer,
    }
}

table! {
    proof_counters (market_id) {
        market_id -> Numeric,
        proof_count -> Integer,
    }
}

table! {
    timing_operations (ask_id) {
        ask_id -> Numeric,
        proof_cycle_completed_on -> Nullable<Numeric>,
        job_completed_on -> Nullable<Numeric>,
        job_matched_on -> Nullable<Numeric>,
        job_created_on -> Nullable<Numeric>,
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


joinable!(proofs -> asks (ask_id));
joinable!(proof_counters -> requestors (market_id));
allow_tables_to_appear_in_same_query!(asks, proofs, requestors, proof_counters, timing_operations);
