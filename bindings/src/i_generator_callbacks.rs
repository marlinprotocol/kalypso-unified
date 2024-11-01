pub use i_generator_callbacks::*;
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
pub mod i_generator_callbacks {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addStakeCallback"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addStakeCallback"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("intendToReduceStakeCallback"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("intendToReduceStakeCallback",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("removeStakeCallback"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeStakeCallback",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("stakeLockImposedCallback"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeLockImposedCallback",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("stakeLockReleasedCallback"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeLockReleasedCallback",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("stakeSlashedCallback"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeSlashedCallback",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("symbioticCompleteSnapshotCallback"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("symbioticCompleteSnapshotCallback",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("captureTimestamp"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("AddedStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddedStake"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemovedStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RemovedStake"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RequestStakeDecrease"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RequestStakeDecrease",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakeLockImposed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeLockImposed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stake"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakeLockReleased"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeLockReleased"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stake"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakeSlashed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeSlashed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stake"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SymbioticCompleteSnapshot"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SymbioticCompleteSnapshot",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("captureTimestamp"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
    pub static IGENERATORCALLBACKS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IGeneratorCallbacks<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IGeneratorCallbacks<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IGeneratorCallbacks<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IGeneratorCallbacks<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IGeneratorCallbacks<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IGeneratorCallbacks))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IGeneratorCallbacks<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IGENERATORCALLBACKS_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `addStakeCallback` (0xa63616e6) function
        pub fn add_stake_callback(
            &self,
            generator_address: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 54, 22, 230], (generator_address, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intendToReduceStakeCallback` (0x078159cd) function
        pub fn intend_to_reduce_stake_callback(
            &self,
            generator_address: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 129, 89, 205], (generator_address, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeStakeCallback` (0xcb80d418) function
        pub fn remove_stake_callback(
            &self,
            generator_address: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 128, 212, 24], (generator_address, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeLockImposedCallback` (0x201a11b3) function
        pub fn stake_lock_imposed_callback(
            &self,
            generator_address: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 26, 17, 179], (generator_address, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeLockReleasedCallback` (0x31b526ba) function
        pub fn stake_lock_released_callback(
            &self,
            generator_address: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 181, 38, 186], (generator_address, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeSlashedCallback` (0x61413f0f) function
        pub fn stake_slashed_callback(
            &self,
            generator_address: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 65, 63, 15], (generator_address, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbioticCompleteSnapshotCallback` (0x056a7668) function
        pub fn symbiotic_complete_snapshot_callback(
            &self,
            capture_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 106, 118, 104], capture_timestamp)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddedStake` event
        pub fn added_stake_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddedStakeFilter> {
            self.0.event()
        }
        ///Gets the contract's `RemovedStake` event
        pub fn removed_stake_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemovedStakeFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RequestStakeDecrease` event
        pub fn request_stake_decrease_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RequestStakeDecreaseFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeLockImposed` event
        pub fn stake_lock_imposed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeLockImposedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeLockReleased` event
        pub fn stake_lock_released_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeLockReleasedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeSlashed` event
        pub fn stake_slashed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeSlashedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SymbioticCompleteSnapshot` event
        pub fn symbiotic_complete_snapshot_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SymbioticCompleteSnapshotFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IGeneratorCallbacksEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IGeneratorCallbacks<M>
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
    #[ethevent(name = "AddedStake", abi = "AddedStake(address,address,uint256)")]
    pub struct AddedStakeFilter {
        #[ethevent(indexed)]
        pub generator_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "RemovedStake", abi = "RemovedStake(address,address,uint256)")]
    pub struct RemovedStakeFilter {
        #[ethevent(indexed)]
        pub generator_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "RequestStakeDecrease",
        abi = "RequestStakeDecrease(address,address,uint256)"
    )]
    pub struct RequestStakeDecreaseFilter {
        #[ethevent(indexed)]
        pub generator_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "StakeLockImposed",
        abi = "StakeLockImposed(address,address,uint256)"
    )]
    pub struct StakeLockImposedFilter {
        #[ethevent(indexed)]
        pub generator_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub stake: ::ethers::core::types::U256,
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
        name = "StakeLockReleased",
        abi = "StakeLockReleased(address,address,uint256)"
    )]
    pub struct StakeLockReleasedFilter {
        #[ethevent(indexed)]
        pub generator_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub stake: ::ethers::core::types::U256,
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
    #[ethevent(name = "StakeSlashed", abi = "StakeSlashed(address,address,uint256)")]
    pub struct StakeSlashedFilter {
        #[ethevent(indexed)]
        pub generator_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub stake: ::ethers::core::types::U256,
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
        name = "SymbioticCompleteSnapshot",
        abi = "SymbioticCompleteSnapshot(uint256)"
    )]
    pub struct SymbioticCompleteSnapshotFilter {
        #[ethevent(indexed)]
        pub capture_timestamp: ::ethers::core::types::U256,
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
    pub enum IGeneratorCallbacksEvents {
        AddedStakeFilter(AddedStakeFilter),
        RemovedStakeFilter(RemovedStakeFilter),
        RequestStakeDecreaseFilter(RequestStakeDecreaseFilter),
        StakeLockImposedFilter(StakeLockImposedFilter),
        StakeLockReleasedFilter(StakeLockReleasedFilter),
        StakeSlashedFilter(StakeSlashedFilter),
        SymbioticCompleteSnapshotFilter(SymbioticCompleteSnapshotFilter),
    }
    impl ::ethers::contract::EthLogDecode for IGeneratorCallbacksEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedStakeFilter::decode_log(log) {
                return Ok(IGeneratorCallbacksEvents::AddedStakeFilter(decoded));
            }
            if let Ok(decoded) = RemovedStakeFilter::decode_log(log) {
                return Ok(IGeneratorCallbacksEvents::RemovedStakeFilter(decoded));
            }
            if let Ok(decoded) = RequestStakeDecreaseFilter::decode_log(log) {
                return Ok(IGeneratorCallbacksEvents::RequestStakeDecreaseFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StakeLockImposedFilter::decode_log(log) {
                return Ok(IGeneratorCallbacksEvents::StakeLockImposedFilter(decoded));
            }
            if let Ok(decoded) = StakeLockReleasedFilter::decode_log(log) {
                return Ok(IGeneratorCallbacksEvents::StakeLockReleasedFilter(decoded));
            }
            if let Ok(decoded) = StakeSlashedFilter::decode_log(log) {
                return Ok(IGeneratorCallbacksEvents::StakeSlashedFilter(decoded));
            }
            if let Ok(decoded) = SymbioticCompleteSnapshotFilter::decode_log(log) {
                return Ok(IGeneratorCallbacksEvents::SymbioticCompleteSnapshotFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IGeneratorCallbacksEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddedStakeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovedStakeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestStakeDecreaseFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockImposedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockReleasedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeSlashedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbioticCompleteSnapshotFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddedStakeFilter> for IGeneratorCallbacksEvents {
        fn from(value: AddedStakeFilter) -> Self {
            Self::AddedStakeFilter(value)
        }
    }
    impl ::core::convert::From<RemovedStakeFilter> for IGeneratorCallbacksEvents {
        fn from(value: RemovedStakeFilter) -> Self {
            Self::RemovedStakeFilter(value)
        }
    }
    impl ::core::convert::From<RequestStakeDecreaseFilter> for IGeneratorCallbacksEvents {
        fn from(value: RequestStakeDecreaseFilter) -> Self {
            Self::RequestStakeDecreaseFilter(value)
        }
    }
    impl ::core::convert::From<StakeLockImposedFilter> for IGeneratorCallbacksEvents {
        fn from(value: StakeLockImposedFilter) -> Self {
            Self::StakeLockImposedFilter(value)
        }
    }
    impl ::core::convert::From<StakeLockReleasedFilter> for IGeneratorCallbacksEvents {
        fn from(value: StakeLockReleasedFilter) -> Self {
            Self::StakeLockReleasedFilter(value)
        }
    }
    impl ::core::convert::From<StakeSlashedFilter> for IGeneratorCallbacksEvents {
        fn from(value: StakeSlashedFilter) -> Self {
            Self::StakeSlashedFilter(value)
        }
    }
    impl ::core::convert::From<SymbioticCompleteSnapshotFilter> for IGeneratorCallbacksEvents {
        fn from(value: SymbioticCompleteSnapshotFilter) -> Self {
            Self::SymbioticCompleteSnapshotFilter(value)
        }
    }
    ///Container type for all input parameters for the `addStakeCallback` function with signature `addStakeCallback(address,address,uint256)` and selector `0xa63616e6`
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
        name = "addStakeCallback",
        abi = "addStakeCallback(address,address,uint256)"
    )]
    pub struct AddStakeCallbackCall {
        pub generator_address: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `intendToReduceStakeCallback` function with signature `intendToReduceStakeCallback(address,address,uint256)` and selector `0x078159cd`
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
        name = "intendToReduceStakeCallback",
        abi = "intendToReduceStakeCallback(address,address,uint256)"
    )]
    pub struct IntendToReduceStakeCallbackCall {
        pub generator_address: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeStakeCallback` function with signature `removeStakeCallback(address,address,uint256)` and selector `0xcb80d418`
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
        name = "removeStakeCallback",
        abi = "removeStakeCallback(address,address,uint256)"
    )]
    pub struct RemoveStakeCallbackCall {
        pub generator_address: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stakeLockImposedCallback` function with signature `stakeLockImposedCallback(address,address,uint256)` and selector `0x201a11b3`
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
        name = "stakeLockImposedCallback",
        abi = "stakeLockImposedCallback(address,address,uint256)"
    )]
    pub struct StakeLockImposedCallbackCall {
        pub generator_address: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stakeLockReleasedCallback` function with signature `stakeLockReleasedCallback(address,address,uint256)` and selector `0x31b526ba`
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
        name = "stakeLockReleasedCallback",
        abi = "stakeLockReleasedCallback(address,address,uint256)"
    )]
    pub struct StakeLockReleasedCallbackCall {
        pub generator_address: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stakeSlashedCallback` function with signature `stakeSlashedCallback(address,address,uint256)` and selector `0x61413f0f`
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
        name = "stakeSlashedCallback",
        abi = "stakeSlashedCallback(address,address,uint256)"
    )]
    pub struct StakeSlashedCallbackCall {
        pub generator_address: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `symbioticCompleteSnapshotCallback` function with signature `symbioticCompleteSnapshotCallback(uint256)` and selector `0x056a7668`
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
        name = "symbioticCompleteSnapshotCallback",
        abi = "symbioticCompleteSnapshotCallback(uint256)"
    )]
    pub struct SymbioticCompleteSnapshotCallbackCall {
        pub capture_timestamp: ::ethers::core::types::U256,
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
    pub enum IGeneratorCallbacksCalls {
        AddStakeCallback(AddStakeCallbackCall),
        IntendToReduceStakeCallback(IntendToReduceStakeCallbackCall),
        RemoveStakeCallback(RemoveStakeCallbackCall),
        StakeLockImposedCallback(StakeLockImposedCallbackCall),
        StakeLockReleasedCallback(StakeLockReleasedCallbackCall),
        StakeSlashedCallback(StakeSlashedCallbackCall),
        SymbioticCompleteSnapshotCallback(SymbioticCompleteSnapshotCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for IGeneratorCallbacksCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AddStakeCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddStakeCallback(decoded));
            }
            if let Ok(decoded) =
                <IntendToReduceStakeCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IntendToReduceStakeCallback(decoded));
            }
            if let Ok(decoded) =
                <RemoveStakeCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveStakeCallback(decoded));
            }
            if let Ok(decoded) =
                <StakeLockImposedCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeLockImposedCallback(decoded));
            }
            if let Ok(decoded) =
                <StakeLockReleasedCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeLockReleasedCallback(decoded));
            }
            if let Ok(decoded) =
                <StakeSlashedCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeSlashedCallback(decoded));
            }
            if let Ok(decoded) =
                <SymbioticCompleteSnapshotCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SymbioticCompleteSnapshotCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IGeneratorCallbacksCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddStakeCallback(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IntendToReduceStakeCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveStakeCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeLockImposedCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeLockReleasedCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeSlashedCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SymbioticCompleteSnapshotCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IGeneratorCallbacksCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddStakeCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntendToReduceStakeCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveStakeCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockImposedCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockReleasedCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeSlashedCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbioticCompleteSnapshotCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddStakeCallbackCall> for IGeneratorCallbacksCalls {
        fn from(value: AddStakeCallbackCall) -> Self {
            Self::AddStakeCallback(value)
        }
    }
    impl ::core::convert::From<IntendToReduceStakeCallbackCall> for IGeneratorCallbacksCalls {
        fn from(value: IntendToReduceStakeCallbackCall) -> Self {
            Self::IntendToReduceStakeCallback(value)
        }
    }
    impl ::core::convert::From<RemoveStakeCallbackCall> for IGeneratorCallbacksCalls {
        fn from(value: RemoveStakeCallbackCall) -> Self {
            Self::RemoveStakeCallback(value)
        }
    }
    impl ::core::convert::From<StakeLockImposedCallbackCall> for IGeneratorCallbacksCalls {
        fn from(value: StakeLockImposedCallbackCall) -> Self {
            Self::StakeLockImposedCallback(value)
        }
    }
    impl ::core::convert::From<StakeLockReleasedCallbackCall> for IGeneratorCallbacksCalls {
        fn from(value: StakeLockReleasedCallbackCall) -> Self {
            Self::StakeLockReleasedCallback(value)
        }
    }
    impl ::core::convert::From<StakeSlashedCallbackCall> for IGeneratorCallbacksCalls {
        fn from(value: StakeSlashedCallbackCall) -> Self {
            Self::StakeSlashedCallback(value)
        }
    }
    impl ::core::convert::From<SymbioticCompleteSnapshotCallbackCall> for IGeneratorCallbacksCalls {
        fn from(value: SymbioticCompleteSnapshotCallbackCall) -> Self {
            Self::SymbioticCompleteSnapshotCallback(value)
        }
    }
}
