pub use symbiotic_staking::*;
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
pub mod symbiotic_staking {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_generator_callback"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "contract IGeneratorCallbacks",
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BRIDGE_ENCLAVE_UPDATES_ROLE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("BRIDGE_ENCLAVE_UPDATES_ROLE",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("COMPLETE_MASK"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("COMPLETE_MASK"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("I_GENERATOR_CALLBACK"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("I_GENERATOR_CALLBACK",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IGeneratorCallbacks",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SIGNATURE_LENGTH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("SIGNATURE_LENGTH"),
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
                    ::std::borrow::ToOwned::to_owned("SLASH_RESULT_MASK"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("SLASH_RESULT_MASK"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("SLASH_RESULT_TYPE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("SLASH_RESULT_TYPE"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("STAKE_SNAPSHOT_MASK"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("STAKE_SNAPSHOT_MASK",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("STAKE_SNAPSHOT_TYPE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("STAKE_SNAPSHOT_TYPE",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("UPGRADE_INTERFACE_VERSION"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("UPGRADE_INTERFACE_VERSION",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addEnclaveImage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addEnclaveImage"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("PCRs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addEnclaveImage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCR2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addStakeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addStakeToken"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_weight"),
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
                    ::std::borrow::ToOwned::to_owned("amountToLock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("amountToLock"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("stakeToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("attestationVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("attestationVerifier",),
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
                    ::std::borrow::ToOwned::to_owned("baseTransmitterComissionRate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("baseTransmitterComissionRate",),
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
                    ::std::borrow::ToOwned::to_owned("confirmedTimestamps"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("confirmedTimestamps",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("captureTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transmitterComissionRate",),
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
                    ::std::borrow::ToOwned::to_owned("emergencyWithdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("emergencyWithdraw"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_to"),
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
                    ::std::borrow::ToOwned::to_owned("enclaveImages"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enclaveImages"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("imageId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("PCR0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("PCR1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("PCR2"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeRewardToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("feeRewardToken"),
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
                    ::std::borrow::ToOwned::to_owned("getImageId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getImageId"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("PCR0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("PCR1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("PCR2"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorActiveStakeAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorActiveStakeAmount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
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
                                name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("role"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("getStakeAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakeAmount"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_vault"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
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
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("grantRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hasRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_admin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_proofMarketplace"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakingManager"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_rewardDistributor",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_feeRewardToken"),
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
                    ::std::borrow::ToOwned::to_owned("isSupportedStakeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isSupportedStakeToken",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
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
                            name: ::std::borrow::ToOwned::to_owned("jobId"),
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
                                name: ::std::borrow::ToOwned::to_owned("_jobId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
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
                                name: ::std::borrow::ToOwned::to_owned("_jobId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_feeRewardAmount"),
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
                    ::std::borrow::ToOwned::to_owned("operatorLockedAmounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorLockedAmounts",),
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
                            name: ::std::borrow::ToOwned::to_owned("locked"),
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
                    ::std::borrow::ToOwned::to_owned("proofMarketplace"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("proofMarketplace"),
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
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("registeredTransmitters"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registeredTransmitters",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("captureTimestamp"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("transmitter"),
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
                    ::std::borrow::ToOwned::to_owned("removeEnclaveImage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeEnclaveImage"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_imageId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeStakeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeStakeToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("callerConfirmation",),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("setAmountToLock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAmountToLock"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                    ::std::borrow::ToOwned::to_owned("setAttestationVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAttestationVerifier",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_attestationVerifier",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setBaseTransmitterComissionRate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setBaseTransmitterComissionRate",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_baseTransmitterComission",),
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
                (
                    ::std::borrow::ToOwned::to_owned("setFeeRewardToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFeeRewardToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_feeRewardToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setJobManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setJobManager"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_jobManager"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setRewardDistributor"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setRewardDistributor",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_rewardDistributor",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setStakeTokenSelectionWeight"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setStakeTokenSelectionWeight",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_weight"),
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
                    ::std::borrow::ToOwned::to_owned("setStakingManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setStakingManager"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_stakingManager"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSubmissionCooldown"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSubmissionCooldown",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_submissionCooldown",),
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
                (
                    ::std::borrow::ToOwned::to_owned("slash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("slash"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_slashedJobs"),
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
                    ::std::borrow::ToOwned::to_owned("stakeTokenSelectionWeight"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeTokenSelectionWeight",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("stakeToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("weight"),
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
                    ::std::borrow::ToOwned::to_owned("stakingManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakingManager"),
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
                    ::std::borrow::ToOwned::to_owned("submissionCooldown"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submissionCooldown"),
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
                    ::std::borrow::ToOwned::to_owned("submissionStatus"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submissionStatus"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("captureTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("status"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("txCountInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("txCountInfo"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("captureTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("submissionType"),
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
                (
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlBadConfirmation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AccessControlBadConfirmation",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlUnauthorizedAccount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AccessControlUnauthorizedAccount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("neededRole"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("target"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignature",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureLength"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureLength",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("length"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureS"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureS",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("s"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967InvalidImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1967InvalidImplementation",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedCall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInitialization"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidInitialization",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotInitializing"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotInitializing"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReentrancyGuardReentrantCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ReentrancyGuardReentrantCall",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnauthorizedCallContext"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UUPSUnauthorizedCallContext",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnsupportedProxiableUUID"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UUPSUnsupportedProxiableUUID",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("slot"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SYMBIOTICSTAKING_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R0`\x80R4\x80\x15a\0\x14W`\0\x80\xFD[P`@QaWt8\x03\x80aWt\x839\x81\x01`@\x81\x90Ra\x003\x91a\0DV[`\x01`\x01`\xA0\x1B\x03\x16`\xA0Ra\0tV[`\0` \x82\x84\x03\x12\x15a\0VW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0mW`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0QaV\xB1a\0\xC3`\09`\0\x81\x81a\x05\x04\x01R\x81\x81a\x18H\x01R\x81\x81a& \x01R\x81\x81a)7\x01Ra8&\x01R`\0\x81\x81a8\xBC\x01R\x81\x81a8\xE5\x01Ra:)\x01RaV\xB1`\0\xF3\xFE`\x80`@R`\x046\x10a\x03\xEFW`\x005`\xE0\x1C\x80c\x81\xC4\\p\x11a\x02\x08W\x80c\xAD<\xB1\xCC\x11a\x01\x18W\x80c\xCB\x95qz\x11a\0\xABW\x80c\xD5Gt\x1F\x11a\0zW\x80c\xD5Gt\x1F\x14a\x0C\xF3W\x80c\xD6t\x8B\x02\x14a\r\x13W\x80c\xDE\xE8\xBC\xE2\x14a\rJW\x80c\xEB_+\x98\x14a\rjW\x80c\xEC\x9ASY\x14a\r\xAEW`\0\x80\xFD[\x80c\xCB\x95qz\x14a\x0C.W\x80c\xD1\x1F)<\x14a\x0C\\W\x80c\xD3e\xD2\xD6\x14a\x0C~W\x80c\xD4s\xE1\"\x14a\x0C\x9EW`\0\x80\xFD[\x80c\xB9,\"z\x11a\0\xE7W\x80c\xB9,\"z\x14a\x0B\xC2W\x80c\xC0\xFF\x97\xEF\x14a\x0B\xD9W\x80c\xC4\x0C\xC6\xE9\x14a\x0B\xF9W\x80c\xC9\x06\x1Aj\x14a\x0C\x19W`\0\x80\xFD[\x80c\xAD<\xB1\xCC\x14a\x0B.W\x80c\xB0\x0B\xBAj\x14a\x0BlW\x80c\xB1\xB3p\x07\x14a\x0B\x8CW\x80c\xB4\xB3\xC5\xA0\x14a\x0B\xA1W`\0\x80\xFD[\x80c\x9B\x88\xEE\x16\x11a\x01\x9BW\x80c\xA1\x80\x9B\x95\x11a\x01jW\x80c\xA1\x80\x9B\x95\x14a\n\xA1W\x80c\xA2\x17\xFD\xDF\x14a\n\xC1W\x80c\xA6\xFF\xE5'\x14a\n\xD6W\x80c\xACh\xD8w\x14a\n\xEDW\x80c\xAC\xC2\x16j\x14a\x0B\rW`\0\x80\xFD[\x80c\x9B\x88\xEE\x16\x14a\n(W\x80c\x9E:,\x9E\x14a\nJW\x80c\x9F=T^\x14a\njW\x80c\x9F\xCD\x11_\x14a\n\x81W`\0\x80\xFD[\x80c\x8D;\x9DZ\x11a\x01\xD7W\x80c\x8D;\x9DZ\x14a\t\x8BW\x80c\x8F\x9C0\x95\x14a\t\xB9W\x80c\x91\xD1HT\x14a\t\xE8W\x80c\x94]\x0FL\x14a\n\x08W`\0\x80\xFD[\x80c\x81\xC4\\p\x14a\t\nW\x80c\x81\xF7\xD9B\x14a\t+W\x80c\x82\xF6\xF6\x08\x14a\tKW\x80c\x89\xC7\xB9\x87\x14a\tkW`\0\x80\xFD[\x80c3\xF4\x90\x9F\x11a\x03\x03W\x80cQ\xC3\x9Cf\x11a\x02\x96W\x80c[\x1AL$\x11a\x02eW\x80c[\x1AL$\x14a\x08%W\x80c_A=8\x14a\x08\x85W\x80c`\xFB\xA4\x8B\x14a\x08\xA8W\x80cc\x82\xD9\xAD\x14a\x08\xCAW\x80c\x80u\xC2j\x14a\x08\xEAW`\0\x80\xFD[\x80cQ\xC3\x9Cf\x14a\x07\xBBW\x80cR\xD1\x90-\x14a\x07\xDBW\x80cT\x0B\xC5\xEA\x14a\x07\xF0W\x80cW$\xB8N\x14a\x08\x05W`\0\x80\xFD[\x80c=t\xA7x\x11a\x02\xD2W\x80c=t\xA7x\x14a\x07GW\x80c=\xF3m\xDF\x14a\x07hW\x80cN\x82\x11\x02\x14a\x07\x88W\x80cO\x1E\xF2\x86\x14a\x07\xA8W`\0\x80\xFD[\x80c3\xF4\x90\x9F\x14a\x06\xC5W\x80c6V\x8A\xBE\x14a\x06\xE5W\x80c6q\x96s\x14a\x07\x05W\x80c8\x11\x06\xD3\x14a\x07'W`\0\x80\xFD[\x80c\x16\xB7\x1E\xF4\x11a\x03\x86W\x80c\x1F=\xC4\xE2\x11a\x03UW\x80c\x1F=\xC4\xE2\x14a\x06/W\x80c\"\x82\x8C\xC2\x14a\x06OW\x80c$\x8A\x9C\xA3\x14a\x06pW\x80c//\xF1]\x14a\x06\x90W\x80c3\xDB$g\x14a\x06\xB0W`\0\x80\xFD[\x80c\x16\xB7\x1E\xF4\x14a\x05~W\x80c\x1B\xE8\xF4\x83\x14a\x05\xC1W\x80c\x1C\xCB=f\x14a\x05\xFAW\x80c\x1E\xF9\x17j\x14a\x06\x0FW`\0\x80\xFD[\x80c\x10\xC9\x99\xBB\x11a\x03\xC2W\x80c\x10\xC9\x99\xBB\x14a\x04\xD2W\x80c\x11\x0C\xBC\x80\x14a\x04\xF2W\x80c\x14YEz\x14a\x05>W\x80c\x16hf\xC7\x14a\x05^W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x03\xF4W\x80c\x08\xB2C\x18\x14a\x04)W\x80c\nY\xBBP\x14a\x04KW\x80c\x0E\x98\x8E%\x14a\x04\x92W[`\0\x80\xFD[4\x80\x15a\x04\0W`\0\x80\xFD[Pa\x04\x14a\x04\x0F6`\x04aE\xB4V[a\r\xC3V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x045W`\0\x80\xFD[Pa\x04Ia\x04D6`\x04aF&V[a\r\xD4V[\0[4\x80\x15a\x04WW`\0\x80\xFD[Pa\x04\x84a\x04f6`\x04aF\xD4V[a\x03\xFA` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x04 V[4\x80\x15a\x04\x9EW`\0\x80\xFD[Pa\x04\xA7a\x0F9V[`@\x80Q\x82Q\x81R` \x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01a\x04 V[4\x80\x15a\x04\xDEW`\0\x80\xFD[Pa\x04\xA7a\x04\xED6`\x04aG\rV[a\x0F\xC9V[4\x80\x15a\x04\xFEW`\0\x80\xFD[Pa\x05&\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04 V[4\x80\x15a\x05JW`\0\x80\xFD[Pa\x04Ia\x05Y6`\x04aG&V[a\x10SV[4\x80\x15a\x05jW`\0\x80\xFD[Pa\x04Ia\x05y6`\x04aG\x97V[a\x14jV[4\x80\x15a\x05\x8AW`\0\x80\xFD[Pa\x05\x9Ea\x05\x996`\x04aG\rV[a\x15=V[`@\x80Q\x93\x84R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\x04 V[4\x80\x15a\x05\xCDW`\0\x80\xFD[Pa\x04\x84a\x05\xDC6`\x04aG\xB4V[a\x03\xF6` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x06\x06W`\0\x80\xFD[Pa\x04\x84`\x11\x81V[4\x80\x15a\x06\x1BW`\0\x80\xFD[Pa\x04Ia\x06*6`\x04aG\x97V[a\x15}V[4\x80\x15a\x06;W`\0\x80\xFD[Pa\x04Ia\x06J6`\x04aG\xD9V[a\x15\xDFV[4\x80\x15a\x06[W`\0\x80\xFD[Pa\x01\xF9Ta\x05&\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06|W`\0\x80\xFD[Pa\x04\x84a\x06\x8B6`\x04aG\rV[a\x163V[4\x80\x15a\x06\x9CW`\0\x80\xFD[Pa\x04Ia\x06\xAB6`\x04aG\xB4V[a\x16UV[4\x80\x15a\x06\xBCW`\0\x80\xFD[Pa\x04\x84a\x16wV[4\x80\x15a\x06\xD1W`\0\x80\xFD[Pa\x04Ia\x06\xE06`\x04aG\xB4V[a\x16\xC1V[4\x80\x15a\x06\xF1W`\0\x80\xFD[Pa\x04Ia\x07\x006`\x04aG\xB4V[a\x18\xB9V[4\x80\x15a\x07\x11W`\0\x80\xFD[Pa\x04\x84`\0\x80Q` aV\x1C\x839\x81Q\x91R\x81V[4\x80\x15a\x073W`\0\x80\xFD[Pa\x04Ia\x07B6`\x04aI4V[a\x18\xF1V[4\x80\x15a\x07SW`\0\x80\xFD[Pa\x01\xFDTa\x05&\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07tW`\0\x80\xFD[Pa\x04Ia\x07\x836`\x04aG\x97V[a\x1B\x02V[4\x80\x15a\x07\x94W`\0\x80\xFD[Pa\x04\x14a\x07\xA36`\x04aG\x97V[a\x1BYV[a\x04Ia\x07\xB66`\x04aI\xC0V[a\x1BgV[4\x80\x15a\x07\xC7W`\0\x80\xFD[Pa\x04Ia\x07\xD66`\x04aG\rV[a\x1B\x86V[4\x80\x15a\x07\xE7W`\0\x80\xFD[Pa\x04\x84a\x1B\xC7V[4\x80\x15a\x07\xFCW`\0\x80\xFD[Pa\x04\x84`A\x81V[4\x80\x15a\x08\x11W`\0\x80\xFD[Pa\x04Ia\x08 6`\x04aG\xD9V[a\x1B\xE4V[4\x80\x15a\x081W`\0\x80\xFD[Pa\x08fa\x08@6`\x04aG\rV[a\x03\xF9` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x04 V[4\x80\x15a\x08\x91W`\0\x80\xFD[Pa\x08\x9Aa\x1C\x91V[`@Qa\x04 \x92\x91\x90aJTV[4\x80\x15a\x08\xB4W`\0\x80\xFD[Pa\x04\x84`\0\x80Q` aV<\x839\x81Q\x91R\x81V[4\x80\x15a\x08\xD6W`\0\x80\xFD[Pa\x04Ia\x08\xE56`\x04aF\xD4V[a\x1DdV[4\x80\x15a\x08\xF6W`\0\x80\xFD[Pa\x04Ia\t\x056`\x04aG\rV[a\x1E\x81V[4\x80\x15a\t\x16W`\0\x80\xFD[Pa\x01\xFATa\x05&\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\t7W`\0\x80\xFD[Pa\x04Ia\tF6`\x04aG\xD9V[a\x1E\xA2V[4\x80\x15a\tWW`\0\x80\xFD[Pa\x04Ia\tf6`\x04aG\x97V[a\x1F]V[4\x80\x15a\twW`\0\x80\xFD[Pa\x04\x84a\t\x866`\x04aF\xD4V[a\x1F~V[4\x80\x15a\t\x97W`\0\x80\xFD[Pa\x04\x84a\t\xA66`\x04aG\x97V[a\x03\xF3` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\t\xC5W`\0\x80\xFD[Pa\t\xD9a\t\xD46`\x04aG\rV[a\x1F\xD6V[`@Qa\x04 \x93\x92\x91\x90aJ\xFEV[4\x80\x15a\t\xF4W`\0\x80\xFD[Pa\x04\x14a\n\x036`\x04aG\xB4V[a!\x91V[4\x80\x15a\n\x14W`\0\x80\xFD[Pa\x04Ia\n#6`\x04aKAV[a!\xC9V[4\x80\x15a\n4W`\0\x80\xFD[Pa\x04\x84`\0\x80Q` aU\xDC\x839\x81Q\x91R\x81V[4\x80\x15a\nVW`\0\x80\xFD[Pa\x04\x84a\ne6`\x04aK}V[a!\xEAV[4\x80\x15a\nvW`\0\x80\xFD[Pa\x04\x84a\x01\xF4T\x81V[4\x80\x15a\n\x8DW`\0\x80\xFD[Pa\x04\x84a\n\x9C6`\x04aL\x0EV[a\"!V[4\x80\x15a\n\xADW`\0\x80\xFD[Pa\x04Ia\n\xBC6`\x04aG\x97V[a\"rV[4\x80\x15a\n\xCDW`\0\x80\xFD[Pa\x04\x84`\0\x81V[4\x80\x15a\n\xE2W`\0\x80\xFD[Pa\x04\x84a\x01\xF8T\x81V[4\x80\x15a\n\xF9W`\0\x80\xFD[Pa\x04Ia\x0B\x086`\x04aK}V[a\"\xCCV[4\x80\x15a\x0B\x19W`\0\x80\xFD[Pa\x01\xFBTa\x05&\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x0B:W`\0\x80\xFD[Pa\x0B_`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x04 \x91\x90aLYV[4\x80\x15a\x0BxW`\0\x80\xFD[Pa\x04Ia\x0B\x876`\x04aG\x97V[a#\x10V[4\x80\x15a\x0B\x98W`\0\x80\xFD[Pa\x04\x84`\x01\x81V[4\x80\x15a\x0B\xADW`\0\x80\xFD[Pa\x01\xFCTa\x05&\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x0B\xCEW`\0\x80\xFD[Pa\x04\x84a\x01\xF5T\x81V[4\x80\x15a\x0B\xE5W`\0\x80\xFD[Pa\x04Ia\x0B\xF46`\x04aLlV[a#gV[4\x80\x15a\x0C\x05W`\0\x80\xFD[Pa\x04Ia\x0C\x146`\x04aG\rV[a&XV[4\x80\x15a\x0C%W`\0\x80\xFD[Pa\x04\x84`\x10\x81V[4\x80\x15a\x0C:W`\0\x80\xFD[Pa\x04\x84a\x0CI6`\x04aG\x97V[a\x03\xF4` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x0ChW`\0\x80\xFD[Pa\x0Cqa&\xE9V[`@Qa\x04 \x91\x90aL\xA4V[4\x80\x15a\x0C\x8AW`\0\x80\xFD[Pa\x04\x84a\x0C\x996`\x04aF\xD4V[a&\xFBV[4\x80\x15a\x0C\xAAW`\0\x80\xFD[Pa\x0C\xDEa\x0C\xB96`\x04aL\xB7V[a\x03\xF5` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x04 V[4\x80\x15a\x0C\xFFW`\0\x80\xFD[Pa\x04Ia\r\x0E6`\x04aG\xB4V[a'AV[4\x80\x15a\r\x1FW`\0\x80\xFD[Pa\x05&a\r.6`\x04aG\rV[a\x03\xFB` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\rVW`\0\x80\xFD[Pa\x04Ia\re6`\x04aL\xD9V[a']V[4\x80\x15a\rvW`\0\x80\xFD[Pa\x04\x84a\r\x856`\x04aG\xB4V[`\0\x91\x82Ra\x03\xF6` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[4\x80\x15a\r\xBAW`\0\x80\xFD[Pa\x04\x84a)\xF4V[`\0a\r\xCE\x82a*\x12V[\x92\x91PPV[`\0a\r\xE2\x84\x86\x01\x86aMqV[\x90Pa\r\xED\x87a*GV[a\x0E\x07\x89\x89\x89`\0\x80Q` aV<\x839\x81Q\x91Ra*\xF0V[a\x0E\x8D\x86`\0\x80Q` aV<\x839\x81Q\x91R\x8B\x8B\x8B\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8E\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8C\x81R\x92P\x8C\x91P\x8B\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa-\x14\x92PPPV[a\x0E\x97\x87\x82a/\xA6V[a\x0E\xB0\x88\x88`\0\x80Q` aV<\x839\x81Q\x91Ra0\xD6V[a\x0E\xBB`\x01\x89aNgV[\x89\x03a\x0E\xE3W`\0\x87\x81Ra\x03\xF6` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 \x80T`\x01\x17\x90U[\x85\x893`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAC\x08E\x01\xA5a>\xD4\xB57rV\x86\xA6\xB9\xF9x\xEC\x9B\xB4a\xCA\x9743\xD1\x9B#]\x03\x86F\x8B\x89\x89\x89\x89`@Qa\x0F&\x95\x94\x93\x92\x91\x90aN\xA3V[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPV[a\x0Ff`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[a\x01\xFEa\x0Fqa)\xF4V[\x81T\x81\x10a\x0F\x81Wa\x0F\x81aN\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R`\x03\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x93\x83\x01\x93\x90\x93R`\x02\x90\x92\x01T\x91\x81\x01\x91\x90\x91R\x91\x90PV[a\x0F\xF6`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[a\x01\xFE\x82\x81T\x81\x10a\x10\nWa\x10\naN\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R`\x03\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x93\x83\x01\x93\x90\x93R`\x02\x90\x92\x01T\x91\x81\x01\x91\x90\x91R\x92\x91PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\x10\x98WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x10\xB4WP0;\x15[\x90P\x81\x15\x80\x15a\x10\xC2WP\x80\x15[\x15a\x10\xE0W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x11\nW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x11\x12a1cV[a\x11\x1Aa1cV[a\x11\"a1cV[a\x11*a1cV[a\x112a1mV[a\x11=`\0\x8Ba1\x9BV[P`\x01`\x01`\xA0\x1B\x03\x88\x16a\x11\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSymbioticStaking: stakingManager`D\x82\x01Rg is zero`\xC0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xF9\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB9\xE8\x9D\x19:q\x1D\xD2\x9A%3(\xFA\xAEn\xF5;\xD4\x1F\x10\x01sDl\x82 \xA3\x8A\x02.$\x0C\x90`\0\x90\xA2`\x01`\x01`\xA0\x1B\x03\x89\x16a\x12[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSymbioticStaking: proofMarketpla`D\x82\x01Rice is zero`\xB0\x1B`d\x82\x01R`\x84\x01a\x11\xA1V[a\x01\xFA\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x89 \xA3\x12\x98y\xB1\xE5L\x92Tjt\x8BbI\xBA/\x9A\x03\x83\xF4I\x9D\x9FJ\xE4;\x90\x9A\xD7\x95\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x87\x16a\x13\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FSymbioticStaking: rewardDistribu`D\x82\x01Rjtor is zero`\xA8\x1B`d\x82\x01R`\x84\x01a\x11\xA1V[a\x01\xFB\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x07\\\x02\xC5\x13\xA4\x15\xBDO\xF5\x97o\x8A\xA6\xFCWg\xD2\x18=\xAC\xA9\xEC\0\xABq\xCEx\xE8\xBF\x81X\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x86\x16a\x13\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSymbioticStaking: feeRewardToken`D\x82\x01Rg is zero`\xC0\x1B`d\x82\x01R`\x84\x01a\x11\xA1V[a\x01\xFD\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x90\x81\x17\x90\x91U`@Q\x7F_\x8E+\x8B8\xD2\x0EPBt\xAB\xF2\x84z\xD3/Jm\x03\xB3\xCCu\xD0\x18\xFBW\x11Y9u\x96\xC7\x90`\0\x90\xA2\x83\x15a\x14^W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPV[`\0a\x14u\x81a2@V[a\x14\x81a\x01\xF6\x83a2JV[a\x14\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x1B\xDA\xD9[\x88\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`b\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03\xF4` R`@\x81 Ta\x01\xF8\x80T\x91\x92\x90\x91a\x14\xF2\x90\x84\x90aNgV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x03\xF4` R`@\x80\x82 \x82\x90UQ\x7FzP/\xFF\xACP\xF2\xA88\xC9U\x04M\xECh\xB2rd\x17B\xA6\x86\x17\xAB\xE1nI\x82\xAA\xC3yu\x91\x90\xA2PPV[a\x01\xFE\x81\x81T\x81\x10a\x15NW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83V[`\0a\x15\x88\x81a2@V[a\x01\xFA\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x89 \xA3\x12\x98y\xB1\xE5L\x92Tjt\x8BbI\xBA/\x9A\x03\x83\xF4I\x9D\x9FJ\xE4;\x90\x9A\xD7\x95\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[`\0a\x15\xEA\x81a2@V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03\xF3` R`@\x80\x82 \x85\x90UQ\x84\x92\x91\x7F\xC3`\x93\x99-6\x17;\xC34\x83\xBA\xE4N\xEC\xC3\x90?\x841\xE5\xC0\x8A\x07\xC1\x94Z\x89f\x133\x83\x91\xA3PPPV[`\0\x90\x81R`\0\x80Q` aV\\\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[a\x16^\x82a\x163V[a\x16g\x81a2@V[a\x16q\x83\x83a1\x9BV[PPPPV[a\x01\xFET`\0\x90\x80a\x16\x8AW`\0a\x16\xBBV[a\x01\xFEa\x16\x98`\x01\x83aNgV[\x81T\x81\x10a\x16\xA8Wa\x16\xA8aN\xDCV[\x90`\0R` `\0 \x90`\x03\x02\x01`\0\x01T[\x91PP\x90V[a\x01\xF9T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x11\xA1\x90aN\xF2V[`\0a\x16\xF7\x82a2_V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x03\xF3` R`@\x90 T\x90\x91P\x80a\x17\x1F\x83\x85a\x1F~V[\x10\x15a\x17mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInsufficient stake amount\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11\xA1V[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x80\x83R` \x80\x84\x01\x86\x81R`\0\x8A\x81Ra\x03\xF9\x83R\x86\x81 \x95Q\x86T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x86\x16\x17\x86U\x90Q`\x01\x90\x95\x01\x94\x90\x94U\x90\x83Ra\x03\xFA\x81R\x83\x83 \x91\x87\x16\x83RR\x90\x81 \x80T\x83\x92\x90a\x17\xDC\x90\x84\x90aO\x1FV[\x92PP\x81\x90UP\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7FV\xAC\xD9\xAAF\x83\x1C\xF3\xE8\x03\xC8\x9A`\xEE\xCA\"\xE7\r\x8D\xD3#;\xCB\x1F\x9EH\x9394\x0C\xA0\x01\x84`@Qa\x18)\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4`@Qc \x1A\x11\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c \x1A\x11\xB3\x90a\x18\x81\x90\x86\x90\x86\x90\x86\x90`\x04\x01aO2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\xAFW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x18\xE2W`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xEC\x82\x82a6\x91V[PPPV[\x81Q``\x90\x15a\x19\x12W\x82\x80` \x01\x90Q\x81\x01\x90a\x19\x0F\x91\x90aOVV[\x90P[`\0\x85\x81Ra\x03\xF6` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\x01\x90\x81\x16\x14a\x19\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FVault Snapshot not submitted\0\0\0\0`D\x82\x01R`d\x01a\x11\xA1V[a\x19\x88\x85a*GV[a\x19\xA2\x87\x87\x87`\0\x80Q` aU\xDC\x839\x81Q\x91Ra*\xF0V[a\x19\xBF\x84`\0\x80Q` aU\xDC\x839\x81Q\x91R\x89\x89\x89\x88\x88a-\x14V[a\x19\xD8\x86\x86`\0\x80Q` aU\xDC\x839\x81Q\x91Ra0\xD6V[`\0\x85\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 `\0\x80Q` aV<\x839\x81Q\x91R\x84R\x82R\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x81Q\x15a\x1A\x80Wa\x01\xF9T`@QcM\"\xFD1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\"\xFD1\x90a\x1AM\x90\x85\x90`\x04\x01aP$V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1AgW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A{W=`\0\x80>=`\0\xFD[PPPP[\x84\x883`\x01`\x01`\xA0\x1B\x03\x16\x7F1\x9C\xE6\r\x9E@\xF3\xA6\x8C\xC1a\xE6\x19\x1Dm-\x8F^l\xA3w\xC8\xBB\x86\xC4\xE2]r\x7F\xE8M\x08\x8A\x88\x88`@Qa\x1A\xBF\x93\x92\x91\x90aP\x8CV[`@Q\x80\x91\x03\x90\xA4a\x1A\xD2`\x01\x88aNgV[\x88\x03a\x18\xAFW`\0\x86\x81Ra\x03\xF6` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 \x80T`\x01\x17\x90Ua\x18\xAF\x86a7\rV[`\0a\x1B\r\x81a2@V[a\x01\xFD\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F_\x8E+\x8B8\xD2\x0EPBt\xAB\xF2\x84z\xD3/Jm\x03\xB3\xCCu\xD0\x18\xFBW\x11Y9u\x96\xC7\x90`\0\x90\xA2PPV[`\0a\r\xCEa\x01\xF6\x83a8\x8FV[a\x1Boa8\xB1V[a\x1Bx\x82a9VV[a\x1B\x82\x82\x82a9aV[PPV[`\0a\x1B\x91\x81a2@V[a\x01\xF4\x82\x90U`@Q\x82\x81R\x7FW\xAD\xFC\xAD\x07\xDD\xCC;\xA3\xEF;\x02\x12\xB8\xAEy\x19q\xF5q\x8FM\xB4\xEClN\x19>\xE1\xFDG\0\x90` \x01a\x15\xD3V[`\0a\x1B\xD1a:\x1EV[P`\0\x80Q` aU\xFC\x839\x81Q\x91R\x90V[`\0a\x1B\xEF\x81a2@V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\xF4` R`@\x81 Ta\x01\xF8\x80T\x91\x92\x90\x91a\x1C\x1D\x90\x84\x90aNgV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\xF4` R`@\x81 \x83\x90Ua\x01\xF8\x80T\x84\x92\x90a\x1CQ\x90\x84\x90aO\x1FV[\x90\x91UPP`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F0#5\xD1>\t\xEC\xAF\xF1\xF0\xAE\xD0\xB1\x9BZ\xD3\xA0T\t>\xA0qZ6\xB6\xA4\xBC\"\x9BR\x95\x82\x90`\0\x90\xA3PPPV[``\x80`\0a\x1C\xA1a\x01\xF6a:gV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xB8Wa\x1C\xB8aH\x05V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C\xE1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[a\x1C\xF2a\x01\xF6a:gV[\x81\x10\x15a\x1DPWa\x03\xF4`\0a\x1D\na\x01\xF6\x84a:qV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x1D=Wa\x1D=aN\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1C\xE7V[Pa\x1D\\a\x01\xF6a:}V[\x93\x90\x92P\x90PV[`\0a\x1Do\x81a2@V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x1D\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqzero token address`p\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1E\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rnzero to address`\x88\x1B`D\x82\x01R`d\x01a\x11\xA1V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x18\xEC\x90\x83\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ELW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ep\x91\x90aP\xB7V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x91\x90a:\x8AV[`\0\x80Q` aV\x1C\x839\x81Q\x91Ra\x1E\x99\x81a2@V[a\x1B\x82\x82a:\xDCV[`\0a\x1E\xAD\x81a2@V[a\x1E\xB9a\x01\xF6\x84a;BV[a\x1E\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsToken already exists``\x1B`D\x82\x01R`d\x01a\x11\xA1V[\x81a\x01\xF8`\0\x82\x82Ta\x1F\x0F\x91\x90aO\x1FV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03\xF4` R`@\x80\x82 \x85\x90UQ\x84\x92\x91\x7F\x0C\x96U\xA8\r\xB9\xD3N#\xDD\x9DH<\xB2\xB6\x84\xE4v\x11T\xA6o\x93]\x9BB\xC1t\x89-\xD3%\x91\xA3PPPV[`\0\x80Q` aV\x1C\x839\x81Q\x91Ra\x1Fu\x81a2@V[a\x1B\x82\x82a;WV[`\0\x80a\x1F\x8B\x84\x84a&\xFBV[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81Ra\x03\xFA` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T\x90\x91P\x80\x82\x11a\x1F\xC3W`\0a\x1F\xCDV[a\x1F\xCD\x81\x83aNgV[\x95\x94PPPPPV[a\x03\xFC` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x1F\xF2\x90aP\xD0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \x1E\x90aP\xD0V[\x80\x15a kW\x80`\x1F\x10a @Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a kV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a NW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta \x80\x90aP\xD0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \xAC\x90aP\xD0V[\x80\x15a \xF9W\x80`\x1F\x10a \xCEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a \xF9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a \xDCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta!\x0E\x90aP\xD0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!:\x90aP\xD0V[\x80\x15a!\x87W\x80`\x1F\x10a!\\Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\x87V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!jW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[`\0\x91\x82R`\0\x80Q` aV\\\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x80Q` aV\x1C\x839\x81Q\x91Ra!\xE1\x81a2@V[a\x1B\x82\x82a;\xACV[`\0\x83\x83\x83`@Q` \x01a\"\x01\x93\x92\x91\x90aJ\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x93\x92PPPV[`\0a\x03\xF8`\0a\"0a\x16wV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x82R\x83R\x81\x81 \x95\x87\x16\x81R\x94\x82R\x80\x85 \x93\x90\x95\x16\x84R\x91\x90\x91RP T\x90V[`\0a\"}\x81a2@V[a\x01\xFB\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x07\\\x02\xC5\x13\xA4\x15\xBDO\xF5\x97o\x8A\xA6\xFCWg\xD2\x18=\xAC\xA9\xEC\0\xABq\xCEx\xE8\xBF\x81X\x90` \x01a\x15\xD3V[`\0\x80Q` aV\x1C\x839\x81Q\x91Ra\"\xE4\x81a2@V[a\x16q\x84\x84\x84`@Q` \x01a\"\xFC\x93\x92\x91\x90aJ\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra;\xACV[`\0a#\x1B\x81a2@V[a\x01\xF9\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB9\xE8\x9D\x19:q\x1D\xD2\x9A%3(\xFA\xAEn\xF5;\xD4\x1F\x10\x01sDl\x82 \xA3\x8A\x02.$\x0C\x90`\0\x90\xA2PPV[a\x01\xF9T`\x01`\x01`\xA0\x1B\x03\x163\x14a#\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x11\xA1\x90aN\xF2V[`\0\x83\x81Ra\x03\xF9` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x01T\x90\x82\x01R\x81\x15a%*W`\0a#\xD2a)\xF4V[\x90P`\0a$\x0E\x84a\x01\xFE\x84\x81T\x81\x10a#\xEEWa#\xEEaN\xDCV[\x90`\0R` `\0 \x90`\x03\x02\x01`\x02\x01Tg\r\xE0\xB6\xB3\xA7d\0\0a=\xBEV[\x90P`\0a$\x1C\x82\x86aNgV[a\x01\xFATa\x01\xFE\x80T\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC0\x88y)\x91\x90\x86\x90\x81\x10a$KWa$KaN\xDCV[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01`\x01\x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a$\xA7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a$\xBBW=`\0\x80>=`\0\xFD[PPa\x01\xFBT\x86Q`@Qc\x01|\xD4\xB5`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93Pc/\x9A\x96\xA0\x92Pa$\xF4\x91\x8A\x90\x86\x90`\x04\x01aO2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a%\x0EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a%\"W=`\0\x80>=`\0\xFD[PPPPPPP[`\0\x84\x81Ra\x03\xF9` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x01\x83\x90U\x83Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84Ra\x03\xF3\x83R\x81\x84 T\x85Q\x82\x16\x85Ra\x03\xFA\x84R\x82\x85 \x91\x88\x16\x85R\x92R\x82 \x80T\x91\x92\x90\x91a%\x90\x90\x84\x90aNgV[\x90\x91UPP\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x81\x81Ra\x03\xF3` R`@\x90\x81\x90 T\x90Q\x91\x92\x86\x16\x91\x87\x91\x7F\xB8\xECr\xF7\xC3EH\xFB\xB7\xD0\xE8\x8D\x93\xF2\xAA\x1E\x8C7\x11\x17y\x86\xB4\xD5\xBC\r\xAB\xF5\x86\x8C\xA1\xAB\x91a%\xEB\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x03\xF3` R`@\x90\x81\x90 T\x90Qc\x18\xDA\x93]`\xE1\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x92c1\xB5&\xBA\x92a\x18\x81\x92\x88\x92\x91`\x04\x01aO2V[`\0a&c\x81a2@V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x10a&\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInvalid comission rate`P\x1B`D\x82\x01R`d\x01a\x11\xA1V[a\x01\xF5\x82\x90U`@Q\x82\x81R\x7F\x18a\x8A!\xCB\xFD\xD5\x91ki\xF4\xC9-\x1B\xBC\x98\xF8\xC0\xE1J\x85\xDD\x9CN\xF8\x90\xBD\x15\xD3I\xD1\xE5\x90` \x01a\x15\xD3V[``a&\xF6a\x01\xF6a:}V[\x90P\x90V[`\0a\x03\xF7`\0a'\na\x16wV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x82R\x83R\x81\x81 \x94\x90\x95\x16\x85R\x92\x90RP\x90 T\x90V[a'J\x82a\x163V[a'S\x81a2@V[a\x16q\x83\x83a6\x91V[a\x01\xF9T`\x01`\x01`\xA0\x1B\x03\x163\x14a'\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x11\xA1\x90aN\xF2V[\x80`\0[\x81\x81\x10\x15a\x16qW`\0a\x03\xF9`\0\x86\x86\x85\x81\x81\x10a'\xADWa'\xADaN\xDCV[``\x02\x91\x90\x91\x015\x82RP` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x82Q\x80\x84\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x80\x82R`\x01\x90\x92\x01T\x81\x86\x01\x81\x90R\x91\x83Ra\x03\xFA\x90\x94R\x91\x81 \x92\x93P\x90\x91\x82\x91\x88\x88\x87\x81\x81\x10a(\x11Wa(\x11aN\xDCV[\x90P``\x02\x01` \x01` \x81\x01\x90a()\x91\x90aG\x97V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta(X\x91\x90aNgV[\x90\x91UPa\x03\xF9\x90P`\0\x87\x87\x86\x81\x81\x10a(uWa(uaN\xDCV[``\x02\x91\x90\x91\x015\x82RP` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x01U\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85\x81\x81\x10a(\xBFWa(\xBFaN\xDCV[\x90P``\x02\x01` \x01` \x81\x01\x90a(\xD7\x91\x90aG\x97V[`\x01`\x01`\xA0\x1B\x03\x16\x87\x87\x86\x81\x81\x10a(\xF2Wa(\xF2aN\xDCV[\x90P``\x02\x01`\0\x015\x7F\x90\x01~\x19\xB4\xF1[\xAB\x96\x828]E\xD8\x16\x94w(|Wb\xBA\x89u\x9F\xF8\xA7\x7FM8\xEC#\x84`@Qa)-\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16caA?\x0F\x87\x87\x86\x81\x81\x10a)vWa)vaN\xDCV[\x90P``\x02\x01` \x01` \x81\x01\x90a)\x8E\x91\x90aG\x97V[\x84Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81Ra)\xB4\x92\x91\x90\x86\x90`\x04\x01aO2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)\xCEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a)\xE2W=`\0\x80>=`\0\xFD[PP`\x01\x90\x94\x01\x93Pa'\x8C\x92PPPV[a\x01\xFET`\0\x90\x80a*\x07W`\0a\x16\xBBV[a\x16\xBB`\x01\x82aNgV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\r\xCEWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\r\xCEV[`\0\x81\x81Ra\x03\xFB` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a*\x86W`\0\x81\x81Ra\x03\xFB` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90UPV[`\0\x81\x81Ra\x03\xFB` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x163\x14a*\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FNot registered transmitter\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11\xA1V[PV[`\0`\0\x80Q` aV<\x839\x81Q\x91R\x82\x03a+\x0FWP`\x01a+(V[`\0\x80Q` aU\xDC\x839\x81Q\x91R\x82\x03a+(WP`\x10[`\0\x83\x81Ra\x03\xF6` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T\x81\x16\x15a+\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs!\xB7\xB6\xB862\xBA2\xB2\x10)\xBA\xB16\xB4\xB9\xB9\xB4\xB7\xB7`a\x1B`D\x82\x01R`d\x01a\x11\xA1V[\x83\x85\x10a+\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\t-\xCE\xCC-\x8D,\x84\r-\xCC\x8C\xAF`\x9B\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\0\x84\x11a,\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\t-\xCE\xCC-\x8D,\x84\r\x8C\xAD\xCC\xEE\x8D`\x93\x1B`D\x82\x01R`d\x01a\x11\xA1V[a\x01\xF4Ta,\x15a\x16wV[a,\x1F\x91\x90aO\x1FV[\x83\x10\x15a,nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FCooldown period not passed\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11\xA1V[B\x83\x11\x15a,\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x04\x96\xE7f\x16\xC6\x96B\x07F\x96\xD6W7F\x16\xD7`|\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\0\x83\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T\x85\x14a-\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\x9B\xDD\x08\x1AY\x1E\x15\x1B\xD4\xDDX\x9BZ]`\x8A\x1B`D\x82\x01R`d\x01a\x11\xA1V[PPPPPV[`\0\x87\x81Ra\x03\xFC` R`@\x90 \x80Ta-.\x90aP\xD0V[\x90P`\0\x03a-qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x12[XY\xD9H\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`\x8A\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\0\x86\x86\x86\x86\x86`@Q` \x01a-\x8C\x95\x94\x93\x92\x91\x90aQ\nV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a-\xB4\x91\x90aQ\x85V[\x91P\x91P`A\x82Q\x14a.\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FM:VP-Signature length mismatch\0\0`D\x82\x01R`d\x01a\x11\xA1V[\x82Q` \x84\x01 \x7F\x19Ethereum Signed Message:\n32\0\0\0\0`\0\x90\x81R`\x1C\x91\x90\x91R`<\x81 a.J\x90\x84a>yV[\x90P`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a.c\x91\x90aQ\xE2V[a\x01\xFCT`@Qc\xEA\xC7\x08\xA3`\xE0\x1B\x81R\x92\x94P\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xEA\xC7\x08\xA3\x90a.\x9A\x90\x85\x90\x85\x90`\x04\x01aS\x0BV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a.\xB2W`\0\x80\xFD[PZ\xFA\x15\x80\x15a.\xC6W=`\0\x80>=`\0\xFD[PPPP`\0a.\xD9\x82`\0\x01Qa>\xA3V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a/<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FM:VP-Enclave key mismatch\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11\xA1V[\x8Da/T\x83` \x01Q\x84`@\x01Q\x85``\x01Qa!\xEAV[\x14a/\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqM:VP-Invalid image`p\x1B`D\x82\x01R`d\x01a\x11\xA1V[PPPPPPPPPPPPPPV[`\0[\x81Q\x81\x10\x15a\x18\xECW`\0\x82\x82\x81Q\x81\x10a/\xC6Wa/\xC6aN\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q``\x81\x01\x80Q`\0\x88\x81Ra\x03\xF8\x85R`@\x80\x82 \x81\x86\x01\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x85R\x91\x88R\x82\x84 \x87\x89\x01Q\x83\x16\x85R\x88R\x82\x84 \x87Q\x83\x16\x85R\x88R\x82\x84 \x94\x90\x94U\x93Q\x8A\x83Ra\x03\xF7\x87R\x81\x83 \x93Q\x85\x16\x83R\x92\x86R\x80\x82 \x85Q\x90\x94\x16\x82R\x92\x90\x94R\x90\x83 \x80T\x92\x94P\x90\x92\x90\x91a0V\x90\x84\x90aO\x1FV[\x90\x91UPPa\x01\xFBT` \x82\x01Q\x82Q`@Qc\"\xD1\xFD\x8F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R\x91\x16\x90cE\xA3\xFB\x1E\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\xC5W=`\0\x80>=`\0\xFD[PP`\x01\x90\x93\x01\x92Pa/\xA9\x91PPV[`\0\x82\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x01T\x91\x81\x01\x82\x90R\x91\x03a1-W`\0\x83\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 `\x01\x01\x84\x90U[`\0\x83\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T`\x01\x92\x90a1X\x90\x84\x90aO\x1FV[\x90\x91UPPPPPPV[a1ka?\x02V[V[a1ua?\x02V[`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0UV[`\0`\0\x80Q` aV\\\x839\x81Q\x91Ra1\xB6\x84\x84a!\x91V[a26W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua1\xEC3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\r\xCEV[`\0\x91PPa\r\xCEV[a*\xED\x813a?KV[`\0a\"\x1A\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a?\x84V[`\0\x80a\x01\xF8T\x11a2\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTotal weight must be greater tha`D\x82\x01Ren zero`\xD0\x1B`d\x82\x01R`\x84\x01a\x11\xA1V[`\0a2\xCFa\x01\xF6a:gV[\x11a3\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrNo tokens available`h\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\0a3\x1Fa\x01\xF6a:gV[`\x01`\x01`@\x1B\x03\x81\x11\x15a36Wa36aH\x05V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a3_W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0a3oa\x01\xF6a:gV[`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x86Wa3\x86aH\x05V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a3\xAFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[Pa\x01\xF8T\x90\x91P`\0\x80a3\xC5a\x01\xF6a:gV[\x90P`\0[\x81\x81\x10\x15a4lW`\0a3\xE0a\x01\xF6\x83a:qV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x03\xF4` R`@\x90 T\x90\x91P\x80\x15a4bW\x81\x88\x86\x81Q\x81\x10a4\x16Wa4\x16aN\xDCV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80\x87\x86\x81Q\x81\x10a4IWa4IaN\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x84a4^\x81aS\x9FV[\x95PP[PP`\x01\x01a3\xCAV[P[`\0\x82\x11a4\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNo stakeToken available to lock\0`D\x82\x01R`d\x01a\x11\xA1V[`\0\x83Ba4\xCD`\x01CaNgV[@3`@Q` \x01a5\x04\x93\x92\x91\x90\x92\x83R` \x83\x01\x91\x90\x91R``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`@\x82\x01R`T\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca5'\x91\x90aS\xCEV[\x90P`\0\x80\x80[\x85\x81\x10\x15a5\x8EW\x87\x81\x81Q\x81\x10a5HWa5HaN\xDCV[` \x02` \x01\x01Q\x83a5[\x91\x90aO\x1FV[\x92P\x82\x84\x10\x15a5\x86W\x88\x81\x81Q\x81\x10a5wWa5waN\xDCV[` \x02` \x01\x01Q\x91Pa5\x8EV[`\x01\x01a5.V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03\xF3` R`@\x90 Ta5\xB2\x83\x8Da\x1F~V[\x10a5\xC5WP\x99\x98PPPPPPPPPV[\x87\x81\x81Q\x81\x10a5\xD7Wa5\xD7aN\xDCV[` \x02` \x01\x01Q\x87a5\xEA\x91\x90aNgV[\x96P\x88a5\xF8`\x01\x88aNgV[\x81Q\x81\x10a6\x08Wa6\x08aN\xDCV[` \x02` \x01\x01Q\x89\x82\x81Q\x81\x10a6\"Wa6\"aN\xDCV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x87a6F`\x01\x88aNgV[\x81Q\x81\x10a6VWa6VaN\xDCV[` \x02` \x01\x01Q\x88\x82\x81Q\x81\x10a6pWa6paN\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x85a6\x85\x81aS\xF0V[\x96PPPPPPa4nV[`\0`\0\x80Q` aV\\\x839\x81Q\x91Ra6\xAC\x84\x84a!\x91V[\x15a26W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\r\xCEV[`\0a7\x19a\x01\xF5T\x90V[`@\x80Q``\x81\x01\x82R\x84\x81R3` \x80\x83\x01\x82\x81R\x83\x85\x01\x86\x81Ra\x01\xFE\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x85Q\x7F\x82\xCA\xFEv\xA8\xC3\x8B;\x8CO'\xAC\xA5&\x15\x01\x96\x86Ro8>\xD1\xF7E4\xAA\x82\xF5\xBE\x9A\xDC`\x03\x90\x92\x02\x91\x82\x01U\x91Q\x7F\x82\xCA\xFEv\xA8\xC3\x8B;\x8CO'\xAC\xA5&\x15\x01\x96\x86Ro8>\xD1\xF7E4\xAA\x82\xF5\xBE\x9A\xDD\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UQ\x7F\x82\xCA\xFEv\xA8\xC3\x8B;\x8CO'\xAC\xA5&\x15\x01\x96\x86Ro8>\xD1\xF7E4\xAA\x82\xF5\xBE\x9A\xDE\x90\x91\x01U\x92Q\x86\x81R\x93\x94P\x90\x92\x90\x91\x7FG\xE4t;xF\xC6\xF1\x14s\xE5\xCC\xCAQ/\x02:\xA5\x08B&\xF7\xB5\x88<\xB7o\x11\xBFa1\x1A\x91\x01`@Q\x80\x91\x03\x90\xA2`@Qb\xADN\xCD`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x05jvh\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a8rW`\0\x80\xFD[PZ\xF1\x15\x80\x15a8\x86W=`\0\x80>=`\0\xFD[PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\"\x1AV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a98WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a9,`\0\x80Q` aU\xFC\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a1kW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B\x82\x81a2@V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a9\xBBWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra9\xB8\x91\x81\x01\x90aP\xB7V[`\x01[a9\xE3W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x11\xA1V[`\0\x80Q` aU\xFC\x839\x81Q\x91R\x81\x14a:\x14W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x11\xA1V[a\x18\xEC\x83\x83a@mV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a1kW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r\xCE\x82T\x90V[`\0a\"\x1A\x83\x83a@\xC3V[```\0a\"\x1A\x83a@\xEDV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x18\xEC\x90\x84\x90aAIV[`\0\x81\x81Ra\x03\xFC` R`@\x81 \x90a:\xF6\x82\x82aEfV[a;\x04`\x01\x83\x01`\0aEfV[a;\x12`\x02\x83\x01`\0aEfV[PP`@Q\x81\x90\x7F\xA4+w\x11\xE1\xED])\xC8\x046\x85Z\x8BG\xA1\xDC\xAAG\xAA\x13.G%\xC0\xEF\x1D\xFC\xE2<\xAA\xFC\x90`\0\x90\xA2PV[`\0a\"\x1A\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aA\xBAV[a\x01\xFC\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x1C^v\0\x7F\xD00\x93\xBC\x99\xEE\xB7%\xE2\x05\xF4/\xE5]\x8A\xE0@D ,\xF4\x0C\r[o\x8C\r\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a;\xC5\x91\x90aT\x07V[\x92P\x92P\x92P`\0a;\xD8\x84\x84\x84a!\xEAV[`\0\x81\x81Ra\x03\xFC` R`@\x90 \x80T\x91\x92P\x90a;\xF6\x90aP\xD0V[\x15\x90Pa<<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsImage already exists``\x1B`D\x82\x01R`d\x01a\x11\xA1V[\x83Q`0\x14a<\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\t-\xCE\xCC-\x8D,\x84\n\x08jF\x04\r\x8C\xAD\xCC\xEE\x8D`k\x1B`D\x82\x01R`d\x01a\x11\xA1V[\x82Q`0\x14a<\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\t-\xCE\xCC-\x8D,\x84\n\x08jF$\r\x8C\xAD\xCC\xEE\x8D`k\x1B`D\x82\x01R`d\x01a\x11\xA1V[\x81Q`0\x14a=\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\t-\xCE\xCC-\x8D,\x84\n\x08jFD\r\x8C\xAD\xCC\xEE\x8D`k\x1B`D\x82\x01R`d\x01a\x11\xA1V[`@\x80Q``\x81\x01\x82R\x85\x81R` \x80\x82\x01\x86\x90R\x81\x83\x01\x85\x90R`\0\x84\x81Ra\x03\xFC\x90\x91R\x91\x90\x91 \x81Q\x82\x91\x90\x81\x90a=L\x90\x82aT\xD5V[P` \x82\x01Q`\x01\x82\x01\x90a=a\x90\x82aT\xD5V[P`@\x82\x01Q`\x02\x82\x01\x90a=v\x90\x82aT\xD5V[P\x90PP\x81\x7Fg\x0E@\x8E.6r\x10\xFEHK\x84\x1F\xAD\xD6\xAFRmJg5`\x12\xF9\xC6GP\xE3\xF2\xB5\\\x04\x86\x86\x86`@Qa=\xAE\x93\x92\x91\x90aJ\xFEV[`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0\x83\x83\x02\x81`\0\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a=\xF5W\x83\x82\x81a=\xEBWa=\xEBaS\xB8V[\x04\x92PPPa\"\x1AV[\x80\x84\x11a>\x0CWa>\x0C`\x03\x85\x15\x02`\x11\x18aB\tV[`\0\x84\x86\x88\t`\0\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[`\0\x80`\0\x80a>\x89\x86\x86aB\x1BV[\x92P\x92P\x92Pa>\x99\x82\x82aBhV[P\x90\x94\x93PPPPV[`\0\x81Q`@\x14a>\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FM:VP-Invalid public key length\0\0`D\x82\x01R`d\x01a\x11\xA1V[P\x80Q` \x90\x91\x01 \x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a1kW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a?U\x82\x82a!\x91V[a\x1B\x82W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x11\xA1V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a26W`\0a?\xA8`\x01\x83aNgV[\x85T\x90\x91P`\0\x90a?\xBC\x90`\x01\x90aNgV[\x90P\x80\x82\x14a@!W`\0\x86`\0\x01\x82\x81T\x81\x10a?\xDCWa?\xDCaN\xDCV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a?\xFFWa?\xFFaN\xDCV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a@2Wa@2aU\x93V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\r\xCEV[a@v\x82aC!V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a@\xBBWa\x18\xEC\x82\x82aC\x86V[a\x1B\x82aC\xF3V[`\0\x82`\0\x01\x82\x81T\x81\x10a@\xDAWa@\xDAaN\xDCV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aA=W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aA)W[PPPPP\x90P\x91\x90PV[`\0\x80` `\0\x84Q` \x86\x01`\0\x88Z\xF1\x80aAlW`@Q=`\0\x82>=\x81\xFD[PP`\0Q=\x91P\x81\x15aA\x84W\x80`\x01\x14\x15aA\x91V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x16qW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x11\xA1V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 TaB\x01WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\r\xCEV[P`\0a\r\xCEV[cNH{q`\0R\x80` R`$`\x1C\xFD[`\0\x80`\0\x83Q`A\x03aBUW` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1AaBG\x88\x82\x85\x85aD\x12V[\x95P\x95P\x95PPPPaBaV[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15aB|WaB|aU\xA9V[\x03aB\x85WPPV[`\x01\x82`\x03\x81\x11\x15aB\x99WaB\x99aU\xA9V[\x03aB\xB7W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15aB\xCBWaB\xCBaU\xA9V[\x03aB\xECW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x11\xA1V[`\x03\x82`\x03\x81\x11\x15aC\0WaC\0aU\xA9V[\x03a\x1B\x82W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x11\xA1V[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03aCWW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x11\xA1V[`\0\x80Q` aU\xFC\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@QaC\xA3\x91\x90aU\xBFV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14aC\xDEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aC\xE3V[``\x91P[P\x91P\x91Pa\x1F\xCD\x85\x83\x83aD\xE1V[4\x15a1kW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15aDMWP`\0\x91P`\x03\x90P\x82aD\xD7V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aD\xA1W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aD\xCDWP`\0\x92P`\x01\x91P\x82\x90PaD\xD7V[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82aD\xF6WaD\xF1\x82aE=V[a\"\x1AV[\x81Q\x15\x80\x15aE\rWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15aE6W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x11\xA1V[P\x80a\"\x1AV[\x80Q\x15aEMW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80TaEr\x90aP\xD0V[`\0\x82U\x80`\x1F\x10aE\x82WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a*\xED\x91\x90[\x80\x82\x11\x15aE\xB0W`\0\x81U`\x01\x01aE\x9CV[P\x90V[`\0` \x82\x84\x03\x12\x15aE\xC6W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\"\x1AW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12aE\xF0W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x07W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aF\x1FW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aFBW`\0\x80\xFD[\x885\x97P` \x89\x015\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aFtW`\0\x80\xFD[aF\x80\x8B\x82\x8C\x01aE\xDEV[\x90\x95P\x93PP`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x9FW`\0\x80\xFD[aF\xAB\x8B\x82\x8C\x01aE\xDEV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a*\xEDW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aF\xE7W`\0\x80\xFD[\x825aF\xF2\x81aF\xBFV[\x91P` \x83\x015aG\x02\x81aF\xBFV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aG\x1FW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aG>W`\0\x80\xFD[\x855aGI\x81aF\xBFV[\x94P` \x86\x015aGY\x81aF\xBFV[\x93P`@\x86\x015aGi\x81aF\xBFV[\x92P``\x86\x015aGy\x81aF\xBFV[\x91P`\x80\x86\x015aG\x89\x81aF\xBFV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15aG\xA9W`\0\x80\xFD[\x815a\"\x1A\x81aF\xBFV[`\0\x80`@\x83\x85\x03\x12\x15aG\xC7W`\0\x80\xFD[\x825\x91P` \x83\x015aG\x02\x81aF\xBFV[`\0\x80`@\x83\x85\x03\x12\x15aG\xECW`\0\x80\xFD[\x825aG\xF7\x81aF\xBFV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH=WaH=aH\x05V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH=WaH=aH\x05V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH=WaH=aH\x05V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH\xAFWaH\xAFaH\x05V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aH\xD0WaH\xD0aH\x05V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aH\xEFW`\0\x80\xFD[\x815aI\x02aH\xFD\x82aH\xB7V[aH\x87V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aI\x17W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aIMW`\0\x80\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x7FW`\0\x80\xFD[aI\x8B\x89\x82\x8A\x01aH\xDEV[\x92PP`\xA0\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xA7W`\0\x80\xFD[aI\xB3\x89\x82\x8A\x01aH\xDEV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15aI\xD3W`\0\x80\xFD[\x825aI\xDE\x81aF\xBFV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xF9W`\0\x80\xFD[aJ\x05\x85\x82\x86\x01aH\xDEV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15aJJW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aJ#V[P\x93\x94\x93PPPPV[`@\x81R`\0aJg`@\x83\x01\x85aJ\x0FV[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x86\x01\x92P`\0[\x81\x81\x10\x15aJ\xA2W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aJ\x84V[P\x90\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15aJ\xC9W\x81\x81\x01Q\x83\x82\x01R` \x01aJ\xB1V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaJ\xEA\x81` \x86\x01` \x86\x01aJ\xAEV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81R`\0aK\x11``\x83\x01\x86aJ\xD2V[\x82\x81\x03` \x84\x01RaK#\x81\x86aJ\xD2V[\x90P\x82\x81\x03`@\x84\x01RaK7\x81\x85aJ\xD2V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aKSW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aKiW`\0\x80\xFD[aKu\x84\x82\x85\x01aH\xDEV[\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aK\x92W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xA8W`\0\x80\xFD[aK\xB4\x86\x82\x87\x01aH\xDEV[\x93PP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xD0W`\0\x80\xFD[aK\xDC\x86\x82\x87\x01aH\xDEV[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xF8W`\0\x80\xFD[aL\x04\x86\x82\x87\x01aH\xDEV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15aL#W`\0\x80\xFD[\x835aL.\x81aF\xBFV[\x92P` \x84\x015aL>\x81aF\xBFV[\x91P`@\x84\x015aLN\x81aF\xBFV[\x80\x91PP\x92P\x92P\x92V[` \x81R`\0a\"\x1A` \x83\x01\x84aJ\xD2V[`\0\x80`\0``\x84\x86\x03\x12\x15aL\x81W`\0\x80\xFD[\x835\x92P` \x84\x015aL\x93\x81aF\xBFV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[` \x81R`\0a\"\x1A` \x83\x01\x84aJ\x0FV[`\0\x80`@\x83\x85\x03\x12\x15aL\xCAW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80` \x83\x85\x03\x12\x15aL\xECW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x02W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aM\x13W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aM)W`\0\x80\xFD[\x85` ``\x83\x02\x84\x01\x01\x11\x15aM>W`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aMgWaMgaH\x05V[P`\x05\x1B` \x01\x90V[`\0` \x82\x84\x03\x12\x15aM\x83W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x99W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aM\xAAW`\0\x80\xFD[\x805aM\xB8aH\xFD\x82aMNV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x07\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aM\xDAW`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aK7W`\x80\x84\x88\x03\x12\x15aM\xF9W`\0\x80\xFD[aN\x01aH\x1BV[\x845aN\x0C\x81aF\xBFV[\x81R` \x85\x015aN\x1C\x81aF\xBFV[` \x82\x01R`@\x85\x015aN/\x81aF\xBFV[`@\x82\x01R``\x85\x81\x015\x90\x82\x01R\x82R`\x80\x90\x93\x01\x92` \x90\x91\x01\x90aM\xE1V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\r\xCEWa\r\xCEaNQV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x85\x81R``` \x82\x01R`\0aN\xBD``\x83\x01\x86\x88aNzV[\x82\x81\x03`@\x84\x01RaN\xD0\x81\x85\x87aNzV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`\x13\x90\x82\x01Rr'\xB76<\x90)\xBA0\xB5\xB4\xB73\xA6\xB0\xB70\xB3\xB2\xB9`i\x1B`@\x82\x01R``\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\r\xCEWa\r\xCEaNQV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15aOhW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO~W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aO\x8FW`\0\x80\xFD[\x80QaO\x9DaH\xFD\x82aMNV[\x80\x82\x82R` \x82\x01\x91P` ``\x84\x02\x85\x01\x01\x92P\x86\x83\x11\x15aO\xBFW`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aK7W``\x84\x88\x03\x12\x15aO\xDEW`\0\x80\xFD[aO\xE6aHCV[\x84Q\x81R` \x85\x01QaO\xF8\x81aF\xBFV[` \x82\x01R`@\x85\x01QaP\x0B\x81aF\xBFV[`@\x82\x01R\x82R``\x93\x90\x93\x01\x92` \x90\x91\x01\x90aO\xC6V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aP\x81W\x83Q\x80Q\x84R` \x80\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x87\x01R`@\x92\x83\x01Q\x16\x91\x85\x01\x91\x90\x91R\x90\x93\x01\x92``\x90\x92\x01\x91`\x01\x01aP>V[P\x90\x95\x94PPPPPV[\x83\x81R``` \x82\x01R`\0aP\xA5``\x83\x01\x85aJ\xD2V[\x82\x81\x03`@\x84\x01RaK7\x81\x85aJ\xD2V[`\0` \x82\x84\x03\x12\x15aP\xC9W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80aP\xE4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aQ\x04WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x85\x81R\x84` \x82\x01R\x83`@\x82\x01R\x82``\x82\x01R`\xA0`\x80\x82\x01R`\0aQ5`\xA0\x83\x01\x84aJ\xD2V[\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12aQQW`\0\x80\xFD[\x81QaQ_aH\xFD\x82aH\xB7V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aQtW`\0\x80\xFD[aKu\x82` \x83\x01` \x87\x01aJ\xAEV[`\0\x80`@\x83\x85\x03\x12\x15aQ\x98W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xAEW`\0\x80\xFD[aQ\xBA\x85\x82\x86\x01aQ@V[\x92PP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xD6W`\0\x80\xFD[aJ\x05\x85\x82\x86\x01aQ@V[`\0\x80`@\x83\x85\x03\x12\x15aQ\xF5W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x0BW`\0\x80\xFD[aR\x17\x85\x82\x86\x01aQ@V[\x92PP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR3W`\0\x80\xFD[\x83\x01`\xA0\x81\x86\x03\x12\x15aREW`\0\x80\xFD[aRMaHeV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aRcW`\0\x80\xFD[aRo\x87\x82\x85\x01aQ@V[\x82RP` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x8BW`\0\x80\xFD[aR\x97\x87\x82\x85\x01aQ@V[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xB6W`\0\x80\xFD[aR\xC2\x87\x82\x85\x01aQ@V[`@\x83\x01RP``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xE1W`\0\x80\xFD[aR\xED\x87\x82\x85\x01aQ@V[``\x83\x01RP`\x80\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R\x91\x94\x91\x93P\x90\x91PPV[`@\x81R`\0aS\x1E`@\x83\x01\x85aJ\xD2V[\x82\x81\x03` \x84\x01R\x83Q`\xA0\x82RaS9`\xA0\x83\x01\x82aJ\xD2V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01RaSR\x82\x82aJ\xD2V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01RaSl\x82\x82aJ\xD2V[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01RaS\x86\x82\x82aJ\xD2V[\x91PP`\x80\x85\x01Q`\x80\x83\x01R\x80\x92PPP\x93\x92PPPV[`\0`\x01\x82\x01aS\xB1WaS\xB1aNQV[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aS\xEBWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x81aS\xFFWaS\xFFaNQV[P`\0\x19\x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aT\x1CW`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15aT2W`\0\x80\xFD[aT>\x86\x82\x87\x01aQ@V[\x93PP` \x84\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aTZW`\0\x80\xFD[aTf\x86\x82\x87\x01aQ@V[\x92PP`@\x84\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aT\x82W`\0\x80\xFD[aL\x04\x86\x82\x87\x01aQ@V[`\x1F\x82\x11\x15a\x18\xECW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aT\xB5WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a-\rW`\0\x81U`\x01\x01aT\xC1V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xEEWaT\xEEaH\x05V[aU\x02\x81aT\xFC\x84TaP\xD0V[\x84aT\x8EV[` `\x1F\x82\x11`\x01\x81\x14aU6W`\0\x83\x15aU\x1EWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua-\rV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aUfW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aUFV[P\x84\x82\x10\x15aU\x84W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82QaU\xD1\x81\x84` \x87\x01aJ\xAEV[\x91\x90\x91\x01\x92\x91PPV\xFE\xC8\x987\xA6\xEA\x060\x8Dj/A\0}-\n\xF1\x8F\xC3L\x85\xAC!K\xC9_\x0FV\xC1\x91\xDB\x87\x076\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x86\xE4\x8C\xDEp\r\xD7\xFD\x18`6D\x94K\x13x|\xC9\xAEB\xD3J!\xF8\xBF\xB6\x9B\xC2\xEA\xB7\xED\xE1\x133\x02\x8F\xE2\xC6wG\xE7\xC1\xEF\nq\x1C\xBD\x02\x88\xF2\x9C\0Y\xF6k\x85`\xD4\xEE\\g\x92\xB4\xA3\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\xA2dipfsX\"\x12 \x83\xF3#\xAA\xCE\x9ChK\xCA(tX\\\x89\x96\x1C\xF9\xDD\xA1Z\\\xDDQ\xD1\x88\x11\x9F\xAB\xDE\x81.\xEAdsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static SYMBIOTICSTAKING_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x03\xEFW`\x005`\xE0\x1C\x80c\x81\xC4\\p\x11a\x02\x08W\x80c\xAD<\xB1\xCC\x11a\x01\x18W\x80c\xCB\x95qz\x11a\0\xABW\x80c\xD5Gt\x1F\x11a\0zW\x80c\xD5Gt\x1F\x14a\x0C\xF3W\x80c\xD6t\x8B\x02\x14a\r\x13W\x80c\xDE\xE8\xBC\xE2\x14a\rJW\x80c\xEB_+\x98\x14a\rjW\x80c\xEC\x9ASY\x14a\r\xAEW`\0\x80\xFD[\x80c\xCB\x95qz\x14a\x0C.W\x80c\xD1\x1F)<\x14a\x0C\\W\x80c\xD3e\xD2\xD6\x14a\x0C~W\x80c\xD4s\xE1\"\x14a\x0C\x9EW`\0\x80\xFD[\x80c\xB9,\"z\x11a\0\xE7W\x80c\xB9,\"z\x14a\x0B\xC2W\x80c\xC0\xFF\x97\xEF\x14a\x0B\xD9W\x80c\xC4\x0C\xC6\xE9\x14a\x0B\xF9W\x80c\xC9\x06\x1Aj\x14a\x0C\x19W`\0\x80\xFD[\x80c\xAD<\xB1\xCC\x14a\x0B.W\x80c\xB0\x0B\xBAj\x14a\x0BlW\x80c\xB1\xB3p\x07\x14a\x0B\x8CW\x80c\xB4\xB3\xC5\xA0\x14a\x0B\xA1W`\0\x80\xFD[\x80c\x9B\x88\xEE\x16\x11a\x01\x9BW\x80c\xA1\x80\x9B\x95\x11a\x01jW\x80c\xA1\x80\x9B\x95\x14a\n\xA1W\x80c\xA2\x17\xFD\xDF\x14a\n\xC1W\x80c\xA6\xFF\xE5'\x14a\n\xD6W\x80c\xACh\xD8w\x14a\n\xEDW\x80c\xAC\xC2\x16j\x14a\x0B\rW`\0\x80\xFD[\x80c\x9B\x88\xEE\x16\x14a\n(W\x80c\x9E:,\x9E\x14a\nJW\x80c\x9F=T^\x14a\njW\x80c\x9F\xCD\x11_\x14a\n\x81W`\0\x80\xFD[\x80c\x8D;\x9DZ\x11a\x01\xD7W\x80c\x8D;\x9DZ\x14a\t\x8BW\x80c\x8F\x9C0\x95\x14a\t\xB9W\x80c\x91\xD1HT\x14a\t\xE8W\x80c\x94]\x0FL\x14a\n\x08W`\0\x80\xFD[\x80c\x81\xC4\\p\x14a\t\nW\x80c\x81\xF7\xD9B\x14a\t+W\x80c\x82\xF6\xF6\x08\x14a\tKW\x80c\x89\xC7\xB9\x87\x14a\tkW`\0\x80\xFD[\x80c3\xF4\x90\x9F\x11a\x03\x03W\x80cQ\xC3\x9Cf\x11a\x02\x96W\x80c[\x1AL$\x11a\x02eW\x80c[\x1AL$\x14a\x08%W\x80c_A=8\x14a\x08\x85W\x80c`\xFB\xA4\x8B\x14a\x08\xA8W\x80cc\x82\xD9\xAD\x14a\x08\xCAW\x80c\x80u\xC2j\x14a\x08\xEAW`\0\x80\xFD[\x80cQ\xC3\x9Cf\x14a\x07\xBBW\x80cR\xD1\x90-\x14a\x07\xDBW\x80cT\x0B\xC5\xEA\x14a\x07\xF0W\x80cW$\xB8N\x14a\x08\x05W`\0\x80\xFD[\x80c=t\xA7x\x11a\x02\xD2W\x80c=t\xA7x\x14a\x07GW\x80c=\xF3m\xDF\x14a\x07hW\x80cN\x82\x11\x02\x14a\x07\x88W\x80cO\x1E\xF2\x86\x14a\x07\xA8W`\0\x80\xFD[\x80c3\xF4\x90\x9F\x14a\x06\xC5W\x80c6V\x8A\xBE\x14a\x06\xE5W\x80c6q\x96s\x14a\x07\x05W\x80c8\x11\x06\xD3\x14a\x07'W`\0\x80\xFD[\x80c\x16\xB7\x1E\xF4\x11a\x03\x86W\x80c\x1F=\xC4\xE2\x11a\x03UW\x80c\x1F=\xC4\xE2\x14a\x06/W\x80c\"\x82\x8C\xC2\x14a\x06OW\x80c$\x8A\x9C\xA3\x14a\x06pW\x80c//\xF1]\x14a\x06\x90W\x80c3\xDB$g\x14a\x06\xB0W`\0\x80\xFD[\x80c\x16\xB7\x1E\xF4\x14a\x05~W\x80c\x1B\xE8\xF4\x83\x14a\x05\xC1W\x80c\x1C\xCB=f\x14a\x05\xFAW\x80c\x1E\xF9\x17j\x14a\x06\x0FW`\0\x80\xFD[\x80c\x10\xC9\x99\xBB\x11a\x03\xC2W\x80c\x10\xC9\x99\xBB\x14a\x04\xD2W\x80c\x11\x0C\xBC\x80\x14a\x04\xF2W\x80c\x14YEz\x14a\x05>W\x80c\x16hf\xC7\x14a\x05^W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x03\xF4W\x80c\x08\xB2C\x18\x14a\x04)W\x80c\nY\xBBP\x14a\x04KW\x80c\x0E\x98\x8E%\x14a\x04\x92W[`\0\x80\xFD[4\x80\x15a\x04\0W`\0\x80\xFD[Pa\x04\x14a\x04\x0F6`\x04aE\xB4V[a\r\xC3V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x045W`\0\x80\xFD[Pa\x04Ia\x04D6`\x04aF&V[a\r\xD4V[\0[4\x80\x15a\x04WW`\0\x80\xFD[Pa\x04\x84a\x04f6`\x04aF\xD4V[a\x03\xFA` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x04 V[4\x80\x15a\x04\x9EW`\0\x80\xFD[Pa\x04\xA7a\x0F9V[`@\x80Q\x82Q\x81R` \x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01a\x04 V[4\x80\x15a\x04\xDEW`\0\x80\xFD[Pa\x04\xA7a\x04\xED6`\x04aG\rV[a\x0F\xC9V[4\x80\x15a\x04\xFEW`\0\x80\xFD[Pa\x05&\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04 V[4\x80\x15a\x05JW`\0\x80\xFD[Pa\x04Ia\x05Y6`\x04aG&V[a\x10SV[4\x80\x15a\x05jW`\0\x80\xFD[Pa\x04Ia\x05y6`\x04aG\x97V[a\x14jV[4\x80\x15a\x05\x8AW`\0\x80\xFD[Pa\x05\x9Ea\x05\x996`\x04aG\rV[a\x15=V[`@\x80Q\x93\x84R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\x04 V[4\x80\x15a\x05\xCDW`\0\x80\xFD[Pa\x04\x84a\x05\xDC6`\x04aG\xB4V[a\x03\xF6` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x06\x06W`\0\x80\xFD[Pa\x04\x84`\x11\x81V[4\x80\x15a\x06\x1BW`\0\x80\xFD[Pa\x04Ia\x06*6`\x04aG\x97V[a\x15}V[4\x80\x15a\x06;W`\0\x80\xFD[Pa\x04Ia\x06J6`\x04aG\xD9V[a\x15\xDFV[4\x80\x15a\x06[W`\0\x80\xFD[Pa\x01\xF9Ta\x05&\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06|W`\0\x80\xFD[Pa\x04\x84a\x06\x8B6`\x04aG\rV[a\x163V[4\x80\x15a\x06\x9CW`\0\x80\xFD[Pa\x04Ia\x06\xAB6`\x04aG\xB4V[a\x16UV[4\x80\x15a\x06\xBCW`\0\x80\xFD[Pa\x04\x84a\x16wV[4\x80\x15a\x06\xD1W`\0\x80\xFD[Pa\x04Ia\x06\xE06`\x04aG\xB4V[a\x16\xC1V[4\x80\x15a\x06\xF1W`\0\x80\xFD[Pa\x04Ia\x07\x006`\x04aG\xB4V[a\x18\xB9V[4\x80\x15a\x07\x11W`\0\x80\xFD[Pa\x04\x84`\0\x80Q` aV\x1C\x839\x81Q\x91R\x81V[4\x80\x15a\x073W`\0\x80\xFD[Pa\x04Ia\x07B6`\x04aI4V[a\x18\xF1V[4\x80\x15a\x07SW`\0\x80\xFD[Pa\x01\xFDTa\x05&\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07tW`\0\x80\xFD[Pa\x04Ia\x07\x836`\x04aG\x97V[a\x1B\x02V[4\x80\x15a\x07\x94W`\0\x80\xFD[Pa\x04\x14a\x07\xA36`\x04aG\x97V[a\x1BYV[a\x04Ia\x07\xB66`\x04aI\xC0V[a\x1BgV[4\x80\x15a\x07\xC7W`\0\x80\xFD[Pa\x04Ia\x07\xD66`\x04aG\rV[a\x1B\x86V[4\x80\x15a\x07\xE7W`\0\x80\xFD[Pa\x04\x84a\x1B\xC7V[4\x80\x15a\x07\xFCW`\0\x80\xFD[Pa\x04\x84`A\x81V[4\x80\x15a\x08\x11W`\0\x80\xFD[Pa\x04Ia\x08 6`\x04aG\xD9V[a\x1B\xE4V[4\x80\x15a\x081W`\0\x80\xFD[Pa\x08fa\x08@6`\x04aG\rV[a\x03\xF9` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x04 V[4\x80\x15a\x08\x91W`\0\x80\xFD[Pa\x08\x9Aa\x1C\x91V[`@Qa\x04 \x92\x91\x90aJTV[4\x80\x15a\x08\xB4W`\0\x80\xFD[Pa\x04\x84`\0\x80Q` aV<\x839\x81Q\x91R\x81V[4\x80\x15a\x08\xD6W`\0\x80\xFD[Pa\x04Ia\x08\xE56`\x04aF\xD4V[a\x1DdV[4\x80\x15a\x08\xF6W`\0\x80\xFD[Pa\x04Ia\t\x056`\x04aG\rV[a\x1E\x81V[4\x80\x15a\t\x16W`\0\x80\xFD[Pa\x01\xFATa\x05&\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\t7W`\0\x80\xFD[Pa\x04Ia\tF6`\x04aG\xD9V[a\x1E\xA2V[4\x80\x15a\tWW`\0\x80\xFD[Pa\x04Ia\tf6`\x04aG\x97V[a\x1F]V[4\x80\x15a\twW`\0\x80\xFD[Pa\x04\x84a\t\x866`\x04aF\xD4V[a\x1F~V[4\x80\x15a\t\x97W`\0\x80\xFD[Pa\x04\x84a\t\xA66`\x04aG\x97V[a\x03\xF3` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\t\xC5W`\0\x80\xFD[Pa\t\xD9a\t\xD46`\x04aG\rV[a\x1F\xD6V[`@Qa\x04 \x93\x92\x91\x90aJ\xFEV[4\x80\x15a\t\xF4W`\0\x80\xFD[Pa\x04\x14a\n\x036`\x04aG\xB4V[a!\x91V[4\x80\x15a\n\x14W`\0\x80\xFD[Pa\x04Ia\n#6`\x04aKAV[a!\xC9V[4\x80\x15a\n4W`\0\x80\xFD[Pa\x04\x84`\0\x80Q` aU\xDC\x839\x81Q\x91R\x81V[4\x80\x15a\nVW`\0\x80\xFD[Pa\x04\x84a\ne6`\x04aK}V[a!\xEAV[4\x80\x15a\nvW`\0\x80\xFD[Pa\x04\x84a\x01\xF4T\x81V[4\x80\x15a\n\x8DW`\0\x80\xFD[Pa\x04\x84a\n\x9C6`\x04aL\x0EV[a\"!V[4\x80\x15a\n\xADW`\0\x80\xFD[Pa\x04Ia\n\xBC6`\x04aG\x97V[a\"rV[4\x80\x15a\n\xCDW`\0\x80\xFD[Pa\x04\x84`\0\x81V[4\x80\x15a\n\xE2W`\0\x80\xFD[Pa\x04\x84a\x01\xF8T\x81V[4\x80\x15a\n\xF9W`\0\x80\xFD[Pa\x04Ia\x0B\x086`\x04aK}V[a\"\xCCV[4\x80\x15a\x0B\x19W`\0\x80\xFD[Pa\x01\xFBTa\x05&\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x0B:W`\0\x80\xFD[Pa\x0B_`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x04 \x91\x90aLYV[4\x80\x15a\x0BxW`\0\x80\xFD[Pa\x04Ia\x0B\x876`\x04aG\x97V[a#\x10V[4\x80\x15a\x0B\x98W`\0\x80\xFD[Pa\x04\x84`\x01\x81V[4\x80\x15a\x0B\xADW`\0\x80\xFD[Pa\x01\xFCTa\x05&\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x0B\xCEW`\0\x80\xFD[Pa\x04\x84a\x01\xF5T\x81V[4\x80\x15a\x0B\xE5W`\0\x80\xFD[Pa\x04Ia\x0B\xF46`\x04aLlV[a#gV[4\x80\x15a\x0C\x05W`\0\x80\xFD[Pa\x04Ia\x0C\x146`\x04aG\rV[a&XV[4\x80\x15a\x0C%W`\0\x80\xFD[Pa\x04\x84`\x10\x81V[4\x80\x15a\x0C:W`\0\x80\xFD[Pa\x04\x84a\x0CI6`\x04aG\x97V[a\x03\xF4` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x0ChW`\0\x80\xFD[Pa\x0Cqa&\xE9V[`@Qa\x04 \x91\x90aL\xA4V[4\x80\x15a\x0C\x8AW`\0\x80\xFD[Pa\x04\x84a\x0C\x996`\x04aF\xD4V[a&\xFBV[4\x80\x15a\x0C\xAAW`\0\x80\xFD[Pa\x0C\xDEa\x0C\xB96`\x04aL\xB7V[a\x03\xF5` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x04 V[4\x80\x15a\x0C\xFFW`\0\x80\xFD[Pa\x04Ia\r\x0E6`\x04aG\xB4V[a'AV[4\x80\x15a\r\x1FW`\0\x80\xFD[Pa\x05&a\r.6`\x04aG\rV[a\x03\xFB` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\rVW`\0\x80\xFD[Pa\x04Ia\re6`\x04aL\xD9V[a']V[4\x80\x15a\rvW`\0\x80\xFD[Pa\x04\x84a\r\x856`\x04aG\xB4V[`\0\x91\x82Ra\x03\xF6` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[4\x80\x15a\r\xBAW`\0\x80\xFD[Pa\x04\x84a)\xF4V[`\0a\r\xCE\x82a*\x12V[\x92\x91PPV[`\0a\r\xE2\x84\x86\x01\x86aMqV[\x90Pa\r\xED\x87a*GV[a\x0E\x07\x89\x89\x89`\0\x80Q` aV<\x839\x81Q\x91Ra*\xF0V[a\x0E\x8D\x86`\0\x80Q` aV<\x839\x81Q\x91R\x8B\x8B\x8B\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8E\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8C\x81R\x92P\x8C\x91P\x8B\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa-\x14\x92PPPV[a\x0E\x97\x87\x82a/\xA6V[a\x0E\xB0\x88\x88`\0\x80Q` aV<\x839\x81Q\x91Ra0\xD6V[a\x0E\xBB`\x01\x89aNgV[\x89\x03a\x0E\xE3W`\0\x87\x81Ra\x03\xF6` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 \x80T`\x01\x17\x90U[\x85\x893`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAC\x08E\x01\xA5a>\xD4\xB57rV\x86\xA6\xB9\xF9x\xEC\x9B\xB4a\xCA\x9743\xD1\x9B#]\x03\x86F\x8B\x89\x89\x89\x89`@Qa\x0F&\x95\x94\x93\x92\x91\x90aN\xA3V[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPV[a\x0Ff`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[a\x01\xFEa\x0Fqa)\xF4V[\x81T\x81\x10a\x0F\x81Wa\x0F\x81aN\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R`\x03\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x93\x83\x01\x93\x90\x93R`\x02\x90\x92\x01T\x91\x81\x01\x91\x90\x91R\x91\x90PV[a\x0F\xF6`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[a\x01\xFE\x82\x81T\x81\x10a\x10\nWa\x10\naN\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R`\x03\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x93\x83\x01\x93\x90\x93R`\x02\x90\x92\x01T\x91\x81\x01\x91\x90\x91R\x92\x91PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\x10\x98WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x10\xB4WP0;\x15[\x90P\x81\x15\x80\x15a\x10\xC2WP\x80\x15[\x15a\x10\xE0W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x11\nW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x11\x12a1cV[a\x11\x1Aa1cV[a\x11\"a1cV[a\x11*a1cV[a\x112a1mV[a\x11=`\0\x8Ba1\x9BV[P`\x01`\x01`\xA0\x1B\x03\x88\x16a\x11\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSymbioticStaking: stakingManager`D\x82\x01Rg is zero`\xC0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xF9\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB9\xE8\x9D\x19:q\x1D\xD2\x9A%3(\xFA\xAEn\xF5;\xD4\x1F\x10\x01sDl\x82 \xA3\x8A\x02.$\x0C\x90`\0\x90\xA2`\x01`\x01`\xA0\x1B\x03\x89\x16a\x12[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSymbioticStaking: proofMarketpla`D\x82\x01Rice is zero`\xB0\x1B`d\x82\x01R`\x84\x01a\x11\xA1V[a\x01\xFA\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x89 \xA3\x12\x98y\xB1\xE5L\x92Tjt\x8BbI\xBA/\x9A\x03\x83\xF4I\x9D\x9FJ\xE4;\x90\x9A\xD7\x95\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x87\x16a\x13\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FSymbioticStaking: rewardDistribu`D\x82\x01Rjtor is zero`\xA8\x1B`d\x82\x01R`\x84\x01a\x11\xA1V[a\x01\xFB\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x07\\\x02\xC5\x13\xA4\x15\xBDO\xF5\x97o\x8A\xA6\xFCWg\xD2\x18=\xAC\xA9\xEC\0\xABq\xCEx\xE8\xBF\x81X\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x86\x16a\x13\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSymbioticStaking: feeRewardToken`D\x82\x01Rg is zero`\xC0\x1B`d\x82\x01R`\x84\x01a\x11\xA1V[a\x01\xFD\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x90\x81\x17\x90\x91U`@Q\x7F_\x8E+\x8B8\xD2\x0EPBt\xAB\xF2\x84z\xD3/Jm\x03\xB3\xCCu\xD0\x18\xFBW\x11Y9u\x96\xC7\x90`\0\x90\xA2\x83\x15a\x14^W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPV[`\0a\x14u\x81a2@V[a\x14\x81a\x01\xF6\x83a2JV[a\x14\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x1B\xDA\xD9[\x88\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`b\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03\xF4` R`@\x81 Ta\x01\xF8\x80T\x91\x92\x90\x91a\x14\xF2\x90\x84\x90aNgV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x03\xF4` R`@\x80\x82 \x82\x90UQ\x7FzP/\xFF\xACP\xF2\xA88\xC9U\x04M\xECh\xB2rd\x17B\xA6\x86\x17\xAB\xE1nI\x82\xAA\xC3yu\x91\x90\xA2PPV[a\x01\xFE\x81\x81T\x81\x10a\x15NW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83V[`\0a\x15\x88\x81a2@V[a\x01\xFA\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x89 \xA3\x12\x98y\xB1\xE5L\x92Tjt\x8BbI\xBA/\x9A\x03\x83\xF4I\x9D\x9FJ\xE4;\x90\x9A\xD7\x95\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[`\0a\x15\xEA\x81a2@V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03\xF3` R`@\x80\x82 \x85\x90UQ\x84\x92\x91\x7F\xC3`\x93\x99-6\x17;\xC34\x83\xBA\xE4N\xEC\xC3\x90?\x841\xE5\xC0\x8A\x07\xC1\x94Z\x89f\x133\x83\x91\xA3PPPV[`\0\x90\x81R`\0\x80Q` aV\\\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[a\x16^\x82a\x163V[a\x16g\x81a2@V[a\x16q\x83\x83a1\x9BV[PPPPV[a\x01\xFET`\0\x90\x80a\x16\x8AW`\0a\x16\xBBV[a\x01\xFEa\x16\x98`\x01\x83aNgV[\x81T\x81\x10a\x16\xA8Wa\x16\xA8aN\xDCV[\x90`\0R` `\0 \x90`\x03\x02\x01`\0\x01T[\x91PP\x90V[a\x01\xF9T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x11\xA1\x90aN\xF2V[`\0a\x16\xF7\x82a2_V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x03\xF3` R`@\x90 T\x90\x91P\x80a\x17\x1F\x83\x85a\x1F~V[\x10\x15a\x17mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInsufficient stake amount\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11\xA1V[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x80\x83R` \x80\x84\x01\x86\x81R`\0\x8A\x81Ra\x03\xF9\x83R\x86\x81 \x95Q\x86T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x86\x16\x17\x86U\x90Q`\x01\x90\x95\x01\x94\x90\x94U\x90\x83Ra\x03\xFA\x81R\x83\x83 \x91\x87\x16\x83RR\x90\x81 \x80T\x83\x92\x90a\x17\xDC\x90\x84\x90aO\x1FV[\x92PP\x81\x90UP\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7FV\xAC\xD9\xAAF\x83\x1C\xF3\xE8\x03\xC8\x9A`\xEE\xCA\"\xE7\r\x8D\xD3#;\xCB\x1F\x9EH\x9394\x0C\xA0\x01\x84`@Qa\x18)\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4`@Qc \x1A\x11\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c \x1A\x11\xB3\x90a\x18\x81\x90\x86\x90\x86\x90\x86\x90`\x04\x01aO2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\xAFW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x18\xE2W`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xEC\x82\x82a6\x91V[PPPV[\x81Q``\x90\x15a\x19\x12W\x82\x80` \x01\x90Q\x81\x01\x90a\x19\x0F\x91\x90aOVV[\x90P[`\0\x85\x81Ra\x03\xF6` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\x01\x90\x81\x16\x14a\x19\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FVault Snapshot not submitted\0\0\0\0`D\x82\x01R`d\x01a\x11\xA1V[a\x19\x88\x85a*GV[a\x19\xA2\x87\x87\x87`\0\x80Q` aU\xDC\x839\x81Q\x91Ra*\xF0V[a\x19\xBF\x84`\0\x80Q` aU\xDC\x839\x81Q\x91R\x89\x89\x89\x88\x88a-\x14V[a\x19\xD8\x86\x86`\0\x80Q` aU\xDC\x839\x81Q\x91Ra0\xD6V[`\0\x85\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 `\0\x80Q` aV<\x839\x81Q\x91R\x84R\x82R\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x81Q\x15a\x1A\x80Wa\x01\xF9T`@QcM\"\xFD1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cM\"\xFD1\x90a\x1AM\x90\x85\x90`\x04\x01aP$V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1AgW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A{W=`\0\x80>=`\0\xFD[PPPP[\x84\x883`\x01`\x01`\xA0\x1B\x03\x16\x7F1\x9C\xE6\r\x9E@\xF3\xA6\x8C\xC1a\xE6\x19\x1Dm-\x8F^l\xA3w\xC8\xBB\x86\xC4\xE2]r\x7F\xE8M\x08\x8A\x88\x88`@Qa\x1A\xBF\x93\x92\x91\x90aP\x8CV[`@Q\x80\x91\x03\x90\xA4a\x1A\xD2`\x01\x88aNgV[\x88\x03a\x18\xAFW`\0\x86\x81Ra\x03\xF6` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 \x80T`\x01\x17\x90Ua\x18\xAF\x86a7\rV[`\0a\x1B\r\x81a2@V[a\x01\xFD\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F_\x8E+\x8B8\xD2\x0EPBt\xAB\xF2\x84z\xD3/Jm\x03\xB3\xCCu\xD0\x18\xFBW\x11Y9u\x96\xC7\x90`\0\x90\xA2PPV[`\0a\r\xCEa\x01\xF6\x83a8\x8FV[a\x1Boa8\xB1V[a\x1Bx\x82a9VV[a\x1B\x82\x82\x82a9aV[PPV[`\0a\x1B\x91\x81a2@V[a\x01\xF4\x82\x90U`@Q\x82\x81R\x7FW\xAD\xFC\xAD\x07\xDD\xCC;\xA3\xEF;\x02\x12\xB8\xAEy\x19q\xF5q\x8FM\xB4\xEClN\x19>\xE1\xFDG\0\x90` \x01a\x15\xD3V[`\0a\x1B\xD1a:\x1EV[P`\0\x80Q` aU\xFC\x839\x81Q\x91R\x90V[`\0a\x1B\xEF\x81a2@V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\xF4` R`@\x81 Ta\x01\xF8\x80T\x91\x92\x90\x91a\x1C\x1D\x90\x84\x90aNgV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\xF4` R`@\x81 \x83\x90Ua\x01\xF8\x80T\x84\x92\x90a\x1CQ\x90\x84\x90aO\x1FV[\x90\x91UPP`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F0#5\xD1>\t\xEC\xAF\xF1\xF0\xAE\xD0\xB1\x9BZ\xD3\xA0T\t>\xA0qZ6\xB6\xA4\xBC\"\x9BR\x95\x82\x90`\0\x90\xA3PPPV[``\x80`\0a\x1C\xA1a\x01\xF6a:gV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xB8Wa\x1C\xB8aH\x05V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C\xE1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[a\x1C\xF2a\x01\xF6a:gV[\x81\x10\x15a\x1DPWa\x03\xF4`\0a\x1D\na\x01\xF6\x84a:qV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x1D=Wa\x1D=aN\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1C\xE7V[Pa\x1D\\a\x01\xF6a:}V[\x93\x90\x92P\x90PV[`\0a\x1Do\x81a2@V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x1D\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqzero token address`p\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1E\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rnzero to address`\x88\x1B`D\x82\x01R`d\x01a\x11\xA1V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x18\xEC\x90\x83\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ELW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ep\x91\x90aP\xB7V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x91\x90a:\x8AV[`\0\x80Q` aV\x1C\x839\x81Q\x91Ra\x1E\x99\x81a2@V[a\x1B\x82\x82a:\xDCV[`\0a\x1E\xAD\x81a2@V[a\x1E\xB9a\x01\xF6\x84a;BV[a\x1E\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsToken already exists``\x1B`D\x82\x01R`d\x01a\x11\xA1V[\x81a\x01\xF8`\0\x82\x82Ta\x1F\x0F\x91\x90aO\x1FV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03\xF4` R`@\x80\x82 \x85\x90UQ\x84\x92\x91\x7F\x0C\x96U\xA8\r\xB9\xD3N#\xDD\x9DH<\xB2\xB6\x84\xE4v\x11T\xA6o\x93]\x9BB\xC1t\x89-\xD3%\x91\xA3PPPV[`\0\x80Q` aV\x1C\x839\x81Q\x91Ra\x1Fu\x81a2@V[a\x1B\x82\x82a;WV[`\0\x80a\x1F\x8B\x84\x84a&\xFBV[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81Ra\x03\xFA` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T\x90\x91P\x80\x82\x11a\x1F\xC3W`\0a\x1F\xCDV[a\x1F\xCD\x81\x83aNgV[\x95\x94PPPPPV[a\x03\xFC` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x1F\xF2\x90aP\xD0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \x1E\x90aP\xD0V[\x80\x15a kW\x80`\x1F\x10a @Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a kV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a NW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta \x80\x90aP\xD0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \xAC\x90aP\xD0V[\x80\x15a \xF9W\x80`\x1F\x10a \xCEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a \xF9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a \xDCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta!\x0E\x90aP\xD0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!:\x90aP\xD0V[\x80\x15a!\x87W\x80`\x1F\x10a!\\Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\x87V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!jW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[`\0\x91\x82R`\0\x80Q` aV\\\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x80Q` aV\x1C\x839\x81Q\x91Ra!\xE1\x81a2@V[a\x1B\x82\x82a;\xACV[`\0\x83\x83\x83`@Q` \x01a\"\x01\x93\x92\x91\x90aJ\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x93\x92PPPV[`\0a\x03\xF8`\0a\"0a\x16wV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x82R\x83R\x81\x81 \x95\x87\x16\x81R\x94\x82R\x80\x85 \x93\x90\x95\x16\x84R\x91\x90\x91RP T\x90V[`\0a\"}\x81a2@V[a\x01\xFB\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x07\\\x02\xC5\x13\xA4\x15\xBDO\xF5\x97o\x8A\xA6\xFCWg\xD2\x18=\xAC\xA9\xEC\0\xABq\xCEx\xE8\xBF\x81X\x90` \x01a\x15\xD3V[`\0\x80Q` aV\x1C\x839\x81Q\x91Ra\"\xE4\x81a2@V[a\x16q\x84\x84\x84`@Q` \x01a\"\xFC\x93\x92\x91\x90aJ\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra;\xACV[`\0a#\x1B\x81a2@V[a\x01\xF9\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB9\xE8\x9D\x19:q\x1D\xD2\x9A%3(\xFA\xAEn\xF5;\xD4\x1F\x10\x01sDl\x82 \xA3\x8A\x02.$\x0C\x90`\0\x90\xA2PPV[a\x01\xF9T`\x01`\x01`\xA0\x1B\x03\x163\x14a#\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x11\xA1\x90aN\xF2V[`\0\x83\x81Ra\x03\xF9` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x01T\x90\x82\x01R\x81\x15a%*W`\0a#\xD2a)\xF4V[\x90P`\0a$\x0E\x84a\x01\xFE\x84\x81T\x81\x10a#\xEEWa#\xEEaN\xDCV[\x90`\0R` `\0 \x90`\x03\x02\x01`\x02\x01Tg\r\xE0\xB6\xB3\xA7d\0\0a=\xBEV[\x90P`\0a$\x1C\x82\x86aNgV[a\x01\xFATa\x01\xFE\x80T\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC0\x88y)\x91\x90\x86\x90\x81\x10a$KWa$KaN\xDCV[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01`\x01\x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a$\xA7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a$\xBBW=`\0\x80>=`\0\xFD[PPa\x01\xFBT\x86Q`@Qc\x01|\xD4\xB5`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93Pc/\x9A\x96\xA0\x92Pa$\xF4\x91\x8A\x90\x86\x90`\x04\x01aO2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a%\x0EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a%\"W=`\0\x80>=`\0\xFD[PPPPPPP[`\0\x84\x81Ra\x03\xF9` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x01\x83\x90U\x83Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84Ra\x03\xF3\x83R\x81\x84 T\x85Q\x82\x16\x85Ra\x03\xFA\x84R\x82\x85 \x91\x88\x16\x85R\x92R\x82 \x80T\x91\x92\x90\x91a%\x90\x90\x84\x90aNgV[\x90\x91UPP\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x81\x81Ra\x03\xF3` R`@\x90\x81\x90 T\x90Q\x91\x92\x86\x16\x91\x87\x91\x7F\xB8\xECr\xF7\xC3EH\xFB\xB7\xD0\xE8\x8D\x93\xF2\xAA\x1E\x8C7\x11\x17y\x86\xB4\xD5\xBC\r\xAB\xF5\x86\x8C\xA1\xAB\x91a%\xEB\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x03\xF3` R`@\x90\x81\x90 T\x90Qc\x18\xDA\x93]`\xE1\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x92c1\xB5&\xBA\x92a\x18\x81\x92\x88\x92\x91`\x04\x01aO2V[`\0a&c\x81a2@V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x10a&\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInvalid comission rate`P\x1B`D\x82\x01R`d\x01a\x11\xA1V[a\x01\xF5\x82\x90U`@Q\x82\x81R\x7F\x18a\x8A!\xCB\xFD\xD5\x91ki\xF4\xC9-\x1B\xBC\x98\xF8\xC0\xE1J\x85\xDD\x9CN\xF8\x90\xBD\x15\xD3I\xD1\xE5\x90` \x01a\x15\xD3V[``a&\xF6a\x01\xF6a:}V[\x90P\x90V[`\0a\x03\xF7`\0a'\na\x16wV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x82R\x83R\x81\x81 \x94\x90\x95\x16\x85R\x92\x90RP\x90 T\x90V[a'J\x82a\x163V[a'S\x81a2@V[a\x16q\x83\x83a6\x91V[a\x01\xF9T`\x01`\x01`\xA0\x1B\x03\x163\x14a'\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x11\xA1\x90aN\xF2V[\x80`\0[\x81\x81\x10\x15a\x16qW`\0a\x03\xF9`\0\x86\x86\x85\x81\x81\x10a'\xADWa'\xADaN\xDCV[``\x02\x91\x90\x91\x015\x82RP` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x82Q\x80\x84\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x80\x82R`\x01\x90\x92\x01T\x81\x86\x01\x81\x90R\x91\x83Ra\x03\xFA\x90\x94R\x91\x81 \x92\x93P\x90\x91\x82\x91\x88\x88\x87\x81\x81\x10a(\x11Wa(\x11aN\xDCV[\x90P``\x02\x01` \x01` \x81\x01\x90a()\x91\x90aG\x97V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta(X\x91\x90aNgV[\x90\x91UPa\x03\xF9\x90P`\0\x87\x87\x86\x81\x81\x10a(uWa(uaN\xDCV[``\x02\x91\x90\x91\x015\x82RP` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x01U\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85\x81\x81\x10a(\xBFWa(\xBFaN\xDCV[\x90P``\x02\x01` \x01` \x81\x01\x90a(\xD7\x91\x90aG\x97V[`\x01`\x01`\xA0\x1B\x03\x16\x87\x87\x86\x81\x81\x10a(\xF2Wa(\xF2aN\xDCV[\x90P``\x02\x01`\0\x015\x7F\x90\x01~\x19\xB4\xF1[\xAB\x96\x828]E\xD8\x16\x94w(|Wb\xBA\x89u\x9F\xF8\xA7\x7FM8\xEC#\x84`@Qa)-\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16caA?\x0F\x87\x87\x86\x81\x81\x10a)vWa)vaN\xDCV[\x90P``\x02\x01` \x01` \x81\x01\x90a)\x8E\x91\x90aG\x97V[\x84Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81Ra)\xB4\x92\x91\x90\x86\x90`\x04\x01aO2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)\xCEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a)\xE2W=`\0\x80>=`\0\xFD[PP`\x01\x90\x94\x01\x93Pa'\x8C\x92PPPV[a\x01\xFET`\0\x90\x80a*\x07W`\0a\x16\xBBV[a\x16\xBB`\x01\x82aNgV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\r\xCEWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\r\xCEV[`\0\x81\x81Ra\x03\xFB` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a*\x86W`\0\x81\x81Ra\x03\xFB` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90UPV[`\0\x81\x81Ra\x03\xFB` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x163\x14a*\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FNot registered transmitter\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11\xA1V[PV[`\0`\0\x80Q` aV<\x839\x81Q\x91R\x82\x03a+\x0FWP`\x01a+(V[`\0\x80Q` aU\xDC\x839\x81Q\x91R\x82\x03a+(WP`\x10[`\0\x83\x81Ra\x03\xF6` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T\x81\x16\x15a+\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs!\xB7\xB6\xB862\xBA2\xB2\x10)\xBA\xB16\xB4\xB9\xB9\xB4\xB7\xB7`a\x1B`D\x82\x01R`d\x01a\x11\xA1V[\x83\x85\x10a+\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\t-\xCE\xCC-\x8D,\x84\r-\xCC\x8C\xAF`\x9B\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\0\x84\x11a,\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\t-\xCE\xCC-\x8D,\x84\r\x8C\xAD\xCC\xEE\x8D`\x93\x1B`D\x82\x01R`d\x01a\x11\xA1V[a\x01\xF4Ta,\x15a\x16wV[a,\x1F\x91\x90aO\x1FV[\x83\x10\x15a,nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FCooldown period not passed\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11\xA1V[B\x83\x11\x15a,\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x04\x96\xE7f\x16\xC6\x96B\x07F\x96\xD6W7F\x16\xD7`|\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\0\x83\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T\x85\x14a-\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\x9B\xDD\x08\x1AY\x1E\x15\x1B\xD4\xDDX\x9BZ]`\x8A\x1B`D\x82\x01R`d\x01a\x11\xA1V[PPPPPV[`\0\x87\x81Ra\x03\xFC` R`@\x90 \x80Ta-.\x90aP\xD0V[\x90P`\0\x03a-qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x12[XY\xD9H\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`\x8A\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\0\x86\x86\x86\x86\x86`@Q` \x01a-\x8C\x95\x94\x93\x92\x91\x90aQ\nV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a-\xB4\x91\x90aQ\x85V[\x91P\x91P`A\x82Q\x14a.\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FM:VP-Signature length mismatch\0\0`D\x82\x01R`d\x01a\x11\xA1V[\x82Q` \x84\x01 \x7F\x19Ethereum Signed Message:\n32\0\0\0\0`\0\x90\x81R`\x1C\x91\x90\x91R`<\x81 a.J\x90\x84a>yV[\x90P`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a.c\x91\x90aQ\xE2V[a\x01\xFCT`@Qc\xEA\xC7\x08\xA3`\xE0\x1B\x81R\x92\x94P\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xEA\xC7\x08\xA3\x90a.\x9A\x90\x85\x90\x85\x90`\x04\x01aS\x0BV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a.\xB2W`\0\x80\xFD[PZ\xFA\x15\x80\x15a.\xC6W=`\0\x80>=`\0\xFD[PPPP`\0a.\xD9\x82`\0\x01Qa>\xA3V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a/<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FM:VP-Enclave key mismatch\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11\xA1V[\x8Da/T\x83` \x01Q\x84`@\x01Q\x85``\x01Qa!\xEAV[\x14a/\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqM:VP-Invalid image`p\x1B`D\x82\x01R`d\x01a\x11\xA1V[PPPPPPPPPPPPPPV[`\0[\x81Q\x81\x10\x15a\x18\xECW`\0\x82\x82\x81Q\x81\x10a/\xC6Wa/\xC6aN\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q``\x81\x01\x80Q`\0\x88\x81Ra\x03\xF8\x85R`@\x80\x82 \x81\x86\x01\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x85R\x91\x88R\x82\x84 \x87\x89\x01Q\x83\x16\x85R\x88R\x82\x84 \x87Q\x83\x16\x85R\x88R\x82\x84 \x94\x90\x94U\x93Q\x8A\x83Ra\x03\xF7\x87R\x81\x83 \x93Q\x85\x16\x83R\x92\x86R\x80\x82 \x85Q\x90\x94\x16\x82R\x92\x90\x94R\x90\x83 \x80T\x92\x94P\x90\x92\x90\x91a0V\x90\x84\x90aO\x1FV[\x90\x91UPPa\x01\xFBT` \x82\x01Q\x82Q`@Qc\"\xD1\xFD\x8F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R\x91\x16\x90cE\xA3\xFB\x1E\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\xC5W=`\0\x80>=`\0\xFD[PP`\x01\x90\x93\x01\x92Pa/\xA9\x91PPV[`\0\x82\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x01T\x91\x81\x01\x82\x90R\x91\x03a1-W`\0\x83\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 `\x01\x01\x84\x90U[`\0\x83\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T`\x01\x92\x90a1X\x90\x84\x90aO\x1FV[\x90\x91UPPPPPPV[a1ka?\x02V[V[a1ua?\x02V[`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0UV[`\0`\0\x80Q` aV\\\x839\x81Q\x91Ra1\xB6\x84\x84a!\x91V[a26W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua1\xEC3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\r\xCEV[`\0\x91PPa\r\xCEV[a*\xED\x813a?KV[`\0a\"\x1A\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a?\x84V[`\0\x80a\x01\xF8T\x11a2\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTotal weight must be greater tha`D\x82\x01Ren zero`\xD0\x1B`d\x82\x01R`\x84\x01a\x11\xA1V[`\0a2\xCFa\x01\xF6a:gV[\x11a3\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrNo tokens available`h\x1B`D\x82\x01R`d\x01a\x11\xA1V[`\0a3\x1Fa\x01\xF6a:gV[`\x01`\x01`@\x1B\x03\x81\x11\x15a36Wa36aH\x05V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a3_W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0a3oa\x01\xF6a:gV[`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x86Wa3\x86aH\x05V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a3\xAFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[Pa\x01\xF8T\x90\x91P`\0\x80a3\xC5a\x01\xF6a:gV[\x90P`\0[\x81\x81\x10\x15a4lW`\0a3\xE0a\x01\xF6\x83a:qV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x03\xF4` R`@\x90 T\x90\x91P\x80\x15a4bW\x81\x88\x86\x81Q\x81\x10a4\x16Wa4\x16aN\xDCV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80\x87\x86\x81Q\x81\x10a4IWa4IaN\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x84a4^\x81aS\x9FV[\x95PP[PP`\x01\x01a3\xCAV[P[`\0\x82\x11a4\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNo stakeToken available to lock\0`D\x82\x01R`d\x01a\x11\xA1V[`\0\x83Ba4\xCD`\x01CaNgV[@3`@Q` \x01a5\x04\x93\x92\x91\x90\x92\x83R` \x83\x01\x91\x90\x91R``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`@\x82\x01R`T\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca5'\x91\x90aS\xCEV[\x90P`\0\x80\x80[\x85\x81\x10\x15a5\x8EW\x87\x81\x81Q\x81\x10a5HWa5HaN\xDCV[` \x02` \x01\x01Q\x83a5[\x91\x90aO\x1FV[\x92P\x82\x84\x10\x15a5\x86W\x88\x81\x81Q\x81\x10a5wWa5waN\xDCV[` \x02` \x01\x01Q\x91Pa5\x8EV[`\x01\x01a5.V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03\xF3` R`@\x90 Ta5\xB2\x83\x8Da\x1F~V[\x10a5\xC5WP\x99\x98PPPPPPPPPV[\x87\x81\x81Q\x81\x10a5\xD7Wa5\xD7aN\xDCV[` \x02` \x01\x01Q\x87a5\xEA\x91\x90aNgV[\x96P\x88a5\xF8`\x01\x88aNgV[\x81Q\x81\x10a6\x08Wa6\x08aN\xDCV[` \x02` \x01\x01Q\x89\x82\x81Q\x81\x10a6\"Wa6\"aN\xDCV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x87a6F`\x01\x88aNgV[\x81Q\x81\x10a6VWa6VaN\xDCV[` \x02` \x01\x01Q\x88\x82\x81Q\x81\x10a6pWa6paN\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x85a6\x85\x81aS\xF0V[\x96PPPPPPa4nV[`\0`\0\x80Q` aV\\\x839\x81Q\x91Ra6\xAC\x84\x84a!\x91V[\x15a26W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\r\xCEV[`\0a7\x19a\x01\xF5T\x90V[`@\x80Q``\x81\x01\x82R\x84\x81R3` \x80\x83\x01\x82\x81R\x83\x85\x01\x86\x81Ra\x01\xFE\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x85Q\x7F\x82\xCA\xFEv\xA8\xC3\x8B;\x8CO'\xAC\xA5&\x15\x01\x96\x86Ro8>\xD1\xF7E4\xAA\x82\xF5\xBE\x9A\xDC`\x03\x90\x92\x02\x91\x82\x01U\x91Q\x7F\x82\xCA\xFEv\xA8\xC3\x8B;\x8CO'\xAC\xA5&\x15\x01\x96\x86Ro8>\xD1\xF7E4\xAA\x82\xF5\xBE\x9A\xDD\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UQ\x7F\x82\xCA\xFEv\xA8\xC3\x8B;\x8CO'\xAC\xA5&\x15\x01\x96\x86Ro8>\xD1\xF7E4\xAA\x82\xF5\xBE\x9A\xDE\x90\x91\x01U\x92Q\x86\x81R\x93\x94P\x90\x92\x90\x91\x7FG\xE4t;xF\xC6\xF1\x14s\xE5\xCC\xCAQ/\x02:\xA5\x08B&\xF7\xB5\x88<\xB7o\x11\xBFa1\x1A\x91\x01`@Q\x80\x91\x03\x90\xA2`@Qb\xADN\xCD`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x05jvh\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a8rW`\0\x80\xFD[PZ\xF1\x15\x80\x15a8\x86W=`\0\x80>=`\0\xFD[PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\"\x1AV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a98WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a9,`\0\x80Q` aU\xFC\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a1kW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B\x82\x81a2@V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a9\xBBWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra9\xB8\x91\x81\x01\x90aP\xB7V[`\x01[a9\xE3W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x11\xA1V[`\0\x80Q` aU\xFC\x839\x81Q\x91R\x81\x14a:\x14W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x11\xA1V[a\x18\xEC\x83\x83a@mV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a1kW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r\xCE\x82T\x90V[`\0a\"\x1A\x83\x83a@\xC3V[```\0a\"\x1A\x83a@\xEDV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x18\xEC\x90\x84\x90aAIV[`\0\x81\x81Ra\x03\xFC` R`@\x81 \x90a:\xF6\x82\x82aEfV[a;\x04`\x01\x83\x01`\0aEfV[a;\x12`\x02\x83\x01`\0aEfV[PP`@Q\x81\x90\x7F\xA4+w\x11\xE1\xED])\xC8\x046\x85Z\x8BG\xA1\xDC\xAAG\xAA\x13.G%\xC0\xEF\x1D\xFC\xE2<\xAA\xFC\x90`\0\x90\xA2PV[`\0a\"\x1A\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aA\xBAV[a\x01\xFC\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x1C^v\0\x7F\xD00\x93\xBC\x99\xEE\xB7%\xE2\x05\xF4/\xE5]\x8A\xE0@D ,\xF4\x0C\r[o\x8C\r\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a;\xC5\x91\x90aT\x07V[\x92P\x92P\x92P`\0a;\xD8\x84\x84\x84a!\xEAV[`\0\x81\x81Ra\x03\xFC` R`@\x90 \x80T\x91\x92P\x90a;\xF6\x90aP\xD0V[\x15\x90Pa<<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsImage already exists``\x1B`D\x82\x01R`d\x01a\x11\xA1V[\x83Q`0\x14a<\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\t-\xCE\xCC-\x8D,\x84\n\x08jF\x04\r\x8C\xAD\xCC\xEE\x8D`k\x1B`D\x82\x01R`d\x01a\x11\xA1V[\x82Q`0\x14a<\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\t-\xCE\xCC-\x8D,\x84\n\x08jF$\r\x8C\xAD\xCC\xEE\x8D`k\x1B`D\x82\x01R`d\x01a\x11\xA1V[\x81Q`0\x14a=\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\t-\xCE\xCC-\x8D,\x84\n\x08jFD\r\x8C\xAD\xCC\xEE\x8D`k\x1B`D\x82\x01R`d\x01a\x11\xA1V[`@\x80Q``\x81\x01\x82R\x85\x81R` \x80\x82\x01\x86\x90R\x81\x83\x01\x85\x90R`\0\x84\x81Ra\x03\xFC\x90\x91R\x91\x90\x91 \x81Q\x82\x91\x90\x81\x90a=L\x90\x82aT\xD5V[P` \x82\x01Q`\x01\x82\x01\x90a=a\x90\x82aT\xD5V[P`@\x82\x01Q`\x02\x82\x01\x90a=v\x90\x82aT\xD5V[P\x90PP\x81\x7Fg\x0E@\x8E.6r\x10\xFEHK\x84\x1F\xAD\xD6\xAFRmJg5`\x12\xF9\xC6GP\xE3\xF2\xB5\\\x04\x86\x86\x86`@Qa=\xAE\x93\x92\x91\x90aJ\xFEV[`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0\x83\x83\x02\x81`\0\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a=\xF5W\x83\x82\x81a=\xEBWa=\xEBaS\xB8V[\x04\x92PPPa\"\x1AV[\x80\x84\x11a>\x0CWa>\x0C`\x03\x85\x15\x02`\x11\x18aB\tV[`\0\x84\x86\x88\t`\0\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[`\0\x80`\0\x80a>\x89\x86\x86aB\x1BV[\x92P\x92P\x92Pa>\x99\x82\x82aBhV[P\x90\x94\x93PPPPV[`\0\x81Q`@\x14a>\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FM:VP-Invalid public key length\0\0`D\x82\x01R`d\x01a\x11\xA1V[P\x80Q` \x90\x91\x01 \x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a1kW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a?U\x82\x82a!\x91V[a\x1B\x82W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x11\xA1V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a26W`\0a?\xA8`\x01\x83aNgV[\x85T\x90\x91P`\0\x90a?\xBC\x90`\x01\x90aNgV[\x90P\x80\x82\x14a@!W`\0\x86`\0\x01\x82\x81T\x81\x10a?\xDCWa?\xDCaN\xDCV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a?\xFFWa?\xFFaN\xDCV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a@2Wa@2aU\x93V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\r\xCEV[a@v\x82aC!V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a@\xBBWa\x18\xEC\x82\x82aC\x86V[a\x1B\x82aC\xF3V[`\0\x82`\0\x01\x82\x81T\x81\x10a@\xDAWa@\xDAaN\xDCV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aA=W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aA)W[PPPPP\x90P\x91\x90PV[`\0\x80` `\0\x84Q` \x86\x01`\0\x88Z\xF1\x80aAlW`@Q=`\0\x82>=\x81\xFD[PP`\0Q=\x91P\x81\x15aA\x84W\x80`\x01\x14\x15aA\x91V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x16qW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x11\xA1V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 TaB\x01WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\r\xCEV[P`\0a\r\xCEV[cNH{q`\0R\x80` R`$`\x1C\xFD[`\0\x80`\0\x83Q`A\x03aBUW` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1AaBG\x88\x82\x85\x85aD\x12V[\x95P\x95P\x95PPPPaBaV[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15aB|WaB|aU\xA9V[\x03aB\x85WPPV[`\x01\x82`\x03\x81\x11\x15aB\x99WaB\x99aU\xA9V[\x03aB\xB7W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15aB\xCBWaB\xCBaU\xA9V[\x03aB\xECW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x11\xA1V[`\x03\x82`\x03\x81\x11\x15aC\0WaC\0aU\xA9V[\x03a\x1B\x82W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x11\xA1V[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03aCWW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x11\xA1V[`\0\x80Q` aU\xFC\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@QaC\xA3\x91\x90aU\xBFV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14aC\xDEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aC\xE3V[``\x91P[P\x91P\x91Pa\x1F\xCD\x85\x83\x83aD\xE1V[4\x15a1kW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15aDMWP`\0\x91P`\x03\x90P\x82aD\xD7V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aD\xA1W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aD\xCDWP`\0\x92P`\x01\x91P\x82\x90PaD\xD7V[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82aD\xF6WaD\xF1\x82aE=V[a\"\x1AV[\x81Q\x15\x80\x15aE\rWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15aE6W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x11\xA1V[P\x80a\"\x1AV[\x80Q\x15aEMW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80TaEr\x90aP\xD0V[`\0\x82U\x80`\x1F\x10aE\x82WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a*\xED\x91\x90[\x80\x82\x11\x15aE\xB0W`\0\x81U`\x01\x01aE\x9CV[P\x90V[`\0` \x82\x84\x03\x12\x15aE\xC6W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\"\x1AW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12aE\xF0W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x07W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aF\x1FW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aFBW`\0\x80\xFD[\x885\x97P` \x89\x015\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aFtW`\0\x80\xFD[aF\x80\x8B\x82\x8C\x01aE\xDEV[\x90\x95P\x93PP`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x9FW`\0\x80\xFD[aF\xAB\x8B\x82\x8C\x01aE\xDEV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a*\xEDW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aF\xE7W`\0\x80\xFD[\x825aF\xF2\x81aF\xBFV[\x91P` \x83\x015aG\x02\x81aF\xBFV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aG\x1FW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aG>W`\0\x80\xFD[\x855aGI\x81aF\xBFV[\x94P` \x86\x015aGY\x81aF\xBFV[\x93P`@\x86\x015aGi\x81aF\xBFV[\x92P``\x86\x015aGy\x81aF\xBFV[\x91P`\x80\x86\x015aG\x89\x81aF\xBFV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15aG\xA9W`\0\x80\xFD[\x815a\"\x1A\x81aF\xBFV[`\0\x80`@\x83\x85\x03\x12\x15aG\xC7W`\0\x80\xFD[\x825\x91P` \x83\x015aG\x02\x81aF\xBFV[`\0\x80`@\x83\x85\x03\x12\x15aG\xECW`\0\x80\xFD[\x825aG\xF7\x81aF\xBFV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH=WaH=aH\x05V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH=WaH=aH\x05V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH=WaH=aH\x05V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH\xAFWaH\xAFaH\x05V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aH\xD0WaH\xD0aH\x05V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aH\xEFW`\0\x80\xFD[\x815aI\x02aH\xFD\x82aH\xB7V[aH\x87V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aI\x17W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aIMW`\0\x80\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x7FW`\0\x80\xFD[aI\x8B\x89\x82\x8A\x01aH\xDEV[\x92PP`\xA0\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xA7W`\0\x80\xFD[aI\xB3\x89\x82\x8A\x01aH\xDEV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15aI\xD3W`\0\x80\xFD[\x825aI\xDE\x81aF\xBFV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xF9W`\0\x80\xFD[aJ\x05\x85\x82\x86\x01aH\xDEV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15aJJW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aJ#V[P\x93\x94\x93PPPPV[`@\x81R`\0aJg`@\x83\x01\x85aJ\x0FV[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x86\x01\x92P`\0[\x81\x81\x10\x15aJ\xA2W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aJ\x84V[P\x90\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15aJ\xC9W\x81\x81\x01Q\x83\x82\x01R` \x01aJ\xB1V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaJ\xEA\x81` \x86\x01` \x86\x01aJ\xAEV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81R`\0aK\x11``\x83\x01\x86aJ\xD2V[\x82\x81\x03` \x84\x01RaK#\x81\x86aJ\xD2V[\x90P\x82\x81\x03`@\x84\x01RaK7\x81\x85aJ\xD2V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aKSW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aKiW`\0\x80\xFD[aKu\x84\x82\x85\x01aH\xDEV[\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aK\x92W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xA8W`\0\x80\xFD[aK\xB4\x86\x82\x87\x01aH\xDEV[\x93PP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xD0W`\0\x80\xFD[aK\xDC\x86\x82\x87\x01aH\xDEV[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xF8W`\0\x80\xFD[aL\x04\x86\x82\x87\x01aH\xDEV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15aL#W`\0\x80\xFD[\x835aL.\x81aF\xBFV[\x92P` \x84\x015aL>\x81aF\xBFV[\x91P`@\x84\x015aLN\x81aF\xBFV[\x80\x91PP\x92P\x92P\x92V[` \x81R`\0a\"\x1A` \x83\x01\x84aJ\xD2V[`\0\x80`\0``\x84\x86\x03\x12\x15aL\x81W`\0\x80\xFD[\x835\x92P` \x84\x015aL\x93\x81aF\xBFV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[` \x81R`\0a\"\x1A` \x83\x01\x84aJ\x0FV[`\0\x80`@\x83\x85\x03\x12\x15aL\xCAW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80` \x83\x85\x03\x12\x15aL\xECW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x02W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aM\x13W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aM)W`\0\x80\xFD[\x85` ``\x83\x02\x84\x01\x01\x11\x15aM>W`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aMgWaMgaH\x05V[P`\x05\x1B` \x01\x90V[`\0` \x82\x84\x03\x12\x15aM\x83W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x99W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aM\xAAW`\0\x80\xFD[\x805aM\xB8aH\xFD\x82aMNV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x07\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aM\xDAW`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aK7W`\x80\x84\x88\x03\x12\x15aM\xF9W`\0\x80\xFD[aN\x01aH\x1BV[\x845aN\x0C\x81aF\xBFV[\x81R` \x85\x015aN\x1C\x81aF\xBFV[` \x82\x01R`@\x85\x015aN/\x81aF\xBFV[`@\x82\x01R``\x85\x81\x015\x90\x82\x01R\x82R`\x80\x90\x93\x01\x92` \x90\x91\x01\x90aM\xE1V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\r\xCEWa\r\xCEaNQV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x85\x81R``` \x82\x01R`\0aN\xBD``\x83\x01\x86\x88aNzV[\x82\x81\x03`@\x84\x01RaN\xD0\x81\x85\x87aNzV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`\x13\x90\x82\x01Rr'\xB76<\x90)\xBA0\xB5\xB4\xB73\xA6\xB0\xB70\xB3\xB2\xB9`i\x1B`@\x82\x01R``\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\r\xCEWa\r\xCEaNQV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15aOhW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO~W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aO\x8FW`\0\x80\xFD[\x80QaO\x9DaH\xFD\x82aMNV[\x80\x82\x82R` \x82\x01\x91P` ``\x84\x02\x85\x01\x01\x92P\x86\x83\x11\x15aO\xBFW`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aK7W``\x84\x88\x03\x12\x15aO\xDEW`\0\x80\xFD[aO\xE6aHCV[\x84Q\x81R` \x85\x01QaO\xF8\x81aF\xBFV[` \x82\x01R`@\x85\x01QaP\x0B\x81aF\xBFV[`@\x82\x01R\x82R``\x93\x90\x93\x01\x92` \x90\x91\x01\x90aO\xC6V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aP\x81W\x83Q\x80Q\x84R` \x80\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x87\x01R`@\x92\x83\x01Q\x16\x91\x85\x01\x91\x90\x91R\x90\x93\x01\x92``\x90\x92\x01\x91`\x01\x01aP>V[P\x90\x95\x94PPPPPV[\x83\x81R``` \x82\x01R`\0aP\xA5``\x83\x01\x85aJ\xD2V[\x82\x81\x03`@\x84\x01RaK7\x81\x85aJ\xD2V[`\0` \x82\x84\x03\x12\x15aP\xC9W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80aP\xE4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aQ\x04WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x85\x81R\x84` \x82\x01R\x83`@\x82\x01R\x82``\x82\x01R`\xA0`\x80\x82\x01R`\0aQ5`\xA0\x83\x01\x84aJ\xD2V[\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12aQQW`\0\x80\xFD[\x81QaQ_aH\xFD\x82aH\xB7V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aQtW`\0\x80\xFD[aKu\x82` \x83\x01` \x87\x01aJ\xAEV[`\0\x80`@\x83\x85\x03\x12\x15aQ\x98W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xAEW`\0\x80\xFD[aQ\xBA\x85\x82\x86\x01aQ@V[\x92PP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xD6W`\0\x80\xFD[aJ\x05\x85\x82\x86\x01aQ@V[`\0\x80`@\x83\x85\x03\x12\x15aQ\xF5W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x0BW`\0\x80\xFD[aR\x17\x85\x82\x86\x01aQ@V[\x92PP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR3W`\0\x80\xFD[\x83\x01`\xA0\x81\x86\x03\x12\x15aREW`\0\x80\xFD[aRMaHeV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aRcW`\0\x80\xFD[aRo\x87\x82\x85\x01aQ@V[\x82RP` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x8BW`\0\x80\xFD[aR\x97\x87\x82\x85\x01aQ@V[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xB6W`\0\x80\xFD[aR\xC2\x87\x82\x85\x01aQ@V[`@\x83\x01RP``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xE1W`\0\x80\xFD[aR\xED\x87\x82\x85\x01aQ@V[``\x83\x01RP`\x80\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R\x91\x94\x91\x93P\x90\x91PPV[`@\x81R`\0aS\x1E`@\x83\x01\x85aJ\xD2V[\x82\x81\x03` \x84\x01R\x83Q`\xA0\x82RaS9`\xA0\x83\x01\x82aJ\xD2V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01RaSR\x82\x82aJ\xD2V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01RaSl\x82\x82aJ\xD2V[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01RaS\x86\x82\x82aJ\xD2V[\x91PP`\x80\x85\x01Q`\x80\x83\x01R\x80\x92PPP\x93\x92PPPV[`\0`\x01\x82\x01aS\xB1WaS\xB1aNQV[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aS\xEBWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x81aS\xFFWaS\xFFaNQV[P`\0\x19\x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aT\x1CW`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15aT2W`\0\x80\xFD[aT>\x86\x82\x87\x01aQ@V[\x93PP` \x84\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aTZW`\0\x80\xFD[aTf\x86\x82\x87\x01aQ@V[\x92PP`@\x84\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aT\x82W`\0\x80\xFD[aL\x04\x86\x82\x87\x01aQ@V[`\x1F\x82\x11\x15a\x18\xECW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aT\xB5WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a-\rW`\0\x81U`\x01\x01aT\xC1V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xEEWaT\xEEaH\x05V[aU\x02\x81aT\xFC\x84TaP\xD0V[\x84aT\x8EV[` `\x1F\x82\x11`\x01\x81\x14aU6W`\0\x83\x15aU\x1EWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua-\rV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aUfW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aUFV[P\x84\x82\x10\x15aU\x84W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82QaU\xD1\x81\x84` \x87\x01aJ\xAEV[\x91\x90\x91\x01\x92\x91PPV\xFE\xC8\x987\xA6\xEA\x060\x8Dj/A\0}-\n\xF1\x8F\xC3L\x85\xAC!K\xC9_\x0FV\xC1\x91\xDB\x87\x076\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x86\xE4\x8C\xDEp\r\xD7\xFD\x18`6D\x94K\x13x|\xC9\xAEB\xD3J!\xF8\xBF\xB6\x9B\xC2\xEA\xB7\xED\xE1\x133\x02\x8F\xE2\xC6wG\xE7\xC1\xEF\nq\x1C\xBD\x02\x88\xF2\x9C\0Y\xF6k\x85`\xD4\xEE\\g\x92\xB4\xA3\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\xA2dipfsX\"\x12 \x83\xF3#\xAA\xCE\x9ChK\xCA(tX\\\x89\x96\x1C\xF9\xDD\xA1Z\\\xDDQ\xD1\x88\x11\x9F\xAB\xDE\x81.\xEAdsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static SYMBIOTICSTAKING_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct SymbioticStaking<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SymbioticStaking<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SymbioticStaking<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SymbioticStaking<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SymbioticStaking<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SymbioticStaking))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SymbioticStaking<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SYMBIOTICSTAKING_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                SYMBIOTICSTAKING_ABI.clone(),
                SYMBIOTICSTAKING_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BRIDGE_ENCLAVE_UPDATES_ROLE` (0x36719673) function
        pub fn bridge_enclave_updates_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 113, 150, 115], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `COMPLETE_MASK` (0x1ccb3d66) function
        pub fn complete_mask(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([28, 203, 61, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `I_GENERATOR_CALLBACK` (0x110cbc80) function
        pub fn i_generator_callback(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([17, 12, 188, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SIGNATURE_LENGTH` (0x540bc5ea) function
        pub fn signature_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 11, 197, 234], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SLASH_RESULT_MASK` (0xc9061a6a) function
        pub fn slash_result_mask(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([201, 6, 26, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SLASH_RESULT_TYPE` (0x9b88ee16) function
        pub fn slash_result_type(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([155, 136, 238, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKE_SNAPSHOT_MASK` (0xb1b37007) function
        pub fn stake_snapshot_mask(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([177, 179, 112, 7], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKE_SNAPSHOT_TYPE` (0x60fba48b) function
        pub fn stake_snapshot_type(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([96, 251, 164, 139], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UPGRADE_INTERFACE_VERSION` (0xad3cb1cc) function
        pub fn upgrade_interface_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([173, 60, 177, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addEnclaveImage` (0x945d0f4c) function
        pub fn add_enclave_image(
            &self,
            pc_rs: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 93, 15, 76], pc_rs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addEnclaveImage` (0xac68d877) function
        pub fn add_enclave_image_with_pcr_0(
            &self,
            pcr0: ::ethers::core::types::Bytes,
            pcr1: ::ethers::core::types::Bytes,
            pcr2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 104, 216, 119], (pcr0, pcr1, pcr2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addStakeToken` (0x81f7d942) function
        pub fn add_stake_token(
            &self,
            stake_token: ::ethers::core::types::Address,
            weight: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 247, 217, 66], (stake_token, weight))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `amountToLock` (0x8d3b9d5a) function
        pub fn amount_to_lock(
            &self,
            stake_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([141, 59, 157, 90], stake_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `attestationVerifier` (0xb4b3c5a0) function
        pub fn attestation_verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([180, 179, 197, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `baseTransmitterComissionRate` (0xb92c227a) function
        pub fn base_transmitter_comission_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([185, 44, 34, 122], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `confirmedTimestamps` (0x16b71ef4) function
        pub fn confirmed_timestamps(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([22, 183, 30, 244], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emergencyWithdraw` (0x6382d9ad) function
        pub fn emergency_withdraw(
            &self,
            token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 130, 217, 173], (token, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enclaveImages` (0x8f9c3095) function
        pub fn enclave_images(
            &self,
            image_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Bytes,
                ::ethers::core::types::Bytes,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([143, 156, 48, 149], image_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeRewardToken` (0x3d74a778) function
        pub fn fee_reward_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([61, 116, 167, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getImageId` (0x9e3a2c9e) function
        pub fn get_image_id(
            &self,
            pcr0: ::ethers::core::types::Bytes,
            pcr1: ::ethers::core::types::Bytes,
            pcr2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([158, 58, 44, 158], (pcr0, pcr1, pcr2))
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
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeAmount` (0x9fcd115f) function
        pub fn get_stake_amount(
            &self,
            stake_token: ::ethers::core::types::Address,
            vault: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 205, 17, 95], (stake_token, vault, operator))
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
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x1459457a) function
        pub fn initialize(
            &self,
            admin: ::ethers::core::types::Address,
            proof_marketplace: ::ethers::core::types::Address,
            staking_manager: ::ethers::core::types::Address,
            reward_distributor: ::ethers::core::types::Address,
            fee_reward_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [20, 89, 69, 122],
                    (
                        admin,
                        proof_marketplace,
                        staking_manager,
                        reward_distributor,
                        fee_reward_token,
                    ),
                )
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
        ///Calls the contract's `operatorLockedAmounts` (0x0a59bb50) function
        pub fn operator_locked_amounts(
            &self,
            stake_token: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 89, 187, 80], (stake_token, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proofMarketplace` (0x81c45c70) function
        pub fn proof_marketplace(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([129, 196, 92, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
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
        ///Calls the contract's `removeEnclaveImage` (0x8075c26a) function
        pub fn remove_enclave_image(
            &self,
            image_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 117, 194, 106], image_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeStakeToken` (0x166866c7) function
        pub fn remove_stake_token(
            &self,
            stake_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 104, 102, 199], stake_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            caller_confirmation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, caller_confirmation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
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
        ///Calls the contract's `setAmountToLock` (0x1f3dc4e2) function
        pub fn set_amount_to_lock(
            &self,
            stake_token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 61, 196, 226], (stake_token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAttestationVerifier` (0x82f6f608) function
        pub fn set_attestation_verifier(
            &self,
            attestation_verifier: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 246, 246, 8], attestation_verifier)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBaseTransmitterComissionRate` (0xc40cc6e9) function
        pub fn set_base_transmitter_comission_rate(
            &self,
            base_transmitter_comission: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 12, 198, 233], base_transmitter_comission)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeRewardToken` (0x3df36ddf) function
        pub fn set_fee_reward_token(
            &self,
            fee_reward_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 243, 109, 223], fee_reward_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setJobManager` (0x1ef9176a) function
        pub fn set_job_manager(
            &self,
            job_manager: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 249, 23, 106], job_manager)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRewardDistributor` (0xa1809b95) function
        pub fn set_reward_distributor(
            &self,
            reward_distributor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 128, 155, 149], reward_distributor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStakeTokenSelectionWeight` (0x5724b84e) function
        pub fn set_stake_token_selection_weight(
            &self,
            stake_token: ::ethers::core::types::Address,
            weight: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 36, 184, 78], (stake_token, weight))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStakingManager` (0xb00bba6a) function
        pub fn set_staking_manager(
            &self,
            staking_manager: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 11, 186, 106], staking_manager)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSubmissionCooldown` (0x51c39c66) function
        pub fn set_submission_cooldown(
            &self,
            submission_cooldown: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 195, 156, 102], submission_cooldown)
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
        ///Calls the contract's `stakeTokenSelectionWeight` (0xcb95717a) function
        pub fn stake_token_selection_weight(
            &self,
            stake_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([203, 149, 113, 122], stake_token)
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
        ///Calls the contract's `stakingManager` (0x22828cc2) function
        pub fn staking_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([34, 130, 140, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submissionCooldown` (0x9f3d545e) function
        pub fn submission_cooldown(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 61, 84, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submissionStatus` (0x1be8f483) function
        pub fn submission_status(
            &self,
            capture_timestamp: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([27, 232, 244, 131], (capture_timestamp, account))
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
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `txCountInfo` (0xd473e122) function
        pub fn tx_count_info(
            &self,
            capture_timestamp: ::ethers::core::types::U256,
            submission_type: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([212, 115, 225, 34], (capture_timestamp, submission_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
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
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
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
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleAdminChangedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleGrantedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleRevokedFilter>
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
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradedFilter> {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SymbioticStakingEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for SymbioticStaking<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccessControlBadConfirmation` with signature `AccessControlBadConfirmation()` and selector `0x6697b232`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "AccessControlBadConfirmation",
        abi = "AccessControlBadConfirmation()"
    )]
    pub struct AccessControlBadConfirmation;
    ///Custom Error type `AccessControlUnauthorizedAccount` with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "AccessControlUnauthorizedAccount",
        abi = "AccessControlUnauthorizedAccount(address,bytes32)"
    )]
    pub struct AccessControlUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
        pub needed_role: [u8; 32],
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `ECDSAInvalidSignature` with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ECDSAInvalidSignature", abi = "ECDSAInvalidSignature()")]
    pub struct ECDSAInvalidSignature;
    ///Custom Error type `ECDSAInvalidSignatureLength` with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ECDSAInvalidSignatureLength",
        abi = "ECDSAInvalidSignatureLength(uint256)"
    )]
    pub struct ECDSAInvalidSignatureLength {
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `ECDSAInvalidSignatureS` with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ECDSAInvalidSignatureS",
        abi = "ECDSAInvalidSignatureS(bytes32)"
    )]
    pub struct ECDSAInvalidSignatureS {
        pub s: [u8; 32],
    }
    ///Custom Error type `ERC1967InvalidImplementation` with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ERC1967InvalidImplementation",
        abi = "ERC1967InvalidImplementation(address)"
    )]
    pub struct ERC1967InvalidImplementation {
        pub implementation: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1967NonPayable` with signature `ERC1967NonPayable()` and selector `0xb398979f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC1967NonPayable", abi = "ERC1967NonPayable()")]
    pub struct ERC1967NonPayable;
    ///Custom Error type `FailedCall` with signature `FailedCall()` and selector `0xd6bda275`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "FailedCall", abi = "FailedCall()")]
    pub struct FailedCall;
    ///Custom Error type `InvalidInitialization` with signature `InvalidInitialization()` and selector `0xf92ee8a9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidInitialization", abi = "InvalidInitialization()")]
    pub struct InvalidInitialization;
    ///Custom Error type `NotInitializing` with signature `NotInitializing()` and selector `0xd7e6bcf8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotInitializing", abi = "NotInitializing()")]
    pub struct NotInitializing;
    ///Custom Error type `ReentrancyGuardReentrantCall` with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ReentrancyGuardReentrantCall",
        abi = "ReentrancyGuardReentrantCall()"
    )]
    pub struct ReentrancyGuardReentrantCall;
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `UUPSUnauthorizedCallContext` with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "UUPSUnauthorizedCallContext",
        abi = "UUPSUnauthorizedCallContext()"
    )]
    pub struct UUPSUnauthorizedCallContext;
    ///Custom Error type `UUPSUnsupportedProxiableUUID` with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "UUPSUnsupportedProxiableUUID",
        abi = "UUPSUnsupportedProxiableUUID(bytes32)"
    )]
    pub struct UUPSUnsupportedProxiableUUID {
        pub slot: [u8; 32],
    }
    ///Container type for all of the contract's custom errors
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
    pub enum SymbioticStakingErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        AddressEmptyCode(AddressEmptyCode),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedCall(FailedCall),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SymbioticStakingErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <AccessControlBadConfirmation as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AccessControlBadConfirmation(decoded));
            }
            if let Ok(decoded) =
                <AccessControlUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AccessControlUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) =
                <ECDSAInvalidSignature as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ECDSAInvalidSignature(decoded));
            }
            if let Ok(decoded) =
                <ECDSAInvalidSignatureLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ECDSAInvalidSignatureLength(decoded));
            }
            if let Ok(decoded) =
                <ECDSAInvalidSignatureS as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ECDSAInvalidSignatureS(decoded));
            }
            if let Ok(decoded) =
                <ERC1967InvalidImplementation as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1967InvalidImplementation(decoded));
            }
            if let Ok(decoded) = <ERC1967NonPayable as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1967NonPayable(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedCall(decoded));
            }
            if let Ok(decoded) =
                <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <NotInitializing as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotInitializing(decoded));
            }
            if let Ok(decoded) =
                <ReentrancyGuardReentrantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReentrancyGuardReentrantCall(decoded));
            }
            if let Ok(decoded) =
                <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) =
                <UUPSUnauthorizedCallContext as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UUPSUnauthorizedCallContext(decoded));
            }
            if let Ok(decoded) =
                <UUPSUnsupportedProxiableUUID as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UUPSUnsupportedProxiableUUID(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SymbioticStakingErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ECDSAInvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967InvalidImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967NonPayable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitializing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnauthorizedCallContext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SymbioticStakingErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccessControlBadConfirmation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967InvalidImplementation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967NonPayable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedCall as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitializing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReentrancyGuardReentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UUPSUnauthorizedCallContext as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UUPSUnsupportedProxiableUUID as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SymbioticStakingErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureS(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInitialization(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnauthorizedCallContext(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SymbioticStakingErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation> for SymbioticStakingErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount> for SymbioticStakingErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for SymbioticStakingErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for SymbioticStakingErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength> for SymbioticStakingErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for SymbioticStakingErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for SymbioticStakingErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for SymbioticStakingErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedCall> for SymbioticStakingErrors {
        fn from(value: FailedCall) -> Self {
            Self::FailedCall(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for SymbioticStakingErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for SymbioticStakingErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for SymbioticStakingErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for SymbioticStakingErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for SymbioticStakingErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for SymbioticStakingErrors {
        fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
            Self::UUPSUnsupportedProxiableUUID(value)
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint64)")]
    pub struct InitializedFilter {
        pub version: u64,
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
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
    pub enum SymbioticStakingEvents {
        AmountToLockSetFilter(AmountToLockSetFilter),
        AttestationVerifierUpdatedFilter(AttestationVerifierUpdatedFilter),
        BaseTransmitterComissionRateSetFilter(BaseTransmitterComissionRateSetFilter),
        EnclaveImageAddedFilter(EnclaveImageAddedFilter),
        EnclaveImageRemovedFilter(EnclaveImageRemovedFilter),
        FeeRewardTokenSetFilter(FeeRewardTokenSetFilter),
        InitializedFilter(InitializedFilter),
        JobSlashedFilter(JobSlashedFilter),
        ProofMarketplaceSetFilter(ProofMarketplaceSetFilter),
        RewardDistributorSetFilter(RewardDistributorSetFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        SlashResultSubmittedFilter(SlashResultSubmittedFilter),
        SnapshotConfirmedFilter(SnapshotConfirmedFilter),
        StakeLockedFilter(StakeLockedFilter),
        StakeTokenAddedFilter(StakeTokenAddedFilter),
        StakeTokenRemovedFilter(StakeTokenRemovedFilter),
        StakeTokenSelectionWeightSetFilter(StakeTokenSelectionWeightSetFilter),
        StakeUnlockedFilter(StakeUnlockedFilter),
        StakingManagerSetFilter(StakingManagerSetFilter),
        SubmissionCooldownSetFilter(SubmissionCooldownSetFilter),
        UpgradedFilter(UpgradedFilter),
        VaultSnapshotSubmittedFilter(VaultSnapshotSubmittedFilter),
    }
    impl ::ethers::contract::EthLogDecode for SymbioticStakingEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AmountToLockSetFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::AmountToLockSetFilter(decoded));
            }
            if let Ok(decoded) = AttestationVerifierUpdatedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::AttestationVerifierUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = BaseTransmitterComissionRateSetFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::BaseTransmitterComissionRateSetFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageAddedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::EnclaveImageAddedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageRemovedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::EnclaveImageRemovedFilter(decoded));
            }
            if let Ok(decoded) = FeeRewardTokenSetFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::FeeRewardTokenSetFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = JobSlashedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::JobSlashedFilter(decoded));
            }
            if let Ok(decoded) = ProofMarketplaceSetFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::ProofMarketplaceSetFilter(decoded));
            }
            if let Ok(decoded) = RewardDistributorSetFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::RewardDistributorSetFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = SlashResultSubmittedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::SlashResultSubmittedFilter(decoded));
            }
            if let Ok(decoded) = SnapshotConfirmedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::SnapshotConfirmedFilter(decoded));
            }
            if let Ok(decoded) = StakeLockedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::StakeLockedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenAddedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::StakeTokenAddedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenRemovedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::StakeTokenRemovedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenSelectionWeightSetFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::StakeTokenSelectionWeightSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StakeUnlockedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::StakeUnlockedFilter(decoded));
            }
            if let Ok(decoded) = StakingManagerSetFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::StakingManagerSetFilter(decoded));
            }
            if let Ok(decoded) = SubmissionCooldownSetFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::SubmissionCooldownSetFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::UpgradedFilter(decoded));
            }
            if let Ok(decoded) = VaultSnapshotSubmittedFilter::decode_log(log) {
                return Ok(SymbioticStakingEvents::VaultSnapshotSubmittedFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SymbioticStakingEvents {
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
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::JobSlashedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketplaceSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardDistributorSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VaultSnapshotSubmittedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AmountToLockSetFilter> for SymbioticStakingEvents {
        fn from(value: AmountToLockSetFilter) -> Self {
            Self::AmountToLockSetFilter(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierUpdatedFilter> for SymbioticStakingEvents {
        fn from(value: AttestationVerifierUpdatedFilter) -> Self {
            Self::AttestationVerifierUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<BaseTransmitterComissionRateSetFilter> for SymbioticStakingEvents {
        fn from(value: BaseTransmitterComissionRateSetFilter) -> Self {
            Self::BaseTransmitterComissionRateSetFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageAddedFilter> for SymbioticStakingEvents {
        fn from(value: EnclaveImageAddedFilter) -> Self {
            Self::EnclaveImageAddedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRemovedFilter> for SymbioticStakingEvents {
        fn from(value: EnclaveImageRemovedFilter) -> Self {
            Self::EnclaveImageRemovedFilter(value)
        }
    }
    impl ::core::convert::From<FeeRewardTokenSetFilter> for SymbioticStakingEvents {
        fn from(value: FeeRewardTokenSetFilter) -> Self {
            Self::FeeRewardTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for SymbioticStakingEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<JobSlashedFilter> for SymbioticStakingEvents {
        fn from(value: JobSlashedFilter) -> Self {
            Self::JobSlashedFilter(value)
        }
    }
    impl ::core::convert::From<ProofMarketplaceSetFilter> for SymbioticStakingEvents {
        fn from(value: ProofMarketplaceSetFilter) -> Self {
            Self::ProofMarketplaceSetFilter(value)
        }
    }
    impl ::core::convert::From<RewardDistributorSetFilter> for SymbioticStakingEvents {
        fn from(value: RewardDistributorSetFilter) -> Self {
            Self::RewardDistributorSetFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for SymbioticStakingEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for SymbioticStakingEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for SymbioticStakingEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<SlashResultSubmittedFilter> for SymbioticStakingEvents {
        fn from(value: SlashResultSubmittedFilter) -> Self {
            Self::SlashResultSubmittedFilter(value)
        }
    }
    impl ::core::convert::From<SnapshotConfirmedFilter> for SymbioticStakingEvents {
        fn from(value: SnapshotConfirmedFilter) -> Self {
            Self::SnapshotConfirmedFilter(value)
        }
    }
    impl ::core::convert::From<StakeLockedFilter> for SymbioticStakingEvents {
        fn from(value: StakeLockedFilter) -> Self {
            Self::StakeLockedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenAddedFilter> for SymbioticStakingEvents {
        fn from(value: StakeTokenAddedFilter) -> Self {
            Self::StakeTokenAddedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenRemovedFilter> for SymbioticStakingEvents {
        fn from(value: StakeTokenRemovedFilter) -> Self {
            Self::StakeTokenRemovedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenSelectionWeightSetFilter> for SymbioticStakingEvents {
        fn from(value: StakeTokenSelectionWeightSetFilter) -> Self {
            Self::StakeTokenSelectionWeightSetFilter(value)
        }
    }
    impl ::core::convert::From<StakeUnlockedFilter> for SymbioticStakingEvents {
        fn from(value: StakeUnlockedFilter) -> Self {
            Self::StakeUnlockedFilter(value)
        }
    }
    impl ::core::convert::From<StakingManagerSetFilter> for SymbioticStakingEvents {
        fn from(value: StakingManagerSetFilter) -> Self {
            Self::StakingManagerSetFilter(value)
        }
    }
    impl ::core::convert::From<SubmissionCooldownSetFilter> for SymbioticStakingEvents {
        fn from(value: SubmissionCooldownSetFilter) -> Self {
            Self::SubmissionCooldownSetFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for SymbioticStakingEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    impl ::core::convert::From<VaultSnapshotSubmittedFilter> for SymbioticStakingEvents {
        fn from(value: VaultSnapshotSubmittedFilter) -> Self {
            Self::VaultSnapshotSubmittedFilter(value)
        }
    }
    ///Container type for all input parameters for the `BRIDGE_ENCLAVE_UPDATES_ROLE` function with signature `BRIDGE_ENCLAVE_UPDATES_ROLE()` and selector `0x36719673`
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
        name = "BRIDGE_ENCLAVE_UPDATES_ROLE",
        abi = "BRIDGE_ENCLAVE_UPDATES_ROLE()"
    )]
    pub struct BridgeEnclaveUpdatesRoleCall;
    ///Container type for all input parameters for the `COMPLETE_MASK` function with signature `COMPLETE_MASK()` and selector `0x1ccb3d66`
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
    #[ethcall(name = "COMPLETE_MASK", abi = "COMPLETE_MASK()")]
    pub struct CompleteMaskCall;
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `I_GENERATOR_CALLBACK` function with signature `I_GENERATOR_CALLBACK()` and selector `0x110cbc80`
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
    #[ethcall(name = "I_GENERATOR_CALLBACK", abi = "I_GENERATOR_CALLBACK()")]
    pub struct IGeneratorCallbackCall;
    ///Container type for all input parameters for the `SIGNATURE_LENGTH` function with signature `SIGNATURE_LENGTH()` and selector `0x540bc5ea`
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
    #[ethcall(name = "SIGNATURE_LENGTH", abi = "SIGNATURE_LENGTH()")]
    pub struct SignatureLengthCall;
    ///Container type for all input parameters for the `SLASH_RESULT_MASK` function with signature `SLASH_RESULT_MASK()` and selector `0xc9061a6a`
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
    #[ethcall(name = "SLASH_RESULT_MASK", abi = "SLASH_RESULT_MASK()")]
    pub struct SlashResultMaskCall;
    ///Container type for all input parameters for the `SLASH_RESULT_TYPE` function with signature `SLASH_RESULT_TYPE()` and selector `0x9b88ee16`
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
    #[ethcall(name = "SLASH_RESULT_TYPE", abi = "SLASH_RESULT_TYPE()")]
    pub struct SlashResultTypeCall;
    ///Container type for all input parameters for the `STAKE_SNAPSHOT_MASK` function with signature `STAKE_SNAPSHOT_MASK()` and selector `0xb1b37007`
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
    #[ethcall(name = "STAKE_SNAPSHOT_MASK", abi = "STAKE_SNAPSHOT_MASK()")]
    pub struct StakeSnapshotMaskCall;
    ///Container type for all input parameters for the `STAKE_SNAPSHOT_TYPE` function with signature `STAKE_SNAPSHOT_TYPE()` and selector `0x60fba48b`
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
    #[ethcall(name = "STAKE_SNAPSHOT_TYPE", abi = "STAKE_SNAPSHOT_TYPE()")]
    pub struct StakeSnapshotTypeCall;
    ///Container type for all input parameters for the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
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
        name = "UPGRADE_INTERFACE_VERSION",
        abi = "UPGRADE_INTERFACE_VERSION()"
    )]
    pub struct UpgradeInterfaceVersionCall;
    ///Container type for all input parameters for the `addEnclaveImage` function with signature `addEnclaveImage(bytes)` and selector `0x945d0f4c`
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
    #[ethcall(name = "addEnclaveImage", abi = "addEnclaveImage(bytes)")]
    pub struct AddEnclaveImageCall {
        pub pc_rs: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `addEnclaveImage` function with signature `addEnclaveImage(bytes,bytes,bytes)` and selector `0xac68d877`
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
    #[ethcall(name = "addEnclaveImage", abi = "addEnclaveImage(bytes,bytes,bytes)")]
    pub struct AddEnclaveImageWithPcr0Call {
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `addStakeToken` function with signature `addStakeToken(address,uint256)` and selector `0x81f7d942`
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
    #[ethcall(name = "addStakeToken", abi = "addStakeToken(address,uint256)")]
    pub struct AddStakeTokenCall {
        pub stake_token: ::ethers::core::types::Address,
        pub weight: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `amountToLock` function with signature `amountToLock(address)` and selector `0x8d3b9d5a`
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
    #[ethcall(name = "amountToLock", abi = "amountToLock(address)")]
    pub struct AmountToLockCall {
        pub stake_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `attestationVerifier` function with signature `attestationVerifier()` and selector `0xb4b3c5a0`
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
    #[ethcall(name = "attestationVerifier", abi = "attestationVerifier()")]
    pub struct AttestationVerifierCall;
    ///Container type for all input parameters for the `baseTransmitterComissionRate` function with signature `baseTransmitterComissionRate()` and selector `0xb92c227a`
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
        name = "baseTransmitterComissionRate",
        abi = "baseTransmitterComissionRate()"
    )]
    pub struct BaseTransmitterComissionRateCall;
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
    ///Container type for all input parameters for the `confirmedTimestamps` function with signature `confirmedTimestamps(uint256)` and selector `0x16b71ef4`
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
    #[ethcall(name = "confirmedTimestamps", abi = "confirmedTimestamps(uint256)")]
    pub struct ConfirmedTimestampsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `emergencyWithdraw` function with signature `emergencyWithdraw(address,address)` and selector `0x6382d9ad`
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
    #[ethcall(name = "emergencyWithdraw", abi = "emergencyWithdraw(address,address)")]
    pub struct EmergencyWithdrawCall {
        pub token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `enclaveImages` function with signature `enclaveImages(bytes32)` and selector `0x8f9c3095`
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
    #[ethcall(name = "enclaveImages", abi = "enclaveImages(bytes32)")]
    pub struct EnclaveImagesCall {
        pub image_id: [u8; 32],
    }
    ///Container type for all input parameters for the `feeRewardToken` function with signature `feeRewardToken()` and selector `0x3d74a778`
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
    #[ethcall(name = "feeRewardToken", abi = "feeRewardToken()")]
    pub struct FeeRewardTokenCall;
    ///Container type for all input parameters for the `getImageId` function with signature `getImageId(bytes,bytes,bytes)` and selector `0x9e3a2c9e`
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
    #[ethcall(name = "getImageId", abi = "getImageId(bytes,bytes,bytes)")]
    pub struct GetImageIdCall {
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
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
        pub vault: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,address)` and selector `0x1459457a`
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
        name = "initialize",
        abi = "initialize(address,address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub admin: ::ethers::core::types::Address,
        pub proof_marketplace: ::ethers::core::types::Address,
        pub staking_manager: ::ethers::core::types::Address,
        pub reward_distributor: ::ethers::core::types::Address,
        pub fee_reward_token: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `operatorLockedAmounts` function with signature `operatorLockedAmounts(address,address)` and selector `0x0a59bb50`
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
        name = "operatorLockedAmounts",
        abi = "operatorLockedAmounts(address,address)"
    )]
    pub struct OperatorLockedAmountsCall {
        pub stake_token: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `proofMarketplace` function with signature `proofMarketplace()` and selector `0x81c45c70`
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
    #[ethcall(name = "proofMarketplace", abi = "proofMarketplace()")]
    pub struct ProofMarketplaceCall;
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
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
    ///Container type for all input parameters for the `removeEnclaveImage` function with signature `removeEnclaveImage(bytes32)` and selector `0x8075c26a`
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
    #[ethcall(name = "removeEnclaveImage", abi = "removeEnclaveImage(bytes32)")]
    pub struct RemoveEnclaveImageCall {
        pub image_id: [u8; 32],
    }
    ///Container type for all input parameters for the `removeStakeToken` function with signature `removeStakeToken(address)` and selector `0x166866c7`
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
    #[ethcall(name = "removeStakeToken", abi = "removeStakeToken(address)")]
    pub struct RemoveStakeTokenCall {
        pub stake_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub caller_confirmation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `setAmountToLock` function with signature `setAmountToLock(address,uint256)` and selector `0x1f3dc4e2`
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
    #[ethcall(name = "setAmountToLock", abi = "setAmountToLock(address,uint256)")]
    pub struct SetAmountToLockCall {
        pub stake_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setAttestationVerifier` function with signature `setAttestationVerifier(address)` and selector `0x82f6f608`
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
        name = "setAttestationVerifier",
        abi = "setAttestationVerifier(address)"
    )]
    pub struct SetAttestationVerifierCall {
        pub attestation_verifier: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setBaseTransmitterComissionRate` function with signature `setBaseTransmitterComissionRate(uint256)` and selector `0xc40cc6e9`
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
        name = "setBaseTransmitterComissionRate",
        abi = "setBaseTransmitterComissionRate(uint256)"
    )]
    pub struct SetBaseTransmitterComissionRateCall {
        pub base_transmitter_comission: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setFeeRewardToken` function with signature `setFeeRewardToken(address)` and selector `0x3df36ddf`
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
    #[ethcall(name = "setFeeRewardToken", abi = "setFeeRewardToken(address)")]
    pub struct SetFeeRewardTokenCall {
        pub fee_reward_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setJobManager` function with signature `setJobManager(address)` and selector `0x1ef9176a`
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
    #[ethcall(name = "setJobManager", abi = "setJobManager(address)")]
    pub struct SetJobManagerCall {
        pub job_manager: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setRewardDistributor` function with signature `setRewardDistributor(address)` and selector `0xa1809b95`
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
    #[ethcall(name = "setRewardDistributor", abi = "setRewardDistributor(address)")]
    pub struct SetRewardDistributorCall {
        pub reward_distributor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setStakeTokenSelectionWeight` function with signature `setStakeTokenSelectionWeight(address,uint256)` and selector `0x5724b84e`
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
        name = "setStakeTokenSelectionWeight",
        abi = "setStakeTokenSelectionWeight(address,uint256)"
    )]
    pub struct SetStakeTokenSelectionWeightCall {
        pub stake_token: ::ethers::core::types::Address,
        pub weight: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setStakingManager` function with signature `setStakingManager(address)` and selector `0xb00bba6a`
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
    #[ethcall(name = "setStakingManager", abi = "setStakingManager(address)")]
    pub struct SetStakingManagerCall {
        pub staking_manager: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setSubmissionCooldown` function with signature `setSubmissionCooldown(uint256)` and selector `0x51c39c66`
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
    #[ethcall(name = "setSubmissionCooldown", abi = "setSubmissionCooldown(uint256)")]
    pub struct SetSubmissionCooldownCall {
        pub submission_cooldown: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `stakeTokenSelectionWeight` function with signature `stakeTokenSelectionWeight(address)` and selector `0xcb95717a`
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
        name = "stakeTokenSelectionWeight",
        abi = "stakeTokenSelectionWeight(address)"
    )]
    pub struct StakeTokenSelectionWeightCall {
        pub stake_token: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `stakingManager` function with signature `stakingManager()` and selector `0x22828cc2`
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
    #[ethcall(name = "stakingManager", abi = "stakingManager()")]
    pub struct StakingManagerCall;
    ///Container type for all input parameters for the `submissionCooldown` function with signature `submissionCooldown()` and selector `0x9f3d545e`
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
    #[ethcall(name = "submissionCooldown", abi = "submissionCooldown()")]
    pub struct SubmissionCooldownCall;
    ///Container type for all input parameters for the `submissionStatus` function with signature `submissionStatus(uint256,address)` and selector `0x1be8f483`
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
    #[ethcall(name = "submissionStatus", abi = "submissionStatus(uint256,address)")]
    pub struct SubmissionStatusCall {
        pub capture_timestamp: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
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
        pub submission_type: [u8; 32],
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
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
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum SymbioticStakingCalls {
        BridgeEnclaveUpdatesRole(BridgeEnclaveUpdatesRoleCall),
        CompleteMask(CompleteMaskCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        IGeneratorCallback(IGeneratorCallbackCall),
        SignatureLength(SignatureLengthCall),
        SlashResultMask(SlashResultMaskCall),
        SlashResultType(SlashResultTypeCall),
        StakeSnapshotMask(StakeSnapshotMaskCall),
        StakeSnapshotType(StakeSnapshotTypeCall),
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        AddEnclaveImage(AddEnclaveImageCall),
        AddEnclaveImageWithPcr0(AddEnclaveImageWithPcr0Call),
        AddStakeToken(AddStakeTokenCall),
        AmountToLock(AmountToLockCall),
        AttestationVerifier(AttestationVerifierCall),
        BaseTransmitterComissionRate(BaseTransmitterComissionRateCall),
        ConfirmedTimestampInfo(ConfirmedTimestampInfoCall),
        ConfirmedTimestamps(ConfirmedTimestampsCall),
        EmergencyWithdraw(EmergencyWithdrawCall),
        EnclaveImages(EnclaveImagesCall),
        FeeRewardToken(FeeRewardTokenCall),
        GetImageId(GetImageIdCall),
        GetOperatorActiveStakeAmount(GetOperatorActiveStakeAmountCall),
        GetOperatorStakeAmount(GetOperatorStakeAmountCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetStakeAmount(GetStakeAmountCall),
        GetStakeTokenList(GetStakeTokenListCall),
        GetStakeTokenWeights(GetStakeTokenWeightsCall),
        GetSubmissionStatus(GetSubmissionStatusCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Initialize(InitializeCall),
        IsSupportedStakeToken(IsSupportedStakeTokenCall),
        LatestConfirmedTimestamp(LatestConfirmedTimestampCall),
        LatestConfirmedTimestampIdx(LatestConfirmedTimestampIdxCall),
        LatestConfirmedTimestampInfo(LatestConfirmedTimestampInfoCall),
        LockInfo(LockInfoCall),
        LockStake(LockStakeCall),
        OnJobCompletion(OnJobCompletionCall),
        OperatorLockedAmounts(OperatorLockedAmountsCall),
        ProofMarketplace(ProofMarketplaceCall),
        ProxiableUUID(ProxiableUUIDCall),
        RegisteredTransmitters(RegisteredTransmittersCall),
        RemoveEnclaveImage(RemoveEnclaveImageCall),
        RemoveStakeToken(RemoveStakeTokenCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RewardDistributor(RewardDistributorCall),
        SetAmountToLock(SetAmountToLockCall),
        SetAttestationVerifier(SetAttestationVerifierCall),
        SetBaseTransmitterComissionRate(SetBaseTransmitterComissionRateCall),
        SetFeeRewardToken(SetFeeRewardTokenCall),
        SetJobManager(SetJobManagerCall),
        SetRewardDistributor(SetRewardDistributorCall),
        SetStakeTokenSelectionWeight(SetStakeTokenSelectionWeightCall),
        SetStakingManager(SetStakingManagerCall),
        SetSubmissionCooldown(SetSubmissionCooldownCall),
        Slash(SlashCall),
        StakeTokenSelectionWeight(StakeTokenSelectionWeightCall),
        StakeTokenSelectionWeightSum(StakeTokenSelectionWeightSumCall),
        StakingManager(StakingManagerCall),
        SubmissionCooldown(SubmissionCooldownCall),
        SubmissionStatus(SubmissionStatusCall),
        SubmitSlashResult(SubmitSlashResultCall),
        SubmitVaultSnapshot(SubmitVaultSnapshotCall),
        SupportsInterface(SupportsInterfaceCall),
        TxCountInfo(TxCountInfoCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for SymbioticStakingCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BridgeEnclaveUpdatesRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BridgeEnclaveUpdatesRole(decoded));
            }
            if let Ok(decoded) = <CompleteMaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CompleteMask(decoded));
            }
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <IGeneratorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IGeneratorCallback(decoded));
            }
            if let Ok(decoded) =
                <SignatureLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignatureLength(decoded));
            }
            if let Ok(decoded) =
                <SlashResultMaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SlashResultMask(decoded));
            }
            if let Ok(decoded) =
                <SlashResultTypeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SlashResultType(decoded));
            }
            if let Ok(decoded) =
                <StakeSnapshotMaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeSnapshotMask(decoded));
            }
            if let Ok(decoded) =
                <StakeSnapshotTypeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeSnapshotType(decoded));
            }
            if let Ok(decoded) =
                <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) =
                <AddEnclaveImageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddEnclaveImage(decoded));
            }
            if let Ok(decoded) =
                <AddEnclaveImageWithPcr0Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddEnclaveImageWithPcr0(decoded));
            }
            if let Ok(decoded) = <AddStakeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddStakeToken(decoded));
            }
            if let Ok(decoded) = <AmountToLockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AmountToLock(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationVerifier(decoded));
            }
            if let Ok(decoded) =
                <BaseTransmitterComissionRateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BaseTransmitterComissionRate(decoded));
            }
            if let Ok(decoded) =
                <ConfirmedTimestampInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConfirmedTimestampInfo(decoded));
            }
            if let Ok(decoded) =
                <ConfirmedTimestampsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConfirmedTimestamps(decoded));
            }
            if let Ok(decoded) =
                <EmergencyWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmergencyWithdraw(decoded));
            }
            if let Ok(decoded) = <EnclaveImagesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnclaveImages(decoded));
            }
            if let Ok(decoded) =
                <FeeRewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FeeRewardToken(decoded));
            }
            if let Ok(decoded) = <GetImageIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetImageId(decoded));
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
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRoleAdmin(decoded));
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
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
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
                <OperatorLockedAmountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorLockedAmounts(decoded));
            }
            if let Ok(decoded) =
                <ProofMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProofMarketplace(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) =
                <RegisteredTransmittersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisteredTransmitters(decoded));
            }
            if let Ok(decoded) =
                <RemoveEnclaveImageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveEnclaveImage(decoded));
            }
            if let Ok(decoded) =
                <RemoveStakeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveStakeToken(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <RewardDistributorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RewardDistributor(decoded));
            }
            if let Ok(decoded) =
                <SetAmountToLockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAmountToLock(decoded));
            }
            if let Ok(decoded) =
                <SetAttestationVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAttestationVerifier(decoded));
            }
            if let Ok(decoded) =
                <SetBaseTransmitterComissionRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetBaseTransmitterComissionRate(decoded));
            }
            if let Ok(decoded) =
                <SetFeeRewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFeeRewardToken(decoded));
            }
            if let Ok(decoded) = <SetJobManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetJobManager(decoded));
            }
            if let Ok(decoded) =
                <SetRewardDistributorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetRewardDistributor(decoded));
            }
            if let Ok(decoded) =
                <SetStakeTokenSelectionWeightCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetStakeTokenSelectionWeight(decoded));
            }
            if let Ok(decoded) =
                <SetStakingManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetStakingManager(decoded));
            }
            if let Ok(decoded) =
                <SetSubmissionCooldownCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetSubmissionCooldown(decoded));
            }
            if let Ok(decoded) = <SlashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Slash(decoded));
            }
            if let Ok(decoded) =
                <StakeTokenSelectionWeightCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeTokenSelectionWeight(decoded));
            }
            if let Ok(decoded) =
                <StakeTokenSelectionWeightSumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeTokenSelectionWeightSum(decoded));
            }
            if let Ok(decoded) =
                <StakingManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakingManager(decoded));
            }
            if let Ok(decoded) =
                <SubmissionCooldownCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmissionCooldown(decoded));
            }
            if let Ok(decoded) =
                <SubmissionStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmissionStatus(decoded));
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
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <TxCountInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TxCountInfo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SymbioticStakingCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BridgeEnclaveUpdatesRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteMask(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IGeneratorCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignatureLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SlashResultMask(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SlashResultType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeSnapshotMask(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeSnapshotType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddEnclaveImage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddEnclaveImageWithPcr0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddStakeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AmountToLock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AttestationVerifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseTransmitterComissionRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConfirmedTimestampInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConfirmedTimestamps(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmergencyWithdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnclaveImages(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeRewardToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetImageId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorActiveStakeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorStakeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStakeAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStakeTokenList(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStakeTokenWeights(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubmissionStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::OperatorLockedAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofMarketplace(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisteredTransmitters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveEnclaveImage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveStakeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardDistributor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAmountToLock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAttestationVerifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBaseTransmitterComissionRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeRewardToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetJobManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetRewardDistributor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStakeTokenSelectionWeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStakingManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSubmissionCooldown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeTokenSelectionWeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTokenSelectionWeightSum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakingManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmissionCooldown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmissionStatus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitSlashResult(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitVaultSnapshot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TxCountInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeToAndCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SymbioticStakingCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BridgeEnclaveUpdatesRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompleteMask(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IGeneratorCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignatureLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::SlashResultMask(element) => ::core::fmt::Display::fmt(element, f),
                Self::SlashResultType(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeSnapshotMask(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeSnapshotType(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeInterfaceVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddEnclaveImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddEnclaveImageWithPcr0(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStakeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountToLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseTransmitterComissionRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfirmedTimestampInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConfirmedTimestamps(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmergencyWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveImages(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeRewardToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetImageId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorActiveStakeAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorStakeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeTokenList(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeTokenWeights(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubmissionStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSupportedStakeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestConfirmedTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestConfirmedTimestampIdx(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestConfirmedTimestampInfo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LockInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnJobCompletion(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorLockedAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisteredTransmitters(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveEnclaveImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveStakeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardDistributor(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAmountToLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAttestationVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBaseTransmitterComissionRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFeeRewardToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetJobManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRewardDistributor(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStakeTokenSelectionWeight(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetStakingManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSubmissionCooldown(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slash(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenSelectionWeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenSelectionWeightSum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakingManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmissionCooldown(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmissionStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitSlashResult(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitVaultSnapshot(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TxCountInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BridgeEnclaveUpdatesRoleCall> for SymbioticStakingCalls {
        fn from(value: BridgeEnclaveUpdatesRoleCall) -> Self {
            Self::BridgeEnclaveUpdatesRole(value)
        }
    }
    impl ::core::convert::From<CompleteMaskCall> for SymbioticStakingCalls {
        fn from(value: CompleteMaskCall) -> Self {
            Self::CompleteMask(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for SymbioticStakingCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<IGeneratorCallbackCall> for SymbioticStakingCalls {
        fn from(value: IGeneratorCallbackCall) -> Self {
            Self::IGeneratorCallback(value)
        }
    }
    impl ::core::convert::From<SignatureLengthCall> for SymbioticStakingCalls {
        fn from(value: SignatureLengthCall) -> Self {
            Self::SignatureLength(value)
        }
    }
    impl ::core::convert::From<SlashResultMaskCall> for SymbioticStakingCalls {
        fn from(value: SlashResultMaskCall) -> Self {
            Self::SlashResultMask(value)
        }
    }
    impl ::core::convert::From<SlashResultTypeCall> for SymbioticStakingCalls {
        fn from(value: SlashResultTypeCall) -> Self {
            Self::SlashResultType(value)
        }
    }
    impl ::core::convert::From<StakeSnapshotMaskCall> for SymbioticStakingCalls {
        fn from(value: StakeSnapshotMaskCall) -> Self {
            Self::StakeSnapshotMask(value)
        }
    }
    impl ::core::convert::From<StakeSnapshotTypeCall> for SymbioticStakingCalls {
        fn from(value: StakeSnapshotTypeCall) -> Self {
            Self::StakeSnapshotType(value)
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for SymbioticStakingCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<AddEnclaveImageCall> for SymbioticStakingCalls {
        fn from(value: AddEnclaveImageCall) -> Self {
            Self::AddEnclaveImage(value)
        }
    }
    impl ::core::convert::From<AddEnclaveImageWithPcr0Call> for SymbioticStakingCalls {
        fn from(value: AddEnclaveImageWithPcr0Call) -> Self {
            Self::AddEnclaveImageWithPcr0(value)
        }
    }
    impl ::core::convert::From<AddStakeTokenCall> for SymbioticStakingCalls {
        fn from(value: AddStakeTokenCall) -> Self {
            Self::AddStakeToken(value)
        }
    }
    impl ::core::convert::From<AmountToLockCall> for SymbioticStakingCalls {
        fn from(value: AmountToLockCall) -> Self {
            Self::AmountToLock(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierCall> for SymbioticStakingCalls {
        fn from(value: AttestationVerifierCall) -> Self {
            Self::AttestationVerifier(value)
        }
    }
    impl ::core::convert::From<BaseTransmitterComissionRateCall> for SymbioticStakingCalls {
        fn from(value: BaseTransmitterComissionRateCall) -> Self {
            Self::BaseTransmitterComissionRate(value)
        }
    }
    impl ::core::convert::From<ConfirmedTimestampInfoCall> for SymbioticStakingCalls {
        fn from(value: ConfirmedTimestampInfoCall) -> Self {
            Self::ConfirmedTimestampInfo(value)
        }
    }
    impl ::core::convert::From<ConfirmedTimestampsCall> for SymbioticStakingCalls {
        fn from(value: ConfirmedTimestampsCall) -> Self {
            Self::ConfirmedTimestamps(value)
        }
    }
    impl ::core::convert::From<EmergencyWithdrawCall> for SymbioticStakingCalls {
        fn from(value: EmergencyWithdrawCall) -> Self {
            Self::EmergencyWithdraw(value)
        }
    }
    impl ::core::convert::From<EnclaveImagesCall> for SymbioticStakingCalls {
        fn from(value: EnclaveImagesCall) -> Self {
            Self::EnclaveImages(value)
        }
    }
    impl ::core::convert::From<FeeRewardTokenCall> for SymbioticStakingCalls {
        fn from(value: FeeRewardTokenCall) -> Self {
            Self::FeeRewardToken(value)
        }
    }
    impl ::core::convert::From<GetImageIdCall> for SymbioticStakingCalls {
        fn from(value: GetImageIdCall) -> Self {
            Self::GetImageId(value)
        }
    }
    impl ::core::convert::From<GetOperatorActiveStakeAmountCall> for SymbioticStakingCalls {
        fn from(value: GetOperatorActiveStakeAmountCall) -> Self {
            Self::GetOperatorActiveStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetOperatorStakeAmountCall> for SymbioticStakingCalls {
        fn from(value: GetOperatorStakeAmountCall) -> Self {
            Self::GetOperatorStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for SymbioticStakingCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetStakeAmountCall> for SymbioticStakingCalls {
        fn from(value: GetStakeAmountCall) -> Self {
            Self::GetStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetStakeTokenListCall> for SymbioticStakingCalls {
        fn from(value: GetStakeTokenListCall) -> Self {
            Self::GetStakeTokenList(value)
        }
    }
    impl ::core::convert::From<GetStakeTokenWeightsCall> for SymbioticStakingCalls {
        fn from(value: GetStakeTokenWeightsCall) -> Self {
            Self::GetStakeTokenWeights(value)
        }
    }
    impl ::core::convert::From<GetSubmissionStatusCall> for SymbioticStakingCalls {
        fn from(value: GetSubmissionStatusCall) -> Self {
            Self::GetSubmissionStatus(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for SymbioticStakingCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for SymbioticStakingCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for SymbioticStakingCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsSupportedStakeTokenCall> for SymbioticStakingCalls {
        fn from(value: IsSupportedStakeTokenCall) -> Self {
            Self::IsSupportedStakeToken(value)
        }
    }
    impl ::core::convert::From<LatestConfirmedTimestampCall> for SymbioticStakingCalls {
        fn from(value: LatestConfirmedTimestampCall) -> Self {
            Self::LatestConfirmedTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestConfirmedTimestampIdxCall> for SymbioticStakingCalls {
        fn from(value: LatestConfirmedTimestampIdxCall) -> Self {
            Self::LatestConfirmedTimestampIdx(value)
        }
    }
    impl ::core::convert::From<LatestConfirmedTimestampInfoCall> for SymbioticStakingCalls {
        fn from(value: LatestConfirmedTimestampInfoCall) -> Self {
            Self::LatestConfirmedTimestampInfo(value)
        }
    }
    impl ::core::convert::From<LockInfoCall> for SymbioticStakingCalls {
        fn from(value: LockInfoCall) -> Self {
            Self::LockInfo(value)
        }
    }
    impl ::core::convert::From<LockStakeCall> for SymbioticStakingCalls {
        fn from(value: LockStakeCall) -> Self {
            Self::LockStake(value)
        }
    }
    impl ::core::convert::From<OnJobCompletionCall> for SymbioticStakingCalls {
        fn from(value: OnJobCompletionCall) -> Self {
            Self::OnJobCompletion(value)
        }
    }
    impl ::core::convert::From<OperatorLockedAmountsCall> for SymbioticStakingCalls {
        fn from(value: OperatorLockedAmountsCall) -> Self {
            Self::OperatorLockedAmounts(value)
        }
    }
    impl ::core::convert::From<ProofMarketplaceCall> for SymbioticStakingCalls {
        fn from(value: ProofMarketplaceCall) -> Self {
            Self::ProofMarketplace(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for SymbioticStakingCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RegisteredTransmittersCall> for SymbioticStakingCalls {
        fn from(value: RegisteredTransmittersCall) -> Self {
            Self::RegisteredTransmitters(value)
        }
    }
    impl ::core::convert::From<RemoveEnclaveImageCall> for SymbioticStakingCalls {
        fn from(value: RemoveEnclaveImageCall) -> Self {
            Self::RemoveEnclaveImage(value)
        }
    }
    impl ::core::convert::From<RemoveStakeTokenCall> for SymbioticStakingCalls {
        fn from(value: RemoveStakeTokenCall) -> Self {
            Self::RemoveStakeToken(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for SymbioticStakingCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for SymbioticStakingCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RewardDistributorCall> for SymbioticStakingCalls {
        fn from(value: RewardDistributorCall) -> Self {
            Self::RewardDistributor(value)
        }
    }
    impl ::core::convert::From<SetAmountToLockCall> for SymbioticStakingCalls {
        fn from(value: SetAmountToLockCall) -> Self {
            Self::SetAmountToLock(value)
        }
    }
    impl ::core::convert::From<SetAttestationVerifierCall> for SymbioticStakingCalls {
        fn from(value: SetAttestationVerifierCall) -> Self {
            Self::SetAttestationVerifier(value)
        }
    }
    impl ::core::convert::From<SetBaseTransmitterComissionRateCall> for SymbioticStakingCalls {
        fn from(value: SetBaseTransmitterComissionRateCall) -> Self {
            Self::SetBaseTransmitterComissionRate(value)
        }
    }
    impl ::core::convert::From<SetFeeRewardTokenCall> for SymbioticStakingCalls {
        fn from(value: SetFeeRewardTokenCall) -> Self {
            Self::SetFeeRewardToken(value)
        }
    }
    impl ::core::convert::From<SetJobManagerCall> for SymbioticStakingCalls {
        fn from(value: SetJobManagerCall) -> Self {
            Self::SetJobManager(value)
        }
    }
    impl ::core::convert::From<SetRewardDistributorCall> for SymbioticStakingCalls {
        fn from(value: SetRewardDistributorCall) -> Self {
            Self::SetRewardDistributor(value)
        }
    }
    impl ::core::convert::From<SetStakeTokenSelectionWeightCall> for SymbioticStakingCalls {
        fn from(value: SetStakeTokenSelectionWeightCall) -> Self {
            Self::SetStakeTokenSelectionWeight(value)
        }
    }
    impl ::core::convert::From<SetStakingManagerCall> for SymbioticStakingCalls {
        fn from(value: SetStakingManagerCall) -> Self {
            Self::SetStakingManager(value)
        }
    }
    impl ::core::convert::From<SetSubmissionCooldownCall> for SymbioticStakingCalls {
        fn from(value: SetSubmissionCooldownCall) -> Self {
            Self::SetSubmissionCooldown(value)
        }
    }
    impl ::core::convert::From<SlashCall> for SymbioticStakingCalls {
        fn from(value: SlashCall) -> Self {
            Self::Slash(value)
        }
    }
    impl ::core::convert::From<StakeTokenSelectionWeightCall> for SymbioticStakingCalls {
        fn from(value: StakeTokenSelectionWeightCall) -> Self {
            Self::StakeTokenSelectionWeight(value)
        }
    }
    impl ::core::convert::From<StakeTokenSelectionWeightSumCall> for SymbioticStakingCalls {
        fn from(value: StakeTokenSelectionWeightSumCall) -> Self {
            Self::StakeTokenSelectionWeightSum(value)
        }
    }
    impl ::core::convert::From<StakingManagerCall> for SymbioticStakingCalls {
        fn from(value: StakingManagerCall) -> Self {
            Self::StakingManager(value)
        }
    }
    impl ::core::convert::From<SubmissionCooldownCall> for SymbioticStakingCalls {
        fn from(value: SubmissionCooldownCall) -> Self {
            Self::SubmissionCooldown(value)
        }
    }
    impl ::core::convert::From<SubmissionStatusCall> for SymbioticStakingCalls {
        fn from(value: SubmissionStatusCall) -> Self {
            Self::SubmissionStatus(value)
        }
    }
    impl ::core::convert::From<SubmitSlashResultCall> for SymbioticStakingCalls {
        fn from(value: SubmitSlashResultCall) -> Self {
            Self::SubmitSlashResult(value)
        }
    }
    impl ::core::convert::From<SubmitVaultSnapshotCall> for SymbioticStakingCalls {
        fn from(value: SubmitVaultSnapshotCall) -> Self {
            Self::SubmitVaultSnapshot(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for SymbioticStakingCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TxCountInfoCall> for SymbioticStakingCalls {
        fn from(value: TxCountInfoCall) -> Self {
            Self::TxCountInfo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for SymbioticStakingCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    ///Container type for all return fields from the `BRIDGE_ENCLAVE_UPDATES_ROLE` function with signature `BRIDGE_ENCLAVE_UPDATES_ROLE()` and selector `0x36719673`
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
    pub struct BridgeEnclaveUpdatesRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `COMPLETE_MASK` function with signature `COMPLETE_MASK()` and selector `0x1ccb3d66`
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
    pub struct CompleteMaskReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `I_GENERATOR_CALLBACK` function with signature `I_GENERATOR_CALLBACK()` and selector `0x110cbc80`
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
    pub struct IGeneratorCallbackReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `SIGNATURE_LENGTH` function with signature `SIGNATURE_LENGTH()` and selector `0x540bc5ea`
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
    pub struct SignatureLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `SLASH_RESULT_MASK` function with signature `SLASH_RESULT_MASK()` and selector `0xc9061a6a`
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
    pub struct SlashResultMaskReturn(pub [u8; 32]);
    ///Container type for all return fields from the `SLASH_RESULT_TYPE` function with signature `SLASH_RESULT_TYPE()` and selector `0x9b88ee16`
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
    pub struct SlashResultTypeReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKE_SNAPSHOT_MASK` function with signature `STAKE_SNAPSHOT_MASK()` and selector `0xb1b37007`
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
    pub struct StakeSnapshotMaskReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKE_SNAPSHOT_TYPE` function with signature `STAKE_SNAPSHOT_TYPE()` and selector `0x60fba48b`
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
    pub struct StakeSnapshotTypeReturn(pub [u8; 32]);
    ///Container type for all return fields from the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
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
    pub struct UpgradeInterfaceVersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `amountToLock` function with signature `amountToLock(address)` and selector `0x8d3b9d5a`
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
    pub struct AmountToLockReturn {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `attestationVerifier` function with signature `attestationVerifier()` and selector `0xb4b3c5a0`
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
    pub struct AttestationVerifierReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `baseTransmitterComissionRate` function with signature `baseTransmitterComissionRate()` and selector `0xb92c227a`
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
    pub struct BaseTransmitterComissionRateReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `confirmedTimestamps` function with signature `confirmedTimestamps(uint256)` and selector `0x16b71ef4`
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
    pub struct ConfirmedTimestampsReturn {
        pub capture_timestamp: ::ethers::core::types::U256,
        pub transmitter: ::ethers::core::types::Address,
        pub transmitter_comission_rate: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `enclaveImages` function with signature `enclaveImages(bytes32)` and selector `0x8f9c3095`
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
    pub struct EnclaveImagesReturn {
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `feeRewardToken` function with signature `feeRewardToken()` and selector `0x3d74a778`
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
    pub struct FeeRewardTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getImageId` function with signature `getImageId(bytes,bytes,bytes)` and selector `0x9e3a2c9e`
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
    pub struct GetImageIdReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
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
    ///Container type for all return fields from the `operatorLockedAmounts` function with signature `operatorLockedAmounts(address,address)` and selector `0x0a59bb50`
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
    pub struct OperatorLockedAmountsReturn {
        pub locked: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `proofMarketplace` function with signature `proofMarketplace()` and selector `0x81c45c70`
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
    pub struct ProofMarketplaceReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
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
    pub struct RegisteredTransmittersReturn {
        pub transmitter: ::ethers::core::types::Address,
    }
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
    ///Container type for all return fields from the `stakeTokenSelectionWeight` function with signature `stakeTokenSelectionWeight(address)` and selector `0xcb95717a`
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
    pub struct StakeTokenSelectionWeightReturn {
        pub weight: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `stakingManager` function with signature `stakingManager()` and selector `0x22828cc2`
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
    pub struct StakingManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `submissionCooldown` function with signature `submissionCooldown()` and selector `0x9f3d545e`
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
    pub struct SubmissionCooldownReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `submissionStatus` function with signature `submissionStatus(uint256,address)` and selector `0x1be8f483`
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
    pub struct SubmissionStatusReturn {
        pub status: [u8; 32],
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
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
