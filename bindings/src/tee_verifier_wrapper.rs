pub use tee_verifier_wrapper::*;
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
pub mod tee_verifier_wrapper {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_av"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IAttestationVerifier",),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_proverPcrs"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Bytes,),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes[]"),
                        ),
                    },
                ],
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
                    ::std::borrow::ToOwned::to_owned("addEnclaveImageToFamily"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addEnclaveImageToFamily",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_proverPcr"),
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
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("checkSampleInputsAndProof"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkSampleInputsAndProof",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("encodeInputAndProofForVerification"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "encodeInputAndProofForVerification",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("inputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proof"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeInputs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("encodeInputs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("inputs"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::String,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string[]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeProof"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("encodeProof"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("proof"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                    "struct AttestationAuther.EnclaveImage",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("sampleInput"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sampleInput"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("sampleProof"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sampleProof"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verify"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("encodedData"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("verifyAgainstSampleInputs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifyAgainstSampleInputs",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("verifyInputs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifyInputs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("verifyProofForTeeVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifyProofForTeeVerifier",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proverData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proofData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proofSignature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
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
                            name: ::std::borrow::ToOwned::to_owned("enclaveAddress"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                                name: ::std::borrow::ToOwned::to_owned("enclaveAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
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
                                name: ::std::borrow::ToOwned::to_owned("enclaveAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
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
                    ::std::borrow::ToOwned::to_owned("InferredImageIdIsDifferent"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InferredImageIdIsDifferent",),
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
                    ::std::borrow::ToOwned::to_owned("InvalidInputs"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidInputs"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TEE_VERIFIER_WRAPPER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0%\x118\x03\x80b\0%\x11\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x05\x0CV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\x80Ra\xEA``\xA0R`\0[\x81Q\x81\x10\x15b\0\x01lW`\0\x80`\0\x84\x84\x81Q\x81\x10b\0\0nWb\0\0nb\0\x06\nV[` \x02` \x01\x01Q\x80` \x01\x90Q\x81\x01\x90b\0\0\x8B\x91\x90b\0\x06 V[\x91\x94P\x92P\x90P`\0b\0\0\xA1\x84\x84\x84b\0\x01\x96V[\x90Pb\0\0\xAE\x81b\0\x01\xD1V[b\0\0\xD3W`@Qc\x065l\xB3`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01`@Q\x80\x91\x03\x90\xFD[`\0b\0\x01\0`@Q\x80``\x01`@R\x80\x87\x81R` \x01\x86\x81R` \x01\x85\x81RPb\0\x02\x07` \x1B` \x1CV[P\x90P\x81\x81\x14b\0\x01$W`@Qc\x0CZ\x1A\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\0\x01P\x82\x7F+t\xD9\x9C\\@\x12=5\xC0\xFEN\xC1\xBCsj1\xA8\x0BpQr\x11\x93\x11\x8D\xC1Q~\\y?b\0\x03\x83V[PPPPPP\x80\x80b\0\x01c\x90b\0\x06\xB1V[\x91PPb\0\0JV[PP`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91UPb\0\x08\xF4V[`\0\x80\x84\x84\x84`@Q` \x01b\0\x01\xB0\x93\x92\x91\x90b\0\x06\xD9V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x81\x15\x80b\0\x02\0WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x82\x14[\x15\x92\x91PPV[`\0\x80\x82`\0\x01QQ`0\x14\x80\x15b\0\x02%WP\x82` \x01QQ`0\x14[\x80\x15b\0\x027WP\x82`@\x01QQ`0\x14[b\0\x02UW`@QcBc\r\xDB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83`\0\x01Q\x84` \x01Q\x85`@\x01Q`@Q` \x01b\0\x02z\x93\x92\x91\x90b\0\x06\xD9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x83\x90R\x91 \x80T\x91\x92P\x90b\0\x02\xAE\x90b\0\x07\"V[\x15\x90Pb\0\x02\xC0W\x93`\0\x93P\x91PPV[`@\x80Q``\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x81\x83\x01R\x86\x83\x01Q\x82\x84\x01R`\0\x84\x81R\x90\x81\x90R\x91\x90\x91 \x81Q\x81\x90b\0\x02\xFD\x90\x82b\0\x07\xB1V[P` \x82\x01Q`\x01\x82\x01\x90b\0\x03\x14\x90\x82b\0\x07\xB1V[P`@\x82\x01Q`\x02\x82\x01\x90b\0\x03+\x90\x82b\0\x07\xB1V[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q`@Qb\0\x03q\x93\x92\x91\x90b\0\x08\xABV[`@Q\x80\x91\x03\x90\xA2\x93`\x01\x93P\x91PPV[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 T`\xFF\x16\x15b\0\x03\xAFWP`\0b\0\x04\nV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90Q\x83\x81R\x84\x91\x7F\xBF\xB1&\xE7B\xCE\x96\x18\xB5\xBFkT\x83\x99\x16\x92o\\9wR\xBE5@L\x83h\xDD\xCFh\xC1\n\x91\x01`@Q\x80\x91\x03\x90\xA2P`\x01[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x04&W`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x04jWb\0\x04jb\0\x04)V[`@R\x91\x90PV[`\0[\x83\x81\x10\x15b\0\x04\x8FW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x04uV[PP`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12b\0\x04\xAAW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x04\xC6Wb\0\x04\xC6b\0\x04)V[b\0\x04\xDB`\x1F\x82\x01`\x1F\x19\x16` \x01b\0\x04?V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15b\0\x04\xF1W`\0\x80\xFD[b\0\x05\x04\x82` \x83\x01` \x87\x01b\0\x04rV[\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x05\"W`\0\x80\xFD[\x83Qb\0\x05/\x81b\0\x04\x10V[\x80\x93PP` \x80\x85\x01Qb\0\x05D\x81b\0\x04\x10V[`@\x86\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x05bW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12b\0\x05wW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x05\x8CWb\0\x05\x8Cb\0\x04)V[\x80`\x05\x1Bb\0\x05\x9D\x85\x82\x01b\0\x04?V[\x91\x82R\x83\x81\x01\x85\x01\x91\x85\x81\x01\x90\x8B\x84\x11\x15b\0\x05\xB8W`\0\x80\xFD[\x86\x86\x01\x92P[\x83\x83\x10\x15b\0\x05\xF9W\x82Q\x85\x81\x11\x15b\0\x05\xD8W`\0\x80\x81\xFD[b\0\x05\xE8\x8D\x89\x83\x8A\x01\x01b\0\x04\x98V[\x83RP\x91\x86\x01\x91\x90\x86\x01\x90b\0\x05\xBEV[\x80\x97PPPPPPPP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x066W`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x06NW`\0\x80\xFD[b\0\x06\\\x87\x83\x88\x01b\0\x04\x98V[\x94P` \x86\x01Q\x91P\x80\x82\x11\x15b\0\x06sW`\0\x80\xFD[b\0\x06\x81\x87\x83\x88\x01b\0\x04\x98V[\x93P`@\x86\x01Q\x91P\x80\x82\x11\x15b\0\x06\x98W`\0\x80\xFD[Pb\0\x06\xA7\x86\x82\x87\x01b\0\x04\x98V[\x91PP\x92P\x92P\x92V[`\0`\x01\x82\x01b\0\x06\xD2WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0\x84Qb\0\x06\xED\x81\x84` \x89\x01b\0\x04rV[\x84Q\x90\x83\x01\x90b\0\x07\x03\x81\x83` \x89\x01b\0\x04rV[\x84Q\x91\x01\x90b\0\x07\x18\x81\x83` \x88\x01b\0\x04rV[\x01\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x077W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x07XWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x07\xACW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x07\x87WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x07\xA8W\x82\x81U`\x01\x01b\0\x07\x93V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x07\xCDWb\0\x07\xCDb\0\x04)V[b\0\x07\xE5\x81b\0\x07\xDE\x84Tb\0\x07\"V[\x84b\0\x07^V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x08\x1DW`\0\x84\x15b\0\x08\x04WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x07\xA8V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x08NW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x08-V[P\x85\x82\x10\x15b\0\x08mW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x81Q\x80\x84Rb\0\x08\x97\x81` \x86\x01` \x86\x01b\0\x04rV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81R`\0b\0\x08\xC0``\x83\x01\x86b\0\x08}V[\x82\x81\x03` \x84\x01Rb\0\x08\xD4\x81\x86b\0\x08}V[\x90P\x82\x81\x03`@\x84\x01Rb\0\x08\xEA\x81\x85b\0\x08}V[\x96\x95PPPPPPV[`\x80Q`\xA0Qa\x1B\xE9b\0\t(`\09`\0\x81\x81a\x02i\x01Ra\x0C\xA8\x01R`\0\x81\x81a\x02\xAE\x01Ra\r\x14\x01Ra\x1B\xE9`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01!W`\x005`\xE0\x1C\x80c}\x8A\xD4+\x11a\0\xADW\x80c\xA7l\x05Q\x11a\0qW\x80c\xA7l\x05Q\x14a\x02\xA1W\x80c\xCDy\xF9\x06\x14a\x02\xA9W\x80c\xD2\xE8\x982\x14a\x02\xE8W\x80c\xF6\xEA\x99b\x14a\x02\xFBW\x80c\xF8Q\xA4@\x14a\x03\x0EW`\0\x80\xFD[\x80c}\x8A\xD4+\x14a\x026W\x80c\x81\xFD?W\x14a\x02>W\x80c\x8Ev\n\xFE\x14a\x02QW\x80c\x9A\xEC\x99\x0E\x14a\x02dW\x80c\xA6\xDF\xBC\x7F\x14a\x02\x8BW`\0\x80\xFD[\x80c\x10\xA5By\x11a\0\xF4W\x80c\x10\xA5By\x14a\x01\xAEW\x80c$\x10\xF6\xBA\x14a\x01\xB6W\x80c[\xE5Y\xAF\x14a\x01\xD6W\x80ck[!\xA6\x14a\x01\xF6W\x80cu\x84{\x84\x14a\x02#W`\0\x80\xFD[\x80c\x01\xD5\x8F\xA3\x14a\x01&W\x80c\x02\xF7}\x19\x14a\x01bW\x80c\x07\x07Y\x1F\x14a\x01\x86W\x80c\x10\x84\xD6\\\x14a\x01\x9BW[`\0\x80\xFD[a\x01Oa\x0146`\x04a\x10\xF9V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01va\x01p6`\x04a\x12\rV[P`\x01\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01YV[a\x01\x99a\x01\x946`\x04a\x12IV[a\x03!V[\0[a\x01\x99a\x01\xA96`\x04a\x12\rV[a\x03/V[a\x01va\x042V[a\x01\xC9a\x01\xC46`\x04a\x12\xBAV[a\x04\xCDV[`@Qa\x01Y\x91\x90a\x13#V[a\x01\xE9a\x01\xE46`\x04a\x14\"V[a\x06\xC6V[`@Qa\x01Y\x91\x90a\x14\x85V[a\x01va\x02\x046`\x04a\x14\x98V[`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x90V[a\x01va\x0216`\x04a\x14\xBAV[a\x07\x03V[a\x01\xE9a\x07\x16V[a\x01\xE9a\x02L6`\x04a\x12\rV[a\x07\xA4V[a\x01va\x02_6`\x04a\x12\rV[a\x07\xCDV[a\x01O\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01va\x02\x996`\x04a\x12IV[`\x01\x92\x91PPV[a\x01\xE9a\x08MV[a\x02\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01YV[a\x01va\x02\xF66`\x04a\x15\xC3V[a\x08ZV[a\x01\xE9a\x03\t6`\x04a\x16JV[a\tZV[`\x05Ta\x02\xD0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03+\x82\x82a\tmV[PPV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03ZW`@QcS\xFCwk`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a\x03s\x91\x90a\x16\xC3V[\x91\x94P\x92P\x90P`\0a\x03\x87\x84\x84\x84a\t\xC7V[\x90Pa\x03\x92\x81a\n\0V[a\x03\xB7W`@Qc\x065l\xB3`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x03\xDC`@Q\x80``\x01`@R\x80\x87\x81R` \x01\x86\x81R` \x01\x85\x81RPa\n5V[P\x90P\x81\x81\x14a\x03\xFFW`@Qc\x0CZ\x1A\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04)\x82\x7F+t\xD9\x9C\\@\x12=5\xC0\xFEN\xC1\xBCsj1\xA8\x0BpQr\x11\x93\x11\x8D\xC1Q~\\y?a\x0B\xA1V[PPPPPPPV[`\0a\x04\xC8`\x04\x80Ta\x04D\x90a\x17@V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04p\x90a\x17@V[\x80\x15a\x04\xBDW\x80`\x1F\x10a\x04\x92Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xBDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPP`\x01\x90V[\x90P\x90V[a\x04\xF1`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81R` \x81\x90R`@\x90\x81\x90 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x05\x19\x90a\x17@V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05E\x90a\x17@V[\x80\x15a\x05\x92W\x80`\x1F\x10a\x05gWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x92V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05uW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x05\xAB\x90a\x17@V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xD7\x90a\x17@V[\x80\x15a\x06$W\x80`\x1F\x10a\x05\xF9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06$V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x07W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x06=\x90a\x17@V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06i\x90a\x17@V[\x80\x15a\x06\xB6W\x80`\x1F\x10a\x06\x8BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xB6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x99W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[``a\x06\xD1\x83a\tZV[a\x06\xDA\x83a\x07\xA4V[`@Q` \x01a\x06\xEB\x92\x91\x90a\x17zV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[\x92\x91PPV[`\0a\x07\x0F\x83\x83a\x0C+V[\x93\x92PPPV[`\x03\x80Ta\x07#\x90a\x17@V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07O\x90a\x17@V[\x80\x15a\x07\x9CW\x80`\x1F\x10a\x07qWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x9CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``\x81`@Q` \x01a\x07\xB7\x91\x90a\x14\x85V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a\x07\xE6\x91\x90a\x17\x9FV[\x91P\x91P`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a\x08\x03\x91\x90a\x16\xC3V[\x92P\x92P\x92P\x82\x80Q\x90` \x01 \x85\x80Q\x90` \x01 \x14a\x087W`@Qcy\xA6}[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08B\x83\x83\x83a\x08ZV[\x97\x96PPPPPPPV[`\x04\x80Ta\x07#\x90a\x17@V[`\0\x80\x84\x84`@Q` \x01a\x08p\x92\x91\x90a\x17zV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x08\xE1\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90P`\0a\x08\xEF\x82\x86a\x0E\x19V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\t#W`@Qc(\x80\xCB\x7F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x03\xAEV[a\tM\x81\x7F+t\xD9\x9C\\@\x12=5\xC0\xFEN\xC1\xBCsj1\xA8\x0BpQr\x11\x93\x11\x8D\xC1Q~\\y?a\x0ECV[P`\x01\x96\x95PPPPPPV[``\x81`@Q` \x01a\x07\xB7\x91\x90a\x17\xF8V[`\0\x80\x80\x80\x80\x80a\t\x80\x87\x89\x01\x89a\x18ZV[\x95P\x95P\x95P\x95P\x95P\x95Pa\t\xBC\x86`@Q\x80`\xA0\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01\x85\x81R` \x01\x84\x81RPa\x0C+V[PPPPPPPPPV[`\0\x80\x84\x84\x84`@Q` \x01a\t\xDF\x93\x92\x91\x90a\x193V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x81\x15\x80a\n.WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x82\x14[\x15\x92\x91PPV[`\0\x80\x82`\0\x01QQ`0\x14\x80\x15a\nRWP\x82` \x01QQ`0\x14[\x80\x15a\ncWP\x82`@\x01QQ`0\x14[a\n\x80W`@QcBc\r\xDB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83`\0\x01Q\x84` \x01Q\x85`@\x01Q`@Q` \x01a\n\xA3\x93\x92\x91\x90a\x193V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x83\x90R\x91 \x80T\x91\x92P\x90a\n\xD5\x90a\x17@V[\x15\x90Pa\n\xE6W\x93`\0\x93P\x91PPV[`@\x80Q``\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x81\x83\x01R\x86\x83\x01Q\x82\x84\x01R`\0\x84\x81R\x90\x81\x90R\x91\x90\x91 \x81Q\x81\x90a\x0B!\x90\x82a\x19\xC4V[P` \x82\x01Q`\x01\x82\x01\x90a\x0B6\x90\x82a\x19\xC4V[P`@\x82\x01Q`\x02\x82\x01\x90a\x0BK\x90\x82a\x19\xC4V[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q`@Qa\x0B\x8F\x93\x92\x91\x90a\x1A\x83V[`@Q\x80\x91\x03\x90\xA2\x93`\x01\x93P\x91PPV[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x0B\xCBWP`\0a\x06\xFDV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90Q\x83\x81R\x84\x91\x7F\xBF\xB1&\xE7B\xCE\x96\x18\xB5\xBFkT\x83\x99\x16\x92o\\9wR\xBE5@L\x83h\xDD\xCFh\xC1\n\x91\x01`@Q\x80\x91\x03\x90\xA2P`\x01\x92\x91PPV[`\0\x80\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x0CO\x93\x92\x91\x90a\x193V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x83\x90R\x91 \x80T\x91\x92P\x90a\x0C\x81\x90a\x17@V[\x90P`\0\x03a\x0C\xA3W`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\xCD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba\x1A\xC6V[a\x03\xE8\x84`\x80\x01Qa\x0C\xDF\x91\x90a\x1A\xE7V[\x11a\x0C\xFDW`@Qc\x0C\xB0/\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xEA\xC7\x08\xA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEA\xC7\x08\xA3\x90a\rK\x90\x87\x90\x87\x90`\x04\x01a\x1B\tV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rcW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\rwW=`\0\x80>=`\0\xFD[PPPP`\0a\r\x8A\x84`\0\x01Qa\x0E\xF4V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x90\x91P\x15a\r\xB6W`\0\x92PPPa\x06\xFDV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x01` R`@\x90\x81\x90 \x84\x90U\x85Q\x90Q\x84\x92\x91\x7F\xBBMd(\xD5>\xA9$\xD9K\xE0H\x84s\xDE\xB5\xC0\xA7\x0C\x97\x9F\x84w\xD2\x19\x11\xCB[\xEE@7\xFD\x91a\x0E\x06\x91\x90a\x14\x85V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0\x80`\0\x80a\x0E)\x86\x86a\x0F$V[\x92P\x92P\x92Pa\x0E9\x82\x82a\x0FqV[P\x90\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 T\x80a\x0EzW`@Qc=\xD8\xCA\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R` \x81\x90R`@\x90 \x80Ta\x0E\x93\x90a\x17@V[\x90P`\0\x03a\x0E\xB5W`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T`\xFF\x16a\x0E\xEFW`@QcHf%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x81Q`@\x14a\x0F\x18W`@Qc\xBD\x9C\x80\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[`\0\x80`\0\x83Q`A\x03a\x0F^W` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa\x0FP\x88\x82\x85\x85a\x10*V[\x95P\x95P\x95PPPPa\x0FjV[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a\x0F\x85Wa\x0F\x85a\x1B\x9DV[\x03a\x0F\x8EWPPV[`\x01\x82`\x03\x81\x11\x15a\x0F\xA2Wa\x0F\xA2a\x1B\x9DV[\x03a\x0F\xC0W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x0F\xD4Wa\x0F\xD4a\x1B\x9DV[\x03a\x0F\xF5W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\xAEV[`\x03\x82`\x03\x81\x11\x15a\x10\tWa\x10\ta\x1B\x9DV[\x03a\x03+W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\xAEV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x10eWP`\0\x91P`\x03\x90P\x82a\x10\xEFV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x10\xB9W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10\xE5WP`\0\x92P`\x01\x91P\x82\x90Pa\x10\xEFV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\x0BW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x0FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11ZWa\x11Za\x11\"V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\x88Wa\x11\x88a\x11\"V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x11\xA9Wa\x11\xA9a\x11\"V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x11\xC8W`\0\x80\xFD[\x815a\x11\xDBa\x11\xD6\x82a\x11\x90V[a\x11`V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x11\xF0W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x12\x1FW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x125W`\0\x80\xFD[a\x12A\x84\x82\x85\x01a\x11\xB7V[\x94\x93PPPPV[`\0\x80` \x83\x85\x03\x12\x15a\x12\\W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x12sW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x12\x87W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x12\x96W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x12\xA8W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x12\xCCW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x12\xEEW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\xD6V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x13\x0F\x81` \x86\x01` \x86\x01a\x12\xD3V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0\x82Q``` \x84\x01Ra\x13?`\x80\x84\x01\x82a\x12\xF7V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra\x13]\x83\x83a\x12\xF7V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa\x13{\x82\x82a\x12\xF7V[\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a\x13\x95W`\0\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x80\x83\x11\x15a\x13\xB1Wa\x13\xB1a\x11\"V[\x82`\x05\x1Ba\x13\xC0\x83\x82\x01a\x11`V[\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x90\x88\x86\x11\x15a\x13\xDAW`\0\x80\xFD[\x84\x88\x01\x92P[\x85\x83\x10\x15a\x14\x16W\x825\x84\x81\x11\x15a\x13\xF8W`\0\x80\x81\xFD[a\x14\x06\x8A\x87\x83\x8C\x01\x01a\x11\xB7V[\x83RP\x91\x84\x01\x91\x90\x84\x01\x90a\x13\xE0V[\x98\x97PPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x145W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x14LW`\0\x80\xFD[a\x14X\x86\x83\x87\x01a\x13\x84V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x14nW`\0\x80\xFD[Pa\x14{\x85\x82\x86\x01a\x11\xB7V[\x91PP\x92P\x92\x90PV[` \x81R`\0a\x07\x0F` \x83\x01\x84a\x12\xF7V[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xABW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xCDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x14\xE4W`\0\x80\xFD[a\x14\xF0\x86\x83\x87\x01a\x11\xB7V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x15\x06W`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\x15\x1AW`\0\x80\xFD[a\x15\"a\x118V[\x825\x82\x81\x11\x15a\x151W`\0\x80\xFD[a\x15=\x88\x82\x86\x01a\x11\xB7V[\x82RP` \x83\x015\x82\x81\x11\x15a\x15RW`\0\x80\xFD[a\x15^\x88\x82\x86\x01a\x11\xB7V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x15vW`\0\x80\xFD[a\x15\x82\x88\x82\x86\x01a\x11\xB7V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\x15\x9AW`\0\x80\xFD[a\x15\xA6\x88\x82\x86\x01a\x11\xB7V[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15\xD8W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x15\xEFW`\0\x80\xFD[a\x15\xFB\x87\x83\x88\x01a\x11\xB7V[\x94P` \x86\x015\x91P\x80\x82\x11\x15a\x16\x11W`\0\x80\xFD[a\x16\x1D\x87\x83\x88\x01a\x11\xB7V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x163W`\0\x80\xFD[Pa\x16@\x86\x82\x87\x01a\x11\xB7V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x16\\W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16rW`\0\x80\xFD[a\x12A\x84\x82\x85\x01a\x13\x84V[`\0\x82`\x1F\x83\x01\x12a\x16\x8FW`\0\x80\xFD[\x81Qa\x16\x9Da\x11\xD6\x82a\x11\x90V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x16\xB2W`\0\x80\xFD[a\x12A\x82` \x83\x01` \x87\x01a\x12\xD3V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\xD8W`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16\xEFW`\0\x80\xFD[a\x16\xFB\x87\x83\x88\x01a\x16~V[\x94P` \x86\x01Q\x91P\x80\x82\x11\x15a\x17\x11W`\0\x80\xFD[a\x17\x1D\x87\x83\x88\x01a\x16~V[\x93P`@\x86\x01Q\x91P\x80\x82\x11\x15a\x173W`\0\x80\xFD[Pa\x16@\x86\x82\x87\x01a\x16~V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x17TW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x17tWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0a\x17\x8D`@\x83\x01\x85a\x12\xF7V[\x82\x81\x03` \x84\x01Ra\x13{\x81\x85a\x12\xF7V[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xB2W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17\xC9W`\0\x80\xFD[a\x17\xD5\x86\x83\x87\x01a\x16~V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\x17\xEBW`\0\x80\xFD[Pa\x14{\x85\x82\x86\x01a\x16~V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x18MW`?\x19\x88\x86\x03\x01\x84Ra\x18;\x85\x83Qa\x12\xF7V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x18\x1FV[P\x92\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x18sW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x18\x8AW`\0\x80\xFD[a\x18\x96\x8A\x83\x8B\x01a\x11\xB7V[\x97P` \x89\x015\x91P\x80\x82\x11\x15a\x18\xACW`\0\x80\xFD[a\x18\xB8\x8A\x83\x8B\x01a\x11\xB7V[\x96P`@\x89\x015\x91P\x80\x82\x11\x15a\x18\xCEW`\0\x80\xFD[a\x18\xDA\x8A\x83\x8B\x01a\x11\xB7V[\x95P``\x89\x015\x91P\x80\x82\x11\x15a\x18\xF0W`\0\x80\xFD[a\x18\xFC\x8A\x83\x8B\x01a\x11\xB7V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15a\x19\x12W`\0\x80\xFD[Pa\x19\x1F\x89\x82\x8A\x01a\x11\xB7V[\x92PP`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x84Qa\x19E\x81\x84` \x89\x01a\x12\xD3V[\x84Q\x90\x83\x01\x90a\x19Y\x81\x83` \x89\x01a\x12\xD3V[\x84Q\x91\x01\x90a\x19l\x81\x83` \x88\x01a\x12\xD3V[\x01\x95\x94PPPPPV[`\x1F\x82\x11\x15a\x0E\xEFW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x19\x9DWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x19\xBCW\x82\x81U`\x01\x01a\x19\xA9V[PPPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xDDWa\x19\xDDa\x11\"V[a\x19\xF1\x81a\x19\xEB\x84Ta\x17@V[\x84a\x19vV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1A&W`\0\x84\x15a\x1A\x0EWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x19\xBCV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x1AUW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1A6V[P\x85\x82\x10\x15a\x1AsW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[``\x81R`\0a\x1A\x96``\x83\x01\x86a\x12\xF7V[\x82\x81\x03` \x84\x01Ra\x1A\xA8\x81\x86a\x12\xF7V[\x90P\x82\x81\x03`@\x84\x01Ra\x1A\xBC\x81\x85a\x12\xF7V[\x96\x95PPPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x06\xFDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x1B\x04WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`@\x81R`\0a\x1B\x1C`@\x83\x01\x85a\x12\xF7V[\x82\x81\x03` \x84\x01R\x83Q`\xA0\x82Ra\x1B7`\xA0\x83\x01\x82a\x12\xF7V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01Ra\x1BP\x82\x82a\x12\xF7V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra\x1Bj\x82\x82a\x12\xF7V[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01Ra\x1B\x84\x82\x82a\x12\xF7V[\x91PP`\x80\x85\x01Q`\x80\x83\x01R\x80\x92PPP\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 (\xB5^\x17\x8CV\x07\x9D\xCD\xA9K\xEC)`\x92\xA7(\xDCN\xE4\xA2Q\xC9k\x06'~\xA38\xBE\xA4\xE1dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static TEE_VERIFIER_WRAPPER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01!W`\x005`\xE0\x1C\x80c}\x8A\xD4+\x11a\0\xADW\x80c\xA7l\x05Q\x11a\0qW\x80c\xA7l\x05Q\x14a\x02\xA1W\x80c\xCDy\xF9\x06\x14a\x02\xA9W\x80c\xD2\xE8\x982\x14a\x02\xE8W\x80c\xF6\xEA\x99b\x14a\x02\xFBW\x80c\xF8Q\xA4@\x14a\x03\x0EW`\0\x80\xFD[\x80c}\x8A\xD4+\x14a\x026W\x80c\x81\xFD?W\x14a\x02>W\x80c\x8Ev\n\xFE\x14a\x02QW\x80c\x9A\xEC\x99\x0E\x14a\x02dW\x80c\xA6\xDF\xBC\x7F\x14a\x02\x8BW`\0\x80\xFD[\x80c\x10\xA5By\x11a\0\xF4W\x80c\x10\xA5By\x14a\x01\xAEW\x80c$\x10\xF6\xBA\x14a\x01\xB6W\x80c[\xE5Y\xAF\x14a\x01\xD6W\x80ck[!\xA6\x14a\x01\xF6W\x80cu\x84{\x84\x14a\x02#W`\0\x80\xFD[\x80c\x01\xD5\x8F\xA3\x14a\x01&W\x80c\x02\xF7}\x19\x14a\x01bW\x80c\x07\x07Y\x1F\x14a\x01\x86W\x80c\x10\x84\xD6\\\x14a\x01\x9BW[`\0\x80\xFD[a\x01Oa\x0146`\x04a\x10\xF9V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01va\x01p6`\x04a\x12\rV[P`\x01\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01YV[a\x01\x99a\x01\x946`\x04a\x12IV[a\x03!V[\0[a\x01\x99a\x01\xA96`\x04a\x12\rV[a\x03/V[a\x01va\x042V[a\x01\xC9a\x01\xC46`\x04a\x12\xBAV[a\x04\xCDV[`@Qa\x01Y\x91\x90a\x13#V[a\x01\xE9a\x01\xE46`\x04a\x14\"V[a\x06\xC6V[`@Qa\x01Y\x91\x90a\x14\x85V[a\x01va\x02\x046`\x04a\x14\x98V[`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x90V[a\x01va\x0216`\x04a\x14\xBAV[a\x07\x03V[a\x01\xE9a\x07\x16V[a\x01\xE9a\x02L6`\x04a\x12\rV[a\x07\xA4V[a\x01va\x02_6`\x04a\x12\rV[a\x07\xCDV[a\x01O\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01va\x02\x996`\x04a\x12IV[`\x01\x92\x91PPV[a\x01\xE9a\x08MV[a\x02\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01YV[a\x01va\x02\xF66`\x04a\x15\xC3V[a\x08ZV[a\x01\xE9a\x03\t6`\x04a\x16JV[a\tZV[`\x05Ta\x02\xD0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03+\x82\x82a\tmV[PPV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03ZW`@QcS\xFCwk`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a\x03s\x91\x90a\x16\xC3V[\x91\x94P\x92P\x90P`\0a\x03\x87\x84\x84\x84a\t\xC7V[\x90Pa\x03\x92\x81a\n\0V[a\x03\xB7W`@Qc\x065l\xB3`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x03\xDC`@Q\x80``\x01`@R\x80\x87\x81R` \x01\x86\x81R` \x01\x85\x81RPa\n5V[P\x90P\x81\x81\x14a\x03\xFFW`@Qc\x0CZ\x1A\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04)\x82\x7F+t\xD9\x9C\\@\x12=5\xC0\xFEN\xC1\xBCsj1\xA8\x0BpQr\x11\x93\x11\x8D\xC1Q~\\y?a\x0B\xA1V[PPPPPPPV[`\0a\x04\xC8`\x04\x80Ta\x04D\x90a\x17@V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04p\x90a\x17@V[\x80\x15a\x04\xBDW\x80`\x1F\x10a\x04\x92Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xBDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPP`\x01\x90V[\x90P\x90V[a\x04\xF1`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81R` \x81\x90R`@\x90\x81\x90 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x05\x19\x90a\x17@V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05E\x90a\x17@V[\x80\x15a\x05\x92W\x80`\x1F\x10a\x05gWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x92V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05uW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x05\xAB\x90a\x17@V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xD7\x90a\x17@V[\x80\x15a\x06$W\x80`\x1F\x10a\x05\xF9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06$V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x07W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x06=\x90a\x17@V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06i\x90a\x17@V[\x80\x15a\x06\xB6W\x80`\x1F\x10a\x06\x8BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xB6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x99W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[``a\x06\xD1\x83a\tZV[a\x06\xDA\x83a\x07\xA4V[`@Q` \x01a\x06\xEB\x92\x91\x90a\x17zV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[\x92\x91PPV[`\0a\x07\x0F\x83\x83a\x0C+V[\x93\x92PPPV[`\x03\x80Ta\x07#\x90a\x17@V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07O\x90a\x17@V[\x80\x15a\x07\x9CW\x80`\x1F\x10a\x07qWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x9CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``\x81`@Q` \x01a\x07\xB7\x91\x90a\x14\x85V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a\x07\xE6\x91\x90a\x17\x9FV[\x91P\x91P`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a\x08\x03\x91\x90a\x16\xC3V[\x92P\x92P\x92P\x82\x80Q\x90` \x01 \x85\x80Q\x90` \x01 \x14a\x087W`@Qcy\xA6}[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08B\x83\x83\x83a\x08ZV[\x97\x96PPPPPPPV[`\x04\x80Ta\x07#\x90a\x17@V[`\0\x80\x84\x84`@Q` \x01a\x08p\x92\x91\x90a\x17zV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x08\xE1\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90P`\0a\x08\xEF\x82\x86a\x0E\x19V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\t#W`@Qc(\x80\xCB\x7F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x03\xAEV[a\tM\x81\x7F+t\xD9\x9C\\@\x12=5\xC0\xFEN\xC1\xBCsj1\xA8\x0BpQr\x11\x93\x11\x8D\xC1Q~\\y?a\x0ECV[P`\x01\x96\x95PPPPPPV[``\x81`@Q` \x01a\x07\xB7\x91\x90a\x17\xF8V[`\0\x80\x80\x80\x80\x80a\t\x80\x87\x89\x01\x89a\x18ZV[\x95P\x95P\x95P\x95P\x95P\x95Pa\t\xBC\x86`@Q\x80`\xA0\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01\x85\x81R` \x01\x84\x81RPa\x0C+V[PPPPPPPPPV[`\0\x80\x84\x84\x84`@Q` \x01a\t\xDF\x93\x92\x91\x90a\x193V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x81\x15\x80a\n.WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x82\x14[\x15\x92\x91PPV[`\0\x80\x82`\0\x01QQ`0\x14\x80\x15a\nRWP\x82` \x01QQ`0\x14[\x80\x15a\ncWP\x82`@\x01QQ`0\x14[a\n\x80W`@QcBc\r\xDB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83`\0\x01Q\x84` \x01Q\x85`@\x01Q`@Q` \x01a\n\xA3\x93\x92\x91\x90a\x193V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x83\x90R\x91 \x80T\x91\x92P\x90a\n\xD5\x90a\x17@V[\x15\x90Pa\n\xE6W\x93`\0\x93P\x91PPV[`@\x80Q``\x81\x01\x82R\x85Q\x81R` \x80\x87\x01Q\x81\x83\x01R\x86\x83\x01Q\x82\x84\x01R`\0\x84\x81R\x90\x81\x90R\x91\x90\x91 \x81Q\x81\x90a\x0B!\x90\x82a\x19\xC4V[P` \x82\x01Q`\x01\x82\x01\x90a\x0B6\x90\x82a\x19\xC4V[P`@\x82\x01Q`\x02\x82\x01\x90a\x0BK\x90\x82a\x19\xC4V[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q`@Qa\x0B\x8F\x93\x92\x91\x90a\x1A\x83V[`@Q\x80\x91\x03\x90\xA2\x93`\x01\x93P\x91PPV[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x0B\xCBWP`\0a\x06\xFDV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90Q\x83\x81R\x84\x91\x7F\xBF\xB1&\xE7B\xCE\x96\x18\xB5\xBFkT\x83\x99\x16\x92o\\9wR\xBE5@L\x83h\xDD\xCFh\xC1\n\x91\x01`@Q\x80\x91\x03\x90\xA2P`\x01\x92\x91PPV[`\0\x80\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x0CO\x93\x92\x91\x90a\x193V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x83\x90R\x91 \x80T\x91\x92P\x90a\x0C\x81\x90a\x17@V[\x90P`\0\x03a\x0C\xA3W`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\xCD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba\x1A\xC6V[a\x03\xE8\x84`\x80\x01Qa\x0C\xDF\x91\x90a\x1A\xE7V[\x11a\x0C\xFDW`@Qc\x0C\xB0/\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xEA\xC7\x08\xA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEA\xC7\x08\xA3\x90a\rK\x90\x87\x90\x87\x90`\x04\x01a\x1B\tV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rcW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\rwW=`\0\x80>=`\0\xFD[PPPP`\0a\r\x8A\x84`\0\x01Qa\x0E\xF4V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x90\x91P\x15a\r\xB6W`\0\x92PPPa\x06\xFDV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x01` R`@\x90\x81\x90 \x84\x90U\x85Q\x90Q\x84\x92\x91\x7F\xBBMd(\xD5>\xA9$\xD9K\xE0H\x84s\xDE\xB5\xC0\xA7\x0C\x97\x9F\x84w\xD2\x19\x11\xCB[\xEE@7\xFD\x91a\x0E\x06\x91\x90a\x14\x85V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0\x80`\0\x80a\x0E)\x86\x86a\x0F$V[\x92P\x92P\x92Pa\x0E9\x82\x82a\x0FqV[P\x90\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 T\x80a\x0EzW`@Qc=\xD8\xCA\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R` \x81\x90R`@\x90 \x80Ta\x0E\x93\x90a\x17@V[\x90P`\0\x03a\x0E\xB5W`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T`\xFF\x16a\x0E\xEFW`@QcHf%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x81Q`@\x14a\x0F\x18W`@Qc\xBD\x9C\x80\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[`\0\x80`\0\x83Q`A\x03a\x0F^W` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa\x0FP\x88\x82\x85\x85a\x10*V[\x95P\x95P\x95PPPPa\x0FjV[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a\x0F\x85Wa\x0F\x85a\x1B\x9DV[\x03a\x0F\x8EWPPV[`\x01\x82`\x03\x81\x11\x15a\x0F\xA2Wa\x0F\xA2a\x1B\x9DV[\x03a\x0F\xC0W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x0F\xD4Wa\x0F\xD4a\x1B\x9DV[\x03a\x0F\xF5W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\xAEV[`\x03\x82`\x03\x81\x11\x15a\x10\tWa\x10\ta\x1B\x9DV[\x03a\x03+W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\xAEV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x10eWP`\0\x91P`\x03\x90P\x82a\x10\xEFV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x10\xB9W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10\xE5WP`\0\x92P`\x01\x91P\x82\x90Pa\x10\xEFV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\x0BW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x0FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11ZWa\x11Za\x11\"V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\x88Wa\x11\x88a\x11\"V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x11\xA9Wa\x11\xA9a\x11\"V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x11\xC8W`\0\x80\xFD[\x815a\x11\xDBa\x11\xD6\x82a\x11\x90V[a\x11`V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x11\xF0W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x12\x1FW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x125W`\0\x80\xFD[a\x12A\x84\x82\x85\x01a\x11\xB7V[\x94\x93PPPPV[`\0\x80` \x83\x85\x03\x12\x15a\x12\\W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x12sW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x12\x87W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x12\x96W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x12\xA8W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x12\xCCW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x12\xEEW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\xD6V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x13\x0F\x81` \x86\x01` \x86\x01a\x12\xD3V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0\x82Q``` \x84\x01Ra\x13?`\x80\x84\x01\x82a\x12\xF7V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra\x13]\x83\x83a\x12\xF7V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa\x13{\x82\x82a\x12\xF7V[\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a\x13\x95W`\0\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x80\x83\x11\x15a\x13\xB1Wa\x13\xB1a\x11\"V[\x82`\x05\x1Ba\x13\xC0\x83\x82\x01a\x11`V[\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x90\x88\x86\x11\x15a\x13\xDAW`\0\x80\xFD[\x84\x88\x01\x92P[\x85\x83\x10\x15a\x14\x16W\x825\x84\x81\x11\x15a\x13\xF8W`\0\x80\x81\xFD[a\x14\x06\x8A\x87\x83\x8C\x01\x01a\x11\xB7V[\x83RP\x91\x84\x01\x91\x90\x84\x01\x90a\x13\xE0V[\x98\x97PPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x145W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x14LW`\0\x80\xFD[a\x14X\x86\x83\x87\x01a\x13\x84V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x14nW`\0\x80\xFD[Pa\x14{\x85\x82\x86\x01a\x11\xB7V[\x91PP\x92P\x92\x90PV[` \x81R`\0a\x07\x0F` \x83\x01\x84a\x12\xF7V[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xABW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xCDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x14\xE4W`\0\x80\xFD[a\x14\xF0\x86\x83\x87\x01a\x11\xB7V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x15\x06W`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\x15\x1AW`\0\x80\xFD[a\x15\"a\x118V[\x825\x82\x81\x11\x15a\x151W`\0\x80\xFD[a\x15=\x88\x82\x86\x01a\x11\xB7V[\x82RP` \x83\x015\x82\x81\x11\x15a\x15RW`\0\x80\xFD[a\x15^\x88\x82\x86\x01a\x11\xB7V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x15vW`\0\x80\xFD[a\x15\x82\x88\x82\x86\x01a\x11\xB7V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\x15\x9AW`\0\x80\xFD[a\x15\xA6\x88\x82\x86\x01a\x11\xB7V[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15\xD8W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x15\xEFW`\0\x80\xFD[a\x15\xFB\x87\x83\x88\x01a\x11\xB7V[\x94P` \x86\x015\x91P\x80\x82\x11\x15a\x16\x11W`\0\x80\xFD[a\x16\x1D\x87\x83\x88\x01a\x11\xB7V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x163W`\0\x80\xFD[Pa\x16@\x86\x82\x87\x01a\x11\xB7V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x16\\W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16rW`\0\x80\xFD[a\x12A\x84\x82\x85\x01a\x13\x84V[`\0\x82`\x1F\x83\x01\x12a\x16\x8FW`\0\x80\xFD[\x81Qa\x16\x9Da\x11\xD6\x82a\x11\x90V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x16\xB2W`\0\x80\xFD[a\x12A\x82` \x83\x01` \x87\x01a\x12\xD3V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\xD8W`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16\xEFW`\0\x80\xFD[a\x16\xFB\x87\x83\x88\x01a\x16~V[\x94P` \x86\x01Q\x91P\x80\x82\x11\x15a\x17\x11W`\0\x80\xFD[a\x17\x1D\x87\x83\x88\x01a\x16~V[\x93P`@\x86\x01Q\x91P\x80\x82\x11\x15a\x173W`\0\x80\xFD[Pa\x16@\x86\x82\x87\x01a\x16~V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x17TW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x17tWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0a\x17\x8D`@\x83\x01\x85a\x12\xF7V[\x82\x81\x03` \x84\x01Ra\x13{\x81\x85a\x12\xF7V[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xB2W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17\xC9W`\0\x80\xFD[a\x17\xD5\x86\x83\x87\x01a\x16~V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\x17\xEBW`\0\x80\xFD[Pa\x14{\x85\x82\x86\x01a\x16~V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x18MW`?\x19\x88\x86\x03\x01\x84Ra\x18;\x85\x83Qa\x12\xF7V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x18\x1FV[P\x92\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x18sW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x18\x8AW`\0\x80\xFD[a\x18\x96\x8A\x83\x8B\x01a\x11\xB7V[\x97P` \x89\x015\x91P\x80\x82\x11\x15a\x18\xACW`\0\x80\xFD[a\x18\xB8\x8A\x83\x8B\x01a\x11\xB7V[\x96P`@\x89\x015\x91P\x80\x82\x11\x15a\x18\xCEW`\0\x80\xFD[a\x18\xDA\x8A\x83\x8B\x01a\x11\xB7V[\x95P``\x89\x015\x91P\x80\x82\x11\x15a\x18\xF0W`\0\x80\xFD[a\x18\xFC\x8A\x83\x8B\x01a\x11\xB7V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15a\x19\x12W`\0\x80\xFD[Pa\x19\x1F\x89\x82\x8A\x01a\x11\xB7V[\x92PP`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x84Qa\x19E\x81\x84` \x89\x01a\x12\xD3V[\x84Q\x90\x83\x01\x90a\x19Y\x81\x83` \x89\x01a\x12\xD3V[\x84Q\x91\x01\x90a\x19l\x81\x83` \x88\x01a\x12\xD3V[\x01\x95\x94PPPPPV[`\x1F\x82\x11\x15a\x0E\xEFW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x19\x9DWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x19\xBCW\x82\x81U`\x01\x01a\x19\xA9V[PPPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xDDWa\x19\xDDa\x11\"V[a\x19\xF1\x81a\x19\xEB\x84Ta\x17@V[\x84a\x19vV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1A&W`\0\x84\x15a\x1A\x0EWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x19\xBCV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x1AUW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1A6V[P\x85\x82\x10\x15a\x1AsW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[``\x81R`\0a\x1A\x96``\x83\x01\x86a\x12\xF7V[\x82\x81\x03` \x84\x01Ra\x1A\xA8\x81\x86a\x12\xF7V[\x90P\x82\x81\x03`@\x84\x01Ra\x1A\xBC\x81\x85a\x12\xF7V[\x96\x95PPPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x06\xFDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x1B\x04WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`@\x81R`\0a\x1B\x1C`@\x83\x01\x85a\x12\xF7V[\x82\x81\x03` \x84\x01R\x83Q`\xA0\x82Ra\x1B7`\xA0\x83\x01\x82a\x12\xF7V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01Ra\x1BP\x82\x82a\x12\xF7V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra\x1Bj\x82\x82a\x12\xF7V[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01Ra\x1B\x84\x82\x82a\x12\xF7V[\x91PP`\x80\x85\x01Q`\x80\x83\x01R\x80\x92PPP\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 (\xB5^\x17\x8CV\x07\x9D\xCD\xA9K\xEC)`\x92\xA7(\xDCN\xE4\xA2Q\xC9k\x06'~\xA38\xBE\xA4\xE1dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static TEE_VERIFIER_WRAPPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct tee_verifier_wrapper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for tee_verifier_wrapper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for tee_verifier_wrapper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for tee_verifier_wrapper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for tee_verifier_wrapper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(tee_verifier_wrapper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> tee_verifier_wrapper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                TEE_VERIFIER_WRAPPER_ABI.clone(),
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
                TEE_VERIFIER_WRAPPER_ABI.clone(),
                TEE_VERIFIER_WRAPPER_BYTECODE.clone().into(),
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
        ///Calls the contract's `addEnclaveImageToFamily` (0x1084d65c) function
        pub fn add_enclave_image_to_family(
            &self,
            prover_pcr: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 132, 214, 92], prover_pcr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSampleInputsAndProof` (0x10a54279) function
        pub fn check_sample_inputs_and_proof(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([16, 165, 66, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeInputAndProofForVerification` (0x5be559af) function
        pub fn encode_input_and_proof_for_verification(
            &self,
            inputs: ::std::vec::Vec<::std::string::String>,
            proof: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([91, 229, 89, 175], (inputs, proof))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeInputs` (0xf6ea9962) function
        pub fn encode_inputs(
            &self,
            inputs: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([246, 234, 153, 98], inputs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeProof` (0x81fd3f57) function
        pub fn encode_proof(
            &self,
            proof: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([129, 253, 63, 87], proof)
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
        ///Calls the contract's `sampleInput` (0x7d8ad42b) function
        pub fn sample_input(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([125, 138, 212, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sampleProof` (0xa76c0551) function
        pub fn sample_proof(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([167, 108, 5, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0x8e760afe) function
        pub fn verify(
            &self,
            encoded_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([142, 118, 10, 254], encoded_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyAgainstSampleInputs` (0x02f77d19) function
        pub fn verify_against_sample_inputs(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([2, 247, 125, 25], p0)
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
        ///Calls the contract's `verifyInputs` (0xa6dfbc7f) function
        pub fn verify_inputs(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 223, 188, 127], p0)
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
        ///Calls the contract's `verifyProofForTeeVerifier` (0xd2e89832) function
        pub fn verify_proof_for_tee_verifier(
            &self,
            prover_data: ::ethers::core::types::Bytes,
            proof_data: ::ethers::core::types::Bytes,
            proof_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [210, 232, 152, 50],
                    (prover_data, proof_data, proof_signature),
                )
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, tee_verifier_wrapperEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for tee_verifier_wrapper<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    pub enum tee_verifier_wrapperErrors {
        AttestationAutherAttestationTooOld(AttestationAutherAttestationTooOld),
        AttestationAutherImageNotInFamily(AttestationAutherImageNotInFamily),
        AttestationAutherImageNotWhitelisted(AttestationAutherImageNotWhitelisted),
        AttestationAutherKeyNotVerified(AttestationAutherKeyNotVerified),
        AttestationAutherMismatchedLengths(AttestationAutherMismatchedLengths),
        AttestationAutherPCRsInvalid(AttestationAutherPCRsInvalid),
        AttestationAutherPubkeyLengthInvalid(AttestationAutherPubkeyLengthInvalid),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        InferredImageIdIsDifferent(InferredImageIdIsDifferent),
        InvalidEnclaveSignature(InvalidEnclaveSignature),
        InvalidInputs(InvalidInputs),
        MustBeAnEnclave(MustBeAnEnclave),
        OnlyAdminCanCall(OnlyAdminCanCall),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for tee_verifier_wrapperErrors {
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
                <InferredImageIdIsDifferent as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InferredImageIdIsDifferent(decoded));
            }
            if let Ok(decoded) =
                <InvalidEnclaveSignature as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidEnclaveSignature(decoded));
            }
            if let Ok(decoded) = <InvalidInputs as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidInputs(decoded));
            }
            if let Ok(decoded) = <MustBeAnEnclave as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MustBeAnEnclave(decoded));
            }
            if let Ok(decoded) = <OnlyAdminCanCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyAdminCanCall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for tee_verifier_wrapperErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
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
                Self::ECDSAInvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InferredImageIdIsDifferent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEnclaveSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInputs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MustBeAnEnclave(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyAdminCanCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for tee_verifier_wrapperErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
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
                    == <InferredImageIdIsDifferent as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidEnclaveSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInputs as ::ethers::contract::EthError>::selector() => {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for tee_verifier_wrapperErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
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
                Self::ECDSAInvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureS(element) => ::core::fmt::Display::fmt(element, f),
                Self::InferredImageIdIsDifferent(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEnclaveSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::MustBeAnEnclave(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyAdminCanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for tee_verifier_wrapperErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AttestationAutherAttestationTooOld> for tee_verifier_wrapperErrors {
        fn from(value: AttestationAutherAttestationTooOld) -> Self {
            Self::AttestationAutherAttestationTooOld(value)
        }
    }
    impl ::core::convert::From<AttestationAutherImageNotInFamily> for tee_verifier_wrapperErrors {
        fn from(value: AttestationAutherImageNotInFamily) -> Self {
            Self::AttestationAutherImageNotInFamily(value)
        }
    }
    impl ::core::convert::From<AttestationAutherImageNotWhitelisted> for tee_verifier_wrapperErrors {
        fn from(value: AttestationAutherImageNotWhitelisted) -> Self {
            Self::AttestationAutherImageNotWhitelisted(value)
        }
    }
    impl ::core::convert::From<AttestationAutherKeyNotVerified> for tee_verifier_wrapperErrors {
        fn from(value: AttestationAutherKeyNotVerified) -> Self {
            Self::AttestationAutherKeyNotVerified(value)
        }
    }
    impl ::core::convert::From<AttestationAutherMismatchedLengths> for tee_verifier_wrapperErrors {
        fn from(value: AttestationAutherMismatchedLengths) -> Self {
            Self::AttestationAutherMismatchedLengths(value)
        }
    }
    impl ::core::convert::From<AttestationAutherPCRsInvalid> for tee_verifier_wrapperErrors {
        fn from(value: AttestationAutherPCRsInvalid) -> Self {
            Self::AttestationAutherPCRsInvalid(value)
        }
    }
    impl ::core::convert::From<AttestationAutherPubkeyLengthInvalid> for tee_verifier_wrapperErrors {
        fn from(value: AttestationAutherPubkeyLengthInvalid) -> Self {
            Self::AttestationAutherPubkeyLengthInvalid(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for tee_verifier_wrapperErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength> for tee_verifier_wrapperErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for tee_verifier_wrapperErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<InferredImageIdIsDifferent> for tee_verifier_wrapperErrors {
        fn from(value: InferredImageIdIsDifferent) -> Self {
            Self::InferredImageIdIsDifferent(value)
        }
    }
    impl ::core::convert::From<InvalidEnclaveSignature> for tee_verifier_wrapperErrors {
        fn from(value: InvalidEnclaveSignature) -> Self {
            Self::InvalidEnclaveSignature(value)
        }
    }
    impl ::core::convert::From<InvalidInputs> for tee_verifier_wrapperErrors {
        fn from(value: InvalidInputs) -> Self {
            Self::InvalidInputs(value)
        }
    }
    impl ::core::convert::From<MustBeAnEnclave> for tee_verifier_wrapperErrors {
        fn from(value: MustBeAnEnclave) -> Self {
            Self::MustBeAnEnclave(value)
        }
    }
    impl ::core::convert::From<OnlyAdminCanCall> for tee_verifier_wrapperErrors {
        fn from(value: OnlyAdminCanCall) -> Self {
            Self::OnlyAdminCanCall(value)
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
    #[ethevent(name = "EnclaveKeyRevoked", abi = "EnclaveKeyRevoked(address)")]
    pub struct EnclaveKeyRevokedFilter {
        #[ethevent(indexed)]
        pub enclave_address: ::ethers::core::types::Address,
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
        name = "EnclaveKeyVerified",
        abi = "EnclaveKeyVerified(address,bytes32,bytes)"
    )]
    pub struct EnclaveKeyVerifiedFilter {
        #[ethevent(indexed)]
        pub enclave_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
        pub enclave_pub_key: ::ethers::core::types::Bytes,
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
        abi = "EnclaveKeyWhitelisted(address,bytes32,bytes)"
    )]
    pub struct EnclaveKeyWhitelistedFilter {
        #[ethevent(indexed)]
        pub enclave_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
        pub enclave_pub_key: ::ethers::core::types::Bytes,
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
    pub enum tee_verifier_wrapperEvents {
        EnclaveImageAddedToFamilyFilter(EnclaveImageAddedToFamilyFilter),
        EnclaveImageRemovedFromFamilyFilter(EnclaveImageRemovedFromFamilyFilter),
        EnclaveImageRevokedFilter(EnclaveImageRevokedFilter),
        EnclaveImageWhitelistedFilter(EnclaveImageWhitelistedFilter),
        EnclaveKeyRevokedFilter(EnclaveKeyRevokedFilter),
        EnclaveKeyVerifiedFilter(EnclaveKeyVerifiedFilter),
        EnclaveKeyWhitelistedFilter(EnclaveKeyWhitelistedFilter),
    }
    impl ::ethers::contract::EthLogDecode for tee_verifier_wrapperEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EnclaveImageAddedToFamilyFilter::decode_log(log) {
                return Ok(tee_verifier_wrapperEvents::EnclaveImageAddedToFamilyFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveImageRemovedFromFamilyFilter::decode_log(log) {
                return Ok(
                    tee_verifier_wrapperEvents::EnclaveImageRemovedFromFamilyFilter(decoded),
                );
            }
            if let Ok(decoded) = EnclaveImageRevokedFilter::decode_log(log) {
                return Ok(tee_verifier_wrapperEvents::EnclaveImageRevokedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveImageWhitelistedFilter::decode_log(log) {
                return Ok(tee_verifier_wrapperEvents::EnclaveImageWhitelistedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveKeyRevokedFilter::decode_log(log) {
                return Ok(tee_verifier_wrapperEvents::EnclaveKeyRevokedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyVerifiedFilter::decode_log(log) {
                return Ok(tee_verifier_wrapperEvents::EnclaveKeyVerifiedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveKeyWhitelistedFilter::decode_log(log) {
                return Ok(tee_verifier_wrapperEvents::EnclaveKeyWhitelistedFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for tee_verifier_wrapperEvents {
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
            }
        }
    }
    impl ::core::convert::From<EnclaveImageAddedToFamilyFilter> for tee_verifier_wrapperEvents {
        fn from(value: EnclaveImageAddedToFamilyFilter) -> Self {
            Self::EnclaveImageAddedToFamilyFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRemovedFromFamilyFilter> for tee_verifier_wrapperEvents {
        fn from(value: EnclaveImageRemovedFromFamilyFilter) -> Self {
            Self::EnclaveImageRemovedFromFamilyFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRevokedFilter> for tee_verifier_wrapperEvents {
        fn from(value: EnclaveImageRevokedFilter) -> Self {
            Self::EnclaveImageRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageWhitelistedFilter> for tee_verifier_wrapperEvents {
        fn from(value: EnclaveImageWhitelistedFilter) -> Self {
            Self::EnclaveImageWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyRevokedFilter> for tee_verifier_wrapperEvents {
        fn from(value: EnclaveKeyRevokedFilter) -> Self {
            Self::EnclaveKeyRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyVerifiedFilter> for tee_verifier_wrapperEvents {
        fn from(value: EnclaveKeyVerifiedFilter) -> Self {
            Self::EnclaveKeyVerifiedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyWhitelistedFilter> for tee_verifier_wrapperEvents {
        fn from(value: EnclaveKeyWhitelistedFilter) -> Self {
            Self::EnclaveKeyWhitelistedFilter(value)
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
    ///Container type for all input parameters for the `addEnclaveImageToFamily` function with signature `addEnclaveImageToFamily(bytes)` and selector `0x1084d65c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "addEnclaveImageToFamily",
        abi = "addEnclaveImageToFamily(bytes)"
    )]
    pub struct AddEnclaveImageToFamilyCall {
        pub prover_pcr: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `admin` function with signature `admin()` and selector `0xf851a440`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `checkSampleInputsAndProof` function with signature `checkSampleInputsAndProof()` and selector `0x10a54279`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "checkSampleInputsAndProof",
        abi = "checkSampleInputsAndProof()"
    )]
    pub struct CheckSampleInputsAndProofCall;
    ///Container type for all input parameters for the `encodeInputAndProofForVerification` function with signature `encodeInputAndProofForVerification(string[],string)` and selector `0x5be559af`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "encodeInputAndProofForVerification",
        abi = "encodeInputAndProofForVerification(string[],string)"
    )]
    pub struct EncodeInputAndProofForVerificationCall {
        pub inputs: ::std::vec::Vec<::std::string::String>,
        pub proof: ::std::string::String,
    }
    ///Container type for all input parameters for the `encodeInputs` function with signature `encodeInputs(string[])` and selector `0xf6ea9962`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "encodeInputs", abi = "encodeInputs(string[])")]
    pub struct EncodeInputsCall {
        pub inputs: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all input parameters for the `encodeProof` function with signature `encodeProof(string)` and selector `0x81fd3f57`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "encodeProof", abi = "encodeProof(string)")]
    pub struct EncodeProofCall {
        pub proof: ::std::string::String,
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
    ///Container type for all input parameters for the `sampleInput` function with signature `sampleInput()` and selector `0x7d8ad42b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "sampleInput", abi = "sampleInput()")]
    pub struct SampleInputCall;
    ///Container type for all input parameters for the `sampleProof` function with signature `sampleProof()` and selector `0xa76c0551`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "sampleProof", abi = "sampleProof()")]
    pub struct SampleProofCall;
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
        pub encoded_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `verifyAgainstSampleInputs` function with signature `verifyAgainstSampleInputs(bytes)` and selector `0x02f77d19`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "verifyAgainstSampleInputs",
        abi = "verifyAgainstSampleInputs(bytes)"
    )]
    pub struct VerifyAgainstSampleInputsCall(pub ::ethers::core::types::Bytes);
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
    ///Container type for all input parameters for the `verifyInputs` function with signature `verifyInputs(bytes)` and selector `0xa6dfbc7f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "verifyInputs", abi = "verifyInputs(bytes)")]
    pub struct VerifyInputsCall(pub ::ethers::core::types::Bytes);
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
    ///Container type for all input parameters for the `verifyProofForTeeVerifier` function with signature `verifyProofForTeeVerifier(bytes,bytes,bytes)` and selector `0xd2e89832`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "verifyProofForTeeVerifier",
        abi = "verifyProofForTeeVerifier(bytes,bytes,bytes)"
    )]
    pub struct VerifyProofForTeeVerifierCall {
        pub prover_data: ::ethers::core::types::Bytes,
        pub proof_data: ::ethers::core::types::Bytes,
        pub proof_signature: ::ethers::core::types::Bytes,
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
    pub enum tee_verifier_wrapperCalls {
        AttestationMaxAge(AttestationMaxAgeCall),
        AttestationVerifier(AttestationVerifierCall),
        AddEnclaveImageToFamily(AddEnclaveImageToFamilyCall),
        Admin(AdminCall),
        CheckSampleInputsAndProof(CheckSampleInputsAndProofCall),
        EncodeInputAndProofForVerification(EncodeInputAndProofForVerificationCall),
        EncodeInputs(EncodeInputsCall),
        EncodeProof(EncodeProofCall),
        GetVerifiedKey(GetVerifiedKeyCall),
        GetWhitelistedImage(GetWhitelistedImageCall),
        IsImageInFamily(IsImageInFamilyCall),
        SampleInput(SampleInputCall),
        SampleProof(SampleProofCall),
        Verify(VerifyCall),
        VerifyAgainstSampleInputs(VerifyAgainstSampleInputsCall),
        VerifyEnclaveKey(VerifyEnclaveKeyCall),
        VerifyInputs(VerifyInputsCall),
        VerifyKey(VerifyKeyCall),
        VerifyProofForTeeVerifier(VerifyProofForTeeVerifierCall),
    }
    impl ::ethers::core::abi::AbiDecode for tee_verifier_wrapperCalls {
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
                <AddEnclaveImageToFamilyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddEnclaveImageToFamily(decoded));
            }
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) =
                <CheckSampleInputsAndProofCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSampleInputsAndProof(decoded));
            }
            if let Ok(decoded) =
                <EncodeInputAndProofForVerificationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::EncodeInputAndProofForVerification(decoded));
            }
            if let Ok(decoded) = <EncodeInputsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EncodeInputs(decoded));
            }
            if let Ok(decoded) = <EncodeProofCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EncodeProof(decoded));
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
            if let Ok(decoded) =
                <IsImageInFamilyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsImageInFamily(decoded));
            }
            if let Ok(decoded) = <SampleInputCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SampleInput(decoded));
            }
            if let Ok(decoded) = <SampleProofCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SampleProof(decoded));
            }
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verify(decoded));
            }
            if let Ok(decoded) =
                <VerifyAgainstSampleInputsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyAgainstSampleInputs(decoded));
            }
            if let Ok(decoded) =
                <VerifyEnclaveKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyEnclaveKey(decoded));
            }
            if let Ok(decoded) = <VerifyInputsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyInputs(decoded));
            }
            if let Ok(decoded) = <VerifyKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VerifyKey(decoded));
            }
            if let Ok(decoded) =
                <VerifyProofForTeeVerifierCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyProofForTeeVerifier(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for tee_verifier_wrapperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AttestationMaxAge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AttestationVerifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddEnclaveImageToFamily(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckSampleInputsAndProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeInputAndProofForVerification(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeInputs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EncodeProof(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetVerifiedKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetWhitelistedImage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsImageInFamily(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SampleInput(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SampleProof(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyAgainstSampleInputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyEnclaveKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyInputs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyProofForTeeVerifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for tee_verifier_wrapperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestationMaxAge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddEnclaveImageToFamily(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSampleInputsAndProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeInputAndProofForVerification(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVerifiedKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWhitelistedImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsImageInFamily(element) => ::core::fmt::Display::fmt(element, f),
                Self::SampleInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::SampleProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyAgainstSampleInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyProofForTeeVerifier(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestationMaxAgeCall> for tee_verifier_wrapperCalls {
        fn from(value: AttestationMaxAgeCall) -> Self {
            Self::AttestationMaxAge(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierCall> for tee_verifier_wrapperCalls {
        fn from(value: AttestationVerifierCall) -> Self {
            Self::AttestationVerifier(value)
        }
    }
    impl ::core::convert::From<AddEnclaveImageToFamilyCall> for tee_verifier_wrapperCalls {
        fn from(value: AddEnclaveImageToFamilyCall) -> Self {
            Self::AddEnclaveImageToFamily(value)
        }
    }
    impl ::core::convert::From<AdminCall> for tee_verifier_wrapperCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<CheckSampleInputsAndProofCall> for tee_verifier_wrapperCalls {
        fn from(value: CheckSampleInputsAndProofCall) -> Self {
            Self::CheckSampleInputsAndProof(value)
        }
    }
    impl ::core::convert::From<EncodeInputAndProofForVerificationCall> for tee_verifier_wrapperCalls {
        fn from(value: EncodeInputAndProofForVerificationCall) -> Self {
            Self::EncodeInputAndProofForVerification(value)
        }
    }
    impl ::core::convert::From<EncodeInputsCall> for tee_verifier_wrapperCalls {
        fn from(value: EncodeInputsCall) -> Self {
            Self::EncodeInputs(value)
        }
    }
    impl ::core::convert::From<EncodeProofCall> for tee_verifier_wrapperCalls {
        fn from(value: EncodeProofCall) -> Self {
            Self::EncodeProof(value)
        }
    }
    impl ::core::convert::From<GetVerifiedKeyCall> for tee_verifier_wrapperCalls {
        fn from(value: GetVerifiedKeyCall) -> Self {
            Self::GetVerifiedKey(value)
        }
    }
    impl ::core::convert::From<GetWhitelistedImageCall> for tee_verifier_wrapperCalls {
        fn from(value: GetWhitelistedImageCall) -> Self {
            Self::GetWhitelistedImage(value)
        }
    }
    impl ::core::convert::From<IsImageInFamilyCall> for tee_verifier_wrapperCalls {
        fn from(value: IsImageInFamilyCall) -> Self {
            Self::IsImageInFamily(value)
        }
    }
    impl ::core::convert::From<SampleInputCall> for tee_verifier_wrapperCalls {
        fn from(value: SampleInputCall) -> Self {
            Self::SampleInput(value)
        }
    }
    impl ::core::convert::From<SampleProofCall> for tee_verifier_wrapperCalls {
        fn from(value: SampleProofCall) -> Self {
            Self::SampleProof(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for tee_verifier_wrapperCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifyAgainstSampleInputsCall> for tee_verifier_wrapperCalls {
        fn from(value: VerifyAgainstSampleInputsCall) -> Self {
            Self::VerifyAgainstSampleInputs(value)
        }
    }
    impl ::core::convert::From<VerifyEnclaveKeyCall> for tee_verifier_wrapperCalls {
        fn from(value: VerifyEnclaveKeyCall) -> Self {
            Self::VerifyEnclaveKey(value)
        }
    }
    impl ::core::convert::From<VerifyInputsCall> for tee_verifier_wrapperCalls {
        fn from(value: VerifyInputsCall) -> Self {
            Self::VerifyInputs(value)
        }
    }
    impl ::core::convert::From<VerifyKeyCall> for tee_verifier_wrapperCalls {
        fn from(value: VerifyKeyCall) -> Self {
            Self::VerifyKey(value)
        }
    }
    impl ::core::convert::From<VerifyProofForTeeVerifierCall> for tee_verifier_wrapperCalls {
        fn from(value: VerifyProofForTeeVerifierCall) -> Self {
            Self::VerifyProofForTeeVerifier(value)
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
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `checkSampleInputsAndProof` function with signature `checkSampleInputsAndProof()` and selector `0x10a54279`
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
    pub struct CheckSampleInputsAndProofReturn(pub bool);
    ///Container type for all return fields from the `encodeInputAndProofForVerification` function with signature `encodeInputAndProofForVerification(string[],string)` and selector `0x5be559af`
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
    pub struct EncodeInputAndProofForVerificationReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `encodeInputs` function with signature `encodeInputs(string[])` and selector `0xf6ea9962`
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
    pub struct EncodeInputsReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `encodeProof` function with signature `encodeProof(string)` and selector `0x81fd3f57`
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
    pub struct EncodeProofReturn(pub ::ethers::core::types::Bytes);
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
    ///Container type for all return fields from the `sampleInput` function with signature `sampleInput()` and selector `0x7d8ad42b`
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
    pub struct SampleInputReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `sampleProof` function with signature `sampleProof()` and selector `0xa76c0551`
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
    pub struct SampleProofReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `verify` function with signature `verify(bytes)` and selector `0x8e760afe`
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
    pub struct VerifyReturn(pub bool);
    ///Container type for all return fields from the `verifyAgainstSampleInputs` function with signature `verifyAgainstSampleInputs(bytes)` and selector `0x02f77d19`
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
    pub struct VerifyAgainstSampleInputsReturn(pub bool);
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
    ///Container type for all return fields from the `verifyInputs` function with signature `verifyInputs(bytes)` and selector `0xa6dfbc7f`
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
    pub struct VerifyInputsReturn(pub bool);
    ///Container type for all return fields from the `verifyProofForTeeVerifier` function with signature `verifyProofForTeeVerifier(bytes,bytes,bytes)` and selector `0xd2e89832`
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
    pub struct VerifyProofForTeeVerifierReturn(pub bool);
}
