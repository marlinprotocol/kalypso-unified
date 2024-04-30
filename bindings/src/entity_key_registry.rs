pub use entity_key_registry::*;
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
pub mod entity_key_registry {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_av"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "contract IAttestationVerifier",
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ATTESTATION_MAX_AGE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ATTESTATION_MAX_AGE",),
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
                    ::std::borrow::ToOwned::to_owned("ATTESTATION_VERIFIER"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ATTESTATION_VERIFIER",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IAttestationVerifier",),
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
                    ::std::borrow::ToOwned::to_owned("KEY_REGISTER_ROLE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("KEY_REGISTER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("MODERATOR_ROLE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MODERATOR_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("addGeneratorRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addGeneratorRegistry",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_generatorRegistry",),
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
                    ::std::borrow::ToOwned::to_owned("allowOnlyVerifiedFamily"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allowOnlyVerifiedFamily",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("familyId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("blackListedImages"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("blackListedImages"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("blacklistImage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("blacklistImage"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("imageId"),
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
                    ::std::borrow::ToOwned::to_owned("getVerifiedKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getVerifiedKey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_key"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("getWhitelistedImage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getWhitelistedImage",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_imageId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct AttestationAutherUpgradeable.EnclaveImage",
                                ),
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
                                name: ::std::borrow::ToOwned::to_owned("initWhitelistImages",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct AttestationAutherUpgradeable.EnclaveImage[]",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isImageInFamily"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isImageInFamily"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("family"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("pub_key"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pub_key"),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeEnclaveImageFromFamily"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeEnclaveImageFromFamily",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("family"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removePubkey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removePubkey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("keyOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("keyIndex"),
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
                    ::std::borrow::ToOwned::to_owned("updatePubkey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updatePubkey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("keyOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("keyIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("attestation_data"),
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
                (
                    ::std::borrow::ToOwned::to_owned("verifyEnclaveKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifyEnclaveKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("attestation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IAttestationVerifier.Attestation",
                                    ),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifyKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifyKey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("attestation_data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("whitelistImageUsingPcrs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("whitelistImageUsingPcrs",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("family"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pcrs"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveImageAddedToFamily"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveImageAddedToFamily",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("family"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveImageRemovedFromFamily"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveImageRemovedFromFamily",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("family"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveImageRevoked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveImageRevoked",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("imageId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveImageWhitelisted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveImageWhitelisted",),
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
                    ::std::borrow::ToOwned::to_owned("EnclaveKeyRevoked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveKeyRevoked"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveKeyVerified"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveKeyVerified"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveKeyWhitelisted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveKeyWhitelisted",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ImageBlacklisted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ImageBlacklisted"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("imageId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
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
                    ::std::borrow::ToOwned::to_owned("RemoveKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RemoveKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("keyIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("UpdateKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("UpdateKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("keyIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("AttestationAutherAttestationTooOld"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationAutherAttestationTooOld",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationAutherImageNotInFamily"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AttestationAutherImageNotInFamily",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationAutherImageNotWhitelisted"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationAutherImageNotWhitelisted",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationAutherKeyNotVerified"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AttestationAutherKeyNotVerified",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationAutherMismatchedLengths"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationAutherMismatchedLengths",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationAutherPCRsInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AttestationAutherPCRsInvalid",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationAutherPubkeyLengthInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationAutherPubkeyLengthInvalid",
                        ),
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
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidEnclaveKey"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidEnclaveKey"),
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
    pub static ENTITYKEYREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R0`\x80R4\x80\x15b\0\0\x15W`\0\x80\xFD[P`@Qb\0+_8\x03\x80b\0+_\x839\x81\x01`@\x81\x90Rb\0\08\x91b\0\x01cV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\xA0Ra\xEA``\xC0R\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15b\0\0\x96WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15b\0\0\xB3WP0;\x15[\x90P\x81\x15\x80\x15b\0\0\xC2WP\x80\x15[\x15b\0\0\xE1W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T`\x01`\x01`@\x1B\x03\x19\x16`\x01\x17\x85U\x83\x15b\0\x01\x10W\x84T`\xFF`@\x1B\x19\x16h\x01\0\0\0\0\0\0\0\0\x17\x85U[\x83\x15b\0\x01WW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPb\0\x01\x95V[`\0` \x82\x84\x03\x12\x15b\0\x01vW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x8EW`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Qa)~b\0\x01\xE1`\09`\0\x81\x81a\x04\xE6\x01Ra\x14\x93\x01R`\0\x81\x81a\x05\xAF\x01Ra\x14\xFF\x01R`\0\x81\x81a\x11{\x01R\x81\x81a\x11\xA4\x01Ra\x12\xE8\x01Ra)~`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xC2W`\x005`\xE0\x1C\x80cr\xB5a\xF9\x11a\0\xF7W\x80c\xAA\x0E\xD0\x9F\x11a\0\x95W\x80c\xD5Gt\x1F\x11a\0dW\x80c\xD5Gt\x1F\x14a\x05\xE9W\x80c\xE1\xA610\x14a\x06\tW\x80c\xED8\r\x03\x14a\x06)W\x80c\xFF\x11J\xE1\x14a\x06IW`\0\x80\xFD[\x80c\xAA\x0E\xD0\x9F\x14a\x05\x1DW\x80c\xAD<\xB1\xCC\x14a\x05=W\x80c\xB8\n\xAA\x89\x14a\x05{W\x80c\xCDy\xF9\x06\x14a\x05\x9DW`\0\x80\xFD[\x80c\x88R!\t\x11a\0\xD1W\x80c\x88R!\t\x14a\x04\x94W\x80c\x91\xD1HT\x14a\x04\xB4W\x80c\x9A\xEC\x99\x0E\x14a\x04\xD4W\x80c\xA2\x17\xFD\xDF\x14a\x05\x08W`\0\x80\xFD[\x80cr\xB5a\xF9\x14a\x04\x0FW\x80cu\x84{\x84\x14a\x04@W\x80cyvi\xC9\x14a\x04`W`\0\x80\xFD[\x80c//\xF1]\x11a\x01dW\x80cR\xD1\x90-\x11a\x01>W\x80cR\xD1\x90-\x14a\x03aW\x80ci\xFD\xBC\xCA\x14a\x03vW\x80ck[!\xA6\x14a\x03\x96W\x80cr\x10Z\xAF\x14a\x03\xEFW`\0\x80\xFD[\x80c//\xF1]\x14a\x03\x0EW\x80c6V\x8A\xBE\x14a\x03.W\x80cO\x1E\xF2\x86\x14a\x03NW`\0\x80\xFD[\x80c\x14\x13\xA9*\x11a\x01\xA0W\x80c\x14\x13\xA9*\x14a\x02\x81W\x80c$\x10\xF6\xBA\x14a\x02\xA1W\x80c$\x8A\x9C\xA3\x14a\x02\xCEW\x80c.\xB3\x9E\xE9\x14a\x02\xEEW`\0\x80\xFD[\x80c\x01\xD5\x8F\xA3\x14a\x01\xC7W\x80c\x01\xFF\xC9\xA7\x14a\x02/W\x80c\x07\x07Y\x1F\x14a\x02_W[`\0\x80\xFD[4\x80\x15a\x01\xD3W`\0\x80\xFD[Pa\x02\x1Ca\x01\xE26`\x04a\x1C\xFEV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02;W`\0\x80\xFD[Pa\x02Oa\x02J6`\x04a\x1D\x19V[a\x06iV[`@Q\x90\x15\x15\x81R` \x01a\x02&V[4\x80\x15a\x02kW`\0\x80\xFD[Pa\x02\x7Fa\x02z6`\x04a\x1D\x8BV[a\x06zV[\0[4\x80\x15a\x02\x8DW`\0\x80\xFD[Pa\x02\x7Fa\x02\x9C6`\x04a\x1D\xCCV[a\x06\xA1V[4\x80\x15a\x02\xADW`\0\x80\xFD[Pa\x02\xC1a\x02\xBC6`\x04a\x1D\xEEV[a\x06\xC9V[`@Qa\x02&\x91\x90a\x1EWV[4\x80\x15a\x02\xDAW`\0\x80\xFD[Pa\x02\x1Ca\x02\xE96`\x04a\x1D\xEEV[a\x08\xD5V[4\x80\x15a\x02\xFAW`\0\x80\xFD[Pa\x02\x7Fa\x03\t6`\x04a\x1F\xAEV[a\x08\xF7V[4\x80\x15a\x03\x1AW`\0\x80\xFD[Pa\x02\x7Fa\x03)6`\x04a!\x05V[a\n]V[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x02\x7Fa\x03I6`\x04a!\x05V[a\nyV[a\x02\x7Fa\x03\\6`\x04a!1V[a\n\xACV[4\x80\x15a\x03mW`\0\x80\xFD[Pa\x02\x1Ca\n\xCBV[4\x80\x15a\x03\x82W`\0\x80\xFD[Pa\x02\x7Fa\x03\x916`\x04a!~V[a\n\xE8V[4\x80\x15a\x03\xA2W`\0\x80\xFD[Pa\x02Oa\x03\xB16`\x04a\x1D\xCCV[`\0\x90\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x02` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x90V[4\x80\x15a\x03\xFBW`\0\x80\xFD[Pa\x02\x7Fa\x04\n6`\x04a!\x05V[a\x0B\x9AV[4\x80\x15a\x04\x1BW`\0\x80\xFD[Pa\x02Oa\x04*6`\x04a\x1D\xEEV[a\x01\xF5` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x04LW`\0\x80\xFD[Pa\x02Oa\x04[6`\x04a\"\x07V[a\x0B\xA4V[4\x80\x15a\x04lW`\0\x80\xFD[Pa\x02\x1C\x7Fq\xF3\xD5XV\xE4\x05\x8E\xD0n\xE0W\xD7\x9A\xDAa_e\xCD\xF5\xF9\xEE\x88\x18\x1B\x91B%\x08\x8F\x83O\x81V[4\x80\x15a\x04\xA0W`\0\x80\xFD[Pa\x02\x7Fa\x04\xAF6`\x04a\x1D\xEEV[a\x0B\xB7V[4\x80\x15a\x04\xC0W`\0\x80\xFD[Pa\x02Oa\x04\xCF6`\x04a!\x05V[a\x0CdV[4\x80\x15a\x04\xE0W`\0\x80\xFD[Pa\x02\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\x14W`\0\x80\xFD[Pa\x02\x1C`\0\x81V[4\x80\x15a\x05)W`\0\x80\xFD[Pa\x02\x7Fa\x0586`\x04a\x1C\xFEV[a\x0C\x9CV[4\x80\x15a\x05IW`\0\x80\xFD[Pa\x05n`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02&\x91\x90a#\x10V[4\x80\x15a\x05\x87W`\0\x80\xFD[Pa\x02\x1C`\0\x80Q` a)\t\x839\x81Q\x91R\x81V[4\x80\x15a\x05\xA9W`\0\x80\xFD[Pa\x05\xD1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02&V[4\x80\x15a\x05\xF5W`\0\x80\xFD[Pa\x02\x7Fa\x06\x046`\x04a!\x05V[a\x0C\xBFV[4\x80\x15a\x06\x15W`\0\x80\xFD[Pa\x05na\x06$6`\x04a##V[a\x0C\xDBV[4\x80\x15a\x065W`\0\x80\xFD[Pa\x02\x7Fa\x06D6`\x04a##V[a\r\x81V[4\x80\x15a\x06UW`\0\x80\xFD[Pa\x02\x7Fa\x06d6`\x04a#MV[a\x0E\x01V[`\0a\x06t\x82a\x0E;V[\x92\x91PPV[`\0\x80Q` a)\t\x839\x81Q\x91Ra\x06\x92\x81a\x0EpV[a\x06\x9C\x83\x83a\x0E}V[PPPV[`\0\x80Q` a)\t\x839\x81Q\x91Ra\x06\xB9\x81a\x0EpV[a\x06\xC3\x83\x83a\x0E\xD7V[PPPPV[a\x06\xED`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81R`\0\x80Q` a(\xC9\x839\x81Q\x91R` \x81\x90R`@\x91\x82\x90 \x82Q``\x81\x01\x90\x93R\x80T\x91\x92\x91\x82\x90\x82\x90a\x07'\x90a#\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07S\x90a#\x98V[\x80\x15a\x07\xA0W\x80`\x1F\x10a\x07uWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xA0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x83W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x07\xB9\x90a#\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xE5\x90a#\x98V[\x80\x15a\x082W\x80`\x1F\x10a\x08\x07Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x082V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x15W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x08K\x90a#\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08w\x90a#\x98V[\x80\x15a\x08\xC4W\x80`\x1F\x10a\x08\x99Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xC4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xA7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[`\0\x90\x81R`\0\x80Q` a))\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\t<WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\tXWP0;\x15[\x90P\x81\x15\x80\x15a\tfWP\x80\x15[\x15a\t\x84W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\t\xAEW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\t\xB6a\x0F\x97V[a\t\xBEa\x0F\x97V[a\t\xC6a\x0F\x97V[a\t\xCEa\x0F\x97V[a\t\xD9`\0\x88a\x0F\xA1V[Pa\n\x05\x7Fq\xF3\xD5XV\xE4\x05\x8E\xD0n\xE0W\xD7\x9A\xDAa_e\xCD\xF5\xF9\xEE\x88\x18\x1B\x91B%\x08\x8F\x83O`\0a\x10FV[a\n\x0E\x86a\x10\xA9V[\x83\x15a\nTW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[a\nf\x82a\x08\xD5V[a\no\x81a\x0EpV[a\x06\xC3\x83\x83a\x0F\xA1V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\n\xA2W`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\x9C\x82\x82a\x10\xF4V[a\n\xB4a\x11pV[a\n\xBD\x82a\x12\x15V[a\n\xC7\x82\x82a\x12 V[PPV[`\0a\n\xD5a\x12\xDDV[P`\0\x80Q` a(\xE9\x839\x81Q\x91R\x90V[`\0\x80Q` a)\t\x839\x81Q\x91Ra\x0B\0\x81a\x0EpV[`@\x84\x14a\x0B!W`@Qc\xD2\x833]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x90 a\x0BP\x85\x87\x83a$ V[Pa\x0B[\x83\x83a\x0E}V[`@Q\x86\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x7FR\x1E=\xAC\x83\xE2\x8EU\xEF\xB9A\xFDXz\xBA\xB2\ty|\xEAqA\xEFP\xAA2\xF0\\\n;\xDB\x17\x90`\0\x90\xA3PPPPPPPV[a\n\xC7\x81\x83a\x13&V[`\0a\x0B\xB0\x83\x83a\x14\x03V[\x93\x92PPPV[\x7Fq\xF3\xD5XV\xE4\x05\x8E\xD0n\xE0W\xD7\x9A\xDAa_e\xCD\xF5\xF9\xEE\x88\x18\x1B\x91B%\x08\x8F\x83Oa\x0B\xE1\x81a\x0EpV[`\0\x82\x81Ra\x01\xF5` R`@\x90 T`\xFF\x16\x15a\x0C\x1AW`@Qc\x1A-s\x9D`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81Ra\x01\xF5` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x83\x91\x7F\xA4\xDFarq\xBD\xBBeA\xF3[G\x89\xDC\xEA\x98\x1F\x12\xB6\xB0\xA8\x932\x9A/\xA7As\xA7\xF1\xF0$\x91\xA2a\x06\x9C\x82a\x16\x0EV[`\0\x91\x82R`\0\x80Q` a))\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x0C\xA7\x81a\x0EpV[a\x06\x9C`\0\x80Q` a)\t\x839\x81Q\x91R\x83a\x0F\xA1V[a\x0C\xC8\x82a\x08\xD5V[a\x0C\xD1\x81a\x0EpV[a\x06\xC3\x83\x83a\x10\xF4V[a\x01\xF4` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\r\0\x90a#\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r,\x90a#\x98V[\x80\x15a\ryW\x80`\x1F\x10a\rNWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\ryV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\\W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x80Q` a)\t\x839\x81Q\x91Ra\r\x99\x81a\x0EpV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 a\r\xC6\x91a\x1C\x94V[`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xD4\x08O{z\x07\x9E|\xBF\xEE\x05\x96\xA4r1\xBAK\x1F\x97\x83a,\xB2r\x0E\xDE{\xF3\x8Eg\x10\xA2\x90`\0\x90\xA3PPPV[`\0\x80Q` a)\t\x839\x81Q\x91Ra\x0E\x19\x81a\x0EpV[`\0\x80\x80a\x0E)\x85\x87\x01\x87a$\xE0V[\x92P\x92P\x92Pa\nT\x87\x84\x84\x84a\x16\xB3V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06tWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x06tV[a\x0Ez\x813a\x17qV[PV[`\0\x80\x80\x80\x80\x80a\x0E\x90\x87\x89\x01\x89a%gV[\x95P\x95P\x95P\x95P\x95P\x95Pa\x0E\xCC\x86`@Q\x80`\xA0\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01\x85\x81R` \x01\x84\x81RPa\x14\x03V[PPPPPPPPPV[`\0\x81\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 T`\0\x80Q` a(\xC9\x839\x81Q\x91R\x90`\xFF\x16\x15\x15`\x01\x14a\x0F6W`\0\x91PPa\x06tV[`\0\x83\x81R`\x02\x82\x01` \x90\x81R`@\x80\x83 \x87\x84R\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x90U\x90Q\x84\x81R\x85\x91\x7F,\xFC\xB7\x95\xC3\x84\x05\x89\x98\xCD}\xC4\x14\xDD\xAB\xC7\xDD\x8C\x87\xADkY\xF4\xB0\r\xA8\x9E\xCBo\x7F\x16\x93\x91\x01[`@Q\x80\x91\x03\x90\xA2P`\x01\x93\x92PPPV[a\x0F\x9Fa\x17\xAAV[V[`\0`\0\x80Q` a))\x839\x81Q\x91Ra\x0F\xBC\x84\x84a\x0CdV[a\x10<W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0F\xF23\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\x06tV[`\0\x91PPa\x06tV[`\0\x80Q` a))\x839\x81Q\x91R`\0a\x10`\x84a\x08\xD5V[`\0\x85\x81R` \x84\x90R`@\x80\x82 `\x01\x01\x86\x90UQ\x91\x92P\x84\x91\x83\x91\x87\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPPV[a\x10\xB1a\x17\xAAV[`\0[\x81Q\x81\x10\x15a\n\xC7Wa\x10\xDF\x82\x82\x81Q\x81\x10a\x10\xD2Wa\x10\xD2a&@V[` \x02` \x01\x01Qa\x17\xF3V[PP\x80\x80a\x10\xEC\x90a&lV[\x91PPa\x10\xB4V[`\0`\0\x80Q` a))\x839\x81Q\x91Ra\x11\x0F\x84\x84a\x0CdV[\x15a\x10<W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\x06tV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x11\xF7WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x11\xEB`\0\x80Q` a(\xE9\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0F\x9FW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\n\xC7\x81a\x0EpV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x12zWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x12w\x91\x81\x01\x90a&\x85V[`\x01[a\x12\xA2W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x0C\x11V[`\0\x80Q` a(\xE9\x839\x81Q\x91R\x81\x14a\x12\xD3W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0C\x11V[a\x06\x9C\x83\x83a\x19pV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0F\x9FW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x01` R`@\x90 T`\0\x80Q` a(\xC9\x839\x81Q\x91R\x90\x80a\x13\x8CW`@Qc=\xD8\xCA\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R` \x83\x90R`@\x90 \x80Ta\x13\xA5\x90a#\x98V[\x90P`\0\x03a\x13\xC7W`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R`\x02\x83\x01` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T`\xFF\x16a\x06\xC3W`@QcHf%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80Q` a(\xC9\x839\x81Q\x91R\x90P`\0\x83` \x01Q\x84`@\x01Q\x85``\x01Q`@Q` \x01a\x14:\x93\x92\x91\x90a&\x9EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x85\x90R\x91 \x80T\x91\x92P\x90a\x14l\x90a#\x98V[\x90P`\0\x03a\x14\x8EW`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba&\xE1V[a\x03\xE8\x85`\x80\x01Qa\x14\xCA\x91\x90a&\xF4V[\x11a\x14\xE8W`@Qc\x0C\xB0/\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xEA\xC7\x08\xA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEA\xC7\x08\xA3\x90a\x156\x90\x88\x90\x88\x90`\x04\x01a'\x16V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15NW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x15bW=`\0\x80>=`\0\xFD[PPPP`\0a\x15u\x85`\0\x01Qa\x19\xC6V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x85\x01` R`@\x90 T\x90\x91P\x15a\x15\xA4W`\0\x93PPPPa\x06tV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x84\x01` R`@\x90\x81\x90 \x83\x90U\x85Q\x90Q\x83\x91a\x15\xD2\x91a'\xAAV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF4\x83\x7F\x1Af\xA1\xE5\xFB\xDE,@\xD0\xF6\xEF\xF7Q\x83\x13\x9Ey\xB0\xFC\x1E\xAA\xB2\x80\xA3\xA0\xBE8\xE7\xB1\x90`\0\x90\xA3P`\x01\x95\x94PPPPPV[`\0\x81\x81R`\0\x80Q` a(\xC9\x839\x81Q\x91R` \x81\x90R`@\x82 \x80Ta\x166\x90a#\x98V[\x90P`\0\x03a\x16HWP`\0\x92\x91PPV[`\0\x83\x81R` \x82\x90R`@\x81 \x90a\x16a\x82\x82a\x1C\x94V[a\x16o`\x01\x83\x01`\0a\x1C\x94V[a\x16}`\x02\x83\x01`\0a\x1C\x94V[PP`@Q\x83\x90\x7FKq\xBE\xDD\xA4!yf\xEA\xC1\xAAI\xE3[\x15P\xE5\xAB\x87\x06J\xE1y\xD1\xB3k9;\xD0N\xB7C\x90`\0\x90\xA2P`\x01\x92\x91PPV[`\0a\x16\xC0\x84\x84\x84a\x19\xF6V[\x90Pa\x16\xCB\x81a\x1A/V[a\x16\xEBW`@Qc\x065l\xB3`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0C\x11V[`\0\x81\x81Ra\x01\xF5` R`@\x90 T`\xFF\x16\x15a\x17\x1FW`@Qc\"m\xD8\xA3`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0C\x11V[`\0a\x17D`@Q\x80``\x01`@R\x80\x87\x81R` \x01\x86\x81R` \x01\x85\x81RPa\x17\xF3V[P\x90P\x81\x81\x14a\x17gW`@Qc\x0CZ\x1A\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\nT\x82\x87a\x1AdV[a\x17{\x82\x82a\x0CdV[a\n\xC7W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x0C\x11V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x0F\x9FW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80QQ`\0\x90\x81\x90`\0\x80Q` a(\xC9\x839\x81Q\x91R\x90`0\x14\x80\x15a\x18\x1FWP\x83` \x01QQ`0\x14[\x80\x15a\x180WP\x83`@\x01QQ`0\x14[a\x18MW`@QcBc\r\xDB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q`@Q` \x01a\x18p\x93\x92\x91\x90a&\x9EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x85\x90R\x91 \x80T\x91\x92P\x90a\x18\xA2\x90a#\x98V[\x15\x90Pa\x18\xB4W\x94`\0\x94P\x92PPPV[`@\x80Q``\x81\x01\x82R\x86Q\x81R` \x80\x88\x01Q\x81\x83\x01R\x87\x83\x01Q\x82\x84\x01R`\0\x84\x81R\x90\x85\x90R\x91\x90\x91 \x81Q\x81\x90a\x18\xEF\x90\x82a'\xC6V[P` \x82\x01Q`\x01\x82\x01\x90a\x19\x04\x90\x82a'\xC6V[P`@\x82\x01Q`\x02\x82\x01\x90a\x19\x19\x90\x82a'\xC6V[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x86`\0\x01Q\x87` \x01Q\x88`@\x01Q`@Qa\x19]\x93\x92\x91\x90a(\x85V[`@Q\x80\x91\x03\x90\xA2\x94`\x01\x94P\x92PPPV[a\x19y\x82a\x1B\x15V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\x19\xBEWa\x06\x9C\x82\x82a\x1BzV[a\n\xC7a\x1B\xF0V[`\0\x81Q`@\x14a\x19\xEAW`@Qc\xBD\x9C\x80\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[`\0\x80\x84\x84\x84`@Q` \x01a\x1A\x0E\x93\x92\x91\x90a&\x9EV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x81\x15\x80a\x1A]WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x82\x14[\x15\x92\x91PPV[`\0\x81\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 T`\0\x80Q` a(\xC9\x839\x81Q\x91R\x90`\xFF\x16\x15a\x1A\xBFW`\0\x91PPa\x06tV[`\0\x83\x81R`\x02\x82\x01` \x90\x81R`@\x80\x83 \x87\x84R\x82R\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90Q\x84\x81R\x85\x91\x7F\xBF\xB1&\xE7B\xCE\x96\x18\xB5\xBFkT\x83\x99\x16\x92o\\9wR\xBE5@L\x83h\xDD\xCFh\xC1\n\x91\x01a\x0F\x85V[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x1BKW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0C\x11V[`\0\x80Q` a(\xE9\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1B\x97\x91\x90a'\xAAV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1B\xD2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1B\xD7V[``\x91P[P\x91P\x91Pa\x1B\xE7\x85\x83\x83a\x1C\x0FV[\x95\x94PPPPPV[4\x15a\x0F\x9FW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x1C$Wa\x1C\x1F\x82a\x1CkV[a\x0B\xB0V[\x81Q\x15\x80\x15a\x1C;WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1CdW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x0C\x11V[P\x80a\x0B\xB0V[\x80Q\x15a\x1C{W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Ta\x1C\xA0\x90a#\x98V[`\0\x82U\x80`\x1F\x10a\x1C\xB0WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x0Ez\x91\x90[\x80\x82\x11\x15a\x1C\xDEW`\0\x81U`\x01\x01a\x1C\xCAV[P\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\xF9W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1D\x10W`\0\x80\xFD[a\x0B\xB0\x82a\x1C\xE2V[`\0` \x82\x84\x03\x12\x15a\x1D+W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B\xB0W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x1DUW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DlW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1D\x84W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x1D\x9EW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xB4W`\0\x80\xFD[a\x1D\xC0\x85\x82\x86\x01a\x1DCV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\xDFW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x1E\0W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x1E\"W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1E\nV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1EC\x81` \x86\x01` \x86\x01a\x1E\x07V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0\x82Q``` \x84\x01Ra\x1Es`\x80\x84\x01\x82a\x1E+V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra\x1E\x91\x83\x83a\x1E+V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa\x1B\xE7\x82\x82a\x1E+V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\xE7Wa\x1E\xE7a\x1E\xAFV[`@R\x90V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\xE7Wa\x1E\xE7a\x1E\xAFV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1F7Wa\x1F7a\x1E\xAFV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1FPW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FiWa\x1Fia\x1E\xAFV[a\x1F|`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1F\x0FV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1F\x91W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\xC1W`\0\x80\xFD[a\x1F\xCA\x83a\x1C\xE2V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1F\xE7W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1F\xFBW`\0\x80\xFD[\x815\x81\x81\x11\x15a \rWa \ra\x1E\xAFV[\x80`\x05\x1Ba \x1C\x85\x82\x01a\x1F\x0FV[\x91\x82R\x83\x81\x01\x85\x01\x91\x85\x81\x01\x90\x8A\x84\x11\x15a 6W`\0\x80\xFD[\x86\x86\x01\x92P[\x83\x83\x10\x15a \xF4W\x825\x85\x81\x11\x15a TW`\0\x80\x81\xFD[\x86\x01``\x81\x8D\x03`\x1F\x19\x01\x81\x13\x15a lW`\0\x80\x81\xFD[a ta\x1E\xC5V[\x89\x83\x015\x88\x81\x11\x15a \x86W`\0\x80\x81\xFD[a \x94\x8F\x8C\x83\x87\x01\x01a\x1F?V[\x82RP`@\x83\x015\x88\x81\x11\x15a \xAAW`\0\x80\x81\xFD[a \xB8\x8F\x8C\x83\x87\x01\x01a\x1F?V[\x82\x8C\x01RP\x90\x82\x015\x90\x87\x82\x11\x15a \xD0W`\0\x80\x81\xFD[a \xDE\x8E\x8B\x84\x86\x01\x01a\x1F?V[`@\x82\x01R\x84RPP\x91\x86\x01\x91\x90\x86\x01\x90a <V[\x80\x97PPPPPPPP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a!\x18W`\0\x80\xFD[\x825\x91Pa!(` \x84\x01a\x1C\xE2V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a!DW`\0\x80\xFD[a!M\x83a\x1C\xE2V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!hW`\0\x80\xFD[a!t\x85\x82\x86\x01a\x1F?V[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a!\x97W`\0\x80\xFD[a!\xA0\x87a\x1C\xE2V[\x95P` \x87\x015\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a!\xC3W`\0\x80\xFD[a!\xCF\x8A\x83\x8B\x01a\x1DCV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a!\xE8W`\0\x80\xFD[Pa!\xF5\x89\x82\x8A\x01a\x1DCV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\"\x1AW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"1W`\0\x80\xFD[a\"=\x86\x83\x87\x01a\x1F?V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\"SW`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\"gW`\0\x80\xFD[a\"oa\x1E\xEDV[\x825\x82\x81\x11\x15a\"~W`\0\x80\xFD[a\"\x8A\x88\x82\x86\x01a\x1F?V[\x82RP` \x83\x015\x82\x81\x11\x15a\"\x9FW`\0\x80\xFD[a\"\xAB\x88\x82\x86\x01a\x1F?V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\"\xC3W`\0\x80\xFD[a\"\xCF\x88\x82\x86\x01a\x1F?V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\"\xE7W`\0\x80\xFD[a\"\xF3\x88\x82\x86\x01a\x1F?V[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x81R`\0a\x0B\xB0` \x83\x01\x84a\x1E+V[`\0\x80`@\x83\x85\x03\x12\x15a#6W`\0\x80\xFD[a#?\x83a\x1C\xE2V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a#bW`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x7FW`\0\x80\xFD[a#\x8B\x86\x82\x87\x01a\x1DCV[\x94\x97\x90\x96P\x93\x94PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a#\xACW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a#\xCCWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x06\x9CW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a#\xF9WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a$\x18W\x82\x81U`\x01\x01a$\x05V[PPPPPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15a$7Wa$7a\x1E\xAFV[a$K\x83a$E\x83Ta#\x98V[\x83a#\xD2V[`\0`\x1F\x84\x11`\x01\x81\x14a$\x7FW`\0\x85\x15a$gWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua$\xD9V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a$\xB0W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a$\x90V[P\x86\x82\x10\x15a$\xCDW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a$\xF5W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%\x0CW`\0\x80\xFD[a%\x18\x87\x83\x88\x01a\x1F?V[\x94P` \x86\x015\x91P\x80\x82\x11\x15a%.W`\0\x80\xFD[a%:\x87\x83\x88\x01a\x1F?V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a%PW`\0\x80\xFD[Pa%]\x86\x82\x87\x01a\x1F?V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a%\x80W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%\x97W`\0\x80\xFD[a%\xA3\x8A\x83\x8B\x01a\x1F?V[\x97P` \x89\x015\x91P\x80\x82\x11\x15a%\xB9W`\0\x80\xFD[a%\xC5\x8A\x83\x8B\x01a\x1F?V[\x96P`@\x89\x015\x91P\x80\x82\x11\x15a%\xDBW`\0\x80\xFD[a%\xE7\x8A\x83\x8B\x01a\x1F?V[\x95P``\x89\x015\x91P\x80\x82\x11\x15a%\xFDW`\0\x80\xFD[a&\t\x8A\x83\x8B\x01a\x1F?V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15a&\x1FW`\0\x80\xFD[Pa&,\x89\x82\x8A\x01a\x1F?V[\x92PP`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a&~Wa&~a&VV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a&\x97W`\0\x80\xFD[PQ\x91\x90PV[`\0\x84Qa&\xB0\x81\x84` \x89\x01a\x1E\x07V[\x84Q\x90\x83\x01\x90a&\xC4\x81\x83` \x89\x01a\x1E\x07V[\x84Q\x91\x01\x90a&\xD7\x81\x83` \x88\x01a\x1E\x07V[\x01\x95\x94PPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x06tWa\x06ta&VV[`\0\x82a'\x11WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`@\x81R`\0a')`@\x83\x01\x85a\x1E+V[\x82\x81\x03` \x84\x01R\x83Q`\xA0\x82Ra'D`\xA0\x83\x01\x82a\x1E+V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01Ra']\x82\x82a\x1E+V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra'w\x82\x82a\x1E+V[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01Ra'\x91\x82\x82a\x1E+V[\x91PP`\x80\x85\x01Q`\x80\x83\x01R\x80\x92PPP\x93\x92PPPV[`\0\x82Qa'\xBC\x81\x84` \x87\x01a\x1E\x07V[\x91\x90\x91\x01\x92\x91PPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xDFWa'\xDFa\x1E\xAFV[a'\xF3\x81a'\xED\x84Ta#\x98V[\x84a#\xD2V[` \x80`\x1F\x83\x11`\x01\x81\x14a((W`\0\x84\x15a(\x10WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua$\x18V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a(WW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a(8V[P\x85\x82\x10\x15a(uW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[``\x81R`\0a(\x98``\x83\x01\x86a\x1E+V[\x82\x81\x03` \x84\x01Ra(\xAA\x81\x86a\x1E+V[\x90P\x82\x81\x03`@\x84\x01Ra(\xBE\x81\x85a\x1E+V[\x96\x95PPPPPPV\xFE\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x006\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xDCi|\xE6\xC0\xC3\xF8\x86\xFB\xBC\x8E\r#:\xBF\xACZ\xC56S\xAC\x08\xB3c[\xB0\xAE\xC7\x0C9\\B\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\xA2dipfsX\"\x12 \0\xBC\x02\"\xD5\xD5i\xFBY>\x18\xFC/\xB4\x18\xC5\ti\xFAmo\x05\xDB\x8E\xEAi\xCF\x0E\x1D\x18_\xB3dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ENTITYKEYREGISTRY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xC2W`\x005`\xE0\x1C\x80cr\xB5a\xF9\x11a\0\xF7W\x80c\xAA\x0E\xD0\x9F\x11a\0\x95W\x80c\xD5Gt\x1F\x11a\0dW\x80c\xD5Gt\x1F\x14a\x05\xE9W\x80c\xE1\xA610\x14a\x06\tW\x80c\xED8\r\x03\x14a\x06)W\x80c\xFF\x11J\xE1\x14a\x06IW`\0\x80\xFD[\x80c\xAA\x0E\xD0\x9F\x14a\x05\x1DW\x80c\xAD<\xB1\xCC\x14a\x05=W\x80c\xB8\n\xAA\x89\x14a\x05{W\x80c\xCDy\xF9\x06\x14a\x05\x9DW`\0\x80\xFD[\x80c\x88R!\t\x11a\0\xD1W\x80c\x88R!\t\x14a\x04\x94W\x80c\x91\xD1HT\x14a\x04\xB4W\x80c\x9A\xEC\x99\x0E\x14a\x04\xD4W\x80c\xA2\x17\xFD\xDF\x14a\x05\x08W`\0\x80\xFD[\x80cr\xB5a\xF9\x14a\x04\x0FW\x80cu\x84{\x84\x14a\x04@W\x80cyvi\xC9\x14a\x04`W`\0\x80\xFD[\x80c//\xF1]\x11a\x01dW\x80cR\xD1\x90-\x11a\x01>W\x80cR\xD1\x90-\x14a\x03aW\x80ci\xFD\xBC\xCA\x14a\x03vW\x80ck[!\xA6\x14a\x03\x96W\x80cr\x10Z\xAF\x14a\x03\xEFW`\0\x80\xFD[\x80c//\xF1]\x14a\x03\x0EW\x80c6V\x8A\xBE\x14a\x03.W\x80cO\x1E\xF2\x86\x14a\x03NW`\0\x80\xFD[\x80c\x14\x13\xA9*\x11a\x01\xA0W\x80c\x14\x13\xA9*\x14a\x02\x81W\x80c$\x10\xF6\xBA\x14a\x02\xA1W\x80c$\x8A\x9C\xA3\x14a\x02\xCEW\x80c.\xB3\x9E\xE9\x14a\x02\xEEW`\0\x80\xFD[\x80c\x01\xD5\x8F\xA3\x14a\x01\xC7W\x80c\x01\xFF\xC9\xA7\x14a\x02/W\x80c\x07\x07Y\x1F\x14a\x02_W[`\0\x80\xFD[4\x80\x15a\x01\xD3W`\0\x80\xFD[Pa\x02\x1Ca\x01\xE26`\x04a\x1C\xFEV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02;W`\0\x80\xFD[Pa\x02Oa\x02J6`\x04a\x1D\x19V[a\x06iV[`@Q\x90\x15\x15\x81R` \x01a\x02&V[4\x80\x15a\x02kW`\0\x80\xFD[Pa\x02\x7Fa\x02z6`\x04a\x1D\x8BV[a\x06zV[\0[4\x80\x15a\x02\x8DW`\0\x80\xFD[Pa\x02\x7Fa\x02\x9C6`\x04a\x1D\xCCV[a\x06\xA1V[4\x80\x15a\x02\xADW`\0\x80\xFD[Pa\x02\xC1a\x02\xBC6`\x04a\x1D\xEEV[a\x06\xC9V[`@Qa\x02&\x91\x90a\x1EWV[4\x80\x15a\x02\xDAW`\0\x80\xFD[Pa\x02\x1Ca\x02\xE96`\x04a\x1D\xEEV[a\x08\xD5V[4\x80\x15a\x02\xFAW`\0\x80\xFD[Pa\x02\x7Fa\x03\t6`\x04a\x1F\xAEV[a\x08\xF7V[4\x80\x15a\x03\x1AW`\0\x80\xFD[Pa\x02\x7Fa\x03)6`\x04a!\x05V[a\n]V[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x02\x7Fa\x03I6`\x04a!\x05V[a\nyV[a\x02\x7Fa\x03\\6`\x04a!1V[a\n\xACV[4\x80\x15a\x03mW`\0\x80\xFD[Pa\x02\x1Ca\n\xCBV[4\x80\x15a\x03\x82W`\0\x80\xFD[Pa\x02\x7Fa\x03\x916`\x04a!~V[a\n\xE8V[4\x80\x15a\x03\xA2W`\0\x80\xFD[Pa\x02Oa\x03\xB16`\x04a\x1D\xCCV[`\0\x90\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x02` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x90V[4\x80\x15a\x03\xFBW`\0\x80\xFD[Pa\x02\x7Fa\x04\n6`\x04a!\x05V[a\x0B\x9AV[4\x80\x15a\x04\x1BW`\0\x80\xFD[Pa\x02Oa\x04*6`\x04a\x1D\xEEV[a\x01\xF5` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x04LW`\0\x80\xFD[Pa\x02Oa\x04[6`\x04a\"\x07V[a\x0B\xA4V[4\x80\x15a\x04lW`\0\x80\xFD[Pa\x02\x1C\x7Fq\xF3\xD5XV\xE4\x05\x8E\xD0n\xE0W\xD7\x9A\xDAa_e\xCD\xF5\xF9\xEE\x88\x18\x1B\x91B%\x08\x8F\x83O\x81V[4\x80\x15a\x04\xA0W`\0\x80\xFD[Pa\x02\x7Fa\x04\xAF6`\x04a\x1D\xEEV[a\x0B\xB7V[4\x80\x15a\x04\xC0W`\0\x80\xFD[Pa\x02Oa\x04\xCF6`\x04a!\x05V[a\x0CdV[4\x80\x15a\x04\xE0W`\0\x80\xFD[Pa\x02\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\x14W`\0\x80\xFD[Pa\x02\x1C`\0\x81V[4\x80\x15a\x05)W`\0\x80\xFD[Pa\x02\x7Fa\x0586`\x04a\x1C\xFEV[a\x0C\x9CV[4\x80\x15a\x05IW`\0\x80\xFD[Pa\x05n`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02&\x91\x90a#\x10V[4\x80\x15a\x05\x87W`\0\x80\xFD[Pa\x02\x1C`\0\x80Q` a)\t\x839\x81Q\x91R\x81V[4\x80\x15a\x05\xA9W`\0\x80\xFD[Pa\x05\xD1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02&V[4\x80\x15a\x05\xF5W`\0\x80\xFD[Pa\x02\x7Fa\x06\x046`\x04a!\x05V[a\x0C\xBFV[4\x80\x15a\x06\x15W`\0\x80\xFD[Pa\x05na\x06$6`\x04a##V[a\x0C\xDBV[4\x80\x15a\x065W`\0\x80\xFD[Pa\x02\x7Fa\x06D6`\x04a##V[a\r\x81V[4\x80\x15a\x06UW`\0\x80\xFD[Pa\x02\x7Fa\x06d6`\x04a#MV[a\x0E\x01V[`\0a\x06t\x82a\x0E;V[\x92\x91PPV[`\0\x80Q` a)\t\x839\x81Q\x91Ra\x06\x92\x81a\x0EpV[a\x06\x9C\x83\x83a\x0E}V[PPPV[`\0\x80Q` a)\t\x839\x81Q\x91Ra\x06\xB9\x81a\x0EpV[a\x06\xC3\x83\x83a\x0E\xD7V[PPPPV[a\x06\xED`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81R`\0\x80Q` a(\xC9\x839\x81Q\x91R` \x81\x90R`@\x91\x82\x90 \x82Q``\x81\x01\x90\x93R\x80T\x91\x92\x91\x82\x90\x82\x90a\x07'\x90a#\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07S\x90a#\x98V[\x80\x15a\x07\xA0W\x80`\x1F\x10a\x07uWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xA0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x83W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x07\xB9\x90a#\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xE5\x90a#\x98V[\x80\x15a\x082W\x80`\x1F\x10a\x08\x07Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x082V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x15W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x08K\x90a#\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08w\x90a#\x98V[\x80\x15a\x08\xC4W\x80`\x1F\x10a\x08\x99Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xC4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xA7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[`\0\x90\x81R`\0\x80Q` a))\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\t<WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\tXWP0;\x15[\x90P\x81\x15\x80\x15a\tfWP\x80\x15[\x15a\t\x84W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\t\xAEW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\t\xB6a\x0F\x97V[a\t\xBEa\x0F\x97V[a\t\xC6a\x0F\x97V[a\t\xCEa\x0F\x97V[a\t\xD9`\0\x88a\x0F\xA1V[Pa\n\x05\x7Fq\xF3\xD5XV\xE4\x05\x8E\xD0n\xE0W\xD7\x9A\xDAa_e\xCD\xF5\xF9\xEE\x88\x18\x1B\x91B%\x08\x8F\x83O`\0a\x10FV[a\n\x0E\x86a\x10\xA9V[\x83\x15a\nTW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[a\nf\x82a\x08\xD5V[a\no\x81a\x0EpV[a\x06\xC3\x83\x83a\x0F\xA1V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\n\xA2W`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\x9C\x82\x82a\x10\xF4V[a\n\xB4a\x11pV[a\n\xBD\x82a\x12\x15V[a\n\xC7\x82\x82a\x12 V[PPV[`\0a\n\xD5a\x12\xDDV[P`\0\x80Q` a(\xE9\x839\x81Q\x91R\x90V[`\0\x80Q` a)\t\x839\x81Q\x91Ra\x0B\0\x81a\x0EpV[`@\x84\x14a\x0B!W`@Qc\xD2\x833]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x90 a\x0BP\x85\x87\x83a$ V[Pa\x0B[\x83\x83a\x0E}V[`@Q\x86\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x7FR\x1E=\xAC\x83\xE2\x8EU\xEF\xB9A\xFDXz\xBA\xB2\ty|\xEAqA\xEFP\xAA2\xF0\\\n;\xDB\x17\x90`\0\x90\xA3PPPPPPPV[a\n\xC7\x81\x83a\x13&V[`\0a\x0B\xB0\x83\x83a\x14\x03V[\x93\x92PPPV[\x7Fq\xF3\xD5XV\xE4\x05\x8E\xD0n\xE0W\xD7\x9A\xDAa_e\xCD\xF5\xF9\xEE\x88\x18\x1B\x91B%\x08\x8F\x83Oa\x0B\xE1\x81a\x0EpV[`\0\x82\x81Ra\x01\xF5` R`@\x90 T`\xFF\x16\x15a\x0C\x1AW`@Qc\x1A-s\x9D`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81Ra\x01\xF5` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x83\x91\x7F\xA4\xDFarq\xBD\xBBeA\xF3[G\x89\xDC\xEA\x98\x1F\x12\xB6\xB0\xA8\x932\x9A/\xA7As\xA7\xF1\xF0$\x91\xA2a\x06\x9C\x82a\x16\x0EV[`\0\x91\x82R`\0\x80Q` a))\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x0C\xA7\x81a\x0EpV[a\x06\x9C`\0\x80Q` a)\t\x839\x81Q\x91R\x83a\x0F\xA1V[a\x0C\xC8\x82a\x08\xD5V[a\x0C\xD1\x81a\x0EpV[a\x06\xC3\x83\x83a\x10\xF4V[a\x01\xF4` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\r\0\x90a#\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r,\x90a#\x98V[\x80\x15a\ryW\x80`\x1F\x10a\rNWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\ryV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\\W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x80Q` a)\t\x839\x81Q\x91Ra\r\x99\x81a\x0EpV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x01\xF4` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 a\r\xC6\x91a\x1C\x94V[`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xD4\x08O{z\x07\x9E|\xBF\xEE\x05\x96\xA4r1\xBAK\x1F\x97\x83a,\xB2r\x0E\xDE{\xF3\x8Eg\x10\xA2\x90`\0\x90\xA3PPPV[`\0\x80Q` a)\t\x839\x81Q\x91Ra\x0E\x19\x81a\x0EpV[`\0\x80\x80a\x0E)\x85\x87\x01\x87a$\xE0V[\x92P\x92P\x92Pa\nT\x87\x84\x84\x84a\x16\xB3V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06tWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x06tV[a\x0Ez\x813a\x17qV[PV[`\0\x80\x80\x80\x80\x80a\x0E\x90\x87\x89\x01\x89a%gV[\x95P\x95P\x95P\x95P\x95P\x95Pa\x0E\xCC\x86`@Q\x80`\xA0\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01\x85\x81R` \x01\x84\x81RPa\x14\x03V[PPPPPPPPPV[`\0\x81\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 T`\0\x80Q` a(\xC9\x839\x81Q\x91R\x90`\xFF\x16\x15\x15`\x01\x14a\x0F6W`\0\x91PPa\x06tV[`\0\x83\x81R`\x02\x82\x01` \x90\x81R`@\x80\x83 \x87\x84R\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x90U\x90Q\x84\x81R\x85\x91\x7F,\xFC\xB7\x95\xC3\x84\x05\x89\x98\xCD}\xC4\x14\xDD\xAB\xC7\xDD\x8C\x87\xADkY\xF4\xB0\r\xA8\x9E\xCBo\x7F\x16\x93\x91\x01[`@Q\x80\x91\x03\x90\xA2P`\x01\x93\x92PPPV[a\x0F\x9Fa\x17\xAAV[V[`\0`\0\x80Q` a))\x839\x81Q\x91Ra\x0F\xBC\x84\x84a\x0CdV[a\x10<W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0F\xF23\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\x06tV[`\0\x91PPa\x06tV[`\0\x80Q` a))\x839\x81Q\x91R`\0a\x10`\x84a\x08\xD5V[`\0\x85\x81R` \x84\x90R`@\x80\x82 `\x01\x01\x86\x90UQ\x91\x92P\x84\x91\x83\x91\x87\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPPV[a\x10\xB1a\x17\xAAV[`\0[\x81Q\x81\x10\x15a\n\xC7Wa\x10\xDF\x82\x82\x81Q\x81\x10a\x10\xD2Wa\x10\xD2a&@V[` \x02` \x01\x01Qa\x17\xF3V[PP\x80\x80a\x10\xEC\x90a&lV[\x91PPa\x10\xB4V[`\0`\0\x80Q` a))\x839\x81Q\x91Ra\x11\x0F\x84\x84a\x0CdV[\x15a\x10<W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\x06tV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x11\xF7WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x11\xEB`\0\x80Q` a(\xE9\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0F\x9FW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\n\xC7\x81a\x0EpV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x12zWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x12w\x91\x81\x01\x90a&\x85V[`\x01[a\x12\xA2W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x0C\x11V[`\0\x80Q` a(\xE9\x839\x81Q\x91R\x81\x14a\x12\xD3W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0C\x11V[a\x06\x9C\x83\x83a\x19pV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0F\x9FW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x01` R`@\x90 T`\0\x80Q` a(\xC9\x839\x81Q\x91R\x90\x80a\x13\x8CW`@Qc=\xD8\xCA\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R` \x83\x90R`@\x90 \x80Ta\x13\xA5\x90a#\x98V[\x90P`\0\x03a\x13\xC7W`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R`\x02\x83\x01` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T`\xFF\x16a\x06\xC3W`@QcHf%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80Q` a(\xC9\x839\x81Q\x91R\x90P`\0\x83` \x01Q\x84`@\x01Q\x85``\x01Q`@Q` \x01a\x14:\x93\x92\x91\x90a&\x9EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x85\x90R\x91 \x80T\x91\x92P\x90a\x14l\x90a#\x98V[\x90P`\0\x03a\x14\x8EW`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba&\xE1V[a\x03\xE8\x85`\x80\x01Qa\x14\xCA\x91\x90a&\xF4V[\x11a\x14\xE8W`@Qc\x0C\xB0/\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xEA\xC7\x08\xA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEA\xC7\x08\xA3\x90a\x156\x90\x88\x90\x88\x90`\x04\x01a'\x16V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15NW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x15bW=`\0\x80>=`\0\xFD[PPPP`\0a\x15u\x85`\0\x01Qa\x19\xC6V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x85\x01` R`@\x90 T\x90\x91P\x15a\x15\xA4W`\0\x93PPPPa\x06tV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x84\x01` R`@\x90\x81\x90 \x83\x90U\x85Q\x90Q\x83\x91a\x15\xD2\x91a'\xAAV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF4\x83\x7F\x1Af\xA1\xE5\xFB\xDE,@\xD0\xF6\xEF\xF7Q\x83\x13\x9Ey\xB0\xFC\x1E\xAA\xB2\x80\xA3\xA0\xBE8\xE7\xB1\x90`\0\x90\xA3P`\x01\x95\x94PPPPPV[`\0\x81\x81R`\0\x80Q` a(\xC9\x839\x81Q\x91R` \x81\x90R`@\x82 \x80Ta\x166\x90a#\x98V[\x90P`\0\x03a\x16HWP`\0\x92\x91PPV[`\0\x83\x81R` \x82\x90R`@\x81 \x90a\x16a\x82\x82a\x1C\x94V[a\x16o`\x01\x83\x01`\0a\x1C\x94V[a\x16}`\x02\x83\x01`\0a\x1C\x94V[PP`@Q\x83\x90\x7FKq\xBE\xDD\xA4!yf\xEA\xC1\xAAI\xE3[\x15P\xE5\xAB\x87\x06J\xE1y\xD1\xB3k9;\xD0N\xB7C\x90`\0\x90\xA2P`\x01\x92\x91PPV[`\0a\x16\xC0\x84\x84\x84a\x19\xF6V[\x90Pa\x16\xCB\x81a\x1A/V[a\x16\xEBW`@Qc\x065l\xB3`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0C\x11V[`\0\x81\x81Ra\x01\xF5` R`@\x90 T`\xFF\x16\x15a\x17\x1FW`@Qc\"m\xD8\xA3`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0C\x11V[`\0a\x17D`@Q\x80``\x01`@R\x80\x87\x81R` \x01\x86\x81R` \x01\x85\x81RPa\x17\xF3V[P\x90P\x81\x81\x14a\x17gW`@Qc\x0CZ\x1A\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\nT\x82\x87a\x1AdV[a\x17{\x82\x82a\x0CdV[a\n\xC7W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x0C\x11V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x0F\x9FW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80QQ`\0\x90\x81\x90`\0\x80Q` a(\xC9\x839\x81Q\x91R\x90`0\x14\x80\x15a\x18\x1FWP\x83` \x01QQ`0\x14[\x80\x15a\x180WP\x83`@\x01QQ`0\x14[a\x18MW`@QcBc\r\xDB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q`@Q` \x01a\x18p\x93\x92\x91\x90a&\x9EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x85\x90R\x91 \x80T\x91\x92P\x90a\x18\xA2\x90a#\x98V[\x15\x90Pa\x18\xB4W\x94`\0\x94P\x92PPPV[`@\x80Q``\x81\x01\x82R\x86Q\x81R` \x80\x88\x01Q\x81\x83\x01R\x87\x83\x01Q\x82\x84\x01R`\0\x84\x81R\x90\x85\x90R\x91\x90\x91 \x81Q\x81\x90a\x18\xEF\x90\x82a'\xC6V[P` \x82\x01Q`\x01\x82\x01\x90a\x19\x04\x90\x82a'\xC6V[P`@\x82\x01Q`\x02\x82\x01\x90a\x19\x19\x90\x82a'\xC6V[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x86`\0\x01Q\x87` \x01Q\x88`@\x01Q`@Qa\x19]\x93\x92\x91\x90a(\x85V[`@Q\x80\x91\x03\x90\xA2\x94`\x01\x94P\x92PPPV[a\x19y\x82a\x1B\x15V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\x19\xBEWa\x06\x9C\x82\x82a\x1BzV[a\n\xC7a\x1B\xF0V[`\0\x81Q`@\x14a\x19\xEAW`@Qc\xBD\x9C\x80\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[`\0\x80\x84\x84\x84`@Q` \x01a\x1A\x0E\x93\x92\x91\x90a&\x9EV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x81\x15\x80a\x1A]WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x82\x14[\x15\x92\x91PPV[`\0\x81\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 T`\0\x80Q` a(\xC9\x839\x81Q\x91R\x90`\xFF\x16\x15a\x1A\xBFW`\0\x91PPa\x06tV[`\0\x83\x81R`\x02\x82\x01` \x90\x81R`@\x80\x83 \x87\x84R\x82R\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90Q\x84\x81R\x85\x91\x7F\xBF\xB1&\xE7B\xCE\x96\x18\xB5\xBFkT\x83\x99\x16\x92o\\9wR\xBE5@L\x83h\xDD\xCFh\xC1\n\x91\x01a\x0F\x85V[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x1BKW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0C\x11V[`\0\x80Q` a(\xE9\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1B\x97\x91\x90a'\xAAV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1B\xD2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1B\xD7V[``\x91P[P\x91P\x91Pa\x1B\xE7\x85\x83\x83a\x1C\x0FV[\x95\x94PPPPPV[4\x15a\x0F\x9FW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x1C$Wa\x1C\x1F\x82a\x1CkV[a\x0B\xB0V[\x81Q\x15\x80\x15a\x1C;WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1CdW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x0C\x11V[P\x80a\x0B\xB0V[\x80Q\x15a\x1C{W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Ta\x1C\xA0\x90a#\x98V[`\0\x82U\x80`\x1F\x10a\x1C\xB0WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x0Ez\x91\x90[\x80\x82\x11\x15a\x1C\xDEW`\0\x81U`\x01\x01a\x1C\xCAV[P\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\xF9W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1D\x10W`\0\x80\xFD[a\x0B\xB0\x82a\x1C\xE2V[`\0` \x82\x84\x03\x12\x15a\x1D+W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B\xB0W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x1DUW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DlW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1D\x84W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x1D\x9EW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xB4W`\0\x80\xFD[a\x1D\xC0\x85\x82\x86\x01a\x1DCV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\xDFW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x1E\0W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x1E\"W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1E\nV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1EC\x81` \x86\x01` \x86\x01a\x1E\x07V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0\x82Q``` \x84\x01Ra\x1Es`\x80\x84\x01\x82a\x1E+V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra\x1E\x91\x83\x83a\x1E+V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa\x1B\xE7\x82\x82a\x1E+V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\xE7Wa\x1E\xE7a\x1E\xAFV[`@R\x90V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\xE7Wa\x1E\xE7a\x1E\xAFV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1F7Wa\x1F7a\x1E\xAFV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1FPW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FiWa\x1Fia\x1E\xAFV[a\x1F|`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1F\x0FV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1F\x91W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\xC1W`\0\x80\xFD[a\x1F\xCA\x83a\x1C\xE2V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1F\xE7W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1F\xFBW`\0\x80\xFD[\x815\x81\x81\x11\x15a \rWa \ra\x1E\xAFV[\x80`\x05\x1Ba \x1C\x85\x82\x01a\x1F\x0FV[\x91\x82R\x83\x81\x01\x85\x01\x91\x85\x81\x01\x90\x8A\x84\x11\x15a 6W`\0\x80\xFD[\x86\x86\x01\x92P[\x83\x83\x10\x15a \xF4W\x825\x85\x81\x11\x15a TW`\0\x80\x81\xFD[\x86\x01``\x81\x8D\x03`\x1F\x19\x01\x81\x13\x15a lW`\0\x80\x81\xFD[a ta\x1E\xC5V[\x89\x83\x015\x88\x81\x11\x15a \x86W`\0\x80\x81\xFD[a \x94\x8F\x8C\x83\x87\x01\x01a\x1F?V[\x82RP`@\x83\x015\x88\x81\x11\x15a \xAAW`\0\x80\x81\xFD[a \xB8\x8F\x8C\x83\x87\x01\x01a\x1F?V[\x82\x8C\x01RP\x90\x82\x015\x90\x87\x82\x11\x15a \xD0W`\0\x80\x81\xFD[a \xDE\x8E\x8B\x84\x86\x01\x01a\x1F?V[`@\x82\x01R\x84RPP\x91\x86\x01\x91\x90\x86\x01\x90a <V[\x80\x97PPPPPPPP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a!\x18W`\0\x80\xFD[\x825\x91Pa!(` \x84\x01a\x1C\xE2V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a!DW`\0\x80\xFD[a!M\x83a\x1C\xE2V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!hW`\0\x80\xFD[a!t\x85\x82\x86\x01a\x1F?V[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a!\x97W`\0\x80\xFD[a!\xA0\x87a\x1C\xE2V[\x95P` \x87\x015\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a!\xC3W`\0\x80\xFD[a!\xCF\x8A\x83\x8B\x01a\x1DCV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a!\xE8W`\0\x80\xFD[Pa!\xF5\x89\x82\x8A\x01a\x1DCV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\"\x1AW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"1W`\0\x80\xFD[a\"=\x86\x83\x87\x01a\x1F?V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\"SW`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\"gW`\0\x80\xFD[a\"oa\x1E\xEDV[\x825\x82\x81\x11\x15a\"~W`\0\x80\xFD[a\"\x8A\x88\x82\x86\x01a\x1F?V[\x82RP` \x83\x015\x82\x81\x11\x15a\"\x9FW`\0\x80\xFD[a\"\xAB\x88\x82\x86\x01a\x1F?V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\"\xC3W`\0\x80\xFD[a\"\xCF\x88\x82\x86\x01a\x1F?V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\"\xE7W`\0\x80\xFD[a\"\xF3\x88\x82\x86\x01a\x1F?V[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x81R`\0a\x0B\xB0` \x83\x01\x84a\x1E+V[`\0\x80`@\x83\x85\x03\x12\x15a#6W`\0\x80\xFD[a#?\x83a\x1C\xE2V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a#bW`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x7FW`\0\x80\xFD[a#\x8B\x86\x82\x87\x01a\x1DCV[\x94\x97\x90\x96P\x93\x94PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a#\xACW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a#\xCCWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x06\x9CW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a#\xF9WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a$\x18W\x82\x81U`\x01\x01a$\x05V[PPPPPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15a$7Wa$7a\x1E\xAFV[a$K\x83a$E\x83Ta#\x98V[\x83a#\xD2V[`\0`\x1F\x84\x11`\x01\x81\x14a$\x7FW`\0\x85\x15a$gWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua$\xD9V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a$\xB0W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a$\x90V[P\x86\x82\x10\x15a$\xCDW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a$\xF5W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%\x0CW`\0\x80\xFD[a%\x18\x87\x83\x88\x01a\x1F?V[\x94P` \x86\x015\x91P\x80\x82\x11\x15a%.W`\0\x80\xFD[a%:\x87\x83\x88\x01a\x1F?V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a%PW`\0\x80\xFD[Pa%]\x86\x82\x87\x01a\x1F?V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a%\x80W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%\x97W`\0\x80\xFD[a%\xA3\x8A\x83\x8B\x01a\x1F?V[\x97P` \x89\x015\x91P\x80\x82\x11\x15a%\xB9W`\0\x80\xFD[a%\xC5\x8A\x83\x8B\x01a\x1F?V[\x96P`@\x89\x015\x91P\x80\x82\x11\x15a%\xDBW`\0\x80\xFD[a%\xE7\x8A\x83\x8B\x01a\x1F?V[\x95P``\x89\x015\x91P\x80\x82\x11\x15a%\xFDW`\0\x80\xFD[a&\t\x8A\x83\x8B\x01a\x1F?V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15a&\x1FW`\0\x80\xFD[Pa&,\x89\x82\x8A\x01a\x1F?V[\x92PP`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a&~Wa&~a&VV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a&\x97W`\0\x80\xFD[PQ\x91\x90PV[`\0\x84Qa&\xB0\x81\x84` \x89\x01a\x1E\x07V[\x84Q\x90\x83\x01\x90a&\xC4\x81\x83` \x89\x01a\x1E\x07V[\x84Q\x91\x01\x90a&\xD7\x81\x83` \x88\x01a\x1E\x07V[\x01\x95\x94PPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x06tWa\x06ta&VV[`\0\x82a'\x11WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`@\x81R`\0a')`@\x83\x01\x85a\x1E+V[\x82\x81\x03` \x84\x01R\x83Q`\xA0\x82Ra'D`\xA0\x83\x01\x82a\x1E+V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01Ra']\x82\x82a\x1E+V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra'w\x82\x82a\x1E+V[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01Ra'\x91\x82\x82a\x1E+V[\x91PP`\x80\x85\x01Q`\x80\x83\x01R\x80\x92PPP\x93\x92PPPV[`\0\x82Qa'\xBC\x81\x84` \x87\x01a\x1E\x07V[\x91\x90\x91\x01\x92\x91PPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xDFWa'\xDFa\x1E\xAFV[a'\xF3\x81a'\xED\x84Ta#\x98V[\x84a#\xD2V[` \x80`\x1F\x83\x11`\x01\x81\x14a((W`\0\x84\x15a(\x10WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua$\x18V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a(WW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a(8V[P\x85\x82\x10\x15a(uW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[``\x81R`\0a(\x98``\x83\x01\x86a\x1E+V[\x82\x81\x03` \x84\x01Ra(\xAA\x81\x86a\x1E+V[\x90P\x82\x81\x03`@\x84\x01Ra(\xBE\x81\x85a\x1E+V[\x96\x95PPPPPPV\xFE\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x006\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xDCi|\xE6\xC0\xC3\xF8\x86\xFB\xBC\x8E\r#:\xBF\xACZ\xC56S\xAC\x08\xB3c[\xB0\xAE\xC7\x0C9\\B\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\xA2dipfsX\"\x12 \0\xBC\x02\"\xD5\xD5i\xFBY>\x18\xFC/\xB4\x18\xC5\ti\xFAmo\x05\xDB\x8E\xEAi\xCF\x0E\x1D\x18_\xB3dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ENTITYKEYREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct EntityKeyRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EntityKeyRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EntityKeyRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EntityKeyRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EntityKeyRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EntityKeyRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EntityKeyRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ENTITYKEYREGISTRY_ABI.clone(),
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
                ENTITYKEYREGISTRY_ABI.clone(),
                ENTITYKEYREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ATTESTATION_MAX_AGE` (0x9aec990e) function
        pub fn attestation_max_age(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 236, 153, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ATTESTATION_VERIFIER` (0xcd79f906) function
        pub fn attestation_verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([205, 121, 249, 6], ())
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
        ///Calls the contract's `KEY_REGISTER_ROLE` (0xb80aaa89) function
        pub fn key_register_role(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([184, 10, 170, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MODERATOR_ROLE` (0x797669c9) function
        pub fn moderator_role(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([121, 118, 105, 201], ())
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
        ///Calls the contract's `addGeneratorRegistry` (0xaa0ed09f) function
        pub fn add_generator_registry(
            &self,
            generator_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 14, 208, 159], generator_registry)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowOnlyVerifiedFamily` (0x72105aaf) function
        pub fn allow_only_verified_family(
            &self,
            family_id: [u8; 32],
            key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 16, 90, 175], (family_id, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blackListedImages` (0x72b561f9) function
        pub fn black_listed_images(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([114, 181, 97, 249], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blacklistImage` (0x88522109) function
        pub fn blacklist_image(
            &self,
            image_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 82, 33, 9], image_id)
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
        ///Calls the contract's `getVerifiedKey` (0x01d58fa3) function
        pub fn get_verified_key(
            &self,
            key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([1, 213, 143, 163], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWhitelistedImage` (0x2410f6ba) function
        pub fn get_whitelisted_image(
            &self,
            image_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, EnclaveImage> {
            self.0
                .method_hash([36, 16, 246, 186], image_id)
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
        ///Calls the contract's `initialize` (0x2eb39ee9) function
        pub fn initialize(
            &self,
            admin: ::ethers::core::types::Address,
            init_whitelist_images: ::std::vec::Vec<EnclaveImage>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 179, 158, 233], (admin, init_whitelist_images))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isImageInFamily` (0x6b5b21a6) function
        pub fn is_image_in_family(
            &self,
            image_id: [u8; 32],
            family: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([107, 91, 33, 166], (image_id, family))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pub_key` (0xe1a63130) function
        pub fn pub_key(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([225, 166, 49, 48], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeEnclaveImageFromFamily` (0x1413a92a) function
        pub fn remove_enclave_image_from_family(
            &self,
            image_id: [u8; 32],
            family: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 19, 169, 42], (image_id, family))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePubkey` (0xed380d03) function
        pub fn remove_pubkey(
            &self,
            key_owner: ::ethers::core::types::Address,
            key_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 56, 13, 3], (key_owner, key_index))
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
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePubkey` (0x69fdbcca) function
        pub fn update_pubkey(
            &self,
            key_owner: ::ethers::core::types::Address,
            key_index: ::ethers::core::types::U256,
            pubkey: ::ethers::core::types::Bytes,
            attestation_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [105, 253, 188, 202],
                    (key_owner, key_index, pubkey, attestation_data),
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
        ///Calls the contract's `verifyEnclaveKey` (0x75847b84) function
        pub fn verify_enclave_key(
            &self,
            signature: ::ethers::core::types::Bytes,
            attestation: Attestation,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([117, 132, 123, 132], (signature, attestation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyKey` (0x0707591f) function
        pub fn verify_key(
            &self,
            attestation_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 7, 89, 31], attestation_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistImageUsingPcrs` (0xff114ae1) function
        pub fn whitelist_image_using_pcrs(
            &self,
            family: [u8; 32],
            pcrs: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 17, 74, 225], (family, pcrs))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EnclaveImageAddedToFamily` event
        pub fn enclave_image_added_to_family_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveImageAddedToFamilyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveImageRemovedFromFamily` event
        pub fn enclave_image_removed_from_family_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveImageRemovedFromFamilyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveImageRevoked` event
        pub fn enclave_image_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EnclaveImageRevokedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EnclaveImageWhitelisted` event
        pub fn enclave_image_whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveImageWhitelistedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveKeyRevoked` event
        pub fn enclave_key_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EnclaveKeyRevokedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EnclaveKeyVerified` event
        pub fn enclave_key_verified_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EnclaveKeyVerifiedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EnclaveKeyWhitelisted` event
        pub fn enclave_key_whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EnclaveKeyWhitelistedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ImageBlacklisted` event
        pub fn image_blacklisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ImageBlacklistedFilter>
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
        ///Gets the contract's `RemoveKey` event
        pub fn remove_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemoveKeyFilter> {
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
        ///Gets the contract's `UpdateKey` event
        pub fn update_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpdateKeyFilter> {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EntityKeyRegistryEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for EntityKeyRegistry<M>
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
    ///Custom Error type `AttestationAutherAttestationTooOld` with signature `AttestationAutherAttestationTooOld()` and selector `0x19605e0a`
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
        name = "AttestationAutherAttestationTooOld",
        abi = "AttestationAutherAttestationTooOld()"
    )]
    pub struct AttestationAutherAttestationTooOld;
    ///Custom Error type `AttestationAutherImageNotInFamily` with signature `AttestationAutherImageNotInFamily()` and selector `0x90cc4b02`
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
        name = "AttestationAutherImageNotInFamily",
        abi = "AttestationAutherImageNotInFamily()"
    )]
    pub struct AttestationAutherImageNotInFamily;
    ///Custom Error type `AttestationAutherImageNotWhitelisted` with signature `AttestationAutherImageNotWhitelisted()` and selector `0x38c4ac16`
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
        name = "AttestationAutherImageNotWhitelisted",
        abi = "AttestationAutherImageNotWhitelisted()"
    )]
    pub struct AttestationAutherImageNotWhitelisted;
    ///Custom Error type `AttestationAutherKeyNotVerified` with signature `AttestationAutherKeyNotVerified()` and selector `0x3dd8ca95`
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
        name = "AttestationAutherKeyNotVerified",
        abi = "AttestationAutherKeyNotVerified()"
    )]
    pub struct AttestationAutherKeyNotVerified;
    ///Custom Error type `AttestationAutherMismatchedLengths` with signature `AttestationAutherMismatchedLengths()` and selector `0xd30d02c5`
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
        name = "AttestationAutherMismatchedLengths",
        abi = "AttestationAutherMismatchedLengths()"
    )]
    pub struct AttestationAutherMismatchedLengths;
    ///Custom Error type `AttestationAutherPCRsInvalid` with signature `AttestationAutherPCRsInvalid()` and selector `0x84c61bb6`
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
        name = "AttestationAutherPCRsInvalid",
        abi = "AttestationAutherPCRsInvalid()"
    )]
    pub struct AttestationAutherPCRsInvalid;
    ///Custom Error type `AttestationAutherPubkeyLengthInvalid` with signature `AttestationAutherPubkeyLengthInvalid()` and selector `0xbd9c80c1`
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
        name = "AttestationAutherPubkeyLengthInvalid",
        abi = "AttestationAutherPubkeyLengthInvalid()"
    )]
    pub struct AttestationAutherPubkeyLengthInvalid;
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
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
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
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
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
    pub enum EntityKeyRegistryErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        AddressEmptyCode(AddressEmptyCode),
        AlreadyABlacklistedImage(AlreadyABlacklistedImage),
        AttestationAutherAttestationTooOld(AttestationAutherAttestationTooOld),
        AttestationAutherImageNotInFamily(AttestationAutherImageNotInFamily),
        AttestationAutherImageNotWhitelisted(AttestationAutherImageNotWhitelisted),
        AttestationAutherKeyNotVerified(AttestationAutherKeyNotVerified),
        AttestationAutherMismatchedLengths(AttestationAutherMismatchedLengths),
        AttestationAutherPCRsInvalid(AttestationAutherPCRsInvalid),
        AttestationAutherPubkeyLengthInvalid(AttestationAutherPubkeyLengthInvalid),
        BlacklistedImage(BlacklistedImage),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedInnerCall(FailedInnerCall),
        InferredImageIdIsDifferent(InferredImageIdIsDifferent),
        InvalidEnclaveKey(InvalidEnclaveKey),
        InvalidInitialization(InvalidInitialization),
        MustBeAnEnclave(MustBeAnEnclave),
        NotInitializing(NotInitializing),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for EntityKeyRegistryErrors {
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
                <AlreadyABlacklistedImage as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AlreadyABlacklistedImage(decoded));
            }
            if let Ok(decoded) =
                <AttestationAutherAttestationTooOld as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationAutherAttestationTooOld(decoded));
            }
            if let Ok(decoded) =
                <AttestationAutherImageNotInFamily as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationAutherImageNotInFamily(decoded));
            }
            if let Ok(decoded) =
                <AttestationAutherImageNotWhitelisted as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AttestationAutherImageNotWhitelisted(decoded));
            }
            if let Ok(decoded) =
                <AttestationAutherKeyNotVerified as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationAutherKeyNotVerified(decoded));
            }
            if let Ok(decoded) =
                <AttestationAutherMismatchedLengths as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationAutherMismatchedLengths(decoded));
            }
            if let Ok(decoded) =
                <AttestationAutherPCRsInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationAutherPCRsInvalid(decoded));
            }
            if let Ok(decoded) =
                <AttestationAutherPubkeyLengthInvalid as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AttestationAutherPubkeyLengthInvalid(decoded));
            }
            if let Ok(decoded) = <BlacklistedImage as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BlacklistedImage(decoded));
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
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) =
                <InferredImageIdIsDifferent as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InferredImageIdIsDifferent(decoded));
            }
            if let Ok(decoded) = <InvalidEnclaveKey as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidEnclaveKey(decoded));
            }
            if let Ok(decoded) =
                <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <MustBeAnEnclave as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MustBeAnEnclave(decoded));
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
    impl ::ethers::core::abi::AbiEncode for EntityKeyRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AlreadyABlacklistedImage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationAutherAttestationTooOld(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationAutherImageNotInFamily(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationAutherImageNotWhitelisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationAutherKeyNotVerified(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationAutherMismatchedLengths(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationAutherPCRsInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationAutherPubkeyLengthInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlacklistedImage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC1967InvalidImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967NonPayable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedInnerCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InferredImageIdIsDifferent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEnclaveKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MustBeAnEnclave(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotInitializing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReentrancyGuardReentrantCall(element) => {
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
    impl ::ethers::contract::ContractRevert for EntityKeyRegistryErrors {
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
                    == <AlreadyABlacklistedImage as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationAutherAttestationTooOld as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationAutherImageNotInFamily as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationAutherImageNotWhitelisted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationAutherKeyNotVerified as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationAutherMismatchedLengths as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationAutherPCRsInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationAutherPubkeyLengthInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BlacklistedImage as ::ethers::contract::EthError>::selector() => {
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
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InferredImageIdIsDifferent as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidEnclaveKey as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MustBeAnEnclave as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for EntityKeyRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AlreadyABlacklistedImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationAutherAttestationTooOld(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationAutherImageNotInFamily(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationAutherImageNotWhitelisted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationAutherKeyNotVerified(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationAutherMismatchedLengths(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationAutherPCRsInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationAutherPubkeyLengthInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BlacklistedImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InferredImageIdIsDifferent(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInitialization(element) => ::core::fmt::Display::fmt(element, f),
                Self::MustBeAnEnclave(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UUPSUnauthorizedCallContext(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for EntityKeyRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation> for EntityKeyRegistryErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount> for EntityKeyRegistryErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for EntityKeyRegistryErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AlreadyABlacklistedImage> for EntityKeyRegistryErrors {
        fn from(value: AlreadyABlacklistedImage) -> Self {
            Self::AlreadyABlacklistedImage(value)
        }
    }
    impl ::core::convert::From<AttestationAutherAttestationTooOld> for EntityKeyRegistryErrors {
        fn from(value: AttestationAutherAttestationTooOld) -> Self {
            Self::AttestationAutherAttestationTooOld(value)
        }
    }
    impl ::core::convert::From<AttestationAutherImageNotInFamily> for EntityKeyRegistryErrors {
        fn from(value: AttestationAutherImageNotInFamily) -> Self {
            Self::AttestationAutherImageNotInFamily(value)
        }
    }
    impl ::core::convert::From<AttestationAutherImageNotWhitelisted> for EntityKeyRegistryErrors {
        fn from(value: AttestationAutherImageNotWhitelisted) -> Self {
            Self::AttestationAutherImageNotWhitelisted(value)
        }
    }
    impl ::core::convert::From<AttestationAutherKeyNotVerified> for EntityKeyRegistryErrors {
        fn from(value: AttestationAutherKeyNotVerified) -> Self {
            Self::AttestationAutherKeyNotVerified(value)
        }
    }
    impl ::core::convert::From<AttestationAutherMismatchedLengths> for EntityKeyRegistryErrors {
        fn from(value: AttestationAutherMismatchedLengths) -> Self {
            Self::AttestationAutherMismatchedLengths(value)
        }
    }
    impl ::core::convert::From<AttestationAutherPCRsInvalid> for EntityKeyRegistryErrors {
        fn from(value: AttestationAutherPCRsInvalid) -> Self {
            Self::AttestationAutherPCRsInvalid(value)
        }
    }
    impl ::core::convert::From<AttestationAutherPubkeyLengthInvalid> for EntityKeyRegistryErrors {
        fn from(value: AttestationAutherPubkeyLengthInvalid) -> Self {
            Self::AttestationAutherPubkeyLengthInvalid(value)
        }
    }
    impl ::core::convert::From<BlacklistedImage> for EntityKeyRegistryErrors {
        fn from(value: BlacklistedImage) -> Self {
            Self::BlacklistedImage(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for EntityKeyRegistryErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for EntityKeyRegistryErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for EntityKeyRegistryErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InferredImageIdIsDifferent> for EntityKeyRegistryErrors {
        fn from(value: InferredImageIdIsDifferent) -> Self {
            Self::InferredImageIdIsDifferent(value)
        }
    }
    impl ::core::convert::From<InvalidEnclaveKey> for EntityKeyRegistryErrors {
        fn from(value: InvalidEnclaveKey) -> Self {
            Self::InvalidEnclaveKey(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for EntityKeyRegistryErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<MustBeAnEnclave> for EntityKeyRegistryErrors {
        fn from(value: MustBeAnEnclave) -> Self {
            Self::MustBeAnEnclave(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for EntityKeyRegistryErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for EntityKeyRegistryErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for EntityKeyRegistryErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for EntityKeyRegistryErrors {
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
    #[ethevent(
        name = "EnclaveImageAddedToFamily",
        abi = "EnclaveImageAddedToFamily(bytes32,bytes32)"
    )]
    pub struct EnclaveImageAddedToFamilyFilter {
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
        pub family: [u8; 32],
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
        name = "EnclaveImageRemovedFromFamily",
        abi = "EnclaveImageRemovedFromFamily(bytes32,bytes32)"
    )]
    pub struct EnclaveImageRemovedFromFamilyFilter {
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
        pub family: [u8; 32],
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
    #[ethevent(name = "EnclaveImageRevoked", abi = "EnclaveImageRevoked(bytes32)")]
    pub struct EnclaveImageRevokedFilter {
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
    #[ethevent(
        name = "EnclaveImageWhitelisted",
        abi = "EnclaveImageWhitelisted(bytes32,bytes,bytes,bytes)"
    )]
    pub struct EnclaveImageWhitelistedFilter {
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
    #[ethevent(name = "EnclaveKeyRevoked", abi = "EnclaveKeyRevoked(bytes)")]
    pub struct EnclaveKeyRevokedFilter {
        #[ethevent(indexed)]
        pub enclave_pub_key: ::ethers::core::types::H256,
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
    #[ethevent(name = "EnclaveKeyVerified", abi = "EnclaveKeyVerified(bytes,bytes32)")]
    pub struct EnclaveKeyVerifiedFilter {
        #[ethevent(indexed)]
        pub enclave_pub_key: ::ethers::core::types::H256,
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
    #[ethevent(
        name = "EnclaveKeyWhitelisted",
        abi = "EnclaveKeyWhitelisted(bytes,bytes32)"
    )]
    pub struct EnclaveKeyWhitelistedFilter {
        #[ethevent(indexed)]
        pub enclave_pub_key: ::ethers::core::types::H256,
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
    #[ethevent(name = "ImageBlacklisted", abi = "ImageBlacklisted(bytes32)")]
    pub struct ImageBlacklistedFilter {
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
    #[ethevent(name = "RemoveKey", abi = "RemoveKey(address,uint256)")]
    pub struct RemoveKeyFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub key_index: ::ethers::core::types::U256,
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
    #[ethevent(name = "UpdateKey", abi = "UpdateKey(address,uint256)")]
    pub struct UpdateKeyFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub key_index: ::ethers::core::types::U256,
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
    pub enum EntityKeyRegistryEvents {
        EnclaveImageAddedToFamilyFilter(EnclaveImageAddedToFamilyFilter),
        EnclaveImageRemovedFromFamilyFilter(EnclaveImageRemovedFromFamilyFilter),
        EnclaveImageRevokedFilter(EnclaveImageRevokedFilter),
        EnclaveImageWhitelistedFilter(EnclaveImageWhitelistedFilter),
        EnclaveKeyRevokedFilter(EnclaveKeyRevokedFilter),
        EnclaveKeyVerifiedFilter(EnclaveKeyVerifiedFilter),
        EnclaveKeyWhitelistedFilter(EnclaveKeyWhitelistedFilter),
        ImageBlacklistedFilter(ImageBlacklistedFilter),
        InitializedFilter(InitializedFilter),
        RemoveKeyFilter(RemoveKeyFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        UpdateKeyFilter(UpdateKeyFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for EntityKeyRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EnclaveImageAddedToFamilyFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveImageAddedToFamilyFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveImageRemovedFromFamilyFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveImageRemovedFromFamilyFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageRevokedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveImageRevokedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageWhitelistedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveImageWhitelistedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveKeyRevokedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveKeyRevokedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyVerifiedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveKeyVerifiedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyWhitelistedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveKeyWhitelistedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ImageBlacklistedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::ImageBlacklistedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = RemoveKeyFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RemoveKeyFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UpdateKeyFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::UpdateKeyFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for EntityKeyRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EnclaveImageAddedToFamilyFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveImageRemovedFromFamilyFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveImageRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveImageWhitelistedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveKeyRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveKeyVerifiedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveKeyWhitelistedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ImageBlacklistedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EnclaveImageAddedToFamilyFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveImageAddedToFamilyFilter) -> Self {
            Self::EnclaveImageAddedToFamilyFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRemovedFromFamilyFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveImageRemovedFromFamilyFilter) -> Self {
            Self::EnclaveImageRemovedFromFamilyFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRevokedFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveImageRevokedFilter) -> Self {
            Self::EnclaveImageRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageWhitelistedFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveImageWhitelistedFilter) -> Self {
            Self::EnclaveImageWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyRevokedFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveKeyRevokedFilter) -> Self {
            Self::EnclaveKeyRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyVerifiedFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveKeyVerifiedFilter) -> Self {
            Self::EnclaveKeyVerifiedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyWhitelistedFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveKeyWhitelistedFilter) -> Self {
            Self::EnclaveKeyWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<ImageBlacklistedFilter> for EntityKeyRegistryEvents {
        fn from(value: ImageBlacklistedFilter) -> Self {
            Self::ImageBlacklistedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for EntityKeyRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<RemoveKeyFilter> for EntityKeyRegistryEvents {
        fn from(value: RemoveKeyFilter) -> Self {
            Self::RemoveKeyFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for EntityKeyRegistryEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for EntityKeyRegistryEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for EntityKeyRegistryEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<UpdateKeyFilter> for EntityKeyRegistryEvents {
        fn from(value: UpdateKeyFilter) -> Self {
            Self::UpdateKeyFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for EntityKeyRegistryEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ATTESTATION_MAX_AGE` function with signature `ATTESTATION_MAX_AGE()` and selector `0x9aec990e`
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
    #[ethcall(name = "ATTESTATION_MAX_AGE", abi = "ATTESTATION_MAX_AGE()")]
    pub struct AttestationMaxAgeCall;
    ///Container type for all input parameters for the `ATTESTATION_VERIFIER` function with signature `ATTESTATION_VERIFIER()` and selector `0xcd79f906`
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
    #[ethcall(name = "ATTESTATION_VERIFIER", abi = "ATTESTATION_VERIFIER()")]
    pub struct AttestationVerifierCall;
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
    ///Container type for all input parameters for the `KEY_REGISTER_ROLE` function with signature `KEY_REGISTER_ROLE()` and selector `0xb80aaa89`
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
    #[ethcall(name = "KEY_REGISTER_ROLE", abi = "KEY_REGISTER_ROLE()")]
    pub struct KeyRegisterRoleCall;
    ///Container type for all input parameters for the `MODERATOR_ROLE` function with signature `MODERATOR_ROLE()` and selector `0x797669c9`
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
    #[ethcall(name = "MODERATOR_ROLE", abi = "MODERATOR_ROLE()")]
    pub struct ModeratorRoleCall;
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
    ///Container type for all input parameters for the `addGeneratorRegistry` function with signature `addGeneratorRegistry(address)` and selector `0xaa0ed09f`
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
    #[ethcall(name = "addGeneratorRegistry", abi = "addGeneratorRegistry(address)")]
    pub struct AddGeneratorRegistryCall {
        pub generator_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allowOnlyVerifiedFamily` function with signature `allowOnlyVerifiedFamily(bytes32,address)` and selector `0x72105aaf`
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
        name = "allowOnlyVerifiedFamily",
        abi = "allowOnlyVerifiedFamily(bytes32,address)"
    )]
    pub struct AllowOnlyVerifiedFamilyCall {
        pub family_id: [u8; 32],
        pub key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `blackListedImages` function with signature `blackListedImages(bytes32)` and selector `0x72b561f9`
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
    #[ethcall(name = "blackListedImages", abi = "blackListedImages(bytes32)")]
    pub struct BlackListedImagesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `blacklistImage` function with signature `blacklistImage(bytes32)` and selector `0x88522109`
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
    #[ethcall(name = "blacklistImage", abi = "blacklistImage(bytes32)")]
    pub struct BlacklistImageCall {
        pub image_id: [u8; 32],
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
    ///Container type for all input parameters for the `getVerifiedKey` function with signature `getVerifiedKey(address)` and selector `0x01d58fa3`
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
    #[ethcall(name = "getVerifiedKey", abi = "getVerifiedKey(address)")]
    pub struct GetVerifiedKeyCall {
        pub key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getWhitelistedImage` function with signature `getWhitelistedImage(bytes32)` and selector `0x2410f6ba`
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
    #[ethcall(name = "getWhitelistedImage", abi = "getWhitelistedImage(bytes32)")]
    pub struct GetWhitelistedImageCall {
        pub image_id: [u8; 32],
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,(bytes,bytes,bytes)[])` and selector `0x2eb39ee9`
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
    #[ethcall(name = "initialize", abi = "initialize(address,(bytes,bytes,bytes)[])")]
    pub struct InitializeCall {
        pub admin: ::ethers::core::types::Address,
        pub init_whitelist_images: ::std::vec::Vec<EnclaveImage>,
    }
    ///Container type for all input parameters for the `isImageInFamily` function with signature `isImageInFamily(bytes32,bytes32)` and selector `0x6b5b21a6`
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
    #[ethcall(name = "isImageInFamily", abi = "isImageInFamily(bytes32,bytes32)")]
    pub struct IsImageInFamilyCall {
        pub image_id: [u8; 32],
        pub family: [u8; 32],
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
    ///Container type for all input parameters for the `pub_key` function with signature `pub_key(address,uint256)` and selector `0xe1a63130`
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
    #[ethcall(name = "pub_key", abi = "pub_key(address,uint256)")]
    pub struct PubKeyCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `removeEnclaveImageFromFamily` function with signature `removeEnclaveImageFromFamily(bytes32,bytes32)` and selector `0x1413a92a`
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
        name = "removeEnclaveImageFromFamily",
        abi = "removeEnclaveImageFromFamily(bytes32,bytes32)"
    )]
    pub struct RemoveEnclaveImageFromFamilyCall {
        pub image_id: [u8; 32],
        pub family: [u8; 32],
    }
    ///Container type for all input parameters for the `removePubkey` function with signature `removePubkey(address,uint256)` and selector `0xed380d03`
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
    #[ethcall(name = "removePubkey", abi = "removePubkey(address,uint256)")]
    pub struct RemovePubkeyCall {
        pub key_owner: ::ethers::core::types::Address,
        pub key_index: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `updatePubkey` function with signature `updatePubkey(address,uint256,bytes,bytes)` and selector `0x69fdbcca`
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
        name = "updatePubkey",
        abi = "updatePubkey(address,uint256,bytes,bytes)"
    )]
    pub struct UpdatePubkeyCall {
        pub key_owner: ::ethers::core::types::Address,
        pub key_index: ::ethers::core::types::U256,
        pub pubkey: ::ethers::core::types::Bytes,
        pub attestation_data: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `verifyEnclaveKey` function with signature `verifyEnclaveKey(bytes,(bytes,bytes,bytes,bytes,uint256))` and selector `0x75847b84`
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
        name = "verifyEnclaveKey",
        abi = "verifyEnclaveKey(bytes,(bytes,bytes,bytes,bytes,uint256))"
    )]
    pub struct VerifyEnclaveKeyCall {
        pub signature: ::ethers::core::types::Bytes,
        pub attestation: Attestation,
    }
    ///Container type for all input parameters for the `verifyKey` function with signature `verifyKey(bytes)` and selector `0x0707591f`
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
    #[ethcall(name = "verifyKey", abi = "verifyKey(bytes)")]
    pub struct VerifyKeyCall {
        pub attestation_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `whitelistImageUsingPcrs` function with signature `whitelistImageUsingPcrs(bytes32,bytes)` and selector `0xff114ae1`
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
        name = "whitelistImageUsingPcrs",
        abi = "whitelistImageUsingPcrs(bytes32,bytes)"
    )]
    pub struct WhitelistImageUsingPcrsCall {
        pub family: [u8; 32],
        pub pcrs: ::ethers::core::types::Bytes,
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
    pub enum EntityKeyRegistryCalls {
        AttestationMaxAge(AttestationMaxAgeCall),
        AttestationVerifier(AttestationVerifierCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        KeyRegisterRole(KeyRegisterRoleCall),
        ModeratorRole(ModeratorRoleCall),
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        AddGeneratorRegistry(AddGeneratorRegistryCall),
        AllowOnlyVerifiedFamily(AllowOnlyVerifiedFamilyCall),
        BlackListedImages(BlackListedImagesCall),
        BlacklistImage(BlacklistImageCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetVerifiedKey(GetVerifiedKeyCall),
        GetWhitelistedImage(GetWhitelistedImageCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Initialize(InitializeCall),
        IsImageInFamily(IsImageInFamilyCall),
        ProxiableUUID(ProxiableUUIDCall),
        PubKey(PubKeyCall),
        RemoveEnclaveImageFromFamily(RemoveEnclaveImageFromFamilyCall),
        RemovePubkey(RemovePubkeyCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SupportsInterface(SupportsInterfaceCall),
        UpdatePubkey(UpdatePubkeyCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        VerifyEnclaveKey(VerifyEnclaveKeyCall),
        VerifyKey(VerifyKeyCall),
        WhitelistImageUsingPcrs(WhitelistImageUsingPcrsCall),
    }
    impl ::ethers::core::abi::AbiDecode for EntityKeyRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AttestationMaxAgeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationMaxAge(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationVerifier(decoded));
            }
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <KeyRegisterRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::KeyRegisterRole(decoded));
            }
            if let Ok(decoded) = <ModeratorRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ModeratorRole(decoded));
            }
            if let Ok(decoded) =
                <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) =
                <AddGeneratorRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddGeneratorRegistry(decoded));
            }
            if let Ok(decoded) =
                <AllowOnlyVerifiedFamilyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllowOnlyVerifiedFamily(decoded));
            }
            if let Ok(decoded) =
                <BlackListedImagesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BlackListedImages(decoded));
            }
            if let Ok(decoded) =
                <BlacklistImageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BlacklistImage(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetVerifiedKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetVerifiedKey(decoded));
            }
            if let Ok(decoded) =
                <GetWhitelistedImageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetWhitelistedImage(decoded));
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
                <IsImageInFamilyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsImageInFamily(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <PubKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PubKey(decoded));
            }
            if let Ok(decoded) =
                <RemoveEnclaveImageFromFamilyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveEnclaveImageFromFamily(decoded));
            }
            if let Ok(decoded) = <RemovePubkeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemovePubkey(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UpdatePubkeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdatePubkey(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) =
                <VerifyEnclaveKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyEnclaveKey(decoded));
            }
            if let Ok(decoded) = <VerifyKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VerifyKey(decoded));
            }
            if let Ok(decoded) =
                <WhitelistImageUsingPcrsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WhitelistImageUsingPcrs(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EntityKeyRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AttestationMaxAge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AttestationVerifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::KeyRegisterRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ModeratorRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddGeneratorRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllowOnlyVerifiedFamily(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlackListedImages(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BlacklistImage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRoleAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetVerifiedKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetWhitelistedImage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsImageInFamily(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PubKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveEnclaveImageFromFamily(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovePubkey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdatePubkey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeToAndCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyEnclaveKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WhitelistImageUsingPcrs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for EntityKeyRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestationMaxAge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::KeyRegisterRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModeratorRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeInterfaceVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddGeneratorRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowOnlyVerifiedFamily(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlackListedImages(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlacklistImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVerifiedKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWhitelistedImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsImageInFamily(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveEnclaveImageFromFamily(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemovePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistImageUsingPcrs(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestationMaxAgeCall> for EntityKeyRegistryCalls {
        fn from(value: AttestationMaxAgeCall) -> Self {
            Self::AttestationMaxAge(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierCall> for EntityKeyRegistryCalls {
        fn from(value: AttestationVerifierCall) -> Self {
            Self::AttestationVerifier(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for EntityKeyRegistryCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<KeyRegisterRoleCall> for EntityKeyRegistryCalls {
        fn from(value: KeyRegisterRoleCall) -> Self {
            Self::KeyRegisterRole(value)
        }
    }
    impl ::core::convert::From<ModeratorRoleCall> for EntityKeyRegistryCalls {
        fn from(value: ModeratorRoleCall) -> Self {
            Self::ModeratorRole(value)
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for EntityKeyRegistryCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<AddGeneratorRegistryCall> for EntityKeyRegistryCalls {
        fn from(value: AddGeneratorRegistryCall) -> Self {
            Self::AddGeneratorRegistry(value)
        }
    }
    impl ::core::convert::From<AllowOnlyVerifiedFamilyCall> for EntityKeyRegistryCalls {
        fn from(value: AllowOnlyVerifiedFamilyCall) -> Self {
            Self::AllowOnlyVerifiedFamily(value)
        }
    }
    impl ::core::convert::From<BlackListedImagesCall> for EntityKeyRegistryCalls {
        fn from(value: BlackListedImagesCall) -> Self {
            Self::BlackListedImages(value)
        }
    }
    impl ::core::convert::From<BlacklistImageCall> for EntityKeyRegistryCalls {
        fn from(value: BlacklistImageCall) -> Self {
            Self::BlacklistImage(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for EntityKeyRegistryCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetVerifiedKeyCall> for EntityKeyRegistryCalls {
        fn from(value: GetVerifiedKeyCall) -> Self {
            Self::GetVerifiedKey(value)
        }
    }
    impl ::core::convert::From<GetWhitelistedImageCall> for EntityKeyRegistryCalls {
        fn from(value: GetWhitelistedImageCall) -> Self {
            Self::GetWhitelistedImage(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for EntityKeyRegistryCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for EntityKeyRegistryCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for EntityKeyRegistryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsImageInFamilyCall> for EntityKeyRegistryCalls {
        fn from(value: IsImageInFamilyCall) -> Self {
            Self::IsImageInFamily(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for EntityKeyRegistryCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<PubKeyCall> for EntityKeyRegistryCalls {
        fn from(value: PubKeyCall) -> Self {
            Self::PubKey(value)
        }
    }
    impl ::core::convert::From<RemoveEnclaveImageFromFamilyCall> for EntityKeyRegistryCalls {
        fn from(value: RemoveEnclaveImageFromFamilyCall) -> Self {
            Self::RemoveEnclaveImageFromFamily(value)
        }
    }
    impl ::core::convert::From<RemovePubkeyCall> for EntityKeyRegistryCalls {
        fn from(value: RemovePubkeyCall) -> Self {
            Self::RemovePubkey(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for EntityKeyRegistryCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for EntityKeyRegistryCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for EntityKeyRegistryCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UpdatePubkeyCall> for EntityKeyRegistryCalls {
        fn from(value: UpdatePubkeyCall) -> Self {
            Self::UpdatePubkey(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for EntityKeyRegistryCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VerifyEnclaveKeyCall> for EntityKeyRegistryCalls {
        fn from(value: VerifyEnclaveKeyCall) -> Self {
            Self::VerifyEnclaveKey(value)
        }
    }
    impl ::core::convert::From<VerifyKeyCall> for EntityKeyRegistryCalls {
        fn from(value: VerifyKeyCall) -> Self {
            Self::VerifyKey(value)
        }
    }
    impl ::core::convert::From<WhitelistImageUsingPcrsCall> for EntityKeyRegistryCalls {
        fn from(value: WhitelistImageUsingPcrsCall) -> Self {
            Self::WhitelistImageUsingPcrs(value)
        }
    }
    ///Container type for all return fields from the `ATTESTATION_MAX_AGE` function with signature `ATTESTATION_MAX_AGE()` and selector `0x9aec990e`
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
    pub struct AttestationMaxAgeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ATTESTATION_VERIFIER` function with signature `ATTESTATION_VERIFIER()` and selector `0xcd79f906`
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
    ///Container type for all return fields from the `KEY_REGISTER_ROLE` function with signature `KEY_REGISTER_ROLE()` and selector `0xb80aaa89`
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
    pub struct KeyRegisterRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MODERATOR_ROLE` function with signature `MODERATOR_ROLE()` and selector `0x797669c9`
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
    pub struct ModeratorRoleReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `blackListedImages` function with signature `blackListedImages(bytes32)` and selector `0x72b561f9`
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
    pub struct BlackListedImagesReturn(pub bool);
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
    ///Container type for all return fields from the `getVerifiedKey` function with signature `getVerifiedKey(address)` and selector `0x01d58fa3`
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
    pub struct GetVerifiedKeyReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getWhitelistedImage` function with signature `getWhitelistedImage(bytes32)` and selector `0x2410f6ba`
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
    pub struct GetWhitelistedImageReturn(pub EnclaveImage);
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
    ///Container type for all return fields from the `isImageInFamily` function with signature `isImageInFamily(bytes32,bytes32)` and selector `0x6b5b21a6`
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
    pub struct IsImageInFamilyReturn(pub bool);
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
    ///Container type for all return fields from the `pub_key` function with signature `pub_key(address,uint256)` and selector `0xe1a63130`
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
    pub struct PubKeyReturn(pub ::ethers::core::types::Bytes);
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
    ///Container type for all return fields from the `verifyEnclaveKey` function with signature `verifyEnclaveKey(bytes,(bytes,bytes,bytes,bytes,uint256))` and selector `0x75847b84`
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
    pub struct VerifyEnclaveKeyReturn(pub bool);
}
