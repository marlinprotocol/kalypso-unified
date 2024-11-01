pub use i_symbiotic_staking::*;
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
pub mod i_symbiotic_staking {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("confirmedTimestampInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("confirmedTimestampInfo",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_idx"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct Struct.ConfirmedTimestamp",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
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
                    ::std::borrow::ToOwned::to_owned("getSubmissionStatus"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSubmissionStatus",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_captureTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("latestConfirmedTimestamp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestConfirmedTimestamp",),
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
                    ::std::borrow::ToOwned::to_owned("latestConfirmedTimestampIdx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestConfirmedTimestampIdx",),
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
                    ::std::borrow::ToOwned::to_owned("latestConfirmedTimestampInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestConfirmedTimestampInfo",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct Struct.ConfirmedTimestamp",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lockInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lockInfo"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_jobId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("registeredTransmitters"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registeredTransmitters",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_captureTimestamp"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("submitSlashResult"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitSlashResult"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numOfTxs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_captureTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_slashResultData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_proof"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitVaultSnapshot"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitVaultSnapshot",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_numOfTxs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_captureTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_vaultSnapshotData",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_proof"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("txCountInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("txCountInfo"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_captureTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_type"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idxToSubmit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("numOfTxs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AttestationVerifierUpdated",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("attestationVerifier",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BaseTransmitterComissionRateSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("BaseTransmitterComissionRateSet",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("rate"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveImageAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveImageAdded"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("PCR0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("PCR1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("PCR2"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveImageRemoved"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveImageRemoved",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("imageId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            indexed: true,
                        },],
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
                    ::std::borrow::ToOwned::to_owned("ProofMarketplaceSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ProofMarketplaceSet",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("proofMarketplace"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardDistributorSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RewardDistributorSet",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("rewardDistributor"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SlashResultSubmitted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SlashResultSubmitted",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("numOfTxs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("slashResultData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("proof"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SnapshotConfirmed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SnapshotConfirmed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("confirmedTimestamp",),
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
                    ::std::borrow::ToOwned::to_owned("SubmissionCooldownSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SubmissionCooldownSet",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("cooldown"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VaultSnapshotSubmitted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("VaultSnapshotSubmitted",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("numOfTxs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("vaultSnapshotData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("proof"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
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
    pub static ISYMBIOTICSTAKING_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct ISymbioticStaking<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISymbioticStaking<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISymbioticStaking<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISymbioticStaking<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISymbioticStaking<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ISymbioticStaking))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISymbioticStaking<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ISYMBIOTICSTAKING_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `confirmedTimestampInfo` (0x10c999bb) function
        pub fn confirmed_timestamp_info(
            &self,
            idx: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ConfirmedTimestamp> {
            self.0
                .method_hash([16, 201, 153, 187], idx)
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `getSubmissionStatus` (0xeb5f2b98) function
        pub fn get_submission_status(
            &self,
            capture_timestamp: ::ethers::core::types::U256,
            transmitter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([235, 95, 43, 152], (capture_timestamp, transmitter))
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
        ///Calls the contract's `latestConfirmedTimestamp` (0x33db2467) function
        pub fn latest_confirmed_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([51, 219, 36, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestConfirmedTimestampIdx` (0xec9a5359) function
        pub fn latest_confirmed_timestamp_idx(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([236, 154, 83, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestConfirmedTimestampInfo` (0x0e988e25) function
        pub fn latest_confirmed_timestamp_info(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ConfirmedTimestamp> {
            self.0
                .method_hash([14, 152, 142, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockInfo` (0x5b1a4c24) function
        pub fn lock_info(
            &self,
            job_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([91, 26, 76, 36], job_id)
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
        ///Calls the contract's `registeredTransmitters` (0xd6748b02) function
        pub fn registered_transmitters(
            &self,
            capture_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([214, 116, 139, 2], capture_timestamp)
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
        ///Calls the contract's `stakeTokenSelectionWeightSum` (0xa6ffe527) function
        pub fn stake_token_selection_weight_sum(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([166, 255, 229, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitSlashResult` (0x381106d3) function
        pub fn submit_slash_result(
            &self,
            index: ::ethers::core::types::U256,
            num_of_txs: ::ethers::core::types::U256,
            capture_timestamp: ::ethers::core::types::U256,
            image_id: [u8; 32],
            slash_result_data: ::ethers::core::types::Bytes,
            proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [56, 17, 6, 211],
                    (
                        index,
                        num_of_txs,
                        capture_timestamp,
                        image_id,
                        slash_result_data,
                        proof,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitVaultSnapshot` (0x08b24318) function
        pub fn submit_vault_snapshot(
            &self,
            index: ::ethers::core::types::U256,
            num_of_txs: ::ethers::core::types::U256,
            capture_timestamp: ::ethers::core::types::U256,
            image_id: [u8; 32],
            vault_snapshot_data: ::ethers::core::types::Bytes,
            proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [8, 178, 67, 24],
                    (
                        index,
                        num_of_txs,
                        capture_timestamp,
                        image_id,
                        vault_snapshot_data,
                        proof,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `txCountInfo` (0xd473e122) function
        pub fn tx_count_info(
            &self,
            capture_timestamp: ::ethers::core::types::U256,
            type_: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([212, 115, 225, 34], (capture_timestamp, type_))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AmountToLockSet` event
        pub fn amount_to_lock_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AmountToLockSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `AttestationVerifierUpdated` event
        pub fn attestation_verifier_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AttestationVerifierUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BaseTransmitterComissionRateSet` event
        pub fn base_transmitter_comission_rate_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BaseTransmitterComissionRateSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveImageAdded` event
        pub fn enclave_image_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EnclaveImageAddedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EnclaveImageRemoved` event
        pub fn enclave_image_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EnclaveImageRemovedFilter>
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
        ///Gets the contract's `ProofMarketplaceSet` event
        pub fn proof_marketplace_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProofMarketplaceSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RewardDistributorSet` event
        pub fn reward_distributor_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RewardDistributorSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SlashResultSubmitted` event
        pub fn slash_result_submitted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SlashResultSubmittedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SnapshotConfirmed` event
        pub fn snapshot_confirmed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SnapshotConfirmedFilter>
        {
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
        ///Gets the contract's `StakingManagerSet` event
        pub fn staking_manager_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakingManagerSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SubmissionCooldownSet` event
        pub fn submission_cooldown_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SubmissionCooldownSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `VaultSnapshotSubmitted` event
        pub fn vault_snapshot_submitted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, VaultSnapshotSubmittedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ISymbioticStakingEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ISymbioticStaking<M>
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
    #[ethevent(
        name = "AttestationVerifierUpdated",
        abi = "AttestationVerifierUpdated(address)"
    )]
    pub struct AttestationVerifierUpdatedFilter {
        pub attestation_verifier: ::ethers::core::types::Address,
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
        name = "BaseTransmitterComissionRateSet",
        abi = "BaseTransmitterComissionRateSet(uint256)"
    )]
    pub struct BaseTransmitterComissionRateSetFilter {
        pub rate: ::ethers::core::types::U256,
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
        name = "EnclaveImageAdded",
        abi = "EnclaveImageAdded(bytes32,bytes,bytes,bytes)"
    )]
    pub struct EnclaveImageAddedFilter {
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "EnclaveImageRemoved", abi = "EnclaveImageRemoved(bytes32)")]
    pub struct EnclaveImageRemovedFilter {
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
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
    #[ethevent(name = "ProofMarketplaceSet", abi = "ProofMarketplaceSet(address)")]
    pub struct ProofMarketplaceSetFilter {
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
    #[ethevent(name = "RewardDistributorSet", abi = "RewardDistributorSet(address)")]
    pub struct RewardDistributorSetFilter {
        pub reward_distributor: ::ethers::core::types::Address,
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
        name = "SlashResultSubmitted",
        abi = "SlashResultSubmitted(address,uint256,uint256,bytes32,bytes,bytes)"
    )]
    pub struct SlashResultSubmittedFilter {
        #[ethevent(indexed)]
        pub transmitter: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub index: ::ethers::core::types::U256,
        pub num_of_txs: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
        pub slash_result_data: ::ethers::core::types::Bytes,
        pub proof: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "SnapshotConfirmed", abi = "SnapshotConfirmed(address,uint256)")]
    pub struct SnapshotConfirmedFilter {
        #[ethevent(indexed)]
        pub transmitter: ::ethers::core::types::Address,
        pub confirmed_timestamp: ::ethers::core::types::U256,
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
    #[ethevent(name = "SubmissionCooldownSet", abi = "SubmissionCooldownSet(uint256)")]
    pub struct SubmissionCooldownSetFilter {
        pub cooldown: ::ethers::core::types::U256,
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
        name = "VaultSnapshotSubmitted",
        abi = "VaultSnapshotSubmitted(address,uint256,uint256,bytes32,bytes,bytes)"
    )]
    pub struct VaultSnapshotSubmittedFilter {
        #[ethevent(indexed)]
        pub transmitter: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub index: ::ethers::core::types::U256,
        pub num_of_txs: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
        pub vault_snapshot_data: ::ethers::core::types::Bytes,
        pub proof: ::ethers::core::types::Bytes,
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
    pub enum ISymbioticStakingEvents {
        AmountToLockSetFilter(AmountToLockSetFilter),
        AttestationVerifierUpdatedFilter(AttestationVerifierUpdatedFilter),
        BaseTransmitterComissionRateSetFilter(BaseTransmitterComissionRateSetFilter),
        EnclaveImageAddedFilter(EnclaveImageAddedFilter),
        EnclaveImageRemovedFilter(EnclaveImageRemovedFilter),
        FeeRewardTokenSetFilter(FeeRewardTokenSetFilter),
        JobSlashedFilter(JobSlashedFilter),
        ProofMarketplaceSetFilter(ProofMarketplaceSetFilter),
        RewardDistributorSetFilter(RewardDistributorSetFilter),
        SlashResultSubmittedFilter(SlashResultSubmittedFilter),
        SnapshotConfirmedFilter(SnapshotConfirmedFilter),
        StakeLockedFilter(StakeLockedFilter),
        StakeTokenAddedFilter(StakeTokenAddedFilter),
        StakeTokenRemovedFilter(StakeTokenRemovedFilter),
        StakeTokenSelectionWeightSetFilter(StakeTokenSelectionWeightSetFilter),
        StakeUnlockedFilter(StakeUnlockedFilter),
        StakingManagerSetFilter(StakingManagerSetFilter),
        SubmissionCooldownSetFilter(SubmissionCooldownSetFilter),
        VaultSnapshotSubmittedFilter(VaultSnapshotSubmittedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ISymbioticStakingEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AmountToLockSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::AmountToLockSetFilter(decoded));
            }
            if let Ok(decoded) = AttestationVerifierUpdatedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::AttestationVerifierUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = BaseTransmitterComissionRateSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::BaseTransmitterComissionRateSetFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageAddedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::EnclaveImageAddedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageRemovedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::EnclaveImageRemovedFilter(decoded));
            }
            if let Ok(decoded) = FeeRewardTokenSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::FeeRewardTokenSetFilter(decoded));
            }
            if let Ok(decoded) = JobSlashedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::JobSlashedFilter(decoded));
            }
            if let Ok(decoded) = ProofMarketplaceSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::ProofMarketplaceSetFilter(decoded));
            }
            if let Ok(decoded) = RewardDistributorSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::RewardDistributorSetFilter(decoded));
            }
            if let Ok(decoded) = SlashResultSubmittedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::SlashResultSubmittedFilter(decoded));
            }
            if let Ok(decoded) = SnapshotConfirmedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::SnapshotConfirmedFilter(decoded));
            }
            if let Ok(decoded) = StakeLockedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::StakeLockedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenAddedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::StakeTokenAddedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenRemovedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::StakeTokenRemovedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenSelectionWeightSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::StakeTokenSelectionWeightSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StakeUnlockedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::StakeUnlockedFilter(decoded));
            }
            if let Ok(decoded) = StakingManagerSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::StakingManagerSetFilter(decoded));
            }
            if let Ok(decoded) = SubmissionCooldownSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::SubmissionCooldownSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = VaultSnapshotSubmittedFilter::decode_log(log) {
                return Ok(ISymbioticStakingEvents::VaultSnapshotSubmittedFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ISymbioticStakingEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AmountToLockSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationVerifierUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BaseTransmitterComissionRateSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveImageAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveImageRemovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeRewardTokenSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::JobSlashedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketplaceSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardDistributorSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SlashResultSubmittedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SnapshotConfirmedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenRemovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenSelectionWeightSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeUnlockedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakingManagerSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmissionCooldownSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VaultSnapshotSubmittedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AmountToLockSetFilter> for ISymbioticStakingEvents {
        fn from(value: AmountToLockSetFilter) -> Self {
            Self::AmountToLockSetFilter(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierUpdatedFilter> for ISymbioticStakingEvents {
        fn from(value: AttestationVerifierUpdatedFilter) -> Self {
            Self::AttestationVerifierUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<BaseTransmitterComissionRateSetFilter> for ISymbioticStakingEvents {
        fn from(value: BaseTransmitterComissionRateSetFilter) -> Self {
            Self::BaseTransmitterComissionRateSetFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageAddedFilter> for ISymbioticStakingEvents {
        fn from(value: EnclaveImageAddedFilter) -> Self {
            Self::EnclaveImageAddedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRemovedFilter> for ISymbioticStakingEvents {
        fn from(value: EnclaveImageRemovedFilter) -> Self {
            Self::EnclaveImageRemovedFilter(value)
        }
    }
    impl ::core::convert::From<FeeRewardTokenSetFilter> for ISymbioticStakingEvents {
        fn from(value: FeeRewardTokenSetFilter) -> Self {
            Self::FeeRewardTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<JobSlashedFilter> for ISymbioticStakingEvents {
        fn from(value: JobSlashedFilter) -> Self {
            Self::JobSlashedFilter(value)
        }
    }
    impl ::core::convert::From<ProofMarketplaceSetFilter> for ISymbioticStakingEvents {
        fn from(value: ProofMarketplaceSetFilter) -> Self {
            Self::ProofMarketplaceSetFilter(value)
        }
    }
    impl ::core::convert::From<RewardDistributorSetFilter> for ISymbioticStakingEvents {
        fn from(value: RewardDistributorSetFilter) -> Self {
            Self::RewardDistributorSetFilter(value)
        }
    }
    impl ::core::convert::From<SlashResultSubmittedFilter> for ISymbioticStakingEvents {
        fn from(value: SlashResultSubmittedFilter) -> Self {
            Self::SlashResultSubmittedFilter(value)
        }
    }
    impl ::core::convert::From<SnapshotConfirmedFilter> for ISymbioticStakingEvents {
        fn from(value: SnapshotConfirmedFilter) -> Self {
            Self::SnapshotConfirmedFilter(value)
        }
    }
    impl ::core::convert::From<StakeLockedFilter> for ISymbioticStakingEvents {
        fn from(value: StakeLockedFilter) -> Self {
            Self::StakeLockedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenAddedFilter> for ISymbioticStakingEvents {
        fn from(value: StakeTokenAddedFilter) -> Self {
            Self::StakeTokenAddedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenRemovedFilter> for ISymbioticStakingEvents {
        fn from(value: StakeTokenRemovedFilter) -> Self {
            Self::StakeTokenRemovedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenSelectionWeightSetFilter> for ISymbioticStakingEvents {
        fn from(value: StakeTokenSelectionWeightSetFilter) -> Self {
            Self::StakeTokenSelectionWeightSetFilter(value)
        }
    }
    impl ::core::convert::From<StakeUnlockedFilter> for ISymbioticStakingEvents {
        fn from(value: StakeUnlockedFilter) -> Self {
            Self::StakeUnlockedFilter(value)
        }
    }
    impl ::core::convert::From<StakingManagerSetFilter> for ISymbioticStakingEvents {
        fn from(value: StakingManagerSetFilter) -> Self {
            Self::StakingManagerSetFilter(value)
        }
    }
    impl ::core::convert::From<SubmissionCooldownSetFilter> for ISymbioticStakingEvents {
        fn from(value: SubmissionCooldownSetFilter) -> Self {
            Self::SubmissionCooldownSetFilter(value)
        }
    }
    impl ::core::convert::From<VaultSnapshotSubmittedFilter> for ISymbioticStakingEvents {
        fn from(value: VaultSnapshotSubmittedFilter) -> Self {
            Self::VaultSnapshotSubmittedFilter(value)
        }
    }
    ///Container type for all input parameters for the `confirmedTimestampInfo` function with signature `confirmedTimestampInfo(uint256)` and selector `0x10c999bb`
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
        name = "confirmedTimestampInfo",
        abi = "confirmedTimestampInfo(uint256)"
    )]
    pub struct ConfirmedTimestampInfoCall {
        pub idx: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `getSubmissionStatus` function with signature `getSubmissionStatus(uint256,address)` and selector `0xeb5f2b98`
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
        name = "getSubmissionStatus",
        abi = "getSubmissionStatus(uint256,address)"
    )]
    pub struct GetSubmissionStatusCall {
        pub capture_timestamp: ::ethers::core::types::U256,
        pub transmitter: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `latestConfirmedTimestamp` function with signature `latestConfirmedTimestamp()` and selector `0x33db2467`
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
    #[ethcall(name = "latestConfirmedTimestamp", abi = "latestConfirmedTimestamp()")]
    pub struct LatestConfirmedTimestampCall;
    ///Container type for all input parameters for the `latestConfirmedTimestampIdx` function with signature `latestConfirmedTimestampIdx()` and selector `0xec9a5359`
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
        name = "latestConfirmedTimestampIdx",
        abi = "latestConfirmedTimestampIdx()"
    )]
    pub struct LatestConfirmedTimestampIdxCall;
    ///Container type for all input parameters for the `latestConfirmedTimestampInfo` function with signature `latestConfirmedTimestampInfo()` and selector `0x0e988e25`
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
        name = "latestConfirmedTimestampInfo",
        abi = "latestConfirmedTimestampInfo()"
    )]
    pub struct LatestConfirmedTimestampInfoCall;
    ///Container type for all input parameters for the `lockInfo` function with signature `lockInfo(uint256)` and selector `0x5b1a4c24`
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
    #[ethcall(name = "lockInfo", abi = "lockInfo(uint256)")]
    pub struct LockInfoCall {
        pub job_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `registeredTransmitters` function with signature `registeredTransmitters(uint256)` and selector `0xd6748b02`
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
        name = "registeredTransmitters",
        abi = "registeredTransmitters(uint256)"
    )]
    pub struct RegisteredTransmittersCall {
        pub capture_timestamp: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `submitSlashResult` function with signature `submitSlashResult(uint256,uint256,uint256,bytes32,bytes,bytes)` and selector `0x381106d3`
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
        name = "submitSlashResult",
        abi = "submitSlashResult(uint256,uint256,uint256,bytes32,bytes,bytes)"
    )]
    pub struct SubmitSlashResultCall {
        pub index: ::ethers::core::types::U256,
        pub num_of_txs: ::ethers::core::types::U256,
        pub capture_timestamp: ::ethers::core::types::U256,
        pub image_id: [u8; 32],
        pub slash_result_data: ::ethers::core::types::Bytes,
        pub proof: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `submitVaultSnapshot` function with signature `submitVaultSnapshot(uint256,uint256,uint256,bytes32,bytes,bytes)` and selector `0x08b24318`
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
        name = "submitVaultSnapshot",
        abi = "submitVaultSnapshot(uint256,uint256,uint256,bytes32,bytes,bytes)"
    )]
    pub struct SubmitVaultSnapshotCall {
        pub index: ::ethers::core::types::U256,
        pub num_of_txs: ::ethers::core::types::U256,
        pub capture_timestamp: ::ethers::core::types::U256,
        pub image_id: [u8; 32],
        pub vault_snapshot_data: ::ethers::core::types::Bytes,
        pub proof: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `txCountInfo` function with signature `txCountInfo(uint256,bytes32)` and selector `0xd473e122`
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
    #[ethcall(name = "txCountInfo", abi = "txCountInfo(uint256,bytes32)")]
    pub struct TxCountInfoCall {
        pub capture_timestamp: ::ethers::core::types::U256,
        pub type_: [u8; 32],
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
    pub enum ISymbioticStakingCalls {
        ConfirmedTimestampInfo(ConfirmedTimestampInfoCall),
        GetOperatorActiveStakeAmount(GetOperatorActiveStakeAmountCall),
        GetOperatorStakeAmount(GetOperatorStakeAmountCall),
        GetStakeAmount(GetStakeAmountCall),
        GetStakeTokenList(GetStakeTokenListCall),
        GetStakeTokenWeights(GetStakeTokenWeightsCall),
        GetSubmissionStatus(GetSubmissionStatusCall),
        IsSupportedStakeToken(IsSupportedStakeTokenCall),
        LatestConfirmedTimestamp(LatestConfirmedTimestampCall),
        LatestConfirmedTimestampIdx(LatestConfirmedTimestampIdxCall),
        LatestConfirmedTimestampInfo(LatestConfirmedTimestampInfoCall),
        LockInfo(LockInfoCall),
        LockStake(LockStakeCall),
        OnJobCompletion(OnJobCompletionCall),
        RegisteredTransmitters(RegisteredTransmittersCall),
        RewardDistributor(RewardDistributorCall),
        Slash(SlashCall),
        StakeTokenSelectionWeightSum(StakeTokenSelectionWeightSumCall),
        SubmitSlashResult(SubmitSlashResultCall),
        SubmitVaultSnapshot(SubmitVaultSnapshotCall),
        TxCountInfo(TxCountInfoCall),
    }
    impl ::ethers::core::abi::AbiDecode for ISymbioticStakingCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ConfirmedTimestampInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConfirmedTimestampInfo(decoded));
            }
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
                <GetSubmissionStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSubmissionStatus(decoded));
            }
            if let Ok(decoded) =
                <IsSupportedStakeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsSupportedStakeToken(decoded));
            }
            if let Ok(decoded) =
                <LatestConfirmedTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestConfirmedTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LatestConfirmedTimestampIdxCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestConfirmedTimestampIdx(decoded));
            }
            if let Ok(decoded) =
                <LatestConfirmedTimestampInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestConfirmedTimestampInfo(decoded));
            }
            if let Ok(decoded) = <LockInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockInfo(decoded));
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
                <RegisteredTransmittersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisteredTransmitters(decoded));
            }
            if let Ok(decoded) =
                <RewardDistributorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RewardDistributor(decoded));
            }
            if let Ok(decoded) = <SlashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Slash(decoded));
            }
            if let Ok(decoded) =
                <StakeTokenSelectionWeightSumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeTokenSelectionWeightSum(decoded));
            }
            if let Ok(decoded) =
                <SubmitSlashResultCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitSlashResult(decoded));
            }
            if let Ok(decoded) =
                <SubmitVaultSnapshotCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitVaultSnapshot(decoded));
            }
            if let Ok(decoded) = <TxCountInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TxCountInfo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISymbioticStakingCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ConfirmedTimestampInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::GetSubmissionStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSupportedStakeToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestConfirmedTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestConfirmedTimestampIdx(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestConfirmedTimestampInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LockInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnJobCompletion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisteredTransmitters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardDistributor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Slash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeTokenSelectionWeightSum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitSlashResult(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitVaultSnapshot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TxCountInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ISymbioticStakingCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConfirmedTimestampInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorActiveStakeAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorStakeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeTokenList(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeTokenWeights(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubmissionStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSupportedStakeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestConfirmedTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestConfirmedTimestampIdx(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestConfirmedTimestampInfo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LockInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnJobCompletion(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisteredTransmitters(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardDistributor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slash(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenSelectionWeightSum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitSlashResult(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitVaultSnapshot(element) => ::core::fmt::Display::fmt(element, f),
                Self::TxCountInfo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConfirmedTimestampInfoCall> for ISymbioticStakingCalls {
        fn from(value: ConfirmedTimestampInfoCall) -> Self {
            Self::ConfirmedTimestampInfo(value)
        }
    }
    impl ::core::convert::From<GetOperatorActiveStakeAmountCall> for ISymbioticStakingCalls {
        fn from(value: GetOperatorActiveStakeAmountCall) -> Self {
            Self::GetOperatorActiveStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetOperatorStakeAmountCall> for ISymbioticStakingCalls {
        fn from(value: GetOperatorStakeAmountCall) -> Self {
            Self::GetOperatorStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetStakeAmountCall> for ISymbioticStakingCalls {
        fn from(value: GetStakeAmountCall) -> Self {
            Self::GetStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetStakeTokenListCall> for ISymbioticStakingCalls {
        fn from(value: GetStakeTokenListCall) -> Self {
            Self::GetStakeTokenList(value)
        }
    }
    impl ::core::convert::From<GetStakeTokenWeightsCall> for ISymbioticStakingCalls {
        fn from(value: GetStakeTokenWeightsCall) -> Self {
            Self::GetStakeTokenWeights(value)
        }
    }
    impl ::core::convert::From<GetSubmissionStatusCall> for ISymbioticStakingCalls {
        fn from(value: GetSubmissionStatusCall) -> Self {
            Self::GetSubmissionStatus(value)
        }
    }
    impl ::core::convert::From<IsSupportedStakeTokenCall> for ISymbioticStakingCalls {
        fn from(value: IsSupportedStakeTokenCall) -> Self {
            Self::IsSupportedStakeToken(value)
        }
    }
    impl ::core::convert::From<LatestConfirmedTimestampCall> for ISymbioticStakingCalls {
        fn from(value: LatestConfirmedTimestampCall) -> Self {
            Self::LatestConfirmedTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestConfirmedTimestampIdxCall> for ISymbioticStakingCalls {
        fn from(value: LatestConfirmedTimestampIdxCall) -> Self {
            Self::LatestConfirmedTimestampIdx(value)
        }
    }
    impl ::core::convert::From<LatestConfirmedTimestampInfoCall> for ISymbioticStakingCalls {
        fn from(value: LatestConfirmedTimestampInfoCall) -> Self {
            Self::LatestConfirmedTimestampInfo(value)
        }
    }
    impl ::core::convert::From<LockInfoCall> for ISymbioticStakingCalls {
        fn from(value: LockInfoCall) -> Self {
            Self::LockInfo(value)
        }
    }
    impl ::core::convert::From<LockStakeCall> for ISymbioticStakingCalls {
        fn from(value: LockStakeCall) -> Self {
            Self::LockStake(value)
        }
    }
    impl ::core::convert::From<OnJobCompletionCall> for ISymbioticStakingCalls {
        fn from(value: OnJobCompletionCall) -> Self {
            Self::OnJobCompletion(value)
        }
    }
    impl ::core::convert::From<RegisteredTransmittersCall> for ISymbioticStakingCalls {
        fn from(value: RegisteredTransmittersCall) -> Self {
            Self::RegisteredTransmitters(value)
        }
    }
    impl ::core::convert::From<RewardDistributorCall> for ISymbioticStakingCalls {
        fn from(value: RewardDistributorCall) -> Self {
            Self::RewardDistributor(value)
        }
    }
    impl ::core::convert::From<SlashCall> for ISymbioticStakingCalls {
        fn from(value: SlashCall) -> Self {
            Self::Slash(value)
        }
    }
    impl ::core::convert::From<StakeTokenSelectionWeightSumCall> for ISymbioticStakingCalls {
        fn from(value: StakeTokenSelectionWeightSumCall) -> Self {
            Self::StakeTokenSelectionWeightSum(value)
        }
    }
    impl ::core::convert::From<SubmitSlashResultCall> for ISymbioticStakingCalls {
        fn from(value: SubmitSlashResultCall) -> Self {
            Self::SubmitSlashResult(value)
        }
    }
    impl ::core::convert::From<SubmitVaultSnapshotCall> for ISymbioticStakingCalls {
        fn from(value: SubmitVaultSnapshotCall) -> Self {
            Self::SubmitVaultSnapshot(value)
        }
    }
    impl ::core::convert::From<TxCountInfoCall> for ISymbioticStakingCalls {
        fn from(value: TxCountInfoCall) -> Self {
            Self::TxCountInfo(value)
        }
    }
    ///Container type for all return fields from the `confirmedTimestampInfo` function with signature `confirmedTimestampInfo(uint256)` and selector `0x10c999bb`
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
    pub struct ConfirmedTimestampInfoReturn(pub ConfirmedTimestamp);
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
    ///Container type for all return fields from the `getSubmissionStatus` function with signature `getSubmissionStatus(uint256,address)` and selector `0xeb5f2b98`
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
    pub struct GetSubmissionStatusReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `latestConfirmedTimestamp` function with signature `latestConfirmedTimestamp()` and selector `0x33db2467`
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
    pub struct LatestConfirmedTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestConfirmedTimestampIdx` function with signature `latestConfirmedTimestampIdx()` and selector `0xec9a5359`
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
    pub struct LatestConfirmedTimestampIdxReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestConfirmedTimestampInfo` function with signature `latestConfirmedTimestampInfo()` and selector `0x0e988e25`
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
    pub struct LatestConfirmedTimestampInfoReturn(pub ConfirmedTimestamp);
    ///Container type for all return fields from the `lockInfo` function with signature `lockInfo(uint256)` and selector `0x5b1a4c24`
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
    pub struct LockInfoReturn {
        pub stake_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `registeredTransmitters` function with signature `registeredTransmitters(uint256)` and selector `0xd6748b02`
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
    pub struct RegisteredTransmittersReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `txCountInfo` function with signature `txCountInfo(uint256,bytes32)` and selector `0xd473e122`
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
    pub struct TxCountInfoReturn {
        pub idx_to_submit: ::ethers::core::types::U256,
        pub num_of_txs: ::ethers::core::types::U256,
    }
}
