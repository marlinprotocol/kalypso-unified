pub use staking_manager::*;
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
pub mod staking_manager {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("GENERATOR_REGISTRY_ROLE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("GENERATOR_REGISTRY_ROLE",),
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
                    ::std::borrow::ToOwned::to_owned("addStakingPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addStakingPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_stakingPool"),
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
                    ::std::borrow::ToOwned::to_owned("feeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("feeToken"),
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
                    ::std::borrow::ToOwned::to_owned("getPoolConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPoolConfig"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pool"),
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
                                name: ::std::borrow::ToOwned::to_owned("_symbioticStaking"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_feeToken"),
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
                    ::std::borrow::ToOwned::to_owned("isEnabledPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isEnabledPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pool"),
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
                    ::std::borrow::ToOwned::to_owned("onJobCreation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onJobCreation"),
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
                    ::std::borrow::ToOwned::to_owned("onSlashResult"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onSlashResult"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_jobsSlashed"),
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
                    ::std::borrow::ToOwned::to_owned("removeStakingPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeStakingPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_stakingPool"),
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
                    ::std::borrow::ToOwned::to_owned("setEnabledPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setEnabledPool"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_enabled"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFeeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFeeToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_feeToken"),
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
                    ::std::borrow::ToOwned::to_owned("setPoolRewardShare"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPoolRewardShare"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pools"),
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
                                name: ::std::borrow::ToOwned::to_owned("_shares"),
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
                (
                    ::std::borrow::ToOwned::to_owned("setProofMarketplace"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setProofMarketplace",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_proofMarketplace"),
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
                    ::std::borrow::ToOwned::to_owned("setSymbioticStaking"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSymbioticStaking",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_symbioticStaking"),
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
                    ::std::borrow::ToOwned::to_owned("symbioticStaking"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("symbioticStaking"),
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
    pub static STAKINGMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15`\x13W`\0\x80\xFD[P`\x80Qa\"*a\0=`\09`\0\x81\x81a\x14\xCB\x01R\x81\x81a\x14\xF4\x01Ra\x16:\x01Ra\"*`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xB7W`\x005`\xE0\x1C\x80c\x91\xC5C:\x11a\0\xECW\x80c\xB1$\xED\xA9\x11a\0\x8AW\x80c\xF2\x94\x86\xA1\x11a\0dW\x80c\xF2\x94\x86\xA1\x14a\x05(W\x80c\xF6t\xA5\xEE\x14a\x05\xADW\x80c\xF8\xC8v^\x14a\x05\xCDW\x80c\xFB\xAA\x83\x03\x14a\x05\xEDW`\0\x80\xFD[\x80c\xB1$\xED\xA9\x14a\x04\xC8W\x80c\xC0\xFF\x97\xEF\x14a\x04\xE8W\x80c\xD5Gt\x1F\x14a\x05\x08W`\0\x80\xFD[\x80c\x9B\xBA\x88D\x11a\0\xC6W\x80c\x9B\xBA\x88D\x14a\x04\x17W\x80c\xA1Y{\xCD\x14a\x04TW\x80c\xA2\x17\xFD\xDF\x14a\x04uW\x80c\xAD<\xB1\xCC\x14a\x04\x8AW`\0\x80\xFD[\x80c\x91\xC5C:\x14a\x03\xB7W\x80c\x91\xD1HT\x14a\x03\xD7W\x80c\x95\xC7Q\xD3\x14a\x03\xF7W`\0\x80\xFD[\x80cM\"\xFD1\x11a\x01YW\x80cc\x82\xD9\xAD\x11a\x013W\x80cc\x82\xD9\xAD\x14a\x03\tW\x80cdxF\xA5\x14a\x03)W\x80c\x81\xC4\\p\x14a\x03bW\x80c\x89\xBE\xB5\xF1\x14a\x03\x83W`\0\x80\xFD[\x80cM\"\xFD1\x14a\x02\xC1W\x80cO\x1E\xF2\x86\x14a\x02\xE1W\x80cR\xD1\x90-\x14a\x02\xF4W`\0\x80\xFD[\x80c$\x8A\x9C\xA3\x11a\x01\x95W\x80c$\x8A\x9C\xA3\x14a\x023W\x80c//\xF1]\x14a\x02aW\x80c6V\x8A\xBE\x14a\x02\x81W\x80cAh'\xDF\x14a\x02\xA1W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xBCW\x80c\x15\xCC\xE2$\x14a\x01\xF1W\x80c\x1E\x9A\xB7T\x14a\x02\x13W[`\0\x80\xFD[4\x80\x15a\x01\xC8W`\0\x80\xFD[Pa\x01\xDCa\x01\xD76`\x04a\x1C\x18V[a\x06\rV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xFDW`\0\x80\xFD[Pa\x02\x11a\x02\x0C6`\x04a\x1C^V[a\x06\x1EV[\0[4\x80\x15a\x02\x1FW`\0\x80\xFD[Pa\x02\x11a\x02.6`\x04a\x1C^V[a\x06uV[4\x80\x15a\x02?W`\0\x80\xFD[Pa\x02Sa\x02N6`\x04a\x1CyV[a\x06\xDEV[`@Q\x90\x81R` \x01a\x01\xE8V[4\x80\x15a\x02mW`\0\x80\xFD[Pa\x02\x11a\x02|6`\x04a\x1C\x92V[a\x07\0V[4\x80\x15a\x02\x8DW`\0\x80\xFD[Pa\x02\x11a\x02\x9C6`\x04a\x1C\x92V[a\x07\"V[4\x80\x15a\x02\xADW`\0\x80\xFD[Pa\x02\x11a\x02\xBC6`\x04a\x1C^V[a\x07ZV[4\x80\x15a\x02\xCDW`\0\x80\xFD[Pa\x02\x11a\x02\xDC6`\x04a\x1C\xBEV[a\x07\xB1V[a\x02\x11a\x02\xEF6`\x04a\x1DKV[a\t\xA5V[4\x80\x15a\x03\0W`\0\x80\xFD[Pa\x02Sa\t\xC4V[4\x80\x15a\x03\x15W`\0\x80\xFD[Pa\x02\x11a\x03$6`\x04a\x1E\x15V[a\t\xE1V[4\x80\x15a\x035W`\0\x80\xFD[Pa\x01\xF8Ta\x03J\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE8V[4\x80\x15a\x03nW`\0\x80\xFD[Pa\x01\xF6Ta\x03J\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03\x8FW`\0\x80\xFD[Pa\x02S\x7F\xA5\xCC-=\xBFh\xC9\x12\x82\xF3qq\xEB5\xEC\x15\xB16z\xBC\xD2\xCEC\x88\x14Met\xA6P\xF6\xD8\x81V[4\x80\x15a\x03\xC3W`\0\x80\xFD[Pa\x02\x11a\x03\xD26`\x04a\x1E\x8BV[a\n\xFEV[4\x80\x15a\x03\xE3W`\0\x80\xFD[Pa\x01\xDCa\x03\xF26`\x04a\x1C\x92V[a\x0C\x98V[4\x80\x15a\x04\x03W`\0\x80\xFD[Pa\x02\x11a\x04\x126`\x04a\x1E\xFCV[a\x0C\xD0V[4\x80\x15a\x04#W`\0\x80\xFD[Pa\x01\xDCa\x0426`\x04a\x1C^V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81Ra\x03\xED` R`@\x90 `\x01\x01T`\xFF\x16\x90V[4\x80\x15a\x04`W`\0\x80\xFD[Pa\x01\xF7Ta\x03J\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\x81W`\0\x80\xFD[Pa\x02S`\0\x81V[4\x80\x15a\x04\x96W`\0\x80\xFD[Pa\x04\xBB`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01\xE8\x91\x90a\x1F\\V[4\x80\x15a\x04\xD4W`\0\x80\xFD[Pa\x02\x11a\x04\xE36`\x04a\x1C^V[a\r\x97V[4\x80\x15a\x04\xF4W`\0\x80\xFD[Pa\x02\x11a\x05\x036`\x04a\x1F\x8FV[a\r\xE7V[4\x80\x15a\x05\x14W`\0\x80\xFD[Pa\x02\x11a\x05#6`\x04a\x1C\x92V[a\x0E\xF7V[4\x80\x15a\x054W`\0\x80\xFD[Pa\x05\x90a\x05C6`\x04a\x1C^V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81Ra\x03\xED` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T`\xFF\x16\x15\x15\x90\x82\x01R\x90V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x15\x15\x92\x81\x01\x92\x90\x92R\x01a\x01\xE8V[4\x80\x15a\x05\xB9W`\0\x80\xFD[Pa\x02\x11a\x05\xC86`\x04a\x1C\x92V[a\x0F\x13V[4\x80\x15a\x05\xD9W`\0\x80\xFD[Pa\x02\x11a\x05\xE86`\x04a\x1F\xC5V[a\x10\nV[4\x80\x15a\x05\xF9W`\0\x80\xFD[Pa\x02\x11a\x06\x086`\x04a\x1C^V[a\x12\xB2V[`\0a\x06\x18\x82a\x13\tV[\x92\x91PPV[`\0a\x06)\x81a\x13>V[a\x01\xF8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7Fr/\xF8L\x124\xB2H a\xDE\xF5\xC8,kP\x80\xC1\x17\xB3\xCB\xB6\x9DhhD\xA0Q\xE4\xB8\xE7\xF3\x90`\0\x90\xA2PPV[`\0a\x06\x80\x81a\x13>V[a\x06\x8Ca\x01\xF4\x83a\x13KV[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x03\xED` R`@\x80\x82 \x82\x81U`\x01\x01\x80T`\xFF\x19\x16\x90UQ\x7FFi\x1Cp:%\xCEN\x92&\xF3\x12+\x98\xEF\xF2\xD44_\xFD!\xF5)\x8D\x1D\xCB\xD8\xAA\xCE\xC2\xEB\x8F\x91\x90\xA2PPV[`\0\x90\x81R`\0\x80Q` a!\xD5\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[a\x07\t\x82a\x06\xDEV[a\x07\x12\x81a\x13>V[a\x07\x1C\x83\x83a\x13gV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x07KW`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07U\x82\x82a\x14\x0CV[PPPV[`\0a\x07e\x81a\x13>V[a\x01\xF7\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F@~=\x05\x80D\x90\x13T\xA6/\x01/\xEB\xECtPeQE\xB6\x9F\xD1\xD8,\x1B\x11\xEF\xC0\x9D\xF0\xB6\x90`\0\x90\xA2PPV[a\x01\xF7T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FStakingManager: Only SymbioticSt`D\x82\x01Rdaking`\xD8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x08+a\x01\xF43a\x14\x88V[a\x08wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FStakingManager: Invalid Pool\0\0\0\0`D\x82\x01R`d\x01a\x08\x16V[`\0[\x81\x81\x10\x15a\t\x0CWa\x01\xF6T`\x01`\x01`\xA0\x1B\x03\x16c\xFC\xEA=u\x84\x84\x84\x81\x81\x10a\x08\xA6Wa\x08\xA6a \x19V[\x90P``\x02\x01`\0\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xCE\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xFCW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x08z\x90PV[P`\0a\t\x1Aa\x01\xF4a\x14\xAAV[\x90P`\0[\x81\x81\x10\x15a\x07\x1CW`\0a\t5a\x01\xF4\x83a\x14\xB4V[`@Qcot^q`\xE1\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xDE\xE8\xBC\xE2\x90a\tf\x90\x88\x90\x88\x90`\x04\x01a /V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x80W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x94W=`\0\x80>=`\0\xFD[PP`\x01\x90\x93\x01\x92Pa\t\x1F\x91PPV[a\t\xADa\x14\xC0V[a\t\xB6\x82a\x15gV[a\t\xC0\x82\x82a\x15rV[PPV[`\0a\t\xCEa\x16/V[P`\0\x80Q` a!\xB5\x839\x81Q\x91R\x90V[`\0a\t\xEC\x81a\x13>V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\n7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqzero token address`p\x1B`D\x82\x01R`d\x01a\x08\x16V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rnzero to address`\x88\x1B`D\x82\x01R`d\x01a\x08\x16V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x07U\x90\x83\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xED\x91\x90a \x9EV[`\x01`\x01`\xA0\x1B\x03\x86\x16\x91\x90a\x16xV[`\0a\x0B\t\x81a\x13>V[\x83\x82\x14\x80a\x0B WPa\x0B\x1Da\x01\xF4a\x14\xAAV[\x84\x14[a\x0B]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\t-\xCE\xCC-\x8D,\x84\t\x8C\xAD\xCC\xEE\x8D`\x93\x1B`D\x82\x01R`d\x01a\x08\x16V[`\0\x80[\x83\x81\x10\x15a\x0B\xF6W\x84\x84\x82\x81\x81\x10a\x0B{Wa\x0B{a \x19V[\x90P` \x02\x015a\x03\xED`\0\x89\x89\x85\x81\x81\x10a\x0B\x99Wa\x0B\x99a \x19V[\x90P` \x02\x01` \x81\x01\x90a\x0B\xAE\x91\x90a\x1C^V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 U\x84\x84\x82\x81\x81\x10a\x0B\xDAWa\x0B\xDAa \x19V[\x90P` \x02\x015\x82a\x0B\xEC\x91\x90a \xCDV[\x91P`\x01\x01a\x0BaV[P\x80g\r\xE0\xB6\xB3\xA7d\0\0\x14a\x0C?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmInvalid Shares`\x90\x1B`D\x82\x01R`d\x01a\x08\x16V[\x85\x85`@Qa\x0CO\x92\x91\x90a \xE0V[`@Q\x80\x91\x03\x90 \x7F7/\x96\xE8\\\xD2\x7F\x93\xF5\x15Y\x19\x18\xB4\x98\xD8P\xBAD\xFD\x1A\xA7\xC834n}\xE6\xE4\xD4D\x80\x85\x85`@Qa\x0C\x88\x92\x91\x90a! V[`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0\x91\x82R`\0\x80Q` a!\xD5\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x0C\xDB\x81a\x13>V[a\x0C\xE7a\x01\xF4\x84a\x14\x88V[a\r3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FStakingManager: Pool not in set\0`D\x82\x01R`d\x01a\x08\x16V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03\xED` \x90\x81R`@\x91\x82\x90 `\x01\x01\x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F$\x94\x94\xD95\x817z\x80\xBE@\xF4.\xA0J\xE4\xF3\x1E\xCE\xA3\xD7\0\x08'\x08N\x958\x19(Ds\x91\x01`@Q\x80\x91\x03\x90\xA2PPPV[`\0a\r\xA2\x81a\x13>V[a\r\xAEa\x01\xF4\x83a\x16\xCAV[P`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xFD\xC4M\xE6K\x861/VE\x89\x962\xFE\0=\xA9\xA6r\xB2\xEB\x10@\0\xE9D\x80O\x15\x03\xE0\x99\x90`\0\x90\xA2PPV[\x7F\xA5\xCC-=\xBFh\xC9\x12\x82\xF3qq\xEB5\xEC\x15\xB16z\xBC\xD2\xCEC\x88\x14Met\xA6P\xF6\xD8a\x0E\x11\x81a\x13>V[`\0a\x0E\x1Ea\x01\xF4a\x14\xAAV[\x90P`\0[\x81\x81\x10\x15a\x0E\xEFW`\0a\x0E9a\x01\xF4\x83a\x14\xB4V[\x90Pa\x0Ea\x81`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81Ra\x03\xED` R`@\x90 `\x01\x01T`\xFF\x16\x90V[a\x0EkWPa\x0E\xE7V[`\0a\x0Ew\x82\x87a\x16\xDFV[`@Qc\xC0\xFF\x97\xEF`\xE0\x1B\x81R`\x04\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R\x91\x92P\x90\x83\x16\x90c\xC0\xFF\x97\xEF\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xCCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xE0W=`\0\x80>=`\0\xFD[PPPPPP[`\x01\x01a\x0E#V[PPPPPPV[a\x0F\0\x82a\x06\xDEV[a\x0F\t\x81a\x13>V[a\x07\x1C\x83\x83a\x14\x0CV[\x7F\xA5\xCC-=\xBFh\xC9\x12\x82\xF3qq\xEB5\xEC\x15\xB16z\xBC\xD2\xCEC\x88\x14Met\xA6P\xF6\xD8a\x0F=\x81a\x13>V[`\0a\x0FJa\x01\xF4a\x14\xAAV[\x90P`\0[\x81\x81\x10\x15a\x10\x03W`\0a\x0Fea\x01\xF4\x83a\x14\xB4V[\x90Pa\x0F\x8D\x81`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81Ra\x03\xED` R`@\x90 `\x01\x01T`\xFF\x16\x90V[a\x0F\x97WPa\x0F\xFBV[`@Qc3\xF4\x90\x9F`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`$\x83\x01R\x82\x16\x90c3\xF4\x90\x9F\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xF5W=`\0\x80>=`\0\xFD[PPPPP[`\x01\x01a\x0FOV[PPPPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15\x80\x15a\x10PWP\x82[\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x10mWP0;\x15[\x90P\x81\x15\x80\x15a\x10{WP\x80\x15[\x15a\x10\x99W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x10\xC3W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x10\xCBa\x17\"V[a\x10\xD3a\x17\"V[a\x10\xDBa\x17\"V[a\x10\xE3a\x17\"V[a\x10\xEE`\0\x8Aa\x13gV[P`\x01`\x01`\xA0\x1B\x03\x88\x16a\x11VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FStakingManager: Invalid ProofMar`D\x82\x01Rgketplace`\xC0\x1B`d\x82\x01R`\x84\x01a\x08\x16V[a\x01\xF6\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x91\x90\x91\x17\x90\x91U\x86\x16a\x11\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStakingManager: Invalid FeeToken`D\x82\x01R`d\x01a\x08\x16V[a\x01\xF8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x91\x90\x91\x17\x90\x91U\x87\x16a\x12EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FStakingManager: Invalid Symbioti`D\x82\x01RgcStaking`\xC0\x1B`d\x82\x01R`\x84\x01a\x08\x16V[a\x01\xF7\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x16\x17\x90U\x83\x15a\x12\xA7W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[`\0a\x12\xBD\x81a\x13>V[a\x01\xF6\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F\x89 \xA3\x12\x98y\xB1\xE5L\x92Tjt\x8BbI\xBA/\x9A\x03\x83\xF4I\x9D\x9FJ\xE4;\x90\x9A\xD7\x95\x90`\0\x90\xA2PPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06\x18WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x06\x18V[a\x13H\x813a\x17*V[PV[`\0a\x13`\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x17cV[\x93\x92PPPV[`\0`\0\x80Q` a!\xD5\x839\x81Q\x91Ra\x13\x82\x84\x84a\x0C\x98V[a\x14\x02W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x13\xB83\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\x06\x18V[`\0\x91PPa\x06\x18V[`\0`\0\x80Q` a!\xD5\x839\x81Q\x91Ra\x14'\x84\x84a\x0C\x98V[\x15a\x14\x02W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\x06\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x13`V[`\0a\x06\x18\x82T\x90V[`\0a\x13`\x83\x83a\x18LV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x15GWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x15;`\0\x80Q` a!\xB5\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x15eW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0a\t\xC0\x81a\x13>V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x15\xCCWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x15\xC9\x91\x81\x01\x90a \x9EV[`\x01[a\x15\xF4W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x08\x16V[`\0\x80Q` a!\xB5\x839\x81Q\x91R\x81\x14a\x16%W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08\x16V[a\x07U\x83\x83a\x18vV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x15eW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x07U\x90\x84\x90a\x18\xCCV[`\0a\x13`\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x19=V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03\xED` R`@\x81 T\x81\x83a\x17\x06W`\0a\x17\x19V[a\x17\x19\x84\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x19\x8CV[\x95\x94PPPPPV[a\x15ea\x1AGV[a\x174\x82\x82a\x0C\x98V[a\t\xC0W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x08\x16V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x14\x02W`\0a\x17\x87`\x01\x83a!YV[\x85T\x90\x91P`\0\x90a\x17\x9B\x90`\x01\x90a!YV[\x90P\x80\x82\x14a\x18\0W`\0\x86`\0\x01\x82\x81T\x81\x10a\x17\xBBWa\x17\xBBa \x19V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x17\xDEWa\x17\xDEa \x19V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x18\x11Wa\x18\x11a!lV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x06\x18V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x18cWa\x18ca \x19V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[a\x18\x7F\x82a\x1A\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\x18\xC4Wa\x07U\x82\x82a\x1A\xF5V[a\t\xC0a\x1BbV[`\0\x80` `\0\x84Q` \x86\x01`\0\x88Z\xF1\x80a\x18\xEFW`@Q=`\0\x82>=\x81\xFD[PP`\0Q=\x91P\x81\x15a\x19\x07W\x80`\x01\x14\x15a\x19\x14V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x07\x1CW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x08\x16V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x19\x84WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x06\x18V[P`\0a\x06\x18V[`\0\x83\x83\x02\x81`\0\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x19\xC3W\x83\x82\x81a\x19\xB9Wa\x19\xB9a!\x82V[\x04\x92PPPa\x13`V[\x80\x84\x11a\x19\xDAWa\x19\xDA`\x03\x85\x15\x02`\x11\x18a\x1B\x81V[`\0\x84\x86\x88\t`\0\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x15eW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x1A\xC6W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x08\x16V[`\0\x80Q` a!\xB5\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1B\x12\x91\x90a!\x98V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1BMW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1BRV[``\x91P[P\x91P\x91Pa\x17\x19\x85\x83\x83a\x1B\x93V[4\x15a\x15eW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[cNH{q`\0R\x80` R`$`\x1C\xFD[``\x82a\x1B\xA8Wa\x1B\xA3\x82a\x1B\xEFV[a\x13`V[\x81Q\x15\x80\x15a\x1B\xBFWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1B\xE8W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x08\x16V[P\x80a\x13`V[\x80Q\x15a\x1B\xFFW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a\x1C*W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x13`W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1CYW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1CpW`\0\x80\xFD[a\x13`\x82a\x1CBV[`\0` \x82\x84\x03\x12\x15a\x1C\x8BW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xA5W`\0\x80\xFD[\x825\x91Pa\x1C\xB5` \x84\x01a\x1CBV[\x90P\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x1C\xD1W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xE8W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x1C\xF9W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x10W`\0\x80\xFD[\x85` ``\x83\x02\x84\x01\x01\x11\x15a\x1D%W`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1D^W`\0\x80\xFD[a\x1Dg\x83a\x1CBV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x83W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x1D\x94W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xAEWa\x1D\xAEa\x1D5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xDDWa\x1D\xDDa\x1D5V[`@R\x81\x81R\x82\x82\x01` \x01\x87\x10\x15a\x1D\xF5W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E(W`\0\x80\xFD[a\x1E1\x83a\x1CBV[\x91Pa\x1C\xB5` \x84\x01a\x1CBV[`\0\x80\x83`\x1F\x84\x01\x12a\x1EQW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1EiW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E\x84W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x1E\xA1W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xB8W`\0\x80\xFD[a\x1E\xC4\x87\x82\x88\x01a\x1E?V[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xE4W`\0\x80\xFD[a\x1E\xF0\x87\x82\x88\x01a\x1E?V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\x0FW`\0\x80\xFD[a\x1F\x18\x83a\x1CBV[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x1F-W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x1FSW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1F;V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x1F{\x81`@\x85\x01` \x87\x01a\x1F8V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1F\xA4W`\0\x80\xFD[\x835\x92Pa\x1F\xB4` \x85\x01a\x1CBV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1F\xDBW`\0\x80\xFD[a\x1F\xE4\x85a\x1CBV[\x93Pa\x1F\xF2` \x86\x01a\x1CBV[\x92Pa \0`@\x86\x01a\x1CBV[\x91Pa \x0E``\x86\x01a\x1CBV[\x90P\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R\x81\x01\x82\x90R`\0\x83`@\x83\x01\x82[\x85\x81\x10\x15a \x94W\x825\x82R`\x01`\x01`\xA0\x1B\x03a b` \x85\x01a\x1CBV[\x16` \x83\x01R`\x01`\x01`\xA0\x1B\x03a |`@\x85\x01a\x1CBV[\x16`@\x83\x01R``\x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a BV[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a \xB0W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\x18Wa\x06\x18a \xB7V[`\0\x81\x84\x82[\x85\x81\x10\x15a!\x15W`\x01`\x01`\xA0\x1B\x03a \xFF\x83a\x1CBV[\x16\x83R` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a \xE6V[P\x90\x95\x94PPPPPV[` \x80\x82R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a!@W`\0\x80\xFD[\x82`\x05\x1B\x80\x85`@\x85\x017\x91\x90\x91\x01`@\x01\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x06\x18Wa\x06\x18a \xB7V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82Qa!\xAA\x81\x84` \x87\x01a\x1F8V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\xA2dipfsX\"\x12 \xC0\xB7\x16\xCA\xD7u\\C\x9F7;\x1E\xE5F\xD2\x8B<\xED\xF7\xF1:&\xAD\xFEGlh\x8F'\xF2\x8D\x10dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static STAKINGMANAGER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xB7W`\x005`\xE0\x1C\x80c\x91\xC5C:\x11a\0\xECW\x80c\xB1$\xED\xA9\x11a\0\x8AW\x80c\xF2\x94\x86\xA1\x11a\0dW\x80c\xF2\x94\x86\xA1\x14a\x05(W\x80c\xF6t\xA5\xEE\x14a\x05\xADW\x80c\xF8\xC8v^\x14a\x05\xCDW\x80c\xFB\xAA\x83\x03\x14a\x05\xEDW`\0\x80\xFD[\x80c\xB1$\xED\xA9\x14a\x04\xC8W\x80c\xC0\xFF\x97\xEF\x14a\x04\xE8W\x80c\xD5Gt\x1F\x14a\x05\x08W`\0\x80\xFD[\x80c\x9B\xBA\x88D\x11a\0\xC6W\x80c\x9B\xBA\x88D\x14a\x04\x17W\x80c\xA1Y{\xCD\x14a\x04TW\x80c\xA2\x17\xFD\xDF\x14a\x04uW\x80c\xAD<\xB1\xCC\x14a\x04\x8AW`\0\x80\xFD[\x80c\x91\xC5C:\x14a\x03\xB7W\x80c\x91\xD1HT\x14a\x03\xD7W\x80c\x95\xC7Q\xD3\x14a\x03\xF7W`\0\x80\xFD[\x80cM\"\xFD1\x11a\x01YW\x80cc\x82\xD9\xAD\x11a\x013W\x80cc\x82\xD9\xAD\x14a\x03\tW\x80cdxF\xA5\x14a\x03)W\x80c\x81\xC4\\p\x14a\x03bW\x80c\x89\xBE\xB5\xF1\x14a\x03\x83W`\0\x80\xFD[\x80cM\"\xFD1\x14a\x02\xC1W\x80cO\x1E\xF2\x86\x14a\x02\xE1W\x80cR\xD1\x90-\x14a\x02\xF4W`\0\x80\xFD[\x80c$\x8A\x9C\xA3\x11a\x01\x95W\x80c$\x8A\x9C\xA3\x14a\x023W\x80c//\xF1]\x14a\x02aW\x80c6V\x8A\xBE\x14a\x02\x81W\x80cAh'\xDF\x14a\x02\xA1W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xBCW\x80c\x15\xCC\xE2$\x14a\x01\xF1W\x80c\x1E\x9A\xB7T\x14a\x02\x13W[`\0\x80\xFD[4\x80\x15a\x01\xC8W`\0\x80\xFD[Pa\x01\xDCa\x01\xD76`\x04a\x1C\x18V[a\x06\rV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xFDW`\0\x80\xFD[Pa\x02\x11a\x02\x0C6`\x04a\x1C^V[a\x06\x1EV[\0[4\x80\x15a\x02\x1FW`\0\x80\xFD[Pa\x02\x11a\x02.6`\x04a\x1C^V[a\x06uV[4\x80\x15a\x02?W`\0\x80\xFD[Pa\x02Sa\x02N6`\x04a\x1CyV[a\x06\xDEV[`@Q\x90\x81R` \x01a\x01\xE8V[4\x80\x15a\x02mW`\0\x80\xFD[Pa\x02\x11a\x02|6`\x04a\x1C\x92V[a\x07\0V[4\x80\x15a\x02\x8DW`\0\x80\xFD[Pa\x02\x11a\x02\x9C6`\x04a\x1C\x92V[a\x07\"V[4\x80\x15a\x02\xADW`\0\x80\xFD[Pa\x02\x11a\x02\xBC6`\x04a\x1C^V[a\x07ZV[4\x80\x15a\x02\xCDW`\0\x80\xFD[Pa\x02\x11a\x02\xDC6`\x04a\x1C\xBEV[a\x07\xB1V[a\x02\x11a\x02\xEF6`\x04a\x1DKV[a\t\xA5V[4\x80\x15a\x03\0W`\0\x80\xFD[Pa\x02Sa\t\xC4V[4\x80\x15a\x03\x15W`\0\x80\xFD[Pa\x02\x11a\x03$6`\x04a\x1E\x15V[a\t\xE1V[4\x80\x15a\x035W`\0\x80\xFD[Pa\x01\xF8Ta\x03J\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE8V[4\x80\x15a\x03nW`\0\x80\xFD[Pa\x01\xF6Ta\x03J\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03\x8FW`\0\x80\xFD[Pa\x02S\x7F\xA5\xCC-=\xBFh\xC9\x12\x82\xF3qq\xEB5\xEC\x15\xB16z\xBC\xD2\xCEC\x88\x14Met\xA6P\xF6\xD8\x81V[4\x80\x15a\x03\xC3W`\0\x80\xFD[Pa\x02\x11a\x03\xD26`\x04a\x1E\x8BV[a\n\xFEV[4\x80\x15a\x03\xE3W`\0\x80\xFD[Pa\x01\xDCa\x03\xF26`\x04a\x1C\x92V[a\x0C\x98V[4\x80\x15a\x04\x03W`\0\x80\xFD[Pa\x02\x11a\x04\x126`\x04a\x1E\xFCV[a\x0C\xD0V[4\x80\x15a\x04#W`\0\x80\xFD[Pa\x01\xDCa\x0426`\x04a\x1C^V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81Ra\x03\xED` R`@\x90 `\x01\x01T`\xFF\x16\x90V[4\x80\x15a\x04`W`\0\x80\xFD[Pa\x01\xF7Ta\x03J\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\x81W`\0\x80\xFD[Pa\x02S`\0\x81V[4\x80\x15a\x04\x96W`\0\x80\xFD[Pa\x04\xBB`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01\xE8\x91\x90a\x1F\\V[4\x80\x15a\x04\xD4W`\0\x80\xFD[Pa\x02\x11a\x04\xE36`\x04a\x1C^V[a\r\x97V[4\x80\x15a\x04\xF4W`\0\x80\xFD[Pa\x02\x11a\x05\x036`\x04a\x1F\x8FV[a\r\xE7V[4\x80\x15a\x05\x14W`\0\x80\xFD[Pa\x02\x11a\x05#6`\x04a\x1C\x92V[a\x0E\xF7V[4\x80\x15a\x054W`\0\x80\xFD[Pa\x05\x90a\x05C6`\x04a\x1C^V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81Ra\x03\xED` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T`\xFF\x16\x15\x15\x90\x82\x01R\x90V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x15\x15\x92\x81\x01\x92\x90\x92R\x01a\x01\xE8V[4\x80\x15a\x05\xB9W`\0\x80\xFD[Pa\x02\x11a\x05\xC86`\x04a\x1C\x92V[a\x0F\x13V[4\x80\x15a\x05\xD9W`\0\x80\xFD[Pa\x02\x11a\x05\xE86`\x04a\x1F\xC5V[a\x10\nV[4\x80\x15a\x05\xF9W`\0\x80\xFD[Pa\x02\x11a\x06\x086`\x04a\x1C^V[a\x12\xB2V[`\0a\x06\x18\x82a\x13\tV[\x92\x91PPV[`\0a\x06)\x81a\x13>V[a\x01\xF8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7Fr/\xF8L\x124\xB2H a\xDE\xF5\xC8,kP\x80\xC1\x17\xB3\xCB\xB6\x9DhhD\xA0Q\xE4\xB8\xE7\xF3\x90`\0\x90\xA2PPV[`\0a\x06\x80\x81a\x13>V[a\x06\x8Ca\x01\xF4\x83a\x13KV[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x03\xED` R`@\x80\x82 \x82\x81U`\x01\x01\x80T`\xFF\x19\x16\x90UQ\x7FFi\x1Cp:%\xCEN\x92&\xF3\x12+\x98\xEF\xF2\xD44_\xFD!\xF5)\x8D\x1D\xCB\xD8\xAA\xCE\xC2\xEB\x8F\x91\x90\xA2PPV[`\0\x90\x81R`\0\x80Q` a!\xD5\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[a\x07\t\x82a\x06\xDEV[a\x07\x12\x81a\x13>V[a\x07\x1C\x83\x83a\x13gV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x07KW`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07U\x82\x82a\x14\x0CV[PPPV[`\0a\x07e\x81a\x13>V[a\x01\xF7\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F@~=\x05\x80D\x90\x13T\xA6/\x01/\xEB\xECtPeQE\xB6\x9F\xD1\xD8,\x1B\x11\xEF\xC0\x9D\xF0\xB6\x90`\0\x90\xA2PPV[a\x01\xF7T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FStakingManager: Only SymbioticSt`D\x82\x01Rdaking`\xD8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x08+a\x01\xF43a\x14\x88V[a\x08wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FStakingManager: Invalid Pool\0\0\0\0`D\x82\x01R`d\x01a\x08\x16V[`\0[\x81\x81\x10\x15a\t\x0CWa\x01\xF6T`\x01`\x01`\xA0\x1B\x03\x16c\xFC\xEA=u\x84\x84\x84\x81\x81\x10a\x08\xA6Wa\x08\xA6a \x19V[\x90P``\x02\x01`\0\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xCE\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xFCW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x08z\x90PV[P`\0a\t\x1Aa\x01\xF4a\x14\xAAV[\x90P`\0[\x81\x81\x10\x15a\x07\x1CW`\0a\t5a\x01\xF4\x83a\x14\xB4V[`@Qcot^q`\xE1\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xDE\xE8\xBC\xE2\x90a\tf\x90\x88\x90\x88\x90`\x04\x01a /V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x80W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x94W=`\0\x80>=`\0\xFD[PP`\x01\x90\x93\x01\x92Pa\t\x1F\x91PPV[a\t\xADa\x14\xC0V[a\t\xB6\x82a\x15gV[a\t\xC0\x82\x82a\x15rV[PPV[`\0a\t\xCEa\x16/V[P`\0\x80Q` a!\xB5\x839\x81Q\x91R\x90V[`\0a\t\xEC\x81a\x13>V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\n7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqzero token address`p\x1B`D\x82\x01R`d\x01a\x08\x16V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rnzero to address`\x88\x1B`D\x82\x01R`d\x01a\x08\x16V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x07U\x90\x83\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xED\x91\x90a \x9EV[`\x01`\x01`\xA0\x1B\x03\x86\x16\x91\x90a\x16xV[`\0a\x0B\t\x81a\x13>V[\x83\x82\x14\x80a\x0B WPa\x0B\x1Da\x01\xF4a\x14\xAAV[\x84\x14[a\x0B]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\t-\xCE\xCC-\x8D,\x84\t\x8C\xAD\xCC\xEE\x8D`\x93\x1B`D\x82\x01R`d\x01a\x08\x16V[`\0\x80[\x83\x81\x10\x15a\x0B\xF6W\x84\x84\x82\x81\x81\x10a\x0B{Wa\x0B{a \x19V[\x90P` \x02\x015a\x03\xED`\0\x89\x89\x85\x81\x81\x10a\x0B\x99Wa\x0B\x99a \x19V[\x90P` \x02\x01` \x81\x01\x90a\x0B\xAE\x91\x90a\x1C^V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 U\x84\x84\x82\x81\x81\x10a\x0B\xDAWa\x0B\xDAa \x19V[\x90P` \x02\x015\x82a\x0B\xEC\x91\x90a \xCDV[\x91P`\x01\x01a\x0BaV[P\x80g\r\xE0\xB6\xB3\xA7d\0\0\x14a\x0C?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmInvalid Shares`\x90\x1B`D\x82\x01R`d\x01a\x08\x16V[\x85\x85`@Qa\x0CO\x92\x91\x90a \xE0V[`@Q\x80\x91\x03\x90 \x7F7/\x96\xE8\\\xD2\x7F\x93\xF5\x15Y\x19\x18\xB4\x98\xD8P\xBAD\xFD\x1A\xA7\xC834n}\xE6\xE4\xD4D\x80\x85\x85`@Qa\x0C\x88\x92\x91\x90a! V[`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0\x91\x82R`\0\x80Q` a!\xD5\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x0C\xDB\x81a\x13>V[a\x0C\xE7a\x01\xF4\x84a\x14\x88V[a\r3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FStakingManager: Pool not in set\0`D\x82\x01R`d\x01a\x08\x16V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03\xED` \x90\x81R`@\x91\x82\x90 `\x01\x01\x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F$\x94\x94\xD95\x817z\x80\xBE@\xF4.\xA0J\xE4\xF3\x1E\xCE\xA3\xD7\0\x08'\x08N\x958\x19(Ds\x91\x01`@Q\x80\x91\x03\x90\xA2PPPV[`\0a\r\xA2\x81a\x13>V[a\r\xAEa\x01\xF4\x83a\x16\xCAV[P`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xFD\xC4M\xE6K\x861/VE\x89\x962\xFE\0=\xA9\xA6r\xB2\xEB\x10@\0\xE9D\x80O\x15\x03\xE0\x99\x90`\0\x90\xA2PPV[\x7F\xA5\xCC-=\xBFh\xC9\x12\x82\xF3qq\xEB5\xEC\x15\xB16z\xBC\xD2\xCEC\x88\x14Met\xA6P\xF6\xD8a\x0E\x11\x81a\x13>V[`\0a\x0E\x1Ea\x01\xF4a\x14\xAAV[\x90P`\0[\x81\x81\x10\x15a\x0E\xEFW`\0a\x0E9a\x01\xF4\x83a\x14\xB4V[\x90Pa\x0Ea\x81`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81Ra\x03\xED` R`@\x90 `\x01\x01T`\xFF\x16\x90V[a\x0EkWPa\x0E\xE7V[`\0a\x0Ew\x82\x87a\x16\xDFV[`@Qc\xC0\xFF\x97\xEF`\xE0\x1B\x81R`\x04\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R\x91\x92P\x90\x83\x16\x90c\xC0\xFF\x97\xEF\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xCCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xE0W=`\0\x80>=`\0\xFD[PPPPPP[`\x01\x01a\x0E#V[PPPPPPV[a\x0F\0\x82a\x06\xDEV[a\x0F\t\x81a\x13>V[a\x07\x1C\x83\x83a\x14\x0CV[\x7F\xA5\xCC-=\xBFh\xC9\x12\x82\xF3qq\xEB5\xEC\x15\xB16z\xBC\xD2\xCEC\x88\x14Met\xA6P\xF6\xD8a\x0F=\x81a\x13>V[`\0a\x0FJa\x01\xF4a\x14\xAAV[\x90P`\0[\x81\x81\x10\x15a\x10\x03W`\0a\x0Fea\x01\xF4\x83a\x14\xB4V[\x90Pa\x0F\x8D\x81`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81Ra\x03\xED` R`@\x90 `\x01\x01T`\xFF\x16\x90V[a\x0F\x97WPa\x0F\xFBV[`@Qc3\xF4\x90\x9F`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`$\x83\x01R\x82\x16\x90c3\xF4\x90\x9F\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xF5W=`\0\x80>=`\0\xFD[PPPPP[`\x01\x01a\x0FOV[PPPPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15\x80\x15a\x10PWP\x82[\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x10mWP0;\x15[\x90P\x81\x15\x80\x15a\x10{WP\x80\x15[\x15a\x10\x99W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x10\xC3W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x10\xCBa\x17\"V[a\x10\xD3a\x17\"V[a\x10\xDBa\x17\"V[a\x10\xE3a\x17\"V[a\x10\xEE`\0\x8Aa\x13gV[P`\x01`\x01`\xA0\x1B\x03\x88\x16a\x11VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FStakingManager: Invalid ProofMar`D\x82\x01Rgketplace`\xC0\x1B`d\x82\x01R`\x84\x01a\x08\x16V[a\x01\xF6\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x91\x90\x91\x17\x90\x91U\x86\x16a\x11\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStakingManager: Invalid FeeToken`D\x82\x01R`d\x01a\x08\x16V[a\x01\xF8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x91\x90\x91\x17\x90\x91U\x87\x16a\x12EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FStakingManager: Invalid Symbioti`D\x82\x01RgcStaking`\xC0\x1B`d\x82\x01R`\x84\x01a\x08\x16V[a\x01\xF7\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x16\x17\x90U\x83\x15a\x12\xA7W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[`\0a\x12\xBD\x81a\x13>V[a\x01\xF6\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F\x89 \xA3\x12\x98y\xB1\xE5L\x92Tjt\x8BbI\xBA/\x9A\x03\x83\xF4I\x9D\x9FJ\xE4;\x90\x9A\xD7\x95\x90`\0\x90\xA2PPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06\x18WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x06\x18V[a\x13H\x813a\x17*V[PV[`\0a\x13`\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x17cV[\x93\x92PPPV[`\0`\0\x80Q` a!\xD5\x839\x81Q\x91Ra\x13\x82\x84\x84a\x0C\x98V[a\x14\x02W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x13\xB83\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\x06\x18V[`\0\x91PPa\x06\x18V[`\0`\0\x80Q` a!\xD5\x839\x81Q\x91Ra\x14'\x84\x84a\x0C\x98V[\x15a\x14\x02W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\x06\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x13`V[`\0a\x06\x18\x82T\x90V[`\0a\x13`\x83\x83a\x18LV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x15GWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x15;`\0\x80Q` a!\xB5\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x15eW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0a\t\xC0\x81a\x13>V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x15\xCCWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x15\xC9\x91\x81\x01\x90a \x9EV[`\x01[a\x15\xF4W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x08\x16V[`\0\x80Q` a!\xB5\x839\x81Q\x91R\x81\x14a\x16%W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08\x16V[a\x07U\x83\x83a\x18vV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x15eW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x07U\x90\x84\x90a\x18\xCCV[`\0a\x13`\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x19=V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03\xED` R`@\x81 T\x81\x83a\x17\x06W`\0a\x17\x19V[a\x17\x19\x84\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x19\x8CV[\x95\x94PPPPPV[a\x15ea\x1AGV[a\x174\x82\x82a\x0C\x98V[a\t\xC0W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x08\x16V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x14\x02W`\0a\x17\x87`\x01\x83a!YV[\x85T\x90\x91P`\0\x90a\x17\x9B\x90`\x01\x90a!YV[\x90P\x80\x82\x14a\x18\0W`\0\x86`\0\x01\x82\x81T\x81\x10a\x17\xBBWa\x17\xBBa \x19V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x17\xDEWa\x17\xDEa \x19V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x18\x11Wa\x18\x11a!lV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x06\x18V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x18cWa\x18ca \x19V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[a\x18\x7F\x82a\x1A\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\x18\xC4Wa\x07U\x82\x82a\x1A\xF5V[a\t\xC0a\x1BbV[`\0\x80` `\0\x84Q` \x86\x01`\0\x88Z\xF1\x80a\x18\xEFW`@Q=`\0\x82>=\x81\xFD[PP`\0Q=\x91P\x81\x15a\x19\x07W\x80`\x01\x14\x15a\x19\x14V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x07\x1CW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x08\x16V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x19\x84WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x06\x18V[P`\0a\x06\x18V[`\0\x83\x83\x02\x81`\0\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x19\xC3W\x83\x82\x81a\x19\xB9Wa\x19\xB9a!\x82V[\x04\x92PPPa\x13`V[\x80\x84\x11a\x19\xDAWa\x19\xDA`\x03\x85\x15\x02`\x11\x18a\x1B\x81V[`\0\x84\x86\x88\t`\0\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x15eW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x1A\xC6W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x08\x16V[`\0\x80Q` a!\xB5\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1B\x12\x91\x90a!\x98V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1BMW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1BRV[``\x91P[P\x91P\x91Pa\x17\x19\x85\x83\x83a\x1B\x93V[4\x15a\x15eW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[cNH{q`\0R\x80` R`$`\x1C\xFD[``\x82a\x1B\xA8Wa\x1B\xA3\x82a\x1B\xEFV[a\x13`V[\x81Q\x15\x80\x15a\x1B\xBFWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1B\xE8W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x08\x16V[P\x80a\x13`V[\x80Q\x15a\x1B\xFFW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a\x1C*W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x13`W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1CYW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1CpW`\0\x80\xFD[a\x13`\x82a\x1CBV[`\0` \x82\x84\x03\x12\x15a\x1C\x8BW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xA5W`\0\x80\xFD[\x825\x91Pa\x1C\xB5` \x84\x01a\x1CBV[\x90P\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x1C\xD1W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xE8W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x1C\xF9W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x10W`\0\x80\xFD[\x85` ``\x83\x02\x84\x01\x01\x11\x15a\x1D%W`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1D^W`\0\x80\xFD[a\x1Dg\x83a\x1CBV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x83W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x1D\x94W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xAEWa\x1D\xAEa\x1D5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xDDWa\x1D\xDDa\x1D5V[`@R\x81\x81R\x82\x82\x01` \x01\x87\x10\x15a\x1D\xF5W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E(W`\0\x80\xFD[a\x1E1\x83a\x1CBV[\x91Pa\x1C\xB5` \x84\x01a\x1CBV[`\0\x80\x83`\x1F\x84\x01\x12a\x1EQW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1EiW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E\x84W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x1E\xA1W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xB8W`\0\x80\xFD[a\x1E\xC4\x87\x82\x88\x01a\x1E?V[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xE4W`\0\x80\xFD[a\x1E\xF0\x87\x82\x88\x01a\x1E?V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\x0FW`\0\x80\xFD[a\x1F\x18\x83a\x1CBV[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x1F-W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x1FSW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1F;V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x1F{\x81`@\x85\x01` \x87\x01a\x1F8V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1F\xA4W`\0\x80\xFD[\x835\x92Pa\x1F\xB4` \x85\x01a\x1CBV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1F\xDBW`\0\x80\xFD[a\x1F\xE4\x85a\x1CBV[\x93Pa\x1F\xF2` \x86\x01a\x1CBV[\x92Pa \0`@\x86\x01a\x1CBV[\x91Pa \x0E``\x86\x01a\x1CBV[\x90P\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R\x81\x01\x82\x90R`\0\x83`@\x83\x01\x82[\x85\x81\x10\x15a \x94W\x825\x82R`\x01`\x01`\xA0\x1B\x03a b` \x85\x01a\x1CBV[\x16` \x83\x01R`\x01`\x01`\xA0\x1B\x03a |`@\x85\x01a\x1CBV[\x16`@\x83\x01R``\x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a BV[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a \xB0W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\x18Wa\x06\x18a \xB7V[`\0\x81\x84\x82[\x85\x81\x10\x15a!\x15W`\x01`\x01`\xA0\x1B\x03a \xFF\x83a\x1CBV[\x16\x83R` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a \xE6V[P\x90\x95\x94PPPPPV[` \x80\x82R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a!@W`\0\x80\xFD[\x82`\x05\x1B\x80\x85`@\x85\x017\x91\x90\x91\x01`@\x01\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x06\x18Wa\x06\x18a \xB7V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82Qa!\xAA\x81\x84` \x87\x01a\x1F8V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\xA2dipfsX\"\x12 \xC0\xB7\x16\xCA\xD7u\\C\x9F7;\x1E\xE5F\xD2\x8B<\xED\xF7\xF1:&\xAD\xFEGlh\x8F'\xF2\x8D\x10dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static STAKINGMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct StakingManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakingManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakingManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakingManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakingManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakingManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakingManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                STAKINGMANAGER_ABI.clone(),
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
                STAKINGMANAGER_ABI.clone(),
                STAKINGMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `GENERATOR_REGISTRY_ROLE` (0x89beb5f1) function
        pub fn generator_registry_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([137, 190, 181, 241], ())
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
        ///Calls the contract's `addStakingPool` (0xb124eda9) function
        pub fn add_staking_pool(
            &self,
            staking_pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 36, 237, 169], staking_pool)
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
        ///Calls the contract's `feeToken` (0x647846a5) function
        pub fn fee_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([100, 120, 70, 165], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
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
        ///Calls the contract's `initialize` (0xf8c8765e) function
        pub fn initialize(
            &self,
            admin: ::ethers::core::types::Address,
            proof_marketplace: ::ethers::core::types::Address,
            symbiotic_staking: ::ethers::core::types::Address,
            fee_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 200, 118, 94],
                    (admin, proof_marketplace, symbiotic_staking, fee_token),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isEnabledPool` (0x9bba8844) function
        pub fn is_enabled_pool(
            &self,
            pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([155, 186, 136, 68], pool)
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
            jobs_slashed: ::std::vec::Vec<JobSlashed>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 34, 253, 49], jobs_slashed)
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
        ///Calls the contract's `removeStakingPool` (0x1e9ab754) function
        pub fn remove_staking_pool(
            &self,
            staking_pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 154, 183, 84], staking_pool)
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
        ///Calls the contract's `setEnabledPool` (0x95c751d3) function
        pub fn set_enabled_pool(
            &self,
            pool: ::ethers::core::types::Address,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 199, 81, 211], (pool, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeToken` (0x15cce224) function
        pub fn set_fee_token(
            &self,
            fee_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 204, 226, 36], fee_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolRewardShare` (0x91c5433a) function
        pub fn set_pool_reward_share(
            &self,
            pools: ::std::vec::Vec<::ethers::core::types::Address>,
            shares: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 197, 67, 58], (pools, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProofMarketplace` (0xfbaa8303) function
        pub fn set_proof_marketplace(
            &self,
            proof_marketplace: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 170, 131, 3], proof_marketplace)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSymbioticStaking` (0x416827df) function
        pub fn set_symbiotic_staking(
            &self,
            symbiotic_staking: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 104, 39, 223], symbiotic_staking)
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
        ///Calls the contract's `symbioticStaking` (0xa1597bcd) function
        pub fn symbiotic_staking(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([161, 89, 123, 205], ())
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
        ///Gets the contract's `FeeTokenSet` event
        pub fn fee_token_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeTokenSetFilter>
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
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakingManagerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for StakingManager<M>
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
    pub enum StakingManagerErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        AddressEmptyCode(AddressEmptyCode),
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
    impl ::ethers::core::abi::AbiDecode for StakingManagerErrors {
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
    impl ::ethers::core::abi::AbiEncode for StakingManagerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
    impl ::ethers::contract::ContractRevert for StakingManagerErrors {
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
    impl ::core::fmt::Display for StakingManagerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<::std::string::String> for StakingManagerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation> for StakingManagerErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount> for StakingManagerErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for StakingManagerErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for StakingManagerErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for StakingManagerErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedCall> for StakingManagerErrors {
        fn from(value: FailedCall) -> Self {
            Self::FailedCall(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for StakingManagerErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for StakingManagerErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for StakingManagerErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for StakingManagerErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for StakingManagerErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for StakingManagerErrors {
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
    pub enum StakingManagerEvents {
        FeeTokenSetFilter(FeeTokenSetFilter),
        InitializedFilter(InitializedFilter),
        PoolEnabledSetFilter(PoolEnabledSetFilter),
        PoolRewardShareSetFilter(PoolRewardShareSetFilter),
        ProofMarketplaceSetFilter(ProofMarketplaceSetFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        StakingPoolAddedFilter(StakingPoolAddedFilter),
        StakingPoolRemovedFilter(StakingPoolRemovedFilter),
        SymbioticStakingSetFilter(SymbioticStakingSetFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakingManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FeeTokenSetFilter::decode_log(log) {
                return Ok(StakingManagerEvents::FeeTokenSetFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(StakingManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = PoolEnabledSetFilter::decode_log(log) {
                return Ok(StakingManagerEvents::PoolEnabledSetFilter(decoded));
            }
            if let Ok(decoded) = PoolRewardShareSetFilter::decode_log(log) {
                return Ok(StakingManagerEvents::PoolRewardShareSetFilter(decoded));
            }
            if let Ok(decoded) = ProofMarketplaceSetFilter::decode_log(log) {
                return Ok(StakingManagerEvents::ProofMarketplaceSetFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(StakingManagerEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(StakingManagerEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(StakingManagerEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = StakingPoolAddedFilter::decode_log(log) {
                return Ok(StakingManagerEvents::StakingPoolAddedFilter(decoded));
            }
            if let Ok(decoded) = StakingPoolRemovedFilter::decode_log(log) {
                return Ok(StakingManagerEvents::StakingPoolRemovedFilter(decoded));
            }
            if let Ok(decoded) = SymbioticStakingSetFilter::decode_log(log) {
                return Ok(StakingManagerEvents::SymbioticStakingSetFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(StakingManagerEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakingManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FeeTokenSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolEnabledSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolRewardShareSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketplaceSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakingPoolAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakingPoolRemovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbioticStakingSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FeeTokenSetFilter> for StakingManagerEvents {
        fn from(value: FeeTokenSetFilter) -> Self {
            Self::FeeTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for StakingManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<PoolEnabledSetFilter> for StakingManagerEvents {
        fn from(value: PoolEnabledSetFilter) -> Self {
            Self::PoolEnabledSetFilter(value)
        }
    }
    impl ::core::convert::From<PoolRewardShareSetFilter> for StakingManagerEvents {
        fn from(value: PoolRewardShareSetFilter) -> Self {
            Self::PoolRewardShareSetFilter(value)
        }
    }
    impl ::core::convert::From<ProofMarketplaceSetFilter> for StakingManagerEvents {
        fn from(value: ProofMarketplaceSetFilter) -> Self {
            Self::ProofMarketplaceSetFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for StakingManagerEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for StakingManagerEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for StakingManagerEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<StakingPoolAddedFilter> for StakingManagerEvents {
        fn from(value: StakingPoolAddedFilter) -> Self {
            Self::StakingPoolAddedFilter(value)
        }
    }
    impl ::core::convert::From<StakingPoolRemovedFilter> for StakingManagerEvents {
        fn from(value: StakingPoolRemovedFilter) -> Self {
            Self::StakingPoolRemovedFilter(value)
        }
    }
    impl ::core::convert::From<SymbioticStakingSetFilter> for StakingManagerEvents {
        fn from(value: SymbioticStakingSetFilter) -> Self {
            Self::SymbioticStakingSetFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for StakingManagerEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `GENERATOR_REGISTRY_ROLE` function with signature `GENERATOR_REGISTRY_ROLE()` and selector `0x89beb5f1`
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
    #[ethcall(name = "GENERATOR_REGISTRY_ROLE", abi = "GENERATOR_REGISTRY_ROLE()")]
    pub struct GeneratorRegistryRoleCall;
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
    ///Container type for all input parameters for the `addStakingPool` function with signature `addStakingPool(address)` and selector `0xb124eda9`
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
    #[ethcall(name = "addStakingPool", abi = "addStakingPool(address)")]
    pub struct AddStakingPoolCall {
        pub staking_pool: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `feeToken` function with signature `feeToken()` and selector `0x647846a5`
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
    #[ethcall(name = "feeToken", abi = "feeToken()")]
    pub struct FeeTokenCall;
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address)` and selector `0xf8c8765e`
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
        abi = "initialize(address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub admin: ::ethers::core::types::Address,
        pub proof_marketplace: ::ethers::core::types::Address,
        pub symbiotic_staking: ::ethers::core::types::Address,
        pub fee_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isEnabledPool` function with signature `isEnabledPool(address)` and selector `0x9bba8844`
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
    #[ethcall(name = "isEnabledPool", abi = "isEnabledPool(address)")]
    pub struct IsEnabledPoolCall {
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
        pub fee_reward_amount: ::ethers::core::types::U256,
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
        pub jobs_slashed: ::std::vec::Vec<JobSlashed>,
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
    ///Container type for all input parameters for the `removeStakingPool` function with signature `removeStakingPool(address)` and selector `0x1e9ab754`
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
    #[ethcall(name = "removeStakingPool", abi = "removeStakingPool(address)")]
    pub struct RemoveStakingPoolCall {
        pub staking_pool: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `setEnabledPool` function with signature `setEnabledPool(address,bool)` and selector `0x95c751d3`
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
    #[ethcall(name = "setEnabledPool", abi = "setEnabledPool(address,bool)")]
    pub struct SetEnabledPoolCall {
        pub pool: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setFeeToken` function with signature `setFeeToken(address)` and selector `0x15cce224`
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
    #[ethcall(name = "setFeeToken", abi = "setFeeToken(address)")]
    pub struct SetFeeTokenCall {
        pub fee_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPoolRewardShare` function with signature `setPoolRewardShare(address[],uint256[])` and selector `0x91c5433a`
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
        name = "setPoolRewardShare",
        abi = "setPoolRewardShare(address[],uint256[])"
    )]
    pub struct SetPoolRewardShareCall {
        pub pools: ::std::vec::Vec<::ethers::core::types::Address>,
        pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `setProofMarketplace` function with signature `setProofMarketplace(address)` and selector `0xfbaa8303`
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
    #[ethcall(name = "setProofMarketplace", abi = "setProofMarketplace(address)")]
    pub struct SetProofMarketplaceCall {
        pub proof_marketplace: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setSymbioticStaking` function with signature `setSymbioticStaking(address)` and selector `0x416827df`
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
    #[ethcall(name = "setSymbioticStaking", abi = "setSymbioticStaking(address)")]
    pub struct SetSymbioticStakingCall {
        pub symbiotic_staking: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `symbioticStaking` function with signature `symbioticStaking()` and selector `0xa1597bcd`
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
    #[ethcall(name = "symbioticStaking", abi = "symbioticStaking()")]
    pub struct SymbioticStakingCall;
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
    pub enum StakingManagerCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        GeneratorRegistryRole(GeneratorRegistryRoleCall),
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        AddStakingPool(AddStakingPoolCall),
        EmergencyWithdraw(EmergencyWithdrawCall),
        FeeToken(FeeTokenCall),
        GetPoolConfig(GetPoolConfigCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Initialize(InitializeCall),
        IsEnabledPool(IsEnabledPoolCall),
        OnJobCompletion(OnJobCompletionCall),
        OnJobCreation(OnJobCreationCall),
        OnSlashResult(OnSlashResultCall),
        ProofMarketplace(ProofMarketplaceCall),
        ProxiableUUID(ProxiableUUIDCall),
        RemoveStakingPool(RemoveStakingPoolCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetEnabledPool(SetEnabledPoolCall),
        SetFeeToken(SetFeeTokenCall),
        SetPoolRewardShare(SetPoolRewardShareCall),
        SetProofMarketplace(SetProofMarketplaceCall),
        SetSymbioticStaking(SetSymbioticStakingCall),
        SupportsInterface(SupportsInterfaceCall),
        SymbioticStaking(SymbioticStakingCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakingManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <GeneratorRegistryRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GeneratorRegistryRole(decoded));
            }
            if let Ok(decoded) =
                <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) =
                <AddStakingPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddStakingPool(decoded));
            }
            if let Ok(decoded) =
                <EmergencyWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmergencyWithdraw(decoded));
            }
            if let Ok(decoded) = <FeeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeToken(decoded));
            }
            if let Ok(decoded) = <GetPoolConfigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolConfig(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRoleAdmin(decoded));
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
            if let Ok(decoded) = <IsEnabledPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsEnabledPool(decoded));
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
                <RemoveStakingPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveStakingPool(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <SetEnabledPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetEnabledPool(decoded));
            }
            if let Ok(decoded) = <SetFeeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeToken(decoded));
            }
            if let Ok(decoded) =
                <SetPoolRewardShareCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPoolRewardShare(decoded));
            }
            if let Ok(decoded) =
                <SetProofMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProofMarketplace(decoded));
            }
            if let Ok(decoded) =
                <SetSymbioticStakingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetSymbioticStaking(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <SymbioticStakingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SymbioticStaking(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakingManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GeneratorRegistryRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddStakingPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmergencyWithdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRoleAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GrantRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsEnabledPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnJobCompletion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnJobCreation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnSlashResult(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProofMarketplace(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveStakingPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetEnabledPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPoolRewardShare(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProofMarketplace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSymbioticStaking(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SymbioticStaking(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeToAndCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for StakingManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::GeneratorRegistryRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeInterfaceVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStakingPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmergencyWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsEnabledPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnJobCompletion(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnJobCreation(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnSlashResult(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveStakingPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEnabledPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPoolRewardShare(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProofMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSymbioticStaking(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbioticStaking(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for StakingManagerCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<GeneratorRegistryRoleCall> for StakingManagerCalls {
        fn from(value: GeneratorRegistryRoleCall) -> Self {
            Self::GeneratorRegistryRole(value)
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for StakingManagerCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<AddStakingPoolCall> for StakingManagerCalls {
        fn from(value: AddStakingPoolCall) -> Self {
            Self::AddStakingPool(value)
        }
    }
    impl ::core::convert::From<EmergencyWithdrawCall> for StakingManagerCalls {
        fn from(value: EmergencyWithdrawCall) -> Self {
            Self::EmergencyWithdraw(value)
        }
    }
    impl ::core::convert::From<FeeTokenCall> for StakingManagerCalls {
        fn from(value: FeeTokenCall) -> Self {
            Self::FeeToken(value)
        }
    }
    impl ::core::convert::From<GetPoolConfigCall> for StakingManagerCalls {
        fn from(value: GetPoolConfigCall) -> Self {
            Self::GetPoolConfig(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for StakingManagerCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for StakingManagerCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for StakingManagerCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for StakingManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsEnabledPoolCall> for StakingManagerCalls {
        fn from(value: IsEnabledPoolCall) -> Self {
            Self::IsEnabledPool(value)
        }
    }
    impl ::core::convert::From<OnJobCompletionCall> for StakingManagerCalls {
        fn from(value: OnJobCompletionCall) -> Self {
            Self::OnJobCompletion(value)
        }
    }
    impl ::core::convert::From<OnJobCreationCall> for StakingManagerCalls {
        fn from(value: OnJobCreationCall) -> Self {
            Self::OnJobCreation(value)
        }
    }
    impl ::core::convert::From<OnSlashResultCall> for StakingManagerCalls {
        fn from(value: OnSlashResultCall) -> Self {
            Self::OnSlashResult(value)
        }
    }
    impl ::core::convert::From<ProofMarketplaceCall> for StakingManagerCalls {
        fn from(value: ProofMarketplaceCall) -> Self {
            Self::ProofMarketplace(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for StakingManagerCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RemoveStakingPoolCall> for StakingManagerCalls {
        fn from(value: RemoveStakingPoolCall) -> Self {
            Self::RemoveStakingPool(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for StakingManagerCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for StakingManagerCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetEnabledPoolCall> for StakingManagerCalls {
        fn from(value: SetEnabledPoolCall) -> Self {
            Self::SetEnabledPool(value)
        }
    }
    impl ::core::convert::From<SetFeeTokenCall> for StakingManagerCalls {
        fn from(value: SetFeeTokenCall) -> Self {
            Self::SetFeeToken(value)
        }
    }
    impl ::core::convert::From<SetPoolRewardShareCall> for StakingManagerCalls {
        fn from(value: SetPoolRewardShareCall) -> Self {
            Self::SetPoolRewardShare(value)
        }
    }
    impl ::core::convert::From<SetProofMarketplaceCall> for StakingManagerCalls {
        fn from(value: SetProofMarketplaceCall) -> Self {
            Self::SetProofMarketplace(value)
        }
    }
    impl ::core::convert::From<SetSymbioticStakingCall> for StakingManagerCalls {
        fn from(value: SetSymbioticStakingCall) -> Self {
            Self::SetSymbioticStaking(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for StakingManagerCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbioticStakingCall> for StakingManagerCalls {
        fn from(value: SymbioticStakingCall) -> Self {
            Self::SymbioticStaking(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for StakingManagerCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
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
    ///Container type for all return fields from the `GENERATOR_REGISTRY_ROLE` function with signature `GENERATOR_REGISTRY_ROLE()` and selector `0x89beb5f1`
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
    pub struct GeneratorRegistryRoleReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `feeToken` function with signature `feeToken()` and selector `0x647846a5`
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
    pub struct FeeTokenReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `isEnabledPool` function with signature `isEnabledPool(address)` and selector `0x9bba8844`
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
    pub struct IsEnabledPoolReturn(pub bool);
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
    ///Container type for all return fields from the `symbioticStaking` function with signature `symbioticStaking()` and selector `0xa1597bcd`
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
    pub struct SymbioticStakingReturn(pub ::ethers::core::types::Address);
}
