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


// table! {
//     cost_store (key) {
//         key -> Int2,
//         value -> Numeric,
//     }
// }

// table! {
//     key_store (address, key_index) {
//         address -> Bytea,
//         key_index -> Int8,
//         ecies_pub_key -> Nullable<Bytea>,
//     }
// }

// table! {
//     token_locks (token) {
//         token -> Bytea,
//         amount -> Numeric,
//     }
// }

// table! {
//     stake_pools (address) {
//         address -> Bytea,
//     }
// }

// table! {
//     operator_stakes (operator, token_address) {
//         operator -> Bytea,
//         token_address -> Bytea,
//         absolute_stake -> Numeric,
//     }
// }

// table! {
//     token_locks (token) {
//         token -> Bytea,
//         amount -> Numeric,
//     }
// }

// table! {
//     vault_snapshots (captured_timestamp, index) {
//         captured_timestamp -> Numeric,
//         index -> Numeric,
//         snapshot_data -> Bytea, // Store VaultSnapshot as serialized data
//     }
// }

// table! {
//     slash_results (captured_timestamp, index) {
//         captured_timestamp -> Numeric,
//         index -> Numeric,
//         result_data -> Bytea, // Store SlashResult as serialized data
//     }
// }

// Generators table: Stores generator-wide information.
table! {
    generators (address) {
        address -> Varchar,                           // Primary key (generator address)
        reward_address -> Varchar,
        total_native_stake -> Text,                   // Serialized TokenTracker (or JSON)
        total_symbiotic_stake -> Text,                // Serialized TokenTracker (or JSON)
        sum_of_compute_allocations -> Text,           // U256 as text
        compute_consumed -> Text,                     // U256 as text
        native_stake_locked -> Text,                  // Serialized TokenTracker (or JSON)
        symbiotic_stake_locked -> Text,               // Serialized TokenTracker (or JSON)
        active_market_places -> Text,                 // U256 as text
        declared_compute -> Text,                     // U256 as text
        intended_stake_util -> Text,                  // U256 as text
        intended_compute_util -> Text,                // U256 as text
        generator_data -> Bytea,                      // Binary data
        active -> Bool,
        earnings -> Text,                             // U256 as text
        kalypso_points -> Text,                       // U256 as text
        jobs_missed_counter -> Int4,                  // Job missed counter as integer
    }
}

// GeneratorMarkets table: Holds per-market details for each generator.
table! {
    generator_markets (generator_address, market_id) {
        generator_address -> Varchar,               // Foreign key to generators.address
        market_id -> Varchar,                         // U256 as text
        compute_required_per_request -> Text,         // U256 as text
        proof_generation_cost -> Text,                // U256 as text
        proposed_time -> Text,                        // U256 as text
        active_requests -> Text,                      // U256 as text
        proofs_submitted -> Text,                     // U256 as text
        proofs_slashed -> Text,                       // U256 as text
        state -> Nullable<Varchar>,                   // Optionally, a string representation of GeneratorState
        earnings -> Text,                             // Earnings per market (U256 as text)
        kalypso_points -> Text,                       // Kalypso points per market (U256 as text)
    }
}

// TokenTrackers table: Stores slashings and similar maps.
// If market_id is NULL, it represents generator-wide data.
table! {
    token_trackers (generator_address, market_id, token) {
        generator_address -> Varchar,               // Foreign key to generators.address
        market_id -> Nullable<Varchar>,               // U256 as text; NULL if generator-wide
        token -> Varchar,                             // Token identifier (H160 as string)
        amount -> Text,                               // U256 as text
    }
}

// SlashingRecords table: Records individual slashing events.
table! {
    slashing_records (id) {
        id -> Int4,                                 // Auto-incrementing primary key
        generator_address -> Varchar,               // Foreign key to generators.address
        ask_id -> Varchar,                          // U256 as text
        slashing_block_number -> Varchar,           // U64 as text
        market_id -> Varchar,                       // U256 as text
        slashing_tx -> Text,                        // Transaction hash as text
        price_offered -> Text,                      // U256 as text
    }
}

// Delegations table: Stores delegation entries for each generator.
table! {
    delegations (id) {
        id -> Int4,                                 // Auto-incrementing primary key
        generator_address -> Varchar,               // Foreign key to generators.address
        delegated_address -> Varchar,               // The address being delegated to (H160 as string)
        delegated_amount -> Text,                   // U256 as text
        source -> Varchar,                          // Source as string or enum representation
        operation -> Varchar,                       // Operation as string or enum representation
        block_number -> Varchar,                    // U64 as text
        transaction_index -> Varchar,               // U64 as text
    }
}

// WithdrawalRequests table: Handles withdrawal requests per generator.
table! {
    withdrawal_requests (id) {
        id -> Int4,                                 // Auto-incrementing primary key
        generator_address -> Varchar,               // Foreign key to generators.address
        account -> Varchar,                         // The account (H160 as string)
        token -> Varchar,                           // The token (H160 as string)
        amount -> Text,                             // U256 as text
        request_index -> Text,                      // U256 as text
        timestamp -> Text,                          // U256 as text 
    }
}

// Allow tables to appear together in queries.
allow_tables_to_appear_in_same_query!(
    generators,
    generator_markets,
    token_trackers,
    slashing_records,
    delegations,
    withdrawal_requests,
);