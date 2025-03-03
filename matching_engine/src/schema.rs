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