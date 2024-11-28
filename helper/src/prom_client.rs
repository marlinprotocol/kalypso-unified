use ethers::types::U64;
use prometheus_client::{
    encoding::{EncodeLabelSet, EncodeLabelValue},
    metrics::{counter::Counter, family::Family, gauge::Gauge},
    registry::Registry,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
pub enum Op {
    TaskAssigned,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Operation {
    pub method: Op,
}

#[derive(Default)]
pub struct TaskMetrics {
    pub requests: Family<Operation, Counter>,
    pub block: Gauge,
}

impl TaskMetrics {
    pub fn inc_tasks_assigned(&self) {
        self.requests
            .get_or_create(&Operation {
                method: Op::TaskAssigned,
            })
            .inc();
    }

    pub fn note_block_parsed_to(&self, parsed_till: U64) {
        let block_value = parsed_till.as_u64() as i64;
        self.block.set(block_value);
    }
}

#[derive(Default)]
pub struct ListenerMetrics {
    pub registry: Registry,
}
