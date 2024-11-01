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
///`ConfirmedTimestamp(uint256,address,uint256)`
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
pub struct ConfirmedTimestamp {
    pub capture_timestamp: ::ethers::core::types::U256,
    pub transmitter: ::ethers::core::types::Address,
    pub transmitter_comission_rate: ::ethers::core::types::U256,
}
///`JobSlashed(uint256,address,address)`
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
pub struct JobSlashed {
    pub job_id: ::ethers::core::types::U256,
    pub operator: ::ethers::core::types::Address,
    pub reward_address: ::ethers::core::types::Address,
}
///`PoolConfig(uint256,bool)`
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
pub struct PoolConfig {
    pub share: ::ethers::core::types::U256,
    pub enabled: bool,
}
