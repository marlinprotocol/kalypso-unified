///`EnclaveImage(bytes,bytes,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct EnclaveImage {
    pub pcr0: ::ethers::core::types::Bytes,
    pub pcr1: ::ethers::core::types::Bytes,
    pub pcr2: ::ethers::core::types::Bytes,
}
///`Attestation(bytes,bytes,bytes,bytes,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Attestation {
    pub enclave_pub_key: ::ethers::core::types::Bytes,
    pub pcr0: ::ethers::core::types::Bytes,
    pub pcr1: ::ethers::core::types::Bytes,
    pub pcr2: ::ethers::core::types::Bytes,
    pub timestamp_in_milliseconds: ::ethers::core::types::U256,
}
///`Ask(uint256,uint256,uint256,uint256,uint256,address,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Ask {
    pub market_id: ::ethers::core::types::U256,
    pub reward: ::ethers::core::types::U256,
    pub expiry: ::ethers::core::types::U256,
    pub time_taken_for_proof_generation: ::ethers::core::types::U256,
    pub deadline: ::ethers::core::types::U256,
    pub refund_address: ::ethers::core::types::Address,
    pub prover_data: ::ethers::core::types::Bytes,
}
