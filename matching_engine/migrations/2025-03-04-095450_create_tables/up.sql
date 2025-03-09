-- Create ask_records table
CREATE TABLE ask_records (
    ask_id BYTEA PRIMARY KEY,
    market_id BYTEA NOT NULL,
    reward BYTEA NOT NULL,
    expiry BYTEA NOT NULL,
    deadline BYTEA NOT NULL,
    time_requested_for_proof_generation BYTEA NOT NULL,
    prover_refund_address BYTEA NOT NULL,
    prover_data BYTEA NOT NULL,
    has_private_inputs BOOLEAN NOT NULL,
    state BYTEA,
    generator BYTEA,
    invalid_secret_flag BOOLEAN NOT NULL,
    created_on BYTEA NOT NULL,
    created_on_l1 BYTEA NOT NULL,
    create_transaction BYTEA NOT NULL,
    proof BYTEA,
    proof_type VARCHAR,
    proving_time_taken BYTEA,
    proving_cost_taken BYTEA,
    proof_transaction TEXT,
    proof_cycle_completed_on BYTEA,
    job_created_on_timestamp BYTEA,
    job_matched_on_timestamp BYTEA,
    job_completed_on_timestamp BYTEA,
    associated_stake_locks TEXT
);

-- Create delegations table
CREATE TABLE delegations (
    id SERIAL PRIMARY KEY,
    generator_address TEXT NOT NULL,
    delegated_address TEXT NOT NULL,
    delegated_amount TEXT NOT NULL,
    source TEXT NOT NULL,
    operation TEXT NOT NULL,
    block_number TEXT NOT NULL,
    transaction_index TEXT NOT NULL,
    log_index TEXT NOT NULL,
    tx TEXT NOT NULL
);

-- Create generator_markets table
CREATE TABLE generator_markets (
    id SERIAL PRIMARY KEY,
    generator_address TEXT NOT NULL,
    market_id TEXT NOT NULL,
    compute_required_per_request TEXT NOT NULL,
    proof_generation_cost TEXT NOT NULL,
    proposed_time TEXT NOT NULL,
    active_requests TEXT NOT NULL,
    proofs_submitted TEXT NOT NULL,
    proofs_slashed TEXT NOT NULL,
    state TEXT,
    earnings TEXT NOT NULL,
    kalypso_points TEXT NOT NULL
);

-- Create generators table
CREATE TABLE generators (
    address TEXT PRIMARY KEY,
    reward_address TEXT NOT NULL,
    total_native_stake TEXT NOT NULL,
    total_symbiotic_stake TEXT NOT NULL,
    sum_of_compute_allocations TEXT NOT NULL,
    compute_consumed TEXT NOT NULL,
    native_stake_locked TEXT NOT NULL,
    symbiotic_stake_locked TEXT NOT NULL,
    active_market_places TEXT NOT NULL,
    declared_compute TEXT NOT NULL,
    intended_stake_util TEXT NOT NULL,
    intended_compute_util TEXT NOT NULL,
    generator_data BYTEA NOT NULL,
    active BOOLEAN NOT NULL,
    earnings TEXT NOT NULL,
    kalypso_points TEXT NOT NULL,
    jobs_missed_counter INTEGER NOT NULL
);

-- Create slashing_records table
CREATE TABLE slashing_records (
    id SERIAL PRIMARY KEY,
    generator_address TEXT NOT NULL,
    ask_id TEXT NOT NULL,
    slashing_block_number TEXT NOT NULL,
    market_id TEXT NOT NULL,
    slashing_tx TEXT NOT NULL,
    price_offered TEXT NOT NULL,
    expected_time TEXT NOT NULL,
    slashing_penalty TEXT NOT NULL,
    slashing_timestamp TEXT NOT NULL,
    source TEXT NOT NULL
);

-- Create token_trackers table
CREATE TABLE token_trackers (
    id SERIAL PRIMARY KEY,
    generator_address TEXT NOT NULL,
    market_id TEXT NOT NULL,
    token_tracker TEXT NOT NULL
);

-- Create withdrawal_requests table
CREATE TABLE withdrawal_requests (
    id SERIAL PRIMARY KEY,
    generator_address TEXT NOT NULL,
    account TEXT NOT NULL,
    token TEXT NOT NULL,
    amount TEXT NOT NULL,
    request_index TEXT NOT NULL,
    timestamp TEXT NOT NULL
);

CREATE TABLE key_store (
    address TEXT NOT NULL,
    key_index BIGINT NOT NULL,
    ecies_pub_key BYTEA,
    PRIMARY KEY (address, key_index)
);

CREATE TABLE market_metadata (
    market_id TEXT PRIMARY KEY,         -- U256 stored as text
    verifier TEXT NOT NULL,             -- Address stored as text (e.g., hex)
    activation_block TEXT NOT NULL,     -- U256 stored as text
    metadata BYTEA NOT NULL,            -- Bytes stored as BYTEA
    proof_time TEXT NOT NULL,           -- U256 stored as text (proof time in blocks)
    proof_cost TEXT NOT NULL,           -- U256 stored as text (proof cost in USDC)
    earnings TEXT NOT NULL              -- U256 stored as text (earnings in USDC)
);

-- Create the market images table to store both prover and ivs images
CREATE TABLE market_images (
    id SERIAL PRIMARY KEY,
    market_id TEXT NOT NULL,            -- U256 stored as text, foreign key referencing market_metadata(market_id)
    image_type TEXT NOT NULL,           -- "prover" or "ivs" (could also be an ENUM)
    image TEXT NOT NULL,                -- H256 stored as text (e.g., hex string)
    FOREIGN KEY (market_id) REFERENCES market_metadata(market_id)
);

CREATE TABLE native_staking_store (
    id SERIAL PRIMARY KEY,
    tokens_to_lock TEXT NOT NULL
);
