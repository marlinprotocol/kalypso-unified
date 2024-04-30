pub use attestation_verifier::*;
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
pub mod attestation_verifier {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("MAX_AGE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_AGE"),
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
                                name: ::std::borrow::ToOwned::to_owned("images"),
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
                                        "struct AttestationVerifier.EnclaveImage[]",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("enclaveKeys"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_admin"),
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
                    ::std::borrow::ToOwned::to_owned("pubKeyToAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pubKeyToAddress"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pubKey"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("revokeEnclaveImage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("revokeEnclaveImage"),
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
                    ::std::borrow::ToOwned::to_owned("revokeEnclaveKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("revokeEnclaveKey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
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
                    ::std::borrow::ToOwned::to_owned("verifiedKeys"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifiedKeys"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAttestationVerifier.Attestation",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("whitelistEnclaveImage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("whitelistEnclaveImage",),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("whitelistEnclaveKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("whitelistEnclaveKey",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("whitelistedImages"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("whitelistedImages"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
            ]),
            events: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierAttestationTooOld"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationVerifierAttestationTooOld",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierCannotRemoveAllAdmins"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationVerifierCannotRemoveAllAdmins",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierImageAlreadyWhitelisted"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationVerifierImageAlreadyWhitelisted",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierImageNotWhitelisted"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationVerifierImageNotWhitelisted",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierInitLengthMismatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationVerifierInitLengthMismatch",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierInvalidAdmin"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AttestationVerifierInvalidAdmin",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierKeyAlreadyVerified"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationVerifierKeyAlreadyVerified",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierKeyNotVerified"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AttestationVerifierKeyNotVerified",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierNoImageProvided"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationVerifierNoImageProvided",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierPCRsInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AttestationVerifierPCRsInvalid",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AttestationVerifierPubkeyLengthInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "AttestationVerifierPubkeyLengthInvalid",
                        ),
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
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
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
    pub static ATTESTATIONVERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15a\0\x14W`\0\x80\xFD[Pa\0\x1Da\0\"V[a\0\xD4V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\0rW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a\0\xD1W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Qa#\xA6b\0\0\xFE`\09`\0\x81\x81a\x0BH\x01R\x81\x81a\x0Bq\x01Ra\x0C\xBC\x01Ra#\xA6`\0\xF3\xFE`\x80`@R`\x046\x10a\x015W`\x005`\xE0\x1C\x80c\x8Ev\n\xFE\x11a\0\xABW\x80c\xC6Z\x91j\x11a\0oW\x80c\xC6Z\x91j\x14a\x03wW\x80c\xD17\x14\xB4\x14a\x03\x97W\x80c\xD5Gt\x1F\x14a\x03\xB7W\x80c\xDA\x99L\xD8\x14a\x03\xD7W\x80c\xDE\xBFrk\x14a\x04\x05W\x80c\xEA\xC7\x08\xA3\x14a\x04%W`\0\x80\xFD[\x80c\x8Ev\n\xFE\x14a\x02\xC4W\x80c\x91\xD1HT\x14a\x02\xE4W\x80c\x95\xC6\x02\x94\x14a\x03\x04W\x80c\xA2\x17\xFD\xDF\x14a\x03$W\x80c\xAD<\xB1\xCC\x14a\x039W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\0\xFDW\x80c6V\x8A\xBE\x14a\x02\x04W\x80cO\x1E\xF2\x86\x14a\x02$W\x80cR\xD1\x90-\x14a\x027W\x80ci\x81^W\x14a\x02LW\x80cp\xAE\x99*\x14a\x02lW\x80cu\x84{\x84\x14a\x02\xA4W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01:W\x80c\r\xCA\xEA\xF2\x14a\x01oW\x80c$\x8A\x9C\xA3\x14a\x01\x93W\x80c//\xF1]\x14a\x01\xB3W\x80c/\x9B\n\xD7\x14a\x01\xD5W[`\0\x80\xFD[4\x80\x15a\x01FW`\0\x80\xFD[Pa\x01Za\x01U6`\x04a\x18\xD7V[a\x04EV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01{W`\0\x80\xFD[Pa\x01\x85a\x01,\x81V[`@Q\x90\x81R` \x01a\x01fV[4\x80\x15a\x01\x9FW`\0\x80\xFD[Pa\x01\x85a\x01\xAE6`\x04a\x19\x01V[a\x04VV[4\x80\x15a\x01\xBFW`\0\x80\xFD[Pa\x01\xD3a\x01\xCE6`\x04a\x196V[a\x04xV[\0[4\x80\x15a\x01\xE1W`\0\x80\xFD[Pa\x01\xF5a\x01\xF06`\x04a\x19\x01V[a\x04\x9AV[`@Qa\x01f\x93\x92\x91\x90a\x19\xB2V[4\x80\x15a\x02\x10W`\0\x80\xFD[Pa\x01\xD3a\x02\x1F6`\x04a\x196V[a\x06UV[a\x01\xD3a\x0226`\x04a\x1B\x02V[a\x06\x8DV[4\x80\x15a\x02CW`\0\x80\xFD[Pa\x01\x85a\x06\xACV[4\x80\x15a\x02XW`\0\x80\xFD[Pa\x01\xD3a\x02g6`\x04a\x1BOV[a\x06\xC9V[4\x80\x15a\x02xW`\0\x80\xFD[Pa\x02\x8Ca\x02\x876`\x04a\x1B\xD6V[a\x06\xFEV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01fV[4\x80\x15a\x02\xB0W`\0\x80\xFD[Pa\x01\xD3a\x02\xBF6`\x04a\x1C\x12V[a\x07\tV[4\x80\x15a\x02\xD0W`\0\x80\xFD[Pa\x01\xD3a\x02\xDF6`\x04a\x1B\xD6V[a\x07\x13V[4\x80\x15a\x02\xF0W`\0\x80\xFD[Pa\x01Za\x02\xFF6`\x04a\x196V[a\x078V[4\x80\x15a\x03\x10W`\0\x80\xFD[Pa\x01\xD3a\x03\x1F6`\x04a\x19\x01V[a\x07pV[4\x80\x15a\x030W`\0\x80\xFD[Pa\x01\x85`\0\x81V[4\x80\x15a\x03EW`\0\x80\xFD[Pa\x03j`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01f\x91\x90a\x1D\x1BV[4\x80\x15a\x03\x83W`\0\x80\xFD[Pa\x01\xD3a\x03\x926`\x04a\x1D\xDBV[a\x07\x84V[4\x80\x15a\x03\xA3W`\0\x80\xFD[Pa\x01\xD3a\x03\xB26`\x04a\x1B\xD6V[a\t\x8BV[4\x80\x15a\x03\xC3W`\0\x80\xFD[Pa\x01\xD3a\x03\xD26`\x04a\x196V[a\t\x9FV[4\x80\x15a\x03\xE3W`\0\x80\xFD[Pa\x01\x85a\x03\xF26`\x04a\x1F<V[a\x01\xF5` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04\x11W`\0\x80\xFD[Pa\x01\xD3a\x04 6`\x04a\x1FWV[a\t\xBBV[4\x80\x15a\x041W`\0\x80\xFD[Pa\x01\xD3a\x04@6`\x04a\x1C\x12V[a\t\xD0V[`\0a\x04P\x82a\t\xDAV[\x92\x91PPV[`\0\x90\x81R`\0\x80Q` a#Q\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[a\x04\x81\x82a\x04VV[a\x04\x8A\x81a\n\x0FV[a\x04\x94\x83\x83a\n\x1CV[PPPPV[a\x01\xF4` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x04\xB6\x90a\x1F\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xE2\x90a\x1F\x9BV[\x80\x15a\x05/W\x80`\x1F\x10a\x05\x04Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05/V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x12W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x05D\x90a\x1F\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05p\x90a\x1F\x9BV[\x80\x15a\x05\xBDW\x80`\x1F\x10a\x05\x92Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xBDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xA0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x05\xD2\x90a\x1F\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xFE\x90a\x1F\x9BV[\x80\x15a\x06KW\x80`\x1F\x10a\x06 Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06KV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06.W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x06~W`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\x88\x82\x82a\n\xC1V[PPPV[a\x06\x95a\x0B=V[a\x06\x9E\x82a\x0B\xE4V[a\x06\xA8\x82\x82a\x0B\xEFV[PPV[`\0a\x06\xB6a\x0C\xB1V[P`\0\x80Q` a#1\x839\x81Q\x91R\x90V[`\0a\x06\xD4\x81a\n\x0FV[a\x06\xF7`@Q\x80``\x01`@R\x80\x86\x81R` \x01\x85\x81R` \x01\x84\x81RPa\x0C\xFAV[PPPPPV[`\0a\x04P\x82a\x0ErV[a\x06\xA8\x82\x82a\x0E\xA2V[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x07*\x91\x90a \x1AV[\x91P\x91Pa\x06\x88\x82\x82a\x10\x0EV[`\0\x91\x82R`\0\x80Q` a#Q\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x07{\x81a\n\x0FV[a\x06\xA8\x82a\x12\0V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\x07\xC9WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x07\xE5WP0;\x15[\x90P\x81\x15\x80\x15a\x07\xF3WP\x80\x15[\x15a\x08\x11W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x08;W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[\x87Q`\0\x03a\x08]W`@Qc\xB6\x0F\"\xAF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86Q\x88Q\x14a\x08\x7FW`@QcF\xD6\x0BW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x86\x16a\x08\xA6W`@Qc\x1F`\xF3\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xAEa\x12\xA2V[a\x08\xB6a\x12\xA2V[a\x08\xBEa\x12\xA2V[a\x08\xC6a\x12\xA2V[a\x08\xD1`\0\x87a\n\x1CV[P`\0[\x87Q\x81\x10\x15a\t:W`\0a\t\x02\x8A\x83\x81Q\x81\x10a\x08\xF5Wa\x08\xF5a!#V[` \x02` \x01\x01Qa\x0C\xFAV[\x90Pa\t'\x89\x83\x81Q\x81\x10a\t\x19Wa\t\x19a!#V[` \x02` \x01\x01Q\x82a\x12\xAAV[P\x80a\t2\x81a!OV[\x91PPa\x08\xD5V[P\x83\x15a\t\x81W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[`\0a\t\x96\x81a\n\x0FV[a\x06\xA8\x82a\x13\x8DV[a\t\xA8\x82a\x04VV[a\t\xB1\x81a\n\x0FV[a\x04\x94\x83\x83a\n\xC1V[`\0a\t\xC6\x81a\n\x0FV[a\x06\x88\x83\x83a\x12\xAAV[a\x06\xA8\x82\x82a\x10\x0EV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04PWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x04PV[a\n\x19\x813a\x140V[PV[`\0`\0\x80Q` a#Q\x839\x81Q\x91Ra\n7\x84\x84a\x078V[a\n\xB7W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\nm3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\x04PV[`\0\x91PPa\x04PV[`\0`\0\x80Q` a#Q\x839\x81Q\x91Ra\n\xDC\x84\x84a\x078V[\x15a\n\xB7W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\x04PV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0B\xC4WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x0B\xB8`\0\x80Q` a#1\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0B\xE2W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0a\x06\xA8\x81a\n\x0FV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x0CIWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x0CF\x91\x81\x01\x90a!hV[`\x01[a\x0CvW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80Q` a#1\x839\x81Q\x91R\x81\x14a\x0C\xA7W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0CmV[a\x06\x88\x83\x83a\x14iV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0B\xE2W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80QQ`\0\x90`0\x14\x80\x15a\r\x14WP\x81` \x01QQ`0\x14[\x80\x15a\r%WP\x81`@\x01QQ`0\x14[a\rBW`@Qc\x977P\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q`@Q` \x01a\re\x93\x92\x91\x90a!\x81V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81Ra\x01\xF4\x90\x93R\x91 \x80T\x91\x92P\x90a\r\x99\x90a\x1F\x9BV[\x15\x90Pa\r\xB9W`@Qc\x1A3\x94i`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q``\x81\x01\x82R\x84Q\x81R` \x80\x86\x01Q\x81\x83\x01R\x85\x83\x01Q\x82\x84\x01R`\0\x84\x81Ra\x01\xF4\x90\x91R\x91\x90\x91 \x81Q\x81\x90a\r\xF6\x90\x82a\"\nV[P` \x82\x01Q`\x01\x82\x01\x90a\x0E\x0B\x90\x82a\"\nV[P`@\x82\x01Q`\x02\x82\x01\x90a\x0E \x90\x82a\"\nV[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q`@Qa\x0Ed\x93\x92\x91\x90a\x19\xB2V[`@Q\x80\x91\x03\x90\xA2\x92\x91PPV[`\0\x81Q`@\x14a\x0E\x96W`@Qc\x123\xAD\xCD`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[a\x0E\xAEa\x01,Ba\"\xC9V[a\x03\xE8\x82`\x80\x01Qa\x0E\xC0\x91\x90a\"\xDCV[\x11a\x0E\xDEW`@Qci\xBB\x1F\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81` \x01Q\x82`@\x01Q\x83``\x01Q`@Q` \x01a\x0F\x01\x93\x92\x91\x90a!\x81V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81Ra\x01\xF4\x90\x93R\x91 \x80T\x91\x92P\x90a\x0F5\x90a\x1F\x9BV[\x90P`\0\x03a\x0FWW`@Qcf\x08\x07\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0Ff\x83`\0\x01Qa\x06\xFEV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90 T\x90\x91P\x15a\x0F\xA1W`@Qc!u\x1C\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xAB\x84\x84a\x10\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90\x81\x90 \x83\x90U\x83Q\x90Q\x83\x91a\x0F\xD8\x91a\"\xFEV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF4\x83\x7F\x1Af\xA1\xE5\xFB\xDE,@\xD0\xF6\xEF\xF7Q\x83\x13\x9Ey\xB0\xFC\x1E\xAA\xB2\x80\xA3\xA0\xBE8\xE7\xB1\x90`\0\x90\xA3PPPPV[`\0\x7Fh\x89\xDFGl\xA3\x8F?KA|\x17\xEBIf\x82\xEB@\x1BOA\xA2%\x97A\xA7\x8A\xCCH\x1E\xA8\x05\x82`\0\x01Q\x80Q\x90` \x01 \x83` \x01Q\x80Q\x90` \x01 \x84`@\x01Q\x80Q\x90` \x01 \x85``\x01Q\x80Q\x90` \x01 \x86`\x80\x01Q`@Q` \x01a\x10\x9E\x96\x95\x94\x93\x92\x91\x90\x95\x86R` \x86\x01\x94\x90\x94R`@\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x7F\xB09HDc4\xEB\x9B!\x96\xD5\xEB\x16oi\xB9\xD4\x94\x03\xEBJ\x12\xF3m\xE8\xD3\xF9\xF3\xCB\x8E\x15\xC3\x82\x85\x01R\x7F\xDD\xD5H\x1F\x81\x8A\xBBX\x93\xC8Re\x0Bu\xBD\x1F\x15T\x9C\xD1\xC0#1\xDB~\xD0\x10\xFCa\xE6Nr\x84\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x80\x86\x01\x91\x90\x91R\x83Q\x80\x86\x03\x90\x91\x01\x81R`\x80\x85\x01\x84R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1B`\xA0\x86\x01R`\xA2\x85\x01R`\xC2\x80\x85\x01\x82\x90R\x83Q\x80\x86\x03\x90\x91\x01\x81R`\xE2\x90\x94\x01\x90\x92R\x82Q\x92\x01\x91\x90\x91 \x90\x91P`\0a\x11\x81\x82\x86a\x14\xBFV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90 T\x90\x91P\x80a\x11\xBCW`@Qc\x0B\xFD\x93G`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81Ra\x01\xF4` R`@\x90 \x80Ta\x11\xD6\x90a\x1F\x9BV[\x90P`\0\x03a\x11\xF8W`@Qcf\x08\x07\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0\x81\x81Ra\x01\xF4` R`@\x90 \x80Ta\x12\x1A\x90a\x1F\x9BV[\x90P`\0\x03a\x12<W`@Qcf\x08\x07\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81Ra\x01\xF4` R`@\x81 \x90a\x12V\x82\x82a\x18\x89V[a\x12d`\x01\x83\x01`\0a\x18\x89V[a\x12r`\x02\x83\x01`\0a\x18\x89V[PP`@Q\x81\x90\x7FKq\xBE\xDD\xA4!yf\xEA\xC1\xAAI\xE3[\x15P\xE5\xAB\x87\x06J\xE1y\xD1\xB3k9;\xD0N\xB7C\x90`\0\x90\xA2PV[a\x0B\xE2a\x14\xE9V[`\0\x81\x81Ra\x01\xF4` R`@\x90 \x80Ta\x12\xC4\x90a\x1F\x9BV[\x90P`\0\x03a\x12\xE6W`@Qcf\x08\x07\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x12\xF1\x83a\x0ErV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90 T\x90\x91P\x15a\x13,W`@Qc!u\x1C\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90\x81\x90 \x83\x90UQ\x82\x90a\x13X\x90\x85\x90a\"\xFEV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\x03\xE7\xF4\xEF\xA4\xEB\xC4J\xE1\xB5\x12T\xC9\x98\x96y7\xE7b\xB8\xEFx\xA5\xED\xD1P\x1A$\x03\xF6\xF3\x87\x90`\0\x90\xA3PPPV[`\0a\x13\x98\x82a\x0ErV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90 T\x90\x91Pa\x13\xD2W`@Qc\x0B\xFD\x93G`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x80\x82 \x91\x90\x91UQa\x13\xFC\x90\x83\x90a\"\xFEV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF9\x10\xFDx\xB9\x8E\x95t\xE8\xB6\xF0\xE4+\xC3\xA9\x96\xA6\xAB\xA3wn\xC2\x0B\x9D\x85Q[\xD7Jl\x8F\xE9\x90`\0\x90\xA2PPV[a\x14:\x82\x82a\x078V[a\x06\xA8W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x0CmV[a\x14r\x82a\x152V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\x14\xB7Wa\x06\x88\x82\x82a\x15\x97V[a\x06\xA8a\x16\rV[`\0\x80`\0\x80a\x14\xCF\x86\x86a\x16,V[\x92P\x92P\x92Pa\x14\xDF\x82\x82a\x16yV[P\x90\x94\x93PPPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x0B\xE2W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x15hW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0CmV[`\0\x80Q` a#1\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x15\xB4\x91\x90a\"\xFEV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x15\xEFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x15\xF4V[``\x91P[P\x91P\x91Pa\x16\x04\x85\x83\x83a\x172V[\x95\x94PPPPPV[4\x15a\x0B\xE2W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x83Q`A\x03a\x16fW` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa\x16X\x88\x82\x85\x85a\x17\x91V[\x95P\x95P\x95PPPPa\x16rV[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a\x16\x8DWa\x16\x8Da#\x1AV[\x03a\x16\x96WPPV[`\x01\x82`\x03\x81\x11\x15a\x16\xAAWa\x16\xAAa#\x1AV[\x03a\x16\xC8W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x16\xDCWa\x16\xDCa#\x1AV[\x03a\x16\xFDW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0CmV[`\x03\x82`\x03\x81\x11\x15a\x17\x11Wa\x17\x11a#\x1AV[\x03a\x06\xA8W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0CmV[``\x82a\x17GWa\x17B\x82a\x18`V[a\x17\x8AV[\x81Q\x15\x80\x15a\x17^WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x17\x87W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x0CmV[P\x80[\x93\x92PPPV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x17\xCCWP`\0\x91P`\x03\x90P\x82a\x18VV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x18 W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18LWP`\0\x92P`\x01\x91P\x82\x90Pa\x18VV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[\x80Q\x15a\x18pW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Ta\x18\x95\x90a\x1F\x9BV[`\0\x82U\x80`\x1F\x10a\x18\xA5WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\n\x19\x91\x90[\x80\x82\x11\x15a\x18\xD3W`\0\x81U`\x01\x01a\x18\xBFV[P\x90V[`\0` \x82\x84\x03\x12\x15a\x18\xE9W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x17\x8AW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19\x13W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x191W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x19IW`\0\x80\xFD[\x825\x91Pa\x19Y` \x84\x01a\x19\x1AV[\x90P\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x19}W\x81\x81\x01Q\x83\x82\x01R` \x01a\x19eV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x19\x9E\x81` \x86\x01` \x86\x01a\x19bV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81R`\0a\x19\xC5``\x83\x01\x86a\x19\x86V[\x82\x81\x03` \x84\x01Ra\x19\xD7\x81\x86a\x19\x86V[\x90P\x82\x81\x03`@\x84\x01Ra\x19\xEB\x81\x85a\x19\x86V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1A-Wa\x1A-a\x19\xF5V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1A-Wa\x1A-a\x19\xF5V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1A}Wa\x1A}a\x19\xF5V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1A\x9EWa\x1A\x9Ea\x19\xF5V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1A\xBDW`\0\x80\xFD[\x815a\x1A\xD0a\x1A\xCB\x82a\x1A\x85V[a\x1AUV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1A\xE5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\x15W`\0\x80\xFD[a\x1B\x1E\x83a\x19\x1AV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B9W`\0\x80\xFD[a\x1BE\x85\x82\x86\x01a\x1A\xACV[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1BdW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1B{W`\0\x80\xFD[a\x1B\x87\x87\x83\x88\x01a\x1A\xACV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a\x1B\x9DW`\0\x80\xFD[a\x1B\xA9\x87\x83\x88\x01a\x1A\xACV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x1B\xBFW`\0\x80\xFD[Pa\x1B\xCC\x86\x82\x87\x01a\x1A\xACV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1B\xE8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xFEW`\0\x80\xFD[a\x1C\n\x84\x82\x85\x01a\x1A\xACV[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C%W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1C<W`\0\x80\xFD[a\x1CH\x86\x83\x87\x01a\x1A\xACV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x1C^W`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\x1CrW`\0\x80\xFD[a\x1Cza\x1A\x0BV[\x825\x82\x81\x11\x15a\x1C\x89W`\0\x80\xFD[a\x1C\x95\x88\x82\x86\x01a\x1A\xACV[\x82RP` \x83\x015\x82\x81\x11\x15a\x1C\xAAW`\0\x80\xFD[a\x1C\xB6\x88\x82\x86\x01a\x1A\xACV[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x1C\xCEW`\0\x80\xFD[a\x1C\xDA\x88\x82\x86\x01a\x1A\xACV[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\x1C\xF2W`\0\x80\xFD[a\x1C\xFE\x88\x82\x86\x01a\x1A\xACV[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x81R`\0a\x17\x8A` \x83\x01\x84a\x19\x86V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1DGWa\x1DGa\x19\xF5V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1DbW`\0\x80\xFD[\x815` a\x1Dra\x1A\xCB\x83a\x1D.V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1D\x91W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1D\xD0W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xB4W`\0\x80\x81\xFD[a\x1D\xC2\x89\x86\x83\x8B\x01\x01a\x1A\xACV[\x84RP\x91\x83\x01\x91\x83\x01a\x1D\x95V[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1D\xF0W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1E\x07W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1E\x1BW`\0\x80\xFD[\x815` a\x1E+a\x1A\xCB\x83a\x1D.V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a\x1EJW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1F\0W\x805\x86\x81\x11\x15a\x1EeW`\0\x80\xFD[\x87\x01``\x81\x8E\x03`\x1F\x19\x01\x12\x15a\x1E{W`\0\x80\xFD[a\x1E\x83a\x1A3V[\x85\x82\x015\x88\x81\x11\x15a\x1E\x94W`\0\x80\xFD[a\x1E\xA2\x8F\x88\x83\x86\x01\x01a\x1A\xACV[\x82RP`@\x82\x015\x88\x81\x11\x15a\x1E\xB8W`\0\x80\x81\xFD[a\x1E\xC6\x8F\x88\x83\x86\x01\x01a\x1A\xACV[\x87\x83\x01RP``\x82\x015\x88\x81\x11\x15a\x1E\xDEW`\0\x80\x81\xFD[a\x1E\xEC\x8F\x88\x83\x86\x01\x01a\x1A\xACV[`@\x83\x01RP\x84RP\x91\x83\x01\x91\x83\x01a\x1ENV[P\x97PP\x87\x015\x92PP\x80\x82\x11\x15a\x1F\x17W`\0\x80\xFD[Pa\x1F$\x86\x82\x87\x01a\x1DQV[\x92PPa\x1F3`@\x85\x01a\x19\x1AV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1FNW`\0\x80\xFD[a\x17\x8A\x82a\x19\x1AV[`\0\x80`@\x83\x85\x03\x12\x15a\x1FjW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x80W`\0\x80\xFD[a\x1F\x8C\x85\x82\x86\x01a\x1A\xACV[\x95` \x94\x90\x94\x015\x94PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1F\xAFW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1F\xCFWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1F\xE6W`\0\x80\xFD[\x81Qa\x1F\xF4a\x1A\xCB\x82a\x1A\x85V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a \tW`\0\x80\xFD[a\x1C\n\x82` \x83\x01` \x87\x01a\x19bV[`\0\x80`@\x83\x85\x03\x12\x15a -W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a DW`\0\x80\xFD[a P\x86\x83\x87\x01a\x1F\xD5V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a fW`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a zW`\0\x80\xFD[a \x82a\x1A\x0BV[\x82Q\x82\x81\x11\x15a \x91W`\0\x80\xFD[a \x9D\x88\x82\x86\x01a\x1F\xD5V[\x82RP` \x83\x01Q\x82\x81\x11\x15a \xB2W`\0\x80\xFD[a \xBE\x88\x82\x86\x01a\x1F\xD5V[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a \xD6W`\0\x80\xFD[a \xE2\x88\x82\x86\x01a\x1F\xD5V[`@\x83\x01RP``\x83\x01Q\x82\x81\x11\x15a \xFAW`\0\x80\xFD[a!\x06\x88\x82\x86\x01a\x1F\xD5V[``\x83\x01RP`\x80\x83\x01Q`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a!aWa!aa!9V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a!zW`\0\x80\xFD[PQ\x91\x90PV[`\0\x84Qa!\x93\x81\x84` \x89\x01a\x19bV[\x84Q\x90\x83\x01\x90a!\xA7\x81\x83` \x89\x01a\x19bV[\x84Q\x91\x01\x90a!\xBA\x81\x83` \x88\x01a\x19bV[\x01\x95\x94PPPPPV[`\x1F\x82\x11\x15a\x06\x88W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a!\xEBWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x11\xF8W\x82\x81U`\x01\x01a!\xF7V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"#Wa\"#a\x19\xF5V[a\"7\x81a\"1\x84Ta\x1F\x9BV[\x84a!\xC4V[` \x80`\x1F\x83\x11`\x01\x81\x14a\"lW`\0\x84\x15a\"TWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x11\xF8V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\"\x9BW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\"|V[P\x85\x82\x10\x15a\"\xB9W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03\x81\x81\x11\x15a\x04PWa\x04Pa!9V[`\0\x82a\"\xF9WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x82Qa#\x10\x81\x84` \x87\x01a\x19bV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\xA2dipfsX\"\x12 \x88\x0F\x82\xD0\x186O\xC5\xCAHMz\x8D\x8DUw`g8\xF9\x86Z-\xE4Q\t\xDA\xFE\\u\x06\x89dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ATTESTATIONVERIFIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x015W`\x005`\xE0\x1C\x80c\x8Ev\n\xFE\x11a\0\xABW\x80c\xC6Z\x91j\x11a\0oW\x80c\xC6Z\x91j\x14a\x03wW\x80c\xD17\x14\xB4\x14a\x03\x97W\x80c\xD5Gt\x1F\x14a\x03\xB7W\x80c\xDA\x99L\xD8\x14a\x03\xD7W\x80c\xDE\xBFrk\x14a\x04\x05W\x80c\xEA\xC7\x08\xA3\x14a\x04%W`\0\x80\xFD[\x80c\x8Ev\n\xFE\x14a\x02\xC4W\x80c\x91\xD1HT\x14a\x02\xE4W\x80c\x95\xC6\x02\x94\x14a\x03\x04W\x80c\xA2\x17\xFD\xDF\x14a\x03$W\x80c\xAD<\xB1\xCC\x14a\x039W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\0\xFDW\x80c6V\x8A\xBE\x14a\x02\x04W\x80cO\x1E\xF2\x86\x14a\x02$W\x80cR\xD1\x90-\x14a\x027W\x80ci\x81^W\x14a\x02LW\x80cp\xAE\x99*\x14a\x02lW\x80cu\x84{\x84\x14a\x02\xA4W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01:W\x80c\r\xCA\xEA\xF2\x14a\x01oW\x80c$\x8A\x9C\xA3\x14a\x01\x93W\x80c//\xF1]\x14a\x01\xB3W\x80c/\x9B\n\xD7\x14a\x01\xD5W[`\0\x80\xFD[4\x80\x15a\x01FW`\0\x80\xFD[Pa\x01Za\x01U6`\x04a\x18\xD7V[a\x04EV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01{W`\0\x80\xFD[Pa\x01\x85a\x01,\x81V[`@Q\x90\x81R` \x01a\x01fV[4\x80\x15a\x01\x9FW`\0\x80\xFD[Pa\x01\x85a\x01\xAE6`\x04a\x19\x01V[a\x04VV[4\x80\x15a\x01\xBFW`\0\x80\xFD[Pa\x01\xD3a\x01\xCE6`\x04a\x196V[a\x04xV[\0[4\x80\x15a\x01\xE1W`\0\x80\xFD[Pa\x01\xF5a\x01\xF06`\x04a\x19\x01V[a\x04\x9AV[`@Qa\x01f\x93\x92\x91\x90a\x19\xB2V[4\x80\x15a\x02\x10W`\0\x80\xFD[Pa\x01\xD3a\x02\x1F6`\x04a\x196V[a\x06UV[a\x01\xD3a\x0226`\x04a\x1B\x02V[a\x06\x8DV[4\x80\x15a\x02CW`\0\x80\xFD[Pa\x01\x85a\x06\xACV[4\x80\x15a\x02XW`\0\x80\xFD[Pa\x01\xD3a\x02g6`\x04a\x1BOV[a\x06\xC9V[4\x80\x15a\x02xW`\0\x80\xFD[Pa\x02\x8Ca\x02\x876`\x04a\x1B\xD6V[a\x06\xFEV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01fV[4\x80\x15a\x02\xB0W`\0\x80\xFD[Pa\x01\xD3a\x02\xBF6`\x04a\x1C\x12V[a\x07\tV[4\x80\x15a\x02\xD0W`\0\x80\xFD[Pa\x01\xD3a\x02\xDF6`\x04a\x1B\xD6V[a\x07\x13V[4\x80\x15a\x02\xF0W`\0\x80\xFD[Pa\x01Za\x02\xFF6`\x04a\x196V[a\x078V[4\x80\x15a\x03\x10W`\0\x80\xFD[Pa\x01\xD3a\x03\x1F6`\x04a\x19\x01V[a\x07pV[4\x80\x15a\x030W`\0\x80\xFD[Pa\x01\x85`\0\x81V[4\x80\x15a\x03EW`\0\x80\xFD[Pa\x03j`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01f\x91\x90a\x1D\x1BV[4\x80\x15a\x03\x83W`\0\x80\xFD[Pa\x01\xD3a\x03\x926`\x04a\x1D\xDBV[a\x07\x84V[4\x80\x15a\x03\xA3W`\0\x80\xFD[Pa\x01\xD3a\x03\xB26`\x04a\x1B\xD6V[a\t\x8BV[4\x80\x15a\x03\xC3W`\0\x80\xFD[Pa\x01\xD3a\x03\xD26`\x04a\x196V[a\t\x9FV[4\x80\x15a\x03\xE3W`\0\x80\xFD[Pa\x01\x85a\x03\xF26`\x04a\x1F<V[a\x01\xF5` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04\x11W`\0\x80\xFD[Pa\x01\xD3a\x04 6`\x04a\x1FWV[a\t\xBBV[4\x80\x15a\x041W`\0\x80\xFD[Pa\x01\xD3a\x04@6`\x04a\x1C\x12V[a\t\xD0V[`\0a\x04P\x82a\t\xDAV[\x92\x91PPV[`\0\x90\x81R`\0\x80Q` a#Q\x839\x81Q\x91R` R`@\x90 `\x01\x01T\x90V[a\x04\x81\x82a\x04VV[a\x04\x8A\x81a\n\x0FV[a\x04\x94\x83\x83a\n\x1CV[PPPPV[a\x01\xF4` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x04\xB6\x90a\x1F\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xE2\x90a\x1F\x9BV[\x80\x15a\x05/W\x80`\x1F\x10a\x05\x04Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05/V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x12W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x05D\x90a\x1F\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05p\x90a\x1F\x9BV[\x80\x15a\x05\xBDW\x80`\x1F\x10a\x05\x92Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xBDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xA0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x05\xD2\x90a\x1F\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xFE\x90a\x1F\x9BV[\x80\x15a\x06KW\x80`\x1F\x10a\x06 Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06KV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06.W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x06~W`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\x88\x82\x82a\n\xC1V[PPPV[a\x06\x95a\x0B=V[a\x06\x9E\x82a\x0B\xE4V[a\x06\xA8\x82\x82a\x0B\xEFV[PPV[`\0a\x06\xB6a\x0C\xB1V[P`\0\x80Q` a#1\x839\x81Q\x91R\x90V[`\0a\x06\xD4\x81a\n\x0FV[a\x06\xF7`@Q\x80``\x01`@R\x80\x86\x81R` \x01\x85\x81R` \x01\x84\x81RPa\x0C\xFAV[PPPPPV[`\0a\x04P\x82a\x0ErV[a\x06\xA8\x82\x82a\x0E\xA2V[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x07*\x91\x90a \x1AV[\x91P\x91Pa\x06\x88\x82\x82a\x10\x0EV[`\0\x91\x82R`\0\x80Q` a#Q\x839\x81Q\x91R` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x07{\x81a\n\x0FV[a\x06\xA8\x82a\x12\0V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\x07\xC9WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x07\xE5WP0;\x15[\x90P\x81\x15\x80\x15a\x07\xF3WP\x80\x15[\x15a\x08\x11W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x08;W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[\x87Q`\0\x03a\x08]W`@Qc\xB6\x0F\"\xAF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86Q\x88Q\x14a\x08\x7FW`@QcF\xD6\x0BW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x86\x16a\x08\xA6W`@Qc\x1F`\xF3\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xAEa\x12\xA2V[a\x08\xB6a\x12\xA2V[a\x08\xBEa\x12\xA2V[a\x08\xC6a\x12\xA2V[a\x08\xD1`\0\x87a\n\x1CV[P`\0[\x87Q\x81\x10\x15a\t:W`\0a\t\x02\x8A\x83\x81Q\x81\x10a\x08\xF5Wa\x08\xF5a!#V[` \x02` \x01\x01Qa\x0C\xFAV[\x90Pa\t'\x89\x83\x81Q\x81\x10a\t\x19Wa\t\x19a!#V[` \x02` \x01\x01Q\x82a\x12\xAAV[P\x80a\t2\x81a!OV[\x91PPa\x08\xD5V[P\x83\x15a\t\x81W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[`\0a\t\x96\x81a\n\x0FV[a\x06\xA8\x82a\x13\x8DV[a\t\xA8\x82a\x04VV[a\t\xB1\x81a\n\x0FV[a\x04\x94\x83\x83a\n\xC1V[`\0a\t\xC6\x81a\n\x0FV[a\x06\x88\x83\x83a\x12\xAAV[a\x06\xA8\x82\x82a\x10\x0EV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04PWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x04PV[a\n\x19\x813a\x140V[PV[`\0`\0\x80Q` a#Q\x839\x81Q\x91Ra\n7\x84\x84a\x078V[a\n\xB7W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\nm3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x91PPa\x04PV[`\0\x91PPa\x04PV[`\0`\0\x80Q` a#Q\x839\x81Q\x91Ra\n\xDC\x84\x84a\x078V[\x15a\n\xB7W`\0\x84\x81R` \x82\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x87\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4`\x01\x91PPa\x04PV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0B\xC4WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x0B\xB8`\0\x80Q` a#1\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0B\xE2W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0a\x06\xA8\x81a\n\x0FV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x0CIWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x0CF\x91\x81\x01\x90a!hV[`\x01[a\x0CvW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80Q` a#1\x839\x81Q\x91R\x81\x14a\x0C\xA7W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0CmV[a\x06\x88\x83\x83a\x14iV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0B\xE2W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80QQ`\0\x90`0\x14\x80\x15a\r\x14WP\x81` \x01QQ`0\x14[\x80\x15a\r%WP\x81`@\x01QQ`0\x14[a\rBW`@Qc\x977P\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q`@Q` \x01a\re\x93\x92\x91\x90a!\x81V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81Ra\x01\xF4\x90\x93R\x91 \x80T\x91\x92P\x90a\r\x99\x90a\x1F\x9BV[\x15\x90Pa\r\xB9W`@Qc\x1A3\x94i`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q``\x81\x01\x82R\x84Q\x81R` \x80\x86\x01Q\x81\x83\x01R\x85\x83\x01Q\x82\x84\x01R`\0\x84\x81Ra\x01\xF4\x90\x91R\x91\x90\x91 \x81Q\x81\x90a\r\xF6\x90\x82a\"\nV[P` \x82\x01Q`\x01\x82\x01\x90a\x0E\x0B\x90\x82a\"\nV[P`@\x82\x01Q`\x02\x82\x01\x90a\x0E \x90\x82a\"\nV[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q`@Qa\x0Ed\x93\x92\x91\x90a\x19\xB2V[`@Q\x80\x91\x03\x90\xA2\x92\x91PPV[`\0\x81Q`@\x14a\x0E\x96W`@Qc\x123\xAD\xCD`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[a\x0E\xAEa\x01,Ba\"\xC9V[a\x03\xE8\x82`\x80\x01Qa\x0E\xC0\x91\x90a\"\xDCV[\x11a\x0E\xDEW`@Qci\xBB\x1F\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81` \x01Q\x82`@\x01Q\x83``\x01Q`@Q` \x01a\x0F\x01\x93\x92\x91\x90a!\x81V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81Ra\x01\xF4\x90\x93R\x91 \x80T\x91\x92P\x90a\x0F5\x90a\x1F\x9BV[\x90P`\0\x03a\x0FWW`@Qcf\x08\x07\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0Ff\x83`\0\x01Qa\x06\xFEV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90 T\x90\x91P\x15a\x0F\xA1W`@Qc!u\x1C\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xAB\x84\x84a\x10\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90\x81\x90 \x83\x90U\x83Q\x90Q\x83\x91a\x0F\xD8\x91a\"\xFEV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF4\x83\x7F\x1Af\xA1\xE5\xFB\xDE,@\xD0\xF6\xEF\xF7Q\x83\x13\x9Ey\xB0\xFC\x1E\xAA\xB2\x80\xA3\xA0\xBE8\xE7\xB1\x90`\0\x90\xA3PPPPV[`\0\x7Fh\x89\xDFGl\xA3\x8F?KA|\x17\xEBIf\x82\xEB@\x1BOA\xA2%\x97A\xA7\x8A\xCCH\x1E\xA8\x05\x82`\0\x01Q\x80Q\x90` \x01 \x83` \x01Q\x80Q\x90` \x01 \x84`@\x01Q\x80Q\x90` \x01 \x85``\x01Q\x80Q\x90` \x01 \x86`\x80\x01Q`@Q` \x01a\x10\x9E\x96\x95\x94\x93\x92\x91\x90\x95\x86R` \x86\x01\x94\x90\x94R`@\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x7F\xB09HDc4\xEB\x9B!\x96\xD5\xEB\x16oi\xB9\xD4\x94\x03\xEBJ\x12\xF3m\xE8\xD3\xF9\xF3\xCB\x8E\x15\xC3\x82\x85\x01R\x7F\xDD\xD5H\x1F\x81\x8A\xBBX\x93\xC8Re\x0Bu\xBD\x1F\x15T\x9C\xD1\xC0#1\xDB~\xD0\x10\xFCa\xE6Nr\x84\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x80\x86\x01\x91\x90\x91R\x83Q\x80\x86\x03\x90\x91\x01\x81R`\x80\x85\x01\x84R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1B`\xA0\x86\x01R`\xA2\x85\x01R`\xC2\x80\x85\x01\x82\x90R\x83Q\x80\x86\x03\x90\x91\x01\x81R`\xE2\x90\x94\x01\x90\x92R\x82Q\x92\x01\x91\x90\x91 \x90\x91P`\0a\x11\x81\x82\x86a\x14\xBFV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90 T\x90\x91P\x80a\x11\xBCW`@Qc\x0B\xFD\x93G`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81Ra\x01\xF4` R`@\x90 \x80Ta\x11\xD6\x90a\x1F\x9BV[\x90P`\0\x03a\x11\xF8W`@Qcf\x08\x07\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0\x81\x81Ra\x01\xF4` R`@\x90 \x80Ta\x12\x1A\x90a\x1F\x9BV[\x90P`\0\x03a\x12<W`@Qcf\x08\x07\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81Ra\x01\xF4` R`@\x81 \x90a\x12V\x82\x82a\x18\x89V[a\x12d`\x01\x83\x01`\0a\x18\x89V[a\x12r`\x02\x83\x01`\0a\x18\x89V[PP`@Q\x81\x90\x7FKq\xBE\xDD\xA4!yf\xEA\xC1\xAAI\xE3[\x15P\xE5\xAB\x87\x06J\xE1y\xD1\xB3k9;\xD0N\xB7C\x90`\0\x90\xA2PV[a\x0B\xE2a\x14\xE9V[`\0\x81\x81Ra\x01\xF4` R`@\x90 \x80Ta\x12\xC4\x90a\x1F\x9BV[\x90P`\0\x03a\x12\xE6W`@Qcf\x08\x07\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x12\xF1\x83a\x0ErV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90 T\x90\x91P\x15a\x13,W`@Qc!u\x1C\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90\x81\x90 \x83\x90UQ\x82\x90a\x13X\x90\x85\x90a\"\xFEV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\x03\xE7\xF4\xEF\xA4\xEB\xC4J\xE1\xB5\x12T\xC9\x98\x96y7\xE7b\xB8\xEFx\xA5\xED\xD1P\x1A$\x03\xF6\xF3\x87\x90`\0\x90\xA3PPPV[`\0a\x13\x98\x82a\x0ErV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x90 T\x90\x91Pa\x13\xD2W`@Qc\x0B\xFD\x93G`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01\xF5` R`@\x80\x82 \x91\x90\x91UQa\x13\xFC\x90\x83\x90a\"\xFEV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF9\x10\xFDx\xB9\x8E\x95t\xE8\xB6\xF0\xE4+\xC3\xA9\x96\xA6\xAB\xA3wn\xC2\x0B\x9D\x85Q[\xD7Jl\x8F\xE9\x90`\0\x90\xA2PPV[a\x14:\x82\x82a\x078V[a\x06\xA8W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x0CmV[a\x14r\x82a\x152V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\x14\xB7Wa\x06\x88\x82\x82a\x15\x97V[a\x06\xA8a\x16\rV[`\0\x80`\0\x80a\x14\xCF\x86\x86a\x16,V[\x92P\x92P\x92Pa\x14\xDF\x82\x82a\x16yV[P\x90\x94\x93PPPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x0B\xE2W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x15hW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0CmV[`\0\x80Q` a#1\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x15\xB4\x91\x90a\"\xFEV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x15\xEFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x15\xF4V[``\x91P[P\x91P\x91Pa\x16\x04\x85\x83\x83a\x172V[\x95\x94PPPPPV[4\x15a\x0B\xE2W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x83Q`A\x03a\x16fW` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa\x16X\x88\x82\x85\x85a\x17\x91V[\x95P\x95P\x95PPPPa\x16rV[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a\x16\x8DWa\x16\x8Da#\x1AV[\x03a\x16\x96WPPV[`\x01\x82`\x03\x81\x11\x15a\x16\xAAWa\x16\xAAa#\x1AV[\x03a\x16\xC8W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x16\xDCWa\x16\xDCa#\x1AV[\x03a\x16\xFDW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0CmV[`\x03\x82`\x03\x81\x11\x15a\x17\x11Wa\x17\x11a#\x1AV[\x03a\x06\xA8W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0CmV[``\x82a\x17GWa\x17B\x82a\x18`V[a\x17\x8AV[\x81Q\x15\x80\x15a\x17^WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x17\x87W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x0CmV[P\x80[\x93\x92PPPV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x17\xCCWP`\0\x91P`\x03\x90P\x82a\x18VV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x18 W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18LWP`\0\x92P`\x01\x91P\x82\x90Pa\x18VV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[\x80Q\x15a\x18pW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Ta\x18\x95\x90a\x1F\x9BV[`\0\x82U\x80`\x1F\x10a\x18\xA5WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\n\x19\x91\x90[\x80\x82\x11\x15a\x18\xD3W`\0\x81U`\x01\x01a\x18\xBFV[P\x90V[`\0` \x82\x84\x03\x12\x15a\x18\xE9W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x17\x8AW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19\x13W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x191W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x19IW`\0\x80\xFD[\x825\x91Pa\x19Y` \x84\x01a\x19\x1AV[\x90P\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x19}W\x81\x81\x01Q\x83\x82\x01R` \x01a\x19eV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x19\x9E\x81` \x86\x01` \x86\x01a\x19bV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81R`\0a\x19\xC5``\x83\x01\x86a\x19\x86V[\x82\x81\x03` \x84\x01Ra\x19\xD7\x81\x86a\x19\x86V[\x90P\x82\x81\x03`@\x84\x01Ra\x19\xEB\x81\x85a\x19\x86V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1A-Wa\x1A-a\x19\xF5V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1A-Wa\x1A-a\x19\xF5V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1A}Wa\x1A}a\x19\xF5V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1A\x9EWa\x1A\x9Ea\x19\xF5V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1A\xBDW`\0\x80\xFD[\x815a\x1A\xD0a\x1A\xCB\x82a\x1A\x85V[a\x1AUV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1A\xE5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\x15W`\0\x80\xFD[a\x1B\x1E\x83a\x19\x1AV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B9W`\0\x80\xFD[a\x1BE\x85\x82\x86\x01a\x1A\xACV[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1BdW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1B{W`\0\x80\xFD[a\x1B\x87\x87\x83\x88\x01a\x1A\xACV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a\x1B\x9DW`\0\x80\xFD[a\x1B\xA9\x87\x83\x88\x01a\x1A\xACV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x1B\xBFW`\0\x80\xFD[Pa\x1B\xCC\x86\x82\x87\x01a\x1A\xACV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1B\xE8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xFEW`\0\x80\xFD[a\x1C\n\x84\x82\x85\x01a\x1A\xACV[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C%W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1C<W`\0\x80\xFD[a\x1CH\x86\x83\x87\x01a\x1A\xACV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x1C^W`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\x1CrW`\0\x80\xFD[a\x1Cza\x1A\x0BV[\x825\x82\x81\x11\x15a\x1C\x89W`\0\x80\xFD[a\x1C\x95\x88\x82\x86\x01a\x1A\xACV[\x82RP` \x83\x015\x82\x81\x11\x15a\x1C\xAAW`\0\x80\xFD[a\x1C\xB6\x88\x82\x86\x01a\x1A\xACV[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x1C\xCEW`\0\x80\xFD[a\x1C\xDA\x88\x82\x86\x01a\x1A\xACV[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\x1C\xF2W`\0\x80\xFD[a\x1C\xFE\x88\x82\x86\x01a\x1A\xACV[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x81R`\0a\x17\x8A` \x83\x01\x84a\x19\x86V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1DGWa\x1DGa\x19\xF5V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1DbW`\0\x80\xFD[\x815` a\x1Dra\x1A\xCB\x83a\x1D.V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1D\x91W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1D\xD0W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xB4W`\0\x80\x81\xFD[a\x1D\xC2\x89\x86\x83\x8B\x01\x01a\x1A\xACV[\x84RP\x91\x83\x01\x91\x83\x01a\x1D\x95V[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1D\xF0W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1E\x07W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1E\x1BW`\0\x80\xFD[\x815` a\x1E+a\x1A\xCB\x83a\x1D.V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a\x1EJW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1F\0W\x805\x86\x81\x11\x15a\x1EeW`\0\x80\xFD[\x87\x01``\x81\x8E\x03`\x1F\x19\x01\x12\x15a\x1E{W`\0\x80\xFD[a\x1E\x83a\x1A3V[\x85\x82\x015\x88\x81\x11\x15a\x1E\x94W`\0\x80\xFD[a\x1E\xA2\x8F\x88\x83\x86\x01\x01a\x1A\xACV[\x82RP`@\x82\x015\x88\x81\x11\x15a\x1E\xB8W`\0\x80\x81\xFD[a\x1E\xC6\x8F\x88\x83\x86\x01\x01a\x1A\xACV[\x87\x83\x01RP``\x82\x015\x88\x81\x11\x15a\x1E\xDEW`\0\x80\x81\xFD[a\x1E\xEC\x8F\x88\x83\x86\x01\x01a\x1A\xACV[`@\x83\x01RP\x84RP\x91\x83\x01\x91\x83\x01a\x1ENV[P\x97PP\x87\x015\x92PP\x80\x82\x11\x15a\x1F\x17W`\0\x80\xFD[Pa\x1F$\x86\x82\x87\x01a\x1DQV[\x92PPa\x1F3`@\x85\x01a\x19\x1AV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1FNW`\0\x80\xFD[a\x17\x8A\x82a\x19\x1AV[`\0\x80`@\x83\x85\x03\x12\x15a\x1FjW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x80W`\0\x80\xFD[a\x1F\x8C\x85\x82\x86\x01a\x1A\xACV[\x95` \x94\x90\x94\x015\x94PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1F\xAFW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1F\xCFWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1F\xE6W`\0\x80\xFD[\x81Qa\x1F\xF4a\x1A\xCB\x82a\x1A\x85V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a \tW`\0\x80\xFD[a\x1C\n\x82` \x83\x01` \x87\x01a\x19bV[`\0\x80`@\x83\x85\x03\x12\x15a -W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a DW`\0\x80\xFD[a P\x86\x83\x87\x01a\x1F\xD5V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a fW`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a zW`\0\x80\xFD[a \x82a\x1A\x0BV[\x82Q\x82\x81\x11\x15a \x91W`\0\x80\xFD[a \x9D\x88\x82\x86\x01a\x1F\xD5V[\x82RP` \x83\x01Q\x82\x81\x11\x15a \xB2W`\0\x80\xFD[a \xBE\x88\x82\x86\x01a\x1F\xD5V[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a \xD6W`\0\x80\xFD[a \xE2\x88\x82\x86\x01a\x1F\xD5V[`@\x83\x01RP``\x83\x01Q\x82\x81\x11\x15a \xFAW`\0\x80\xFD[a!\x06\x88\x82\x86\x01a\x1F\xD5V[``\x83\x01RP`\x80\x83\x01Q`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a!aWa!aa!9V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a!zW`\0\x80\xFD[PQ\x91\x90PV[`\0\x84Qa!\x93\x81\x84` \x89\x01a\x19bV[\x84Q\x90\x83\x01\x90a!\xA7\x81\x83` \x89\x01a\x19bV[\x84Q\x91\x01\x90a!\xBA\x81\x83` \x88\x01a\x19bV[\x01\x95\x94PPPPPV[`\x1F\x82\x11\x15a\x06\x88W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a!\xEBWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x11\xF8W\x82\x81U`\x01\x01a!\xF7V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"#Wa\"#a\x19\xF5V[a\"7\x81a\"1\x84Ta\x1F\x9BV[\x84a!\xC4V[` \x80`\x1F\x83\x11`\x01\x81\x14a\"lW`\0\x84\x15a\"TWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x11\xF8V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\"\x9BW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\"|V[P\x85\x82\x10\x15a\"\xB9W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03\x81\x81\x11\x15a\x04PWa\x04Pa!9V[`\0\x82a\"\xF9WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x82Qa#\x10\x81\x84` \x87\x01a\x19bV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\xA2dipfsX\"\x12 \x88\x0F\x82\xD0\x186O\xC5\xCAHMz\x8D\x8DUw`g8\xF9\x86Z-\xE4Q\t\xDA\xFE\\u\x06\x89dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ATTESTATIONVERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct AttestationVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AttestationVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AttestationVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AttestationVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AttestationVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AttestationVerifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AttestationVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ATTESTATIONVERIFIER_ABI.clone(),
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
                ATTESTATIONVERIFIER_ABI.clone(),
                ATTESTATIONVERIFIER_BYTECODE.clone().into(),
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
        ///Calls the contract's `MAX_AGE` (0x0dcaeaf2) function
        pub fn max_age(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([13, 202, 234, 242], ())
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
        ///Calls the contract's `initialize` (0xc65a916a) function
        pub fn initialize(
            &self,
            images: ::std::vec::Vec<EnclaveImage>,
            enclave_keys: ::std::vec::Vec<::ethers::core::types::Bytes>,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 90, 145, 106], (images, enclave_keys, admin))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pubKeyToAddress` (0x70ae992a) function
        pub fn pub_key_to_address(
            &self,
            pub_key: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([112, 174, 153, 42], pub_key)
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
        ///Calls the contract's `revokeEnclaveImage` (0x95c60294) function
        pub fn revoke_enclave_image(
            &self,
            image_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 198, 2, 148], image_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeEnclaveKey` (0xd13714b4) function
        pub fn revoke_enclave_key(
            &self,
            enclave_pub_key: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 55, 20, 180], enclave_pub_key)
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
        ///Calls the contract's `verifiedKeys` (0xda994cd8) function
        pub fn verified_keys(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([218, 153, 76, 216], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0x8e760afe) function
        pub fn verify(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 118, 10, 254], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0xeac708a3) function
        pub fn verify_with_signature(
            &self,
            signature: ::ethers::core::types::Bytes,
            attestation: Attestation,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 199, 8, 163], (signature, attestation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyEnclaveKey` (0x75847b84) function
        pub fn verify_enclave_key(
            &self,
            signature: ::ethers::core::types::Bytes,
            attestation: Attestation,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 132, 123, 132], (signature, attestation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistEnclaveImage` (0x69815e57) function
        pub fn whitelist_enclave_image(
            &self,
            pcr0: ::ethers::core::types::Bytes,
            pcr1: ::ethers::core::types::Bytes,
            pcr2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 129, 94, 87], (pcr0, pcr1, pcr2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistEnclaveKey` (0xdebf726b) function
        pub fn whitelist_enclave_key(
            &self,
            enclave_pub_key: ::ethers::core::types::Bytes,
            image_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 191, 114, 107], (enclave_pub_key, image_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistedImages` (0x2f9b0ad7) function
        pub fn whitelisted_images(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Bytes,
                ::ethers::core::types::Bytes,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([47, 155, 10, 215], p0)
                .expect("method not found (this should never happen)")
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
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
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
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AttestationVerifierEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for AttestationVerifier<M>
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
    ///Custom Error type `AttestationVerifierAttestationTooOld` with signature `AttestationVerifierAttestationTooOld()` and selector `0x69bb1fdd`
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
        name = "AttestationVerifierAttestationTooOld",
        abi = "AttestationVerifierAttestationTooOld()"
    )]
    pub struct AttestationVerifierAttestationTooOld;
    ///Custom Error type `AttestationVerifierCannotRemoveAllAdmins` with signature `AttestationVerifierCannotRemoveAllAdmins()` and selector `0x86421fd4`
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
        name = "AttestationVerifierCannotRemoveAllAdmins",
        abi = "AttestationVerifierCannotRemoveAllAdmins()"
    )]
    pub struct AttestationVerifierCannotRemoveAllAdmins;
    ///Custom Error type `AttestationVerifierImageAlreadyWhitelisted` with signature `AttestationVerifierImageAlreadyWhitelisted()` and selector `0xd19ca348`
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
        name = "AttestationVerifierImageAlreadyWhitelisted",
        abi = "AttestationVerifierImageAlreadyWhitelisted()"
    )]
    pub struct AttestationVerifierImageAlreadyWhitelisted;
    ///Custom Error type `AttestationVerifierImageNotWhitelisted` with signature `AttestationVerifierImageNotWhitelisted()` and selector `0x66080711`
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
        name = "AttestationVerifierImageNotWhitelisted",
        abi = "AttestationVerifierImageNotWhitelisted()"
    )]
    pub struct AttestationVerifierImageNotWhitelisted;
    ///Custom Error type `AttestationVerifierInitLengthMismatch` with signature `AttestationVerifierInitLengthMismatch()` and selector `0x46d60b57`
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
        name = "AttestationVerifierInitLengthMismatch",
        abi = "AttestationVerifierInitLengthMismatch()"
    )]
    pub struct AttestationVerifierInitLengthMismatch;
    ///Custom Error type `AttestationVerifierInvalidAdmin` with signature `AttestationVerifierInvalidAdmin()` and selector `0xfb079cf8`
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
        name = "AttestationVerifierInvalidAdmin",
        abi = "AttestationVerifierInvalidAdmin()"
    )]
    pub struct AttestationVerifierInvalidAdmin;
    ///Custom Error type `AttestationVerifierKeyAlreadyVerified` with signature `AttestationVerifierKeyAlreadyVerified()` and selector `0x21751cfb`
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
        name = "AttestationVerifierKeyAlreadyVerified",
        abi = "AttestationVerifierKeyAlreadyVerified()"
    )]
    pub struct AttestationVerifierKeyAlreadyVerified;
    ///Custom Error type `AttestationVerifierKeyNotVerified` with signature `AttestationVerifierKeyNotVerified()` and selector `0x17fb268e`
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
        name = "AttestationVerifierKeyNotVerified",
        abi = "AttestationVerifierKeyNotVerified()"
    )]
    pub struct AttestationVerifierKeyNotVerified;
    ///Custom Error type `AttestationVerifierNoImageProvided` with signature `AttestationVerifierNoImageProvided()` and selector `0xb60f22af`
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
        name = "AttestationVerifierNoImageProvided",
        abi = "AttestationVerifierNoImageProvided()"
    )]
    pub struct AttestationVerifierNoImageProvided;
    ///Custom Error type `AttestationVerifierPCRsInvalid` with signature `AttestationVerifierPCRsInvalid()` and selector `0x973750ff`
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
        name = "AttestationVerifierPCRsInvalid",
        abi = "AttestationVerifierPCRsInvalid()"
    )]
    pub struct AttestationVerifierPCRsInvalid;
    ///Custom Error type `AttestationVerifierPubkeyLengthInvalid` with signature `AttestationVerifierPubkeyLengthInvalid()` and selector `0x48ceb734`
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
        name = "AttestationVerifierPubkeyLengthInvalid",
        abi = "AttestationVerifierPubkeyLengthInvalid()"
    )]
    pub struct AttestationVerifierPubkeyLengthInvalid;
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
    pub enum AttestationVerifierErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        AddressEmptyCode(AddressEmptyCode),
        AttestationVerifierAttestationTooOld(AttestationVerifierAttestationTooOld),
        AttestationVerifierCannotRemoveAllAdmins(AttestationVerifierCannotRemoveAllAdmins),
        AttestationVerifierImageAlreadyWhitelisted(AttestationVerifierImageAlreadyWhitelisted),
        AttestationVerifierImageNotWhitelisted(AttestationVerifierImageNotWhitelisted),
        AttestationVerifierInitLengthMismatch(AttestationVerifierInitLengthMismatch),
        AttestationVerifierInvalidAdmin(AttestationVerifierInvalidAdmin),
        AttestationVerifierKeyAlreadyVerified(AttestationVerifierKeyAlreadyVerified),
        AttestationVerifierKeyNotVerified(AttestationVerifierKeyNotVerified),
        AttestationVerifierNoImageProvided(AttestationVerifierNoImageProvided),
        AttestationVerifierPCRsInvalid(AttestationVerifierPCRsInvalid),
        AttestationVerifierPubkeyLengthInvalid(AttestationVerifierPubkeyLengthInvalid),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedInnerCall(FailedInnerCall),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationVerifierErrors {
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
                <AttestationVerifierAttestationTooOld as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AttestationVerifierAttestationTooOld(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierCannotRemoveAllAdmins as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AttestationVerifierCannotRemoveAllAdmins(decoded));
            }
            if let Ok(decoded) = <AttestationVerifierImageAlreadyWhitelisted as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AttestationVerifierImageAlreadyWhitelisted(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierImageNotWhitelisted as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AttestationVerifierImageNotWhitelisted(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierInitLengthMismatch as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AttestationVerifierInitLengthMismatch(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierInvalidAdmin as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationVerifierInvalidAdmin(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierKeyAlreadyVerified as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AttestationVerifierKeyAlreadyVerified(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierKeyNotVerified as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationVerifierKeyNotVerified(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierNoImageProvided as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationVerifierNoImageProvided(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierPCRsInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AttestationVerifierPCRsInvalid(decoded));
            }
            if let Ok(decoded) =
                <AttestationVerifierPubkeyLengthInvalid as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AttestationVerifierPubkeyLengthInvalid(decoded));
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
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
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
    impl ::ethers::core::abi::AbiEncode for AttestationVerifierErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AttestationVerifierAttestationTooOld(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifierCannotRemoveAllAdmins(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifierImageAlreadyWhitelisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifierImageNotWhitelisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifierInitLengthMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifierInvalidAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifierKeyAlreadyVerified(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifierKeyNotVerified(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifierNoImageProvided(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifierPCRsInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifierPubkeyLengthInvalid(element) => {
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
                Self::FailedInnerCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitializing(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
    impl ::ethers::contract::ContractRevert for AttestationVerifierErrors {
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
                    == <AttestationVerifierAttestationTooOld as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationVerifierCannotRemoveAllAdmins as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationVerifierImageAlreadyWhitelisted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationVerifierImageNotWhitelisted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationVerifierInitLengthMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationVerifierInvalidAdmin as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationVerifierKeyAlreadyVerified as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationVerifierKeyNotVerified as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationVerifierNoImageProvided as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationVerifierPCRsInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationVerifierPubkeyLengthInvalid as ::ethers::contract::EthError>::selector() => {
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
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitializing as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for AttestationVerifierErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationVerifierAttestationTooOld(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifierCannotRemoveAllAdmins(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifierImageAlreadyWhitelisted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifierImageNotWhitelisted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifierInitLengthMismatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifierInvalidAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifierKeyAlreadyVerified(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifierKeyNotVerified(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifierNoImageProvided(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifierPCRsInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationVerifierPubkeyLengthInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureS(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInitialization(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnauthorizedCallContext(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AttestationVerifierErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation> for AttestationVerifierErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount> for AttestationVerifierErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for AttestationVerifierErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierAttestationTooOld> for AttestationVerifierErrors {
        fn from(value: AttestationVerifierAttestationTooOld) -> Self {
            Self::AttestationVerifierAttestationTooOld(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierCannotRemoveAllAdmins> for AttestationVerifierErrors {
        fn from(value: AttestationVerifierCannotRemoveAllAdmins) -> Self {
            Self::AttestationVerifierCannotRemoveAllAdmins(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierImageAlreadyWhitelisted>
        for AttestationVerifierErrors
    {
        fn from(value: AttestationVerifierImageAlreadyWhitelisted) -> Self {
            Self::AttestationVerifierImageAlreadyWhitelisted(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierImageNotWhitelisted> for AttestationVerifierErrors {
        fn from(value: AttestationVerifierImageNotWhitelisted) -> Self {
            Self::AttestationVerifierImageNotWhitelisted(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierInitLengthMismatch> for AttestationVerifierErrors {
        fn from(value: AttestationVerifierInitLengthMismatch) -> Self {
            Self::AttestationVerifierInitLengthMismatch(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierInvalidAdmin> for AttestationVerifierErrors {
        fn from(value: AttestationVerifierInvalidAdmin) -> Self {
            Self::AttestationVerifierInvalidAdmin(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierKeyAlreadyVerified> for AttestationVerifierErrors {
        fn from(value: AttestationVerifierKeyAlreadyVerified) -> Self {
            Self::AttestationVerifierKeyAlreadyVerified(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierKeyNotVerified> for AttestationVerifierErrors {
        fn from(value: AttestationVerifierKeyNotVerified) -> Self {
            Self::AttestationVerifierKeyNotVerified(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierNoImageProvided> for AttestationVerifierErrors {
        fn from(value: AttestationVerifierNoImageProvided) -> Self {
            Self::AttestationVerifierNoImageProvided(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierPCRsInvalid> for AttestationVerifierErrors {
        fn from(value: AttestationVerifierPCRsInvalid) -> Self {
            Self::AttestationVerifierPCRsInvalid(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierPubkeyLengthInvalid> for AttestationVerifierErrors {
        fn from(value: AttestationVerifierPubkeyLengthInvalid) -> Self {
            Self::AttestationVerifierPubkeyLengthInvalid(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for AttestationVerifierErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength> for AttestationVerifierErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for AttestationVerifierErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for AttestationVerifierErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for AttestationVerifierErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for AttestationVerifierErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for AttestationVerifierErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for AttestationVerifierErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for AttestationVerifierErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for AttestationVerifierErrors {
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
    pub enum AttestationVerifierEvents {
        EnclaveImageRevokedFilter(EnclaveImageRevokedFilter),
        EnclaveImageWhitelistedFilter(EnclaveImageWhitelistedFilter),
        EnclaveKeyRevokedFilter(EnclaveKeyRevokedFilter),
        EnclaveKeyVerifiedFilter(EnclaveKeyVerifiedFilter),
        EnclaveKeyWhitelistedFilter(EnclaveKeyWhitelistedFilter),
        InitializedFilter(InitializedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AttestationVerifierEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EnclaveImageRevokedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::EnclaveImageRevokedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveImageWhitelistedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::EnclaveImageWhitelistedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveKeyRevokedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::EnclaveKeyRevokedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyVerifiedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::EnclaveKeyVerifiedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyWhitelistedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::EnclaveKeyWhitelistedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(AttestationVerifierEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AttestationVerifierEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EnclaveImageRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveImageWhitelistedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveKeyRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveKeyVerifiedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveKeyWhitelistedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EnclaveImageRevokedFilter> for AttestationVerifierEvents {
        fn from(value: EnclaveImageRevokedFilter) -> Self {
            Self::EnclaveImageRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageWhitelistedFilter> for AttestationVerifierEvents {
        fn from(value: EnclaveImageWhitelistedFilter) -> Self {
            Self::EnclaveImageWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyRevokedFilter> for AttestationVerifierEvents {
        fn from(value: EnclaveKeyRevokedFilter) -> Self {
            Self::EnclaveKeyRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyVerifiedFilter> for AttestationVerifierEvents {
        fn from(value: EnclaveKeyVerifiedFilter) -> Self {
            Self::EnclaveKeyVerifiedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyWhitelistedFilter> for AttestationVerifierEvents {
        fn from(value: EnclaveKeyWhitelistedFilter) -> Self {
            Self::EnclaveKeyWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for AttestationVerifierEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for AttestationVerifierEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for AttestationVerifierEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for AttestationVerifierEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for AttestationVerifierEvents {
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
    ///Container type for all input parameters for the `MAX_AGE` function with signature `MAX_AGE()` and selector `0x0dcaeaf2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MAX_AGE", abi = "MAX_AGE()")]
    pub struct MaxAgeCall;
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize((bytes,bytes,bytes)[],bytes[],address)` and selector `0xc65a916a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        abi = "initialize((bytes,bytes,bytes)[],bytes[],address)"
    )]
    pub struct InitializeCall {
        pub images: ::std::vec::Vec<EnclaveImage>,
        pub enclave_keys: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub admin: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `pubKeyToAddress` function with signature `pubKeyToAddress(bytes)` and selector `0x70ae992a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pubKeyToAddress", abi = "pubKeyToAddress(bytes)")]
    pub struct PubKeyToAddressCall {
        pub pub_key: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `revokeEnclaveImage` function with signature `revokeEnclaveImage(bytes32)` and selector `0x95c60294`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "revokeEnclaveImage", abi = "revokeEnclaveImage(bytes32)")]
    pub struct RevokeEnclaveImageCall {
        pub image_id: [u8; 32],
    }
    ///Container type for all input parameters for the `revokeEnclaveKey` function with signature `revokeEnclaveKey(bytes)` and selector `0xd13714b4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "revokeEnclaveKey", abi = "revokeEnclaveKey(bytes)")]
    pub struct RevokeEnclaveKeyCall {
        pub enclave_pub_key: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `verifiedKeys` function with signature `verifiedKeys(address)` and selector `0xda994cd8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "verifiedKeys", abi = "verifiedKeys(address)")]
    pub struct VerifiedKeysCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes)` and selector `0x8e760afe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "verify", abi = "verify(bytes)")]
    pub struct VerifyCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes,(bytes,bytes,bytes,bytes,uint256))` and selector `0xeac708a3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "verify",
        abi = "verify(bytes,(bytes,bytes,bytes,bytes,uint256))"
    )]
    pub struct VerifyWithSignatureCall {
        pub signature: ::ethers::core::types::Bytes,
        pub attestation: Attestation,
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
    ///Container type for all input parameters for the `whitelistEnclaveImage` function with signature `whitelistEnclaveImage(bytes,bytes,bytes)` and selector `0x69815e57`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "whitelistEnclaveImage",
        abi = "whitelistEnclaveImage(bytes,bytes,bytes)"
    )]
    pub struct WhitelistEnclaveImageCall {
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `whitelistEnclaveKey` function with signature `whitelistEnclaveKey(bytes,bytes32)` and selector `0xdebf726b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "whitelistEnclaveKey",
        abi = "whitelistEnclaveKey(bytes,bytes32)"
    )]
    pub struct WhitelistEnclaveKeyCall {
        pub enclave_pub_key: ::ethers::core::types::Bytes,
        pub image_id: [u8; 32],
    }
    ///Container type for all input parameters for the `whitelistedImages` function with signature `whitelistedImages(bytes32)` and selector `0x2f9b0ad7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "whitelistedImages", abi = "whitelistedImages(bytes32)")]
    pub struct WhitelistedImagesCall(pub [u8; 32]);
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
    pub enum AttestationVerifierCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        MaxAge(MaxAgeCall),
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Initialize(InitializeCall),
        ProxiableUUID(ProxiableUUIDCall),
        PubKeyToAddress(PubKeyToAddressCall),
        RenounceRole(RenounceRoleCall),
        RevokeEnclaveImage(RevokeEnclaveImageCall),
        RevokeEnclaveKey(RevokeEnclaveKeyCall),
        RevokeRole(RevokeRoleCall),
        SupportsInterface(SupportsInterfaceCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        VerifiedKeys(VerifiedKeysCall),
        Verify(VerifyCall),
        VerifyWithSignature(VerifyWithSignatureCall),
        VerifyEnclaveKey(VerifyEnclaveKeyCall),
        WhitelistEnclaveImage(WhitelistEnclaveImageCall),
        WhitelistEnclaveKey(WhitelistEnclaveKeyCall),
        WhitelistedImages(WhitelistedImagesCall),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationVerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <MaxAgeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxAge(decoded));
            }
            if let Ok(decoded) =
                <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
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
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) =
                <PubKeyToAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PubKeyToAddress(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeEnclaveImageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevokeEnclaveImage(decoded));
            }
            if let Ok(decoded) =
                <RevokeEnclaveKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevokeEnclaveKey(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
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
            if let Ok(decoded) = <VerifiedKeysCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifiedKeys(decoded));
            }
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verify(decoded));
            }
            if let Ok(decoded) =
                <VerifyWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyWithSignature(decoded));
            }
            if let Ok(decoded) =
                <VerifyEnclaveKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyEnclaveKey(decoded));
            }
            if let Ok(decoded) =
                <WhitelistEnclaveImageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WhitelistEnclaveImage(decoded));
            }
            if let Ok(decoded) =
                <WhitelistEnclaveKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WhitelistEnclaveKey(decoded));
            }
            if let Ok(decoded) =
                <WhitelistedImagesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WhitelistedImages(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AttestationVerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxAge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GrantRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PubKeyToAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeEnclaveImage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeEnclaveKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeToAndCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifiedKeys(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyEnclaveKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WhitelistEnclaveImage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WhitelistEnclaveKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WhitelistedImages(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AttestationVerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxAge(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeInterfaceVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubKeyToAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeEnclaveImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifiedKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyWithSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistEnclaveImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistedImages(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for AttestationVerifierCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<MaxAgeCall> for AttestationVerifierCalls {
        fn from(value: MaxAgeCall) -> Self {
            Self::MaxAge(value)
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for AttestationVerifierCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for AttestationVerifierCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for AttestationVerifierCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for AttestationVerifierCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for AttestationVerifierCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for AttestationVerifierCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<PubKeyToAddressCall> for AttestationVerifierCalls {
        fn from(value: PubKeyToAddressCall) -> Self {
            Self::PubKeyToAddress(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for AttestationVerifierCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeEnclaveImageCall> for AttestationVerifierCalls {
        fn from(value: RevokeEnclaveImageCall) -> Self {
            Self::RevokeEnclaveImage(value)
        }
    }
    impl ::core::convert::From<RevokeEnclaveKeyCall> for AttestationVerifierCalls {
        fn from(value: RevokeEnclaveKeyCall) -> Self {
            Self::RevokeEnclaveKey(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for AttestationVerifierCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for AttestationVerifierCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for AttestationVerifierCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VerifiedKeysCall> for AttestationVerifierCalls {
        fn from(value: VerifiedKeysCall) -> Self {
            Self::VerifiedKeys(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for AttestationVerifierCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifyWithSignatureCall> for AttestationVerifierCalls {
        fn from(value: VerifyWithSignatureCall) -> Self {
            Self::VerifyWithSignature(value)
        }
    }
    impl ::core::convert::From<VerifyEnclaveKeyCall> for AttestationVerifierCalls {
        fn from(value: VerifyEnclaveKeyCall) -> Self {
            Self::VerifyEnclaveKey(value)
        }
    }
    impl ::core::convert::From<WhitelistEnclaveImageCall> for AttestationVerifierCalls {
        fn from(value: WhitelistEnclaveImageCall) -> Self {
            Self::WhitelistEnclaveImage(value)
        }
    }
    impl ::core::convert::From<WhitelistEnclaveKeyCall> for AttestationVerifierCalls {
        fn from(value: WhitelistEnclaveKeyCall) -> Self {
            Self::WhitelistEnclaveKey(value)
        }
    }
    impl ::core::convert::From<WhitelistedImagesCall> for AttestationVerifierCalls {
        fn from(value: WhitelistedImagesCall) -> Self {
            Self::WhitelistedImages(value)
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
    ///Container type for all return fields from the `MAX_AGE` function with signature `MAX_AGE()` and selector `0x0dcaeaf2`
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
    pub struct MaxAgeReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `pubKeyToAddress` function with signature `pubKeyToAddress(bytes)` and selector `0x70ae992a`
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
    pub struct PubKeyToAddressReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `verifiedKeys` function with signature `verifiedKeys(address)` and selector `0xda994cd8`
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
    pub struct VerifiedKeysReturn(pub [u8; 32]);
    ///Container type for all return fields from the `whitelistedImages` function with signature `whitelistedImages(bytes32)` and selector `0x2f9b0ad7`
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
    pub struct WhitelistedImagesReturn {
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
    }
    ///`EnclaveImage(bytes,bytes,bytes)`
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
    pub struct EnclaveImage {
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
    }
}
