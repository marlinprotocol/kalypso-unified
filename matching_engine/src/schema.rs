// @generated automatically by Diesel CLI.

diesel::table! {
    ask_records (ask_id) {
        ask_id -> Bytea,
        market_id -> Bytea,
        reward -> Bytea,
        expiry -> Bytea,
        deadline -> Bytea,
        time_requested_for_proof_generation -> Bytea,
        prover_refund_address -> Bytea,
        prover_data -> Bytea,
        has_private_inputs -> Bool,
        state -> Nullable<Bytea>,
        generator -> Nullable<Bytea>,
        invalid_secret_flag -> Bool,
        created_on -> Bytea,
        created_on_l1 -> Bytea,
        create_transaction -> Bytea,
        proof -> Nullable<Bytea>,
        proof_type -> Nullable<Varchar>,
        proving_time_taken -> Nullable<Bytea>,
        proving_cost_taken -> Nullable<Bytea>,
        proof_transaction -> Nullable<Text>,
        proof_cycle_completed_on -> Nullable<Bytea>,
        job_created_on_timestamp -> Nullable<Bytea>,
        job_matched_on_timestamp -> Nullable<Bytea>,
        job_completed_on_timestamp -> Nullable<Bytea>,
        associated_stake_locks -> Nullable<Text>,
    }
}

diesel::table! {
    delegations (id) {
        id -> Int4,
        generator_address -> Text,
        delegated_address -> Text,
        delegated_amount -> Text,
        source -> Text,
        operation -> Text,
        block_number -> Text,
        transaction_index -> Text,
        log_index -> Text,
        tx -> Text,
    }
}

diesel::table! {
    generator_markets (id) {
        id -> Int4,
        generator_address -> Text,
        market_id -> Text,
        compute_required_per_request -> Text,
        proof_generation_cost -> Text,
        proposed_time -> Text,
        active_requests -> Text,
        proofs_submitted -> Text,
        proofs_slashed -> Text,
        state -> Nullable<Text>,
        earnings -> Text,
        kalypso_points -> Text,
    }
}

diesel::table! {
    generators (address) {
        address -> Text,
        reward_address -> Text,
        total_native_stake -> Text,
        total_symbiotic_stake -> Text,
        sum_of_compute_allocations -> Text,
        compute_consumed -> Text,
        native_stake_locked -> Text,
        symbiotic_stake_locked -> Text,
        active_market_places -> Text,
        declared_compute -> Text,
        intended_stake_util -> Text,
        intended_compute_util -> Text,
        generator_data -> Bytea,
        active -> Bool,
        earnings -> Text,
        kalypso_points -> Text,
        jobs_missed_counter -> Int4,
    }
}

diesel::table! {
    slashing_records (id) {
        id -> Int4,
        generator_address -> Text,
        ask_id -> Text,
        slashing_block_number -> Text,
        market_id -> Text,
        slashing_tx -> Text,
        price_offered -> Text,
        expected_time -> Text,
        slashing_penalty -> Text,
        slashing_timestamp -> Text,
        source -> Text,
    }
}

diesel::table! {
    token_trackers (id) {
        id -> Int4,
        generator_address -> Text,
        market_id -> Text, // or Nullable<Text> if we need to allow generator-wide entries
        token_tracker -> Text, // here we store the aggregated U256 as a string (JSON is also an option)
    }
}

diesel::table! {
    withdrawal_requests (id) {
        id -> Int4,
        generator_address -> Text,
        account -> Text,
        token -> Text,
        amount -> Text,
        request_index -> Text,
        timestamp -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ask_records,
    delegations,
    generator_markets,
    generators,
    slashing_records,
    token_trackers,
    withdrawal_requests,
);


diesel::table! {
    cost_record (key) {
        key -> Int2,
        value -> Bytea,
    }
}

diesel::table! {
    key_record (address, key_index) {
        address -> Text,
        key_index -> BigInt,
        ecies_pub_key -> Nullable<Bytea>,
    }
}

diesel::table! {
    market_metadata (market_id) {
        market_id -> Text,             // U256 stored as text
        verifier -> Text,              // Address stored as text (e.g., hex)
        activation_block -> Text,      // U256 stored as text
        metadata -> Bytea,             // Binary metadata stored as BYTEA
        proof_time -> Text,            // U256 stored as text (proof time in blocks)
        proof_cost -> Text,            // U256 stored as text (proof cost in USDC)
        earnings -> Text,              // U256 stored as text (earnings in USDC)
    }
}

diesel::table! {
    market_images (id) {
        id -> Int4,                  // Auto-increment primary key
        market_id -> Text,           // U256 stored as text, foreign key to market_metadata(market_id)
        image_type -> Text,          // Image type ("prover" or "ivs")
        image -> Text,               // H256 stored as text (e.g., hex string)
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    market_metadata,
    market_images,
);