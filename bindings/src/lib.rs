use ethers::prelude::*;

// Use the abigen! macro to generate bindings for your contract.
// Replace "YourContract" with the desired Rust struct name for your contract.
// Ensure that the path to the ABI file is correct relative to your project root.
abigen!(
    ProofMarketplace,              // The Rust struct name you want for your contract
    "./src/ProofMarketplace.json", // Path to the ABI file
    event_derives(serde::Deserialize, serde::Serialize)  // Derive traits for event structs
);

abigen!(
    ProverManager,              // The Rust struct name you want for your contract
    "./src/ProverManager.json", // Path to the ABI file
    event_derives(serde::Deserialize, serde::Serialize)  // Derive traits for event structs
);

abigen!(
    EntityKeyRegistry,              // The Rust struct name you want for your contract
    "./src/EntityKeyRegistry.json", // Path to the ABI file
    event_derives(serde::Deserialize, serde::Serialize)  // Derive traits for event structs
);

abigen!(
    NativeStaking,              // The Rust struct name you want for your contract
    "./src/NativeStaking.json", // Path to the ABI file
    event_derives(serde::Deserialize, serde::Serialize)  // Derive traits for event structs
);

abigen!(
    StakingManager,              // The Rust struct name you want for your contract
    "./src/StakingManager.json", // Path to the ABI file
    event_derives(serde::Deserialize, serde::Serialize)  // Derive traits for event structs
);

abigen!(
    SymbioticStaking,              // The Rust struct name you want for your contract
    "./src/SymbioticStaking.json", // Path to the ABI file
    event_derives(serde::Deserialize, serde::Serialize)  // Derive traits for event structs
);

abigen!(
    SymbioticStakingReward, // The Rust struct name you want for your contract
    "./src/SymbioticStakingReward.json", // Path to the ABI file
    event_derives(serde::Deserialize, serde::Serialize)  // Derive traits for event structs
);

abigen!(
    IERC20,              // The Rust struct name you want for your contract
    "./src/IERC20.json", // Path to the ABI file
    event_derives(serde::Deserialize, serde::Serialize)  // Derive traits for event structs
);

abigen!(
    Error,              // The Rust struct name you want for your contract
    "./src/Error.json", // Path to the ABI file
    event_derives(serde::Deserialize, serde::Serialize)  // Derive traits for event structs
);
