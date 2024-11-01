pub use generator_registry::*;
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
pub mod generator_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_entityRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract EntityKeyRegistry",),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stakingManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract StakingManager"),
                        ),
                    },
                ],
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
                    ::std::borrow::ToOwned::to_owned("ENTITY_KEY_REGISTRY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ENTITY_KEY_REGISTRY",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract EntityKeyRegistry",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PARALLEL_REQUESTS_UPPER_LIMIT"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("PARALLEL_REQUESTS_UPPER_LIMIT",),
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
                    ::std::borrow::ToOwned::to_owned("PROOF_MARKET_PLACE_ROLE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("PROOF_MARKET_PLACE_ROLE",),
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
                    ::std::borrow::ToOwned::to_owned("STAKING_MANAGER"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("STAKING_MANAGER"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract StakingManager"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UNLOCK_WAIT_BLOCKS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("UNLOCK_WAIT_BLOCKS"),
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
                    ::std::borrow::ToOwned::to_owned("addIvsKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addIvsKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("attestationData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("enclaveSignature"),
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
                    ::std::borrow::ToOwned::to_owned("assignGeneratorTask"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assignGeneratorTask",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("askId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("changeRewardAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("changeRewardAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newRewardAddress"),
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
                    ::std::borrow::ToOwned::to_owned("completeGeneratorTask"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("completeGeneratorTask",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("askId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeToRelease"),
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
                    ::std::borrow::ToOwned::to_owned("decreaseDeclaredCompute"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("decreaseDeclaredCompute",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregister"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deregister"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("generatorInfoPerMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("generatorInfoPerMarket",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
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
                                name: ::std::borrow::ToOwned::to_owned("state"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum GeneratorRegistry.GeneratorState",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("computePerRequestRequired",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proofGenerationCost",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proposedTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("activeRequests"),
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
                    ::std::borrow::ToOwned::to_owned("generatorRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("generatorRegistry"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rewardAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sumOfComputeAllocations",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("computeConsumed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("activeMarketplaces",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("declaredCompute"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "intendedComputeUtilization",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorData"),
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
                    ::std::borrow::ToOwned::to_owned("getGeneratorAssignmentDetails"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getGeneratorAssignmentDetails",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getGeneratorRewardDetails"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getGeneratorRewardDetails",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
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
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getGeneratorState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getGeneratorState"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum GeneratorRegistry.GeneratorState",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("increaseDeclaredCompute"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("increaseDeclaredCompute",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("computeToIncrease"),
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
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intendToReduceCompute"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("intendToReduceCompute",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("computeToReduce"),
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
                    ::std::borrow::ToOwned::to_owned("joinMarketplace"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("joinMarketplace"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("computePerRequestRequired",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proofGenerationCost",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proposedTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("updateMarketDedicatedKey",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("attestationData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("enclaveSignature"),
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
                    ::std::borrow::ToOwned::to_owned("leaveMarketplace"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("leaveMarketplace"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("leaveMarketplaces"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("leaveMarketplaces"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("marketIds"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                                ::std::borrow::ToOwned::to_owned("contract ProofMarketplace",),
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
                    ::std::borrow::ToOwned::to_owned("register"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("register"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rewardAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("declaredCompute"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorData"),
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
                    ::std::borrow::ToOwned::to_owned("releaseGeneratorResources"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("releaseGeneratorResources",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("removeEncryptionKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeEncryptionKey",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("requestForExitMarketplace"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestForExitMarketplace",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("requestForExitMarketplaces"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestForExitMarketplaces",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("marketIds"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
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
                (
                    ::std::borrow::ToOwned::to_owned("updateEncryptionKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateEncryptionKey",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("attestationData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("enclaveSignature"),
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
                    ::std::borrow::ToOwned::to_owned("AddIvsKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddIvsKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("signer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
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
                    ::std::borrow::ToOwned::to_owned("ChangedGeneratorRewardAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ChangedGeneratorRewardAddress",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newRewardAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ComputeLockImposed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ComputeLockImposed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("compute"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ComputeLockReleased"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ComputeLockReleased",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("compute"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DecreaseCompute"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DecreaseCompute"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("compute"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DeregisteredGenerator"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DeregisteredGenerator",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("generator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncreasedCompute"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("IncreasedCompute"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("compute"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("JoinedMarketplace"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("JoinedMarketplace"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("computeAllocation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LeftMarketplace"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LeftMarketplace"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RegisteredGenerator"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RegisteredGenerator",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("initialCompute"),
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
                    ::std::borrow::ToOwned::to_owned("RequestComputeDecrease"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RequestComputeDecrease",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("intendedUtilization",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RequestExitMarketplace"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RequestExitMarketplace",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("AlreadyJoinedMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AlreadyJoinedMarket",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssignOnlyToIdleGenerators"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AssignOnlyToIdleGenerators",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotBeMoreThanDeclaredCompute"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CannotBeMoreThanDeclaredCompute",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotBeSlashed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CannotBeSlashed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotBeZero"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CannotBeZero"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotLeaveMarketWithActiveRequest"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "CannotLeaveMarketWithActiveRequest",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotLeaveWithActiveMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CannotLeaveWithActiveMarket",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("ExceedsAcceptableRange"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ExceedsAcceptableRange",),
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
                    ::std::borrow::ToOwned::to_owned("GeneratorAlreadyExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("GeneratorAlreadyExists",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectImageId"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("IncorrectImageId"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientGeneratorComputeAvailable"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "InsufficientGeneratorComputeAvailable",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidContractAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidContractAddress",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidEnclaveKey"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidEnclaveKey"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidEnclaveSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidEnclaveSignature",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("invalidSignerAddress",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidGenerator"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidGenerator"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidGeneratorStatePerMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidGeneratorStatePerMarket",),
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
                    ::std::borrow::ToOwned::to_owned("InvalidMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidMarket"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MaxParallelRequestsPerMarketExceeded"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "MaxParallelRequestsPerMarketExceeded",
                        ),
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
                    ::std::borrow::ToOwned::to_owned("OnlyValidGeneratorsCanRequestExit"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyValidGeneratorsCanRequestExit",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyWorkingGenerators"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyWorkingGenerators",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PublicMarketsDontNeedKey"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PublicMarketsDontNeedKey",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReduceComputeRequestNotInPlace"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ReduceComputeRequestNotInPlace",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReductionRequestNotValid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ReductionRequestNotValid",),
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
                    ::std::borrow::ToOwned::to_owned("RequestAlreadyInPlace"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RequestAlreadyInPlace",),
                        inputs: ::std::vec![],
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
    pub static GENERATORREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R0`\x80R4\x80\x15a\0\x14W`\0\x80\xFD[P`@QaKU8\x03\x80aKU\x839\x81\x01`@\x81\x90Ra\x003\x91a\x01pV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\0}WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\0\x99WP0;\x15[\x90P\x81\x15\x80\x15a\0\xA7WP\x80\x15[\x15a\0\xC5W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T`\x01`\x01`@\x1B\x03\x19\x16`\x01\x17\x85U\x83\x15a\0\xF3W\x84T`\xFF`@\x1B\x19\x16h\x01\0\0\0\0\0\0\0\0\x17\x85U[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\xA0R\x86\x16`\xC0R\x83\x15a\x01LW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPa\x01\xAAV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01mW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\x83W`\0\x80\xFD[\x82Qa\x01\x8E\x81a\x01XV[` \x84\x01Q\x90\x92Pa\x01\x9F\x81a\x01XV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0QaI\x1Aa\x02;`\09`\0\x81\x81a\x06\x88\x01R\x81\x81a\tV\x01R\x81\x81a\n)\x01R\x81\x81a\r\x1E\x01R\x81\x81a\x17\r\x01R\x81\x81a\x19\x9D\x01R\x81\x81a\"\xDC\x01Ra'\x11\x01R`\0\x81\x81a\x06\x13\x01R\x81\x81a\r\xF5\x01R\x81\x81a\x0E\xD9\x01R\x81\x81a\x18\xFF\x01R\x81\x81a4\xBF\x01Ra5\xB0\x01R`\0\x81\x81a0c\x01R\x81\x81a0\x8C\x01Ra1\xD0\x01RaI\x1A`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x88W`\x005`\xE0\x1C\x80caA?\x0F\x11a\x01ZW\x80c\x9A\x7F\xCA\x8E\x11a\0\xC1W\x80c\xC0\xC5;\x8B\x11a\0zW\x80c\xC0\xC5;\x8B\x14a\x08pW\x80c\xCB\x80\xD4\x18\x14a\x08\x90W\x80c\xD0n\x1F{\x14a\x08\xB0W\x80c\xD5Gt\x1F\x14a\x08\xD0W\x80c\xE2\xFA3\xCE\x14a\x08\xF0W\x80c\xE7\xBC\x96\0\x14a\t\x10W`\0\x80\xFD[\x80c\x9A\x7F\xCA\x8E\x14a\x07]W\x80c\x9F]\xB9\x86\x14a\x07\xC8W\x80c\xA2\x17\xFD\xDF\x14a\x07\xE8W\x80c\xA66\x16\xE6\x14a\x07\xFDW\x80c\xAD<\xB1\xCC\x14a\x08\x1DW\x80c\xAF\xF5\xED\xB1\x14a\x08[W`\0\x80\xFD[\x80c\x86J\x8E\xFA\x11a\x01\x13W\x80c\x86J\x8E\xFA\x14a\x06vW\x80c\x8C\xFCV\xD8\x14a\x06\xAAW\x80c\x91\xD1HT\x14a\x06\xDDW\x80c\x92\xEB\x91\xE2\x14a\x06\xFDW\x80c\x95\xA7\x8DI\x14a\x07\x1DW\x80c\x96\xDE\x0E\xEF\x14a\x07=W`\0\x80\xFD[\x80caA?\x0F\x14a\x05\xB3W\x80cdmQ\xB4\x14a\x05\xD3W\x80cf\x1D\xE5\xAC\x14a\x06\x01W\x80cm@Xw\x14a\x065W\x80cz\x14\xC4c\x14a\x056W\x80c\x81\xC4\\p\x14a\x06UW`\0\x80\xFD[\x80c+a\x0C-\x11a\x01\xFEW\x80c6V\x8A\xBE\x11a\x01\xB7W\x80c6V\x8A\xBE\x14a\x05\x16W\x80c<^\xB5|\x14a\x056W\x80cM*\xAB\x9A\x14a\x05KW\x80cO\x1E\xF2\x86\x14a\x05kW\x80cR\xD1\x90-\x14a\x05~W\x80cT\x1A\x8C\x18\x14a\x05\x93W`\0\x80\xFD[\x80c+a\x0C-\x14a\x04@W\x80c,\x1F\xBD\x03\x14a\x04\x7FW\x80c,4l\x0C\x14a\x04\xA1W\x80c//\xF1]\x14a\x04\xC1W\x80c/\x8FJ;\x14a\x04\xE1W\x80c1\xB5&\xBA\x14a\x04\xF6W`\0\x80\xFD[\x80c\x1C~\xAEe\x11a\x02PW\x80c\x1C~\xAEe\x14a\x03DW\x80c \x1A\x11\xB3\x14a\x03yW\x80c!\x80\xDE]\x14a\x03\x99W\x80c\"\x82\x8C\xC2\x14a\x03\xB9W\x80c$\x8A\x9C\xA3\x14a\x03\xF2W\x80c)S\xE7\xB3\x14a\x04 W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02\x8DW\x80c\x05jvh\x14a\x02\xC2W\x80c\x07\x81Y\xCD\x14a\x02\xE4W\x80c\x08\xBEk\xAD\x14a\x03\x04W\x80c\x1A\x1E\x8C\xF8\x14a\x03$W[`\0\x80\xFD[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\xADa\x02\xA86`\x04a=XV[a\t0V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xCEW`\0\x80\xFD[Pa\x02\xE2a\x02\xDD6`\x04a=\x82V[a\tAV[\0[4\x80\x15a\x02\xF0W`\0\x80\xFD[Pa\x02\xE2a\x02\xFF6`\x04a=\xB0V[a\n\x14V[4\x80\x15a\x03\x10W`\0\x80\xFD[Pa\x02\xE2a\x03\x1F6`\x04a=\xF1V[a\x0B\x0BV[4\x80\x15a\x030W`\0\x80\xFD[Pa\x02\xE2a\x03?6`\x04a>fV[a\x0BEV[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x03da\x03_6`\x04a>fV[a\x0CgV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xB9V[4\x80\x15a\x03\x85W`\0\x80\xFD[Pa\x02\xE2a\x03\x946`\x04a=\xB0V[a\r\tV[4\x80\x15a\x03\xA5W`\0\x80\xFD[Pa\x02\xE2a\x03\xB46`\x04a?\x96V[a\r\xF3V[4\x80\x15a\x03\xC5W`\0\x80\xFD[Pa\x01\xF8Ta\x03\xDA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xB9V[4\x80\x15a\x03\xFEW`\0\x80\xFD[Pa\x04\x12a\x04\r6`\x04a=\x82V[a\x0F\x85V[`@Q\x90\x81R` \x01a\x02\xB9V[4\x80\x15a\x04,W`\0\x80\xFD[Pa\x02\xE2a\x04;6`\x04a@\tV[a\x0F\xA7V[4\x80\x15a\x04LW`\0\x80\xFD[Pa\x04`a\x04[6`\x04a>fV[a\x11\x9FV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xB9V[4\x80\x15a\x04\x8BW`\0\x80\xFD[Pa\x04\x12`\0\x80Q` aHe\x839\x81Q\x91R\x81V[4\x80\x15a\x04\xADW`\0\x80\xFD[Pa\x02\xE2a\x04\xBC6`\x04a@0V[a\x13gV[4\x80\x15a\x04\xCDW`\0\x80\xFD[Pa\x02\xE2a\x04\xDC6`\x04a@mV[a\x14\xFFV[4\x80\x15a\x04\xEDW`\0\x80\xFD[Pa\x02\xE2a\x15!V[4\x80\x15a\x05\x02W`\0\x80\xFD[Pa\x02\xE2a\x05\x116`\x04a=\xB0V[a\x16\xF8V[4\x80\x15a\x05\"W`\0\x80\xFD[Pa\x02\xE2a\x0516`\x04a@mV[a\x17\xE2V[4\x80\x15a\x05BW`\0\x80\xFD[Pa\x04\x12`d\x81V[4\x80\x15a\x05WW`\0\x80\xFD[Pa\x02\xE2a\x05f6`\x04a@\x9DV[a\x18\x15V[a\x02\xE2a\x05y6`\x04a@\xBAV[a\x18\xB9V[4\x80\x15a\x05\x8AW`\0\x80\xFD[Pa\x04\x12a\x18\xD8V[4\x80\x15a\x05\x9FW`\0\x80\xFD[Pa\x02\xE2a\x05\xAE6`\x04a=\x82V[a\x18\xF5V[4\x80\x15a\x05\xBFW`\0\x80\xFD[Pa\x02\xE2a\x05\xCE6`\x04a=\xB0V[a\x19\x88V[4\x80\x15a\x05\xDFW`\0\x80\xFD[Pa\x05\xF3a\x05\xEE6`\x04a>fV[a\x1ArV[`@Qa\x02\xB9\x92\x91\x90aAAV[4\x80\x15a\x06\rW`\0\x80\xFD[Pa\x03\xDA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06AW`\0\x80\xFD[Pa\x02\xE2a\x06P6`\x04a=\x82V[a\x1D\x17V[4\x80\x15a\x06aW`\0\x80\xFD[Pa\x01\xF7Ta\x03\xDA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\x82W`\0\x80\xFD[Pa\x03\xDA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06\xB6W`\0\x80\xFD[Pa\x06\xCAa\x06\xC56`\x04a@\x9DV[a\x1D\xBEV[`@Qa\x02\xB9\x97\x96\x95\x94\x93\x92\x91\x90aA\xACV[4\x80\x15a\x06\xE9W`\0\x80\xFD[Pa\x02\xADa\x06\xF86`\x04a@mV[a\x1E\x90V[4\x80\x15a\x07\tW`\0\x80\xFD[Pa\x02\xE2a\x07\x186`\x04a?\x96V[a\x1E\xC8V[4\x80\x15a\x07)W`\0\x80\xFD[Pa\x02\xE2a\x0786`\x04aA\xF8V[a\x1E\xD5V[4\x80\x15a\x07IW`\0\x80\xFD[Pa\x02\xE2a\x07X6`\x04a=\x82V[a!;V[4\x80\x15a\x07iW`\0\x80\xFD[Pa\x07\xB7a\x07x6`\x04a>fV[a\x01\xF5` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T`\xFF\x90\x93\x16\x93\x91\x92\x90\x91\x90\x85V[`@Qa\x02\xB9\x95\x94\x93\x92\x91\x90aBPV[4\x80\x15a\x07\xD4W`\0\x80\xFD[Pa\x02\xE2a\x07\xE36`\x04a=\x82V[a\"\xBAV[4\x80\x15a\x07\xF4W`\0\x80\xFD[Pa\x04\x12`\0\x81V[4\x80\x15a\x08\tW`\0\x80\xFD[Pa\x02\xE2a\x08\x186`\x04a=\xB0V[a\"\xC7V[4\x80\x15a\x08)W`\0\x80\xFD[Pa\x08N`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02\xB9\x91\x90aB\x80V[4\x80\x15a\x08gW`\0\x80\xFD[Pa\x02\xE2a#\xB1V[4\x80\x15a\x08|W`\0\x80\xFD[Pa\x02\xE2a\x08\x8B6`\x04aB\x93V[a%~V[4\x80\x15a\x08\x9CW`\0\x80\xFD[Pa\x02\xE2a\x08\xAB6`\x04a=\xB0V[a&\xFCV[4\x80\x15a\x08\xBCW`\0\x80\xFD[Pa\x02\xE2a\x08\xCB6`\x04a=\xF1V[a'\xE6V[4\x80\x15a\x08\xDCW`\0\x80\xFD[Pa\x02\xE2a\x08\xEB6`\x04a@mV[a(\x1BV[4\x80\x15a\x08\xFCW`\0\x80\xFD[Pa\x02\xE2a\t\x0B6`\x04aB\xECV[a(7V[4\x80\x15a\t\x1CW`\0\x80\xFD[Pa\x02\xE2a\t+6`\x04a=\x82V[a*\xDBV[`\0a\t;\x82a*\xE5V[\x92\x91PPV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xC9\x91\x90aC\x92V[a\t\xE6W`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x81\x90\x7F\rAv7\xDA<\xA4\xE3\xCEN\xD4@?FLzU\xD3\xF0\xA9\x1F\"\xDDJo\xF1\x1D4V\xB1z\xDF\x90`\0\x90\xA2PV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nxW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9C\x91\x90aC\x92V[a\n\xB9W`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDB\xC0\x91\xCCw\x93\xE6t\x90\x95\xEA\x02\xFD\xC5\xB7Uy\x11u\xE4\x0E\xD8\0\x9D\xC8\x18 pyp\xB3\x99\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\0[\x81\x81\x10\x15a\x0B@Wa\x0B83\x84\x84\x84\x81\x81\x10a\x0B,Wa\x0B,aC\xAFV[\x90P` \x02\x015a+\x1AV[`\x01\x01a\x0B\x0EV[PPPV[`\0\x80Q` aHe\x839\x81Q\x91Ra\x0B]\x81a-sV[`\0a\x0Bi\x84\x84a\x1ArV[P\x90P`\0\x81`\x04\x81\x11\x15a\x0B\x80Wa\x0B\x80aA\tV[\x14\x80a\x0B\x9DWP`\x01\x81`\x04\x81\x11\x15a\x0B\x9BWa\x0B\x9BaA\tV[\x14[\x15a\x0B\xBAW`@Qb\xED=\xF3`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 a\x01\xF5\x83R\x81\x84 \x87\x85R\x90\x92R\x82 `\x04\x81\x01\x80T\x92\x93\x91\x92\x91a\x0B\xFA\x83aC\xDBV[\x91\x90PUP\x80`\x01\x01T\x82`\x02\x01`\0\x82\x82Ta\x0C\x17\x91\x90aC\xF2V[\x90\x91UPP`\x01\x81\x01T`@Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90\x7F\x0Bi \xA1i\xFB\x88\x9C\x15T\x91\x06\xAE\xE0p\xDA\x98\x18kz\xB4\x06\xDD\x93\x84\xA0\xD3\x18\x99\xFD\xF0\x8A\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x0C\xB3Wa\x0C\xB3aA\tV[`\x04\x81\x11\x15a\x0C\xC4Wa\x0C\xC4aA\tV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P\x80`@\x01Q\x81``\x01Q\x92P\x92PP[\x92P\x92\x90PV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x91\x91\x90aC\x92V[a\r\xAEW`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F5\x97\x03\x012<\xD4\x17\xC1\x11ER\x17y\xF7\xA9Q\x98\x84`\x82\xAC\x86[\x05\xD1\xC7\x96\xA3\\!\x13\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ck[!\xA6a\x0E+\x85a-}V[a\x0E4\x87a-\xB6V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EuW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x99\x91\x90aC\x92V[a\x0E\xB6W`@Qc\xC4e\xE6\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xC2\x83\x83\x833a-\xFEV[`@Qc\x07\x07Y\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x07\x07Y\x1F\x90a\x0F\x0E\x90\x86\x90`\x04\x01aB\x80V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F(W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F<W=`\0\x80>=`\0\xFD[PPPPa\x0FI\x83a.\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F\xC3\xFB\xBD\xB6\xAA\x8D\x99\xF6\xEF\xE2J:\"\xE9\xA9\x9F\xFE\xF2J/9\x9C\x0B\x1ET\x99F\xF9\x1B\xF06\xE4`@Q`@Q\x80\x91\x03\x90\xA3PPPPV[`\0\x90\x81R`\0\x80Q` aH\xA5\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[a\x0F\xAFa.\xFFV[`\0\x80Q` aHe\x839\x81Q\x91Ra\x0F\xC7\x81a-sV[`\0\x80a\x0F\xD4\x85\x85a\x1ArV[\x90\x92P\x90P`\x01\x82`\x04\x81\x11\x15a\x0F\xEDWa\x0F\xEDaA\tV[\x14\x80a\x10\nWP`\x03\x82`\x04\x81\x11\x15a\x10\x08Wa\x10\x08aA\tV[\x14[a\x10'W`@QcC\x9FL\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 a\x01\xF5\x83R\x81\x84 \x88\x85R\x90\x92R\x90\x91 `\x01\x81\x01T\x83\x10\x15a\x10yW`@Qc\x01\x19\"\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`d\x81`\x04\x01T\x11\x15a\x10\x9FW`@Qc\xCA\xBDP\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`\x01\x01T\x90P\x80\x83`\x02\x01`\0\x82\x82Ta\x10\xBC\x91\x90aD\x05V[\x90\x91UPPa\x01\xF8T`@Qc{:R\xF7`\xE1\x1B\x81R`\x04\x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`$\x83\x01R\x90\x91\x16\x90c\xF6t\xA5\xEE\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\x10W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11$W=`\0\x80>=`\0\xFD[PPPP\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD2\x9B\x9A\xDE\xE3+\xAF\x11\xBE\x04\0\x0Bsc\x99c\xD2\xC7n\x89\x9Fh0\xD3\xB5\xC1q]\xF1}\x82\xBD\x82`@Qa\x11c\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x04\x82\x01\x80T\x90`\0a\x11}\x83aD\x18V[\x91\x90PUPPPPPPPa\x0B@`\x01`\0\x80Q` aH\xC5\x839\x81Q\x91RUV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x11\xEBWa\x11\xEBaA\tV[`\x04\x81\x11\x15a\x11\xFCWa\x11\xFCaA\tV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x01\xF4`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`\xE0\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01\x80Ta\x12\xCE\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xFA\x90aD1V[\x80\x15a\x13GW\x80`\x1F\x10a\x13\x1CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13GV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13*W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x90Q`@\x90\x93\x01Q\x92\x94P\x91\x92PPP\x92P\x92\x90PV[`\0\x80Q` aHe\x839\x81Q\x91Ra\x13\x7F\x81a-sV[`\0a\x13\x8B\x85\x85a\x1ArV[P\x90P`\0\x81`\x04\x81\x11\x15a\x13\xA2Wa\x13\xA2aA\tV[\x14\x80a\x13\xBFWP`\x01\x81`\x04\x81\x11\x15a\x13\xBDWa\x13\xBDaA\tV[\x14[\x15a\x13\xDDW`@Qc\x1C\x0E\xC29`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 a\x01\xF5\x83R\x81\x84 \x88\x85R\x90\x92R\x82 `\x01\x81\x01T`\x02\x83\x01\x80T\x93\x94\x92\x93\x91\x92\x83\x92a\x14'\x90\x84\x90aC\xF2V[\x90\x91UPPa\x01\xF8T`@Qc\xC0\xFF\x97\xEF`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`$\x83\x01R`D\x82\x01\x89\x90R\x90\x91\x16\x90c\xC0\xFF\x97\xEF\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x96W=`\0\x80>=`\0\xFD[PPPP\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\x0Bi \xA1i\xFB\x88\x9C\x15T\x91\x06\xAE\xE0p\xDA\x98\x18kz\xB4\x06\xDD\x93\x84\xA0\xD3\x18\x99\xFD\xF0\x8A\x82`@Qa\x14\xD5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x04\x82\x01\x80T\x90`\0a\x14\xEF\x83aC\xDBV[\x91\x90PUPPPPPPPPPPV[a\x15\x08\x82a\x0F\x85V[a\x15\x11\x81a-sV[a\x15\x1B\x83\x83a/7V[PPPPV[3`\0\x81\x81Ra\x01\xF4` R`@\x90 `\x06\x81\x01\x80Ta\x15@\x90aD1V[\x15\x90P\x80a\x15VWP\x80T`\x01`\x01`\xA0\x1B\x03\x16\x15[\x15a\x15tW`@QcdF\xF9\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81`\x05\x01T\x03a\x15\xA0W`@Qc\x89\x83`\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x04\x01T\x83`\x05\x01Ta\x15\xBF\x91\x90aDkV[a\x15\xC9\x91\x90aD\x82V[\x90P`\0\x81\x83`\x04\x01Ta\x15\xDD\x91\x90aC\xF2V[\x90P\x82`\x02\x01T\x82\x10\x15a\x16\x04W`@Qc\x01\x19\"\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01\x01T\x82\x10\x15a\x16)W`@Qc\x01\x19\"\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x83\x01\x82\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x05\x84\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01\xF6` R`@\x90 TC\x10\x80\x15\x90a\x16\x7FWP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01\xF6` R`@\x90 T\x15\x15[a\x16\x9CW`@Qc8l\xF4\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x01\xF6` R`@\x80\x82 \x91\x90\x91UQ\x7F\x11\xDE\xAE(\x9Epx\xFC\xE2\x88\xF4\xE9sN\x9C=\xF6{U\xEA\xC4\xFFA\xB8\x90\xB6\xD68\x1A\xCE\xE7b\x90a\x16\xEA\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x80\x91\x90aC\x92V[a\x17\x9DW`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE7\xA4\x8C\x90\x98\xE0\xE2\xAEV\xF8\xF1g0N\xEFTw\xB7*\x9D\x9E\xE1\xBA\x99Mb\xCD\xAF\x9B1\x0B\t\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x18\x0BW`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B@\x82\x82a/\xDCV[3`\0\x81\x81Ra\x01\xF4` R`@\x90 `\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80a\x18CWP\x80T`\x01`\x01`\xA0\x1B\x03\x16\x15[\x15a\x18aW`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x83U`@Q\x91\x82R\x83\x16\x90\x7F\xB1\x95\xA9K\xEC\xD3\x88\xC9\xA0\x7F\xE8\x18qh3\xBD\xF9\x8Bu\\x\xC9\xB48\xF4\xC8\xF0o5O\xA3h\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPPV[a\x18\xC1a0XV[a\x18\xCA\x82a0\xFDV[a\x18\xD4\x82\x82a1\x08V[PPV[`\0a\x18\xE2a1\xC5V[P`\0\x80Q` aH\x85\x839\x81Q\x91R\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xED8\r\x033`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19mW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\x81W=`\0\x80>=`\0\xFD[PPPPPV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x10\x91\x90aC\x92V[a\x1A-W`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F@\x93(\x8B\xC9[\xFE\xD3Z\x9C>\xA0\xDB\x07\xD8\xC6\xCB\xD9r,\xDFv\x972\xEE]s]a\xC3\xCF+\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x1A\xBEWa\x1A\xBEaA\tV[`\x04\x81\x11\x15a\x1A\xCFWa\x1A\xCFaA\tV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x01\xF4`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`\xE0\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01\x80Ta\x1B\xA1\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B\xCD\x90aD1V[\x80\x15a\x1C\x1AW\x80`\x1F\x10a\x1B\xEFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\x1AV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B\xFDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0`\x04\x81\x11\x15a\x1C8Wa\x1C8aA\tV[\x82Q`\x04\x81\x11\x15a\x1CKWa\x1CKaA\tV[\x03a\x1C^W`\0\x80\x93P\x93PPPa\r\x02V[`\x04\x82Q`\x04\x81\x11\x15a\x1CsWa\x1CsaA\tV[\x03a\x1C\x87W`\x04`\0\x93P\x93PPPa\r\x02V[`\0a\x1C\x92\x87a2\x0EV[\x90P`\0\x83Q`\x04\x81\x11\x15a\x1C\xA9Wa\x1C\xA9aA\tV[\x14\x15\x80\x15a\x1C\xB5WP\x80\x15[\x15a\x1C\xCAW`\x02`\0\x94P\x94PPPPa\r\x02V[\x81`\x80\x01Q\x81\x03a\x1C\xE3W`\x01\x94P\x92Pa\r\x02\x91PPV[\x80\x15\x80\x15\x90a\x1C\xF5WP\x81`\x80\x01Q\x81\x10[\x15a\x1D\x08W`\x03\x94P\x92Pa\r\x02\x91PPV[P`\0\x96\x87\x96P\x94PPPPPV[3`\0\x81\x81Ra\x01\xF4` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\x1DLWP\x80`\x06\x01\x80Ta\x1DH\x90aD1V[\x15\x90P[\x15a\x1DjW`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81`\x04\x01`\0\x82\x82Ta\x1D~\x91\x90aD\x05V[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F \x98X\xCE\xC2\x8CQ\x8A\xEAD\xAA?\xA7\n\x93\"S\xF8\xBD\xF7j\x1C\x9A\x0B\x08\"j\x13\xE6\x975\xA3\x90` \x01a\x18\xACV[a\x01\xF4` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x97\x95\x96\x94\x95\x93\x94\x92\x93\x91\x92a\x1E\r\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E9\x90aD1V[\x80\x15a\x1E\x86W\x80`\x1F\x10a\x1E[Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x86V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1EiW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x87V[`\0\x91\x82R`\0\x80Q` aH\xA5\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x15\x1B3\x85\x85\x85\x85a3dV[a\x1E\xDDa.\xFFV[3`\0\x81\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01\x80T`\xC0\x84\x01\x91\x90a\x1FP\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F|\x90aD1V[\x80\x15a\x1F\xC9W\x80`\x1F\x10a\x1F\x9EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xC9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\xACW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x82Q`\0\x14\x80a\x1F\xEBWP`\x01`\x01`\xA0\x1B\x03\x85\x16\x15[\x80a\x1F\xF4WP\x83\x15[\x15a \x12W`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x15a ;W`@QcXt\xF9{`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xE0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R`\0` \x80\x84\x01\x82\x81R\x84\x86\x01\x83\x81R``\x86\x01\x84\x81R`\x80\x87\x01\x8C\x81Rg\r\xE0\xB6\xB3\xA7d\0\0`\xA0\x89\x01\x90\x81R`\xC0\x89\x01\x8D\x81R\x8C\x89\x16\x88Ra\x01\xF4\x90\x96R\x98\x90\x95 \x87Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16\x97\x16\x96\x90\x96\x17\x86U\x91Q`\x01\x86\x01UQ`\x02\x85\x01UQ`\x03\x84\x01U\x90Q`\x04\x83\x01U\x92Q`\x05\x82\x01U\x91Q\x90\x91\x90`\x06\x82\x01\x90a \xE1\x90\x82aD\xEBV[PP`@Q\x85\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x91P\x7F&>\xEC}8t\x99-\xAF.\x9B\xAB\x85(|a\x1A@\x01\x8A\xC6\xCD\x01\x08\xF1\xEE\xD1$\x91\x82\xDE\x04\x90` \x01`@Q\x80\x91\x03\x90\xA2PPa\x0B@`\x01`\0\x80Q` aH\xC5\x839\x81Q\x91RUV[3`\0\x81\x81Ra\x01\xF4` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a!pWP\x80`\x06\x01\x80Ta!l\x90aD1V[\x15\x90P[\x80a!yWP\x82\x15[\x15a!\x97W`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81`\x05\x01T\x14a!\xC3W`@Qc\x07\xECv9`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x82`\x04\x01Ta!\xD5\x91\x90aC\xF2V[\x90P\x81`\x01\x01T\x81\x11a!\xFBW`@Qc\xCA^b/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x82\x01T`\0\x90a\"\x15g\r\xE0\xB6\xB3\xA7d\0\0\x84aDkV[a\"\x1F\x91\x90aD\x82V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\"IW`@Qc\xCA^b/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x83\x01\x81\x90Ua\"[`\x01CaD\x05V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x81\x81Ra\x01\xF6` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x7Fv\x12,\xFD_<h\0\xA2#\xE6\x0C\xC6$G\xC6I6\x03}x\xD0-\x91\x92\x93\xF6U\xBA}T\xCB\x90a\"\xAB\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[a\"\xC43\x82a+\x1AV[PV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#+W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#O\x91\x90aC\x92V[a#lW`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E=\xF7GA3\x9A\xFB\xBCk\xCA\xC6\n\xEB\xCD\x0E\xA5\xA2\xBB(\x07,\xFB\x016\x13\x8F\xCC^\xD8c\xFF\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[a#\xB9a.\xFFV[3`\0\x81\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01\x80T`\xC0\x84\x01\x91\x90a$,\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$X\x90aD1V[\x80\x15a$\xA5W\x80`\x1F\x10a$zWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$\xA5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\x88W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80` \x01Q`\0\x14a$\xD5W`@Qc\xF8\xC20S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01\xF4` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U`\x03\x81\x01\x82\x90U`\x04\x81\x01\x82\x90U`\x05\x81\x01\x82\x90U\x90a%-`\x06\x83\x01\x82a=\nV[PP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\x9F70\xAD\xE9K\xE5\xCE?\xAD\x97;\x88\x8At\x86j:\x91]\0\x8E\x8C\xBD\xE2\x13\x82\xB91\xB6|c\x90`\0\x90\xA2PPa%|`\x01`\0\x80Q` aH\xC5\x839\x81Q\x91RUV[V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a%\xC3WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a%\xDFWP0;\x15[\x90P\x81\x15\x80\x15a%\xEDWP\x80\x15[\x15a&\x0BW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a&5W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a&=a6'V[a&Ea6'V[a&Ma6'V[a&Ua6'V[a&``\0\x89a/7V[Pa&y`\0\x80Q` aHe\x839\x81Q\x91R\x88a/7V[Pa\x01\xF7\x80T`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92Ua\x01\xF8\x80T\x92\x89\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x83\x15a&\xF2W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x84\x91\x90aC\x92V[a'\xA1W`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x85r\xE9\xA7M\x9B\x02psW\xC4\x81\x9AR\xCF\xF3]TZH\x7F\xA3\xC9a\xD8\xBE\x8F\x9E\xC6Xa\xB6\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[`\0[\x81\x81\x10\x15a\x0B@Wa(\x133\x84\x84\x84\x81\x81\x10a(\x07Wa(\x07aC\xAFV[\x90P` \x02\x015a6/V[`\x01\x01a'\xE9V[a($\x82a\x0F\x85V[a(-\x81a-sV[a\x15\x1B\x83\x83a/\xDCV[3`\0\x81\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 a\x01\xF5\x83R\x81\x84 \x8D\x85R\x90\x92R\x80\x83 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x92\x93\x92\x82\x90`\xFF\x16`\x04\x81\x11\x15a(\x7FWa(\x7FaA\tV[`\x04\x81\x11\x15a(\x90Wa(\x90aA\tV[\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x90\x91\x01R\x82T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a(\xD4WP\x87\x15[\x80a(\xDDWP\x89\x15[\x15a(\xFBW`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a)\x06\x8Ca7\x01V[P\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a)0W`@Qc\x9D\xB8\xD5\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82Q`\x04\x81\x11\x15a)EWa)EaA\tV[\x14a)cW`@Qc+[X\x0F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x8A\x83`\x01\x01`\0\x82\x82Ta)w\x91\x90aD\x05V[\x90\x91UPP`\x04\x83\x01T`\x01\x84\x01T\x11\x15a)\xA5W`@Qc\x07\xD9\x93\x89`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x83\x01\x80T\x90`\0a)\xB7\x83aD\x18V[\x90\x91UPP`@\x80Q`\xA0\x81\x01\x90\x91R\x80`\x01\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01\x8A\x81R` \x01`\0\x81RPa\x01\xF5`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x04\x81\x11\x15a*GWa*GaA\tV[\x02\x17\x90UP` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01U``\x82\x01Q`\x03\x82\x01U`\x80\x90\x91\x01Q`\x04\x90\x91\x01U\x87\x15a*\x89Wa*\x89\x84\x8D\x89\x89\x89a3dV[\x8B\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x04Y\xABWU\x08\xB4\xA5\x89O{\x13\x87\xBF6-\x03!;;\xF81\xDAE&!\x8C3\xE4\xA9\x06\xD8\x8D`@Qa*\xC5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPPV[a\"\xC43\x82a6/V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\t;WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\t;V[a\x01\xF7T`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+\x8D\x91\x90\x81\x01\x90aE\xEEV[P\x94\x95PP`\x01`\x01`\xA0\x1B\x03\x85\x16\x93Pa+\xBF\x92PPPW`@Qc\x9D\xB8\xD5\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x04\x81\x11\x15a,\x05Wa,\x05aA\tV[`\x04\x81\x11\x15a,\x16Wa,\x16aA\tV[\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x90\x91\x01R\x90P`\0\x81Q`\x04\x81\x11\x15a,YWa,YaA\tV[\x03a,wW`@Qc\x04\xC9\xDE\x83`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q\x15a,\x9AW`@Qc\x02\xE0}\x87`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x01\xF4`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x90P\x81` \x01Q\x81`\x01\x01`\0\x82\x82Ta,\xDC\x91\x90aC\xF2V[\x92PP\x81\x90UP`\x01\x81`\x03\x01`\0\x82\x82Ta,\xF8\x91\x90aC\xF2V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x81\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x88\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16\x81U`\x01\x81\x01\x83\x90U`\x02\x81\x01\x83\x90U`\x03\x81\x01\x83\x90U`\x04\x01\x82\x90UQ\x86\x92\x91\x7F\x83\x1C\xD5\xB7S\x83\x80\xB0\xA2\xA3\x14\x14\xD64\xECBq\x16\x07V\x84\xA2v\x82\x8A\xB4\xD2ut\xA2\xDF\xDF\x91\xA3PPPPPV[a\"\xC4\x813a7\x8EV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a-\x97\x91\x90aFzV[PPP\x94P\x94P\x94PPPa-\xAD\x83\x83\x83a7\xC7V[\x95\x94PPPPPV[`@\x80Q` \x81\x01\x82\x90R`\x03``\x82\x01Rbivs`\xE8\x1B`\x80\x82\x01R\x90\x81\x01\x82\x90R`\0\x90`\xA0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x84\x82`@Q` \x01a.\x13\x92\x91\x90aG~V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a.6\x82a8\0V[\x90P`\0a.z\x82\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa8;\x92PPPV[\x90Pa.\x85\x87a.\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a.\xC6W`@Qc(\x80\xCB\x7F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a.\xE6\x91\x90aFzV[PPPPPP\x91PPa.\xF8\x81a8eV[\x93\x92PPPV[`\0\x80Q` aH\xC5\x839\x81Q\x91R\x80T`\x01\x19\x01a/1W`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x90UV[`\0`\0\x80Q` aH\xA5\x839\x81Q\x91Ra/R\x84\x84a\x1E\x90V[a/\xD2W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua/\x883\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\t;V[`\0\x91PPa\t;V[`\0`\0\x80Q` aH\xA5\x839\x81Q\x91Ra/\xF7\x84\x84a\x1E\x90V[\x15a/\xD2W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\t;V[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a0\xDFWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a0\xD3`\0\x80Q` aH\x85\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a%|W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x18\xD4\x81a-sV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a1bWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra1_\x91\x81\x01\x90aG\xA8V[`\x01[a1\x8AW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a.\xBDV[`\0\x80Q` aH\x85\x839\x81Q\x91R\x81\x14a1\xBBW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a.\xBDV[a\x0B@\x83\x83a8\x95V[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a%|W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01\x80T\x92\x93\x84\x93\x90\x92\x91`\xC0\x84\x01\x91a2\x88\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta2\xB4\x90aD1V[\x80\x15a3\x01W\x80`\x1F\x10a2\xD6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a3\x01V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a2\xE4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\xA0\x01Q\x83`\x80\x01Qa3+\x91\x90aDkV[a35\x91\x90aD\x82V[\x90P\x81`@\x01Q\x81\x10\x15a3MWP`\0\x93\x92PPPV[`@\x82\x01Qa3\\\x90\x82aC\xF2V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01\x80T\x92\x93\x92`\xC0\x84\x01\x91\x90a3\xDB\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta4\x07\x90aD1V[\x80\x15a4TW\x80`\x1F\x10a4)Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a4TV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a47W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x81Q\x91\x92PP`\x01`\x01`\xA0\x1B\x03\x16a4\x89W`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a4\x94\x86a7\x01V[\x91PPa4\xA0\x81a8\xEBV[a4\xBDW`@Qc!\xA4\x8B\x8B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ck[!\xA6a4\xF5\x87a-}V[a4\xFE\x89a9 V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5c\x91\x90aC\x92V[a5\x80W`@Qc\xC4e\xE6\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a5\x8B\x86a9OV[\x90Pa5\x99\x86\x86\x86\x8Ba-\xFEV[`@Qc4\xFE\xDEe`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ci\xFD\xBC\xCA\x90a5\xEB\x90\x8B\x90\x8B\x90\x86\x90\x8C\x90`\x04\x01aG\xC1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a6\x05W`\0\x80\xFD[PZ\xF1\x15\x80\x15a6\x19W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[a%|a9vV[`\0a6;\x83\x83a\x1ArV[P\x90P`\0\x81`\x04\x81\x11\x15a6RWa6RaA\tV[\x14\x15\x80\x15a6rWP`\x04\x81`\x04\x81\x11\x15a6oWa6oaA\tV[\x14\x15[a6\x8FW`@Qc0\x04\x82\x87`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16`\x04\x17\x81U\x90Q\x90\x92\x85\x92\x90\x91\x7F\xF9\xE1yp\xDFW\xA6\xA8E}\xCB\xB5\xC2\x91gkF1\xD37\xCFv\xB0\xC8\x01\xF6\xB8\xADj|_\x92\x91\x90\xA3\x80`\x04\x01T`\0\x03a\x15\x1BWa\x15\x1B\x84\x84a+\x1AV[a\x01\xF7T`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7|\x91\x90\x81\x01\x90aE\xEEV[P\x94\x9A\x93\x99P\x92\x97PPPPPPPPV[a7\x98\x82\x82a\x1E\x90V[a\x18\xD4W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a.\xBDV[`\0\x80\x84\x84\x84`@Q` \x01a7\xDF\x93\x92\x91\x90aH\x05V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01a-\xE1V[`\0\x80`\0\x80a8K\x86\x86a9\xBFV[\x92P\x92P\x92Pa8[\x82\x82a:\x0CV[P\x90\x94\x93PPPPV[`\0\x81Q`@\x14a8\x89W`@Qc\xD2\x833]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[a8\x9E\x82a:\xC5V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a8\xE3Wa\x0B@\x82\x82a;*V[a\x18\xD4a;\x97V[`\0\x81\x15\x80a9\x19WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x82\x14[\x15\x92\x91PPV[`@\x80Q` \x81\x01\x82\x90R`\x03``\x82\x01Rb3\xB2\xB7`\xE9\x1B`\x80\x82\x01R\x90\x81\x01\x82\x90R`\0\x90`\xA0\x01a-\xE1V[```\0\x82\x80` \x01\x90Q\x81\x01\x90a9g\x91\x90aFzV[P\x94\x99\x98PPPPPPPPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a%|W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x83Q`A\x03a9\xF9W` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa9\xEB\x88\x82\x85\x85a;\xB6V[\x95P\x95P\x95PPPPa:\x05V[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a: Wa: aA\tV[\x03a:)WPPV[`\x01\x82`\x03\x81\x11\x15a:=Wa:=aA\tV[\x03a:[W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a:oWa:oaA\tV[\x03a:\x90W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a.\xBDV[`\x03\x82`\x03\x81\x11\x15a:\xA4Wa:\xA4aA\tV[\x03a\x18\xD4W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a.\xBDV[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a:\xFBW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a.\xBDV[`\0\x80Q` aH\x85\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa;G\x91\x90aHHV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a;\x82W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a;\x87V[``\x91P[P\x91P\x91Pa-\xAD\x85\x83\x83a<\x85V[4\x15a%|W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a;\xF1WP`\0\x91P`\x03\x90P\x82a<{V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a<EW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a<qWP`\0\x92P`\x01\x91P\x82\x90Pa<{V[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82a<\x9AWa<\x95\x82a<\xE1V[a.\xF8V[\x81Q\x15\x80\x15a<\xB1WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a<\xDAW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a.\xBDV[P\x80a.\xF8V[\x80Q\x15a<\xF1W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Ta=\x16\x90aD1V[`\0\x82U\x80`\x1F\x10a=&WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\"\xC4\x91\x90[\x80\x82\x11\x15a=TW`\0\x81U`\x01\x01a=@V[P\x90V[`\0` \x82\x84\x03\x12\x15a=jW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a.\xF8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a=\x94W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"\xC4W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a=\xC5W`\0\x80\xFD[\x835a=\xD0\x81a=\x9BV[\x92P` \x84\x015a=\xE0\x81a=\x9BV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80` \x83\x85\x03\x12\x15a>\x04W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a>\x1AW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a>+W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a>AW`\0\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a>VW`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a>yW`\0\x80\xFD[\x825a>\x84\x81a=\x9BV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>\xD0Wa>\xD0a>\x92V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a>\xF1Wa>\xF1a>\x92V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a?\x10W`\0\x80\xFD[\x815a?#a?\x1E\x82a>\xD8V[a>\xA8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a?8W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a?gW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a?~W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r\x02W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a?\xACW`\0\x80\xFD[\x845\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xC9W`\0\x80\xFD[a?\xD5\x87\x82\x88\x01a>\xFFV[\x93PP`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xF1W`\0\x80\xFD[a?\xFD\x87\x82\x88\x01a?UV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a@\x1EW`\0\x80\xFD[\x835\x92P` \x84\x015a=\xE0\x81a=\x9BV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a@FW`\0\x80\xFD[\x845\x93P` \x85\x015a@X\x81a=\x9BV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a@\x80W`\0\x80\xFD[\x825\x91P` \x83\x015a@\x92\x81a=\x9BV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a@\xAFW`\0\x80\xFD[\x815a.\xF8\x81a=\x9BV[`\0\x80`@\x83\x85\x03\x12\x15a@\xCDW`\0\x80\xFD[\x825a@\xD8\x81a=\x9BV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xF3W`\0\x80\xFD[a@\xFF\x85\x82\x86\x01a>\xFFV[\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x05\x81\x10aA=WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`@\x81\x01aAO\x82\x85aA\x1FV[\x82` \x83\x01R\x93\x92PPPV[`\0[\x83\x81\x10\x15aAwW\x81\x81\x01Q\x83\x82\x01R` \x01aA_V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaA\x98\x81` \x86\x01` \x86\x01aA\\V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x88\x16\x81R\x86` \x82\x01R\x85`@\x82\x01R\x84``\x82\x01R\x83`\x80\x82\x01R\x82`\xA0\x82\x01R`\xE0`\xC0\x82\x01R`\0aA\xEB`\xE0\x83\x01\x84aA\x80V[\x99\x98PPPPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aB\rW`\0\x80\xFD[\x835aB\x18\x81a=\x9BV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aB:W`\0\x80\xFD[aBF\x86\x82\x87\x01a>\xFFV[\x91PP\x92P\x92P\x92V[`\xA0\x81\x01aB^\x82\x88aA\x1FV[\x85` \x83\x01R\x84`@\x83\x01R\x83``\x83\x01R\x82`\x80\x83\x01R\x96\x95PPPPPPV[` \x81R`\0a.\xF8` \x83\x01\x84aA\x80V[`\0\x80`\0``\x84\x86\x03\x12\x15aB\xA8W`\0\x80\xFD[\x835aB\xB3\x81a=\x9BV[\x92P` \x84\x015aB\xC3\x81a=\x9BV[\x91P`@\x84\x015aB\xD3\x81a=\x9BV[\x80\x91PP\x92P\x92P\x92V[\x80\x15\x15\x81\x14a\"\xC4W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15aC\x08W`\0\x80\xFD[\x885\x97P` \x89\x015\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015aC/\x81aB\xDEV[\x93P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aCJW`\0\x80\xFD[aCV\x8B\x82\x8C\x01a>\xFFV[\x93PP`\xC0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aCrW`\0\x80\xFD[aC~\x8B\x82\x8C\x01a?UV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0` \x82\x84\x03\x12\x15aC\xA4W`\0\x80\xFD[\x81Qa.\xF8\x81aB\xDEV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81aC\xEAWaC\xEAaC\xC5V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\t;Wa\t;aC\xC5V[\x80\x82\x01\x80\x82\x11\x15a\t;Wa\t;aC\xC5V[`\0`\x01\x82\x01aD*WaD*aC\xC5V[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aDEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aDeWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\t;Wa\t;aC\xC5V[`\0\x82aD\x9FWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x1F\x82\x11\x15a\x0B@W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aD\xCBWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x19\x81W`\0\x81U`\x01\x01aD\xD7V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x04WaE\x04a>\x92V[aE\x18\x81aE\x12\x84TaD1V[\x84aD\xA4V[` `\x1F\x82\x11`\x01\x81\x14aELW`\0\x83\x15aE4WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x19\x81V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aE|W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aE\\V[P\x84\x82\x10\x15aE\x9AW\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x82`\x1F\x83\x01\x12aE\xBAW`\0\x80\xFD[\x81QaE\xC8a?\x1E\x82a>\xD8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aE\xDDW`\0\x80\xFD[a3\\\x82` \x83\x01` \x87\x01aA\\V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15aF\tW`\0\x80\xFD[\x87QaF\x14\x81a=\x9BV[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q\x94\x9BP\x92\x99P\x90\x97P\x95P\x93PaFC\x81a=\x9BV[`\xC0\x89\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15aF_W`\0\x80\xFD[aFk\x8A\x82\x8B\x01aE\xA9V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aF\x97W`\0\x80\xFD[\x88Q`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xADW`\0\x80\xFD[aF\xB9\x8B\x82\x8C\x01aE\xA9V[\x98PP` \x89\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xD5W`\0\x80\xFD[aF\xE1\x8B\x82\x8C\x01aE\xA9V[\x97PP`@\x89\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xFDW`\0\x80\xFD[aG\t\x8B\x82\x8C\x01aE\xA9V[\x96PP``\x89\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aG%W`\0\x80\xFD[aG1\x8B\x82\x8C\x01aE\xA9V[\x95PP`\x80\x89\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aGMW`\0\x80\xFD[aGY\x8B\x82\x8C\x01aE\xA9V[`\xA0\x8B\x01Q`\xC0\x8C\x01Q`\xE0\x90\x9C\x01Q\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x90\x98\x90\x95P\x93PPPPV[`@\x81R`\0aG\x91`@\x83\x01\x85aA\x80V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aG\xBAW`\0\x80\xFD[PQ\x91\x90PV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0aG\xE8`\x80\x83\x01\x85aA\x80V[\x82\x81\x03``\x84\x01RaG\xFA\x81\x85aA\x80V[\x97\x96PPPPPPPV[`\0\x84QaH\x17\x81\x84` \x89\x01aA\\V[\x84Q\x90\x83\x01\x90aH+\x81\x83` \x89\x01aA\\V[\x84Q\x91\x01\x90aH>\x81\x83` \x88\x01aA\\V[\x01\x95\x94PPPPPV[`\0\x82QaHZ\x81\x84` \x87\x01aA\\V[\x91\x90\x91\x01\x92\x91PPV\xFE\xC7\x9BP*\x85%\xF5\x83\xD1)\xC1Ep\xE1|\xE9\xBC\xA2a\x10\xA5\xC4\x1A\xB7\xE2Uo\x95\xE0\x81\xFE\xC56\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\xA2dipfsX\"\x12 \xC2\xC7\x1Dn\x08\xA9\x0Fx\x86\xE5\x8F_\xB7P\xB5\x1F\x07\xF4\x88w\xED6\x14\xFE\xEC\xC4\x92\xBD\xF1:\x91\xE3dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static GENERATORREGISTRY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02\x88W`\x005`\xE0\x1C\x80caA?\x0F\x11a\x01ZW\x80c\x9A\x7F\xCA\x8E\x11a\0\xC1W\x80c\xC0\xC5;\x8B\x11a\0zW\x80c\xC0\xC5;\x8B\x14a\x08pW\x80c\xCB\x80\xD4\x18\x14a\x08\x90W\x80c\xD0n\x1F{\x14a\x08\xB0W\x80c\xD5Gt\x1F\x14a\x08\xD0W\x80c\xE2\xFA3\xCE\x14a\x08\xF0W\x80c\xE7\xBC\x96\0\x14a\t\x10W`\0\x80\xFD[\x80c\x9A\x7F\xCA\x8E\x14a\x07]W\x80c\x9F]\xB9\x86\x14a\x07\xC8W\x80c\xA2\x17\xFD\xDF\x14a\x07\xE8W\x80c\xA66\x16\xE6\x14a\x07\xFDW\x80c\xAD<\xB1\xCC\x14a\x08\x1DW\x80c\xAF\xF5\xED\xB1\x14a\x08[W`\0\x80\xFD[\x80c\x86J\x8E\xFA\x11a\x01\x13W\x80c\x86J\x8E\xFA\x14a\x06vW\x80c\x8C\xFCV\xD8\x14a\x06\xAAW\x80c\x91\xD1HT\x14a\x06\xDDW\x80c\x92\xEB\x91\xE2\x14a\x06\xFDW\x80c\x95\xA7\x8DI\x14a\x07\x1DW\x80c\x96\xDE\x0E\xEF\x14a\x07=W`\0\x80\xFD[\x80caA?\x0F\x14a\x05\xB3W\x80cdmQ\xB4\x14a\x05\xD3W\x80cf\x1D\xE5\xAC\x14a\x06\x01W\x80cm@Xw\x14a\x065W\x80cz\x14\xC4c\x14a\x056W\x80c\x81\xC4\\p\x14a\x06UW`\0\x80\xFD[\x80c+a\x0C-\x11a\x01\xFEW\x80c6V\x8A\xBE\x11a\x01\xB7W\x80c6V\x8A\xBE\x14a\x05\x16W\x80c<^\xB5|\x14a\x056W\x80cM*\xAB\x9A\x14a\x05KW\x80cO\x1E\xF2\x86\x14a\x05kW\x80cR\xD1\x90-\x14a\x05~W\x80cT\x1A\x8C\x18\x14a\x05\x93W`\0\x80\xFD[\x80c+a\x0C-\x14a\x04@W\x80c,\x1F\xBD\x03\x14a\x04\x7FW\x80c,4l\x0C\x14a\x04\xA1W\x80c//\xF1]\x14a\x04\xC1W\x80c/\x8FJ;\x14a\x04\xE1W\x80c1\xB5&\xBA\x14a\x04\xF6W`\0\x80\xFD[\x80c\x1C~\xAEe\x11a\x02PW\x80c\x1C~\xAEe\x14a\x03DW\x80c \x1A\x11\xB3\x14a\x03yW\x80c!\x80\xDE]\x14a\x03\x99W\x80c\"\x82\x8C\xC2\x14a\x03\xB9W\x80c$\x8A\x9C\xA3\x14a\x03\xF2W\x80c)S\xE7\xB3\x14a\x04 W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02\x8DW\x80c\x05jvh\x14a\x02\xC2W\x80c\x07\x81Y\xCD\x14a\x02\xE4W\x80c\x08\xBEk\xAD\x14a\x03\x04W\x80c\x1A\x1E\x8C\xF8\x14a\x03$W[`\0\x80\xFD[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\xADa\x02\xA86`\x04a=XV[a\t0V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xCEW`\0\x80\xFD[Pa\x02\xE2a\x02\xDD6`\x04a=\x82V[a\tAV[\0[4\x80\x15a\x02\xF0W`\0\x80\xFD[Pa\x02\xE2a\x02\xFF6`\x04a=\xB0V[a\n\x14V[4\x80\x15a\x03\x10W`\0\x80\xFD[Pa\x02\xE2a\x03\x1F6`\x04a=\xF1V[a\x0B\x0BV[4\x80\x15a\x030W`\0\x80\xFD[Pa\x02\xE2a\x03?6`\x04a>fV[a\x0BEV[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x03da\x03_6`\x04a>fV[a\x0CgV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xB9V[4\x80\x15a\x03\x85W`\0\x80\xFD[Pa\x02\xE2a\x03\x946`\x04a=\xB0V[a\r\tV[4\x80\x15a\x03\xA5W`\0\x80\xFD[Pa\x02\xE2a\x03\xB46`\x04a?\x96V[a\r\xF3V[4\x80\x15a\x03\xC5W`\0\x80\xFD[Pa\x01\xF8Ta\x03\xDA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xB9V[4\x80\x15a\x03\xFEW`\0\x80\xFD[Pa\x04\x12a\x04\r6`\x04a=\x82V[a\x0F\x85V[`@Q\x90\x81R` \x01a\x02\xB9V[4\x80\x15a\x04,W`\0\x80\xFD[Pa\x02\xE2a\x04;6`\x04a@\tV[a\x0F\xA7V[4\x80\x15a\x04LW`\0\x80\xFD[Pa\x04`a\x04[6`\x04a>fV[a\x11\x9FV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xB9V[4\x80\x15a\x04\x8BW`\0\x80\xFD[Pa\x04\x12`\0\x80Q` aHe\x839\x81Q\x91R\x81V[4\x80\x15a\x04\xADW`\0\x80\xFD[Pa\x02\xE2a\x04\xBC6`\x04a@0V[a\x13gV[4\x80\x15a\x04\xCDW`\0\x80\xFD[Pa\x02\xE2a\x04\xDC6`\x04a@mV[a\x14\xFFV[4\x80\x15a\x04\xEDW`\0\x80\xFD[Pa\x02\xE2a\x15!V[4\x80\x15a\x05\x02W`\0\x80\xFD[Pa\x02\xE2a\x05\x116`\x04a=\xB0V[a\x16\xF8V[4\x80\x15a\x05\"W`\0\x80\xFD[Pa\x02\xE2a\x0516`\x04a@mV[a\x17\xE2V[4\x80\x15a\x05BW`\0\x80\xFD[Pa\x04\x12`d\x81V[4\x80\x15a\x05WW`\0\x80\xFD[Pa\x02\xE2a\x05f6`\x04a@\x9DV[a\x18\x15V[a\x02\xE2a\x05y6`\x04a@\xBAV[a\x18\xB9V[4\x80\x15a\x05\x8AW`\0\x80\xFD[Pa\x04\x12a\x18\xD8V[4\x80\x15a\x05\x9FW`\0\x80\xFD[Pa\x02\xE2a\x05\xAE6`\x04a=\x82V[a\x18\xF5V[4\x80\x15a\x05\xBFW`\0\x80\xFD[Pa\x02\xE2a\x05\xCE6`\x04a=\xB0V[a\x19\x88V[4\x80\x15a\x05\xDFW`\0\x80\xFD[Pa\x05\xF3a\x05\xEE6`\x04a>fV[a\x1ArV[`@Qa\x02\xB9\x92\x91\x90aAAV[4\x80\x15a\x06\rW`\0\x80\xFD[Pa\x03\xDA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06AW`\0\x80\xFD[Pa\x02\xE2a\x06P6`\x04a=\x82V[a\x1D\x17V[4\x80\x15a\x06aW`\0\x80\xFD[Pa\x01\xF7Ta\x03\xDA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\x82W`\0\x80\xFD[Pa\x03\xDA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06\xB6W`\0\x80\xFD[Pa\x06\xCAa\x06\xC56`\x04a@\x9DV[a\x1D\xBEV[`@Qa\x02\xB9\x97\x96\x95\x94\x93\x92\x91\x90aA\xACV[4\x80\x15a\x06\xE9W`\0\x80\xFD[Pa\x02\xADa\x06\xF86`\x04a@mV[a\x1E\x90V[4\x80\x15a\x07\tW`\0\x80\xFD[Pa\x02\xE2a\x07\x186`\x04a?\x96V[a\x1E\xC8V[4\x80\x15a\x07)W`\0\x80\xFD[Pa\x02\xE2a\x0786`\x04aA\xF8V[a\x1E\xD5V[4\x80\x15a\x07IW`\0\x80\xFD[Pa\x02\xE2a\x07X6`\x04a=\x82V[a!;V[4\x80\x15a\x07iW`\0\x80\xFD[Pa\x07\xB7a\x07x6`\x04a>fV[a\x01\xF5` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T`\xFF\x90\x93\x16\x93\x91\x92\x90\x91\x90\x85V[`@Qa\x02\xB9\x95\x94\x93\x92\x91\x90aBPV[4\x80\x15a\x07\xD4W`\0\x80\xFD[Pa\x02\xE2a\x07\xE36`\x04a=\x82V[a\"\xBAV[4\x80\x15a\x07\xF4W`\0\x80\xFD[Pa\x04\x12`\0\x81V[4\x80\x15a\x08\tW`\0\x80\xFD[Pa\x02\xE2a\x08\x186`\x04a=\xB0V[a\"\xC7V[4\x80\x15a\x08)W`\0\x80\xFD[Pa\x08N`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02\xB9\x91\x90aB\x80V[4\x80\x15a\x08gW`\0\x80\xFD[Pa\x02\xE2a#\xB1V[4\x80\x15a\x08|W`\0\x80\xFD[Pa\x02\xE2a\x08\x8B6`\x04aB\x93V[a%~V[4\x80\x15a\x08\x9CW`\0\x80\xFD[Pa\x02\xE2a\x08\xAB6`\x04a=\xB0V[a&\xFCV[4\x80\x15a\x08\xBCW`\0\x80\xFD[Pa\x02\xE2a\x08\xCB6`\x04a=\xF1V[a'\xE6V[4\x80\x15a\x08\xDCW`\0\x80\xFD[Pa\x02\xE2a\x08\xEB6`\x04a@mV[a(\x1BV[4\x80\x15a\x08\xFCW`\0\x80\xFD[Pa\x02\xE2a\t\x0B6`\x04aB\xECV[a(7V[4\x80\x15a\t\x1CW`\0\x80\xFD[Pa\x02\xE2a\t+6`\x04a=\x82V[a*\xDBV[`\0a\t;\x82a*\xE5V[\x92\x91PPV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xC9\x91\x90aC\x92V[a\t\xE6W`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x81\x90\x7F\rAv7\xDA<\xA4\xE3\xCEN\xD4@?FLzU\xD3\xF0\xA9\x1F\"\xDDJo\xF1\x1D4V\xB1z\xDF\x90`\0\x90\xA2PV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nxW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9C\x91\x90aC\x92V[a\n\xB9W`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDB\xC0\x91\xCCw\x93\xE6t\x90\x95\xEA\x02\xFD\xC5\xB7Uy\x11u\xE4\x0E\xD8\0\x9D\xC8\x18 pyp\xB3\x99\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\0[\x81\x81\x10\x15a\x0B@Wa\x0B83\x84\x84\x84\x81\x81\x10a\x0B,Wa\x0B,aC\xAFV[\x90P` \x02\x015a+\x1AV[`\x01\x01a\x0B\x0EV[PPPV[`\0\x80Q` aHe\x839\x81Q\x91Ra\x0B]\x81a-sV[`\0a\x0Bi\x84\x84a\x1ArV[P\x90P`\0\x81`\x04\x81\x11\x15a\x0B\x80Wa\x0B\x80aA\tV[\x14\x80a\x0B\x9DWP`\x01\x81`\x04\x81\x11\x15a\x0B\x9BWa\x0B\x9BaA\tV[\x14[\x15a\x0B\xBAW`@Qb\xED=\xF3`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 a\x01\xF5\x83R\x81\x84 \x87\x85R\x90\x92R\x82 `\x04\x81\x01\x80T\x92\x93\x91\x92\x91a\x0B\xFA\x83aC\xDBV[\x91\x90PUP\x80`\x01\x01T\x82`\x02\x01`\0\x82\x82Ta\x0C\x17\x91\x90aC\xF2V[\x90\x91UPP`\x01\x81\x01T`@Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90\x7F\x0Bi \xA1i\xFB\x88\x9C\x15T\x91\x06\xAE\xE0p\xDA\x98\x18kz\xB4\x06\xDD\x93\x84\xA0\xD3\x18\x99\xFD\xF0\x8A\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x0C\xB3Wa\x0C\xB3aA\tV[`\x04\x81\x11\x15a\x0C\xC4Wa\x0C\xC4aA\tV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P\x80`@\x01Q\x81``\x01Q\x92P\x92PP[\x92P\x92\x90PV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x91\x91\x90aC\x92V[a\r\xAEW`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F5\x97\x03\x012<\xD4\x17\xC1\x11ER\x17y\xF7\xA9Q\x98\x84`\x82\xAC\x86[\x05\xD1\xC7\x96\xA3\\!\x13\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ck[!\xA6a\x0E+\x85a-}V[a\x0E4\x87a-\xB6V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EuW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x99\x91\x90aC\x92V[a\x0E\xB6W`@Qc\xC4e\xE6\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xC2\x83\x83\x833a-\xFEV[`@Qc\x07\x07Y\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x07\x07Y\x1F\x90a\x0F\x0E\x90\x86\x90`\x04\x01aB\x80V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F(W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F<W=`\0\x80>=`\0\xFD[PPPPa\x0FI\x83a.\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F\xC3\xFB\xBD\xB6\xAA\x8D\x99\xF6\xEF\xE2J:\"\xE9\xA9\x9F\xFE\xF2J/9\x9C\x0B\x1ET\x99F\xF9\x1B\xF06\xE4`@Q`@Q\x80\x91\x03\x90\xA3PPPPV[`\0\x90\x81R`\0\x80Q` aH\xA5\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[a\x0F\xAFa.\xFFV[`\0\x80Q` aHe\x839\x81Q\x91Ra\x0F\xC7\x81a-sV[`\0\x80a\x0F\xD4\x85\x85a\x1ArV[\x90\x92P\x90P`\x01\x82`\x04\x81\x11\x15a\x0F\xEDWa\x0F\xEDaA\tV[\x14\x80a\x10\nWP`\x03\x82`\x04\x81\x11\x15a\x10\x08Wa\x10\x08aA\tV[\x14[a\x10'W`@QcC\x9FL\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 a\x01\xF5\x83R\x81\x84 \x88\x85R\x90\x92R\x90\x91 `\x01\x81\x01T\x83\x10\x15a\x10yW`@Qc\x01\x19\"\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`d\x81`\x04\x01T\x11\x15a\x10\x9FW`@Qc\xCA\xBDP\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`\x01\x01T\x90P\x80\x83`\x02\x01`\0\x82\x82Ta\x10\xBC\x91\x90aD\x05V[\x90\x91UPPa\x01\xF8T`@Qc{:R\xF7`\xE1\x1B\x81R`\x04\x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`$\x83\x01R\x90\x91\x16\x90c\xF6t\xA5\xEE\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\x10W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11$W=`\0\x80>=`\0\xFD[PPPP\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD2\x9B\x9A\xDE\xE3+\xAF\x11\xBE\x04\0\x0Bsc\x99c\xD2\xC7n\x89\x9Fh0\xD3\xB5\xC1q]\xF1}\x82\xBD\x82`@Qa\x11c\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x04\x82\x01\x80T\x90`\0a\x11}\x83aD\x18V[\x91\x90PUPPPPPPPa\x0B@`\x01`\0\x80Q` aH\xC5\x839\x81Q\x91RUV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x11\xEBWa\x11\xEBaA\tV[`\x04\x81\x11\x15a\x11\xFCWa\x11\xFCaA\tV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x01\xF4`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`\xE0\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01\x80Ta\x12\xCE\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xFA\x90aD1V[\x80\x15a\x13GW\x80`\x1F\x10a\x13\x1CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13GV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13*W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x90Q`@\x90\x93\x01Q\x92\x94P\x91\x92PPP\x92P\x92\x90PV[`\0\x80Q` aHe\x839\x81Q\x91Ra\x13\x7F\x81a-sV[`\0a\x13\x8B\x85\x85a\x1ArV[P\x90P`\0\x81`\x04\x81\x11\x15a\x13\xA2Wa\x13\xA2aA\tV[\x14\x80a\x13\xBFWP`\x01\x81`\x04\x81\x11\x15a\x13\xBDWa\x13\xBDaA\tV[\x14[\x15a\x13\xDDW`@Qc\x1C\x0E\xC29`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 a\x01\xF5\x83R\x81\x84 \x88\x85R\x90\x92R\x82 `\x01\x81\x01T`\x02\x83\x01\x80T\x93\x94\x92\x93\x91\x92\x83\x92a\x14'\x90\x84\x90aC\xF2V[\x90\x91UPPa\x01\xF8T`@Qc\xC0\xFF\x97\xEF`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`$\x83\x01R`D\x82\x01\x89\x90R\x90\x91\x16\x90c\xC0\xFF\x97\xEF\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x96W=`\0\x80>=`\0\xFD[PPPP\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\x0Bi \xA1i\xFB\x88\x9C\x15T\x91\x06\xAE\xE0p\xDA\x98\x18kz\xB4\x06\xDD\x93\x84\xA0\xD3\x18\x99\xFD\xF0\x8A\x82`@Qa\x14\xD5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x04\x82\x01\x80T\x90`\0a\x14\xEF\x83aC\xDBV[\x91\x90PUPPPPPPPPPPV[a\x15\x08\x82a\x0F\x85V[a\x15\x11\x81a-sV[a\x15\x1B\x83\x83a/7V[PPPPV[3`\0\x81\x81Ra\x01\xF4` R`@\x90 `\x06\x81\x01\x80Ta\x15@\x90aD1V[\x15\x90P\x80a\x15VWP\x80T`\x01`\x01`\xA0\x1B\x03\x16\x15[\x15a\x15tW`@QcdF\xF9\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81`\x05\x01T\x03a\x15\xA0W`@Qc\x89\x83`\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x04\x01T\x83`\x05\x01Ta\x15\xBF\x91\x90aDkV[a\x15\xC9\x91\x90aD\x82V[\x90P`\0\x81\x83`\x04\x01Ta\x15\xDD\x91\x90aC\xF2V[\x90P\x82`\x02\x01T\x82\x10\x15a\x16\x04W`@Qc\x01\x19\"\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01\x01T\x82\x10\x15a\x16)W`@Qc\x01\x19\"\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x83\x01\x82\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x05\x84\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01\xF6` R`@\x90 TC\x10\x80\x15\x90a\x16\x7FWP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x01\xF6` R`@\x90 T\x15\x15[a\x16\x9CW`@Qc8l\xF4\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x01\xF6` R`@\x80\x82 \x91\x90\x91UQ\x7F\x11\xDE\xAE(\x9Epx\xFC\xE2\x88\xF4\xE9sN\x9C=\xF6{U\xEA\xC4\xFFA\xB8\x90\xB6\xD68\x1A\xCE\xE7b\x90a\x16\xEA\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x80\x91\x90aC\x92V[a\x17\x9DW`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE7\xA4\x8C\x90\x98\xE0\xE2\xAEV\xF8\xF1g0N\xEFTw\xB7*\x9D\x9E\xE1\xBA\x99Mb\xCD\xAF\x9B1\x0B\t\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x18\x0BW`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B@\x82\x82a/\xDCV[3`\0\x81\x81Ra\x01\xF4` R`@\x90 `\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80a\x18CWP\x80T`\x01`\x01`\xA0\x1B\x03\x16\x15[\x15a\x18aW`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x83U`@Q\x91\x82R\x83\x16\x90\x7F\xB1\x95\xA9K\xEC\xD3\x88\xC9\xA0\x7F\xE8\x18qh3\xBD\xF9\x8Bu\\x\xC9\xB48\xF4\xC8\xF0o5O\xA3h\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPPV[a\x18\xC1a0XV[a\x18\xCA\x82a0\xFDV[a\x18\xD4\x82\x82a1\x08V[PPV[`\0a\x18\xE2a1\xC5V[P`\0\x80Q` aH\x85\x839\x81Q\x91R\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xED8\r\x033`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19mW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\x81W=`\0\x80>=`\0\xFD[PPPPPV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x10\x91\x90aC\x92V[a\x1A-W`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F@\x93(\x8B\xC9[\xFE\xD3Z\x9C>\xA0\xDB\x07\xD8\xC6\xCB\xD9r,\xDFv\x972\xEE]s]a\xC3\xCF+\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x1A\xBEWa\x1A\xBEaA\tV[`\x04\x81\x11\x15a\x1A\xCFWa\x1A\xCFaA\tV[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x01\xF4`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`\xE0\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01\x80Ta\x1B\xA1\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B\xCD\x90aD1V[\x80\x15a\x1C\x1AW\x80`\x1F\x10a\x1B\xEFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\x1AV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B\xFDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0`\x04\x81\x11\x15a\x1C8Wa\x1C8aA\tV[\x82Q`\x04\x81\x11\x15a\x1CKWa\x1CKaA\tV[\x03a\x1C^W`\0\x80\x93P\x93PPPa\r\x02V[`\x04\x82Q`\x04\x81\x11\x15a\x1CsWa\x1CsaA\tV[\x03a\x1C\x87W`\x04`\0\x93P\x93PPPa\r\x02V[`\0a\x1C\x92\x87a2\x0EV[\x90P`\0\x83Q`\x04\x81\x11\x15a\x1C\xA9Wa\x1C\xA9aA\tV[\x14\x15\x80\x15a\x1C\xB5WP\x80\x15[\x15a\x1C\xCAW`\x02`\0\x94P\x94PPPPa\r\x02V[\x81`\x80\x01Q\x81\x03a\x1C\xE3W`\x01\x94P\x92Pa\r\x02\x91PPV[\x80\x15\x80\x15\x90a\x1C\xF5WP\x81`\x80\x01Q\x81\x10[\x15a\x1D\x08W`\x03\x94P\x92Pa\r\x02\x91PPV[P`\0\x96\x87\x96P\x94PPPPPV[3`\0\x81\x81Ra\x01\xF4` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\x1DLWP\x80`\x06\x01\x80Ta\x1DH\x90aD1V[\x15\x90P[\x15a\x1DjW`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81`\x04\x01`\0\x82\x82Ta\x1D~\x91\x90aD\x05V[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F \x98X\xCE\xC2\x8CQ\x8A\xEAD\xAA?\xA7\n\x93\"S\xF8\xBD\xF7j\x1C\x9A\x0B\x08\"j\x13\xE6\x975\xA3\x90` \x01a\x18\xACV[a\x01\xF4` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x97\x95\x96\x94\x95\x93\x94\x92\x93\x91\x92a\x1E\r\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E9\x90aD1V[\x80\x15a\x1E\x86W\x80`\x1F\x10a\x1E[Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x86V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1EiW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x87V[`\0\x91\x82R`\0\x80Q` aH\xA5\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x15\x1B3\x85\x85\x85\x85a3dV[a\x1E\xDDa.\xFFV[3`\0\x81\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01\x80T`\xC0\x84\x01\x91\x90a\x1FP\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F|\x90aD1V[\x80\x15a\x1F\xC9W\x80`\x1F\x10a\x1F\x9EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xC9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\xACW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x82Q`\0\x14\x80a\x1F\xEBWP`\x01`\x01`\xA0\x1B\x03\x85\x16\x15[\x80a\x1F\xF4WP\x83\x15[\x15a \x12W`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x15a ;W`@QcXt\xF9{`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xE0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R`\0` \x80\x84\x01\x82\x81R\x84\x86\x01\x83\x81R``\x86\x01\x84\x81R`\x80\x87\x01\x8C\x81Rg\r\xE0\xB6\xB3\xA7d\0\0`\xA0\x89\x01\x90\x81R`\xC0\x89\x01\x8D\x81R\x8C\x89\x16\x88Ra\x01\xF4\x90\x96R\x98\x90\x95 \x87Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16\x97\x16\x96\x90\x96\x17\x86U\x91Q`\x01\x86\x01UQ`\x02\x85\x01UQ`\x03\x84\x01U\x90Q`\x04\x83\x01U\x92Q`\x05\x82\x01U\x91Q\x90\x91\x90`\x06\x82\x01\x90a \xE1\x90\x82aD\xEBV[PP`@Q\x85\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x91P\x7F&>\xEC}8t\x99-\xAF.\x9B\xAB\x85(|a\x1A@\x01\x8A\xC6\xCD\x01\x08\xF1\xEE\xD1$\x91\x82\xDE\x04\x90` \x01`@Q\x80\x91\x03\x90\xA2PPa\x0B@`\x01`\0\x80Q` aH\xC5\x839\x81Q\x91RUV[3`\0\x81\x81Ra\x01\xF4` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a!pWP\x80`\x06\x01\x80Ta!l\x90aD1V[\x15\x90P[\x80a!yWP\x82\x15[\x15a!\x97W`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81`\x05\x01T\x14a!\xC3W`@Qc\x07\xECv9`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x82`\x04\x01Ta!\xD5\x91\x90aC\xF2V[\x90P\x81`\x01\x01T\x81\x11a!\xFBW`@Qc\xCA^b/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x82\x01T`\0\x90a\"\x15g\r\xE0\xB6\xB3\xA7d\0\0\x84aDkV[a\"\x1F\x91\x90aD\x82V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a\"IW`@Qc\xCA^b/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x83\x01\x81\x90Ua\"[`\x01CaD\x05V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x81\x81Ra\x01\xF6` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x7Fv\x12,\xFD_<h\0\xA2#\xE6\x0C\xC6$G\xC6I6\x03}x\xD0-\x91\x92\x93\xF6U\xBA}T\xCB\x90a\"\xAB\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[a\"\xC43\x82a+\x1AV[PV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#+W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#O\x91\x90aC\x92V[a#lW`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E=\xF7GA3\x9A\xFB\xBCk\xCA\xC6\n\xEB\xCD\x0E\xA5\xA2\xBB(\x07,\xFB\x016\x13\x8F\xCC^\xD8c\xFF\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[a#\xB9a.\xFFV[3`\0\x81\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01\x80T`\xC0\x84\x01\x91\x90a$,\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$X\x90aD1V[\x80\x15a$\xA5W\x80`\x1F\x10a$zWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$\xA5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\x88W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80` \x01Q`\0\x14a$\xD5W`@Qc\xF8\xC20S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01\xF4` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U`\x03\x81\x01\x82\x90U`\x04\x81\x01\x82\x90U`\x05\x81\x01\x82\x90U\x90a%-`\x06\x83\x01\x82a=\nV[PP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\x9F70\xAD\xE9K\xE5\xCE?\xAD\x97;\x88\x8At\x86j:\x91]\0\x8E\x8C\xBD\xE2\x13\x82\xB91\xB6|c\x90`\0\x90\xA2PPa%|`\x01`\0\x80Q` aH\xC5\x839\x81Q\x91RUV[V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a%\xC3WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a%\xDFWP0;\x15[\x90P\x81\x15\x80\x15a%\xEDWP\x80\x15[\x15a&\x0BW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a&5W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a&=a6'V[a&Ea6'V[a&Ma6'V[a&Ua6'V[a&``\0\x89a/7V[Pa&y`\0\x80Q` aHe\x839\x81Q\x91R\x88a/7V[Pa\x01\xF7\x80T`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92Ua\x01\xF8\x80T\x92\x89\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x83\x15a&\xF2W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[`@Qc&\xEE\xA2\x11`\xE2\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B\xBA\x88D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x84\x91\x90aC\x92V[a'\xA1W`@Qc\xA7\x10B\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x85r\xE9\xA7M\x9B\x02psW\xC4\x81\x9AR\xCF\xF3]TZH\x7F\xA3\xC9a\xD8\xBE\x8F\x9E\xC6Xa\xB6\x83`@Qa\n\xFE\x91\x81R` \x01\x90V[`\0[\x81\x81\x10\x15a\x0B@Wa(\x133\x84\x84\x84\x81\x81\x10a(\x07Wa(\x07aC\xAFV[\x90P` \x02\x015a6/V[`\x01\x01a'\xE9V[a($\x82a\x0F\x85V[a(-\x81a-sV[a\x15\x1B\x83\x83a/\xDCV[3`\0\x81\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 a\x01\xF5\x83R\x81\x84 \x8D\x85R\x90\x92R\x80\x83 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x92\x93\x92\x82\x90`\xFF\x16`\x04\x81\x11\x15a(\x7FWa(\x7FaA\tV[`\x04\x81\x11\x15a(\x90Wa(\x90aA\tV[\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x90\x91\x01R\x82T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a(\xD4WP\x87\x15[\x80a(\xDDWP\x89\x15[\x15a(\xFBW`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a)\x06\x8Ca7\x01V[P\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a)0W`@Qc\x9D\xB8\xD5\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82Q`\x04\x81\x11\x15a)EWa)EaA\tV[\x14a)cW`@Qc+[X\x0F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x8A\x83`\x01\x01`\0\x82\x82Ta)w\x91\x90aD\x05V[\x90\x91UPP`\x04\x83\x01T`\x01\x84\x01T\x11\x15a)\xA5W`@Qc\x07\xD9\x93\x89`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x83\x01\x80T\x90`\0a)\xB7\x83aD\x18V[\x90\x91UPP`@\x80Q`\xA0\x81\x01\x90\x91R\x80`\x01\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01\x8A\x81R` \x01`\0\x81RPa\x01\xF5`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x04\x81\x11\x15a*GWa*GaA\tV[\x02\x17\x90UP` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01U``\x82\x01Q`\x03\x82\x01U`\x80\x90\x91\x01Q`\x04\x90\x91\x01U\x87\x15a*\x89Wa*\x89\x84\x8D\x89\x89\x89a3dV[\x8B\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x04Y\xABWU\x08\xB4\xA5\x89O{\x13\x87\xBF6-\x03!;;\xF81\xDAE&!\x8C3\xE4\xA9\x06\xD8\x8D`@Qa*\xC5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPPV[a\"\xC43\x82a6/V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\t;WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\t;V[a\x01\xF7T`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+\x8D\x91\x90\x81\x01\x90aE\xEEV[P\x94\x95PP`\x01`\x01`\xA0\x1B\x03\x85\x16\x93Pa+\xBF\x92PPPW`@Qc\x9D\xB8\xD5\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x04\x81\x11\x15a,\x05Wa,\x05aA\tV[`\x04\x81\x11\x15a,\x16Wa,\x16aA\tV[\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x90\x91\x01R\x90P`\0\x81Q`\x04\x81\x11\x15a,YWa,YaA\tV[\x03a,wW`@Qc\x04\xC9\xDE\x83`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q\x15a,\x9AW`@Qc\x02\xE0}\x87`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x01\xF4`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x90P\x81` \x01Q\x81`\x01\x01`\0\x82\x82Ta,\xDC\x91\x90aC\xF2V[\x92PP\x81\x90UP`\x01\x81`\x03\x01`\0\x82\x82Ta,\xF8\x91\x90aC\xF2V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x81\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x88\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16\x81U`\x01\x81\x01\x83\x90U`\x02\x81\x01\x83\x90U`\x03\x81\x01\x83\x90U`\x04\x01\x82\x90UQ\x86\x92\x91\x7F\x83\x1C\xD5\xB7S\x83\x80\xB0\xA2\xA3\x14\x14\xD64\xECBq\x16\x07V\x84\xA2v\x82\x8A\xB4\xD2ut\xA2\xDF\xDF\x91\xA3PPPPPV[a\"\xC4\x813a7\x8EV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a-\x97\x91\x90aFzV[PPP\x94P\x94P\x94PPPa-\xAD\x83\x83\x83a7\xC7V[\x95\x94PPPPPV[`@\x80Q` \x81\x01\x82\x90R`\x03``\x82\x01Rbivs`\xE8\x1B`\x80\x82\x01R\x90\x81\x01\x82\x90R`\0\x90`\xA0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x84\x82`@Q` \x01a.\x13\x92\x91\x90aG~V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a.6\x82a8\0V[\x90P`\0a.z\x82\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa8;\x92PPPV[\x90Pa.\x85\x87a.\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a.\xC6W`@Qc(\x80\xCB\x7F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a.\xE6\x91\x90aFzV[PPPPPP\x91PPa.\xF8\x81a8eV[\x93\x92PPPV[`\0\x80Q` aH\xC5\x839\x81Q\x91R\x80T`\x01\x19\x01a/1W`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x90UV[`\0`\0\x80Q` aH\xA5\x839\x81Q\x91Ra/R\x84\x84a\x1E\x90V[a/\xD2W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua/\x883\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\t;V[`\0\x91PPa\t;V[`\0`\0\x80Q` aH\xA5\x839\x81Q\x91Ra/\xF7\x84\x84a\x1E\x90V[\x15a/\xD2W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\t;V[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a0\xDFWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a0\xD3`\0\x80Q` aH\x85\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a%|W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x18\xD4\x81a-sV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a1bWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra1_\x91\x81\x01\x90aG\xA8V[`\x01[a1\x8AW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a.\xBDV[`\0\x80Q` aH\x85\x839\x81Q\x91R\x81\x14a1\xBBW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a.\xBDV[a\x0B@\x83\x83a8\x95V[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a%|W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01\x80T\x92\x93\x84\x93\x90\x92\x91`\xC0\x84\x01\x91a2\x88\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta2\xB4\x90aD1V[\x80\x15a3\x01W\x80`\x1F\x10a2\xD6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a3\x01V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a2\xE4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\xA0\x01Q\x83`\x80\x01Qa3+\x91\x90aDkV[a35\x91\x90aD\x82V[\x90P\x81`@\x01Q\x81\x10\x15a3MWP`\0\x93\x92PPPV[`@\x82\x01Qa3\\\x90\x82aC\xF2V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01\x80T\x92\x93\x92`\xC0\x84\x01\x91\x90a3\xDB\x90aD1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta4\x07\x90aD1V[\x80\x15a4TW\x80`\x1F\x10a4)Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a4TV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a47W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x81Q\x91\x92PP`\x01`\x01`\xA0\x1B\x03\x16a4\x89W`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a4\x94\x86a7\x01V[\x91PPa4\xA0\x81a8\xEBV[a4\xBDW`@Qc!\xA4\x8B\x8B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ck[!\xA6a4\xF5\x87a-}V[a4\xFE\x89a9 V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5c\x91\x90aC\x92V[a5\x80W`@Qc\xC4e\xE6\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a5\x8B\x86a9OV[\x90Pa5\x99\x86\x86\x86\x8Ba-\xFEV[`@Qc4\xFE\xDEe`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ci\xFD\xBC\xCA\x90a5\xEB\x90\x8B\x90\x8B\x90\x86\x90\x8C\x90`\x04\x01aG\xC1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a6\x05W`\0\x80\xFD[PZ\xF1\x15\x80\x15a6\x19W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[a%|a9vV[`\0a6;\x83\x83a\x1ArV[P\x90P`\0\x81`\x04\x81\x11\x15a6RWa6RaA\tV[\x14\x15\x80\x15a6rWP`\x04\x81`\x04\x81\x11\x15a6oWa6oaA\tV[\x14\x15[a6\x8FW`@Qc0\x04\x82\x87`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16`\x04\x17\x81U\x90Q\x90\x92\x85\x92\x90\x91\x7F\xF9\xE1yp\xDFW\xA6\xA8E}\xCB\xB5\xC2\x91gkF1\xD37\xCFv\xB0\xC8\x01\xF6\xB8\xADj|_\x92\x91\x90\xA3\x80`\x04\x01T`\0\x03a\x15\x1BWa\x15\x1B\x84\x84a+\x1AV[a\x01\xF7T`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7|\x91\x90\x81\x01\x90aE\xEEV[P\x94\x9A\x93\x99P\x92\x97PPPPPPPPV[a7\x98\x82\x82a\x1E\x90V[a\x18\xD4W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a.\xBDV[`\0\x80\x84\x84\x84`@Q` \x01a7\xDF\x93\x92\x91\x90aH\x05V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01a-\xE1V[`\0\x80`\0\x80a8K\x86\x86a9\xBFV[\x92P\x92P\x92Pa8[\x82\x82a:\x0CV[P\x90\x94\x93PPPPV[`\0\x81Q`@\x14a8\x89W`@Qc\xD2\x833]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[a8\x9E\x82a:\xC5V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a8\xE3Wa\x0B@\x82\x82a;*V[a\x18\xD4a;\x97V[`\0\x81\x15\x80a9\x19WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x82\x14[\x15\x92\x91PPV[`@\x80Q` \x81\x01\x82\x90R`\x03``\x82\x01Rb3\xB2\xB7`\xE9\x1B`\x80\x82\x01R\x90\x81\x01\x82\x90R`\0\x90`\xA0\x01a-\xE1V[```\0\x82\x80` \x01\x90Q\x81\x01\x90a9g\x91\x90aFzV[P\x94\x99\x98PPPPPPPPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a%|W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x83Q`A\x03a9\xF9W` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa9\xEB\x88\x82\x85\x85a;\xB6V[\x95P\x95P\x95PPPPa:\x05V[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a: Wa: aA\tV[\x03a:)WPPV[`\x01\x82`\x03\x81\x11\x15a:=Wa:=aA\tV[\x03a:[W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a:oWa:oaA\tV[\x03a:\x90W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a.\xBDV[`\x03\x82`\x03\x81\x11\x15a:\xA4Wa:\xA4aA\tV[\x03a\x18\xD4W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a.\xBDV[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a:\xFBW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a.\xBDV[`\0\x80Q` aH\x85\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa;G\x91\x90aHHV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a;\x82W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a;\x87V[``\x91P[P\x91P\x91Pa-\xAD\x85\x83\x83a<\x85V[4\x15a%|W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a;\xF1WP`\0\x91P`\x03\x90P\x82a<{V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a<EW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a<qWP`\0\x92P`\x01\x91P\x82\x90Pa<{V[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82a<\x9AWa<\x95\x82a<\xE1V[a.\xF8V[\x81Q\x15\x80\x15a<\xB1WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a<\xDAW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a.\xBDV[P\x80a.\xF8V[\x80Q\x15a<\xF1W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Ta=\x16\x90aD1V[`\0\x82U\x80`\x1F\x10a=&WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\"\xC4\x91\x90[\x80\x82\x11\x15a=TW`\0\x81U`\x01\x01a=@V[P\x90V[`\0` \x82\x84\x03\x12\x15a=jW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a.\xF8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a=\x94W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"\xC4W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a=\xC5W`\0\x80\xFD[\x835a=\xD0\x81a=\x9BV[\x92P` \x84\x015a=\xE0\x81a=\x9BV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80` \x83\x85\x03\x12\x15a>\x04W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a>\x1AW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a>+W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a>AW`\0\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a>VW`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a>yW`\0\x80\xFD[\x825a>\x84\x81a=\x9BV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>\xD0Wa>\xD0a>\x92V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a>\xF1Wa>\xF1a>\x92V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a?\x10W`\0\x80\xFD[\x815a?#a?\x1E\x82a>\xD8V[a>\xA8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a?8W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a?gW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a?~W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r\x02W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a?\xACW`\0\x80\xFD[\x845\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xC9W`\0\x80\xFD[a?\xD5\x87\x82\x88\x01a>\xFFV[\x93PP`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xF1W`\0\x80\xFD[a?\xFD\x87\x82\x88\x01a?UV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a@\x1EW`\0\x80\xFD[\x835\x92P` \x84\x015a=\xE0\x81a=\x9BV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a@FW`\0\x80\xFD[\x845\x93P` \x85\x015a@X\x81a=\x9BV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a@\x80W`\0\x80\xFD[\x825\x91P` \x83\x015a@\x92\x81a=\x9BV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a@\xAFW`\0\x80\xFD[\x815a.\xF8\x81a=\x9BV[`\0\x80`@\x83\x85\x03\x12\x15a@\xCDW`\0\x80\xFD[\x825a@\xD8\x81a=\x9BV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xF3W`\0\x80\xFD[a@\xFF\x85\x82\x86\x01a>\xFFV[\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x05\x81\x10aA=WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`@\x81\x01aAO\x82\x85aA\x1FV[\x82` \x83\x01R\x93\x92PPPV[`\0[\x83\x81\x10\x15aAwW\x81\x81\x01Q\x83\x82\x01R` \x01aA_V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaA\x98\x81` \x86\x01` \x86\x01aA\\V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x88\x16\x81R\x86` \x82\x01R\x85`@\x82\x01R\x84``\x82\x01R\x83`\x80\x82\x01R\x82`\xA0\x82\x01R`\xE0`\xC0\x82\x01R`\0aA\xEB`\xE0\x83\x01\x84aA\x80V[\x99\x98PPPPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aB\rW`\0\x80\xFD[\x835aB\x18\x81a=\x9BV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aB:W`\0\x80\xFD[aBF\x86\x82\x87\x01a>\xFFV[\x91PP\x92P\x92P\x92V[`\xA0\x81\x01aB^\x82\x88aA\x1FV[\x85` \x83\x01R\x84`@\x83\x01R\x83``\x83\x01R\x82`\x80\x83\x01R\x96\x95PPPPPPV[` \x81R`\0a.\xF8` \x83\x01\x84aA\x80V[`\0\x80`\0``\x84\x86\x03\x12\x15aB\xA8W`\0\x80\xFD[\x835aB\xB3\x81a=\x9BV[\x92P` \x84\x015aB\xC3\x81a=\x9BV[\x91P`@\x84\x015aB\xD3\x81a=\x9BV[\x80\x91PP\x92P\x92P\x92V[\x80\x15\x15\x81\x14a\"\xC4W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15aC\x08W`\0\x80\xFD[\x885\x97P` \x89\x015\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015aC/\x81aB\xDEV[\x93P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aCJW`\0\x80\xFD[aCV\x8B\x82\x8C\x01a>\xFFV[\x93PP`\xC0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aCrW`\0\x80\xFD[aC~\x8B\x82\x8C\x01a?UV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0` \x82\x84\x03\x12\x15aC\xA4W`\0\x80\xFD[\x81Qa.\xF8\x81aB\xDEV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81aC\xEAWaC\xEAaC\xC5V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\t;Wa\t;aC\xC5V[\x80\x82\x01\x80\x82\x11\x15a\t;Wa\t;aC\xC5V[`\0`\x01\x82\x01aD*WaD*aC\xC5V[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aDEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aDeWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\t;Wa\t;aC\xC5V[`\0\x82aD\x9FWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x1F\x82\x11\x15a\x0B@W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aD\xCBWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x19\x81W`\0\x81U`\x01\x01aD\xD7V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x04WaE\x04a>\x92V[aE\x18\x81aE\x12\x84TaD1V[\x84aD\xA4V[` `\x1F\x82\x11`\x01\x81\x14aELW`\0\x83\x15aE4WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x19\x81V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aE|W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aE\\V[P\x84\x82\x10\x15aE\x9AW\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x82`\x1F\x83\x01\x12aE\xBAW`\0\x80\xFD[\x81QaE\xC8a?\x1E\x82a>\xD8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aE\xDDW`\0\x80\xFD[a3\\\x82` \x83\x01` \x87\x01aA\\V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15aF\tW`\0\x80\xFD[\x87QaF\x14\x81a=\x9BV[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q\x94\x9BP\x92\x99P\x90\x97P\x95P\x93PaFC\x81a=\x9BV[`\xC0\x89\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15aF_W`\0\x80\xFD[aFk\x8A\x82\x8B\x01aE\xA9V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aF\x97W`\0\x80\xFD[\x88Q`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xADW`\0\x80\xFD[aF\xB9\x8B\x82\x8C\x01aE\xA9V[\x98PP` \x89\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xD5W`\0\x80\xFD[aF\xE1\x8B\x82\x8C\x01aE\xA9V[\x97PP`@\x89\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xFDW`\0\x80\xFD[aG\t\x8B\x82\x8C\x01aE\xA9V[\x96PP``\x89\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aG%W`\0\x80\xFD[aG1\x8B\x82\x8C\x01aE\xA9V[\x95PP`\x80\x89\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aGMW`\0\x80\xFD[aGY\x8B\x82\x8C\x01aE\xA9V[`\xA0\x8B\x01Q`\xC0\x8C\x01Q`\xE0\x90\x9C\x01Q\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x90\x98\x90\x95P\x93PPPPV[`@\x81R`\0aG\x91`@\x83\x01\x85aA\x80V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aG\xBAW`\0\x80\xFD[PQ\x91\x90PV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0aG\xE8`\x80\x83\x01\x85aA\x80V[\x82\x81\x03``\x84\x01RaG\xFA\x81\x85aA\x80V[\x97\x96PPPPPPPV[`\0\x84QaH\x17\x81\x84` \x89\x01aA\\V[\x84Q\x90\x83\x01\x90aH+\x81\x83` \x89\x01aA\\V[\x84Q\x91\x01\x90aH>\x81\x83` \x88\x01aA\\V[\x01\x95\x94PPPPPV[`\0\x82QaHZ\x81\x84` \x87\x01aA\\V[\x91\x90\x91\x01\x92\x91PPV\xFE\xC7\x9BP*\x85%\xF5\x83\xD1)\xC1Ep\xE1|\xE9\xBC\xA2a\x10\xA5\xC4\x1A\xB7\xE2Uo\x95\xE0\x81\xFE\xC56\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\xA2dipfsX\"\x12 \xC2\xC7\x1Dn\x08\xA9\x0Fx\x86\xE5\x8F_\xB7P\xB5\x1F\x07\xF4\x88w\xED6\x14\xFE\xEC\xC4\x92\xBD\xF1:\x91\xE3dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static GENERATORREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct GeneratorRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GeneratorRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GeneratorRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GeneratorRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GeneratorRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GeneratorRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GeneratorRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                GENERATORREGISTRY_ABI.clone(),
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
                GENERATORREGISTRY_ABI.clone(),
                GENERATORREGISTRY_BYTECODE.clone().into(),
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
        ///Calls the contract's `ENTITY_KEY_REGISTRY` (0x661de5ac) function
        pub fn entity_key_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([102, 29, 229, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PARALLEL_REQUESTS_UPPER_LIMIT` (0x7a14c463) function
        pub fn parallel_requests_upper_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 20, 196, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROOF_MARKET_PLACE_ROLE` (0x2c1fbd03) function
        pub fn proof_market_place_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([44, 31, 189, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKING_MANAGER` (0x864a8efa) function
        pub fn STAKING_MANAGER(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([134, 74, 142, 250], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNLOCK_WAIT_BLOCKS` (0x3c5eb57c) function
        pub fn unlock_wait_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 94, 181, 124], ())
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
        ///Calls the contract's `addIvsKey` (0x2180de5d) function
        pub fn add_ivs_key(
            &self,
            market_id: ::ethers::core::types::U256,
            attestation_data: ::ethers::core::types::Bytes,
            enclave_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [33, 128, 222, 93],
                    (market_id, attestation_data, enclave_signature),
                )
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `assignGeneratorTask` (0x2953e7b3) function
        pub fn assign_generator_task(
            &self,
            ask_id: ::ethers::core::types::U256,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 83, 231, 179], (ask_id, generator_address, market_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeRewardAddress` (0x4d2aab9a) function
        pub fn change_reward_address(
            &self,
            new_reward_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 42, 171, 154], new_reward_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeGeneratorTask` (0x2c346c0c) function
        pub fn complete_generator_task(
            &self,
            ask_id: ::ethers::core::types::U256,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
            stake_to_release: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 52, 108, 12],
                    (ask_id, generator_address, market_id, stake_to_release),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseDeclaredCompute` (0x2f8f4a3b) function
        pub fn decrease_declared_compute(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 143, 74, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregister` (0xaff5edb1) function
        pub fn deregister(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([175, 245, 237, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `generatorInfoPerMarket` (0x9a7fca8e) function
        pub fn generator_info_per_market(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u8,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([154, 127, 202, 142], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `generatorRegistry` (0x8cfc56d8) function
        pub fn generator_registry(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([140, 252, 86, 216], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGeneratorAssignmentDetails` (0x1c7eae65) function
        pub fn get_generator_assignment_details(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([28, 126, 174, 101], (generator_address, market_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGeneratorRewardDetails` (0x2b610c2d) function
        pub fn get_generator_reward_details(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([43, 97, 12, 45], (generator_address, market_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGeneratorState` (0x646d51b4) function
        pub fn get_generator_state(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (u8, ::ethers::core::types::U256)>
        {
            self.0
                .method_hash([100, 109, 81, 180], (generator_address, market_id))
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
        ///Calls the contract's `increaseDeclaredCompute` (0x6d405877) function
        pub fn increase_declared_compute(
            &self,
            compute_to_increase: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 64, 88, 119], compute_to_increase)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc0c53b8b) function
        pub fn initialize(
            &self,
            admin: ::ethers::core::types::Address,
            proof_marketplace: ::ethers::core::types::Address,
            staking_manager: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [192, 197, 59, 139],
                    (admin, proof_marketplace, staking_manager),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intendToReduceCompute` (0x96de0eef) function
        pub fn intend_to_reduce_compute(
            &self,
            compute_to_reduce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 222, 14, 239], compute_to_reduce)
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
        ///Calls the contract's `joinMarketplace` (0xe2fa33ce) function
        pub fn join_marketplace(
            &self,
            market_id: ::ethers::core::types::U256,
            compute_per_request_required: ::ethers::core::types::U256,
            proof_generation_cost: ::ethers::core::types::U256,
            proposed_time: ::ethers::core::types::U256,
            update_market_dedicated_key: bool,
            attestation_data: ::ethers::core::types::Bytes,
            enclave_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [226, 250, 51, 206],
                    (
                        market_id,
                        compute_per_request_required,
                        proof_generation_cost,
                        proposed_time,
                        update_market_dedicated_key,
                        attestation_data,
                        enclave_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `leaveMarketplace` (0x9f5db986) function
        pub fn leave_marketplace(
            &self,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 93, 185, 134], market_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `leaveMarketplaces` (0x08be6bad) function
        pub fn leave_marketplaces(
            &self,
            market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 190, 107, 173], market_ids)
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
        ///Calls the contract's `register` (0x95a78d49) function
        pub fn register(
            &self,
            reward_address: ::ethers::core::types::Address,
            declared_compute: ::ethers::core::types::U256,
            generator_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [149, 167, 141, 73],
                    (reward_address, declared_compute, generator_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `releaseGeneratorResources` (0x1a1e8cf8) function
        pub fn release_generator_resources(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([26, 30, 140, 248], (generator_address, market_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeEncryptionKey` (0x541a8c18) function
        pub fn remove_encryption_key(
            &self,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 26, 140, 24], market_id)
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
        ///Calls the contract's `requestForExitMarketplace` (0xe7bc9600) function
        pub fn request_for_exit_marketplace(
            &self,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 188, 150, 0], market_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestForExitMarketplaces` (0xd06e1f7b) function
        pub fn request_for_exit_marketplaces(
            &self,
            market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 110, 31, 123], market_ids)
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
        ///Calls the contract's `stakingManager` (0x22828cc2) function
        pub fn stakingManager(
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
        ///Calls the contract's `symbioticCompleteSnapshotCallback` (0x056a7668) function
        pub fn symbiotic_complete_snapshot_callback(
            &self,
            capture_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 106, 118, 104], capture_timestamp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateEncryptionKey` (0x92eb91e2) function
        pub fn update_encryption_key(
            &self,
            market_id: ::ethers::core::types::U256,
            attestation_data: ::ethers::core::types::Bytes,
            enclave_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [146, 235, 145, 226],
                    (market_id, attestation_data, enclave_signature),
                )
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
        ///Gets the contract's `AddIvsKey` event
        pub fn add_ivs_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddIvsKeyFilter> {
            self.0.event()
        }
        ///Gets the contract's `AddedStake` event
        pub fn added_stake_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddedStakeFilter> {
            self.0.event()
        }
        ///Gets the contract's `ChangedGeneratorRewardAddress` event
        pub fn changed_generator_reward_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedGeneratorRewardAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ComputeLockImposed` event
        pub fn compute_lock_imposed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ComputeLockImposedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ComputeLockReleased` event
        pub fn compute_lock_released_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ComputeLockReleasedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `DecreaseCompute` event
        pub fn decrease_compute_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DecreaseComputeFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `DeregisteredGenerator` event
        pub fn deregistered_generator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DeregisteredGeneratorFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `IncreasedCompute` event
        pub fn increased_compute_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IncreasedComputeFilter>
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
        ///Gets the contract's `JoinedMarketplace` event
        pub fn joined_marketplace_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, JoinedMarketplaceFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LeftMarketplace` event
        pub fn left_marketplace_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LeftMarketplaceFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RegisteredGenerator` event
        pub fn registered_generator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RegisteredGeneratorFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RemovedStake` event
        pub fn removed_stake_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemovedStakeFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RequestComputeDecrease` event
        pub fn request_compute_decrease_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RequestComputeDecreaseFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RequestExitMarketplace` event
        pub fn request_exit_marketplace_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RequestExitMarketplaceFilter>
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
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GeneratorRegistryEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for GeneratorRegistry<M>
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
    ///Custom Error type `AlreadyJoinedMarket` with signature `AlreadyJoinedMarket()` and selector `0xad6d603c`
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
    #[etherror(name = "AlreadyJoinedMarket", abi = "AlreadyJoinedMarket()")]
    pub struct AlreadyJoinedMarket;
    ///Custom Error type `AssignOnlyToIdleGenerators` with signature `AssignOnlyToIdleGenerators()` and selector `0x439f4ca7`
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
        name = "AssignOnlyToIdleGenerators",
        abi = "AssignOnlyToIdleGenerators()"
    )]
    pub struct AssignOnlyToIdleGenerators;
    ///Custom Error type `CannotBeMoreThanDeclaredCompute` with signature `CannotBeMoreThanDeclaredCompute()` and selector `0x7d993890`
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
        name = "CannotBeMoreThanDeclaredCompute",
        abi = "CannotBeMoreThanDeclaredCompute()"
    )]
    pub struct CannotBeMoreThanDeclaredCompute;
    ///Custom Error type `CannotBeSlashed` with signature `CannotBeSlashed()` and selector `0xed3df300`
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
    #[etherror(name = "CannotBeSlashed", abi = "CannotBeSlashed()")]
    pub struct CannotBeSlashed;
    ///Custom Error type `CannotBeZero` with signature `CannotBeZero()` and selector `0x1e1d0ab5`
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
    #[etherror(name = "CannotBeZero", abi = "CannotBeZero()")]
    pub struct CannotBeZero;
    ///Custom Error type `CannotLeaveMarketWithActiveRequest` with signature `CannotLeaveMarketWithActiveRequest()` and selector `0xb81f61c0`
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
        name = "CannotLeaveMarketWithActiveRequest",
        abi = "CannotLeaveMarketWithActiveRequest()"
    )]
    pub struct CannotLeaveMarketWithActiveRequest;
    ///Custom Error type `CannotLeaveWithActiveMarket` with signature `CannotLeaveWithActiveMarket()` and selector `0xf8c23053`
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
        name = "CannotLeaveWithActiveMarket",
        abi = "CannotLeaveWithActiveMarket()"
    )]
    pub struct CannotLeaveWithActiveMarket;
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
    ///Custom Error type `ExceedsAcceptableRange` with signature `ExceedsAcceptableRange()` and selector `0xca5e622f`
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
    #[etherror(name = "ExceedsAcceptableRange", abi = "ExceedsAcceptableRange()")]
    pub struct ExceedsAcceptableRange;
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
    ///Custom Error type `GeneratorAlreadyExists` with signature `GeneratorAlreadyExists()` and selector `0x5874f97b`
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
    #[etherror(name = "GeneratorAlreadyExists", abi = "GeneratorAlreadyExists()")]
    pub struct GeneratorAlreadyExists;
    ///Custom Error type `IncorrectImageId` with signature `IncorrectImageId()` and selector `0xc465e69d`
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
    #[etherror(name = "IncorrectImageId", abi = "IncorrectImageId()")]
    pub struct IncorrectImageId;
    ///Custom Error type `InsufficientGeneratorComputeAvailable` with signature `InsufficientGeneratorComputeAvailable()` and selector `0x08c915b8`
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
        name = "InsufficientGeneratorComputeAvailable",
        abi = "InsufficientGeneratorComputeAvailable()"
    )]
    pub struct InsufficientGeneratorComputeAvailable;
    ///Custom Error type `InvalidContractAddress` with signature `InvalidContractAddress()` and selector `0xa710429d`
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
    #[etherror(name = "InvalidContractAddress", abi = "InvalidContractAddress()")]
    pub struct InvalidContractAddress;
    ///Custom Error type `InvalidEnclaveKey` with signature `InvalidEnclaveKey()` and selector `0xd283335d`
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
    #[etherror(name = "InvalidEnclaveKey", abi = "InvalidEnclaveKey()")]
    pub struct InvalidEnclaveKey;
    ///Custom Error type `InvalidEnclaveSignature` with signature `InvalidEnclaveSignature(address)` and selector `0x2880cb7f`
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
        name = "InvalidEnclaveSignature",
        abi = "InvalidEnclaveSignature(address)"
    )]
    pub struct InvalidEnclaveSignature {
        pub invalid_signer_address: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidGenerator` with signature `InvalidGenerator()` and selector `0x6446f917`
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
    #[etherror(name = "InvalidGenerator", abi = "InvalidGenerator()")]
    pub struct InvalidGenerator;
    ///Custom Error type `InvalidGeneratorStatePerMarket` with signature `InvalidGeneratorStatePerMarket()` and selector `0x264ef418`
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
        name = "InvalidGeneratorStatePerMarket",
        abi = "InvalidGeneratorStatePerMarket()"
    )]
    pub struct InvalidGeneratorStatePerMarket;
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
    ///Custom Error type `InvalidMarket` with signature `InvalidMarket()` and selector `0x9db8d5b1`
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
    #[etherror(name = "InvalidMarket", abi = "InvalidMarket()")]
    pub struct InvalidMarket;
    ///Custom Error type `MaxParallelRequestsPerMarketExceeded` with signature `MaxParallelRequestsPerMarketExceeded()` and selector `0xcabd50d7`
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
        name = "MaxParallelRequestsPerMarketExceeded",
        abi = "MaxParallelRequestsPerMarketExceeded()"
    )]
    pub struct MaxParallelRequestsPerMarketExceeded;
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
    ///Custom Error type `OnlyValidGeneratorsCanRequestExit` with signature `OnlyValidGeneratorsCanRequestExit()` and selector `0xc0120a1c`
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
        name = "OnlyValidGeneratorsCanRequestExit",
        abi = "OnlyValidGeneratorsCanRequestExit()"
    )]
    pub struct OnlyValidGeneratorsCanRequestExit;
    ///Custom Error type `OnlyWorkingGenerators` with signature `OnlyWorkingGenerators()` and selector `0x703b08e4`
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
    #[etherror(name = "OnlyWorkingGenerators", abi = "OnlyWorkingGenerators()")]
    pub struct OnlyWorkingGenerators;
    ///Custom Error type `PublicMarketsDontNeedKey` with signature `PublicMarketsDontNeedKey()` and selector `0x86922e2c`
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
    #[etherror(name = "PublicMarketsDontNeedKey", abi = "PublicMarketsDontNeedKey()")]
    pub struct PublicMarketsDontNeedKey;
    ///Custom Error type `ReduceComputeRequestNotInPlace` with signature `ReduceComputeRequestNotInPlace()` and selector `0x8983609d`
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
        name = "ReduceComputeRequestNotInPlace",
        abi = "ReduceComputeRequestNotInPlace()"
    )]
    pub struct ReduceComputeRequestNotInPlace;
    ///Custom Error type `ReductionRequestNotValid` with signature `ReductionRequestNotValid()` and selector `0x386cf407`
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
    #[etherror(name = "ReductionRequestNotValid", abi = "ReductionRequestNotValid()")]
    pub struct ReductionRequestNotValid;
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
    ///Custom Error type `RequestAlreadyInPlace` with signature `RequestAlreadyInPlace()` and selector `0x7ec76390`
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
    #[etherror(name = "RequestAlreadyInPlace", abi = "RequestAlreadyInPlace()")]
    pub struct RequestAlreadyInPlace;
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
    pub enum GeneratorRegistryErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        AddressEmptyCode(AddressEmptyCode),
        AlreadyJoinedMarket(AlreadyJoinedMarket),
        AssignOnlyToIdleGenerators(AssignOnlyToIdleGenerators),
        CannotBeMoreThanDeclaredCompute(CannotBeMoreThanDeclaredCompute),
        CannotBeSlashed(CannotBeSlashed),
        CannotBeZero(CannotBeZero),
        CannotLeaveMarketWithActiveRequest(CannotLeaveMarketWithActiveRequest),
        CannotLeaveWithActiveMarket(CannotLeaveWithActiveMarket),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        ExceedsAcceptableRange(ExceedsAcceptableRange),
        FailedCall(FailedCall),
        GeneratorAlreadyExists(GeneratorAlreadyExists),
        IncorrectImageId(IncorrectImageId),
        InsufficientGeneratorComputeAvailable(InsufficientGeneratorComputeAvailable),
        InvalidContractAddress(InvalidContractAddress),
        InvalidEnclaveKey(InvalidEnclaveKey),
        InvalidEnclaveSignature(InvalidEnclaveSignature),
        InvalidGenerator(InvalidGenerator),
        InvalidGeneratorStatePerMarket(InvalidGeneratorStatePerMarket),
        InvalidInitialization(InvalidInitialization),
        InvalidMarket(InvalidMarket),
        MaxParallelRequestsPerMarketExceeded(MaxParallelRequestsPerMarketExceeded),
        NotInitializing(NotInitializing),
        OnlyValidGeneratorsCanRequestExit(OnlyValidGeneratorsCanRequestExit),
        OnlyWorkingGenerators(OnlyWorkingGenerators),
        PublicMarketsDontNeedKey(PublicMarketsDontNeedKey),
        ReduceComputeRequestNotInPlace(ReduceComputeRequestNotInPlace),
        ReductionRequestNotValid(ReductionRequestNotValid),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        RequestAlreadyInPlace(RequestAlreadyInPlace),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GeneratorRegistryErrors {
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
                <AlreadyJoinedMarket as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AlreadyJoinedMarket(decoded));
            }
            if let Ok(decoded) =
                <AssignOnlyToIdleGenerators as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssignOnlyToIdleGenerators(decoded));
            }
            if let Ok(decoded) =
                <CannotBeMoreThanDeclaredCompute as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotBeMoreThanDeclaredCompute(decoded));
            }
            if let Ok(decoded) = <CannotBeSlashed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CannotBeSlashed(decoded));
            }
            if let Ok(decoded) = <CannotBeZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CannotBeZero(decoded));
            }
            if let Ok(decoded) =
                <CannotLeaveMarketWithActiveRequest as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotLeaveMarketWithActiveRequest(decoded));
            }
            if let Ok(decoded) =
                <CannotLeaveWithActiveMarket as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotLeaveWithActiveMarket(decoded));
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
            if let Ok(decoded) =
                <ExceedsAcceptableRange as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExceedsAcceptableRange(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedCall(decoded));
            }
            if let Ok(decoded) =
                <GeneratorAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GeneratorAlreadyExists(decoded));
            }
            if let Ok(decoded) = <IncorrectImageId as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncorrectImageId(decoded));
            }
            if let Ok(decoded) =
                <InsufficientGeneratorComputeAvailable as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::InsufficientGeneratorComputeAvailable(decoded));
            }
            if let Ok(decoded) =
                <InvalidContractAddress as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidContractAddress(decoded));
            }
            if let Ok(decoded) = <InvalidEnclaveKey as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidEnclaveKey(decoded));
            }
            if let Ok(decoded) =
                <InvalidEnclaveSignature as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidEnclaveSignature(decoded));
            }
            if let Ok(decoded) = <InvalidGenerator as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidGenerator(decoded));
            }
            if let Ok(decoded) =
                <InvalidGeneratorStatePerMarket as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidGeneratorStatePerMarket(decoded));
            }
            if let Ok(decoded) =
                <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <InvalidMarket as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidMarket(decoded));
            }
            if let Ok(decoded) =
                <MaxParallelRequestsPerMarketExceeded as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::MaxParallelRequestsPerMarketExceeded(decoded));
            }
            if let Ok(decoded) = <NotInitializing as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotInitializing(decoded));
            }
            if let Ok(decoded) =
                <OnlyValidGeneratorsCanRequestExit as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyValidGeneratorsCanRequestExit(decoded));
            }
            if let Ok(decoded) =
                <OnlyWorkingGenerators as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyWorkingGenerators(decoded));
            }
            if let Ok(decoded) =
                <PublicMarketsDontNeedKey as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PublicMarketsDontNeedKey(decoded));
            }
            if let Ok(decoded) =
                <ReduceComputeRequestNotInPlace as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReduceComputeRequestNotInPlace(decoded));
            }
            if let Ok(decoded) =
                <ReductionRequestNotValid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReductionRequestNotValid(decoded));
            }
            if let Ok(decoded) =
                <ReentrancyGuardReentrantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReentrancyGuardReentrantCall(decoded));
            }
            if let Ok(decoded) =
                <RequestAlreadyInPlace as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestAlreadyInPlace(decoded));
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
    impl ::ethers::core::abi::AbiEncode for GeneratorRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AlreadyJoinedMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssignOnlyToIdleGenerators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotBeMoreThanDeclaredCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotBeSlashed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CannotBeZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CannotLeaveMarketWithActiveRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotLeaveWithActiveMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::ExceedsAcceptableRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GeneratorAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectImageId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientGeneratorComputeAvailable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidContractAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEnclaveKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidEnclaveSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidGenerator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidGeneratorStatePerMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMarket(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxParallelRequestsPerMarketExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitializing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyValidGeneratorsCanRequestExit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyWorkingGenerators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PublicMarketsDontNeedKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReduceComputeRequestNotInPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReductionRequestNotValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestAlreadyInPlace(element) => {
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
    impl ::ethers::contract::ContractRevert for GeneratorRegistryErrors {
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
                    == <AlreadyJoinedMarket as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AssignOnlyToIdleGenerators as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotBeMoreThanDeclaredCompute as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotBeSlashed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotBeZero as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <CannotLeaveMarketWithActiveRequest as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotLeaveWithActiveMarket as ::ethers::contract::EthError>::selector() => {
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
                    == <ExceedsAcceptableRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedCall as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <GeneratorAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <IncorrectImageId as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientGeneratorComputeAvailable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidContractAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidEnclaveKey as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidEnclaveSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidGenerator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidGeneratorStatePerMarket as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMarket as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MaxParallelRequestsPerMarketExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitializing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyValidGeneratorsCanRequestExit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyWorkingGenerators as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PublicMarketsDontNeedKey as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReduceComputeRequestNotInPlace as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReductionRequestNotValid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReentrancyGuardReentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RequestAlreadyInPlace as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for GeneratorRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AlreadyJoinedMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignOnlyToIdleGenerators(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotBeMoreThanDeclaredCompute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotBeSlashed(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotBeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotLeaveMarketWithActiveRequest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotLeaveWithActiveMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureS(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExceedsAcceptableRange(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::GeneratorAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectImageId(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientGeneratorComputeAvailable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidContractAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEnclaveSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidGenerator(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidGeneratorStatePerMarket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInitialization(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxParallelRequestsPerMarketExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyValidGeneratorsCanRequestExit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyWorkingGenerators(element) => ::core::fmt::Display::fmt(element, f),
                Self::PublicMarketsDontNeedKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReduceComputeRequestNotInPlace(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReductionRequestNotValid(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestAlreadyInPlace(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnauthorizedCallContext(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for GeneratorRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation> for GeneratorRegistryErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount> for GeneratorRegistryErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for GeneratorRegistryErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AlreadyJoinedMarket> for GeneratorRegistryErrors {
        fn from(value: AlreadyJoinedMarket) -> Self {
            Self::AlreadyJoinedMarket(value)
        }
    }
    impl ::core::convert::From<AssignOnlyToIdleGenerators> for GeneratorRegistryErrors {
        fn from(value: AssignOnlyToIdleGenerators) -> Self {
            Self::AssignOnlyToIdleGenerators(value)
        }
    }
    impl ::core::convert::From<CannotBeMoreThanDeclaredCompute> for GeneratorRegistryErrors {
        fn from(value: CannotBeMoreThanDeclaredCompute) -> Self {
            Self::CannotBeMoreThanDeclaredCompute(value)
        }
    }
    impl ::core::convert::From<CannotBeSlashed> for GeneratorRegistryErrors {
        fn from(value: CannotBeSlashed) -> Self {
            Self::CannotBeSlashed(value)
        }
    }
    impl ::core::convert::From<CannotBeZero> for GeneratorRegistryErrors {
        fn from(value: CannotBeZero) -> Self {
            Self::CannotBeZero(value)
        }
    }
    impl ::core::convert::From<CannotLeaveMarketWithActiveRequest> for GeneratorRegistryErrors {
        fn from(value: CannotLeaveMarketWithActiveRequest) -> Self {
            Self::CannotLeaveMarketWithActiveRequest(value)
        }
    }
    impl ::core::convert::From<CannotLeaveWithActiveMarket> for GeneratorRegistryErrors {
        fn from(value: CannotLeaveWithActiveMarket) -> Self {
            Self::CannotLeaveWithActiveMarket(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for GeneratorRegistryErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength> for GeneratorRegistryErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for GeneratorRegistryErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for GeneratorRegistryErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for GeneratorRegistryErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<ExceedsAcceptableRange> for GeneratorRegistryErrors {
        fn from(value: ExceedsAcceptableRange) -> Self {
            Self::ExceedsAcceptableRange(value)
        }
    }
    impl ::core::convert::From<FailedCall> for GeneratorRegistryErrors {
        fn from(value: FailedCall) -> Self {
            Self::FailedCall(value)
        }
    }
    impl ::core::convert::From<GeneratorAlreadyExists> for GeneratorRegistryErrors {
        fn from(value: GeneratorAlreadyExists) -> Self {
            Self::GeneratorAlreadyExists(value)
        }
    }
    impl ::core::convert::From<IncorrectImageId> for GeneratorRegistryErrors {
        fn from(value: IncorrectImageId) -> Self {
            Self::IncorrectImageId(value)
        }
    }
    impl ::core::convert::From<InsufficientGeneratorComputeAvailable> for GeneratorRegistryErrors {
        fn from(value: InsufficientGeneratorComputeAvailable) -> Self {
            Self::InsufficientGeneratorComputeAvailable(value)
        }
    }
    impl ::core::convert::From<InvalidContractAddress> for GeneratorRegistryErrors {
        fn from(value: InvalidContractAddress) -> Self {
            Self::InvalidContractAddress(value)
        }
    }
    impl ::core::convert::From<InvalidEnclaveKey> for GeneratorRegistryErrors {
        fn from(value: InvalidEnclaveKey) -> Self {
            Self::InvalidEnclaveKey(value)
        }
    }
    impl ::core::convert::From<InvalidEnclaveSignature> for GeneratorRegistryErrors {
        fn from(value: InvalidEnclaveSignature) -> Self {
            Self::InvalidEnclaveSignature(value)
        }
    }
    impl ::core::convert::From<InvalidGenerator> for GeneratorRegistryErrors {
        fn from(value: InvalidGenerator) -> Self {
            Self::InvalidGenerator(value)
        }
    }
    impl ::core::convert::From<InvalidGeneratorStatePerMarket> for GeneratorRegistryErrors {
        fn from(value: InvalidGeneratorStatePerMarket) -> Self {
            Self::InvalidGeneratorStatePerMarket(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for GeneratorRegistryErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<InvalidMarket> for GeneratorRegistryErrors {
        fn from(value: InvalidMarket) -> Self {
            Self::InvalidMarket(value)
        }
    }
    impl ::core::convert::From<MaxParallelRequestsPerMarketExceeded> for GeneratorRegistryErrors {
        fn from(value: MaxParallelRequestsPerMarketExceeded) -> Self {
            Self::MaxParallelRequestsPerMarketExceeded(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for GeneratorRegistryErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OnlyValidGeneratorsCanRequestExit> for GeneratorRegistryErrors {
        fn from(value: OnlyValidGeneratorsCanRequestExit) -> Self {
            Self::OnlyValidGeneratorsCanRequestExit(value)
        }
    }
    impl ::core::convert::From<OnlyWorkingGenerators> for GeneratorRegistryErrors {
        fn from(value: OnlyWorkingGenerators) -> Self {
            Self::OnlyWorkingGenerators(value)
        }
    }
    impl ::core::convert::From<PublicMarketsDontNeedKey> for GeneratorRegistryErrors {
        fn from(value: PublicMarketsDontNeedKey) -> Self {
            Self::PublicMarketsDontNeedKey(value)
        }
    }
    impl ::core::convert::From<ReduceComputeRequestNotInPlace> for GeneratorRegistryErrors {
        fn from(value: ReduceComputeRequestNotInPlace) -> Self {
            Self::ReduceComputeRequestNotInPlace(value)
        }
    }
    impl ::core::convert::From<ReductionRequestNotValid> for GeneratorRegistryErrors {
        fn from(value: ReductionRequestNotValid) -> Self {
            Self::ReductionRequestNotValid(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for GeneratorRegistryErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<RequestAlreadyInPlace> for GeneratorRegistryErrors {
        fn from(value: RequestAlreadyInPlace) -> Self {
            Self::RequestAlreadyInPlace(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for GeneratorRegistryErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for GeneratorRegistryErrors {
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
    #[ethevent(name = "AddIvsKey", abi = "AddIvsKey(uint256,address)")]
    pub struct AddIvsKeyFilter {
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub signer: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "ChangedGeneratorRewardAddress",
        abi = "ChangedGeneratorRewardAddress(address,address)"
    )]
    pub struct ChangedGeneratorRewardAddressFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub new_reward_address: ::ethers::core::types::Address,
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
        name = "ComputeLockImposed",
        abi = "ComputeLockImposed(address,uint256)"
    )]
    pub struct ComputeLockImposedFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub compute: ::ethers::core::types::U256,
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
        name = "ComputeLockReleased",
        abi = "ComputeLockReleased(address,uint256)"
    )]
    pub struct ComputeLockReleasedFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub compute: ::ethers::core::types::U256,
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
    #[ethevent(name = "DecreaseCompute", abi = "DecreaseCompute(address,uint256)")]
    pub struct DecreaseComputeFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub compute: ::ethers::core::types::U256,
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
    #[ethevent(name = "DeregisteredGenerator", abi = "DeregisteredGenerator(address)")]
    pub struct DeregisteredGeneratorFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
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
    #[ethevent(name = "IncreasedCompute", abi = "IncreasedCompute(address,uint256)")]
    pub struct IncreasedComputeFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub compute: ::ethers::core::types::U256,
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
        name = "JoinedMarketplace",
        abi = "JoinedMarketplace(address,uint256,uint256)"
    )]
    pub struct JoinedMarketplaceFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
        pub compute_allocation: ::ethers::core::types::U256,
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
    #[ethevent(name = "LeftMarketplace", abi = "LeftMarketplace(address,uint256)")]
    pub struct LeftMarketplaceFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
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
        name = "RegisteredGenerator",
        abi = "RegisteredGenerator(address,uint256)"
    )]
    pub struct RegisteredGeneratorFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub initial_compute: ::ethers::core::types::U256,
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
        name = "RequestComputeDecrease",
        abi = "RequestComputeDecrease(address,uint256)"
    )]
    pub struct RequestComputeDecreaseFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub intended_utilization: ::ethers::core::types::U256,
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
        name = "RequestExitMarketplace",
        abi = "RequestExitMarketplace(address,uint256)"
    )]
    pub struct RequestExitMarketplaceFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
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
    pub enum GeneratorRegistryEvents {
        AddIvsKeyFilter(AddIvsKeyFilter),
        AddedStakeFilter(AddedStakeFilter),
        ChangedGeneratorRewardAddressFilter(ChangedGeneratorRewardAddressFilter),
        ComputeLockImposedFilter(ComputeLockImposedFilter),
        ComputeLockReleasedFilter(ComputeLockReleasedFilter),
        DecreaseComputeFilter(DecreaseComputeFilter),
        DeregisteredGeneratorFilter(DeregisteredGeneratorFilter),
        IncreasedComputeFilter(IncreasedComputeFilter),
        InitializedFilter(InitializedFilter),
        JoinedMarketplaceFilter(JoinedMarketplaceFilter),
        LeftMarketplaceFilter(LeftMarketplaceFilter),
        RegisteredGeneratorFilter(RegisteredGeneratorFilter),
        RemovedStakeFilter(RemovedStakeFilter),
        RequestComputeDecreaseFilter(RequestComputeDecreaseFilter),
        RequestExitMarketplaceFilter(RequestExitMarketplaceFilter),
        RequestStakeDecreaseFilter(RequestStakeDecreaseFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        StakeLockImposedFilter(StakeLockImposedFilter),
        StakeLockReleasedFilter(StakeLockReleasedFilter),
        StakeSlashedFilter(StakeSlashedFilter),
        SymbioticCompleteSnapshotFilter(SymbioticCompleteSnapshotFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for GeneratorRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddIvsKeyFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::AddIvsKeyFilter(decoded));
            }
            if let Ok(decoded) = AddedStakeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::AddedStakeFilter(decoded));
            }
            if let Ok(decoded) = ChangedGeneratorRewardAddressFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::ChangedGeneratorRewardAddressFilter(decoded));
            }
            if let Ok(decoded) = ComputeLockImposedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::ComputeLockImposedFilter(decoded));
            }
            if let Ok(decoded) = ComputeLockReleasedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::ComputeLockReleasedFilter(decoded));
            }
            if let Ok(decoded) = DecreaseComputeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::DecreaseComputeFilter(decoded));
            }
            if let Ok(decoded) = DeregisteredGeneratorFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::DeregisteredGeneratorFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = IncreasedComputeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::IncreasedComputeFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = JoinedMarketplaceFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::JoinedMarketplaceFilter(decoded));
            }
            if let Ok(decoded) = LeftMarketplaceFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::LeftMarketplaceFilter(decoded));
            }
            if let Ok(decoded) = RegisteredGeneratorFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RegisteredGeneratorFilter(decoded));
            }
            if let Ok(decoded) = RemovedStakeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RemovedStakeFilter(decoded));
            }
            if let Ok(decoded) = RequestComputeDecreaseFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RequestComputeDecreaseFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RequestExitMarketplaceFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RequestExitMarketplaceFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RequestStakeDecreaseFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RequestStakeDecreaseFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = StakeLockImposedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::StakeLockImposedFilter(decoded));
            }
            if let Ok(decoded) = StakeLockReleasedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::StakeLockReleasedFilter(decoded));
            }
            if let Ok(decoded) = StakeSlashedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::StakeSlashedFilter(decoded));
            }
            if let Ok(decoded) = SymbioticCompleteSnapshotFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::SymbioticCompleteSnapshotFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GeneratorRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddIvsKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddedStakeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangedGeneratorRewardAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeLockImposedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeLockReleasedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseComputeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisteredGeneratorFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreasedComputeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::JoinedMarketplaceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LeftMarketplaceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisteredGeneratorFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovedStakeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestComputeDecreaseFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestExitMarketplaceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestStakeDecreaseFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockImposedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockReleasedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeSlashedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbioticCompleteSnapshotFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddIvsKeyFilter> for GeneratorRegistryEvents {
        fn from(value: AddIvsKeyFilter) -> Self {
            Self::AddIvsKeyFilter(value)
        }
    }
    impl ::core::convert::From<AddedStakeFilter> for GeneratorRegistryEvents {
        fn from(value: AddedStakeFilter) -> Self {
            Self::AddedStakeFilter(value)
        }
    }
    impl ::core::convert::From<ChangedGeneratorRewardAddressFilter> for GeneratorRegistryEvents {
        fn from(value: ChangedGeneratorRewardAddressFilter) -> Self {
            Self::ChangedGeneratorRewardAddressFilter(value)
        }
    }
    impl ::core::convert::From<ComputeLockImposedFilter> for GeneratorRegistryEvents {
        fn from(value: ComputeLockImposedFilter) -> Self {
            Self::ComputeLockImposedFilter(value)
        }
    }
    impl ::core::convert::From<ComputeLockReleasedFilter> for GeneratorRegistryEvents {
        fn from(value: ComputeLockReleasedFilter) -> Self {
            Self::ComputeLockReleasedFilter(value)
        }
    }
    impl ::core::convert::From<DecreaseComputeFilter> for GeneratorRegistryEvents {
        fn from(value: DecreaseComputeFilter) -> Self {
            Self::DecreaseComputeFilter(value)
        }
    }
    impl ::core::convert::From<DeregisteredGeneratorFilter> for GeneratorRegistryEvents {
        fn from(value: DeregisteredGeneratorFilter) -> Self {
            Self::DeregisteredGeneratorFilter(value)
        }
    }
    impl ::core::convert::From<IncreasedComputeFilter> for GeneratorRegistryEvents {
        fn from(value: IncreasedComputeFilter) -> Self {
            Self::IncreasedComputeFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for GeneratorRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<JoinedMarketplaceFilter> for GeneratorRegistryEvents {
        fn from(value: JoinedMarketplaceFilter) -> Self {
            Self::JoinedMarketplaceFilter(value)
        }
    }
    impl ::core::convert::From<LeftMarketplaceFilter> for GeneratorRegistryEvents {
        fn from(value: LeftMarketplaceFilter) -> Self {
            Self::LeftMarketplaceFilter(value)
        }
    }
    impl ::core::convert::From<RegisteredGeneratorFilter> for GeneratorRegistryEvents {
        fn from(value: RegisteredGeneratorFilter) -> Self {
            Self::RegisteredGeneratorFilter(value)
        }
    }
    impl ::core::convert::From<RemovedStakeFilter> for GeneratorRegistryEvents {
        fn from(value: RemovedStakeFilter) -> Self {
            Self::RemovedStakeFilter(value)
        }
    }
    impl ::core::convert::From<RequestComputeDecreaseFilter> for GeneratorRegistryEvents {
        fn from(value: RequestComputeDecreaseFilter) -> Self {
            Self::RequestComputeDecreaseFilter(value)
        }
    }
    impl ::core::convert::From<RequestExitMarketplaceFilter> for GeneratorRegistryEvents {
        fn from(value: RequestExitMarketplaceFilter) -> Self {
            Self::RequestExitMarketplaceFilter(value)
        }
    }
    impl ::core::convert::From<RequestStakeDecreaseFilter> for GeneratorRegistryEvents {
        fn from(value: RequestStakeDecreaseFilter) -> Self {
            Self::RequestStakeDecreaseFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for GeneratorRegistryEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for GeneratorRegistryEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for GeneratorRegistryEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<StakeLockImposedFilter> for GeneratorRegistryEvents {
        fn from(value: StakeLockImposedFilter) -> Self {
            Self::StakeLockImposedFilter(value)
        }
    }
    impl ::core::convert::From<StakeLockReleasedFilter> for GeneratorRegistryEvents {
        fn from(value: StakeLockReleasedFilter) -> Self {
            Self::StakeLockReleasedFilter(value)
        }
    }
    impl ::core::convert::From<StakeSlashedFilter> for GeneratorRegistryEvents {
        fn from(value: StakeSlashedFilter) -> Self {
            Self::StakeSlashedFilter(value)
        }
    }
    impl ::core::convert::From<SymbioticCompleteSnapshotFilter> for GeneratorRegistryEvents {
        fn from(value: SymbioticCompleteSnapshotFilter) -> Self {
            Self::SymbioticCompleteSnapshotFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for GeneratorRegistryEvents {
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
    ///Container type for all input parameters for the `ENTITY_KEY_REGISTRY` function with signature `ENTITY_KEY_REGISTRY()` and selector `0x661de5ac`
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
    #[ethcall(name = "ENTITY_KEY_REGISTRY", abi = "ENTITY_KEY_REGISTRY()")]
    pub struct EntityKeyRegistryCall;
    ///Container type for all input parameters for the `PARALLEL_REQUESTS_UPPER_LIMIT` function with signature `PARALLEL_REQUESTS_UPPER_LIMIT()` and selector `0x7a14c463`
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
        name = "PARALLEL_REQUESTS_UPPER_LIMIT",
        abi = "PARALLEL_REQUESTS_UPPER_LIMIT()"
    )]
    pub struct ParallelRequestsUpperLimitCall;
    ///Container type for all input parameters for the `PROOF_MARKET_PLACE_ROLE` function with signature `PROOF_MARKET_PLACE_ROLE()` and selector `0x2c1fbd03`
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
    #[ethcall(name = "PROOF_MARKET_PLACE_ROLE", abi = "PROOF_MARKET_PLACE_ROLE()")]
    pub struct ProofMarketPlaceRoleCall;
    ///Container type for all input parameters for the `STAKING_MANAGER` function with signature `STAKING_MANAGER()` and selector `0x864a8efa`
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
    #[ethcall(name = "STAKING_MANAGER", abi = "STAKING_MANAGER()")]
    pub struct STAKING_MANAGERCall;
    ///Container type for all input parameters for the `UNLOCK_WAIT_BLOCKS` function with signature `UNLOCK_WAIT_BLOCKS()` and selector `0x3c5eb57c`
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
    #[ethcall(name = "UNLOCK_WAIT_BLOCKS", abi = "UNLOCK_WAIT_BLOCKS()")]
    pub struct UnlockWaitBlocksCall;
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
    ///Container type for all input parameters for the `addIvsKey` function with signature `addIvsKey(uint256,bytes,bytes)` and selector `0x2180de5d`
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
    #[ethcall(name = "addIvsKey", abi = "addIvsKey(uint256,bytes,bytes)")]
    pub struct AddIvsKeyCall {
        pub market_id: ::ethers::core::types::U256,
        pub attestation_data: ::ethers::core::types::Bytes,
        pub enclave_signature: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `assignGeneratorTask` function with signature `assignGeneratorTask(uint256,address,uint256)` and selector `0x2953e7b3`
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
        name = "assignGeneratorTask",
        abi = "assignGeneratorTask(uint256,address,uint256)"
    )]
    pub struct AssignGeneratorTaskCall {
        pub ask_id: ::ethers::core::types::U256,
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `changeRewardAddress` function with signature `changeRewardAddress(address)` and selector `0x4d2aab9a`
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
    #[ethcall(name = "changeRewardAddress", abi = "changeRewardAddress(address)")]
    pub struct ChangeRewardAddressCall {
        pub new_reward_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `completeGeneratorTask` function with signature `completeGeneratorTask(uint256,address,uint256,uint256)` and selector `0x2c346c0c`
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
        name = "completeGeneratorTask",
        abi = "completeGeneratorTask(uint256,address,uint256,uint256)"
    )]
    pub struct CompleteGeneratorTaskCall {
        pub ask_id: ::ethers::core::types::U256,
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
        pub stake_to_release: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decreaseDeclaredCompute` function with signature `decreaseDeclaredCompute()` and selector `0x2f8f4a3b`
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
    #[ethcall(name = "decreaseDeclaredCompute", abi = "decreaseDeclaredCompute()")]
    pub struct DecreaseDeclaredComputeCall;
    ///Container type for all input parameters for the `deregister` function with signature `deregister()` and selector `0xaff5edb1`
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
    #[ethcall(name = "deregister", abi = "deregister()")]
    pub struct DeregisterCall;
    ///Container type for all input parameters for the `generatorInfoPerMarket` function with signature `generatorInfoPerMarket(address,uint256)` and selector `0x9a7fca8e`
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
        name = "generatorInfoPerMarket",
        abi = "generatorInfoPerMarket(address,uint256)"
    )]
    pub struct GeneratorInfoPerMarketCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `generatorRegistry` function with signature `generatorRegistry(address)` and selector `0x8cfc56d8`
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
    #[ethcall(name = "generatorRegistry", abi = "generatorRegistry(address)")]
    pub struct GeneratorRegistryCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getGeneratorAssignmentDetails` function with signature `getGeneratorAssignmentDetails(address,uint256)` and selector `0x1c7eae65`
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
        name = "getGeneratorAssignmentDetails",
        abi = "getGeneratorAssignmentDetails(address,uint256)"
    )]
    pub struct GetGeneratorAssignmentDetailsCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getGeneratorRewardDetails` function with signature `getGeneratorRewardDetails(address,uint256)` and selector `0x2b610c2d`
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
        name = "getGeneratorRewardDetails",
        abi = "getGeneratorRewardDetails(address,uint256)"
    )]
    pub struct GetGeneratorRewardDetailsCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getGeneratorState` function with signature `getGeneratorState(address,uint256)` and selector `0x646d51b4`
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
    #[ethcall(name = "getGeneratorState", abi = "getGeneratorState(address,uint256)")]
    pub struct GetGeneratorStateCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `increaseDeclaredCompute` function with signature `increaseDeclaredCompute(uint256)` and selector `0x6d405877`
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
        name = "increaseDeclaredCompute",
        abi = "increaseDeclaredCompute(uint256)"
    )]
    pub struct IncreaseDeclaredComputeCall {
        pub compute_to_increase: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address)` and selector `0xc0c53b8b`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,address)")]
    pub struct InitializeCall {
        pub admin: ::ethers::core::types::Address,
        pub proof_marketplace: ::ethers::core::types::Address,
        pub staking_manager: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `intendToReduceCompute` function with signature `intendToReduceCompute(uint256)` and selector `0x96de0eef`
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
    #[ethcall(name = "intendToReduceCompute", abi = "intendToReduceCompute(uint256)")]
    pub struct IntendToReduceComputeCall {
        pub compute_to_reduce: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `joinMarketplace` function with signature `joinMarketplace(uint256,uint256,uint256,uint256,bool,bytes,bytes)` and selector `0xe2fa33ce`
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
        name = "joinMarketplace",
        abi = "joinMarketplace(uint256,uint256,uint256,uint256,bool,bytes,bytes)"
    )]
    pub struct JoinMarketplaceCall {
        pub market_id: ::ethers::core::types::U256,
        pub compute_per_request_required: ::ethers::core::types::U256,
        pub proof_generation_cost: ::ethers::core::types::U256,
        pub proposed_time: ::ethers::core::types::U256,
        pub update_market_dedicated_key: bool,
        pub attestation_data: ::ethers::core::types::Bytes,
        pub enclave_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `leaveMarketplace` function with signature `leaveMarketplace(uint256)` and selector `0x9f5db986`
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
    #[ethcall(name = "leaveMarketplace", abi = "leaveMarketplace(uint256)")]
    pub struct LeaveMarketplaceCall {
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `leaveMarketplaces` function with signature `leaveMarketplaces(uint256[])` and selector `0x08be6bad`
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
    #[ethcall(name = "leaveMarketplaces", abi = "leaveMarketplaces(uint256[])")]
    pub struct LeaveMarketplacesCall {
        pub market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
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
    ///Container type for all input parameters for the `register` function with signature `register(address,uint256,bytes)` and selector `0x95a78d49`
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
    #[ethcall(name = "register", abi = "register(address,uint256,bytes)")]
    pub struct RegisterCall {
        pub reward_address: ::ethers::core::types::Address,
        pub declared_compute: ::ethers::core::types::U256,
        pub generator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `releaseGeneratorResources` function with signature `releaseGeneratorResources(address,uint256)` and selector `0x1a1e8cf8`
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
        name = "releaseGeneratorResources",
        abi = "releaseGeneratorResources(address,uint256)"
    )]
    pub struct ReleaseGeneratorResourcesCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeEncryptionKey` function with signature `removeEncryptionKey(uint256)` and selector `0x541a8c18`
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
    #[ethcall(name = "removeEncryptionKey", abi = "removeEncryptionKey(uint256)")]
    pub struct RemoveEncryptionKeyCall {
        pub market_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `requestForExitMarketplace` function with signature `requestForExitMarketplace(uint256)` and selector `0xe7bc9600`
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
        name = "requestForExitMarketplace",
        abi = "requestForExitMarketplace(uint256)"
    )]
    pub struct RequestForExitMarketplaceCall {
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requestForExitMarketplaces` function with signature `requestForExitMarketplaces(uint256[])` and selector `0xd06e1f7b`
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
        name = "requestForExitMarketplaces",
        abi = "requestForExitMarketplaces(uint256[])"
    )]
    pub struct RequestForExitMarketplacesCall {
        pub market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
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
    pub struct stakingManagerCall;
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
    ///Container type for all input parameters for the `updateEncryptionKey` function with signature `updateEncryptionKey(uint256,bytes,bytes)` and selector `0x92eb91e2`
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
        name = "updateEncryptionKey",
        abi = "updateEncryptionKey(uint256,bytes,bytes)"
    )]
    pub struct UpdateEncryptionKeyCall {
        pub market_id: ::ethers::core::types::U256,
        pub attestation_data: ::ethers::core::types::Bytes,
        pub enclave_signature: ::ethers::core::types::Bytes,
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
    pub enum GeneratorRegistryCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        EntityKeyRegistry(EntityKeyRegistryCall),
        ParallelRequestsUpperLimit(ParallelRequestsUpperLimitCall),
        ProofMarketPlaceRole(ProofMarketPlaceRoleCall),
        STAKING_MANAGER(STAKING_MANAGERCall),
        UnlockWaitBlocks(UnlockWaitBlocksCall),
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        AddIvsKey(AddIvsKeyCall),
        AddStakeCallback(AddStakeCallbackCall),
        AssignGeneratorTask(AssignGeneratorTaskCall),
        ChangeRewardAddress(ChangeRewardAddressCall),
        CompleteGeneratorTask(CompleteGeneratorTaskCall),
        DecreaseDeclaredCompute(DecreaseDeclaredComputeCall),
        Deregister(DeregisterCall),
        GeneratorInfoPerMarket(GeneratorInfoPerMarketCall),
        GeneratorRegistry(GeneratorRegistryCall),
        GetGeneratorAssignmentDetails(GetGeneratorAssignmentDetailsCall),
        GetGeneratorRewardDetails(GetGeneratorRewardDetailsCall),
        GetGeneratorState(GetGeneratorStateCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IncreaseDeclaredCompute(IncreaseDeclaredComputeCall),
        Initialize(InitializeCall),
        IntendToReduceCompute(IntendToReduceComputeCall),
        IntendToReduceStakeCallback(IntendToReduceStakeCallbackCall),
        JoinMarketplace(JoinMarketplaceCall),
        LeaveMarketplace(LeaveMarketplaceCall),
        LeaveMarketplaces(LeaveMarketplacesCall),
        ProofMarketplace(ProofMarketplaceCall),
        ProxiableUUID(ProxiableUUIDCall),
        Register(RegisterCall),
        ReleaseGeneratorResources(ReleaseGeneratorResourcesCall),
        RemoveEncryptionKey(RemoveEncryptionKeyCall),
        RemoveStakeCallback(RemoveStakeCallbackCall),
        RenounceRole(RenounceRoleCall),
        RequestForExitMarketplace(RequestForExitMarketplaceCall),
        RequestForExitMarketplaces(RequestForExitMarketplacesCall),
        RevokeRole(RevokeRoleCall),
        StakeLockImposedCallback(StakeLockImposedCallbackCall),
        StakeLockReleasedCallback(StakeLockReleasedCallbackCall),
        StakeSlashedCallback(StakeSlashedCallbackCall),
        stakingManager(stakingManagerCall),
        SupportsInterface(SupportsInterfaceCall),
        SymbioticCompleteSnapshotCallback(SymbioticCompleteSnapshotCallbackCall),
        UpdateEncryptionKey(UpdateEncryptionKeyCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for GeneratorRegistryCalls {
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
                <EntityKeyRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EntityKeyRegistry(decoded));
            }
            if let Ok(decoded) =
                <ParallelRequestsUpperLimitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ParallelRequestsUpperLimit(decoded));
            }
            if let Ok(decoded) =
                <ProofMarketPlaceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProofMarketPlaceRole(decoded));
            }
            if let Ok(decoded) =
                <STAKING_MANAGERCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::STAKING_MANAGER(decoded));
            }
            if let Ok(decoded) =
                <UnlockWaitBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnlockWaitBlocks(decoded));
            }
            if let Ok(decoded) =
                <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) = <AddIvsKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddIvsKey(decoded));
            }
            if let Ok(decoded) =
                <AddStakeCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddStakeCallback(decoded));
            }
            if let Ok(decoded) =
                <AssignGeneratorTaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssignGeneratorTask(decoded));
            }
            if let Ok(decoded) =
                <ChangeRewardAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChangeRewardAddress(decoded));
            }
            if let Ok(decoded) =
                <CompleteGeneratorTaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CompleteGeneratorTask(decoded));
            }
            if let Ok(decoded) =
                <DecreaseDeclaredComputeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DecreaseDeclaredCompute(decoded));
            }
            if let Ok(decoded) = <DeregisterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deregister(decoded));
            }
            if let Ok(decoded) =
                <GeneratorInfoPerMarketCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GeneratorInfoPerMarket(decoded));
            }
            if let Ok(decoded) =
                <GeneratorRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GeneratorRegistry(decoded));
            }
            if let Ok(decoded) =
                <GetGeneratorAssignmentDetailsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetGeneratorAssignmentDetails(decoded));
            }
            if let Ok(decoded) =
                <GetGeneratorRewardDetailsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetGeneratorRewardDetails(decoded));
            }
            if let Ok(decoded) =
                <GetGeneratorStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetGeneratorState(decoded));
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
            if let Ok(decoded) =
                <IncreaseDeclaredComputeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncreaseDeclaredCompute(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IntendToReduceComputeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IntendToReduceCompute(decoded));
            }
            if let Ok(decoded) =
                <IntendToReduceStakeCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IntendToReduceStakeCallback(decoded));
            }
            if let Ok(decoded) =
                <JoinMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::JoinMarketplace(decoded));
            }
            if let Ok(decoded) =
                <LeaveMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LeaveMarketplace(decoded));
            }
            if let Ok(decoded) =
                <LeaveMarketplacesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LeaveMarketplaces(decoded));
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
            if let Ok(decoded) = <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) =
                <ReleaseGeneratorResourcesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReleaseGeneratorResources(decoded));
            }
            if let Ok(decoded) =
                <RemoveEncryptionKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveEncryptionKey(decoded));
            }
            if let Ok(decoded) =
                <RemoveStakeCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveStakeCallback(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RequestForExitMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestForExitMarketplace(decoded));
            }
            if let Ok(decoded) =
                <RequestForExitMarketplacesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestForExitMarketplaces(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
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
                <stakingManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::stakingManager(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <SymbioticCompleteSnapshotCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SymbioticCompleteSnapshotCallback(decoded));
            }
            if let Ok(decoded) =
                <UpdateEncryptionKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateEncryptionKey(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GeneratorRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EntityKeyRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ParallelRequestsUpperLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofMarketPlaceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::STAKING_MANAGER(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnlockWaitBlocks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddIvsKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddStakeCallback(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssignGeneratorTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeRewardAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteGeneratorTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseDeclaredCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deregister(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GeneratorInfoPerMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GeneratorRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetGeneratorAssignmentDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGeneratorRewardDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGeneratorState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRoleAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GrantRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncreaseDeclaredCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IntendToReduceCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntendToReduceStakeCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::JoinMarketplace(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LeaveMarketplace(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LeaveMarketplaces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProofMarketplace(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Register(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReleaseGeneratorResources(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveEncryptionKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveStakeCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestForExitMarketplace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestForExitMarketplaces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeLockImposedCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeLockReleasedCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeSlashedCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::stakingManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SymbioticCompleteSnapshotCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateEncryptionKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GeneratorRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::EntityKeyRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParallelRequestsUpperLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketPlaceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::STAKING_MANAGER(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnlockWaitBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeInterfaceVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddIvsKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStakeCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignGeneratorTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeRewardAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompleteGeneratorTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseDeclaredCompute(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deregister(element) => ::core::fmt::Display::fmt(element, f),
                Self::GeneratorInfoPerMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::GeneratorRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGeneratorAssignmentDetails(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetGeneratorRewardDetails(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGeneratorState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseDeclaredCompute(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntendToReduceCompute(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntendToReduceStakeCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::JoinMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::LeaveMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::LeaveMarketplaces(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseGeneratorResources(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveEncryptionKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveStakeCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestForExitMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestForExitMarketplaces(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockImposedCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockReleasedCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeSlashedCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::stakingManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbioticCompleteSnapshotCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateEncryptionKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for GeneratorRegistryCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<EntityKeyRegistryCall> for GeneratorRegistryCalls {
        fn from(value: EntityKeyRegistryCall) -> Self {
            Self::EntityKeyRegistry(value)
        }
    }
    impl ::core::convert::From<ParallelRequestsUpperLimitCall> for GeneratorRegistryCalls {
        fn from(value: ParallelRequestsUpperLimitCall) -> Self {
            Self::ParallelRequestsUpperLimit(value)
        }
    }
    impl ::core::convert::From<ProofMarketPlaceRoleCall> for GeneratorRegistryCalls {
        fn from(value: ProofMarketPlaceRoleCall) -> Self {
            Self::ProofMarketPlaceRole(value)
        }
    }
    impl ::core::convert::From<STAKING_MANAGERCall> for GeneratorRegistryCalls {
        fn from(value: STAKING_MANAGERCall) -> Self {
            Self::STAKING_MANAGER(value)
        }
    }
    impl ::core::convert::From<UnlockWaitBlocksCall> for GeneratorRegistryCalls {
        fn from(value: UnlockWaitBlocksCall) -> Self {
            Self::UnlockWaitBlocks(value)
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for GeneratorRegistryCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<AddIvsKeyCall> for GeneratorRegistryCalls {
        fn from(value: AddIvsKeyCall) -> Self {
            Self::AddIvsKey(value)
        }
    }
    impl ::core::convert::From<AddStakeCallbackCall> for GeneratorRegistryCalls {
        fn from(value: AddStakeCallbackCall) -> Self {
            Self::AddStakeCallback(value)
        }
    }
    impl ::core::convert::From<AssignGeneratorTaskCall> for GeneratorRegistryCalls {
        fn from(value: AssignGeneratorTaskCall) -> Self {
            Self::AssignGeneratorTask(value)
        }
    }
    impl ::core::convert::From<ChangeRewardAddressCall> for GeneratorRegistryCalls {
        fn from(value: ChangeRewardAddressCall) -> Self {
            Self::ChangeRewardAddress(value)
        }
    }
    impl ::core::convert::From<CompleteGeneratorTaskCall> for GeneratorRegistryCalls {
        fn from(value: CompleteGeneratorTaskCall) -> Self {
            Self::CompleteGeneratorTask(value)
        }
    }
    impl ::core::convert::From<DecreaseDeclaredComputeCall> for GeneratorRegistryCalls {
        fn from(value: DecreaseDeclaredComputeCall) -> Self {
            Self::DecreaseDeclaredCompute(value)
        }
    }
    impl ::core::convert::From<DeregisterCall> for GeneratorRegistryCalls {
        fn from(value: DeregisterCall) -> Self {
            Self::Deregister(value)
        }
    }
    impl ::core::convert::From<GeneratorInfoPerMarketCall> for GeneratorRegistryCalls {
        fn from(value: GeneratorInfoPerMarketCall) -> Self {
            Self::GeneratorInfoPerMarket(value)
        }
    }
    impl ::core::convert::From<GeneratorRegistryCall> for GeneratorRegistryCalls {
        fn from(value: GeneratorRegistryCall) -> Self {
            Self::GeneratorRegistry(value)
        }
    }
    impl ::core::convert::From<GetGeneratorAssignmentDetailsCall> for GeneratorRegistryCalls {
        fn from(value: GetGeneratorAssignmentDetailsCall) -> Self {
            Self::GetGeneratorAssignmentDetails(value)
        }
    }
    impl ::core::convert::From<GetGeneratorRewardDetailsCall> for GeneratorRegistryCalls {
        fn from(value: GetGeneratorRewardDetailsCall) -> Self {
            Self::GetGeneratorRewardDetails(value)
        }
    }
    impl ::core::convert::From<GetGeneratorStateCall> for GeneratorRegistryCalls {
        fn from(value: GetGeneratorStateCall) -> Self {
            Self::GetGeneratorState(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for GeneratorRegistryCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for GeneratorRegistryCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for GeneratorRegistryCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IncreaseDeclaredComputeCall> for GeneratorRegistryCalls {
        fn from(value: IncreaseDeclaredComputeCall) -> Self {
            Self::IncreaseDeclaredCompute(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for GeneratorRegistryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IntendToReduceComputeCall> for GeneratorRegistryCalls {
        fn from(value: IntendToReduceComputeCall) -> Self {
            Self::IntendToReduceCompute(value)
        }
    }
    impl ::core::convert::From<IntendToReduceStakeCallbackCall> for GeneratorRegistryCalls {
        fn from(value: IntendToReduceStakeCallbackCall) -> Self {
            Self::IntendToReduceStakeCallback(value)
        }
    }
    impl ::core::convert::From<JoinMarketplaceCall> for GeneratorRegistryCalls {
        fn from(value: JoinMarketplaceCall) -> Self {
            Self::JoinMarketplace(value)
        }
    }
    impl ::core::convert::From<LeaveMarketplaceCall> for GeneratorRegistryCalls {
        fn from(value: LeaveMarketplaceCall) -> Self {
            Self::LeaveMarketplace(value)
        }
    }
    impl ::core::convert::From<LeaveMarketplacesCall> for GeneratorRegistryCalls {
        fn from(value: LeaveMarketplacesCall) -> Self {
            Self::LeaveMarketplaces(value)
        }
    }
    impl ::core::convert::From<ProofMarketplaceCall> for GeneratorRegistryCalls {
        fn from(value: ProofMarketplaceCall) -> Self {
            Self::ProofMarketplace(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for GeneratorRegistryCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for GeneratorRegistryCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<ReleaseGeneratorResourcesCall> for GeneratorRegistryCalls {
        fn from(value: ReleaseGeneratorResourcesCall) -> Self {
            Self::ReleaseGeneratorResources(value)
        }
    }
    impl ::core::convert::From<RemoveEncryptionKeyCall> for GeneratorRegistryCalls {
        fn from(value: RemoveEncryptionKeyCall) -> Self {
            Self::RemoveEncryptionKey(value)
        }
    }
    impl ::core::convert::From<RemoveStakeCallbackCall> for GeneratorRegistryCalls {
        fn from(value: RemoveStakeCallbackCall) -> Self {
            Self::RemoveStakeCallback(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for GeneratorRegistryCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RequestForExitMarketplaceCall> for GeneratorRegistryCalls {
        fn from(value: RequestForExitMarketplaceCall) -> Self {
            Self::RequestForExitMarketplace(value)
        }
    }
    impl ::core::convert::From<RequestForExitMarketplacesCall> for GeneratorRegistryCalls {
        fn from(value: RequestForExitMarketplacesCall) -> Self {
            Self::RequestForExitMarketplaces(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for GeneratorRegistryCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<StakeLockImposedCallbackCall> for GeneratorRegistryCalls {
        fn from(value: StakeLockImposedCallbackCall) -> Self {
            Self::StakeLockImposedCallback(value)
        }
    }
    impl ::core::convert::From<StakeLockReleasedCallbackCall> for GeneratorRegistryCalls {
        fn from(value: StakeLockReleasedCallbackCall) -> Self {
            Self::StakeLockReleasedCallback(value)
        }
    }
    impl ::core::convert::From<StakeSlashedCallbackCall> for GeneratorRegistryCalls {
        fn from(value: StakeSlashedCallbackCall) -> Self {
            Self::StakeSlashedCallback(value)
        }
    }
    impl ::core::convert::From<stakingManagerCall> for GeneratorRegistryCalls {
        fn from(value: stakingManagerCall) -> Self {
            Self::stakingManager(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for GeneratorRegistryCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbioticCompleteSnapshotCallbackCall> for GeneratorRegistryCalls {
        fn from(value: SymbioticCompleteSnapshotCallbackCall) -> Self {
            Self::SymbioticCompleteSnapshotCallback(value)
        }
    }
    impl ::core::convert::From<UpdateEncryptionKeyCall> for GeneratorRegistryCalls {
        fn from(value: UpdateEncryptionKeyCall) -> Self {
            Self::UpdateEncryptionKey(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for GeneratorRegistryCalls {
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
    ///Container type for all return fields from the `ENTITY_KEY_REGISTRY` function with signature `ENTITY_KEY_REGISTRY()` and selector `0x661de5ac`
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
    pub struct EntityKeyRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `PARALLEL_REQUESTS_UPPER_LIMIT` function with signature `PARALLEL_REQUESTS_UPPER_LIMIT()` and selector `0x7a14c463`
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
    pub struct ParallelRequestsUpperLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PROOF_MARKET_PLACE_ROLE` function with signature `PROOF_MARKET_PLACE_ROLE()` and selector `0x2c1fbd03`
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
    pub struct ProofMarketPlaceRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKING_MANAGER` function with signature `STAKING_MANAGER()` and selector `0x864a8efa`
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
    pub struct STAKING_MANAGERReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `UNLOCK_WAIT_BLOCKS` function with signature `UNLOCK_WAIT_BLOCKS()` and selector `0x3c5eb57c`
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
    pub struct UnlockWaitBlocksReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `generatorInfoPerMarket` function with signature `generatorInfoPerMarket(address,uint256)` and selector `0x9a7fca8e`
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
    pub struct GeneratorInfoPerMarketReturn {
        pub state: u8,
        pub compute_per_request_required: ::ethers::core::types::U256,
        pub proof_generation_cost: ::ethers::core::types::U256,
        pub proposed_time: ::ethers::core::types::U256,
        pub active_requests: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `generatorRegistry` function with signature `generatorRegistry(address)` and selector `0x8cfc56d8`
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
    pub struct GeneratorRegistryReturn {
        pub reward_address: ::ethers::core::types::Address,
        pub sum_of_compute_allocations: ::ethers::core::types::U256,
        pub compute_consumed: ::ethers::core::types::U256,
        pub active_marketplaces: ::ethers::core::types::U256,
        pub declared_compute: ::ethers::core::types::U256,
        pub intended_compute_utilization: ::ethers::core::types::U256,
        pub generator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getGeneratorAssignmentDetails` function with signature `getGeneratorAssignmentDetails(address,uint256)` and selector `0x1c7eae65`
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
    pub struct GetGeneratorAssignmentDetailsReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getGeneratorRewardDetails` function with signature `getGeneratorRewardDetails(address,uint256)` and selector `0x2b610c2d`
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
    pub struct GetGeneratorRewardDetailsReturn(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getGeneratorState` function with signature `getGeneratorState(address,uint256)` and selector `0x646d51b4`
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
    pub struct GetGeneratorStateReturn(pub u8, pub ::ethers::core::types::U256);
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
    pub struct stakingManagerReturn(pub ::ethers::core::types::Address);
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
}
