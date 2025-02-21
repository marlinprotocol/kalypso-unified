use ethers::prelude::*;

/// Represents a range of blocks and the points associated with that range.
struct KalypsoPointRange {
    start_block: u64, // Inclusive start of the block range
    end_block: u64,   // Inclusive end of the block range
    points: U256,     // Points associated with this block range
}

static POINTS_RANGES: Lazy<Vec<KalypsoPointRange>> = Lazy::new(|| {
    let block_ranges = vec![
        KalypsoPointRange {
            start_block: 0,
            end_block: 7930000,
            points: U256::from_dec_str("11").unwrap().pow(18.into()),
        },
        KalypsoPointRange {
            start_block: 8930000,
            end_block: 9930000,
            points: U256::from_dec_str("12").unwrap().pow(18.into()),
        },
        KalypsoPointRange {
            start_block: 9930000,
            end_block: 10930000,
            points: U256::from_dec_str("13").unwrap().pow(18.into()),
        },
        KalypsoPointRange {
            start_block: 9930000,
            end_block: u64::MAX,
            points: U256::from_dec_str("14").unwrap().pow(18.into()),
        }, // Add more KalypsoPointRange entries as needed
    ];

    block_ranges
});

pub fn get_points(block_number: u64) -> U256 {
    for range in POINTS_RANGES.iter() {
        if block_number >= range.start_block && block_number <= range.end_block {
            return range.points;
        }
    }
    // If block_number doesn't fall within any range, return zero points.
    U256::zero()
}
