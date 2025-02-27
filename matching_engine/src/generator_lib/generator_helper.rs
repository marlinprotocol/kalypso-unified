use std::collections::HashMap;

use super::generator_store::GeneratorInfoPerMarket;
use ethers::abi::Address;
use ethers::core::rand;
use ethers::types::U256;

use rand::distributions::{Distribution, WeightedIndex};
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

pub fn weighted_time_cost_random_selection(
    vec: Vec<GeneratorInfoPerMarket>,
    missed_jobs: HashMap<Address, usize>,
) -> Option<GeneratorInfoPerMarket> {
    if vec.is_empty() {
        return None;
    }

    // Find the maximum values (if they are zero, replace with one to avoid division by zero)
    let max_proof_generation_cost = vec
        .iter()
        .map(|gen| gen.proof_generation_cost)
        .max()
        .unwrap_or(U256::one());
    let max_proposed_time = vec
        .iter()
        .map(|gen| gen.proposed_time)
        .max()
        .unwrap_or(U256::one());
    let max_proof_generation_cost = if max_proof_generation_cost.is_zero() {
        U256::one()
    } else {
        max_proof_generation_cost
    };
    let max_proposed_time = if max_proposed_time.is_zero() {
        U256::one()
    } else {
        max_proposed_time
    };

    log::debug!(
        "Max Proof Generation Cost: {}",
        max_proof_generation_cost.to_string()
    );
    log::debug!(
        "Max Proof Generation Time: {}",
        max_proposed_time.to_string()
    );

    // Compute a positive weight for each candidate.
    // Convert U256 values to f64 (assuming they fit into u64) and normalize.
    let weights: Vec<f64> = vec
        .iter()
        .map(|gen| {
            log::debug!("============#================");
            let cost = if gen.proof_generation_cost.is_zero() {
                U256::one()
            } else {
                gen.proof_generation_cost
            }
            .as_u64() as f64;
            let time = gen.proposed_time.as_u64() as f64;
            let max_cost = max_proof_generation_cost.as_u64() as f64;
            let max_time = max_proposed_time.as_u64() as f64;

            log::debug!("Generator: {:?} cost: {} time: {}", gen.address, cost, time);

            // Normalize (0 is best, 1 is worst)
            let norm_cost = cost / max_cost;
            let norm_time = time / max_time;
            log::debug!(
                "Generator: {:?} norm_cost: {} norm_time: {}",
                gen.address,
                norm_cost,
                norm_time
            );
            // Higher weight is better (prefer lower cost and time).
            // We add 1.0 so that even the worst candidate gets a strictly positive weight.
            let mut weight =
                (1.0 * (1.0 / norm_cost) * (1.0 / norm_time)).clamp(f64::MIN, f64::MAX);

            log::debug!("Generator: {:?} clamped_weights: {}", gen.address, weight);

            // Exponentially decrease weight with the number of missed jobs.
            // For each missed job, multiply weight by 0.5.
            if let Some(&missed) = missed_jobs.get(&gen.address) {
                weight *= 2_f64.powi(-(missed as i32));
            }

            log::debug!(
                "Generator: {:?} adjusted weighted for missed jobs: {}",
                gen.address,
                weight
            );

            // Ensure weight is strictly positive.
            if weight <= 0.0 {
                weight = f64::MIN;
            }

            log::debug!("Generator: {:?} weight: {}", gen.address, weight);

            log::debug!("============#================");
            weight
        })
        .collect();

    let generators: Vec<Address> = vec.iter().map(|gen| gen.address).collect();
    log::debug!("Generators used in weight calc. {:?}", generators);
    log::debug!(
        "Weights: {:?}. Total Weight {:?}",
        weights,
        weights
            .iter()
            .copied()
            .reduce(|a, x| a + x)
            .unwrap_or_default()
    );

    // Create a weighted distribution and sample an index.
    let dist = WeightedIndex::new(&weights).ok()?;
    let mut rng = rand::thread_rng();
    let index = dist.sample(&mut rng);

    Some(vec[index].clone())
}

#[deprecated(note = "use weighted_time_cost_random_selection")]
pub fn weighted_random_selection(
    vec: Vec<GeneratorInfoPerMarket>,
) -> Option<GeneratorInfoPerMarket> {
    if vec.is_empty() {
        return None;
    }

    let max_proof_generation_cost = vec
        .iter()
        .map(|gen| gen.proof_generation_cost)
        .max()
        .unwrap_or(U256::one());
    let max_proposed_time = vec
        .iter()
        .map(|gen| gen.proposed_time)
        .max()
        .unwrap_or(U256::one());
    let max_active_requests = vec
        .iter()
        .map(|gen| gen.active_requests)
        .max()
        .unwrap_or(U256::one());
    let max_proofs_submitted = vec
        .iter()
        .map(|gen| gen.proofs_submitted)
        .max()
        .unwrap_or(U256::one());
    let max_proofs_slashed = vec
        .iter()
        .map(|gen| gen.proofs_slashed)
        .max()
        .unwrap_or(U256::one());

    let max_proof_generation_cost = if max_proof_generation_cost.is_zero() {
        U256::one()
    } else {
        max_proof_generation_cost
    };
    let max_proposed_time = if max_proposed_time.is_zero() {
        U256::one()
    } else {
        max_proposed_time
    };
    let max_active_requests = if max_active_requests.is_zero() {
        U256::one()
    } else {
        max_active_requests
    };
    let max_proofs_submitted = if max_proofs_submitted.is_zero() {
        U256::one()
    } else {
        max_proofs_submitted
    };
    let max_proofs_slashed = if max_proofs_slashed.is_zero() {
        U256::one()
    } else {
        max_proofs_slashed
    };

    let weights: Vec<f64> = vec
        .iter()
        .map(|gen| {
            let base = U256::from_dec_str("1000000000000000000").unwrap_or(U256::one());
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

    #[test]
    fn test_weighted_time_cost_random_selection_empty() {
        let generators = vec![];
        let missed_jobs = HashMap::new();
        let result = weighted_time_cost_random_selection(generators, missed_jobs);
        assert!(result.is_none());
    }

    #[test]
    fn test_weighted_time_cost_random_selection_single() {
        let generator = create_generator_info(1, 1, 1, 1, 1);
        let generators = vec![generator.clone()];
        let missed_jobs = HashMap::new();
        let result = weighted_time_cost_random_selection(generators, missed_jobs);
        assert_eq!(result, Some(generator));
    }

    #[test]
    fn test_weighted_time_cost_random_selection_multiple() {
        let generator1 = create_generator_info(1, 1, 1, 1, 1);
        let generator2 = create_generator_info(2, 2, 2, 2, 2);
        let generators = vec![generator1.clone(), generator2.clone()];
        let missed_jobs = HashMap::new();
        let result = weighted_time_cost_random_selection(generators, missed_jobs);
        assert!(result == Some(generator1) || result == Some(generator2));
    }

    #[test]
    fn test_weighted_time_cost_random_selection_large() {
        let mut generators = vec![];
        for i in 0..100 {
            generators.push(create_generator_info(i, i, i, i, i));
        }
        let missed_jobs = HashMap::new();
        let result = weighted_time_cost_random_selection(generators.clone(), missed_jobs);
        assert!(result.is_some());
        assert!(generators.contains(&result.unwrap()));
    }

    #[test]
    fn test_weighted_time_cost_random_selection_with_missed_jobs() {
        let generator1 = create_generator_info(1, 1, 1, 1, 1);
        let generator2 = create_generator_info(1, 1, 1, 1, 1);
        let generators = vec![generator1.clone(), generator2.clone()];
        let mut missed_jobs = HashMap::new();
        missed_jobs.insert(generator1.address, 5);
        missed_jobs.insert(generator2.address, 0);

        let mut generator1_count = 0;
        let mut generator2_count = 0;

        for _ in 0..100000 {
            let result =
                weighted_time_cost_random_selection(generators.clone(), missed_jobs.clone());
            if result == Some(generator1.clone()) {
                generator1_count += 1;
            } else if result == Some(generator2.clone()) {
                generator2_count += 1;
            }
        }

        let expected_ratio = 2_f64.powi(5);
        let actual_ratio = generator2_count as f64 / generator1_count as f64;
        assert!(
            (actual_ratio - expected_ratio).abs() < 0.1 * expected_ratio,
            "Expected ratio: {}, Actual ratio: {}",
            expected_ratio,
            actual_ratio
        );
    }

    #[test]
    fn test_weighted_time_cost_random_selection_proposed_time_variation() {
        let generator1 = create_generator_info(1, 1, 1, 1, 1);
        let generator2 = create_generator_info(1, 12, 1, 1, 1);
        let generators = vec![generator1.clone(), generator2.clone()];
        let mut missed_jobs = HashMap::new();
        missed_jobs.insert(generator1.address, 0);
        missed_jobs.insert(generator2.address, 0);

        let mut generator1_count = 0;
        let mut generator2_count = 0;

        for _ in 0..100000 {
            let result =
                weighted_time_cost_random_selection(generators.clone(), missed_jobs.clone());
            if result == Some(generator1.clone()) {
                generator1_count += 1;
            } else if result == Some(generator2.clone()) {
                generator2_count += 1;
            }
        }

        let expected_ratio = 12.0;
        let actual_ratio = generator1_count as f64 / generator2_count as f64;

        assert!(
            (actual_ratio - expected_ratio).abs() < 0.1 * expected_ratio,
            "Expected ratio: {}, Actual ratio: {}",
            expected_ratio,
            actual_ratio
        );
    }

    #[test]
    fn test_weighted_time_cost_random_selection_cost_variation() {
        let generator1 = create_generator_info(1, 1, 1, 1, 1);
        let generator2 = create_generator_info(98, 1, 1, 1, 1);
        let generators = vec![generator1.clone(), generator2.clone()];
        let mut missed_jobs = HashMap::new();
        missed_jobs.insert(generator1.address, 0);
        missed_jobs.insert(generator2.address, 0);

        let mut generator1_count = 0;
        let mut generator2_count = 0;

        for _ in 0..100000 {
            let result =
                weighted_time_cost_random_selection(generators.clone(), missed_jobs.clone());
            if result == Some(generator1.clone()) {
                generator1_count += 1;
            } else if result == Some(generator2.clone()) {
                generator2_count += 1;
            }
        }

        let expected_ratio = 98.0;
        let actual_ratio = generator1_count as f64 / generator2_count as f64;

        assert!(
            (actual_ratio - expected_ratio).abs() < 0.1 * expected_ratio,
            "Expected ratio: {}, Actual ratio: {}",
            expected_ratio,
            actual_ratio
        );
    }

    #[test]
    fn test_weighted_time_cost_random_selection_time_and_cost_variation() {
        let generator1 = create_generator_info(2, 1, 1, 1, 1);
        let generator2 = create_generator_info(1, 2, 1, 1, 1);
        let generators = vec![generator1.clone(), generator2.clone()];
        let mut missed_jobs = HashMap::new();
        missed_jobs.insert(generator1.address, 0);
        missed_jobs.insert(generator2.address, 0);

        let mut generator1_count = 0;
        let mut generator2_count = 0;

        for _ in 0..100000 {
            let result =
                weighted_time_cost_random_selection(generators.clone(), missed_jobs.clone());
            if result == Some(generator1.clone()) {
                generator1_count += 1;
            } else if result == Some(generator2.clone()) {
                generator2_count += 1;
            }
        }

        let expected_ratio = 1.0;
        let actual_ratio = generator1_count as f64 / generator2_count as f64;

        assert!(
            (actual_ratio - expected_ratio).abs() < 0.01 * expected_ratio,
            "Expected ratio: {}, Actual ratio: {}",
            expected_ratio,
            actual_ratio
        );
    }

    #[test]
    fn test_weighted_time_cost_random_selection_time_and_cost_variation_2() {
        let generator1 = create_generator_info(50000, 5000, 0, 0, 0);
        let generator2 = create_generator_info(100000, 10000, 0, 0, 0);
        let generator3 = create_generator_info(1000000, 10000, 0, 0, 0);
        let generator4 = create_generator_info(100000, 10000, 0, 0, 0);
        let generator5 = create_generator_info(1000000, 10000, 0, 0, 0);
        let generator6 = create_generator_info(1000000, 10000, 0, 0, 0);
        let generator7 = create_generator_info(1000000, 10000, 0, 0, 0);
        let generator8 = create_generator_info(1000000, 10000, 0, 0, 0);
        let generator9 = create_generator_info(1000000, 10000, 0, 0, 0);
        let generator10 = create_generator_info(1000000, 10000, 0, 0, 0);
        let generator11 = create_generator_info(1000000, 10000, 0, 0, 0);

        let generators = vec![
            generator1.clone(),
            generator2.clone(),
            generator3.clone(),
            generator4.clone(),
            generator5.clone(),
            generator6.clone(),
            generator7.clone(),
            generator8.clone(),
            generator9.clone(),
            generator10.clone(),
            generator11.clone(),
        ];

        let mut missed_jobs = HashMap::new();
        missed_jobs.insert(generator1.address, 0);
        missed_jobs.insert(generator2.address, 1);
        missed_jobs.insert(generator3.address, 0);
        missed_jobs.insert(generator4.address, 0);
        missed_jobs.insert(generator5.address, 0);
        missed_jobs.insert(generator6.address, 2);
        missed_jobs.insert(generator7.address, 3);
        missed_jobs.insert(generator8.address, 0);
        missed_jobs.insert(generator9.address, 0);
        missed_jobs.insert(generator10.address, 0);
        missed_jobs.insert(generator11.address, 0);

        let total_count = 10000;
        let mut selection_counts: HashMap<_, usize> = HashMap::new();

        for _ in 0..total_count {
            if let Some(selected) =
                weighted_time_cost_random_selection(generators.clone(), missed_jobs.clone())
            {
                *selection_counts.entry(selected.address).or_insert(0) += 1;
            }
        }

        println!("Selection Ratios:");
        for generator in &generators {
            let count = selection_counts
                .get(&generator.address)
                .copied()
                .unwrap_or(0);
            let ratio = count as f64 / total_count as f64;
            println!("Generator {}: {:.6}", generator.address, ratio);
        }
    }
}
