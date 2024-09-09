use super::generator_store::GeneratorInfoPerMarket;
use ethers::core::rand;
use rand::Rng;
use std::ops::{AddAssign, Div};

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

pub fn idle_generator_selector(
    generators: Vec<&GeneratorInfoPerMarket>,
) -> Vec<GeneratorInfoPerMarket> {
    // sort generators based on total stake
    let vec_by_stake = sort_by_total_stake(generators.clone());

    // sort generators based on proof generation cost
    let vec_by_cost = sort_by_proof_generation_cost(generators.clone());

    // sort generators based on proofs submitted
    let vec_by_proofs = sort_by_proofs_submitted(generators.clone());

    // sort generator based on proof generation time
    let vec_by_time = sort_by_proposed_time(generators.clone());

    // Calculating generator score and collecting values
    let mut generator_percentiles = vec![];
    for elem in generators {
        // calculating percentile by total stake
        let percentile_by_stake = get_percentile_by_position(&vec_by_stake, elem);

        // calculating percentile by proof generation cost
        let percentile_by_cost = get_percentile_by_position(&vec_by_cost, elem);

        // calculating percentile by proofs submitted
        let percentile_by_proofs = get_percentile_by_position(&vec_by_proofs, elem);

        // calculating percentile by proof generation time
        let percentile_by_time = get_percentile_by_position(&vec_by_time, elem);

        // calculating generator score for each generator
        let percentile_weights = vec![
            ((100.0 - percentile_by_time), 40.0),
            (percentile_by_stake, 30.0),
            (percentile_by_proofs, 20.0),
            ((100.0 - percentile_by_cost), 10.0),
        ];

        let generator_score = get_generator_score(percentile_weights);

        // Collecting it in a vector
        generator_percentiles.push((elem.clone(), unsafe {
            generator_score.floor().to_int_unchecked::<usize>()
        }));
    }

    // Sorting generators based on scores
    generator_percentiles.sort_by(|a, b| a.1.cmp(&b.1));
    generator_percentiles.reverse();

    // Selecting only the generators with 5 highest generator scores
    let mut to_return = vec![];
    let mut counter = 5;
    for elem in generator_percentiles {
        to_return.push(elem.0.clone());
        counter -= 1;
        if counter == 0 {
            break;
        }
    }
    to_return
}

fn sort_by_total_stake(
    mut generators: Vec<&GeneratorInfoPerMarket>,
) -> Vec<&GeneratorInfoPerMarket> {
    generators.sort_by(|a, b| a.total_stake.cmp(&b.total_stake));
    generators
}

fn sort_by_proof_generation_cost(
    mut generators: Vec<&GeneratorInfoPerMarket>,
) -> Vec<&GeneratorInfoPerMarket> {
    generators.sort_by(|a, b| a.proof_generation_cost.cmp(&b.proof_generation_cost));
    generators
}

fn sort_by_proposed_time(
    mut generators: Vec<&GeneratorInfoPerMarket>,
) -> Vec<&GeneratorInfoPerMarket> {
    generators.sort_by(|a, b| a.proposed_time.cmp(&b.proposed_time));
    generators
}

fn sort_by_proofs_submitted(
    mut generators: Vec<&GeneratorInfoPerMarket>,
) -> Vec<&GeneratorInfoPerMarket> {
    generators.sort_by(|a, b| a.proofs_submitted.cmp(&b.proofs_submitted));
    generators
}

fn get_percentile_by_position(
    vec: &[&GeneratorInfoPerMarket],
    generator: &GeneratorInfoPerMarket,
) -> f64 {
    let index = vec.iter().position(|&x| x == generator).unwrap() as f64;
    let total_generators = vec.len() as f64;

    (index / total_generators) * 100_f64
}
fn get_generator_score(vec: Vec<(f64, f64)>) -> f64 {
    let mut sum: f64 = 0.0;
    for elem in vec {
        sum.add_assign(elem.0 * elem.1);
    }

    sum.div(100.0)
}
