use ethers::types::U64;
use prometheus_client::{
    encoding::{EncodeLabelSet, EncodeLabelValue},
    metrics::{counter::Counter, family::Family, gauge::Gauge, histogram::Histogram},
    registry::Registry,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
pub enum Op {
    TaskAssigned,
    TaskProven,
    TaskChallengedSuccessfully,

    // on chain submission check
    JobTransactionSubmitted,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Operation {
    pub method: Op,
}

pub struct TaskMetrics {
    pub requests: Family<Operation, Counter>,
    pub block: Gauge,
    pub proving_time: Histogram,
}

impl Default for TaskMetrics {
    fn default() -> Self {
        let custom_buckets = [0.1, 1.0, 10.0, 100.0, 1000.0, 10000.0];
        let histogram = Histogram::new(custom_buckets.into_iter());

        Self {
            requests: Family::default(),
            block: Gauge::default(),
            proving_time: histogram,
        }
    }
}

impl TaskMetrics {
    pub fn inc_tasks_assigned(&self) {
        self.requests
            .get_or_create(&Operation {
                method: Op::TaskAssigned,
            })
            .inc();
    }

    pub fn increase_tasks_proven(&self) {
        self.requests
            .get_or_create(&Operation {
                method: Op::TaskProven,
            })
            .inc();
    }

    pub fn increase_challenge_request(&self) {
        self.requests
            .get_or_create(&Operation {
                method: Op::TaskChallengedSuccessfully,
            })
            .inc();
    }

    pub fn increase_job_submitted_on_chain(&self) {
        self.requests
            .get_or_create(&Operation {
                method: Op::JobTransactionSubmitted,
            })
            .inc();
    }

    pub fn note_block_parsed_to(&self, parsed_till: U64) {
        let block_value = parsed_till.as_u64() as i64;
        self.block.set(block_value);
    }

    pub fn observe_time_spent(&self, time: f64) {
        self.proving_time.observe(time);
    }
}

#[derive(Default)]
pub struct ListenerMetrics {
    pub registry: Registry,
}
