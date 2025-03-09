use crate::{ask_lib::ask_store::{
        AskManagementRead, AskManagementWrite, CompletedProofsManagement, MarketRequestCounters,
        ProofCounters, RequestorCounters, TimingOperations,},
        generator_lib::{
        key_store::KeyStoreOperations,
        native_stake_store::NativeStakingOperations,
        stake_manager_store::StakeManagerOperations,
        symbiotic_stake_store::{
            OperatorStakeManagement, SlashResultManagement, TokenLockManagement,
            VaultSnapshotManagement,
        },
        traits::{
            GeneratorAdditionalQuery, GeneratorAvailability, GeneratorEarningsAndSlashing,
            GeneratorFilter, GeneratorKeyStoreFilterInterfaceTrait, GeneratorLockManagement,
            GeneratorMarketManagement, GeneratorMetadata, GeneratorQuery, GeneratorRegistration,
            GeneratorSlashingManagement, GeneratorStakeComputeManagement, JobMissedCounter,
            WithdrawalManagement,
        },
    },};

// Here traits only for reference, need to implement everyone explicity
pub trait AskStoreTrait:
    AskManagementRead
    + AskManagementWrite
    + RequestorCounters
    + ProofCounters
    + MarketRequestCounters
    + CompletedProofsManagement
    + TimingOperations
    + ProofMarketStakeLockManagement
{
}

pub trait GeneratorStoreTrait<KS: KeyStoreOperations>:
    GeneratorRegistration
    + GeneratorStakeComputeManagement
    + GeneratorMarketManagement
    + GeneratorSlashingManagement
    + GeneratorLockManagement
    + GeneratorAvailability
    + GeneratorMetadata
    + GeneratorQuery
    + GeneratorFilter
    + GeneratorKeyStoreFilterInterfaceTrait<KS>
    + GeneratorEarningsAndSlashing
    + WithdrawalManagement
    + JobMissedCounter
    + GeneratorAdditionalQuery
{
}

pub trait MarketStoreTrait: MarketMetadataStoreRead + MarketMetadataStoreWrite {}

pub trait KeyStoreTrait: KeyStoreOperations {}

pub trait CostStoreTrait: CostStoreOperations {}

pub trait SymbioticStore:
    OperatorStakeManagement + TokenLockManagement + VaultSnapshotManagement + SlashResultManagement
{
}

pub trait NativeStakeStoreTrait: NativeStakingOperations {}

pub trait StakeManagerStoreTrait: StakeManagerOperations {}