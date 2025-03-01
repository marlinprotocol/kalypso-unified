use diesel::Insertable;
use crate::schema::{
    generators, generator_markets, token_trackers, slashing_records, delegations, withdrawal_requests,
};
use ethers::core::types::{Address, U256, U64, Bytes};

/// New record for the `generators` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = generators)]
pub struct NewGenerator {
    pub address: String,                      // e.g., format!("{:?}", generator.address)
    pub reward_address: String,               // e.g., format!("{:?}", generator.reward_address)
    pub total_native_stake: String,           // Serialize U256 or TokenTracker as a string or JSON
    pub total_symbiotic_stake: String,        // Serialize U256 or TokenTracker as a string or JSON
    pub sum_of_compute_allocations: String,   // U256 as string
    pub compute_consumed: String,             // U256 as string
    pub native_stake_locked: String,          // Serialize TokenTracker
    pub symbiotic_stake_locked: String,       // Serialize TokenTracker
    pub active_market_places: String,         // U256 as string
    pub declared_compute: String,             // U256 as string
    pub intended_stake_util: String,          // U256 as string
    pub intended_compute_util: String,        // U256 as string
    pub generator_data: Vec<u8>,              // Bytea for binary data
    pub active: bool,
    pub earnings: String,                     // U256 as string
    pub kalypso_points: String,               // U256 as string
    pub jobs_missed_counter: i32,             // i32 for counter
}

/// New record for the `generator_markets` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = generator_markets)]
pub struct NewGeneratorMarket {
    pub generator_address: String,            // e.g., format!("{:?}", generator_info.address)
    pub market_id: String,                    // U256 as string
    pub compute_required_per_request: String, // U256 as string
    pub proof_generation_cost: String,        // U256 as string
    pub proposed_time: String,                // U256 as string
    pub active_requests: String,              // U256 as string
    pub proofs_submitted: String,             // U256 as string
    pub proofs_slashed: String,               // U256 as string
    pub state: Option<String>,                // Optional state as a string (or enum)
    pub earnings: String,                     // U256 as string (per market earnings)
    pub kalypso_points: String,               // U256 as string (per market points)
}

/// New record for the `token_trackers` table.
/// Here, if `market_id` is None, the record applies to generator-wide data.
#[derive(Debug, Insertable)]
#[diesel(table_name = token_trackers)]
pub struct NewTokenTracker {
    pub generator_address: String,            // Generator address as string
    pub market_id: Option<String>,            // U256 as string or None for generator-wide
    pub token: String,                        // Token identifier (H160 as string)
    pub amount: String,                       // U256 as string
}

/// New record for the `slashing_records` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = slashing_records)]
pub struct NewSlashingRecord {
    pub generator_address: String,            // Generator address as string
    pub ask_id: String,                       // U256 as string
    pub slashing_block_number: String,        // U64 as string
    pub market_id: String,                    // U256 as string
    pub slashing_tx: String,                  // Transaction hash as string
    pub price_offered: String,                // U256 as string
    // Additional fields can be added if needed.
}

/// New record for the `delegations` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = delegations)]
pub struct NewDelegation {
    pub generator_address: String,            // Generator address as string
    pub delegated_address: String,            // Delegated address (H160 as string)
    pub delegated_amount: String,             // U256 as string
    pub source: String,                       // Source (as string or serialized enum)
    pub operation: String,                    // Operation (as string or serialized enum)
    pub block_number: String,                 // U64 as string
    pub transaction_index: String,            // U64 as string
}

/// New record for the `withdrawal_requests` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = withdrawal_requests)]
pub struct NewWithdrawalRequest {
    pub generator_address: String,            // Generator address as string
    pub account: String,                      // Account (H160 as string)
    pub token: String,                        // Token (H160 as string)
    pub amount: String,                       // U256 as string
    pub request_index: String,                // U256 as string
    pub timestamp: String,                    // U256 as string (or a proper timestamp type)
}

impl From<Generator> for NewGenerator {
    fn from(gen: Generator) -> Self {
        NewGenerator {
            address: format!("{:?}", gen.address),
            reward_address: format!("{:?}", gen.reward_address),
            total_native_stake: gen.total_native_stake.to_string(),
            total_symbiotic_stake: gen.total_symbiotic_stake.to_string(),
            sum_of_compute_allocations: gen.sum_of_compute_allocations.to_string(),
            compute_consumed: gen.compute_consumed.to_string(),
            native_stake_locked: gen.native_stake_locked.to_string(),
            symbiotic_stake_locked: gen.symbiotic_stake_locked.to_string(),
            active_market_places: gen.active_market_places.to_string(),
            declared_compute: gen.declared_compute.to_string(),
            intended_stake_util: gen.intended_stake_util.to_string(),
            intended_compute_util: gen.intended_compute_util.to_string(),
            generator_data: gen.generator_data.to_vec(), // Convert Bytes to Vec<u8>
            active: gen.active,
            earnings: gen.earnings.to_string(),
            kalypso_points: gen.kalypso_points.to_string(),
            jobs_missed_counter: gen.jobs_missed_counter,
        }
    }
}

impl From<GeneratorRecord> for Generator {
    fn from(rec: GeneratorRecord) -> Self {
        Generator {
            address: rec.address.parse().expect("Invalid address"),
            reward_address: rec.reward_address.parse().expect("Invalid reward address"),
            total_native_stake: rec.total_native_stake.parse().expect("Invalid TokenTracker"),
            total_symbiotic_stake: rec.total_symbiotic_stake.parse().expect("Invalid TokenTracker"),
            sum_of_compute_allocations: rec.sum_of_compute_allocations.parse().expect("Invalid U256"),
            compute_consumed: rec.compute_consumed.parse().expect("Invalid U256"),
            native_stake_locked: rec.native_stake_locked.parse().expect("Invalid TokenTracker"),
            symbiotic_stake_locked: rec.symbiotic_stake_locked.parse().expect("Invalid TokenTracker"),
            active_market_places: rec.active_market_places.parse().expect("Invalid U256"),
            declared_compute: rec.declared_compute.parse().expect("Invalid U256"),
            intended_stake_util: rec.intended_stake_util.parse().expect("Invalid U256"),
            intended_compute_util: rec.intended_compute_util.parse().expect("Invalid U256"),
            generator_data: Bytes::from(rec.generator_data),
            active: rec.active,
            earnings: rec.earnings.parse().expect("Invalid U256"),
            kalypso_points: rec.kalypso_points.parse().expect("Invalid U256"),
            jobs_missed_counter: rec.jobs_missed_counter,
        }
    }
}

// ================================================================
// Conversions for GeneratorInfoPerMarket
// ================================================================

impl From<GeneratorInfoPerMarket> for NewGeneratorMarket {
    fn from(info: GeneratorInfoPerMarket) -> Self {
        NewGeneratorMarket {
            generator_address: format!("{:?}", info.address),
            market_id: info.market_id.to_string(),
            compute_required_per_request: info.compute_required_per_request.to_string(),
            proof_generation_cost: info.proof_generation_cost.to_string(),
            proposed_time: info.proposed_time.to_string(),
            active_requests: info.active_requests.to_string(),
            proofs_submitted: info.proofs_submitted.to_string(),
            proofs_slashed: info.proofs_slashed.to_string(),
            state: info.state.map(|s| s.to_string()),
            // If earnings and kalypso_points are not part of your domain,
            // you can initialize them to a default (here, U256::zero()).
            earnings: U256::zero().to_string(),
            kalypso_points: U256::zero().to_string(),
        }
    }
}

impl From<GeneratorMarketRecord> for GeneratorInfoPerMarket {
    fn from(rec: GeneratorMarketRecord) -> Self {
        GeneratorInfoPerMarket {
            address: rec.generator_address.parse().expect("Invalid address"),
            market_id: rec.market_id.parse().expect("Invalid market id"),
            compute_required_per_request: rec.compute_required_per_request.parse().expect("Invalid U256"),
            proof_generation_cost: rec.proof_generation_cost.parse().expect("Invalid U256"),
            proposed_time: rec.proposed_time.parse().expect("Invalid U256"),
            active_requests: rec.active_requests.parse().expect("Invalid U256"),
            proofs_submitted: rec.proofs_submitted.parse().expect("Invalid U256"),
            proofs_slashed: rec.proofs_slashed.parse().expect("Invalid U256"),
            state: rec.state.map(|s| s.parse().expect("Invalid state")), // Assuming GeneratorState implements FromStr
        }
    }
}

// ================================================================
// Conversions for WithdrawlRequest
// ================================================================

impl From<WithdrawlRequest> for NewWithdrawalRequest {
    fn from(req: WithdrawlRequest) -> Self {
        NewWithdrawalRequest {
            // The generator_address must be set by the caller (e.g., via context)
            generator_address: String::new(),
            account: format!("{:?}", req.account),
            token: format!("{:?}", req.token),
            amount: req.amount.to_string(),
            request_index: req.index.to_string(),
            timestamp: req.timestamp.to_string(),
        }
    }
}

impl From<WithdrawalRequestRecord> for WithdrawlRequest {
    fn from(rec: WithdrawalRequestRecord) -> Self {
        WithdrawlRequest {
            account: rec.account.parse().expect("Invalid account"),
            token: rec.token.parse().expect("Invalid token"),
            amount: rec.amount.parse().expect("Invalid amount"),
            index: rec.request_index.parse().expect("Invalid index"),
            timestamp: rec.timestamp.parse().expect("Invalid timestamp"),
        }
    }
}

// diesel_store.rs

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use ethers::core::types::{Address, U256, U64, Bytes};
use std::time::SystemTime;
use std::collections::HashMap;
use std::collections::BTreeSet;
use tokio::sync::RwLockReadGuard;

// Import your Diesel schema and model conversion modules.
use crate::schema::*;
use crate::models_insertable::*;
use crate::models_query::*;

// Import your domain types and traits (assumed to be defined elsewhere).
use crate::domain::{
    Generator,
    GeneratorInfoPerMarket,
    WithdrawlRequest,
    GeneratorState,
    TokenTracker,
    SlashingRecord,
    Delegation,
    Operation,
    Source,
    GeneratorQueryResult,
};

/// Our Diesel-based store which uses a connection pool.
pub struct DieselGeneratorStore {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselGeneratorStore {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create connection pool");
        DieselGeneratorStore { pool }
    }
}

/// -------------------------
/// Generator Registration
/// -------------------------
impl GeneratorRegistration for DieselGeneratorStore {
    fn register_generator(&mut self, generator: Generator) {
        // Convert the domain generator to an insertable record.
        let new_gen: NewGenerator = generator.into();
        let conn = &mut self.pool.get().expect("DB connection error");
        diesel::insert_into(generators::table)
            .values(&new_gen)
            .execute(conn)
            .expect("Error inserting generator");
    }

    fn register_generator_in_market(&mut self, generator_market: GeneratorInfoPerMarket) {
        let new_market: NewGeneratorMarket = generator_market.into();
        let conn = &mut self.pool.get().expect("DB connection error");
        diesel::insert_into(generator_markets::table)
            .values(&new_market)
            .execute(conn)
            .expect("Error inserting generator market");
    }

    fn remove_by_address_and_market(&mut self, address: &Address, market_id: &U256) {
        let conn = &mut self.pool.get().expect("DB connection error");
        // Convert address and market_id to strings (per your mapping).
        let addr_str = format!("{:?}", address);
        let market_str = market_id.to_string();
        diesel::delete(
            generator_markets::table
                .filter(generator_markets::generator_address.eq(addr_str))
                .filter(generator_markets::market_id.eq(market_str)),
        )
        .execute(conn)
        .expect("Error deleting generator market");
    }

    fn remove_by_address(&mut self, address: &Address) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = format!("{:?}", address);
        // Remove from generators table.
        diesel::delete(generators::table.filter(generators::address.eq(addr_str.clone())))
            .execute(conn)
            .expect("Error deleting generator");
        // Cascade-delete related records (markets, etc.) as needed.
        diesel::delete(generator_markets::table.filter(generator_markets::generator_address.eq(addr_str)))
            .execute(conn)
            .expect("Error deleting generator markets");
    }

    fn get_by_address(&self, address: &Address) -> Option<Generator> {
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = format!("{:?}", address);
        let rec = generators::table
            .filter(generators::address.eq(addr_str))
            .first::<GeneratorRecord>(conn)
            .ok()?;
        Some(rec.into())
    }

    fn is_active(&self, generator_address: &Address) -> bool {
        let conn = &mut self.pool.get().expect("DB connection error");
        let addr_str = format!("{:?}", generator_address);
        generators::table
            .filter(generators::address.eq(addr_str))
            .select(generators::active)
            .first::<bool>(conn)
            .unwrap_or(false)
    }
}

/// -------------------------
/// Generator Stake & Compute Management
/// -------------------------
impl GeneratorStakeComputeManagement for DieselGeneratorStore {
    fn add_extra_stake(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        amount: &U256,
        block_number: U64,
        transaction_index: U64,
        log_index: U256,
        tx: String,
        source: Source,
    ) {
        // Update the appropriate token tracker in the generators table.
        // You might need to deserialize, update, and reserialize the TokenTracker.
        unimplemented!("Update extra stake in the DB");
    }

    fn update_intended_stake_util(&mut self, address: &Address, new_stake_util: U256) {
        // Update the 'intended_stake_util' column in the generators table.
        unimplemented!("Update intended stake util");
    }

    fn remove_stake(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        amount: &U256,
        block_number: U64,
        transaction_index: U64,
        log_index: U256,
        tx: String,
        operation: Operation,
        source: Source,
    ) {
        // Update token tracker by reducing the stake.
        unimplemented!("Remove stake from the DB");
    }

    fn update_reward_address(&mut self, address: &Address, new_reward_address: Address) {
        // Update the reward_address field.
        unimplemented!("Update reward address");
    }

    fn add_extra_compute(&mut self, address: &Address, compute: U256) {
        // Update compute allocations in the generators table.
        unimplemented!("Add extra compute");
    }

    fn update_intended_compute_util(&mut self, address: &Address, new_compute_util: U256) {
        unimplemented!("Update intended compute util");
    }

    fn remove_compute(&mut self, address: &Address, compute: U256) {
        unimplemented!("Remove compute");
    }
}

/// -------------------------
/// Generator Market Management
/// -------------------------
impl GeneratorMarketManagement for DieselGeneratorStore {
    fn update_state(&mut self, address: &Address, market_id: &U256, new_state: GeneratorState) {
        // Update the 'state' column in the generator_markets table.
        unimplemented!("Update generator market state");
    }

    fn update_on_assigned_task(&mut self, address: &Address, market_id: &U256) {
        // Update fields (like active_requests) on assignment.
        unimplemented!("Update on assigned task");
    }

    fn update_on_submit_proof(
        &mut self,
        address: &Address,
        market_id: &U256,
        earning: &U256,
        block_number: &U64,
    ) {
        // Update proofs_submitted counter and add earning to the earnings field.
        unimplemented!("Update on submit proof");
    }

    fn reduce_active_requests(&mut self, generator_address: &Address, market_id: &U256) {
        unimplemented!("Reduce active requests");
    }

    fn pause_assignments_across_all_markets(&mut self, address: &Address) {
        // Set a paused state in all market records for this generator.
        unimplemented!("Pause assignments across all markets");
    }

    fn resume_assignments_accross_all_markets(&mut self, address: &Address) {
        unimplemented!("Resume assignments across all markets");
    }
}

/// -------------------------
/// Generator Slashing Management
/// -------------------------
impl GeneratorSlashingManagement for DieselGeneratorStore {
    fn note_entry_slashing(
        &mut self,
        generator_address: &Address,
        ask_id: &U256,
        market_id: &U256,
        native_tokens_slashed: Vec<Address>,
        native_slashings: Vec<U256>,
        symbiotic_tokens_slashed: Vec<Address>,
        symbiotic_slashings: Vec<U256>,
        slashing_tx: String,
        price_offered: &U256,
        deadline: &U256,
        slashing_block_number: &U64,
        slashing_timestamp: &U256,
    ) {
        // Insert a new record into the slashing_records table and update token_trackers.
        unimplemented!("Record a slashing event");
    }
}

/// -------------------------
/// Generator Lock Management
/// -------------------------
impl GeneratorLockManagement for DieselGeneratorStore {
    fn update_on_stake_locked(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_locked: U256,
        source: Source,
    ) {
        // Update locked stake (native or symbiotic) in the generators table.
        unimplemented!("Update stake locked");
    }

    fn update_on_stake_released(
        &mut self,
        generator_address: &Address,
        token_address: &Address,
        stake_released: U256,
        source: Source,
    ) {
        unimplemented!("Update stake released");
    }

    fn update_on_compute_locked(&mut self, address: &Address, compute_locked: U256) {
        unimplemented!("Update compute locked");
    }

    fn update_on_compute_released(&mut self, address: &Address, compute_released: U256) {
        unimplemented!("Update compute released");
    }
}

/// -------------------------
/// Generator Availability
/// -------------------------
impl GeneratorAvailability for DieselGeneratorStore {
    fn get_available_compute(&self, address: Address) -> Option<U256> {
        // Calculate available compute (e.g., declared minus locked).
        unimplemented!("Compute available compute");
    }

    fn get_available_native_stake(&self, generator_address: &Address) -> Option<TokenTracker> {
        // Retrieve and compute available native stake.
        unimplemented!("Compute available native stake");
    }

    fn get_available_symbiotic_stake(&self, generator_address: &Address) -> Option<TokenTracker> {
        unimplemented!("Compute available symbiotic stake");
    }

    fn get_native_stake_locked(&self, generator_address: &Address) -> Option<TokenTracker> {
        unimplemented!("Fetch native stake locked");
    }

    fn get_symbiotic_stake_locked(&self, generator_address: &Address) -> Option<TokenTracker> {
        unimplemented!("Fetch symbiotic stake locked");
    }

    fn get_all_by_market_id(&self, market_id: &U256) -> Vec<GeneratorInfoPerMarket> {
        // Query generator_markets table for the specified market.
        unimplemented!("Fetch all generators by market id");
    }
}

/// -------------------------
/// Generator Query
/// -------------------------
impl GeneratorQuery for DieselGeneratorStore {
    fn query(&self) -> GeneratorQueryResult {
        // Return a result representing all generators.
        unimplemented!("Query all generators");
    }

    fn query_by_market_id_and_only_active(&self, market_id: &U256) -> GeneratorQueryResult {
        unimplemented!("Query active generators by market id");
    }

    fn query_by_states(&self, states: Vec<GeneratorState>) -> GeneratorQueryResult {
        unimplemented!("Query generators by state");
    }

    fn query_by_address(&self, address: Address) -> GeneratorQueryResult {
        unimplemented!("Query generator by address");
    }
}

/// -------------------------
/// Generator Filter
/// -------------------------
impl GeneratorFilter for DieselGeneratorStore {
    fn filter_by_has_idle_compute(
        &self,
        generator_query: GeneratorQueryResult,
    ) -> GeneratorQueryResult {
        // Filter out generators that have no idle compute.
        unimplemented!("Filter by idle compute");
    }

    fn filter_by_available_native_stake(
        &self,
        generator_query: GeneratorQueryResult,
        min_stake: Vec<(Address, U256)>,
    ) -> GeneratorQueryResult {
        unimplemented!("Filter by available native stake");
    }

    fn filter_by_available_symbiotic_stake(
        &self,
        generator_query: GeneratorQueryResult,
        min_stake: Vec<(Address, U256)>,
    ) -> GeneratorQueryResult {
        unimplemented!("Filter by available symbiotic stake");
    }
}

/// -------------------------
/// Generator KeyStore Filter Interface
/// -------------------------
impl<KS: KeyStoreOperations> GeneratorKeyStoreFilterInterfaceTrait<KS> for DieselGeneratorStore {
    fn filter_by_has_private_inputs_support(
        &self,
        generator_query: GeneratorQueryResult,
        key_store: RwLockReadGuard<'_, KS>,
    ) -> GeneratorQueryResult {
        // Use the provided key store to filter generators that support private inputs.
        unimplemented!("Filter by private inputs support");
    }
}

/// -------------------------
/// Generator Earnings and Slashing
/// -------------------------
impl GeneratorEarningsAndSlashing for DieselGeneratorStore {
    fn get_total_earning(&self, address: &Address) -> Option<U256> {
        // Query the generators table for total earnings.
        unimplemented!("Get total earnings");
    }

    fn get_earning_per_market(&self, address: &Address, market_id: &U256) -> Option<U256> {
        // Query generator_markets table for per-market earnings.
        unimplemented!("Get per-market earnings");
    }

    fn get_kalypso_points(&self, address: &Address) -> Option<U256> {
        unimplemented!("Get total kalypso points");
    }

    fn get_kalypso_points_per_market(&self, address: &Address, market_id: &U256) -> Option<U256> {
        unimplemented!("Get kalypso points per market");
    }

    fn get_total_slashing(&self, generator_address: &Address) -> Option<TokenTracker> {
        // Aggregate generator-wide slashings from token_trackers table.
        unimplemented!("Get total slashing");
    }

    fn get_slashing_per_generator_per_market(
        &self,
        address: &Address,
        market_id: &U256,
    ) -> Option<TokenTracker> {
        // Query token_trackers table for slashings specific to a market.
        unimplemented!("Get slashing per market");
    }

    fn get_slashing_records(&self, address: &Address) -> Vec<SlashingRecord> {
        // Query slashing_records table for records related to the generator.
        unimplemented!("Get slashing records");
    }
}

/// -------------------------
/// Withdrawal Management
/// -------------------------
impl WithdrawalManagement for DieselGeneratorStore {
    fn insert_withdrawal_request(
        &mut self,
        operator_address: &Address,
        withdrawal_request: WithdrawlRequest,
    ) {
        // Convert and insert into withdrawal_requests table.
        unimplemented!("Insert withdrawal request");
    }

    fn get_withdrawl_requests(&self, operator_address: &Address) -> Vec<WithdrawlRequest> {
        unimplemented!("Get withdrawal requests");
    }

    fn remove_withdrawal_request(
        &mut self,
        operator_address: &Address,
        withdrawal_request: WithdrawlRequest,
    ) -> bool {
        unimplemented!("Remove withdrawal request")
    }
}

/// -------------------------
/// Generator Metadata
/// -------------------------
impl GeneratorMetadata for DieselGeneratorStore {
    fn update_generator_metadata(&mut self, generator_address: Address, generator_meta_data: Bytes) {
        // Update the generator_data field in the generators table.
        unimplemented!("Update generator metadata");
    }
}

/// -------------------------
/// Job Missed Counter
/// -------------------------
impl JobMissedCounter for DieselGeneratorStore {
    fn count_job_missed_by_generator(
        &mut self,
        generator_address: Address,
        timestamp: SystemTime,
    ) {
        // Increment or log the job missed counter for the generator.
        unimplemented!("Count job missed by generator");
    }

    fn get_job_missed_count(&self, generator_address: &Address) -> usize {
        unimplemented!("Get job missed count")
    }
}

/// -------------------------
/// Generator Additional Query
/// -------------------------
impl GeneratorAdditionalQuery for DieselGeneratorStore {
    fn all_generators_address(&self) -> Vec<Address> {
        // Query the generators table and return all addresses.
        unimplemented!("Return all generator addresses");
    }

    fn get_delegations(
        &self,
        generator_address: &Address,
        operations: Vec<Operation>,
        skip: Option<usize>,
        count: Option<usize>,
    ) -> Vec<Delegation> {
        // Query the delegations table filtering by generator_address and operations.
        unimplemented!("Return delegations");
    }

    fn total_native_stake_accross_all_generators(&self) -> TokenTracker {
        // Aggregate native stake from all generators.
        unimplemented!("Aggregate total native stake");
    }

    fn total_symbiotic_stake_across_all_generators(&self) -> TokenTracker {
        unimplemented!("Aggregate total symbiotic stake");
    }

    fn get_by_address_and_market(
        &self,
        address: &Address,
        market_id: &U256,
    ) -> Option<GeneratorInfoPerMarket> {
        // Query the generator_markets table for a specific generator-market pair.
        unimplemented!("Get generator info for address and market");
    }

    fn get_all_markets_of_generator(&self, address: &Address) -> Vec<GeneratorInfoPerMarket> {
        // Query the generator_markets table filtering by generator_address.
        unimplemented!("Get all markets of generator");
    }
}