use ethers::types::{Bytes, H256};
use ethers::utils::keccak256;

pub fn get_kalypso_image_id_from_pcrs(pcr0: Bytes, pcr1: Bytes, pcr2: Bytes) -> H256 {
    let mut data = Vec::new();
    data.extend_from_slice(&pcr0);
    data.extend_from_slice(&pcr1);
    data.extend_from_slice(&pcr2);

    // Compute the keccak256 hash
    keccak256(data).into()
}

pub fn hashed_image_id_for_non_confidential_market() -> H256 {
    let pcr0 = vec![0; 48];
    let pcr1 = vec![0; 48];
    let pcr2 = vec![0; 48];

    let mut data = Vec::new();
    data.extend_from_slice(&pcr0);
    data.extend_from_slice(&pcr1);
    data.extend_from_slice(&pcr2);

    // Compute the keccak256 hash
    keccak256(data).into()
}
