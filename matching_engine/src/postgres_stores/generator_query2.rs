use ethers::types::U256;

use crate::generator_lib::{
    generator_state::GeneratorState, generator_store::GeneratorInfoPerMarket,
};

#[derive(Clone)]
pub struct GeneratorQueryResult2 {
    generator_markets: Vec<GeneratorInfoPerMarket>,
}

impl GeneratorQueryResult2 {
    // Initialize with a collection of generators (owned values)
    pub fn new(generator_markets: Vec<GeneratorInfoPerMarket>) -> Self {
        Self { generator_markets }
    }

    // Filter by reward
    pub fn filter_by_reward(mut self, task_reward: U256) -> Self {
        self.generator_markets = self
            .generator_markets
            .into_iter()
            .filter(|gen| gen.proof_generation_cost.lt(&task_reward))
            .collect();

        log::debug!(
            "Generators with reward: {:?} = {}",
            task_reward,
            self.generator_markets.len()
        );
        self
    }

    pub fn filter_by_time(mut self, task_time: U256) -> Self {
        self.generator_markets = self
            .generator_markets
            .into_iter()
            .filter(|gen| {
                log::debug!(
                    "Generator: {:?} proposed time: {:?} vs task time: {:?}",
                    gen.address,
                    gen.proposed_time,
                    &task_time
                );
                gen.proposed_time.lt(&task_time)
            })
            .collect();

        log::debug!(
            "Generators with time: {:?} = {}",
            task_time,
            self.generator_markets.len()
        );
        self
    }

    // Filter by state
    pub fn filter_by_state(mut self, states: Vec<GeneratorState>) -> Self {
        let states_set: std::collections::HashSet<_> = states.into_iter().collect();

        self.generator_markets = self
            .generator_markets
            .into_iter()
            .filter(|gen| {
                if let Some(gen_state) = gen.state {
                    states_set.contains(&gen_state)
                } else {
                    false
                }
            })
            .collect();

        log::debug!(
            "Generators with state: {:?} = {}",
            states_set,
            self.generator_markets.len()
        );
        self
    }

    pub fn filter_by_market_id(mut self, market_id: U256) -> Self {
        log::debug!("Filter by market id");
        self.generator_markets = self
            .generator_markets
            .into_iter()
            .filter(|gen| gen.market_id == market_id)
            .collect();
        self
    }

    // Final getter: returns the owned vector.
    pub fn result(self) -> Vec<GeneratorInfoPerMarket> {
        self.generator_markets
    }
}
