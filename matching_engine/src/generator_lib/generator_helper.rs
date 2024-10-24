use super::generator_store::GeneratorInfoPerMarket;
use ethers::core::rand;
use rand::Rng;

pub fn random_generator_selection(
    vec: Vec<GeneratorInfoPerMarket>,
) -> Option<GeneratorInfoPerMarket> {
    if vec.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let element = &vec[rng.gen_range(0..vec.len())];
        Some(element.clone())
    }
}

pub fn select_idle_generators(
    generators: Vec<GeneratorInfoPerMarket>,
) -> Vec<GeneratorInfoPerMarket> {
    let mut to_return = vec![];
    for generator in generators {
        to_return.push(generator.clone());
    }

    to_return
}
