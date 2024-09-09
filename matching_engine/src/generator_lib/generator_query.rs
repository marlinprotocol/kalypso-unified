use super::{generator_state::GeneratorState, generator_store::GeneratorInfoPerMarket};
use ethers::prelude::*;

#[derive(Clone)]
pub struct GeneratorQueryResult<'a> {
    generator_markets: Vec<&'a GeneratorInfoPerMarket>,
}

impl<'a> GeneratorQueryResult<'a> {
    // Initialize with a collection of generators
    pub fn new(generator_markets: Vec<&'a GeneratorInfoPerMarket>) -> Self {
        Self { generator_markets }
    }

    // Filter by reward
    pub fn filter_by_reward(mut self, task_reward: U256) -> Self {
        log::debug!("Filter by reward");
        self.generator_markets
            .retain(|&gen| gen.proof_generation_cost.lt(&task_reward));
        self
    }

    // Filter by market ID
    pub fn filter_by_market_id(mut self, market_id: U256) -> Self {
        log::debug!("Filter by market id");
        self.generator_markets
            .retain(|&gen| gen.market_id == market_id);
        self
    }

    // Filter by state
    #[allow(unused)]
    pub fn filter_by_state(mut self, state: GeneratorState) -> Self {
        self.generator_markets
            .retain(|&gen| gen.state == Some(state));
        self
    }

    // Final getter to consume the object and retrieve the filtered generators
    pub fn result(self) -> Vec<&'a GeneratorInfoPerMarket> {
        self.generator_markets
    }
}
