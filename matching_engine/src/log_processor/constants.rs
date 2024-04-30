use ethers::prelude::H256;
use lazy_static::lazy_static;
use std::{collections::HashSet, str::FromStr};

// Define the lazy_static set
lazy_static! {
    pub static ref TOPICS_TO_SKIP: HashSet<H256> = {
        let mut set = HashSet::new();
        // Initialize your set here
        set.insert(H256::from_str("bc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b").unwrap()); // UUPS.Upgraded
        set.insert(H256::from_str("2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d").unwrap()); // AccessControl.RoleGranted
        set.insert(H256::from_str("bd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff").unwrap()); // AccessControl.RoleAdminChanged
        set.insert(H256::from_str("7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498").unwrap()); // Initializable.Initialized
        set
    };
}
