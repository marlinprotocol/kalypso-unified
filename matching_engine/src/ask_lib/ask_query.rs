use ethers::core::types::U256;
use ethers::prelude::*;
use rayon::prelude::*;

use super::{ask::LocalAsk, ask_status::Comparison};

pub struct AskQueryResult {
    pub asks: Option<Vec<LocalAsk>>,
}

impl AskQueryResult {
    #[allow(unused)]
    pub fn sort_by_expiry(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.par_sort_by(|a, b| a.expiry.cmp(&b.expiry));
        }
        self
    }

    pub fn result(self) -> Option<Vec<LocalAsk>> {
        self.asks
    }

    pub fn sort_by_ask_id(mut self, asc: bool) -> Self {
        if let Some(ref mut asks) = self.asks {
            if asc {
                // Sort in ascending order
                asks.par_sort_by(|a, b| a.ask_id.cmp(&b.ask_id));
            } else {
                // Sort in descending order
                asks.par_sort_by(|a, b| b.ask_id.cmp(&a.ask_id));
            }
        }
        self
    }

    pub fn filter_by_market_id(self, market_id: U256) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_par_iter() // Parallel iterator over asks
                .filter(|ask| ask.market_id == market_id) // Filter by market_id
                .collect::<Vec<_>>() // Collect filtered results
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn sort_by_reward(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.par_sort_by(|a, b| a.reward.cmp(&b.reward));
        }
        self
    }

    #[allow(unused)]
    pub fn sort_by_deadline(mut self) -> Self {
        if let Some(ref mut asks) = self.asks {
            asks.par_sort_by(|a, b| a.deadline.cmp(&b.deadline));
        }
        self
    }

    #[allow(unused)]
    pub fn filter_by_has_private_inputs(self, value: bool) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_par_iter()
                .filter(|ask| ask.has_private_inputs == value)
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    pub fn filter_by_flag(self, value: bool) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_par_iter()
                .filter(|ask| ask.invalid_secret_flag == value)
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    fn compare(value: U256, other: U256, comparison: &Comparison) -> bool {
        match comparison {
            Comparison::Equal => value == other,
            Comparison::LessThan => value < other,
            Comparison::GreaterThan => value > other,
            Comparison::LessThanOrEqual => value <= other,
            Comparison::GreaterThanOrEqual => value >= other,
        }
    }

    #[allow(unused)]
    pub fn filter_by_expiry(self, value: U256, comparison: Comparison) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_par_iter()
                .filter(|ask| Self::compare(ask.expiry, value, &comparison))
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn filter_by_reward(self, value: U256, comparison: Comparison) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_par_iter()
                .filter(|ask| Self::compare(ask.reward, value, &comparison))
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn filter_by_deadline(self, value: U256, comparison: Comparison) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_par_iter()
                .filter(|ask| Self::compare(ask.deadline, value, &comparison))
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    #[allow(unused)]
    pub fn filter_by_prover_refund_address(self, address: Address) -> Self {
        let filtered = self.asks.map(|asks| {
            asks.into_par_iter()
                .filter(|ask| ask.prover_refund_address == address)
                .collect::<Vec<_>>()
        });
        AskQueryResult { asks: filtered }
    }

    pub fn get_count(self) -> usize {
        self.asks.map(|v| v.len()).unwrap_or(0)
    }
}
