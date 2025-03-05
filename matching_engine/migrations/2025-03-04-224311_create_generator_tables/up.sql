-- Table: generators
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

-- Table: generator_markets
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

-- Table: token_trackers
CREATE TABLE token_trackers (
    id SERIAL PRIMARY KEY,
    generator_address TEXT NOT NULL,
    market_id TEXT,  -- Nullable: applies to generator-wide data if NULL
    token TEXT NOT NULL,
    amount TEXT NOT NULL
);

-- Table: slashing_records
CREATE TABLE slashing_records (
    id SERIAL PRIMARY KEY,
    generator_address TEXT NOT NULL,
    ask_id TEXT NOT NULL,
    slashing_block_number TEXT NOT NULL,
    market_id TEXT NOT NULL,
    slashing_tx TEXT NOT NULL,
    price_offered TEXT NOT NULL
);

-- Table: delegations
CREATE TABLE delegations (
    id SERIAL PRIMARY KEY,
    generator_address TEXT NOT NULL,
    delegated_address TEXT NOT NULL,
    delegated_amount TEXT NOT NULL,
    source TEXT NOT NULL,
    operation TEXT NOT NULL,
    block_number TEXT NOT NULL,
    transaction_index TEXT NOT NULL
);

-- Table: withdrawal_requests
CREATE TABLE withdrawal_requests (
    id SERIAL PRIMARY KEY,
    generator_address TEXT NOT NULL,
    account TEXT NOT NULL,
    token TEXT NOT NULL,
    amount TEXT NOT NULL,
    request_index TEXT NOT NULL,
    timestamp TEXT NOT NULL
);
