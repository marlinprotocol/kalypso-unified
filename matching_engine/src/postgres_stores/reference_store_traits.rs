use crate::ask_lib::ask_store::{
        AskManagementRead, AskManagementWrite, CompletedProofsManagement, MarketRequestCounters,
        ProofCounters, RequestorCounters, TimingOperations,
    };

// Here traits only for reference, need to implement everyone explicity
pub trait AskStoreTrait:
    AskManagementRead
    + AskManagementWrite
    + RequestorCounters
    + ProofCounters
    + MarketRequestCounters
    + CompletedProofsManagement
    + TimingOperations
{
}