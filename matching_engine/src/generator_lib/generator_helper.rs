use super::generator_store::GeneratorInfoPerMarket;
use ethers::core::rand;
use ethers::types::U256;
use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;

#[deprecated(note = "Use weighted_random_selection instead")]
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

pub fn weighted_random_selection(
    vec: Vec<GeneratorInfoPerMarket>,
) -> Option<GeneratorInfoPerMarket> {
    if vec.is_empty() {
        return None;
    }

    let max_proof_generation_cost = vec
        .iter()
        .map(|gen| gen.proof_generation_cost.as_u64())
        .max()
        .unwrap();
    let max_proposed_time = vec
        .iter()
        .map(|gen| gen.proposed_time.as_u64())
        .max()
        .unwrap();
    let max_active_requests = vec
        .iter()
        .map(|gen| gen.active_requests.as_u64())
        .max()
        .unwrap();
    let max_proofs_submitted = vec
        .iter()
        .map(|gen| gen.proofs_submitted.as_u64())
        .max()
        .unwrap();
    let max_proofs_slashed = vec
        .iter()
        .map(|gen| gen.proofs_slashed.as_u64())
        .max()
        .unwrap();

    let weights: Vec<f64> = vec
        .iter()
        .map(|gen| {
            let base = U256::from_dec_str("1000000000000000000").unwrap();
            let proof_generation_cost_weight = U256::one() * 3;
            let proposed_time_weight = U256::one() * 3;
            let active_requests_weight = U256::one() * 2;
            let proofs_submitted_weight = U256::one() * 15;
            let proofs_slashed_weight = U256::one() * 15;

            let proof_generation_cost: U256 =
                proof_generation_cost_weight * gen.proof_generation_cost * base
                    / max_proof_generation_cost;
            let proposed_time: U256 =
                proposed_time_weight * gen.proposed_time * base / max_proposed_time;
            let active_requests: U256 =
                active_requests_weight * gen.active_requests * base / max_active_requests;
            let proofs_submitted: U256 =
                proofs_submitted_weight * gen.proofs_submitted * base / max_proofs_submitted;
            let proofs_slashed: U256 =
                proofs_slashed_weight * gen.proofs_slashed * base / max_proofs_slashed;

            let result = proofs_submitted.as_u64() as f64
                - proof_generation_cost.as_u64() as f64
                - proposed_time.as_u64() as f64
                - active_requests.as_u64() as f64
                - proofs_slashed.as_u64() as f64;

            result
        })
        .collect();

    let min_weight = weights.iter().cloned().fold(f64::INFINITY, f64::min);
    let adjusted_weights: Vec<f64> = weights.iter().map(|&w| w - min_weight + 1.0).collect();

    // println!("{:?}",&weights);
    // println!();
    // println!("{:?}",&adjusted_weights);
    let dist = WeightedIndex::new(&adjusted_weights).unwrap();
    let mut rng = rand::thread_rng();
    let index = dist.sample(&mut rng);

    Some(vec[index].clone())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_generator_info(
        proof_generation_cost: u64,
        proposed_time: u64,
        active_requests: u64,
        proofs_submitted: u64,
        proofs_slashed: u64,
    ) -> GeneratorInfoPerMarket {
        let mut rng = rand::thread_rng();
        let address_bytes: [u8; 20] = rng.gen();

        GeneratorInfoPerMarket {
            proof_generation_cost: U256::from(proof_generation_cost),
            proposed_time: U256::from(proposed_time),
            active_requests: U256::from(active_requests),
            proofs_submitted: U256::from(proofs_submitted),
            proofs_slashed: U256::from(proofs_slashed),
            address: address_bytes.into(),
            compute_required_per_request: U256::one() * 100,
            market_id: U256::one(),
            state: None,
        }
    }

    #[test]
    fn test_weighted_random_selection_empty() {
        let generators = vec![];
        let result = weighted_random_selection(generators);
        assert!(result.is_none());
    }

    #[test]
    fn test_weighted_random_selection_single() {
        let generator = create_generator_info(1, 1, 1, 1, 1);
        let generators = vec![generator.clone()];
        let result = weighted_random_selection(generators);
        assert_eq!(result, Some(generator));
    }

    #[test]
    fn test_weighted_random_selection_multiple() {
        let generator1 = create_generator_info(1, 1, 1, 1, 1);
        let generator2 = create_generator_info(2, 2, 2, 2, 2);
        let generators = vec![generator1.clone(), generator2.clone()];
        let result = weighted_random_selection(generators);
        assert!(result == Some(generator1) || result == Some(generator2));
    }

    #[test]
    fn test_weighted_random_selection_large() {
        let mut generators = vec![];
        for i in 0..100 {
            generators.push(create_generator_info(i, i, i, i, i));
        }
        let result = weighted_random_selection(generators.clone());
        assert!(result.is_some());
        assert!(generators.contains(&result.unwrap()));
    }

    #[test]
    fn test_weighted_random_selection_weights() {
        let generator1 = create_generator_info(1, 1, 1, 10, 1);
        let generator2 = create_generator_info(1, 1, 1, 1, 1);
        let generators = vec![generator1.clone(), generator2.clone()];
        let mut generator1_count = 0;
        let mut generator2_count = 0;

        for _ in 0..1000 {
            let result = weighted_random_selection(generators.clone());
            if result == Some(generator1.clone()) {
                generator1_count += 1;
            } else if result == Some(generator2.clone()) {
                generator2_count += 1;
            }
        }

        assert!(generator1_count > generator2_count);
    }
}
