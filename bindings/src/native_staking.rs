pub use native_staking::*;
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
pub mod native_staking {
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
                    ::std::borrow::ToOwned::to_owned("addStakeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addStakeToken"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_token"),
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
                            name: ::std::borrow::ToOwned::to_owned("lockAmount"),
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
                    ::std::borrow::ToOwned::to_owned("getOperatorLockedAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorLockedAmount",),
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
                                name: ::std::borrow::ToOwned::to_owned("_account"),
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
                                name: ::std::borrow::ToOwned::to_owned("_stakingManager"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_withdrawalDuration",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                                name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("operatorstakeAmounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorstakeAmounts",),
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
                    ::std::borrow::ToOwned::to_owned("removeStakeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeStakeToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_token"),
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
                    ::std::borrow::ToOwned::to_owned("requestStakeWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestStakeWithdrawal",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
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
                                name: ::std::borrow::ToOwned::to_owned("_token"),
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
                    ::std::borrow::ToOwned::to_owned("setFeeRewardToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFeeRewardToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_token"),
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
                                name: ::std::borrow::ToOwned::to_owned("_token"),
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
                    ::std::borrow::ToOwned::to_owned("setWithdrawalDuration"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setWithdrawalDuration",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_duration"),
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
                    ::std::borrow::ToOwned::to_owned("stake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stake"),
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
                    ::std::borrow::ToOwned::to_owned("stakeAmounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeAmounts"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
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
                (
                    ::std::borrow::ToOwned::to_owned("withdrawStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawStake"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_index"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawalDuration"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawalDuration"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawalRequests"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawalRequests"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
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
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalTime"),
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
    pub static NATIVESTAKING_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R0`\x80R4\x80\x15a\0\x14W`\0\x80\xFD[P`@Qa5\xE88\x03\x80a5\xE8\x839\x81\x01`@\x81\x90Ra\x003\x91a\0DV[`\x01`\x01`\xA0\x1B\x03\x16`\xA0Ra\0tV[`\0` \x82\x84\x03\x12\x15a\0VW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0mW`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Qa5\x17a\0\xD1`\09`\0\x81\x81a\x03\x05\x01R\x81\x81a\x0C\xBF\x01R\x81\x81a\x16\xA6\x01R\x81\x81a\x18\x18\x01R\x81\x81a\x1A\\\x01R\x81\x81a\x1E\x05\x01Ra Y\x01R`\0\x81\x81a&\xD8\x01R\x81\x81a'\x01\x01Ra(G\x01Ra5\x17`\0\xF3\xFE`\x80`@R`\x046\x10a\x02rW`\x005`\xE0\x1C\x80cu\xE3\xB4p\x11a\x01OW\x80c\xAEk\x19\x83\x11a\0\xC1W\x80c\xCB\x95qz\x11a\0zW\x80c\xCB\x95qz\x14a\x08\xB3W\x80c\xD1\x1F)<\x14a\x08\xE1W\x80c\xD3e\xD2\xD6\x14a\t\x03W\x80c\xD5Gt\x1F\x14a\tJW\x80c\xD5]\t#\x14a\tjW\x80c\xDE\xE8\xBC\xE2\x14a\t\x8AW`\0\x80\xFD[\x80c\xAEk\x19\x83\x14a\x07\xCEW\x80c\xB0\x0B\xBAj\x14a\x08\x13W\x80c\xBE 0\x94\x14a\x083W\x80c\xBFn\xAC/\x14a\x08SW\x80c\xC0\xFF\x97\xEF\x14a\x08sW\x80c\xC9\x85\xD9i\x14a\x08\x93W`\0\x80\xFD[\x80c\x91\xD1HT\x11a\x01\x13W\x80c\x91\xD1HT\x14a\x06\xD2W\x80c\x9F\xCD\x11_\x14a\x06\xF2W\x80c\xA2\x17\xFD\xDF\x14a\x07CW\x80c\xA6\xFF\xE5'\x14a\x07XW\x80c\xAC\xC2\x16j\x14a\x07oW\x80c\xAD<\xB1\xCC\x14a\x07\x90W`\0\x80\xFD[\x80cu\xE3\xB4p\x14a\x06\x14W\x80cz\x8C\xA0\xBB\x14a\x06+W\x80c\x81\xF7\xD9B\x14a\x06dW\x80c\x89\xC7\xB9\x87\x14a\x06\x84W\x80c\x8D;\x9DZ\x14a\x06\xA4W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01\xE8W\x80cR\xD1\x90-\x11a\x01\xACW\x80cR\xD1\x90-\x14a\x04\xFDW\x80cW$\xB8N\x14a\x05\x12W\x80c[\x1AL$\x14a\x052W\x80c_A=8\x14a\x05\x92W\x80cc\x82\xD9\xAD\x14a\x05\xB5W\x80ck\xC0K)\x14a\x05\xD5W`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x04iW\x80c=t\xA7x\x14a\x04\x89W\x80c=\xF3m\xDF\x14a\x04\xAAW\x80cN\x82\x11\x02\x14a\x04\xCAW\x80cO\x1E\xF2\x86\x14a\x04\xEAW`\0\x80\xFD[\x80c\"\x82\x8C\xC2\x11a\x02:W\x80c\"\x82\x8C\xC2\x14a\x03\x81W\x80c$\x8A\x9C\xA3\x14a\x03\xA2W\x80c)\xF3\x84\xE2\x14a\x03\xC2W\x80c//\xF1]\x14a\x03\xE2W\x80c1\xA3\x80v\x14a\x04\x02W\x80c3\xF4\x90\x9F\x14a\x04IW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02wW\x80c\nY\xBBP\x14a\x02\xACW\x80c\x11\x0C\xBC\x80\x14a\x02\xF3W\x80c\x16hf\xC7\x14a\x03?W\x80c\x1F=\xC4\xE2\x14a\x03aW[`\0\x80\xFD[4\x80\x15a\x02\x83W`\0\x80\xFD[Pa\x02\x97a\x02\x926`\x04a.mV[a\t\xAAV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB8W`\0\x80\xFD[Pa\x02\xE5a\x02\xC76`\x04a.\xB3V[a\x03\xF5` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x02\xA3V[4\x80\x15a\x02\xFFW`\0\x80\xFD[Pa\x03'\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA3V[4\x80\x15a\x03KW`\0\x80\xFD[Pa\x03_a\x03Z6`\x04a.\xE6V[a\t\xBBV[\0[4\x80\x15a\x03mW`\0\x80\xFD[Pa\x03_a\x03|6`\x04a/\x01V[a\n`V[4\x80\x15a\x03\x8DW`\0\x80\xFD[Pa\x01\xF6Ta\x03'\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03\xAEW`\0\x80\xFD[Pa\x02\xE5a\x03\xBD6`\x04a/+V[a\n\xB4V[4\x80\x15a\x03\xCEW`\0\x80\xFD[Pa\x03_a\x03\xDD6`\x04a/+V[a\n\xD6V[4\x80\x15a\x03\xEEW`\0\x80\xFD[Pa\x03_a\x03\xFD6`\x04a/DV[a\x0B\x16V[4\x80\x15a\x04\x0EW`\0\x80\xFD[Pa\x02\xE5a\x04\x1D6`\x04a.\xB3V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[4\x80\x15a\x04UW`\0\x80\xFD[Pa\x03_a\x04d6`\x04a/DV[a\x0B8V[4\x80\x15a\x04uW`\0\x80\xFD[Pa\x03_a\x04\x846`\x04a/DV[a\r0V[4\x80\x15a\x04\x95W`\0\x80\xFD[Pa\x01\xF8Ta\x03'\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xB6W`\0\x80\xFD[Pa\x03_a\x04\xC56`\x04a.\xE6V[a\rhV[4\x80\x15a\x04\xD6W`\0\x80\xFD[Pa\x02\x97a\x04\xE56`\x04a.\xE6V[a\r\xBFV[a\x03_a\x04\xF86`\x04a/}V[a\r\xCDV[4\x80\x15a\x05\tW`\0\x80\xFD[Pa\x02\xE5a\r\xECV[4\x80\x15a\x05\x1EW`\0\x80\xFD[Pa\x03_a\x05-6`\x04a/\x01V[a\x0E\tV[4\x80\x15a\x05>W`\0\x80\xFD[Pa\x05sa\x05M6`\x04a/+V[a\x03\xF4` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xA3V[4\x80\x15a\x05\x9EW`\0\x80\xFD[Pa\x05\xA7a\x0E\xB6V[`@Qa\x02\xA3\x92\x91\x90a0\x8CV[4\x80\x15a\x05\xC1W`\0\x80\xFD[Pa\x03_a\x05\xD06`\x04a.\xB3V[a\x0F\x8AV[4\x80\x15a\x05\xE1W`\0\x80\xFD[Pa\x02\xE5a\x05\xF06`\x04a0\xE6V[a\x03\xF1` \x90\x81R`\0\x93\x84R`@\x80\x85 \x82R\x92\x84R\x82\x84 \x90R\x82R\x90 T\x81V[4\x80\x15a\x06 W`\0\x80\xFD[Pa\x02\xE5a\x01\xF9T\x81V[4\x80\x15a\x067W`\0\x80\xFD[Pa\x02\xE5a\x06F6`\x04a.\xB3V[a\x03\xF2` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x06pW`\0\x80\xFD[Pa\x03_a\x06\x7F6`\x04a/\x01V[a\x10\xA7V[4\x80\x15a\x06\x90W`\0\x80\xFD[Pa\x02\xE5a\x06\x9F6`\x04a.\xB3V[a\x11pV[4\x80\x15a\x06\xB0W`\0\x80\xFD[Pa\x02\xE5a\x06\xBF6`\x04a.\xE6V[a\x03\xEF` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x06\xDEW`\0\x80\xFD[Pa\x02\x97a\x06\xED6`\x04a/DV[a\x11\xC0V[4\x80\x15a\x06\xFEW`\0\x80\xFD[Pa\x02\xE5a\x07\r6`\x04a0\xE6V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81Ra\x03\xF1` \x90\x81R`@\x80\x83 \x94\x86\x16\x83R\x93\x81R\x83\x82 \x92\x90\x94\x16\x81R\x92R\x90 T\x90V[4\x80\x15a\x07OW`\0\x80\xFD[Pa\x02\xE5`\0\x81V[4\x80\x15a\x07dW`\0\x80\xFD[Pa\x02\xE5a\x01\xFAT\x81V[4\x80\x15a\x07{W`\0\x80\xFD[Pa\x01\xF7Ta\x03'\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\x9CW`\0\x80\xFD[Pa\x07\xC1`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02\xA3\x91\x90a1MV[4\x80\x15a\x07\xDAW`\0\x80\xFD[Pa\x07\xEEa\x07\xE96`\x04a1\x80V[a\x11\xF8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xA3V[4\x80\x15a\x08\x1FW`\0\x80\xFD[Pa\x03_a\x08.6`\x04a.\xE6V[a\x12SV[4\x80\x15a\x08?W`\0\x80\xFD[Pa\x03_a\x08N6`\x04a1\xBDV[a\x12\xAAV[4\x80\x15a\x08_W`\0\x80\xFD[Pa\x03_a\x08n6`\x04a1\x80V[a\x15\x04V[4\x80\x15a\x08\x7FW`\0\x80\xFD[Pa\x03_a\x08\x8E6`\x04a2\nV[a\x17(V[4\x80\x15a\x08\x9FW`\0\x80\xFD[Pa\x03_a\x08\xAE6`\x04a1\x80V[a\x18OV[4\x80\x15a\x08\xBFW`\0\x80\xFD[Pa\x02\xE5a\x08\xCE6`\x04a.\xE6V[a\x03\xF0` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x08\xEDW`\0\x80\xFD[Pa\x08\xF6a\x1A\xDFV[`@Qa\x02\xA3\x91\x90a2/V[4\x80\x15a\t\x0FW`\0\x80\xFD[Pa\x02\xE5a\t\x1E6`\x04a.\xB3V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81Ra\x03\xF2` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[4\x80\x15a\tVW`\0\x80\xFD[Pa\x03_a\te6`\x04a/DV[a\x1A\xF1V[4\x80\x15a\tvW`\0\x80\xFD[Pa\x03_a\t\x856`\x04a2BV[a\x1B\rV[4\x80\x15a\t\x96W`\0\x80\xFD[Pa\x03_a\t\xA56`\x04a2\xCAV[a\x1E\x93V[`\0a\t\xB5\x82a!\x13V[\x92\x91PPV[`\0a\t\xC6\x81a!HV[a\t\xD2a\x01\xF4\x83a!UV[a\n\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x1B\xDA\xD9[\x88\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`b\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x03\xF0` R`@\x80\x82 \x82\x90UQ\x7FzP/\xFF\xACP\xF2\xA88\xC9U\x04M\xECh\xB2rd\x17B\xA6\x86\x17\xAB\xE1nI\x82\xAA\xC3yu\x91\x90\xA2PPV[`\0a\nk\x81a!HV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03\xEF` R`@\x80\x82 \x85\x90UQ\x84\x92\x91\x7F\xC3`\x93\x99-6\x17;\xC34\x83\xBA\xE4N\xEC\xC3\x90?\x841\xE5\xC0\x8A\x07\xC1\x94Z\x89f\x133\x83\x91\xA3PPPV[`\0\x90\x81R`\0\x80Q` a4\xA2\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[`\0a\n\xE1\x81a!HV[a\x01\xF9\x82\x90U`@Q\x82\x90\x7F\x06\x9F\x0F\xCA-\xB0-]\0KsF\xA0\xE6g\x19{\x90q\xFEe\xEE\xAA\xA1\x1Dx\xE3\x82\x89\x85\x1D\xC2\x90`\0\x90\xA2PPV[a\x0B\x1F\x82a\n\xB4V[a\x0B(\x81a!HV[a\x0B2\x83\x83a!jV[PPPPV[a\x01\xF6T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0BcW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x11\x90a3AV[`\0a\x0Bn\x82a\"\x0FV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x03\xEF` R`@\x90 T\x90\x91P\x80a\x0B\x96\x83\x85a\x11pV[\x10\x15a\x0B\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInsufficient stake to lock\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\x11V[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x80\x83R` \x80\x84\x01\x86\x81R`\0\x8A\x81Ra\x03\xF4\x83R\x86\x81 \x95Q\x86T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x86\x16\x17\x86U\x90Q`\x01\x90\x95\x01\x94\x90\x94U\x90\x83Ra\x03\xF5\x81R\x83\x83 \x91\x87\x16\x83RR\x90\x81 \x80T\x83\x92\x90a\x0CS\x90\x84\x90a3\x84V[\x92PP\x81\x90UP\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7FV\xAC\xD9\xAAF\x83\x1C\xF3\xE8\x03\xC8\x9A`\xEE\xCA\"\xE7\r\x8D\xD3#;\xCB\x1F\x9EH\x9394\x0C\xA0\x01\x84`@Qa\x0C\xA0\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4`@Qc \x1A\x11\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c \x1A\x11\xB3\x90a\x0C\xF8\x90\x86\x90\x86\x90\x86\x90`\x04\x01a3\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r&W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\rYW`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rc\x82\x82a&/V[PPPV[`\0a\rs\x81a!HV[a\x01\xF8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F_\x8E+\x8B8\xD2\x0EPBt\xAB\xF2\x84z\xD3/Jm\x03\xB3\xCCu\xD0\x18\xFBW\x11Y9u\x96\xC7\x90`\0\x90\xA2PPV[`\0a\t\xB5a\x01\xF4\x83a&\xABV[a\r\xD5a&\xCDV[a\r\xDE\x82a'tV[a\r\xE8\x82\x82a'\x7FV[PPV[`\0a\r\xF6a(<V[P`\0\x80Q` a4\x82\x839\x81Q\x91R\x90V[`\0a\x0E\x14\x81a!HV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\xF0` R`@\x81 Ta\x01\xFA\x80T\x91\x92\x90\x91a\x0EB\x90\x84\x90a3\xBBV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\xF0` R`@\x81 \x83\x90Ua\x01\xFA\x80T\x84\x92\x90a\x0Ev\x90\x84\x90a3\x84V[\x90\x91UPP`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F0#5\xD1>\t\xEC\xAF\xF1\xF0\xAE\xD0\xB1\x9BZ\xD3\xA0T\t>\xA0qZ6\xB6\xA4\xBC\"\x9BR\x95\x82\x90`\0\x90\xA3PPPV[``\x80`\0a\x0E\xC6a\x01\xF4a(\x85V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xDEWa\x0E\xDEa/gV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x07W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[a\x0F\x18a\x01\xF4a(\x85V[\x81\x10\x15a\x0FvWa\x03\xF0`\0a\x0F0a\x01\xF4\x84a(\x8FV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x0FcWa\x0Fca3\xCEV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F\rV[Pa\x0F\x82a\x01\xF4a(\x9BV[\x93\x90\x92P\x90PV[`\0a\x0F\x95\x81a!HV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0F\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqzero token address`p\x1B`D\x82\x01R`d\x01a\n\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rnzero to address`\x88\x1B`D\x82\x01R`d\x01a\n\x11V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\rc\x90\x83\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x96\x91\x90a3\xE4V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x91\x90a(\xA8V[`\0a\x10\xB2\x81a!HV[a\x10\xBEa\x01\xF4\x84a)\x07V[a\x11\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsToken already exists``\x1B`D\x82\x01R`d\x01a\n\x11V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\xF0` R`@\x81 \x83\x90Ua\x01\xFA\x80T\x84\x92\x90a\x110\x90\x84\x90a3\x84V[\x90\x91UPP`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\x0C\x96U\xA8\r\xB9\xD3N#\xDD\x9DH<\xB2\xB6\x84\xE4v\x11T\xA6o\x93]\x9BB\xC1t\x89-\xD3%\x90`\0\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83Ra\x03\xF2\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 Ta\x11\xB9\x91\x90a3\xBBV[\x93\x92PPPV[`\0\x91\x82R`\0\x80Q` a4\xA2\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x03\xF3` R\x82`\0R`@`\0 ` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x12!W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x90\x92\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x94P\x90\x92P\x90P\x83V[`\0a\x12^\x81a!HV[a\x01\xF6\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB9\xE8\x9D\x19:q\x1D\xD2\x9A%3(\xFA\xAEn\xF5;\xD4\x1F\x10\x01sDl\x82 \xA3\x8A\x02.$\x0C\x90`\0\x90\xA2PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15\x80\x15a\x12\xF0WP\x82[\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x13\rWP0;\x15[\x90P\x81\x15\x80\x15a\x13\x1BWP\x80\x15[\x15a\x139W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x13cW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x13ka)\x1CV[a\x13sa)\x1CV[a\x13{a)\x1CV[a\x13\x83a)\x1CV[a\x13\x8E`\0\x8Aa!jV[P`\x01`\x01`\xA0\x1B\x03\x88\x16a\x13\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FNativeStaking: Invalid StakingMa`D\x82\x01Rd70\xB3\xB2\xB9`\xD9\x1B`d\x82\x01R`\x84\x01a\n\x11V[a\x01\xF6\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB9\xE8\x9D\x19:q\x1D\xD2\x9A%3(\xFA\xAEn\xF5;\xD4\x1F\x10\x01sDl\x82 \xA3\x8A\x02.$\x0C\x90`\0\x90\xA2a\x01\xF9\x87\x90U`\x01`\x01`\xA0\x1B\x03\x86\x16a\x14\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNativeStaking: Invalid Fee Token`D\x82\x01R`d\x01a\n\x11V[a\x01\xF8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x17\x90U\x83\x15a\x14\xF9W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[\x82a\x15\x11a\x01\xF4\x82a&\xABV[a\x15SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1B\xDA\xD9[\x88\x1B\x9B\xDD\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`j\x1B`D\x82\x01R`d\x01a\n\x11V[a\x15[a)$V[3`\x01`\x01`\xA0\x1B\x03\x84\x16\x14a\x15\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FOnly operator can stake\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\x11V[a\x15\xC8`\x01`\x01`\xA0\x1B\x03\x85\x1630\x85a)\\V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81Ra\x03\xF1` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x80T\x84\x92\x90a\x16\x08\x90\x84\x90a3\x84V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81Ra\x03\xF2` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x80T\x84\x92\x90a\x16E\x90\x84\x90a3\x84V[\x90\x91UPP`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x90\x85\x16\x903\x90\x7Fna>PM\xCB\xE2g\xF6\x0E)[\x08\xE0\xA2\x11\xB6=\xB8i\rf\x0E^\xD4\xF8d\xD4\t\xBBf \x90` \x01`@Q\x80\x91\x03\x90\xA4`@QcS\x1B\x0Bs`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA66\x16\xE6\x90a\x16\xDF\x90\x86\x90\x88\x90\x87\x90`\x04\x01a3\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\rW=`\0\x80>=`\0\xFD[PPPPa\x0B2`\x01`\0\x80Q` a4\xC2\x839\x81Q\x91RUV[a\x01\xF6T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x11\x90a3AV[`\0\x83\x81Ra\x03\xF4` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x82R`\x01\x01T\x91\x81\x01\x82\x90R\x91\x03a\x17\x90WPPPPV[a\x17\xA4\x84\x82`\0\x01Q\x85\x84` \x01Qa)\x84V[\x80`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F\xB8\xECr\xF7\xC3EH\xFB\xB7\xD0\xE8\x8D\x93\xF2\xAA\x1E\x8C7\x11\x17y\x86\xB4\xD5\xBC\r\xAB\xF5\x86\x8C\xA1\xAB\x84` \x01Q`@Qa\x17\xF2\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x80Q` \x82\x01Q`@Qc\x18\xDA\x93]`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c1\xB5&\xBA\x92a\x0C\xF8\x92\x88\x92\x91\x90`\x04\x01a3\x97V[a\x18Wa)$V[\x80a\x18b\x83\x85a\x11pV[\x10\x15a\x18\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInsufficient stake`p\x1B`D\x82\x01R`d\x01a\n\x11V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81Ra\x03\xF1` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x18\xE5\x90\x84\x90a3\xBBV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81Ra\x03\xF2` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x19\"\x90\x84\x90a3\xBBV[\x90\x91UPP3`\0\x90\x81Ra\x03\xF3` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x85R\x90\x83R\x92\x81\x90 \x81Q``\x81\x01\x83R\x93\x86\x16\x84R\x91\x83\x01\x84\x90Ra\x01\xF9T\x91\x92\x91\x90\x82\x01\x90a\x19v\x90Ba3\x84V[\x90R\x81T`\x01\x80\x82\x01\x84U`\0\x93\x84R` \x80\x85 \x84Q`\x03\x90\x94\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x81U\x84\x82\x01Q\x81\x84\x01U`@\x94\x85\x01Q`\x02\x90\x91\x01U3\x85Ra\x03\xF3\x81R\x83\x85 \x92\x88\x16\x85R\x91\x90\x91R\x90\x82 Ta\x19\xE3\x91\x90a3\xBBV[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFC)T\xF2\xDCj\xF3\x90]xK\xA1\xBA\x1AQ\xF2[\x82\x90\xFA\x08g\xFF5;.qV\x85\x0CF}\x84\x86`@Qa\x1A=\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4`@Qc\x07\x81Y\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x07\x81Y\xCD\x90a\x1A\x95\x90\x87\x90\x87\x90\x87\x90`\x04\x01a3\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xAFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xC3W=`\0\x80>=`\0\xFD[PPPPPa\rc`\x01`\0\x80Q` a4\xC2\x839\x81Q\x91RUV[``a\x1A\xECa\x01\xF4a(\x9BV[\x90P\x90V[a\x1A\xFA\x82a\n\xB4V[a\x1B\x03\x81a!HV[a\x0B2\x83\x83a&/V[a\x1B\x15a)$V[3`\x01`\x01`\xA0\x1B\x03\x84\x16\x14a\x1BmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOnly operator can withdraw stake`D\x82\x01R`d\x01a\n\x11V[\x80a\x1B\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\t-\xCE\xCC-\x8D,\x84\r-\xCC\x8C\xAF\x04\r\x8C\xAD\xCC\xEE\x8D`c\x1B`D\x82\x01R`d\x01a\n\x11V[`\0[\x81\x81\x10\x15a\x1E{W3`\0\x90\x81Ra\x03\xF3` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R\x90\x91R\x81 \x84\x84\x84\x81\x81\x10a\x1B\xF2Wa\x1B\xF2a3\xCEV[\x90P` \x02\x015\x81T\x81\x10a\x1C\tWa\x1C\ta3\xCEV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R`\x03\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x90\x92\x01T\x91\x81\x01\x82\x90R\x91PB\x10\x15a\x1C\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FWithdrawal time not reached\0\0\0\0\0`D\x82\x01R`d\x01a\n\x11V[`\0\x81` \x01Q\x11a\x1C\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid withdrawal request\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\x11V[3`\0\x90\x81Ra\x03\xF3` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x89\x16\x84R\x90\x91R\x81 \x85\x85\x85\x81\x81\x10a\x1D&Wa\x1D&a3\xCEV[\x90P` \x02\x015\x81T\x81\x10a\x1D=Wa\x1D=a3\xCEV[\x90`\0R` `\0 \x90`\x03\x02\x01`\x01\x01\x81\x90UPa\x1Dx3\x82` \x01Q\x83`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16a(\xA8\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x86\x163\x7F:\xC2\xAD\xBF\xEB\xB3V\xD2p*#M\xF3\xBD\xFDv\xF2\x88\x8A\0)cF\xC5\x7Fo7\xAE\xA7OF\xA9\x87\x87\x87\x81\x81\x10a\x1D\xBCWa\x1D\xBCa3\xCEV[\x90P` \x02\x015\x85` \x01Q`@Qa\x1D\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4\x80Q` \x82\x01Q`@Qc\x19p\x1A\x83`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\xCB\x80\xD4\x18\x92a\x1E<\x92\x8A\x92\x91\x90`\x04\x01a3\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1EVW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1EjW=`\0\x80>=`\0\xFD[PP`\x01\x90\x93\x01\x92Pa\x1B\xB4\x91PPV[Pa\rc`\x01`\0\x80Q` a4\xC2\x839\x81Q\x91RUV[a\x01\xF6T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x11\x90a3AV[\x80`\0[\x81\x81\x10\x15a\x0B2W`\0a\x03\xF4`\0\x86\x86\x85\x81\x81\x10a\x1E\xE3Wa\x1E\xE3a3\xCEV[``\x02\x91\x90\x91\x015\x82RP` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x01T\x92\x82\x01\x83\x90R\x90\x92P\x81\x90\x03a\x1F1WPPa!\x0BV[a\x1F\x85\x86\x86\x85\x81\x81\x10a\x1FFWa\x1FFa3\xCEV[\x90P``\x02\x01`\0\x015\x83`\0\x01Q\x88\x88\x87\x81\x81\x10a\x1FgWa\x1Fga3\xCEV[\x90P``\x02\x01` \x01` \x81\x01\x90a\x1F\x7F\x91\x90a.\xE6V[\x84a)\x84V[a\x1F\xC4\x86\x86\x85\x81\x81\x10a\x1F\x9AWa\x1F\x9Aa3\xCEV[\x90P``\x02\x01`@\x01` \x81\x01\x90a\x1F\xB2\x91\x90a.\xE6V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x83a(\xA8V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85\x81\x81\x10a\x1F\xE1Wa\x1F\xE1a3\xCEV[\x90P``\x02\x01` \x01` \x81\x01\x90a\x1F\xF9\x91\x90a.\xE6V[`\x01`\x01`\xA0\x1B\x03\x16\x87\x87\x86\x81\x81\x10a \x14Wa \x14a3\xCEV[\x90P``\x02\x01`\0\x015\x7F\x90\x01~\x19\xB4\xF1[\xAB\x96\x828]E\xD8\x16\x94w(|Wb\xBA\x89u\x9F\xF8\xA7\x7FM8\xEC#\x84`@Qa O\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16caA?\x0F\x87\x87\x86\x81\x81\x10a \x98Wa \x98a3\xCEV[\x90P``\x02\x01` \x01` \x81\x01\x90a \xB0\x91\x90a.\xE6V[\x84Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81Ra \xD6\x92\x91\x90\x86\x90`\x04\x01a3\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\x04W=`\0\x80>=`\0\xFD[PPPPPP[`\x01\x01a\x1E\xC2V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\t\xB5WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\t\xB5V[a!R\x813a)\xE7V[PV[`\0a\x11\xB9\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a* V[`\0`\0\x80Q` a4\xA2\x839\x81Q\x91Ra!\x85\x84\x84a\x11\xC0V[a\"\x05W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua!\xBB3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\t\xB5V[`\0\x91PPa\t\xB5V[`\0\x80a\x01\xFAT\x11a\"rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTotal weight must be greater tha`D\x82\x01Ren zero`\xD0\x1B`d\x82\x01R`\x84\x01a\n\x11V[`\0a\"\x7Fa\x01\xF4a(\x85V[\x11a\"\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrNo tokens available`h\x1B`D\x82\x01R`d\x01a\n\x11V[`\0a\"\xCFa\x01\xF4a(\x85V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xECWa\"\xECa/gV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#\x15W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#3Wa#3a/gV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#\\W\x81` \x01` \x82\x02\x806\x837\x01\x90P[Pa\x01\xFAT\x90\x91P`\0\x80[\x85\x81\x10\x15a$\nW`\0a#~a\x01\xF4\x83a(\x8FV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x03\xF0` R`@\x90 T\x90\x91P\x80\x15a$\0W\x81\x87\x85\x81Q\x81\x10a#\xB4Wa#\xB4a3\xCEV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80\x86\x85\x81Q\x81\x10a#\xE7Wa#\xE7a3\xCEV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x83a#\xFC\x81a3\xFDV[\x94PP[PP`\x01\x01a#hV[P[`\0\x81\x11a$\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNo stakeToken available to lock\0`D\x82\x01R`d\x01a\n\x11V[`\0\x82Ba$k`\x01Ca3\xBBV[@3`@Q` \x01a$\xA2\x93\x92\x91\x90\x92\x83R` \x83\x01\x91\x90\x91R``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`@\x82\x01R`T\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca$\xC5\x91\x90a4\x16V[\x90P`\0\x80\x80[\x84\x81\x10\x15a%,W\x86\x81\x81Q\x81\x10a$\xE6Wa$\xE6a3\xCEV[` \x02` \x01\x01Q\x83a$\xF9\x91\x90a3\x84V[\x92P\x82\x84\x10\x15a%$W\x87\x81\x81Q\x81\x10a%\x15Wa%\x15a3\xCEV[` \x02` \x01\x01Q\x91Pa%,V[`\x01\x01a$\xCCV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03\xEF` R`@\x90 Ta%P\x83\x8Da\x11pV[\x10a%cWP\x99\x98PPPPPPPPPV[\x86\x81\x81Q\x81\x10a%uWa%ua3\xCEV[` \x02` \x01\x01Q\x86a%\x88\x91\x90a3\xBBV[\x95P\x87a%\x96`\x01\x87a3\xBBV[\x81Q\x81\x10a%\xA6Wa%\xA6a3\xCEV[` \x02` \x01\x01Q\x88\x82\x81Q\x81\x10a%\xC0Wa%\xC0a3\xCEV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x86a%\xE4`\x01\x87a3\xBBV[\x81Q\x81\x10a%\xF4Wa%\xF4a3\xCEV[` \x02` \x01\x01Q\x87\x82\x81Q\x81\x10a&\x0EWa&\x0Ea3\xCEV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x84a&#\x81a48V[\x95PPPPPPa$\x0CV[`\0`\0\x80Q` a4\xA2\x839\x81Q\x91Ra&J\x84\x84a\x11\xC0V[\x15a\"\x05W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\t\xB5V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x11\xB9V[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a'TWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a'H`\0\x80Q` a4\x82\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a'rW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0a\r\xE8\x81a!HV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a'\xD9WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra'\xD6\x91\x81\x01\x90a3\xE4V[`\x01[a(\x01W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\n\x11V[`\0\x80Q` a4\x82\x839\x81Q\x91R\x81\x14a(2W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\n\x11V[a\rc\x83\x83a+\tV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a'rW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\t\xB5\x82T\x90V[`\0a\x11\xB9\x83\x83a+_V[```\0a\x11\xB9\x83a+\x89V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra\rc\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa+\xE5V[`\0a\x11\xB9\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a,VV[a'ra,\xA5V[`\0\x80Q` a4\xC2\x839\x81Q\x91R\x80T`\x01\x19\x01a)VW`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x90UV[a\x0B2\x84\x85`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD\x86\x86\x86`@Q`$\x01a(\xD5\x93\x92\x91\x90a3\x97V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a)\xBC\x90\x84\x90a3\xBBV[\x90\x91UPPP`\0\x92\x83RPPa\x03\xF4` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x01UV[a)\xF1\x82\x82a\x11\xC0V[a\r\xE8W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\n\x11V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\"\x05W`\0a*D`\x01\x83a3\xBBV[\x85T\x90\x91P`\0\x90a*X\x90`\x01\x90a3\xBBV[\x90P\x80\x82\x14a*\xBDW`\0\x86`\0\x01\x82\x81T\x81\x10a*xWa*xa3\xCEV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a*\x9BWa*\x9Ba3\xCEV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a*\xCEWa*\xCEa4OV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\t\xB5V[a+\x12\x82a,\xEEV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a+WWa\rc\x82\x82a-SV[a\r\xE8a-\xC9V[`\0\x82`\0\x01\x82\x81T\x81\x10a+vWa+va3\xCEV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a+\xD9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a+\xC5W[PPPPP\x90P\x91\x90PV[`\0\x80` `\0\x84Q` \x86\x01`\0\x88Z\xF1\x80a,\x08W`@Q=`\0\x82>=\x81\xFD[PP`\0Q=\x91P\x81\x15a, W\x80`\x01\x14\x15a,-V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x0B2W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\n\x11V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta,\x9DWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\t\xB5V[P`\0a\t\xB5V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a'rW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a-$W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\n\x11V[`\0\x80Q` a4\x82\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa-p\x91\x90a4eV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a-\xABW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a-\xB0V[``\x91P[P\x91P\x91Pa-\xC0\x85\x83\x83a-\xE8V[\x95\x94PPPPPV[4\x15a'rW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a-\xFDWa-\xF8\x82a.DV[a\x11\xB9V[\x81Q\x15\x80\x15a.\x14WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a.=W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\n\x11V[P\x80a\x11\xB9V[\x80Q\x15a.TW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a.\x7FW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x11\xB9W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a.\xAEW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a.\xC6W`\0\x80\xFD[a.\xCF\x83a.\x97V[\x91Pa.\xDD` \x84\x01a.\x97V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a.\xF8W`\0\x80\xFD[a\x11\xB9\x82a.\x97V[`\0\x80`@\x83\x85\x03\x12\x15a/\x14W`\0\x80\xFD[a/\x1D\x83a.\x97V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a/=W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a/WW`\0\x80\xFD[\x825\x91Pa.\xDD` \x84\x01a.\x97V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a/\x90W`\0\x80\xFD[a/\x99\x83a.\x97V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\xB5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a/\xC6W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\xE0Wa/\xE0a/gV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a0\x0FWa0\x0Fa/gV[`@R\x81\x81R\x82\x82\x01` \x01\x87\x10\x15a0'W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15a0\x82W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a0[V[P\x93\x94\x93PPPPV[`@\x81R`\0a0\x9F`@\x83\x01\x85a0GV[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x86\x01\x92P`\0[\x81\x81\x10\x15a0\xDAW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a0\xBCV[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a0\xFBW`\0\x80\xFD[a1\x04\x84a.\x97V[\x92Pa1\x12` \x85\x01a.\x97V[\x91Pa1 `@\x85\x01a.\x97V[\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a1DW\x81\x81\x01Q\x83\x82\x01R` \x01a1,V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra1l\x81`@\x85\x01` \x87\x01a1)V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a1\x95W`\0\x80\xFD[a1\x9E\x84a.\x97V[\x92Pa1\xAC` \x85\x01a.\x97V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a1\xD3W`\0\x80\xFD[a1\xDC\x85a.\x97V[\x93Pa1\xEA` \x86\x01a.\x97V[\x92P`@\x85\x015\x91Pa1\xFF``\x86\x01a.\x97V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a2\x1FW`\0\x80\xFD[\x835\x92Pa1\xAC` \x85\x01a.\x97V[` \x81R`\0a\x11\xB9` \x83\x01\x84a0GV[`\0\x80`\0`@\x84\x86\x03\x12\x15a2WW`\0\x80\xFD[a2`\x84a.\x97V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2|W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a2\x8DW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xA4W`\0\x80\xFD[\x86` \x82`\x05\x1B\x84\x01\x01\x11\x15a2\xB9W`\0\x80\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[`\0\x80` \x83\x85\x03\x12\x15a2\xDDW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xF4W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a3\x05W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\x1CW`\0\x80\xFD[\x85` ``\x83\x02\x84\x01\x01\x11\x15a31W`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[` \x80\x82R`\x13\x90\x82\x01Rr'\xB76<\x90)\xBA0\xB5\xB4\xB73\xA6\xB0\xB70\xB3\xB2\xB9`i\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\t\xB5Wa\t\xB5a3nV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\t\xB5Wa\t\xB5a3nV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a3\xF6W`\0\x80\xFD[PQ\x91\x90PV[`\0`\x01\x82\x01a4\x0FWa4\x0Fa3nV[P`\x01\x01\x90V[`\0\x82a43WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x81a4GWa4Ga3nV[P`\0\x19\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82Qa4w\x81\x84` \x87\x01a1)V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\xA2dipfsX\"\x12 F_\x11[3\x13)\xBD\xCD\x95\xEE)\xC81{\xF2\xED\xFE\xBB\x88\xB1S*\xBF\xEF\x9E\xFA\xEAb\x05\xF6SdsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static NATIVESTAKING_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02rW`\x005`\xE0\x1C\x80cu\xE3\xB4p\x11a\x01OW\x80c\xAEk\x19\x83\x11a\0\xC1W\x80c\xCB\x95qz\x11a\0zW\x80c\xCB\x95qz\x14a\x08\xB3W\x80c\xD1\x1F)<\x14a\x08\xE1W\x80c\xD3e\xD2\xD6\x14a\t\x03W\x80c\xD5Gt\x1F\x14a\tJW\x80c\xD5]\t#\x14a\tjW\x80c\xDE\xE8\xBC\xE2\x14a\t\x8AW`\0\x80\xFD[\x80c\xAEk\x19\x83\x14a\x07\xCEW\x80c\xB0\x0B\xBAj\x14a\x08\x13W\x80c\xBE 0\x94\x14a\x083W\x80c\xBFn\xAC/\x14a\x08SW\x80c\xC0\xFF\x97\xEF\x14a\x08sW\x80c\xC9\x85\xD9i\x14a\x08\x93W`\0\x80\xFD[\x80c\x91\xD1HT\x11a\x01\x13W\x80c\x91\xD1HT\x14a\x06\xD2W\x80c\x9F\xCD\x11_\x14a\x06\xF2W\x80c\xA2\x17\xFD\xDF\x14a\x07CW\x80c\xA6\xFF\xE5'\x14a\x07XW\x80c\xAC\xC2\x16j\x14a\x07oW\x80c\xAD<\xB1\xCC\x14a\x07\x90W`\0\x80\xFD[\x80cu\xE3\xB4p\x14a\x06\x14W\x80cz\x8C\xA0\xBB\x14a\x06+W\x80c\x81\xF7\xD9B\x14a\x06dW\x80c\x89\xC7\xB9\x87\x14a\x06\x84W\x80c\x8D;\x9DZ\x14a\x06\xA4W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01\xE8W\x80cR\xD1\x90-\x11a\x01\xACW\x80cR\xD1\x90-\x14a\x04\xFDW\x80cW$\xB8N\x14a\x05\x12W\x80c[\x1AL$\x14a\x052W\x80c_A=8\x14a\x05\x92W\x80cc\x82\xD9\xAD\x14a\x05\xB5W\x80ck\xC0K)\x14a\x05\xD5W`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x04iW\x80c=t\xA7x\x14a\x04\x89W\x80c=\xF3m\xDF\x14a\x04\xAAW\x80cN\x82\x11\x02\x14a\x04\xCAW\x80cO\x1E\xF2\x86\x14a\x04\xEAW`\0\x80\xFD[\x80c\"\x82\x8C\xC2\x11a\x02:W\x80c\"\x82\x8C\xC2\x14a\x03\x81W\x80c$\x8A\x9C\xA3\x14a\x03\xA2W\x80c)\xF3\x84\xE2\x14a\x03\xC2W\x80c//\xF1]\x14a\x03\xE2W\x80c1\xA3\x80v\x14a\x04\x02W\x80c3\xF4\x90\x9F\x14a\x04IW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02wW\x80c\nY\xBBP\x14a\x02\xACW\x80c\x11\x0C\xBC\x80\x14a\x02\xF3W\x80c\x16hf\xC7\x14a\x03?W\x80c\x1F=\xC4\xE2\x14a\x03aW[`\0\x80\xFD[4\x80\x15a\x02\x83W`\0\x80\xFD[Pa\x02\x97a\x02\x926`\x04a.mV[a\t\xAAV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB8W`\0\x80\xFD[Pa\x02\xE5a\x02\xC76`\x04a.\xB3V[a\x03\xF5` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x02\xA3V[4\x80\x15a\x02\xFFW`\0\x80\xFD[Pa\x03'\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA3V[4\x80\x15a\x03KW`\0\x80\xFD[Pa\x03_a\x03Z6`\x04a.\xE6V[a\t\xBBV[\0[4\x80\x15a\x03mW`\0\x80\xFD[Pa\x03_a\x03|6`\x04a/\x01V[a\n`V[4\x80\x15a\x03\x8DW`\0\x80\xFD[Pa\x01\xF6Ta\x03'\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03\xAEW`\0\x80\xFD[Pa\x02\xE5a\x03\xBD6`\x04a/+V[a\n\xB4V[4\x80\x15a\x03\xCEW`\0\x80\xFD[Pa\x03_a\x03\xDD6`\x04a/+V[a\n\xD6V[4\x80\x15a\x03\xEEW`\0\x80\xFD[Pa\x03_a\x03\xFD6`\x04a/DV[a\x0B\x16V[4\x80\x15a\x04\x0EW`\0\x80\xFD[Pa\x02\xE5a\x04\x1D6`\x04a.\xB3V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[4\x80\x15a\x04UW`\0\x80\xFD[Pa\x03_a\x04d6`\x04a/DV[a\x0B8V[4\x80\x15a\x04uW`\0\x80\xFD[Pa\x03_a\x04\x846`\x04a/DV[a\r0V[4\x80\x15a\x04\x95W`\0\x80\xFD[Pa\x01\xF8Ta\x03'\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xB6W`\0\x80\xFD[Pa\x03_a\x04\xC56`\x04a.\xE6V[a\rhV[4\x80\x15a\x04\xD6W`\0\x80\xFD[Pa\x02\x97a\x04\xE56`\x04a.\xE6V[a\r\xBFV[a\x03_a\x04\xF86`\x04a/}V[a\r\xCDV[4\x80\x15a\x05\tW`\0\x80\xFD[Pa\x02\xE5a\r\xECV[4\x80\x15a\x05\x1EW`\0\x80\xFD[Pa\x03_a\x05-6`\x04a/\x01V[a\x0E\tV[4\x80\x15a\x05>W`\0\x80\xFD[Pa\x05sa\x05M6`\x04a/+V[a\x03\xF4` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xA3V[4\x80\x15a\x05\x9EW`\0\x80\xFD[Pa\x05\xA7a\x0E\xB6V[`@Qa\x02\xA3\x92\x91\x90a0\x8CV[4\x80\x15a\x05\xC1W`\0\x80\xFD[Pa\x03_a\x05\xD06`\x04a.\xB3V[a\x0F\x8AV[4\x80\x15a\x05\xE1W`\0\x80\xFD[Pa\x02\xE5a\x05\xF06`\x04a0\xE6V[a\x03\xF1` \x90\x81R`\0\x93\x84R`@\x80\x85 \x82R\x92\x84R\x82\x84 \x90R\x82R\x90 T\x81V[4\x80\x15a\x06 W`\0\x80\xFD[Pa\x02\xE5a\x01\xF9T\x81V[4\x80\x15a\x067W`\0\x80\xFD[Pa\x02\xE5a\x06F6`\x04a.\xB3V[a\x03\xF2` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x06pW`\0\x80\xFD[Pa\x03_a\x06\x7F6`\x04a/\x01V[a\x10\xA7V[4\x80\x15a\x06\x90W`\0\x80\xFD[Pa\x02\xE5a\x06\x9F6`\x04a.\xB3V[a\x11pV[4\x80\x15a\x06\xB0W`\0\x80\xFD[Pa\x02\xE5a\x06\xBF6`\x04a.\xE6V[a\x03\xEF` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x06\xDEW`\0\x80\xFD[Pa\x02\x97a\x06\xED6`\x04a/DV[a\x11\xC0V[4\x80\x15a\x06\xFEW`\0\x80\xFD[Pa\x02\xE5a\x07\r6`\x04a0\xE6V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x90\x81Ra\x03\xF1` \x90\x81R`@\x80\x83 \x94\x86\x16\x83R\x93\x81R\x83\x82 \x92\x90\x94\x16\x81R\x92R\x90 T\x90V[4\x80\x15a\x07OW`\0\x80\xFD[Pa\x02\xE5`\0\x81V[4\x80\x15a\x07dW`\0\x80\xFD[Pa\x02\xE5a\x01\xFAT\x81V[4\x80\x15a\x07{W`\0\x80\xFD[Pa\x01\xF7Ta\x03'\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\x9CW`\0\x80\xFD[Pa\x07\xC1`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02\xA3\x91\x90a1MV[4\x80\x15a\x07\xDAW`\0\x80\xFD[Pa\x07\xEEa\x07\xE96`\x04a1\x80V[a\x11\xF8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xA3V[4\x80\x15a\x08\x1FW`\0\x80\xFD[Pa\x03_a\x08.6`\x04a.\xE6V[a\x12SV[4\x80\x15a\x08?W`\0\x80\xFD[Pa\x03_a\x08N6`\x04a1\xBDV[a\x12\xAAV[4\x80\x15a\x08_W`\0\x80\xFD[Pa\x03_a\x08n6`\x04a1\x80V[a\x15\x04V[4\x80\x15a\x08\x7FW`\0\x80\xFD[Pa\x03_a\x08\x8E6`\x04a2\nV[a\x17(V[4\x80\x15a\x08\x9FW`\0\x80\xFD[Pa\x03_a\x08\xAE6`\x04a1\x80V[a\x18OV[4\x80\x15a\x08\xBFW`\0\x80\xFD[Pa\x02\xE5a\x08\xCE6`\x04a.\xE6V[a\x03\xF0` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x08\xEDW`\0\x80\xFD[Pa\x08\xF6a\x1A\xDFV[`@Qa\x02\xA3\x91\x90a2/V[4\x80\x15a\t\x0FW`\0\x80\xFD[Pa\x02\xE5a\t\x1E6`\x04a.\xB3V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81Ra\x03\xF2` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[4\x80\x15a\tVW`\0\x80\xFD[Pa\x03_a\te6`\x04a/DV[a\x1A\xF1V[4\x80\x15a\tvW`\0\x80\xFD[Pa\x03_a\t\x856`\x04a2BV[a\x1B\rV[4\x80\x15a\t\x96W`\0\x80\xFD[Pa\x03_a\t\xA56`\x04a2\xCAV[a\x1E\x93V[`\0a\t\xB5\x82a!\x13V[\x92\x91PPV[`\0a\t\xC6\x81a!HV[a\t\xD2a\x01\xF4\x83a!UV[a\n\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x1B\xDA\xD9[\x88\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`b\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x03\xF0` R`@\x80\x82 \x82\x90UQ\x7FzP/\xFF\xACP\xF2\xA88\xC9U\x04M\xECh\xB2rd\x17B\xA6\x86\x17\xAB\xE1nI\x82\xAA\xC3yu\x91\x90\xA2PPV[`\0a\nk\x81a!HV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03\xEF` R`@\x80\x82 \x85\x90UQ\x84\x92\x91\x7F\xC3`\x93\x99-6\x17;\xC34\x83\xBA\xE4N\xEC\xC3\x90?\x841\xE5\xC0\x8A\x07\xC1\x94Z\x89f\x133\x83\x91\xA3PPPV[`\0\x90\x81R`\0\x80Q` a4\xA2\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[`\0a\n\xE1\x81a!HV[a\x01\xF9\x82\x90U`@Q\x82\x90\x7F\x06\x9F\x0F\xCA-\xB0-]\0KsF\xA0\xE6g\x19{\x90q\xFEe\xEE\xAA\xA1\x1Dx\xE3\x82\x89\x85\x1D\xC2\x90`\0\x90\xA2PPV[a\x0B\x1F\x82a\n\xB4V[a\x0B(\x81a!HV[a\x0B2\x83\x83a!jV[PPPPV[a\x01\xF6T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0BcW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x11\x90a3AV[`\0a\x0Bn\x82a\"\x0FV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x03\xEF` R`@\x90 T\x90\x91P\x80a\x0B\x96\x83\x85a\x11pV[\x10\x15a\x0B\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInsufficient stake to lock\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\x11V[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x80\x83R` \x80\x84\x01\x86\x81R`\0\x8A\x81Ra\x03\xF4\x83R\x86\x81 \x95Q\x86T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x86\x16\x17\x86U\x90Q`\x01\x90\x95\x01\x94\x90\x94U\x90\x83Ra\x03\xF5\x81R\x83\x83 \x91\x87\x16\x83RR\x90\x81 \x80T\x83\x92\x90a\x0CS\x90\x84\x90a3\x84V[\x92PP\x81\x90UP\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7FV\xAC\xD9\xAAF\x83\x1C\xF3\xE8\x03\xC8\x9A`\xEE\xCA\"\xE7\r\x8D\xD3#;\xCB\x1F\x9EH\x9394\x0C\xA0\x01\x84`@Qa\x0C\xA0\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4`@Qc \x1A\x11\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c \x1A\x11\xB3\x90a\x0C\xF8\x90\x86\x90\x86\x90\x86\x90`\x04\x01a3\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r&W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\rYW`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rc\x82\x82a&/V[PPPV[`\0a\rs\x81a!HV[a\x01\xF8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F_\x8E+\x8B8\xD2\x0EPBt\xAB\xF2\x84z\xD3/Jm\x03\xB3\xCCu\xD0\x18\xFBW\x11Y9u\x96\xC7\x90`\0\x90\xA2PPV[`\0a\t\xB5a\x01\xF4\x83a&\xABV[a\r\xD5a&\xCDV[a\r\xDE\x82a'tV[a\r\xE8\x82\x82a'\x7FV[PPV[`\0a\r\xF6a(<V[P`\0\x80Q` a4\x82\x839\x81Q\x91R\x90V[`\0a\x0E\x14\x81a!HV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\xF0` R`@\x81 Ta\x01\xFA\x80T\x91\x92\x90\x91a\x0EB\x90\x84\x90a3\xBBV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\xF0` R`@\x81 \x83\x90Ua\x01\xFA\x80T\x84\x92\x90a\x0Ev\x90\x84\x90a3\x84V[\x90\x91UPP`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F0#5\xD1>\t\xEC\xAF\xF1\xF0\xAE\xD0\xB1\x9BZ\xD3\xA0T\t>\xA0qZ6\xB6\xA4\xBC\"\x9BR\x95\x82\x90`\0\x90\xA3PPPV[``\x80`\0a\x0E\xC6a\x01\xF4a(\x85V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xDEWa\x0E\xDEa/gV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x07W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[a\x0F\x18a\x01\xF4a(\x85V[\x81\x10\x15a\x0FvWa\x03\xF0`\0a\x0F0a\x01\xF4\x84a(\x8FV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x0FcWa\x0Fca3\xCEV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F\rV[Pa\x0F\x82a\x01\xF4a(\x9BV[\x93\x90\x92P\x90PV[`\0a\x0F\x95\x81a!HV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0F\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqzero token address`p\x1B`D\x82\x01R`d\x01a\n\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rnzero to address`\x88\x1B`D\x82\x01R`d\x01a\n\x11V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\rc\x90\x83\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x96\x91\x90a3\xE4V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x91\x90a(\xA8V[`\0a\x10\xB2\x81a!HV[a\x10\xBEa\x01\xF4\x84a)\x07V[a\x11\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsToken already exists``\x1B`D\x82\x01R`d\x01a\n\x11V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\xF0` R`@\x81 \x83\x90Ua\x01\xFA\x80T\x84\x92\x90a\x110\x90\x84\x90a3\x84V[\x90\x91UPP`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\x0C\x96U\xA8\r\xB9\xD3N#\xDD\x9DH<\xB2\xB6\x84\xE4v\x11T\xA6o\x93]\x9BB\xC1t\x89-\xD3%\x90`\0\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83Ra\x03\xF2\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 Ta\x11\xB9\x91\x90a3\xBBV[\x93\x92PPPV[`\0\x91\x82R`\0\x80Q` a4\xA2\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x03\xF3` R\x82`\0R`@`\0 ` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x12!W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x90\x92\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x94P\x90\x92P\x90P\x83V[`\0a\x12^\x81a!HV[a\x01\xF6\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB9\xE8\x9D\x19:q\x1D\xD2\x9A%3(\xFA\xAEn\xF5;\xD4\x1F\x10\x01sDl\x82 \xA3\x8A\x02.$\x0C\x90`\0\x90\xA2PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15\x80\x15a\x12\xF0WP\x82[\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x13\rWP0;\x15[\x90P\x81\x15\x80\x15a\x13\x1BWP\x80\x15[\x15a\x139W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x13cW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x13ka)\x1CV[a\x13sa)\x1CV[a\x13{a)\x1CV[a\x13\x83a)\x1CV[a\x13\x8E`\0\x8Aa!jV[P`\x01`\x01`\xA0\x1B\x03\x88\x16a\x13\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FNativeStaking: Invalid StakingMa`D\x82\x01Rd70\xB3\xB2\xB9`\xD9\x1B`d\x82\x01R`\x84\x01a\n\x11V[a\x01\xF6\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB9\xE8\x9D\x19:q\x1D\xD2\x9A%3(\xFA\xAEn\xF5;\xD4\x1F\x10\x01sDl\x82 \xA3\x8A\x02.$\x0C\x90`\0\x90\xA2a\x01\xF9\x87\x90U`\x01`\x01`\xA0\x1B\x03\x86\x16a\x14\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNativeStaking: Invalid Fee Token`D\x82\x01R`d\x01a\n\x11V[a\x01\xF8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x17\x90U\x83\x15a\x14\xF9W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[\x82a\x15\x11a\x01\xF4\x82a&\xABV[a\x15SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1B\xDA\xD9[\x88\x1B\x9B\xDD\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`j\x1B`D\x82\x01R`d\x01a\n\x11V[a\x15[a)$V[3`\x01`\x01`\xA0\x1B\x03\x84\x16\x14a\x15\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FOnly operator can stake\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\x11V[a\x15\xC8`\x01`\x01`\xA0\x1B\x03\x85\x1630\x85a)\\V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81Ra\x03\xF1` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x80T\x84\x92\x90a\x16\x08\x90\x84\x90a3\x84V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81Ra\x03\xF2` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x80T\x84\x92\x90a\x16E\x90\x84\x90a3\x84V[\x90\x91UPP`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x90\x85\x16\x903\x90\x7Fna>PM\xCB\xE2g\xF6\x0E)[\x08\xE0\xA2\x11\xB6=\xB8i\rf\x0E^\xD4\xF8d\xD4\t\xBBf \x90` \x01`@Q\x80\x91\x03\x90\xA4`@QcS\x1B\x0Bs`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA66\x16\xE6\x90a\x16\xDF\x90\x86\x90\x88\x90\x87\x90`\x04\x01a3\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\rW=`\0\x80>=`\0\xFD[PPPPa\x0B2`\x01`\0\x80Q` a4\xC2\x839\x81Q\x91RUV[a\x01\xF6T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x11\x90a3AV[`\0\x83\x81Ra\x03\xF4` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x82R`\x01\x01T\x91\x81\x01\x82\x90R\x91\x03a\x17\x90WPPPPV[a\x17\xA4\x84\x82`\0\x01Q\x85\x84` \x01Qa)\x84V[\x80`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F\xB8\xECr\xF7\xC3EH\xFB\xB7\xD0\xE8\x8D\x93\xF2\xAA\x1E\x8C7\x11\x17y\x86\xB4\xD5\xBC\r\xAB\xF5\x86\x8C\xA1\xAB\x84` \x01Q`@Qa\x17\xF2\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x80Q` \x82\x01Q`@Qc\x18\xDA\x93]`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c1\xB5&\xBA\x92a\x0C\xF8\x92\x88\x92\x91\x90`\x04\x01a3\x97V[a\x18Wa)$V[\x80a\x18b\x83\x85a\x11pV[\x10\x15a\x18\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInsufficient stake`p\x1B`D\x82\x01R`d\x01a\n\x11V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81Ra\x03\xF1` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x18\xE5\x90\x84\x90a3\xBBV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81Ra\x03\xF2` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x19\"\x90\x84\x90a3\xBBV[\x90\x91UPP3`\0\x90\x81Ra\x03\xF3` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x85R\x90\x83R\x92\x81\x90 \x81Q``\x81\x01\x83R\x93\x86\x16\x84R\x91\x83\x01\x84\x90Ra\x01\xF9T\x91\x92\x91\x90\x82\x01\x90a\x19v\x90Ba3\x84V[\x90R\x81T`\x01\x80\x82\x01\x84U`\0\x93\x84R` \x80\x85 \x84Q`\x03\x90\x94\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x81U\x84\x82\x01Q\x81\x84\x01U`@\x94\x85\x01Q`\x02\x90\x91\x01U3\x85Ra\x03\xF3\x81R\x83\x85 \x92\x88\x16\x85R\x91\x90\x91R\x90\x82 Ta\x19\xE3\x91\x90a3\xBBV[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFC)T\xF2\xDCj\xF3\x90]xK\xA1\xBA\x1AQ\xF2[\x82\x90\xFA\x08g\xFF5;.qV\x85\x0CF}\x84\x86`@Qa\x1A=\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4`@Qc\x07\x81Y\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x07\x81Y\xCD\x90a\x1A\x95\x90\x87\x90\x87\x90\x87\x90`\x04\x01a3\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xAFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xC3W=`\0\x80>=`\0\xFD[PPPPPa\rc`\x01`\0\x80Q` a4\xC2\x839\x81Q\x91RUV[``a\x1A\xECa\x01\xF4a(\x9BV[\x90P\x90V[a\x1A\xFA\x82a\n\xB4V[a\x1B\x03\x81a!HV[a\x0B2\x83\x83a&/V[a\x1B\x15a)$V[3`\x01`\x01`\xA0\x1B\x03\x84\x16\x14a\x1BmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOnly operator can withdraw stake`D\x82\x01R`d\x01a\n\x11V[\x80a\x1B\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\t-\xCE\xCC-\x8D,\x84\r-\xCC\x8C\xAF\x04\r\x8C\xAD\xCC\xEE\x8D`c\x1B`D\x82\x01R`d\x01a\n\x11V[`\0[\x81\x81\x10\x15a\x1E{W3`\0\x90\x81Ra\x03\xF3` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R\x90\x91R\x81 \x84\x84\x84\x81\x81\x10a\x1B\xF2Wa\x1B\xF2a3\xCEV[\x90P` \x02\x015\x81T\x81\x10a\x1C\tWa\x1C\ta3\xCEV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R`\x03\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x90\x92\x01T\x91\x81\x01\x82\x90R\x91PB\x10\x15a\x1C\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FWithdrawal time not reached\0\0\0\0\0`D\x82\x01R`d\x01a\n\x11V[`\0\x81` \x01Q\x11a\x1C\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid withdrawal request\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\x11V[3`\0\x90\x81Ra\x03\xF3` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x89\x16\x84R\x90\x91R\x81 \x85\x85\x85\x81\x81\x10a\x1D&Wa\x1D&a3\xCEV[\x90P` \x02\x015\x81T\x81\x10a\x1D=Wa\x1D=a3\xCEV[\x90`\0R` `\0 \x90`\x03\x02\x01`\x01\x01\x81\x90UPa\x1Dx3\x82` \x01Q\x83`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16a(\xA8\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x86\x163\x7F:\xC2\xAD\xBF\xEB\xB3V\xD2p*#M\xF3\xBD\xFDv\xF2\x88\x8A\0)cF\xC5\x7Fo7\xAE\xA7OF\xA9\x87\x87\x87\x81\x81\x10a\x1D\xBCWa\x1D\xBCa3\xCEV[\x90P` \x02\x015\x85` \x01Q`@Qa\x1D\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4\x80Q` \x82\x01Q`@Qc\x19p\x1A\x83`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\xCB\x80\xD4\x18\x92a\x1E<\x92\x8A\x92\x91\x90`\x04\x01a3\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1EVW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1EjW=`\0\x80>=`\0\xFD[PP`\x01\x90\x93\x01\x92Pa\x1B\xB4\x91PPV[Pa\rc`\x01`\0\x80Q` a4\xC2\x839\x81Q\x91RUV[a\x01\xF6T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x11\x90a3AV[\x80`\0[\x81\x81\x10\x15a\x0B2W`\0a\x03\xF4`\0\x86\x86\x85\x81\x81\x10a\x1E\xE3Wa\x1E\xE3a3\xCEV[``\x02\x91\x90\x91\x015\x82RP` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x01T\x92\x82\x01\x83\x90R\x90\x92P\x81\x90\x03a\x1F1WPPa!\x0BV[a\x1F\x85\x86\x86\x85\x81\x81\x10a\x1FFWa\x1FFa3\xCEV[\x90P``\x02\x01`\0\x015\x83`\0\x01Q\x88\x88\x87\x81\x81\x10a\x1FgWa\x1Fga3\xCEV[\x90P``\x02\x01` \x01` \x81\x01\x90a\x1F\x7F\x91\x90a.\xE6V[\x84a)\x84V[a\x1F\xC4\x86\x86\x85\x81\x81\x10a\x1F\x9AWa\x1F\x9Aa3\xCEV[\x90P``\x02\x01`@\x01` \x81\x01\x90a\x1F\xB2\x91\x90a.\xE6V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x83a(\xA8V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85\x81\x81\x10a\x1F\xE1Wa\x1F\xE1a3\xCEV[\x90P``\x02\x01` \x01` \x81\x01\x90a\x1F\xF9\x91\x90a.\xE6V[`\x01`\x01`\xA0\x1B\x03\x16\x87\x87\x86\x81\x81\x10a \x14Wa \x14a3\xCEV[\x90P``\x02\x01`\0\x015\x7F\x90\x01~\x19\xB4\xF1[\xAB\x96\x828]E\xD8\x16\x94w(|Wb\xBA\x89u\x9F\xF8\xA7\x7FM8\xEC#\x84`@Qa O\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16caA?\x0F\x87\x87\x86\x81\x81\x10a \x98Wa \x98a3\xCEV[\x90P``\x02\x01` \x01` \x81\x01\x90a \xB0\x91\x90a.\xE6V[\x84Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81Ra \xD6\x92\x91\x90\x86\x90`\x04\x01a3\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\x04W=`\0\x80>=`\0\xFD[PPPPPP[`\x01\x01a\x1E\xC2V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\t\xB5WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\t\xB5V[a!R\x813a)\xE7V[PV[`\0a\x11\xB9\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a* V[`\0`\0\x80Q` a4\xA2\x839\x81Q\x91Ra!\x85\x84\x84a\x11\xC0V[a\"\x05W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua!\xBB3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\t\xB5V[`\0\x91PPa\t\xB5V[`\0\x80a\x01\xFAT\x11a\"rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTotal weight must be greater tha`D\x82\x01Ren zero`\xD0\x1B`d\x82\x01R`\x84\x01a\n\x11V[`\0a\"\x7Fa\x01\xF4a(\x85V[\x11a\"\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrNo tokens available`h\x1B`D\x82\x01R`d\x01a\n\x11V[`\0a\"\xCFa\x01\xF4a(\x85V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xECWa\"\xECa/gV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#\x15W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#3Wa#3a/gV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#\\W\x81` \x01` \x82\x02\x806\x837\x01\x90P[Pa\x01\xFAT\x90\x91P`\0\x80[\x85\x81\x10\x15a$\nW`\0a#~a\x01\xF4\x83a(\x8FV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x03\xF0` R`@\x90 T\x90\x91P\x80\x15a$\0W\x81\x87\x85\x81Q\x81\x10a#\xB4Wa#\xB4a3\xCEV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80\x86\x85\x81Q\x81\x10a#\xE7Wa#\xE7a3\xCEV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x83a#\xFC\x81a3\xFDV[\x94PP[PP`\x01\x01a#hV[P[`\0\x81\x11a$\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FNo stakeToken available to lock\0`D\x82\x01R`d\x01a\n\x11V[`\0\x82Ba$k`\x01Ca3\xBBV[@3`@Q` \x01a$\xA2\x93\x92\x91\x90\x92\x83R` \x83\x01\x91\x90\x91R``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`@\x82\x01R`T\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca$\xC5\x91\x90a4\x16V[\x90P`\0\x80\x80[\x84\x81\x10\x15a%,W\x86\x81\x81Q\x81\x10a$\xE6Wa$\xE6a3\xCEV[` \x02` \x01\x01Q\x83a$\xF9\x91\x90a3\x84V[\x92P\x82\x84\x10\x15a%$W\x87\x81\x81Q\x81\x10a%\x15Wa%\x15a3\xCEV[` \x02` \x01\x01Q\x91Pa%,V[`\x01\x01a$\xCCV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03\xEF` R`@\x90 Ta%P\x83\x8Da\x11pV[\x10a%cWP\x99\x98PPPPPPPPPV[\x86\x81\x81Q\x81\x10a%uWa%ua3\xCEV[` \x02` \x01\x01Q\x86a%\x88\x91\x90a3\xBBV[\x95P\x87a%\x96`\x01\x87a3\xBBV[\x81Q\x81\x10a%\xA6Wa%\xA6a3\xCEV[` \x02` \x01\x01Q\x88\x82\x81Q\x81\x10a%\xC0Wa%\xC0a3\xCEV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x86a%\xE4`\x01\x87a3\xBBV[\x81Q\x81\x10a%\xF4Wa%\xF4a3\xCEV[` \x02` \x01\x01Q\x87\x82\x81Q\x81\x10a&\x0EWa&\x0Ea3\xCEV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x84a&#\x81a48V[\x95PPPPPPa$\x0CV[`\0`\0\x80Q` a4\xA2\x839\x81Q\x91Ra&J\x84\x84a\x11\xC0V[\x15a\"\x05W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\t\xB5V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x11\xB9V[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a'TWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a'H`\0\x80Q` a4\x82\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a'rW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0a\r\xE8\x81a!HV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a'\xD9WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra'\xD6\x91\x81\x01\x90a3\xE4V[`\x01[a(\x01W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\n\x11V[`\0\x80Q` a4\x82\x839\x81Q\x91R\x81\x14a(2W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\n\x11V[a\rc\x83\x83a+\tV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a'rW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\t\xB5\x82T\x90V[`\0a\x11\xB9\x83\x83a+_V[```\0a\x11\xB9\x83a+\x89V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra\rc\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa+\xE5V[`\0a\x11\xB9\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a,VV[a'ra,\xA5V[`\0\x80Q` a4\xC2\x839\x81Q\x91R\x80T`\x01\x19\x01a)VW`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x90UV[a\x0B2\x84\x85`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD\x86\x86\x86`@Q`$\x01a(\xD5\x93\x92\x91\x90a3\x97V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81Ra\x03\xF5` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a)\xBC\x90\x84\x90a3\xBBV[\x90\x91UPPP`\0\x92\x83RPPa\x03\xF4` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x01UV[a)\xF1\x82\x82a\x11\xC0V[a\r\xE8W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\n\x11V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\"\x05W`\0a*D`\x01\x83a3\xBBV[\x85T\x90\x91P`\0\x90a*X\x90`\x01\x90a3\xBBV[\x90P\x80\x82\x14a*\xBDW`\0\x86`\0\x01\x82\x81T\x81\x10a*xWa*xa3\xCEV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a*\x9BWa*\x9Ba3\xCEV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a*\xCEWa*\xCEa4OV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\t\xB5V[a+\x12\x82a,\xEEV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a+WWa\rc\x82\x82a-SV[a\r\xE8a-\xC9V[`\0\x82`\0\x01\x82\x81T\x81\x10a+vWa+va3\xCEV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a+\xD9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a+\xC5W[PPPPP\x90P\x91\x90PV[`\0\x80` `\0\x84Q` \x86\x01`\0\x88Z\xF1\x80a,\x08W`@Q=`\0\x82>=\x81\xFD[PP`\0Q=\x91P\x81\x15a, W\x80`\x01\x14\x15a,-V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x0B2W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\n\x11V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta,\x9DWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\t\xB5V[P`\0a\t\xB5V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a'rW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a-$W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\n\x11V[`\0\x80Q` a4\x82\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa-p\x91\x90a4eV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a-\xABW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a-\xB0V[``\x91P[P\x91P\x91Pa-\xC0\x85\x83\x83a-\xE8V[\x95\x94PPPPPV[4\x15a'rW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a-\xFDWa-\xF8\x82a.DV[a\x11\xB9V[\x81Q\x15\x80\x15a.\x14WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a.=W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\n\x11V[P\x80a\x11\xB9V[\x80Q\x15a.TW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a.\x7FW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x11\xB9W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a.\xAEW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a.\xC6W`\0\x80\xFD[a.\xCF\x83a.\x97V[\x91Pa.\xDD` \x84\x01a.\x97V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a.\xF8W`\0\x80\xFD[a\x11\xB9\x82a.\x97V[`\0\x80`@\x83\x85\x03\x12\x15a/\x14W`\0\x80\xFD[a/\x1D\x83a.\x97V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a/=W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a/WW`\0\x80\xFD[\x825\x91Pa.\xDD` \x84\x01a.\x97V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a/\x90W`\0\x80\xFD[a/\x99\x83a.\x97V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\xB5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a/\xC6W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\xE0Wa/\xE0a/gV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a0\x0FWa0\x0Fa/gV[`@R\x81\x81R\x82\x82\x01` \x01\x87\x10\x15a0'W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15a0\x82W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a0[V[P\x93\x94\x93PPPPV[`@\x81R`\0a0\x9F`@\x83\x01\x85a0GV[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x86\x01\x92P`\0[\x81\x81\x10\x15a0\xDAW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a0\xBCV[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a0\xFBW`\0\x80\xFD[a1\x04\x84a.\x97V[\x92Pa1\x12` \x85\x01a.\x97V[\x91Pa1 `@\x85\x01a.\x97V[\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a1DW\x81\x81\x01Q\x83\x82\x01R` \x01a1,V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra1l\x81`@\x85\x01` \x87\x01a1)V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a1\x95W`\0\x80\xFD[a1\x9E\x84a.\x97V[\x92Pa1\xAC` \x85\x01a.\x97V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a1\xD3W`\0\x80\xFD[a1\xDC\x85a.\x97V[\x93Pa1\xEA` \x86\x01a.\x97V[\x92P`@\x85\x015\x91Pa1\xFF``\x86\x01a.\x97V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a2\x1FW`\0\x80\xFD[\x835\x92Pa1\xAC` \x85\x01a.\x97V[` \x81R`\0a\x11\xB9` \x83\x01\x84a0GV[`\0\x80`\0`@\x84\x86\x03\x12\x15a2WW`\0\x80\xFD[a2`\x84a.\x97V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2|W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a2\x8DW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xA4W`\0\x80\xFD[\x86` \x82`\x05\x1B\x84\x01\x01\x11\x15a2\xB9W`\0\x80\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[`\0\x80` \x83\x85\x03\x12\x15a2\xDDW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xF4W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a3\x05W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\x1CW`\0\x80\xFD[\x85` ``\x83\x02\x84\x01\x01\x11\x15a31W`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[` \x80\x82R`\x13\x90\x82\x01Rr'\xB76<\x90)\xBA0\xB5\xB4\xB73\xA6\xB0\xB70\xB3\xB2\xB9`i\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\t\xB5Wa\t\xB5a3nV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\t\xB5Wa\t\xB5a3nV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a3\xF6W`\0\x80\xFD[PQ\x91\x90PV[`\0`\x01\x82\x01a4\x0FWa4\x0Fa3nV[P`\x01\x01\x90V[`\0\x82a43WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x81a4GWa4Ga3nV[P`\0\x19\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82Qa4w\x81\x84` \x87\x01a1)V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\xA2dipfsX\"\x12 F_\x11[3\x13)\xBD\xCD\x95\xEE)\xC81{\xF2\xED\xFE\xBB\x88\xB1S*\xBF\xEF\x9E\xFA\xEAb\x05\xF6SdsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static NATIVESTAKING_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct NativeStaking<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for NativeStaking<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for NativeStaking<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for NativeStaking<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for NativeStaking<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(NativeStaking))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> NativeStaking<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                NATIVESTAKING_ABI.clone(),
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
                NATIVESTAKING_ABI.clone(),
                NATIVESTAKING_BYTECODE.clone().into(),
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
        ///Calls the contract's `I_GENERATOR_CALLBACK` (0x110cbc80) function
        pub fn i_generator_callback(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([17, 12, 188, 128], ())
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
        ///Calls the contract's `addStakeToken` (0x81f7d942) function
        pub fn add_stake_token(
            &self,
            token: ::ethers::core::types::Address,
            weight: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 247, 217, 66], (token, weight))
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
        ///Calls the contract's `feeRewardToken` (0x3d74a778) function
        pub fn fee_reward_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([61, 116, 167, 120], ())
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
        ///Calls the contract's `getOperatorLockedAmount` (0x31a38076) function
        pub fn get_operator_locked_amount(
            &self,
            stake_token: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 163, 128, 118], (stake_token, operator))
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
            account: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 205, 17, 95], (stake_token, account, operator))
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
        ///Calls the contract's `initialize` (0xbe203094) function
        pub fn initialize(
            &self,
            admin: ::ethers::core::types::Address,
            staking_manager: ::ethers::core::types::Address,
            withdrawal_duration: ::ethers::core::types::U256,
            fee_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [190, 32, 48, 148],
                    (admin, staking_manager, withdrawal_duration, fee_token),
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
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 255, 151, 239], (job_id, operator, p2))
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
        ///Calls the contract's `operatorstakeAmounts` (0x7a8ca0bb) function
        pub fn operatorstake_amounts(
            &self,
            stake_token: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 140, 160, 187], (stake_token, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeStakeToken` (0x166866c7) function
        pub fn remove_stake_token(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 104, 102, 199], token)
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
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 61, 196, 226], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeRewardToken` (0x3df36ddf) function
        pub fn set_fee_reward_token(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 243, 109, 223], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStakeTokenSelectionWeight` (0x5724b84e) function
        pub fn set_stake_token_selection_weight(
            &self,
            token: ::ethers::core::types::Address,
            weight: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 36, 184, 78], (token, weight))
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
        ///Calls the contract's `setWithdrawalDuration` (0x29f384e2) function
        pub fn set_withdrawal_duration(
            &self,
            duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 243, 132, 226], duration)
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
        ///Calls the contract's `stakeAmounts` (0x6bc04b29) function
        pub fn stake_amounts(
            &self,
            stake_token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([107, 192, 75, 41], (stake_token, account, operator))
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
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
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
        ///Calls the contract's `withdrawalDuration` (0x75e3b470) function
        pub fn withdrawal_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([117, 227, 180, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawalRequests` (0xae6b1983) function
        pub fn withdrawal_requests(
            &self,
            account: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([174, 107, 25, 131], (account, operator, p2))
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
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradedFilter> {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NativeStakingEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for NativeStaking<M>
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
    pub enum NativeStakingErrors {
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
    impl ::ethers::core::abi::AbiDecode for NativeStakingErrors {
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
    impl ::ethers::core::abi::AbiEncode for NativeStakingErrors {
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
    impl ::ethers::contract::ContractRevert for NativeStakingErrors {
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
    impl ::core::fmt::Display for NativeStakingErrors {
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
    impl ::core::convert::From<::std::string::String> for NativeStakingErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation> for NativeStakingErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount> for NativeStakingErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for NativeStakingErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for NativeStakingErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for NativeStakingErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedCall> for NativeStakingErrors {
        fn from(value: FailedCall) -> Self {
            Self::FailedCall(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for NativeStakingErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for NativeStakingErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for NativeStakingErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for NativeStakingErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for NativeStakingErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for NativeStakingErrors {
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
    pub enum NativeStakingEvents {
        AmountToLockSetFilter(AmountToLockSetFilter),
        FeeRewardTokenSetFilter(FeeRewardTokenSetFilter),
        InitializedFilter(InitializedFilter),
        JobSlashedFilter(JobSlashedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        StakeLockedFilter(StakeLockedFilter),
        StakeTokenAddedFilter(StakeTokenAddedFilter),
        StakeTokenRemovedFilter(StakeTokenRemovedFilter),
        StakeTokenSelectionWeightSetFilter(StakeTokenSelectionWeightSetFilter),
        StakeUnlockedFilter(StakeUnlockedFilter),
        StakeWithdrawalRequestedFilter(StakeWithdrawalRequestedFilter),
        StakeWithdrawnFilter(StakeWithdrawnFilter),
        StakedFilter(StakedFilter),
        StakingManagerSetFilter(StakingManagerSetFilter),
        UpgradedFilter(UpgradedFilter),
        WithdrawalDurationSetFilter(WithdrawalDurationSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for NativeStakingEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AmountToLockSetFilter::decode_log(log) {
                return Ok(NativeStakingEvents::AmountToLockSetFilter(decoded));
            }
            if let Ok(decoded) = FeeRewardTokenSetFilter::decode_log(log) {
                return Ok(NativeStakingEvents::FeeRewardTokenSetFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = JobSlashedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::JobSlashedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = StakeLockedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::StakeLockedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenAddedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::StakeTokenAddedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenRemovedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::StakeTokenRemovedFilter(decoded));
            }
            if let Ok(decoded) = StakeTokenSelectionWeightSetFilter::decode_log(log) {
                return Ok(NativeStakingEvents::StakeTokenSelectionWeightSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StakeUnlockedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::StakeUnlockedFilter(decoded));
            }
            if let Ok(decoded) = StakeWithdrawalRequestedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::StakeWithdrawalRequestedFilter(decoded));
            }
            if let Ok(decoded) = StakeWithdrawnFilter::decode_log(log) {
                return Ok(NativeStakingEvents::StakeWithdrawnFilter(decoded));
            }
            if let Ok(decoded) = StakedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::StakedFilter(decoded));
            }
            if let Ok(decoded) = StakingManagerSetFilter::decode_log(log) {
                return Ok(NativeStakingEvents::StakingManagerSetFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(NativeStakingEvents::UpgradedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalDurationSetFilter::decode_log(log) {
                return Ok(NativeStakingEvents::WithdrawalDurationSetFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for NativeStakingEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AmountToLockSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeRewardTokenSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::JobSlashedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalDurationSetFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AmountToLockSetFilter> for NativeStakingEvents {
        fn from(value: AmountToLockSetFilter) -> Self {
            Self::AmountToLockSetFilter(value)
        }
    }
    impl ::core::convert::From<FeeRewardTokenSetFilter> for NativeStakingEvents {
        fn from(value: FeeRewardTokenSetFilter) -> Self {
            Self::FeeRewardTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for NativeStakingEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<JobSlashedFilter> for NativeStakingEvents {
        fn from(value: JobSlashedFilter) -> Self {
            Self::JobSlashedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for NativeStakingEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for NativeStakingEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for NativeStakingEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<StakeLockedFilter> for NativeStakingEvents {
        fn from(value: StakeLockedFilter) -> Self {
            Self::StakeLockedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenAddedFilter> for NativeStakingEvents {
        fn from(value: StakeTokenAddedFilter) -> Self {
            Self::StakeTokenAddedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenRemovedFilter> for NativeStakingEvents {
        fn from(value: StakeTokenRemovedFilter) -> Self {
            Self::StakeTokenRemovedFilter(value)
        }
    }
    impl ::core::convert::From<StakeTokenSelectionWeightSetFilter> for NativeStakingEvents {
        fn from(value: StakeTokenSelectionWeightSetFilter) -> Self {
            Self::StakeTokenSelectionWeightSetFilter(value)
        }
    }
    impl ::core::convert::From<StakeUnlockedFilter> for NativeStakingEvents {
        fn from(value: StakeUnlockedFilter) -> Self {
            Self::StakeUnlockedFilter(value)
        }
    }
    impl ::core::convert::From<StakeWithdrawalRequestedFilter> for NativeStakingEvents {
        fn from(value: StakeWithdrawalRequestedFilter) -> Self {
            Self::StakeWithdrawalRequestedFilter(value)
        }
    }
    impl ::core::convert::From<StakeWithdrawnFilter> for NativeStakingEvents {
        fn from(value: StakeWithdrawnFilter) -> Self {
            Self::StakeWithdrawnFilter(value)
        }
    }
    impl ::core::convert::From<StakedFilter> for NativeStakingEvents {
        fn from(value: StakedFilter) -> Self {
            Self::StakedFilter(value)
        }
    }
    impl ::core::convert::From<StakingManagerSetFilter> for NativeStakingEvents {
        fn from(value: StakingManagerSetFilter) -> Self {
            Self::StakingManagerSetFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for NativeStakingEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalDurationSetFilter> for NativeStakingEvents {
        fn from(value: WithdrawalDurationSetFilter) -> Self {
            Self::WithdrawalDurationSetFilter(value)
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
        pub token: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `getOperatorLockedAmount` function with signature `getOperatorLockedAmount(address,address)` and selector `0x31a38076`
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
        name = "getOperatorLockedAmount",
        abi = "getOperatorLockedAmount(address,address)"
    )]
    pub struct GetOperatorLockedAmountCall {
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
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256,address)` and selector `0xbe203094`
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
        abi = "initialize(address,address,uint256,address)"
    )]
    pub struct InitializeCall {
        pub admin: ::ethers::core::types::Address,
        pub staking_manager: ::ethers::core::types::Address,
        pub withdrawal_duration: ::ethers::core::types::U256,
        pub fee_token: ::ethers::core::types::Address,
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
        pub p2: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `operatorstakeAmounts` function with signature `operatorstakeAmounts(address,address)` and selector `0x7a8ca0bb`
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
        name = "operatorstakeAmounts",
        abi = "operatorstakeAmounts(address,address)"
    )]
    pub struct OperatorstakeAmountsCall {
        pub stake_token: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
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
        pub token: ::ethers::core::types::Address,
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
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        pub token: ::ethers::core::types::Address,
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
        pub token: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `setWithdrawalDuration` function with signature `setWithdrawalDuration(uint256)` and selector `0x29f384e2`
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
    #[ethcall(name = "setWithdrawalDuration", abi = "setWithdrawalDuration(uint256)")]
    pub struct SetWithdrawalDurationCall {
        pub duration: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `stakeAmounts` function with signature `stakeAmounts(address,address,address)` and selector `0x6bc04b29`
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
    #[ethcall(name = "stakeAmounts", abi = "stakeAmounts(address,address,address)")]
    pub struct StakeAmountsCall {
        pub stake_token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `withdrawalDuration` function with signature `withdrawalDuration()` and selector `0x75e3b470`
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
    #[ethcall(name = "withdrawalDuration", abi = "withdrawalDuration()")]
    pub struct WithdrawalDurationCall;
    ///Container type for all input parameters for the `withdrawalRequests` function with signature `withdrawalRequests(address,address,uint256)` and selector `0xae6b1983`
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
        name = "withdrawalRequests",
        abi = "withdrawalRequests(address,address,uint256)"
    )]
    pub struct WithdrawalRequestsCall {
        pub account: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
        pub p2: ::ethers::core::types::U256,
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
    pub enum NativeStakingCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        IGeneratorCallback(IGeneratorCallbackCall),
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        AddStakeToken(AddStakeTokenCall),
        AmountToLock(AmountToLockCall),
        EmergencyWithdraw(EmergencyWithdrawCall),
        FeeRewardToken(FeeRewardTokenCall),
        GetOperatorActiveStakeAmount(GetOperatorActiveStakeAmountCall),
        GetOperatorLockedAmount(GetOperatorLockedAmountCall),
        GetOperatorStakeAmount(GetOperatorStakeAmountCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetStakeAmount(GetStakeAmountCall),
        GetStakeTokenList(GetStakeTokenListCall),
        GetStakeTokenWeights(GetStakeTokenWeightsCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Initialize(InitializeCall),
        IsSupportedStakeToken(IsSupportedStakeTokenCall),
        LockInfo(LockInfoCall),
        LockStake(LockStakeCall),
        OnJobCompletion(OnJobCompletionCall),
        OperatorLockedAmounts(OperatorLockedAmountsCall),
        OperatorstakeAmounts(OperatorstakeAmountsCall),
        ProxiableUUID(ProxiableUUIDCall),
        RemoveStakeToken(RemoveStakeTokenCall),
        RenounceRole(RenounceRoleCall),
        RequestStakeWithdrawal(RequestStakeWithdrawalCall),
        RevokeRole(RevokeRoleCall),
        RewardDistributor(RewardDistributorCall),
        SetAmountToLock(SetAmountToLockCall),
        SetFeeRewardToken(SetFeeRewardTokenCall),
        SetStakeTokenSelectionWeight(SetStakeTokenSelectionWeightCall),
        SetStakingManager(SetStakingManagerCall),
        SetWithdrawalDuration(SetWithdrawalDurationCall),
        Slash(SlashCall),
        Stake(StakeCall),
        StakeAmounts(StakeAmountsCall),
        StakeTokenSelectionWeight(StakeTokenSelectionWeightCall),
        StakeTokenSelectionWeightSum(StakeTokenSelectionWeightSumCall),
        StakingManager(StakingManagerCall),
        SupportsInterface(SupportsInterfaceCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        WithdrawStake(WithdrawStakeCall),
        WithdrawalDuration(WithdrawalDurationCall),
        WithdrawalRequests(WithdrawalRequestsCall),
    }
    impl ::ethers::core::abi::AbiDecode for NativeStakingCalls {
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
                <IGeneratorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IGeneratorCallback(decoded));
            }
            if let Ok(decoded) =
                <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
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
                <EmergencyWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmergencyWithdraw(decoded));
            }
            if let Ok(decoded) =
                <FeeRewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FeeRewardToken(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorActiveStakeAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorActiveStakeAmount(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorLockedAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorLockedAmount(decoded));
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
                <OperatorstakeAmountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorstakeAmounts(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProxiableUUID(decoded));
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
            if let Ok(decoded) =
                <RequestStakeWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestStakeWithdrawal(decoded));
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
                <SetFeeRewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFeeRewardToken(decoded));
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
                <SetWithdrawalDurationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetWithdrawalDuration(decoded));
            }
            if let Ok(decoded) = <SlashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Slash(decoded));
            }
            if let Ok(decoded) = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded) = <StakeAmountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeAmounts(decoded));
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
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <WithdrawStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawStake(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalDurationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawalDuration(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalRequestsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawalRequests(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NativeStakingCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IGeneratorCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddStakeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AmountToLock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmergencyWithdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeRewardToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorActiveStakeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorLockedAmount(element) => {
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
                Self::GrantRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsSupportedStakeToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LockInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnJobCompletion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorLockedAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorstakeAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxiableUUID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveStakeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestStakeWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardDistributor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAmountToLock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeRewardToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetStakeTokenSelectionWeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStakingManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetWithdrawalDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeAmounts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeTokenSelectionWeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTokenSelectionWeightSum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakingManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeToAndCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawalDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawalRequests(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for NativeStakingCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IGeneratorCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeInterfaceVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStakeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountToLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmergencyWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeRewardToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorActiveStakeAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorLockedAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorStakeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeTokenList(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeTokenWeights(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSupportedStakeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnJobCompletion(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorLockedAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorstakeAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveStakeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestStakeWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardDistributor(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAmountToLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeRewardToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStakeTokenSelectionWeight(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetStakingManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetWithdrawalDuration(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenSelectionWeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTokenSelectionWeightSum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakingManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalDuration(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalRequests(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for NativeStakingCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<IGeneratorCallbackCall> for NativeStakingCalls {
        fn from(value: IGeneratorCallbackCall) -> Self {
            Self::IGeneratorCallback(value)
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for NativeStakingCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<AddStakeTokenCall> for NativeStakingCalls {
        fn from(value: AddStakeTokenCall) -> Self {
            Self::AddStakeToken(value)
        }
    }
    impl ::core::convert::From<AmountToLockCall> for NativeStakingCalls {
        fn from(value: AmountToLockCall) -> Self {
            Self::AmountToLock(value)
        }
    }
    impl ::core::convert::From<EmergencyWithdrawCall> for NativeStakingCalls {
        fn from(value: EmergencyWithdrawCall) -> Self {
            Self::EmergencyWithdraw(value)
        }
    }
    impl ::core::convert::From<FeeRewardTokenCall> for NativeStakingCalls {
        fn from(value: FeeRewardTokenCall) -> Self {
            Self::FeeRewardToken(value)
        }
    }
    impl ::core::convert::From<GetOperatorActiveStakeAmountCall> for NativeStakingCalls {
        fn from(value: GetOperatorActiveStakeAmountCall) -> Self {
            Self::GetOperatorActiveStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetOperatorLockedAmountCall> for NativeStakingCalls {
        fn from(value: GetOperatorLockedAmountCall) -> Self {
            Self::GetOperatorLockedAmount(value)
        }
    }
    impl ::core::convert::From<GetOperatorStakeAmountCall> for NativeStakingCalls {
        fn from(value: GetOperatorStakeAmountCall) -> Self {
            Self::GetOperatorStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for NativeStakingCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetStakeAmountCall> for NativeStakingCalls {
        fn from(value: GetStakeAmountCall) -> Self {
            Self::GetStakeAmount(value)
        }
    }
    impl ::core::convert::From<GetStakeTokenListCall> for NativeStakingCalls {
        fn from(value: GetStakeTokenListCall) -> Self {
            Self::GetStakeTokenList(value)
        }
    }
    impl ::core::convert::From<GetStakeTokenWeightsCall> for NativeStakingCalls {
        fn from(value: GetStakeTokenWeightsCall) -> Self {
            Self::GetStakeTokenWeights(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for NativeStakingCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for NativeStakingCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for NativeStakingCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsSupportedStakeTokenCall> for NativeStakingCalls {
        fn from(value: IsSupportedStakeTokenCall) -> Self {
            Self::IsSupportedStakeToken(value)
        }
    }
    impl ::core::convert::From<LockInfoCall> for NativeStakingCalls {
        fn from(value: LockInfoCall) -> Self {
            Self::LockInfo(value)
        }
    }
    impl ::core::convert::From<LockStakeCall> for NativeStakingCalls {
        fn from(value: LockStakeCall) -> Self {
            Self::LockStake(value)
        }
    }
    impl ::core::convert::From<OnJobCompletionCall> for NativeStakingCalls {
        fn from(value: OnJobCompletionCall) -> Self {
            Self::OnJobCompletion(value)
        }
    }
    impl ::core::convert::From<OperatorLockedAmountsCall> for NativeStakingCalls {
        fn from(value: OperatorLockedAmountsCall) -> Self {
            Self::OperatorLockedAmounts(value)
        }
    }
    impl ::core::convert::From<OperatorstakeAmountsCall> for NativeStakingCalls {
        fn from(value: OperatorstakeAmountsCall) -> Self {
            Self::OperatorstakeAmounts(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for NativeStakingCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RemoveStakeTokenCall> for NativeStakingCalls {
        fn from(value: RemoveStakeTokenCall) -> Self {
            Self::RemoveStakeToken(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for NativeStakingCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RequestStakeWithdrawalCall> for NativeStakingCalls {
        fn from(value: RequestStakeWithdrawalCall) -> Self {
            Self::RequestStakeWithdrawal(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for NativeStakingCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RewardDistributorCall> for NativeStakingCalls {
        fn from(value: RewardDistributorCall) -> Self {
            Self::RewardDistributor(value)
        }
    }
    impl ::core::convert::From<SetAmountToLockCall> for NativeStakingCalls {
        fn from(value: SetAmountToLockCall) -> Self {
            Self::SetAmountToLock(value)
        }
    }
    impl ::core::convert::From<SetFeeRewardTokenCall> for NativeStakingCalls {
        fn from(value: SetFeeRewardTokenCall) -> Self {
            Self::SetFeeRewardToken(value)
        }
    }
    impl ::core::convert::From<SetStakeTokenSelectionWeightCall> for NativeStakingCalls {
        fn from(value: SetStakeTokenSelectionWeightCall) -> Self {
            Self::SetStakeTokenSelectionWeight(value)
        }
    }
    impl ::core::convert::From<SetStakingManagerCall> for NativeStakingCalls {
        fn from(value: SetStakingManagerCall) -> Self {
            Self::SetStakingManager(value)
        }
    }
    impl ::core::convert::From<SetWithdrawalDurationCall> for NativeStakingCalls {
        fn from(value: SetWithdrawalDurationCall) -> Self {
            Self::SetWithdrawalDuration(value)
        }
    }
    impl ::core::convert::From<SlashCall> for NativeStakingCalls {
        fn from(value: SlashCall) -> Self {
            Self::Slash(value)
        }
    }
    impl ::core::convert::From<StakeCall> for NativeStakingCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<StakeAmountsCall> for NativeStakingCalls {
        fn from(value: StakeAmountsCall) -> Self {
            Self::StakeAmounts(value)
        }
    }
    impl ::core::convert::From<StakeTokenSelectionWeightCall> for NativeStakingCalls {
        fn from(value: StakeTokenSelectionWeightCall) -> Self {
            Self::StakeTokenSelectionWeight(value)
        }
    }
    impl ::core::convert::From<StakeTokenSelectionWeightSumCall> for NativeStakingCalls {
        fn from(value: StakeTokenSelectionWeightSumCall) -> Self {
            Self::StakeTokenSelectionWeightSum(value)
        }
    }
    impl ::core::convert::From<StakingManagerCall> for NativeStakingCalls {
        fn from(value: StakingManagerCall) -> Self {
            Self::StakingManager(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for NativeStakingCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for NativeStakingCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<WithdrawStakeCall> for NativeStakingCalls {
        fn from(value: WithdrawStakeCall) -> Self {
            Self::WithdrawStake(value)
        }
    }
    impl ::core::convert::From<WithdrawalDurationCall> for NativeStakingCalls {
        fn from(value: WithdrawalDurationCall) -> Self {
            Self::WithdrawalDuration(value)
        }
    }
    impl ::core::convert::From<WithdrawalRequestsCall> for NativeStakingCalls {
        fn from(value: WithdrawalRequestsCall) -> Self {
            Self::WithdrawalRequests(value)
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
        pub lock_amount: ::ethers::core::types::U256,
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
    ///Container type for all return fields from the `getOperatorLockedAmount` function with signature `getOperatorLockedAmount(address,address)` and selector `0x31a38076`
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
    pub struct GetOperatorLockedAmountReturn(pub ::ethers::core::types::U256);
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
        pub token: ::ethers::core::types::Address,
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
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `operatorstakeAmounts` function with signature `operatorstakeAmounts(address,address)` and selector `0x7a8ca0bb`
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
    pub struct OperatorstakeAmountsReturn {
        pub amount: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `stakeAmounts` function with signature `stakeAmounts(address,address,address)` and selector `0x6bc04b29`
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
    pub struct StakeAmountsReturn {
        pub amount: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `withdrawalDuration` function with signature `withdrawalDuration()` and selector `0x75e3b470`
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
    pub struct WithdrawalDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `withdrawalRequests` function with signature `withdrawalRequests(address,address,uint256)` and selector `0xae6b1983`
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
    pub struct WithdrawalRequestsReturn {
        pub stake_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub withdrawal_time: ::ethers::core::types::U256,
    }
}
