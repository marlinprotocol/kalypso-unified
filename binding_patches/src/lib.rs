// src/lib.rs

use ethers::prelude::*;

// Use the abigen! macro to generate bindings for your contract.
// Replace "YourContract" with the desired Rust struct name for your contract.
// Ensure that the path to the ABI file is correct relative to your project root.
abigen!(
    UpdateProofMarketplaceMetadataPatch, // The Rust struct name you want for your contract
    "./src/proof_marketplace.json",      // Path to the ABI file
    event_derives(serde::Deserialize, serde::Serialize)  // Derive traits for event structs
);

// Re-export the generated contract bindings to make them accessible when your library is used as a dependency.

abigen!(
    GeneratorRegistryPatch,
    "./src/generator_registry.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    VaultSnapshotPatch,
    "./src/vaultsnapshot.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub use GeneratorRegistryPatch;
pub use UpdateProofMarketplaceMetadataPatch;
pub use VaultSnapshotPatch;
