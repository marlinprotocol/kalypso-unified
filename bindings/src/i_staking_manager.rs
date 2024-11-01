pub use i_staking_manager::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod i_staking_manager {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getPoolConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPoolConfig"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Struct.PoolConfig"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onJobCompletion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onJobCompletion"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("jobId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("feePaid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onJobCreation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onJobCreation"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("jobId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onSlashResult"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onSlashResult"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("slashedJobs"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Struct.JobSlashed[]",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FeeTokenSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FeeTokenSet"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("feeToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolEnabledSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PoolEnabledSet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("enabled"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolRewardShareSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PoolRewardShareSet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pools"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProofMarketplaceSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ProofMarketplaceSet",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("proofMarketplace"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakingPoolAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakingPoolAdded"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakingPoolRemoved"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakingPoolRemoved"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SymbioticStakingSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SymbioticStakingSet",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("symbioticStaking"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ISTAKINGMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IStakingManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IStakingManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IStakingManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IStakingManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IStakingManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IStakingManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IStakingManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ISTAKINGMANAGER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `getPoolConfig` (0xf29486a1) function
        pub fn get_pool_config(
            &self,
            pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, PoolConfig> {
            self.0
                .method_hash([242, 148, 134, 161], pool)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onJobCompletion` (0xc0ff97ef) function
        pub fn on_job_completion(
            &self,
            job_id: ::ethers::core::types::U256,
            operator: ::ethers::core::types::Address,
            fee_paid: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 255, 151, 239], (job_id, operator, fee_paid))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onJobCreation` (0xf674a5ee) function
        pub fn on_job_creation(
            &self,
            job_id: ::ethers::core::types::U256,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 116, 165, 238], (job_id, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onSlashResult` (0x4d22fd31) function
        pub fn on_slash_result(
            &self,
            slashed_jobs: ::std::vec::Vec<JobSlashed>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 34, 253, 49], slashed_jobs)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FeeTokenSet` event
        pub fn fee_token_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeTokenSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PoolEnabledSet` event
        pub fn pool_enabled_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PoolEnabledSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PoolRewardShareSet` event
        pub fn pool_reward_share_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PoolRewardShareSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ProofMarketplaceSet` event
        pub fn proof_marketplace_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProofMarketplaceSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakingPoolAdded` event
        pub fn staking_pool_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakingPoolAddedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakingPoolRemoved` event
        pub fn staking_pool_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakingPoolRemovedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SymbioticStakingSet` event
        pub fn symbiotic_staking_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SymbioticStakingSetFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IStakingManagerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IStakingManager<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "FeeTokenSet", abi = "FeeTokenSet(address)")]
    pub struct FeeTokenSetFilter {
        #[ethevent(indexed)]
        pub fee_token: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PoolEnabledSet", abi = "PoolEnabledSet(address,bool)")]
    pub struct PoolEnabledSetFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "PoolRewardShareSet",
        abi = "PoolRewardShareSet(address[],uint256[])"
    )]
    pub struct PoolRewardShareSetFilter {
        #[ethevent(indexed)]
        pub pools: ::ethers::core::types::H256,
        pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ProofMarketplaceSet", abi = "ProofMarketplaceSet(address)")]
    pub struct ProofMarketplaceSetFilter {
        #[ethevent(indexed)]
        pub proof_marketplace: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "StakingPoolAdded", abi = "StakingPoolAdded(address)")]
    pub struct StakingPoolAddedFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "StakingPoolRemoved", abi = "StakingPoolRemoved(address)")]
    pub struct StakingPoolRemovedFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SymbioticStakingSet", abi = "SymbioticStakingSet(address)")]
    pub struct SymbioticStakingSetFilter {
        #[ethevent(indexed)]
        pub symbiotic_staking: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum IStakingManagerEvents {
        FeeTokenSetFilter(FeeTokenSetFilter),
        PoolEnabledSetFilter(PoolEnabledSetFilter),
        PoolRewardShareSetFilter(PoolRewardShareSetFilter),
        ProofMarketplaceSetFilter(ProofMarketplaceSetFilter),
        StakingPoolAddedFilter(StakingPoolAddedFilter),
        StakingPoolRemovedFilter(StakingPoolRemovedFilter),
        SymbioticStakingSetFilter(SymbioticStakingSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for IStakingManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FeeTokenSetFilter::decode_log(log) {
                return Ok(IStakingManagerEvents::FeeTokenSetFilter(decoded));
            }
            if let Ok(decoded) = PoolEnabledSetFilter::decode_log(log) {
                return Ok(IStakingManagerEvents::PoolEnabledSetFilter(decoded));
            }
            if let Ok(decoded) = PoolRewardShareSetFilter::decode_log(log) {
                return Ok(IStakingManagerEvents::PoolRewardShareSetFilter(decoded));
            }
            if let Ok(decoded) = ProofMarketplaceSetFilter::decode_log(log) {
                return Ok(IStakingManagerEvents::ProofMarketplaceSetFilter(decoded));
            }
            if let Ok(decoded) = StakingPoolAddedFilter::decode_log(log) {
                return Ok(IStakingManagerEvents::StakingPoolAddedFilter(decoded));
            }
            if let Ok(decoded) = StakingPoolRemovedFilter::decode_log(log) {
                return Ok(IStakingManagerEvents::StakingPoolRemovedFilter(decoded));
            }
            if let Ok(decoded) = SymbioticStakingSetFilter::decode_log(log) {
                return Ok(IStakingManagerEvents::SymbioticStakingSetFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IStakingManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FeeTokenSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolEnabledSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolRewardShareSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketplaceSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakingPoolAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakingPoolRemovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbioticStakingSetFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FeeTokenSetFilter> for IStakingManagerEvents {
        fn from(value: FeeTokenSetFilter) -> Self {
            Self::FeeTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<PoolEnabledSetFilter> for IStakingManagerEvents {
        fn from(value: PoolEnabledSetFilter) -> Self {
            Self::PoolEnabledSetFilter(value)
        }
    }
    impl ::core::convert::From<PoolRewardShareSetFilter> for IStakingManagerEvents {
        fn from(value: PoolRewardShareSetFilter) -> Self {
            Self::PoolRewardShareSetFilter(value)
        }
    }
    impl ::core::convert::From<ProofMarketplaceSetFilter> for IStakingManagerEvents {
        fn from(value: ProofMarketplaceSetFilter) -> Self {
            Self::ProofMarketplaceSetFilter(value)
        }
    }
    impl ::core::convert::From<StakingPoolAddedFilter> for IStakingManagerEvents {
        fn from(value: StakingPoolAddedFilter) -> Self {
            Self::StakingPoolAddedFilter(value)
        }
    }
    impl ::core::convert::From<StakingPoolRemovedFilter> for IStakingManagerEvents {
        fn from(value: StakingPoolRemovedFilter) -> Self {
            Self::StakingPoolRemovedFilter(value)
        }
    }
    impl ::core::convert::From<SymbioticStakingSetFilter> for IStakingManagerEvents {
        fn from(value: SymbioticStakingSetFilter) -> Self {
            Self::SymbioticStakingSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `getPoolConfig` function with signature `getPoolConfig(address)` and selector `0xf29486a1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPoolConfig", abi = "getPoolConfig(address)")]
    pub struct GetPoolConfigCall {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `onJobCompletion` function with signature `onJobCompletion(uint256,address,uint256)` and selector `0xc0ff97ef`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "onJobCompletion",
        abi = "onJobCompletion(uint256,address,uint256)"
    )]
    pub struct OnJobCompletionCall {
        pub job_id: ::ethers::core::types::U256,
        pub operator: ::ethers::core::types::Address,
        pub fee_paid: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `onJobCreation` function with signature `onJobCreation(uint256,address)` and selector `0xf674a5ee`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "onJobCreation", abi = "onJobCreation(uint256,address)")]
    pub struct OnJobCreationCall {
        pub job_id: ::ethers::core::types::U256,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `onSlashResult` function with signature `onSlashResult((uint256,address,address)[])` and selector `0x4d22fd31`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "onSlashResult",
        abi = "onSlashResult((uint256,address,address)[])"
    )]
    pub struct OnSlashResultCall {
        pub slashed_jobs: ::std::vec::Vec<JobSlashed>,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum IStakingManagerCalls {
        GetPoolConfig(GetPoolConfigCall),
        OnJobCompletion(OnJobCompletionCall),
        OnJobCreation(OnJobCreationCall),
        OnSlashResult(OnSlashResultCall),
    }
    impl ::ethers::core::abi::AbiDecode for IStakingManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetPoolConfigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolConfig(decoded));
            }
            if let Ok(decoded) =
                <OnJobCompletionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnJobCompletion(decoded));
            }
            if let Ok(decoded) = <OnJobCreationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnJobCreation(decoded));
            }
            if let Ok(decoded) = <OnSlashResultCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnSlashResult(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IStakingManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetPoolConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnJobCompletion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnJobCreation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnSlashResult(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IStakingManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetPoolConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnJobCompletion(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnJobCreation(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnSlashResult(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetPoolConfigCall> for IStakingManagerCalls {
        fn from(value: GetPoolConfigCall) -> Self {
            Self::GetPoolConfig(value)
        }
    }
    impl ::core::convert::From<OnJobCompletionCall> for IStakingManagerCalls {
        fn from(value: OnJobCompletionCall) -> Self {
            Self::OnJobCompletion(value)
        }
    }
    impl ::core::convert::From<OnJobCreationCall> for IStakingManagerCalls {
        fn from(value: OnJobCreationCall) -> Self {
            Self::OnJobCreation(value)
        }
    }
    impl ::core::convert::From<OnSlashResultCall> for IStakingManagerCalls {
        fn from(value: OnSlashResultCall) -> Self {
            Self::OnSlashResult(value)
        }
    }
    ///Container type for all return fields from the `getPoolConfig` function with signature `getPoolConfig(address)` and selector `0xf29486a1`
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
    pub struct GetPoolConfigReturn(pub PoolConfig);
}
