pub use i_native_staking::*;
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
pub mod i_native_staking {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorActiveStakeAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorActiveStakeAmount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorStakeAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorStakeAmount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakeAmount"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeTokenList"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakeTokenList"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeTokenWeights"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakeTokenWeights",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isSupportedStakeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isSupportedStakeToken",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("stakeToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lockStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lockStake"),
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
                                name: ::std::borrow::ToOwned::to_owned("feeRewardAmount"),
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
                    ::std::borrow::ToOwned::to_owned("requestStakeWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestStakeWithdrawal",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeToken"),
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
                    ::std::borrow::ToOwned::to_owned("rewardDistributor"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rewardDistributor"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("slash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("slash"),
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
                (
                    ::std::borrow::ToOwned::to_owned("stake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stake"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("stakeTokenSelectionWeightSum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeTokenSelectionWeightSum",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawStake"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AmountToLockSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AmountToLockSet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeRewardTokenSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FeeRewardTokenSet"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("JobSlashed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("JobSlashed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("jobId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("StakeLocked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeLocked"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("jobId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("StakeTokenAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeTokenAdded"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("weight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakeTokenRemoved"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeTokenRemoved"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakeTokenSelectionWeightSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeTokenSelectionWeightSet",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("weight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakeUnlocked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeUnlocked"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("jobId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("StakeWithdrawalRequested"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeWithdrawalRequested",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("StakeWithdrawn"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeWithdrawn"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("Staked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Staked"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("StakingManagerSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakingManagerSet"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("stakingManager"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalDurationSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawalDurationSet",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("duration"),
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
    pub static INATIVESTAKING_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct INativeStaking<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for INativeStaking<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for INativeStaking<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for INativeStaking<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for INativeStaking<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(INativeStaking))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> INativeStaking<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                INATIVESTAKING_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `getOperatorActiveStakeAmount` (0x89c7b987) function
        pub fn get_operator_active_stake_amount(
            &self,
            stake_token: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([137, 199, 185, 135], (stake_token, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorStakeAmount` (0xd365d2d6) function
        pub fn get_operator_stake_amount(
            &self,
            stake_token: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([211, 101, 210, 214], (stake_token, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeAmount` (0x9fcd115f) function
        pub fn get_stake_amount(
            &self,
            stake_token: ::ethers::core::types::Address,
            staker: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 205, 17, 95], (stake_token, staker, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeTokenList` (0xd11f293c) function
        pub fn get_stake_token_list(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([209, 31, 41, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeTokenWeights` (0x5f413d38) function
        pub fn get_stake_token_weights(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([95, 65, 61, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSupportedStakeToken` (0x4e821102) function
        pub fn is_supported_stake_token(
            &self,
            stake_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([78, 130, 17, 2], stake_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockStake` (0x33f4909f) function
        pub fn lock_stake(
            &self,
            job_id: ::ethers::core::types::U256,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 244, 144, 159], (job_id, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onJobCompletion` (0xc0ff97ef) function
        pub fn on_job_completion(
            &self,
            job_id: ::ethers::core::types::U256,
            operator: ::ethers::core::types::Address,
            fee_reward_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 255, 151, 239], (job_id, operator, fee_reward_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestStakeWithdrawal` (0xc985d969) function
        pub fn request_stake_withdrawal(
            &self,
            operator: ::ethers::core::types::Address,
            stake_token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 133, 217, 105], (operator, stake_token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardDistributor` (0xacc2166a) function
        pub fn reward_distributor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([172, 194, 22, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slash` (0xdee8bce2) function
        pub fn slash(
            &self,
            slashed_jobs: ::std::vec::Vec<JobSlashed>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 232, 188, 226], slashed_jobs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stake` (0xbf6eac2f) function
        pub fn stake(
            &self,
            stake_token: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 110, 172, 47], (stake_token, operator, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeTokenSelectionWeightSum` (0xa6ffe527) function
        pub fn stake_token_selection_weight_sum(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([166, 255, 229, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawStake` (0xd55d0923) function
        pub fn withdraw_stake(
            &self,
            operator: ::ethers::core::types::Address,
            index: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 93, 9, 35], (operator, index))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AmountToLockSet` event
        pub fn amount_to_lock_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AmountToLockSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `FeeRewardTokenSet` event
        pub fn fee_reward_token_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeRewardTokenSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `JobSlashed` event
        pub fn job_slashed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, JobSlashedFilter> {
            self.0.event()
        }
        ///Gets the contract's `StakeLocked` event
        pub fn stake_locked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeLockedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeTokenAdded` event
        pub fn stake_token_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeTokenAddedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeTokenRemoved` event
        pub fn stake_token_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeTokenRemovedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeTokenSelectionWeightSet` event
        pub fn stake_token_selection_weight_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakeTokenSelectionWeightSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakeUnlocked` event
        pub fn stake_unlocked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeUnlockedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeWithdrawalRequested` event
        pub fn stake_withdrawal_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakeWithdrawalRequestedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakeWithdrawn` event
        pub fn stake_withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeWithdrawnFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Staked` event
        pub fn staked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakedFilter> {
            self.0.event()
        }
        ///Gets the contract's `StakingManagerSet` event
        pub fn staking_manager_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakingManagerSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalDurationSet` event
        pub fn withdrawal_duration_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawalDurationSetFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, INativeStakingEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for INativeStaking<M>
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
    #[ethevent(name = "AmountToLockSet", abi = "AmountToLockSet(address,uint256)")]
    pub struct AmountToLockSetFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
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
    #[ethevent(name = "FeeRewardTokenSet", abi = "FeeRewardTokenSet(address)")]
    pub struct FeeRewardTokenSetFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
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
        name = "JobSlashed",
        abi = "JobSlashed(uint256,address,address,uint256)"
    )]
    pub struct JobSlashedFilter {
        #[ethevent(indexed)]
        pub job_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
        name = "StakeLocked",
        abi = "StakeLocked(uint256,address,address,uint256)"
    )]
    pub struct StakeLockedFilter {
        #[ethevent(indexed)]
        pub job_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "StakeTokenAdded", abi = "StakeTokenAdded(address,uint256)")]
    pub struct StakeTokenAddedFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub weight: ::ethers::core::types::U256,
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
    #[ethevent(name = "StakeTokenRemoved", abi = "StakeTokenRemoved(address)")]
    pub struct StakeTokenRemovedFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
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
        name = "StakeTokenSelectionWeightSet",
        abi = "StakeTokenSelectionWeightSet(address,uint256)"
    )]
    pub struct StakeTokenSelectionWeightSetFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub weight: ::ethers::core::types::U256,
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
        name = "StakeUnlocked",
        abi = "StakeUnlocked(uint256,address,address,uint256)"
    )]
    pub struct StakeUnlockedFilter {
        #[ethevent(indexed)]
        pub job_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
        name = "StakeWithdrawalRequested",
        abi = "StakeWithdrawalRequested(address,address,address,uint256,uint256)"
    )]
    pub struct StakeWithdrawalRequestedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
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
        name = "StakeWithdrawn",
        abi = "StakeWithdrawn(address,address,address,uint256,uint256)"
    )]
    pub struct StakeWithdrawnFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
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
    #[ethevent(name = "Staked", abi = "Staked(address,address,address,uint256)")]
    pub struct StakedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "StakingManagerSet", abi = "StakingManagerSet(address)")]
    pub struct StakingManagerSetFilter {
        #[ethevent(indexed)]
        pub staking_manager: ::ethers::core::types::Address,
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
    #[ethevent(name = "WithdrawalDurationSet", abi = "WithdrawalDurationSet(uint256)")]
    pub struct WithdrawalDurationSetFilter {
        #[ethevent(indexed)]
        pub duration: ::ethers::core::types::U256,
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
    pub enum INativeStakingEvents {
        AmountToLockSetFilter(AmountToLockSetFilter),
        FeeRewardTokenSetFilter(FeeRewardTokenSetFilter),
        JobSlashedFilter(JobSlashedFilter),
        StakeLockedFilter(StakeLockedFilter),
        StakeTokenAddedFilter(StakeTokenAddedFilter),
        StakeTokenRemovedFilter(StakeTokenRemovedFilter),
        StakeTokenSelectionWeightSetFilter(StakeTokenSelectionWeightSetFilter),
        StakeUnlockedFilter(StakeUnlockedFilter),
        StakeWithdrawalRequestedFilter(StakeWithdrawalRequestedFilter),
        StakeWithdrawnFilter(StakeWithdrawnFilter),
        StakedFilter(StakedFilter),
        StakingManagerSetFilter(StakingManagerSetFilter),
        WithdrawalDurationSetFilter(WithdrawalDurationSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for INativeStakingEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AmountToLockSetFilter::decode_log(log) {
                return Ok(INativeStakingEvents::AmountToLockSetFilter(decoded));
            }
            if let Ok(decoded) = FeeRewardTokenSetFilter::decode_log(log) {
                return Ok(INativeStakingEvents::FeeRewardTokenSetFilter(decoded));
            }
            if let Ok(decoded) = JobSlashedFilter::decode_log(log) {
                return Ok(INativeStakingEvents::JobSlashedFilter(decoded));
            }
            if let Ok(decoded) = StakeLockedFilter::decode_log(log) {
                return Ok(INativeStakingEvents::StakeLockedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenAddedFilter::decode_log(log) {
                return Ok(INativeStakingEvents::StakeTokenAddedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenRemovedFilter::decode_log(log) {
                return Ok(INativeStakingEvents::StakeTokenRemovedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenSelectionWeightSetFilter::decode_log(log) {
                return Ok(INativeStakingEvents::StakeTokenSelectionWeightSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StakeUnlockedFilter::decode_log(log) {
                return Ok(INativeStakingEvents::StakeUnlockedFilter(decoded));
            }
            if let Ok(decoded) = StakeWithdrawalRequestedFilter::decode_log(log) {
                return Ok(INativeStakingEvents::StakeWithdrawalRequestedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StakeWithdrawnFilter::decode_log(log) {
                return Ok(INativeStakingEvents::StakeWithdrawnFilter(decoded));
            }
            if let Ok(decoded) = StakedFilter::decode_log(log) {
                return Ok(INativeStakingEvents::StakedFilter(decoded));
            }
            if let Ok(decoded) = StakingManagerSetFilter::decode_log(log) {
                return Ok(INativeStakingEvents::StakingManagerSetFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalDurationSetFilter::decode_log(log) {
                return Ok(INativeStakingEvents::WithdrawalDurationSetFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for INativeStakingEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AmountToLockSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeRewardTokenSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::JobSlashedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenRemovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenSelectionWeightSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeUnlockedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeWithdrawalRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeWithdrawnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakingManagerSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalDurationSetFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AmountToLockSetFilter> for INativeStakingEvents {
        fn from(value: AmountToLockSetFilter) -> Self {
            Self::AmountToLockSetFilter(value)
        }
    }
    impl ::core::convert::From<FeeRewardTokenSetFilter> for INativeStakingEvents {
        fn from(value: FeeRewardTokenSetFilter) -> Self {
            Self::FeeRewardTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<JobSlashedFilter> for INativeStakingEvents {
        fn from(value: JobSlashedFilter) -> Self {
            Self::JobSlashedFilter(value)
        }
    }
    impl ::core::convert::From<StakeLockedFilter> for INativeStakingEvents {
        fn from(value: StakeLockedFilter) -> Self {
            Self::StakeLockedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenAddedFilter> for INativeStakingEvents {
        fn from(value: StakeTokenAddedFilter) -> Self {
            Self::StakeTokenAddedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenRemovedFilter> for INativeStakingEvents {
        fn from(value: StakeTokenRemovedFilter) -> Self {
            Self::StakeTokenRemovedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenSelectionWeightSetFilter> for INativeStakingEvents {
        fn from(value: StakeTokenSelectionWeightSetFilter) -> Self {
            Self::StakeTokenSelectionWeightSetFilter(value)
        }
    }
    impl ::core::convert::From<StakeUnlockedFilter> for INativeStakingEvents {
        fn from(value: StakeUnlockedFilter) -> Self {
            Self::StakeUnlockedFilter(value)
        }
    }
    impl ::core::convert::From<StakeWithdrawalRequestedFilter> for INativeStakingEvents {
        fn from(value: StakeWithdrawalRequestedFilter) -> Self {
            Self::StakeWithdrawalRequestedFilter(value)
        }
    }
    impl ::core::convert::From<StakeWithdrawnFilter> for INativeStakingEvents {
        fn from(value: StakeWithdrawnFilter) -> Self {
            Self::StakeWithdrawnFilter(value)
        }
    }
    impl ::core::convert::From<StakedFilter> for INativeStakingEvents {
        fn from(value: StakedFilter) -> Self {
            Self::StakedFilter(value)
        }
    }
    impl ::core::convert::From<StakingManagerSetFilter> for INativeStakingEvents {
        fn from(value: StakingManagerSetFilter) -> Self {
            Self::StakingManagerSetFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalDurationSetFilter> for INativeStakingEvents {
        fn from(value: WithdrawalDurationSetFilter) -> Self {
            Self::WithdrawalDurationSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `getOperatorActiveStakeAmount` function with signature `getOperatorActiveStakeAmount(address,address)` and selector `0x89c7b987`
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
        name = "getOperatorActiveStakeAmount",
        abi = "getOperatorActiveStakeAmount(address,address)"
    )]
    pub struct GetOperatorActiveStakeAmountCall {
        pub stake_token: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOperatorStakeAmount` function with signature `getOperatorStakeAmount(address,address)` and selector `0xd365d2d6`
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
        name = "getOperatorStakeAmount",
        abi = "getOperatorStakeAmount(address,address)"
    )]
    pub struct GetOperatorStakeAmountCall {
        pub stake_token: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getStakeAmount` function with signature `getStakeAmount(address,address,address)` and selector `0x9fcd115f`
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
        name = "getStakeAmount",
        abi = "getStakeAmount(address,address,address)"
    )]
    pub struct GetStakeAmountCall {
        pub stake_token: ::ethers::core::types::Address,
        pub staker: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getStakeTokenList` function with signature `getStakeTokenList()` and selector `0xd11f293c`
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
    #[ethcall(name = "getStakeTokenList", abi = "getStakeTokenList()")]
    pub struct GetStakeTokenListCall;
    ///Container type for all input parameters for the `getStakeTokenWeights` function with signature `getStakeTokenWeights()` and selector `0x5f413d38`
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
    #[ethcall(name = "getStakeTokenWeights", abi = "getStakeTokenWeights()")]
    pub struct GetStakeTokenWeightsCall;
    ///Container type for all input parameters for the `isSupportedStakeToken` function with signature `isSupportedStakeToken(address)` and selector `0x4e821102`
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
    #[ethcall(name = "isSupportedStakeToken", abi = "isSupportedStakeToken(address)")]
    pub struct IsSupportedStakeTokenCall {
        pub stake_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lockStake` function with signature `lockStake(uint256,address)` and selector `0x33f4909f`
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
    #[ethcall(name = "lockStake", abi = "lockStake(uint256,address)")]
    pub struct LockStakeCall {
        pub job_id: ::ethers::core::types::U256,
        pub operator: ::ethers::core::types::Address,
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
        pub fee_reward_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requestStakeWithdrawal` function with signature `requestStakeWithdrawal(address,address,uint256)` and selector `0xc985d969`
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
        name = "requestStakeWithdrawal",
        abi = "requestStakeWithdrawal(address,address,uint256)"
    )]
    pub struct RequestStakeWithdrawalCall {
        pub operator: ::ethers::core::types::Address,
        pub stake_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rewardDistributor` function with signature `rewardDistributor()` and selector `0xacc2166a`
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
    #[ethcall(name = "rewardDistributor", abi = "rewardDistributor()")]
    pub struct RewardDistributorCall;
    ///Container type for all input parameters for the `slash` function with signature `slash((uint256,address,address)[])` and selector `0xdee8bce2`
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
    #[ethcall(name = "slash", abi = "slash((uint256,address,address)[])")]
    pub struct SlashCall {
        pub slashed_jobs: ::std::vec::Vec<JobSlashed>,
    }
    ///Container type for all input parameters for the `stake` function with signature `stake(address,address,uint256)` and selector `0xbf6eac2f`
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
    #[ethcall(name = "stake", abi = "stake(address,address,uint256)")]
    pub struct StakeCall {
        pub stake_token: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stakeTokenSelectionWeightSum` function with signature `stakeTokenSelectionWeightSum()` and selector `0xa6ffe527`
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
        name = "stakeTokenSelectionWeightSum",
        abi = "stakeTokenSelectionWeightSum()"
    )]
    pub struct StakeTokenSelectionWeightSumCall;
    ///Container type for all input parameters for the `withdrawStake` function with signature `withdrawStake(address,uint256[])` and selector `0xd55d0923`
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
    #[ethcall(name = "withdrawStake", abi = "withdrawStake(address,uint256[])")]
    pub struct WithdrawStakeCall {
        pub operator: ::ethers::core::types::Address,
        pub index: ::std::vec::Vec<::ethers::core::types::U256>,
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
    pub enum INativeStakingCalls {
        GetOperatorActiveStakeAmount(GetOperatorActiveStakeAmountCall),
        GetOperatorStakeAmount(GetOperatorStakeAmountCall),
        GetStakeAmount(GetStakeAmountCall),
        GetStakeTokenList(GetStakeTokenListCall),
        GetStakeTokenWeights(GetStakeTokenWeightsCall),
        IsSupportedStakeToken(IsSupportedStakeTokenCall),
        LockStake(LockStakeCall),
        OnJobCompletion(OnJobCompletionCall),
        RequestStakeWithdrawal(RequestStakeWithdrawalCall),
        RewardDistributor(RewardDistributorCall),
        Slash(SlashCall),
        Stake(StakeCall),
        StakeTokenSelectionWeightSum(StakeTokenSelectionWeightSumCall),
        WithdrawStake(WithdrawStakeCall),
    }
    impl ::ethers::core::abi::AbiDecode for INativeStakingCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <GetOperatorActiveStakeAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorActiveStakeAmount(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorStakeAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorStakeAmount(decoded));
            }
            if let Ok(decoded) =
                <GetStakeAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStakeAmount(decoded));
            }
            if let Ok(decoded) =
                <GetStakeTokenListCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStakeTokenList(decoded));
            }
            if let Ok(decoded) =
                <GetStakeTokenWeightsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStakeTokenWeights(decoded));
            }
            if let Ok(decoded) =
                <IsSupportedStakeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsSupportedStakeToken(decoded));
            }
            if let Ok(decoded) = <LockStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockStake(decoded));
            }
            if let Ok(decoded) =
                <OnJobCompletionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnJobCompletion(decoded));
            }
            if let Ok(decoded) =
                <RequestStakeWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestStakeWithdrawal(decoded));
            }
            if let Ok(decoded) =
                <RewardDistributorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RewardDistributor(decoded));
            }
            if let Ok(decoded) = <SlashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Slash(decoded));
            }
            if let Ok(decoded) = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded) =
                <StakeTokenSelectionWeightSumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeTokenSelectionWeightSum(decoded));
            }
            if let Ok(decoded) = <WithdrawStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawStake(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for INativeStakingCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetOperatorActiveStakeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorStakeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStakeTokenList(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStakeTokenWeights(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSupportedStakeToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LockStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnJobCompletion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestStakeWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardDistributor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Slash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeTokenSelectionWeightSum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for INativeStakingCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetOperatorActiveStakeAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorStakeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeTokenList(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeTokenWeights(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSupportedStakeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnJobCompletion(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestStakeWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardDistributor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenSelectionWeightSum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawStake(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetOperatorActiveStakeAmountCall> for INativeStakingCalls {
        fn from(value: GetOperatorActiveStakeAmountCall) -> Self {
            Self::GetOperatorActiveStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetOperatorStakeAmountCall> for INativeStakingCalls {
        fn from(value: GetOperatorStakeAmountCall) -> Self {
            Self::GetOperatorStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetStakeAmountCall> for INativeStakingCalls {
        fn from(value: GetStakeAmountCall) -> Self {
            Self::GetStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetStakeTokenListCall> for INativeStakingCalls {
        fn from(value: GetStakeTokenListCall) -> Self {
            Self::GetStakeTokenList(value)
        }
    }
    impl ::core::convert::From<GetStakeTokenWeightsCall> for INativeStakingCalls {
        fn from(value: GetStakeTokenWeightsCall) -> Self {
            Self::GetStakeTokenWeights(value)
        }
    }
    impl ::core::convert::From<IsSupportedStakeTokenCall> for INativeStakingCalls {
        fn from(value: IsSupportedStakeTokenCall) -> Self {
            Self::IsSupportedStakeToken(value)
        }
    }
    impl ::core::convert::From<LockStakeCall> for INativeStakingCalls {
        fn from(value: LockStakeCall) -> Self {
            Self::LockStake(value)
        }
    }
    impl ::core::convert::From<OnJobCompletionCall> for INativeStakingCalls {
        fn from(value: OnJobCompletionCall) -> Self {
            Self::OnJobCompletion(value)
        }
    }
    impl ::core::convert::From<RequestStakeWithdrawalCall> for INativeStakingCalls {
        fn from(value: RequestStakeWithdrawalCall) -> Self {
            Self::RequestStakeWithdrawal(value)
        }
    }
    impl ::core::convert::From<RewardDistributorCall> for INativeStakingCalls {
        fn from(value: RewardDistributorCall) -> Self {
            Self::RewardDistributor(value)
        }
    }
    impl ::core::convert::From<SlashCall> for INativeStakingCalls {
        fn from(value: SlashCall) -> Self {
            Self::Slash(value)
        }
    }
    impl ::core::convert::From<StakeCall> for INativeStakingCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<StakeTokenSelectionWeightSumCall> for INativeStakingCalls {
        fn from(value: StakeTokenSelectionWeightSumCall) -> Self {
            Self::StakeTokenSelectionWeightSum(value)
        }
    }
    impl ::core::convert::From<WithdrawStakeCall> for INativeStakingCalls {
        fn from(value: WithdrawStakeCall) -> Self {
            Self::WithdrawStake(value)
        }
    }
    ///Container type for all return fields from the `getOperatorActiveStakeAmount` function with signature `getOperatorActiveStakeAmount(address,address)` and selector `0x89c7b987`
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
    pub struct GetOperatorActiveStakeAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getOperatorStakeAmount` function with signature `getOperatorStakeAmount(address,address)` and selector `0xd365d2d6`
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
    pub struct GetOperatorStakeAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStakeAmount` function with signature `getStakeAmount(address,address,address)` and selector `0x9fcd115f`
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
    pub struct GetStakeAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStakeTokenList` function with signature `getStakeTokenList()` and selector `0xd11f293c`
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
    pub struct GetStakeTokenListReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getStakeTokenWeights` function with signature `getStakeTokenWeights()` and selector `0x5f413d38`
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
    pub struct GetStakeTokenWeightsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `isSupportedStakeToken` function with signature `isSupportedStakeToken(address)` and selector `0x4e821102`
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
    pub struct IsSupportedStakeTokenReturn(pub bool);
    ///Container type for all return fields from the `rewardDistributor` function with signature `rewardDistributor()` and selector `0xacc2166a`
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
    pub struct RewardDistributorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stakeTokenSelectionWeightSum` function with signature `stakeTokenSelectionWeightSum()` and selector `0xa6ffe527`
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
    pub struct StakeTokenSelectionWeightSumReturn(pub ::ethers::core::types::U256);
}
