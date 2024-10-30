pub use attestation_auther_upgradeable::*;
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
pub mod attestation_auther_upgradeable {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("attestationVerifier"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IAttestationVerifier",),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("maxAge"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ATTESTATIONAUTHERUPGRADEABLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0B\x828\x03\x80a\x0B\x82\x839\x81\x01`@\x81\x90Ra\0/\x91a\0EV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x80R`\xA0Ra\0\x7FV[`\0\x80`@\x83\x85\x03\x12\x15a\0XW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0oW`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\x80Q`\xA0Qa\n\xD0a\0\xB2`\09`\0\x81\x81a\x01\x1D\x01Ra\x04\x94\x01R`\0\x81\x81a\x01D\x01Ra\x05\0\x01Ra\n\xD0`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x01\xD5\x8F\xA3\x14a\0gW\x80c$\x10\xF6\xBA\x14a\0\xC2W\x80ck[!\xA6\x14a\0\xE2W\x80cu\x84{\x84\x14a\x01\x05W\x80c\x9A\xEC\x99\x0E\x14a\x01\x18W\x80c\xCDy\xF9\x06\x14a\x01?W[`\0\x80\xFD[a\0\xAFa\0u6`\x04a\x06?V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD5a\0\xD06`\x04a\x06hV[a\x01~V[`@Qa\0\xB9\x91\x90a\x06\xD1V[a\0\xF5a\0\xF06`\x04a\x072V[a\x03\x9CV[`@Q\x90\x15\x15\x81R` \x01a\0\xB9V[a\0\xF5a\x01\x136`\x04a\x08 V[a\x03\xDFV[a\0\xAF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01f\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB9V[a\x01\xA2`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\0` \x81\x90R`@\x91\x82\x90 \x82Q``\x81\x01\x90\x93R\x80T\x91\x92\x91\x82\x90\x82\x90a\x01\xEE\x90a\t*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x1A\x90a\t*V[\x80\x15a\x02gW\x80`\x1F\x10a\x02<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02gV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02JW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x02\x80\x90a\t*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xAC\x90a\t*V[\x80\x15a\x02\xF9W\x80`\x1F\x10a\x02\xCEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xF9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xDCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x03\x12\x90a\t*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03>\x90a\t*V[\x80\x15a\x03\x8BW\x80`\x1F\x10a\x03`Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x8BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03nW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[`\0\x81\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T`\xFF\x16[\x92\x91PPV[`\0a\x03\xEB\x83\x83a\x03\xF2V[\x93\x92PPPV[`\0\x80\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\0\x90P`\0\x83` \x01Q\x84`@\x01Q\x85``\x01Q`@Q` \x01a\x04;\x93\x92\x91\x90a\tdV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x85\x90R\x91 \x80T\x91\x92P\x90a\x04m\x90a\t*V[\x90P`\0\x03a\x04\x8FW`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba\t\xA7V[a\x03\xE8\x85`\x80\x01Qa\x04\xCB\x91\x90a\t\xC8V[\x11a\x04\xE9W`@Qc\x0C\xB0/\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xEA\xC7\x08\xA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEA\xC7\x08\xA3\x90a\x057\x90\x88\x90\x88\x90`\x04\x01a\t\xEAV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05OW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x05cW=`\0\x80>=`\0\xFD[PPPP`\0a\x05v\x85`\0\x01Qa\x06\x0FV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x85\x01` R`@\x90 T\x90\x91P\x15a\x05\xA5W`\0\x93PPPPa\x03\xD9V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x84\x01` R`@\x90\x81\x90 \x83\x90U\x85Q\x90Q\x83\x91a\x05\xD3\x91a\n~V[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF4\x83\x7F\x1Af\xA1\xE5\xFB\xDE,@\xD0\xF6\xEF\xF7Q\x83\x13\x9Ey\xB0\xFC\x1E\xAA\xB2\x80\xA3\xA0\xBE8\xE7\xB1\x90`\0\x90\xA3P`\x01\x95\x94PPPPPV[`\0\x81Q`@\x14a\x063W`@Qc\xBD\x9C\x80\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[`\0` \x82\x84\x03\x12\x15a\x06QW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xEBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x06zW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x06\x9CW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\x84V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x06\xBD\x81` \x86\x01` \x86\x01a\x06\x81V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0\x82Q``` \x84\x01Ra\x06\xED`\x80\x84\x01\x82a\x06\xA5V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra\x07\x0B\x83\x83a\x06\xA5V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa\x07)\x82\x82a\x06\xA5V[\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07EW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x8DWa\x07\x8Da\x07TV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\xA4W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xBFWa\x07\xBFa\x07TV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x07\xE7Wa\x07\xE7a\x07TV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x08\0W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x083W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08KW`\0\x80\xFD[a\x08W\x86\x83\x87\x01a\x07\x93V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x08mW`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\x08\x81W`\0\x80\xFD[a\x08\x89a\x07jV[\x825\x82\x81\x11\x15a\x08\x98W`\0\x80\xFD[a\x08\xA4\x88\x82\x86\x01a\x07\x93V[\x82RP` \x83\x015\x82\x81\x11\x15a\x08\xB9W`\0\x80\xFD[a\x08\xC5\x88\x82\x86\x01a\x07\x93V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x08\xDDW`\0\x80\xFD[a\x08\xE9\x88\x82\x86\x01a\x07\x93V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\t\x01W`\0\x80\xFD[a\t\r\x88\x82\x86\x01a\x07\x93V[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\t>W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t^WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x84Qa\tv\x81\x84` \x89\x01a\x06\x81V[\x84Q\x90\x83\x01\x90a\t\x8A\x81\x83` \x89\x01a\x06\x81V[\x84Q\x91\x01\x90a\t\x9D\x81\x83` \x88\x01a\x06\x81V[\x01\x95\x94PPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x03\xD9WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\t\xE5WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`@\x81R`\0a\t\xFD`@\x83\x01\x85a\x06\xA5V[\x82\x81\x03` \x84\x01R\x83Q`\xA0\x82Ra\n\x18`\xA0\x83\x01\x82a\x06\xA5V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01Ra\n1\x82\x82a\x06\xA5V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra\nK\x82\x82a\x06\xA5V[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01Ra\ne\x82\x82a\x06\xA5V[\x91PP`\x80\x85\x01Q`\x80\x83\x01R\x80\x92PPP\x93\x92PPPV[`\0\x82Qa\n\x90\x81\x84` \x87\x01a\x06\x81V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 R\x94\x98\xFA9\xD1Q\x0C\x1B\x1Ci\xD7\xE8s\x04\x87\xE4\x85P\xCDZZ\x1C\\\xD6\xBA\x7FZ\xDE3\xE3\x82dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ATTESTATIONAUTHERUPGRADEABLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x01\xD5\x8F\xA3\x14a\0gW\x80c$\x10\xF6\xBA\x14a\0\xC2W\x80ck[!\xA6\x14a\0\xE2W\x80cu\x84{\x84\x14a\x01\x05W\x80c\x9A\xEC\x99\x0E\x14a\x01\x18W\x80c\xCDy\xF9\x06\x14a\x01?W[`\0\x80\xFD[a\0\xAFa\0u6`\x04a\x06?V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD5a\0\xD06`\x04a\x06hV[a\x01~V[`@Qa\0\xB9\x91\x90a\x06\xD1V[a\0\xF5a\0\xF06`\x04a\x072V[a\x03\x9CV[`@Q\x90\x15\x15\x81R` \x01a\0\xB9V[a\0\xF5a\x01\x136`\x04a\x08 V[a\x03\xDFV[a\0\xAF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01f\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB9V[a\x01\xA2`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\0` \x81\x90R`@\x91\x82\x90 \x82Q``\x81\x01\x90\x93R\x80T\x91\x92\x91\x82\x90\x82\x90a\x01\xEE\x90a\t*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x1A\x90a\t*V[\x80\x15a\x02gW\x80`\x1F\x10a\x02<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02gV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02JW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x02\x80\x90a\t*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xAC\x90a\t*V[\x80\x15a\x02\xF9W\x80`\x1F\x10a\x02\xCEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xF9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xDCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x03\x12\x90a\t*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03>\x90a\t*V[\x80\x15a\x03\x8BW\x80`\x1F\x10a\x03`Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x8BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03nW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[`\0\x81\x81R\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T`\xFF\x16[\x92\x91PPV[`\0a\x03\xEB\x83\x83a\x03\xF2V[\x93\x92PPPV[`\0\x80\x7F\xC1{Kp\x8BoD%\\ \x91:\x9D\x97\xA0S\0\xB6p4,q\xFEZ\xE5\xB6\x17\xBDM\xB5P\0\x90P`\0\x83` \x01Q\x84`@\x01Q\x85``\x01Q`@Q` \x01a\x04;\x93\x92\x91\x90a\tdV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x85\x90R\x91 \x80T\x91\x92P\x90a\x04m\x90a\t*V[\x90P`\0\x03a\x04\x8FW`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba\t\xA7V[a\x03\xE8\x85`\x80\x01Qa\x04\xCB\x91\x90a\t\xC8V[\x11a\x04\xE9W`@Qc\x0C\xB0/\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xEA\xC7\x08\xA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEA\xC7\x08\xA3\x90a\x057\x90\x88\x90\x88\x90`\x04\x01a\t\xEAV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05OW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x05cW=`\0\x80>=`\0\xFD[PPPP`\0a\x05v\x85`\0\x01Qa\x06\x0FV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x85\x01` R`@\x90 T\x90\x91P\x15a\x05\xA5W`\0\x93PPPPa\x03\xD9V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x84\x01` R`@\x90\x81\x90 \x83\x90U\x85Q\x90Q\x83\x91a\x05\xD3\x91a\n~V[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF4\x83\x7F\x1Af\xA1\xE5\xFB\xDE,@\xD0\xF6\xEF\xF7Q\x83\x13\x9Ey\xB0\xFC\x1E\xAA\xB2\x80\xA3\xA0\xBE8\xE7\xB1\x90`\0\x90\xA3P`\x01\x95\x94PPPPPV[`\0\x81Q`@\x14a\x063W`@Qc\xBD\x9C\x80\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[`\0` \x82\x84\x03\x12\x15a\x06QW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xEBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x06zW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x06\x9CW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\x84V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x06\xBD\x81` \x86\x01` \x86\x01a\x06\x81V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0\x82Q``` \x84\x01Ra\x06\xED`\x80\x84\x01\x82a\x06\xA5V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra\x07\x0B\x83\x83a\x06\xA5V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa\x07)\x82\x82a\x06\xA5V[\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07EW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x8DWa\x07\x8Da\x07TV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\xA4W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xBFWa\x07\xBFa\x07TV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x07\xE7Wa\x07\xE7a\x07TV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x08\0W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x083W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08KW`\0\x80\xFD[a\x08W\x86\x83\x87\x01a\x07\x93V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x08mW`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\x08\x81W`\0\x80\xFD[a\x08\x89a\x07jV[\x825\x82\x81\x11\x15a\x08\x98W`\0\x80\xFD[a\x08\xA4\x88\x82\x86\x01a\x07\x93V[\x82RP` \x83\x015\x82\x81\x11\x15a\x08\xB9W`\0\x80\xFD[a\x08\xC5\x88\x82\x86\x01a\x07\x93V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x08\xDDW`\0\x80\xFD[a\x08\xE9\x88\x82\x86\x01a\x07\x93V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\t\x01W`\0\x80\xFD[a\t\r\x88\x82\x86\x01a\x07\x93V[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\t>W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t^WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x84Qa\tv\x81\x84` \x89\x01a\x06\x81V[\x84Q\x90\x83\x01\x90a\t\x8A\x81\x83` \x89\x01a\x06\x81V[\x84Q\x91\x01\x90a\t\x9D\x81\x83` \x88\x01a\x06\x81V[\x01\x95\x94PPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x03\xD9WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\t\xE5WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`@\x81R`\0a\t\xFD`@\x83\x01\x85a\x06\xA5V[\x82\x81\x03` \x84\x01R\x83Q`\xA0\x82Ra\n\x18`\xA0\x83\x01\x82a\x06\xA5V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01Ra\n1\x82\x82a\x06\xA5V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra\nK\x82\x82a\x06\xA5V[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01Ra\ne\x82\x82a\x06\xA5V[\x91PP`\x80\x85\x01Q`\x80\x83\x01R\x80\x92PPP\x93\x92PPPV[`\0\x82Qa\n\x90\x81\x84` \x87\x01a\x06\x81V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 R\x94\x98\xFA9\xD1Q\x0C\x1B\x1Ci\xD7\xE8s\x04\x87\xE4\x85P\xCDZZ\x1C\\\xD6\xBA\x7FZ\xDE3\xE3\x82dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ATTESTATIONAUTHERUPGRADEABLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct AttestationAutherUpgradeable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AttestationAutherUpgradeable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AttestationAutherUpgradeable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AttestationAutherUpgradeable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AttestationAutherUpgradeable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AttestationAutherUpgradeable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AttestationAutherUpgradeable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ATTESTATIONAUTHERUPGRADEABLE_ABI.clone(),
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
                ATTESTATIONAUTHERUPGRADEABLE_ABI.clone(),
                ATTESTATIONAUTHERUPGRADEABLE_BYTECODE.clone().into(),
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
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AttestationAutherUpgradeableEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for AttestationAutherUpgradeable<M>
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
    pub enum AttestationAutherUpgradeableErrors {
        AttestationAutherAttestationTooOld(AttestationAutherAttestationTooOld),
        AttestationAutherImageNotInFamily(AttestationAutherImageNotInFamily),
        AttestationAutherImageNotWhitelisted(AttestationAutherImageNotWhitelisted),
        AttestationAutherKeyNotVerified(AttestationAutherKeyNotVerified),
        AttestationAutherMismatchedLengths(AttestationAutherMismatchedLengths),
        AttestationAutherPCRsInvalid(AttestationAutherPCRsInvalid),
        AttestationAutherPubkeyLengthInvalid(AttestationAutherPubkeyLengthInvalid),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationAutherUpgradeableErrors {
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
                <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <NotInitializing as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotInitializing(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AttestationAutherUpgradeableErrors {
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
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitializing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AttestationAutherUpgradeableErrors {
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
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitializing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AttestationAutherUpgradeableErrors {
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
                Self::InvalidInitialization(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AttestationAutherUpgradeableErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AttestationAutherAttestationTooOld>
        for AttestationAutherUpgradeableErrors
    {
        fn from(value: AttestationAutherAttestationTooOld) -> Self {
            Self::AttestationAutherAttestationTooOld(value)
        }
    }
    impl ::core::convert::From<AttestationAutherImageNotInFamily>
        for AttestationAutherUpgradeableErrors
    {
        fn from(value: AttestationAutherImageNotInFamily) -> Self {
            Self::AttestationAutherImageNotInFamily(value)
        }
    }
    impl ::core::convert::From<AttestationAutherImageNotWhitelisted>
        for AttestationAutherUpgradeableErrors
    {
        fn from(value: AttestationAutherImageNotWhitelisted) -> Self {
            Self::AttestationAutherImageNotWhitelisted(value)
        }
    }
    impl ::core::convert::From<AttestationAutherKeyNotVerified> for AttestationAutherUpgradeableErrors {
        fn from(value: AttestationAutherKeyNotVerified) -> Self {
            Self::AttestationAutherKeyNotVerified(value)
        }
    }
    impl ::core::convert::From<AttestationAutherMismatchedLengths>
        for AttestationAutherUpgradeableErrors
    {
        fn from(value: AttestationAutherMismatchedLengths) -> Self {
            Self::AttestationAutherMismatchedLengths(value)
        }
    }
    impl ::core::convert::From<AttestationAutherPCRsInvalid> for AttestationAutherUpgradeableErrors {
        fn from(value: AttestationAutherPCRsInvalid) -> Self {
            Self::AttestationAutherPCRsInvalid(value)
        }
    }
    impl ::core::convert::From<AttestationAutherPubkeyLengthInvalid>
        for AttestationAutherUpgradeableErrors
    {
        fn from(value: AttestationAutherPubkeyLengthInvalid) -> Self {
            Self::AttestationAutherPubkeyLengthInvalid(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for AttestationAutherUpgradeableErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for AttestationAutherUpgradeableErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint64)")]
    pub struct InitializedFilter {
        pub version: u64,
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
    pub enum AttestationAutherUpgradeableEvents {
        EnclaveImageAddedToFamilyFilter(EnclaveImageAddedToFamilyFilter),
        EnclaveImageRemovedFromFamilyFilter(EnclaveImageRemovedFromFamilyFilter),
        EnclaveImageRevokedFilter(EnclaveImageRevokedFilter),
        EnclaveImageWhitelistedFilter(EnclaveImageWhitelistedFilter),
        EnclaveKeyRevokedFilter(EnclaveKeyRevokedFilter),
        EnclaveKeyVerifiedFilter(EnclaveKeyVerifiedFilter),
        EnclaveKeyWhitelistedFilter(EnclaveKeyWhitelistedFilter),
        InitializedFilter(InitializedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AttestationAutherUpgradeableEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EnclaveImageAddedToFamilyFilter::decode_log(log) {
                return Ok(
                    AttestationAutherUpgradeableEvents::EnclaveImageAddedToFamilyFilter(decoded),
                );
            }
            if let Ok(decoded) = EnclaveImageRemovedFromFamilyFilter::decode_log(log) {
                return Ok(
                    AttestationAutherUpgradeableEvents::EnclaveImageRemovedFromFamilyFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = EnclaveImageRevokedFilter::decode_log(log) {
                return Ok(AttestationAutherUpgradeableEvents::EnclaveImageRevokedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageWhitelistedFilter::decode_log(log) {
                return Ok(
                    AttestationAutherUpgradeableEvents::EnclaveImageWhitelistedFilter(decoded),
                );
            }
            if let Ok(decoded) = EnclaveKeyRevokedFilter::decode_log(log) {
                return Ok(AttestationAutherUpgradeableEvents::EnclaveKeyRevokedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveKeyVerifiedFilter::decode_log(log) {
                return Ok(AttestationAutherUpgradeableEvents::EnclaveKeyVerifiedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyWhitelistedFilter::decode_log(log) {
                return Ok(
                    AttestationAutherUpgradeableEvents::EnclaveKeyWhitelistedFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(AttestationAutherUpgradeableEvents::InitializedFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AttestationAutherUpgradeableEvents {
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
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EnclaveImageAddedToFamilyFilter> for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveImageAddedToFamilyFilter) -> Self {
            Self::EnclaveImageAddedToFamilyFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRemovedFromFamilyFilter>
        for AttestationAutherUpgradeableEvents
    {
        fn from(value: EnclaveImageRemovedFromFamilyFilter) -> Self {
            Self::EnclaveImageRemovedFromFamilyFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRevokedFilter> for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveImageRevokedFilter) -> Self {
            Self::EnclaveImageRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageWhitelistedFilter> for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveImageWhitelistedFilter) -> Self {
            Self::EnclaveImageWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyRevokedFilter> for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveKeyRevokedFilter) -> Self {
            Self::EnclaveKeyRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyVerifiedFilter> for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveKeyVerifiedFilter) -> Self {
            Self::EnclaveKeyVerifiedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyWhitelistedFilter> for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveKeyWhitelistedFilter) -> Self {
            Self::EnclaveKeyWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for AttestationAutherUpgradeableEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
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
    pub enum AttestationAutherUpgradeableCalls {
        AttestationMaxAge(AttestationMaxAgeCall),
        AttestationVerifier(AttestationVerifierCall),
        GetVerifiedKey(GetVerifiedKeyCall),
        GetWhitelistedImage(GetWhitelistedImageCall),
        IsImageInFamily(IsImageInFamilyCall),
        VerifyEnclaveKey(VerifyEnclaveKeyCall),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationAutherUpgradeableCalls {
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
            if let Ok(decoded) =
                <VerifyEnclaveKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyEnclaveKey(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AttestationAutherUpgradeableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AttestationMaxAge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AttestationVerifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVerifiedKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetWhitelistedImage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsImageInFamily(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyEnclaveKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AttestationAutherUpgradeableCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestationMaxAge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationVerifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVerifiedKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWhitelistedImage(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsImageInFamily(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestationMaxAgeCall> for AttestationAutherUpgradeableCalls {
        fn from(value: AttestationMaxAgeCall) -> Self {
            Self::AttestationMaxAge(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierCall> for AttestationAutherUpgradeableCalls {
        fn from(value: AttestationVerifierCall) -> Self {
            Self::AttestationVerifier(value)
        }
    }
    impl ::core::convert::From<GetVerifiedKeyCall> for AttestationAutherUpgradeableCalls {
        fn from(value: GetVerifiedKeyCall) -> Self {
            Self::GetVerifiedKey(value)
        }
    }
    impl ::core::convert::From<GetWhitelistedImageCall> for AttestationAutherUpgradeableCalls {
        fn from(value: GetWhitelistedImageCall) -> Self {
            Self::GetWhitelistedImage(value)
        }
    }
    impl ::core::convert::From<IsImageInFamilyCall> for AttestationAutherUpgradeableCalls {
        fn from(value: IsImageInFamilyCall) -> Self {
            Self::IsImageInFamily(value)
        }
    }
    impl ::core::convert::From<VerifyEnclaveKeyCall> for AttestationAutherUpgradeableCalls {
        fn from(value: VerifyEnclaveKeyCall) -> Self {
            Self::VerifyEnclaveKey(value)
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
