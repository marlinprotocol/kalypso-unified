pub use error::*;
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
pub mod error {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AlreadyABlacklistedImage"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AlreadyABlacklistedImage",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("imageId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("ArityMismatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ArityMismatch"),
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
                    ::std::borrow::ToOwned::to_owned("AttestationTimeout"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AttestationTimeout"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BlacklistedImage"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BlacklistedImage"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("imageId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotAssignExpiredTasks"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CannotAssignExpiredTasks",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotBeAdminLess"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CannotBeAdminLess"),
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
                    ::std::borrow::ToOwned::to_owned("CannotModifyImagesForPublicMarkets"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "CannotModifyImagesForPublicMarkets",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotRemoveDefaultImageFromMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "CannotRemoveDefaultImageFromMarket",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotSlashUsingValidInputs"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CannotSlashUsingValidInputs",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotUseMatchingEngineRole"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CannotUseMatchingEngineRole",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveKeyNotVerified"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveKeyNotVerified",),
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
                    ::std::borrow::ToOwned::to_owned("FailedAddingToFamily"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedAddingToFamily",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("familyId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedWhitelistingImages"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedWhitelistingImages",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("imageId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("InactiveMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InactiveMarket"),
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
                    ::std::borrow::ToOwned::to_owned("InferredImageIdIsDifferent"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InferredImageIdIsDifferent",),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientStakeToLock"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientStakeToLock",),
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
                    ::std::borrow::ToOwned::to_owned("InvalidECIESACL"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidECIESACL"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidInputs"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidInputs"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidProof"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidProof"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("KeyAlreadyExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("KeyAlreadyExists"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_address"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MarketAlreadyExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MarketAlreadyExists",),
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
                    ::std::borrow::ToOwned::to_owned("MustBeAnEnclave"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MustBeAnEnclave"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("imageId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyAdminCanCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyAdminCanCall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyAssignedAsksCanBeProved"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyAssignedAsksCanBeProved",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyExpiredAsksCanBeCancelled"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyExpiredAsksCanBeCancelled",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyGeneratorCanDiscardRequest"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyGeneratorCanDiscardRequest",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyMarketCreator"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyMarketCreator"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyMatchingEngineCanAssign"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyMatchingEngineCanAssign",),
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
                    ::std::borrow::ToOwned::to_owned("ProofPriceMismatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ProofPriceMismatch"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProofTimeMismatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ProofTimeMismatch"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("RequestAlreadyInPlace"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RequestAlreadyInPlace",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ShouldBeInAssignedState"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ShouldBeInAssignedState",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ShouldBeInCreateState"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ShouldBeInCreateState",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ShouldBeInCrossedDeadlineState"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ShouldBeInCrossedDeadlineState",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnstakeRequestNotInPlace"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UnstakeRequestNotInPlace",),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ERROR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`V`7`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`*WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 ^\x93\xC1q1d`\x1D^Bo\xD8\xC3\xF0\xC7\x19\xCA\x93\x82\x0C\xDC\xC2\xC4\x8A\x03\x11\xB1\x80\xB9\x9Cm\x14dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ERROR_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 ^\x93\xC1q1d`\x1D^Bo\xD8\xC3\xF0\xC7\x19\xCA\x93\x82\x0C\xDC\xC2\xC4\x8A\x03\x11\xB1\x80\xB9\x9Cm\x14dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ERROR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Error<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Error<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Error<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Error<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Error<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Error))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Error<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ERROR_ABI.clone(),
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
                ERROR_ABI.clone(),
                ERROR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Error<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadyABlacklistedImage` with signature `AlreadyABlacklistedImage(bytes32)` and selector `0x345ae73a`
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
        name = "AlreadyABlacklistedImage",
        abi = "AlreadyABlacklistedImage(bytes32)"
    )]
    pub struct AlreadyABlacklistedImage {
        pub image_id: [u8; 32],
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
    ///Custom Error type `ArityMismatch` with signature `ArityMismatch()` and selector `0xc21fe6bf`
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
    #[etherror(name = "ArityMismatch", abi = "ArityMismatch()")]
    pub struct ArityMismatch;
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
    ///Custom Error type `AttestationTimeout` with signature `AttestationTimeout()` and selector `0x4d59de39`
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
    #[etherror(name = "AttestationTimeout", abi = "AttestationTimeout()")]
    pub struct AttestationTimeout;
    ///Custom Error type `BlacklistedImage` with signature `BlacklistedImage(bytes32)` and selector `0x89b7628c`
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
    #[etherror(name = "BlacklistedImage", abi = "BlacklistedImage(bytes32)")]
    pub struct BlacklistedImage {
        pub image_id: [u8; 32],
    }
    ///Custom Error type `CannotAssignExpiredTasks` with signature `CannotAssignExpiredTasks()` and selector `0xe832909a`
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
    #[etherror(name = "CannotAssignExpiredTasks", abi = "CannotAssignExpiredTasks()")]
    pub struct CannotAssignExpiredTasks;
    ///Custom Error type `CannotBeAdminLess` with signature `CannotBeAdminLess()` and selector `0xb42456a9`
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
    #[etherror(name = "CannotBeAdminLess", abi = "CannotBeAdminLess()")]
    pub struct CannotBeAdminLess;
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
    ///Custom Error type `CannotModifyImagesForPublicMarkets` with signature `CannotModifyImagesForPublicMarkets()` and selector `0x7afcef7f`
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
        name = "CannotModifyImagesForPublicMarkets",
        abi = "CannotModifyImagesForPublicMarkets()"
    )]
    pub struct CannotModifyImagesForPublicMarkets;
    ///Custom Error type `CannotRemoveDefaultImageFromMarket` with signature `CannotRemoveDefaultImageFromMarket(uint256,bytes32)` and selector `0xb565f792`
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
        name = "CannotRemoveDefaultImageFromMarket",
        abi = "CannotRemoveDefaultImageFromMarket(uint256,bytes32)"
    )]
    pub struct CannotRemoveDefaultImageFromMarket {
        pub market_id: ::ethers::core::types::U256,
        pub image_id: [u8; 32],
    }
    ///Custom Error type `CannotSlashUsingValidInputs` with signature `CannotSlashUsingValidInputs(uint256)` and selector `0x80d35e7a`
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
        name = "CannotSlashUsingValidInputs",
        abi = "CannotSlashUsingValidInputs(uint256)"
    )]
    pub struct CannotSlashUsingValidInputs {
        pub ask_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `CannotUseMatchingEngineRole` with signature `CannotUseMatchingEngineRole()` and selector `0x01948f42`
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
        name = "CannotUseMatchingEngineRole",
        abi = "CannotUseMatchingEngineRole()"
    )]
    pub struct CannotUseMatchingEngineRole;
    ///Custom Error type `EnclaveKeyNotVerified` with signature `EnclaveKeyNotVerified()` and selector `0xa135a55d`
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
    #[etherror(name = "EnclaveKeyNotVerified", abi = "EnclaveKeyNotVerified()")]
    pub struct EnclaveKeyNotVerified;
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
    ///Custom Error type `FailedAddingToFamily` with signature `FailedAddingToFamily(bytes32,bytes32)` and selector `0xcfac77c3`
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
        name = "FailedAddingToFamily",
        abi = "FailedAddingToFamily(bytes32,bytes32)"
    )]
    pub struct FailedAddingToFamily {
        pub image_id: [u8; 32],
        pub family_id: [u8; 32],
    }
    ///Custom Error type `FailedWhitelistingImages` with signature `FailedWhitelistingImages(bytes32)` and selector `0xc6a23213`
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
        name = "FailedWhitelistingImages",
        abi = "FailedWhitelistingImages(bytes32)"
    )]
    pub struct FailedWhitelistingImages {
        pub image_id: [u8; 32],
    }
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
    ///Custom Error type `InactiveMarket` with signature `InactiveMarket()` and selector `0xbd2da74c`
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
    #[etherror(name = "InactiveMarket", abi = "InactiveMarket()")]
    pub struct InactiveMarket;
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
    ///Custom Error type `InferredImageIdIsDifferent` with signature `InferredImageIdIsDifferent()` and selector `0x18b43556`
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
        name = "InferredImageIdIsDifferent",
        abi = "InferredImageIdIsDifferent()"
    )]
    pub struct InferredImageIdIsDifferent;
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
    ///Custom Error type `InsufficientStakeToLock` with signature `InsufficientStakeToLock()` and selector `0xe0631462`
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
    #[etherror(name = "InsufficientStakeToLock", abi = "InsufficientStakeToLock()")]
    pub struct InsufficientStakeToLock;
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
    ///Custom Error type `InvalidECIESACL` with signature `InvalidECIESACL()` and selector `0x338857e8`
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
    #[etherror(name = "InvalidECIESACL", abi = "InvalidECIESACL()")]
    pub struct InvalidECIESACL;
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
    ///Custom Error type `InvalidInputs` with signature `InvalidInputs()` and selector `0xf34cfab6`
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
    #[etherror(name = "InvalidInputs", abi = "InvalidInputs()")]
    pub struct InvalidInputs;
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
    ///Custom Error type `InvalidProof` with signature `InvalidProof(uint256)` and selector `0x5e3fa051`
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
    #[etherror(name = "InvalidProof", abi = "InvalidProof(uint256)")]
    pub struct InvalidProof {
        pub ask_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `KeyAlreadyExists` with signature `KeyAlreadyExists(address)` and selector `0xe0accd63`
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
    #[etherror(name = "KeyAlreadyExists", abi = "KeyAlreadyExists(address)")]
    pub struct KeyAlreadyExists {
        pub address: ::ethers::core::types::Address,
    }
    ///Custom Error type `MarketAlreadyExists` with signature `MarketAlreadyExists()` and selector `0x0313b285`
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
    #[etherror(name = "MarketAlreadyExists", abi = "MarketAlreadyExists()")]
    pub struct MarketAlreadyExists;
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
    ///Custom Error type `MustBeAnEnclave` with signature `MustBeAnEnclave(bytes32)` and selector `0x06356cb3`
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
    #[etherror(name = "MustBeAnEnclave", abi = "MustBeAnEnclave(bytes32)")]
    pub struct MustBeAnEnclave {
        pub image_id: [u8; 32],
    }
    ///Custom Error type `OnlyAdminCanCall` with signature `OnlyAdminCanCall()` and selector `0xa7f8eed6`
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
    #[etherror(name = "OnlyAdminCanCall", abi = "OnlyAdminCanCall()")]
    pub struct OnlyAdminCanCall;
    ///Custom Error type `OnlyAssignedAsksCanBeProved` with signature `OnlyAssignedAsksCanBeProved(uint256)` and selector `0x16f2d83f`
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
        name = "OnlyAssignedAsksCanBeProved",
        abi = "OnlyAssignedAsksCanBeProved(uint256)"
    )]
    pub struct OnlyAssignedAsksCanBeProved {
        pub ask_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `OnlyExpiredAsksCanBeCancelled` with signature `OnlyExpiredAsksCanBeCancelled(uint256)` and selector `0xa6d23aaa`
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
        name = "OnlyExpiredAsksCanBeCancelled",
        abi = "OnlyExpiredAsksCanBeCancelled(uint256)"
    )]
    pub struct OnlyExpiredAsksCanBeCancelled {
        pub ask_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `OnlyGeneratorCanDiscardRequest` with signature `OnlyGeneratorCanDiscardRequest(uint256)` and selector `0x86d0ee98`
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
        name = "OnlyGeneratorCanDiscardRequest",
        abi = "OnlyGeneratorCanDiscardRequest(uint256)"
    )]
    pub struct OnlyGeneratorCanDiscardRequest {
        pub ask_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `OnlyMarketCreator` with signature `OnlyMarketCreator()` and selector `0x38993a49`
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
    #[etherror(name = "OnlyMarketCreator", abi = "OnlyMarketCreator()")]
    pub struct OnlyMarketCreator;
    ///Custom Error type `OnlyMatchingEngineCanAssign` with signature `OnlyMatchingEngineCanAssign()` and selector `0x169759c1`
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
        name = "OnlyMatchingEngineCanAssign",
        abi = "OnlyMatchingEngineCanAssign()"
    )]
    pub struct OnlyMatchingEngineCanAssign;
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
    ///Custom Error type `ProofPriceMismatch` with signature `ProofPriceMismatch(uint256)` and selector `0xc4bb553a`
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
    #[etherror(name = "ProofPriceMismatch", abi = "ProofPriceMismatch(uint256)")]
    pub struct ProofPriceMismatch {
        pub ask_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `ProofTimeMismatch` with signature `ProofTimeMismatch(uint256)` and selector `0xf84faa49`
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
    #[etherror(name = "ProofTimeMismatch", abi = "ProofTimeMismatch(uint256)")]
    pub struct ProofTimeMismatch {
        pub ask_id: ::ethers::core::types::U256,
    }
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
    ///Custom Error type `ShouldBeInAssignedState` with signature `ShouldBeInAssignedState(uint256)` and selector `0x0d9485f1`
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
        name = "ShouldBeInAssignedState",
        abi = "ShouldBeInAssignedState(uint256)"
    )]
    pub struct ShouldBeInAssignedState {
        pub ask_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `ShouldBeInCreateState` with signature `ShouldBeInCreateState()` and selector `0x7cb69d0a`
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
    #[etherror(name = "ShouldBeInCreateState", abi = "ShouldBeInCreateState()")]
    pub struct ShouldBeInCreateState;
    ///Custom Error type `ShouldBeInCrossedDeadlineState` with signature `ShouldBeInCrossedDeadlineState(uint256)` and selector `0xb2cdf6a8`
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
        name = "ShouldBeInCrossedDeadlineState",
        abi = "ShouldBeInCrossedDeadlineState(uint256)"
    )]
    pub struct ShouldBeInCrossedDeadlineState {
        pub ask_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `UnstakeRequestNotInPlace` with signature `UnstakeRequestNotInPlace()` and selector `0xb28c1c0e`
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
    #[etherror(name = "UnstakeRequestNotInPlace", abi = "UnstakeRequestNotInPlace()")]
    pub struct UnstakeRequestNotInPlace;
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
    pub enum ErrorErrors {
        AlreadyABlacklistedImage(AlreadyABlacklistedImage),
        AlreadyJoinedMarket(AlreadyJoinedMarket),
        ArityMismatch(ArityMismatch),
        AssignOnlyToIdleGenerators(AssignOnlyToIdleGenerators),
        AttestationTimeout(AttestationTimeout),
        BlacklistedImage(BlacklistedImage),
        CannotAssignExpiredTasks(CannotAssignExpiredTasks),
        CannotBeAdminLess(CannotBeAdminLess),
        CannotBeMoreThanDeclaredCompute(CannotBeMoreThanDeclaredCompute),
        CannotBeSlashed(CannotBeSlashed),
        CannotBeZero(CannotBeZero),
        CannotLeaveMarketWithActiveRequest(CannotLeaveMarketWithActiveRequest),
        CannotLeaveWithActiveMarket(CannotLeaveWithActiveMarket),
        CannotModifyImagesForPublicMarkets(CannotModifyImagesForPublicMarkets),
        CannotRemoveDefaultImageFromMarket(CannotRemoveDefaultImageFromMarket),
        CannotSlashUsingValidInputs(CannotSlashUsingValidInputs),
        CannotUseMatchingEngineRole(CannotUseMatchingEngineRole),
        EnclaveKeyNotVerified(EnclaveKeyNotVerified),
        ExceedsAcceptableRange(ExceedsAcceptableRange),
        FailedAddingToFamily(FailedAddingToFamily),
        FailedWhitelistingImages(FailedWhitelistingImages),
        GeneratorAlreadyExists(GeneratorAlreadyExists),
        InactiveMarket(InactiveMarket),
        IncorrectImageId(IncorrectImageId),
        InferredImageIdIsDifferent(InferredImageIdIsDifferent),
        InsufficientGeneratorComputeAvailable(InsufficientGeneratorComputeAvailable),
        InsufficientStakeToLock(InsufficientStakeToLock),
        InvalidContractAddress(InvalidContractAddress),
        InvalidECIESACL(InvalidECIESACL),
        InvalidEnclaveKey(InvalidEnclaveKey),
        InvalidEnclaveSignature(InvalidEnclaveSignature),
        InvalidGenerator(InvalidGenerator),
        InvalidGeneratorStatePerMarket(InvalidGeneratorStatePerMarket),
        InvalidInputs(InvalidInputs),
        InvalidMarket(InvalidMarket),
        InvalidProof(InvalidProof),
        KeyAlreadyExists(KeyAlreadyExists),
        MarketAlreadyExists(MarketAlreadyExists),
        MaxParallelRequestsPerMarketExceeded(MaxParallelRequestsPerMarketExceeded),
        MustBeAnEnclave(MustBeAnEnclave),
        OnlyAdminCanCall(OnlyAdminCanCall),
        OnlyAssignedAsksCanBeProved(OnlyAssignedAsksCanBeProved),
        OnlyExpiredAsksCanBeCancelled(OnlyExpiredAsksCanBeCancelled),
        OnlyGeneratorCanDiscardRequest(OnlyGeneratorCanDiscardRequest),
        OnlyMarketCreator(OnlyMarketCreator),
        OnlyMatchingEngineCanAssign(OnlyMatchingEngineCanAssign),
        OnlyValidGeneratorsCanRequestExit(OnlyValidGeneratorsCanRequestExit),
        OnlyWorkingGenerators(OnlyWorkingGenerators),
        ProofPriceMismatch(ProofPriceMismatch),
        ProofTimeMismatch(ProofTimeMismatch),
        PublicMarketsDontNeedKey(PublicMarketsDontNeedKey),
        ReduceComputeRequestNotInPlace(ReduceComputeRequestNotInPlace),
        ReductionRequestNotValid(ReductionRequestNotValid),
        RequestAlreadyInPlace(RequestAlreadyInPlace),
        ShouldBeInAssignedState(ShouldBeInAssignedState),
        ShouldBeInCreateState(ShouldBeInCreateState),
        ShouldBeInCrossedDeadlineState(ShouldBeInCrossedDeadlineState),
        UnstakeRequestNotInPlace(UnstakeRequestNotInPlace),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ErrorErrors {
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
                <AlreadyABlacklistedImage as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AlreadyABlacklistedImage(decoded));
            }
            if let Ok(decoded) =
                <AlreadyJoinedMarket as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AlreadyJoinedMarket(decoded));
            }
            if let Ok(decoded) = <ArityMismatch as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ArityMismatch(decoded));
            }
            if let Ok(decoded) =
                <AssignOnlyToIdleGenerators as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssignOnlyToIdleGenerators(decoded));
            }
            if let Ok(decoded) =
                <AttestationTimeout as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationTimeout(decoded));
            }
            if let Ok(decoded) = <BlacklistedImage as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BlacklistedImage(decoded));
            }
            if let Ok(decoded) =
                <CannotAssignExpiredTasks as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotAssignExpiredTasks(decoded));
            }
            if let Ok(decoded) = <CannotBeAdminLess as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotBeAdminLess(decoded));
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
                <CannotModifyImagesForPublicMarkets as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotModifyImagesForPublicMarkets(decoded));
            }
            if let Ok(decoded) =
                <CannotRemoveDefaultImageFromMarket as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotRemoveDefaultImageFromMarket(decoded));
            }
            if let Ok(decoded) =
                <CannotSlashUsingValidInputs as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotSlashUsingValidInputs(decoded));
            }
            if let Ok(decoded) =
                <CannotUseMatchingEngineRole as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotUseMatchingEngineRole(decoded));
            }
            if let Ok(decoded) =
                <EnclaveKeyNotVerified as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnclaveKeyNotVerified(decoded));
            }
            if let Ok(decoded) =
                <ExceedsAcceptableRange as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExceedsAcceptableRange(decoded));
            }
            if let Ok(decoded) =
                <FailedAddingToFamily as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FailedAddingToFamily(decoded));
            }
            if let Ok(decoded) =
                <FailedWhitelistingImages as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FailedWhitelistingImages(decoded));
            }
            if let Ok(decoded) =
                <GeneratorAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GeneratorAlreadyExists(decoded));
            }
            if let Ok(decoded) = <InactiveMarket as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InactiveMarket(decoded));
            }
            if let Ok(decoded) = <IncorrectImageId as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncorrectImageId(decoded));
            }
            if let Ok(decoded) =
                <InferredImageIdIsDifferent as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InferredImageIdIsDifferent(decoded));
            }
            if let Ok(decoded) =
                <InsufficientGeneratorComputeAvailable as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::InsufficientGeneratorComputeAvailable(decoded));
            }
            if let Ok(decoded) =
                <InsufficientStakeToLock as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientStakeToLock(decoded));
            }
            if let Ok(decoded) =
                <InvalidContractAddress as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidContractAddress(decoded));
            }
            if let Ok(decoded) = <InvalidECIESACL as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidECIESACL(decoded));
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
            if let Ok(decoded) = <InvalidInputs as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidInputs(decoded));
            }
            if let Ok(decoded) = <InvalidMarket as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidMarket(decoded));
            }
            if let Ok(decoded) = <InvalidProof as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidProof(decoded));
            }
            if let Ok(decoded) = <KeyAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::KeyAlreadyExists(decoded));
            }
            if let Ok(decoded) =
                <MarketAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MarketAlreadyExists(decoded));
            }
            if let Ok(decoded) =
                <MaxParallelRequestsPerMarketExceeded as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::MaxParallelRequestsPerMarketExceeded(decoded));
            }
            if let Ok(decoded) = <MustBeAnEnclave as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MustBeAnEnclave(decoded));
            }
            if let Ok(decoded) = <OnlyAdminCanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyAdminCanCall(decoded));
            }
            if let Ok(decoded) =
                <OnlyAssignedAsksCanBeProved as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyAssignedAsksCanBeProved(decoded));
            }
            if let Ok(decoded) =
                <OnlyExpiredAsksCanBeCancelled as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyExpiredAsksCanBeCancelled(decoded));
            }
            if let Ok(decoded) =
                <OnlyGeneratorCanDiscardRequest as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyGeneratorCanDiscardRequest(decoded));
            }
            if let Ok(decoded) = <OnlyMarketCreator as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyMarketCreator(decoded));
            }
            if let Ok(decoded) =
                <OnlyMatchingEngineCanAssign as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyMatchingEngineCanAssign(decoded));
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
                <ProofPriceMismatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProofPriceMismatch(decoded));
            }
            if let Ok(decoded) = <ProofTimeMismatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProofTimeMismatch(decoded));
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
                <RequestAlreadyInPlace as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestAlreadyInPlace(decoded));
            }
            if let Ok(decoded) =
                <ShouldBeInAssignedState as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ShouldBeInAssignedState(decoded));
            }
            if let Ok(decoded) =
                <ShouldBeInCreateState as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ShouldBeInCreateState(decoded));
            }
            if let Ok(decoded) =
                <ShouldBeInCrossedDeadlineState as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ShouldBeInCrossedDeadlineState(decoded));
            }
            if let Ok(decoded) =
                <UnstakeRequestNotInPlace as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnstakeRequestNotInPlace(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ErrorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadyABlacklistedImage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AlreadyJoinedMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ArityMismatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssignOnlyToIdleGenerators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationTimeout(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlacklistedImage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CannotAssignExpiredTasks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotBeAdminLess(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::CannotModifyImagesForPublicMarkets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotRemoveDefaultImageFromMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotSlashUsingValidInputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotUseMatchingEngineRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnclaveKeyNotVerified(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExceedsAcceptableRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedAddingToFamily(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedWhitelistingImages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GeneratorAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InactiveMarket(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncorrectImageId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InferredImageIdIsDifferent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientGeneratorComputeAvailable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientStakeToLock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidContractAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidECIESACL(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidEnclaveKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidEnclaveSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidGenerator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidGeneratorStatePerMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInputs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidMarket(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidProof(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::KeyAlreadyExists(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MarketAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxParallelRequestsPerMarketExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MustBeAnEnclave(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyAdminCanCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyAssignedAsksCanBeProved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyExpiredAsksCanBeCancelled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyGeneratorCanDiscardRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyMarketCreator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyMatchingEngineCanAssign(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyValidGeneratorsCanRequestExit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyWorkingGenerators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofPriceMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofTimeMismatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PublicMarketsDontNeedKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReduceComputeRequestNotInPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReductionRequestNotValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestAlreadyInPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShouldBeInAssignedState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShouldBeInCreateState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShouldBeInCrossedDeadlineState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnstakeRequestNotInPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ErrorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AlreadyABlacklistedImage as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AlreadyJoinedMarket as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ArityMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AssignOnlyToIdleGenerators as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationTimeout as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BlacklistedImage as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotAssignExpiredTasks as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotBeAdminLess as ::ethers::contract::EthError>::selector() => {
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
                    == <CannotModifyImagesForPublicMarkets as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotRemoveDefaultImageFromMarket as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotSlashUsingValidInputs as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotUseMatchingEngineRole as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EnclaveKeyNotVerified as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExceedsAcceptableRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedAddingToFamily as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedWhitelistingImages as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <GeneratorAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InactiveMarket as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <IncorrectImageId as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InferredImageIdIsDifferent as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientGeneratorComputeAvailable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientStakeToLock as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidContractAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidECIESACL as ::ethers::contract::EthError>::selector() => {
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
                    == <InvalidInputs as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMarket as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidProof as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <KeyAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MarketAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MaxParallelRequestsPerMarketExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MustBeAnEnclave as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyAdminCanCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyAssignedAsksCanBeProved as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyExpiredAsksCanBeCancelled as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyGeneratorCanDiscardRequest as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyMarketCreator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyMatchingEngineCanAssign as ::ethers::contract::EthError>::selector() => {
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
                    == <ProofPriceMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ProofTimeMismatch as ::ethers::contract::EthError>::selector() => {
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
                    == <RequestAlreadyInPlace as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ShouldBeInAssignedState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ShouldBeInCreateState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ShouldBeInCrossedDeadlineState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnstakeRequestNotInPlace as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ErrorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadyABlacklistedImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::AlreadyJoinedMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::ArityMismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignOnlyToIdleGenerators(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationTimeout(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlacklistedImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotAssignExpiredTasks(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotBeAdminLess(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotBeMoreThanDeclaredCompute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotBeSlashed(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotBeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotLeaveMarketWithActiveRequest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotLeaveWithActiveMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotModifyImagesForPublicMarkets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotRemoveDefaultImageFromMarket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotSlashUsingValidInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotUseMatchingEngineRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveKeyNotVerified(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExceedsAcceptableRange(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedAddingToFamily(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedWhitelistingImages(element) => ::core::fmt::Display::fmt(element, f),
                Self::GeneratorAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::InactiveMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectImageId(element) => ::core::fmt::Display::fmt(element, f),
                Self::InferredImageIdIsDifferent(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientGeneratorComputeAvailable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientStakeToLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidContractAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidECIESACL(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEnclaveSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidGenerator(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidGeneratorStatePerMarket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::KeyAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxParallelRequestsPerMarketExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MustBeAnEnclave(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyAdminCanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyAssignedAsksCanBeProved(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyExpiredAsksCanBeCancelled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyGeneratorCanDiscardRequest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyMarketCreator(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyMatchingEngineCanAssign(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyValidGeneratorsCanRequestExit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyWorkingGenerators(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofPriceMismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofTimeMismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::PublicMarketsDontNeedKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReduceComputeRequestNotInPlace(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReductionRequestNotValid(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestAlreadyInPlace(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShouldBeInAssignedState(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShouldBeInCreateState(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShouldBeInCrossedDeadlineState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnstakeRequestNotInPlace(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ErrorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadyABlacklistedImage> for ErrorErrors {
        fn from(value: AlreadyABlacklistedImage) -> Self {
            Self::AlreadyABlacklistedImage(value)
        }
    }
    impl ::core::convert::From<AlreadyJoinedMarket> for ErrorErrors {
        fn from(value: AlreadyJoinedMarket) -> Self {
            Self::AlreadyJoinedMarket(value)
        }
    }
    impl ::core::convert::From<ArityMismatch> for ErrorErrors {
        fn from(value: ArityMismatch) -> Self {
            Self::ArityMismatch(value)
        }
    }
    impl ::core::convert::From<AssignOnlyToIdleGenerators> for ErrorErrors {
        fn from(value: AssignOnlyToIdleGenerators) -> Self {
            Self::AssignOnlyToIdleGenerators(value)
        }
    }
    impl ::core::convert::From<AttestationTimeout> for ErrorErrors {
        fn from(value: AttestationTimeout) -> Self {
            Self::AttestationTimeout(value)
        }
    }
    impl ::core::convert::From<BlacklistedImage> for ErrorErrors {
        fn from(value: BlacklistedImage) -> Self {
            Self::BlacklistedImage(value)
        }
    }
    impl ::core::convert::From<CannotAssignExpiredTasks> for ErrorErrors {
        fn from(value: CannotAssignExpiredTasks) -> Self {
            Self::CannotAssignExpiredTasks(value)
        }
    }
    impl ::core::convert::From<CannotBeAdminLess> for ErrorErrors {
        fn from(value: CannotBeAdminLess) -> Self {
            Self::CannotBeAdminLess(value)
        }
    }
    impl ::core::convert::From<CannotBeMoreThanDeclaredCompute> for ErrorErrors {
        fn from(value: CannotBeMoreThanDeclaredCompute) -> Self {
            Self::CannotBeMoreThanDeclaredCompute(value)
        }
    }
    impl ::core::convert::From<CannotBeSlashed> for ErrorErrors {
        fn from(value: CannotBeSlashed) -> Self {
            Self::CannotBeSlashed(value)
        }
    }
    impl ::core::convert::From<CannotBeZero> for ErrorErrors {
        fn from(value: CannotBeZero) -> Self {
            Self::CannotBeZero(value)
        }
    }
    impl ::core::convert::From<CannotLeaveMarketWithActiveRequest> for ErrorErrors {
        fn from(value: CannotLeaveMarketWithActiveRequest) -> Self {
            Self::CannotLeaveMarketWithActiveRequest(value)
        }
    }
    impl ::core::convert::From<CannotLeaveWithActiveMarket> for ErrorErrors {
        fn from(value: CannotLeaveWithActiveMarket) -> Self {
            Self::CannotLeaveWithActiveMarket(value)
        }
    }
    impl ::core::convert::From<CannotModifyImagesForPublicMarkets> for ErrorErrors {
        fn from(value: CannotModifyImagesForPublicMarkets) -> Self {
            Self::CannotModifyImagesForPublicMarkets(value)
        }
    }
    impl ::core::convert::From<CannotRemoveDefaultImageFromMarket> for ErrorErrors {
        fn from(value: CannotRemoveDefaultImageFromMarket) -> Self {
            Self::CannotRemoveDefaultImageFromMarket(value)
        }
    }
    impl ::core::convert::From<CannotSlashUsingValidInputs> for ErrorErrors {
        fn from(value: CannotSlashUsingValidInputs) -> Self {
            Self::CannotSlashUsingValidInputs(value)
        }
    }
    impl ::core::convert::From<CannotUseMatchingEngineRole> for ErrorErrors {
        fn from(value: CannotUseMatchingEngineRole) -> Self {
            Self::CannotUseMatchingEngineRole(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyNotVerified> for ErrorErrors {
        fn from(value: EnclaveKeyNotVerified) -> Self {
            Self::EnclaveKeyNotVerified(value)
        }
    }
    impl ::core::convert::From<ExceedsAcceptableRange> for ErrorErrors {
        fn from(value: ExceedsAcceptableRange) -> Self {
            Self::ExceedsAcceptableRange(value)
        }
    }
    impl ::core::convert::From<FailedAddingToFamily> for ErrorErrors {
        fn from(value: FailedAddingToFamily) -> Self {
            Self::FailedAddingToFamily(value)
        }
    }
    impl ::core::convert::From<FailedWhitelistingImages> for ErrorErrors {
        fn from(value: FailedWhitelistingImages) -> Self {
            Self::FailedWhitelistingImages(value)
        }
    }
    impl ::core::convert::From<GeneratorAlreadyExists> for ErrorErrors {
        fn from(value: GeneratorAlreadyExists) -> Self {
            Self::GeneratorAlreadyExists(value)
        }
    }
    impl ::core::convert::From<InactiveMarket> for ErrorErrors {
        fn from(value: InactiveMarket) -> Self {
            Self::InactiveMarket(value)
        }
    }
    impl ::core::convert::From<IncorrectImageId> for ErrorErrors {
        fn from(value: IncorrectImageId) -> Self {
            Self::IncorrectImageId(value)
        }
    }
    impl ::core::convert::From<InferredImageIdIsDifferent> for ErrorErrors {
        fn from(value: InferredImageIdIsDifferent) -> Self {
            Self::InferredImageIdIsDifferent(value)
        }
    }
    impl ::core::convert::From<InsufficientGeneratorComputeAvailable> for ErrorErrors {
        fn from(value: InsufficientGeneratorComputeAvailable) -> Self {
            Self::InsufficientGeneratorComputeAvailable(value)
        }
    }
    impl ::core::convert::From<InsufficientStakeToLock> for ErrorErrors {
        fn from(value: InsufficientStakeToLock) -> Self {
            Self::InsufficientStakeToLock(value)
        }
    }
    impl ::core::convert::From<InvalidContractAddress> for ErrorErrors {
        fn from(value: InvalidContractAddress) -> Self {
            Self::InvalidContractAddress(value)
        }
    }
    impl ::core::convert::From<InvalidECIESACL> for ErrorErrors {
        fn from(value: InvalidECIESACL) -> Self {
            Self::InvalidECIESACL(value)
        }
    }
    impl ::core::convert::From<InvalidEnclaveKey> for ErrorErrors {
        fn from(value: InvalidEnclaveKey) -> Self {
            Self::InvalidEnclaveKey(value)
        }
    }
    impl ::core::convert::From<InvalidEnclaveSignature> for ErrorErrors {
        fn from(value: InvalidEnclaveSignature) -> Self {
            Self::InvalidEnclaveSignature(value)
        }
    }
    impl ::core::convert::From<InvalidGenerator> for ErrorErrors {
        fn from(value: InvalidGenerator) -> Self {
            Self::InvalidGenerator(value)
        }
    }
    impl ::core::convert::From<InvalidGeneratorStatePerMarket> for ErrorErrors {
        fn from(value: InvalidGeneratorStatePerMarket) -> Self {
            Self::InvalidGeneratorStatePerMarket(value)
        }
    }
    impl ::core::convert::From<InvalidInputs> for ErrorErrors {
        fn from(value: InvalidInputs) -> Self {
            Self::InvalidInputs(value)
        }
    }
    impl ::core::convert::From<InvalidMarket> for ErrorErrors {
        fn from(value: InvalidMarket) -> Self {
            Self::InvalidMarket(value)
        }
    }
    impl ::core::convert::From<InvalidProof> for ErrorErrors {
        fn from(value: InvalidProof) -> Self {
            Self::InvalidProof(value)
        }
    }
    impl ::core::convert::From<KeyAlreadyExists> for ErrorErrors {
        fn from(value: KeyAlreadyExists) -> Self {
            Self::KeyAlreadyExists(value)
        }
    }
    impl ::core::convert::From<MarketAlreadyExists> for ErrorErrors {
        fn from(value: MarketAlreadyExists) -> Self {
            Self::MarketAlreadyExists(value)
        }
    }
    impl ::core::convert::From<MaxParallelRequestsPerMarketExceeded> for ErrorErrors {
        fn from(value: MaxParallelRequestsPerMarketExceeded) -> Self {
            Self::MaxParallelRequestsPerMarketExceeded(value)
        }
    }
    impl ::core::convert::From<MustBeAnEnclave> for ErrorErrors {
        fn from(value: MustBeAnEnclave) -> Self {
            Self::MustBeAnEnclave(value)
        }
    }
    impl ::core::convert::From<OnlyAdminCanCall> for ErrorErrors {
        fn from(value: OnlyAdminCanCall) -> Self {
            Self::OnlyAdminCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyAssignedAsksCanBeProved> for ErrorErrors {
        fn from(value: OnlyAssignedAsksCanBeProved) -> Self {
            Self::OnlyAssignedAsksCanBeProved(value)
        }
    }
    impl ::core::convert::From<OnlyExpiredAsksCanBeCancelled> for ErrorErrors {
        fn from(value: OnlyExpiredAsksCanBeCancelled) -> Self {
            Self::OnlyExpiredAsksCanBeCancelled(value)
        }
    }
    impl ::core::convert::From<OnlyGeneratorCanDiscardRequest> for ErrorErrors {
        fn from(value: OnlyGeneratorCanDiscardRequest) -> Self {
            Self::OnlyGeneratorCanDiscardRequest(value)
        }
    }
    impl ::core::convert::From<OnlyMarketCreator> for ErrorErrors {
        fn from(value: OnlyMarketCreator) -> Self {
            Self::OnlyMarketCreator(value)
        }
    }
    impl ::core::convert::From<OnlyMatchingEngineCanAssign> for ErrorErrors {
        fn from(value: OnlyMatchingEngineCanAssign) -> Self {
            Self::OnlyMatchingEngineCanAssign(value)
        }
    }
    impl ::core::convert::From<OnlyValidGeneratorsCanRequestExit> for ErrorErrors {
        fn from(value: OnlyValidGeneratorsCanRequestExit) -> Self {
            Self::OnlyValidGeneratorsCanRequestExit(value)
        }
    }
    impl ::core::convert::From<OnlyWorkingGenerators> for ErrorErrors {
        fn from(value: OnlyWorkingGenerators) -> Self {
            Self::OnlyWorkingGenerators(value)
        }
    }
    impl ::core::convert::From<ProofPriceMismatch> for ErrorErrors {
        fn from(value: ProofPriceMismatch) -> Self {
            Self::ProofPriceMismatch(value)
        }
    }
    impl ::core::convert::From<ProofTimeMismatch> for ErrorErrors {
        fn from(value: ProofTimeMismatch) -> Self {
            Self::ProofTimeMismatch(value)
        }
    }
    impl ::core::convert::From<PublicMarketsDontNeedKey> for ErrorErrors {
        fn from(value: PublicMarketsDontNeedKey) -> Self {
            Self::PublicMarketsDontNeedKey(value)
        }
    }
    impl ::core::convert::From<ReduceComputeRequestNotInPlace> for ErrorErrors {
        fn from(value: ReduceComputeRequestNotInPlace) -> Self {
            Self::ReduceComputeRequestNotInPlace(value)
        }
    }
    impl ::core::convert::From<ReductionRequestNotValid> for ErrorErrors {
        fn from(value: ReductionRequestNotValid) -> Self {
            Self::ReductionRequestNotValid(value)
        }
    }
    impl ::core::convert::From<RequestAlreadyInPlace> for ErrorErrors {
        fn from(value: RequestAlreadyInPlace) -> Self {
            Self::RequestAlreadyInPlace(value)
        }
    }
    impl ::core::convert::From<ShouldBeInAssignedState> for ErrorErrors {
        fn from(value: ShouldBeInAssignedState) -> Self {
            Self::ShouldBeInAssignedState(value)
        }
    }
    impl ::core::convert::From<ShouldBeInCreateState> for ErrorErrors {
        fn from(value: ShouldBeInCreateState) -> Self {
            Self::ShouldBeInCreateState(value)
        }
    }
    impl ::core::convert::From<ShouldBeInCrossedDeadlineState> for ErrorErrors {
        fn from(value: ShouldBeInCrossedDeadlineState) -> Self {
            Self::ShouldBeInCrossedDeadlineState(value)
        }
    }
    impl ::core::convert::From<UnstakeRequestNotInPlace> for ErrorErrors {
        fn from(value: UnstakeRequestNotInPlace) -> Self {
            Self::UnstakeRequestNotInPlace(value)
        }
    }
}
