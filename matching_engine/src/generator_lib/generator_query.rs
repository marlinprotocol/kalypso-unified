use super::{generator_state::GeneratorState, generator_store::GeneratorInfoPerMarket};
use ethers::prelude::*;

#[derive(Clone)]
pub struct GeneratorQueryResult {
    generator_markets: Vec<GeneratorInfoPerMarket>,
}

impl GeneratorQueryResult {
    // Initialize with a collection of generators
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
        let states_set: std::collections::HashSet<_> = states.into_iter().collect(); // Convert Vec to HashSet for fast lookup

        self.generator_markets = self
            .generator_markets
            .into_iter() // Use rayon's parallel iterator
            .filter(|gen| {
                if let Some(gen_state) = gen.state {
                    states_set.contains(&gen_state) // Check if the generator state is in the provided states
                } else {
                    false // If the generator has no state, exclude it
                }
            })
            .collect(); // Collect the filtered generator markets into a Vec

        log::debug!(
            "Generators with state: {:?} = {}",
            states_set,
            self.generator_markets.len()
        );
        self
    }

    #[allow(unused)]
    pub fn filter_by_market_id(mut self, market_id: U256) -> Self {
        log::debug!("Filter by market id");
        // Use rayon's par_iter_mut to process the vector in parallel
        self.generator_markets = self
            .generator_markets
            .into_iter() // Convert to a parallel iterator
            .filter(|gen| gen.market_id == market_id) // Filter in parallel
            .collect(); // Collect back into a Vec
        self
    }

    // Final getter to consume the object and retrieve the filtered generators
    pub fn result(self) -> Vec<GeneratorInfoPerMarket> {
        // Clone the elements or map them to owned values
        self.generator_markets
    }
}
