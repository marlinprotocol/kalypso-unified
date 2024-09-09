pub use attestation_auther::*;
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
pub mod attestation_auther {
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ATTESTATIONAUTHER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\n\xE68\x03\x80a\n\xE6\x839\x81\x01`@\x81\x90Ra\0/\x91a\0EV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x80R`\xA0Ra\0\x7FV[`\0\x80`@\x83\x85\x03\x12\x15a\0XW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0oW`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\x80Q`\xA0Qa\n5a\0\xB1`\09`\0\x81\x81`\xFE\x01Ra\x04\x0C\x01R`\0\x81\x81a\x01%\x01Ra\x04x\x01Ra\n5`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x01\xD5\x8F\xA3\x14a\0gW\x80c$\x10\xF6\xBA\x14a\0\xA3W\x80ck[!\xA6\x14a\0\xC3W\x80cu\x84{\x84\x14a\0\xE6W\x80c\x9A\xEC\x99\x0E\x14a\0\xF9W\x80c\xCDy\xF9\x06\x14a\x01 W[`\0\x80\xFD[a\0\x90a\0u6`\x04a\x05\xADV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xB16`\x04a\x05\xD6V[a\x01_V[`@Qa\0\x9A\x91\x90a\x06?V[a\0\xD6a\0\xD16`\x04a\x06\xA0V[a\x03XV[`@Q\x90\x15\x15\x81R` \x01a\0\x9AV[a\0\xD6a\0\xF46`\x04a\x07\x8EV[a\x03|V[a\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01G\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x9AV[a\x01\x83`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81R` \x81\x90R`@\x90\x81\x90 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x01\xAB\x90a\x08\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xD7\x90a\x08\x98V[\x80\x15a\x02$W\x80`\x1F\x10a\x01\xF9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02$V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x07W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x02=\x90a\x08\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02i\x90a\x08\x98V[\x80\x15a\x02\xB6W\x80`\x1F\x10a\x02\x8BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xB6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x99W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x02\xCF\x90a\x08\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xFB\x90a\x08\x98V[\x80\x15a\x03HW\x80`\x1F\x10a\x03\x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03HV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03+W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T`\xFF\x16[\x92\x91PPV[`\0a\x03\x88\x83\x83a\x03\x8FV[\x93\x92PPPV[`\0\x80\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x03\xB3\x93\x92\x91\x90a\x08\xD2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x83\x90R\x91 \x80T\x91\x92P\x90a\x03\xE5\x90a\x08\x98V[\x90P`\0\x03a\x04\x07W`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x041\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba\t\x15V[a\x03\xE8\x84`\x80\x01Qa\x04C\x91\x90a\t6V[\x11a\x04aW`@Qc\x0C\xB0/\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xEA\xC7\x08\xA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEA\xC7\x08\xA3\x90a\x04\xAF\x90\x87\x90\x87\x90`\x04\x01a\tXV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xC7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x04\xDBW=`\0\x80>=`\0\xFD[PPPP`\0a\x04\xEE\x84`\0\x01Qa\x05}V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x90\x91P\x15a\x05\x1AW`\0\x92PPPa\x03vV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x01` R`@\x90\x81\x90 \x84\x90U\x85Q\x90Q\x84\x92\x91\x7F\xBBMd(\xD5>\xA9$\xD9K\xE0H\x84s\xDE\xB5\xC0\xA7\x0C\x97\x9F\x84w\xD2\x19\x11\xCB[\xEE@7\xFD\x91a\x05j\x91\x90a\t\xECV[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0\x81Q`@\x14a\x05\xA1W`@Qc\xBD\x9C\x80\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[`\0` \x82\x84\x03\x12\x15a\x05\xBFW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x88W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x05\xE8W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x06\nW\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xF2V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x06+\x81` \x86\x01` \x86\x01a\x05\xEFV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0\x82Q``` \x84\x01Ra\x06[`\x80\x84\x01\x82a\x06\x13V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra\x06y\x83\x83a\x06\x13V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa\x06\x97\x82\x82a\x06\x13V[\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xB3W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\xFBWa\x06\xFBa\x06\xC2V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\x12W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07-Wa\x07-a\x06\xC2V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x07UWa\x07Ua\x06\xC2V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x07nW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xA1W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xB9W`\0\x80\xFD[a\x07\xC5\x86\x83\x87\x01a\x07\x01V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x07\xDBW`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\x07\xEFW`\0\x80\xFD[a\x07\xF7a\x06\xD8V[\x825\x82\x81\x11\x15a\x08\x06W`\0\x80\xFD[a\x08\x12\x88\x82\x86\x01a\x07\x01V[\x82RP` \x83\x015\x82\x81\x11\x15a\x08'W`\0\x80\xFD[a\x083\x88\x82\x86\x01a\x07\x01V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x08KW`\0\x80\xFD[a\x08W\x88\x82\x86\x01a\x07\x01V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\x08oW`\0\x80\xFD[a\x08{\x88\x82\x86\x01a\x07\x01V[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x08\xACW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x08\xCCWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x84Qa\x08\xE4\x81\x84` \x89\x01a\x05\xEFV[\x84Q\x90\x83\x01\x90a\x08\xF8\x81\x83` \x89\x01a\x05\xEFV[\x84Q\x91\x01\x90a\t\x0B\x81\x83` \x88\x01a\x05\xEFV[\x01\x95\x94PPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x03vWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\tSWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`@\x81R`\0a\tk`@\x83\x01\x85a\x06\x13V[\x82\x81\x03` \x84\x01R\x83Q`\xA0\x82Ra\t\x86`\xA0\x83\x01\x82a\x06\x13V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01Ra\t\x9F\x82\x82a\x06\x13V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra\t\xB9\x82\x82a\x06\x13V[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01Ra\t\xD3\x82\x82a\x06\x13V[\x91PP`\x80\x85\x01Q`\x80\x83\x01R\x80\x92PPP\x93\x92PPPV[` \x81R`\0a\x03\x88` \x83\x01\x84a\x06\x13V\xFE\xA2dipfsX\"\x12 \xE6lm\x0EW;xg\xCC\xA0\xC7\xD95\xC5H(\xD6a\xCA\xB4\x15'\xDFp%U\xCC\xE3J\x83\xEC\ndsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static ATTESTATIONAUTHER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x01\xD5\x8F\xA3\x14a\0gW\x80c$\x10\xF6\xBA\x14a\0\xA3W\x80ck[!\xA6\x14a\0\xC3W\x80cu\x84{\x84\x14a\0\xE6W\x80c\x9A\xEC\x99\x0E\x14a\0\xF9W\x80c\xCDy\xF9\x06\x14a\x01 W[`\0\x80\xFD[a\0\x90a\0u6`\x04a\x05\xADV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xB16`\x04a\x05\xD6V[a\x01_V[`@Qa\0\x9A\x91\x90a\x06?V[a\0\xD6a\0\xD16`\x04a\x06\xA0V[a\x03XV[`@Q\x90\x15\x15\x81R` \x01a\0\x9AV[a\0\xD6a\0\xF46`\x04a\x07\x8EV[a\x03|V[a\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01G\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x9AV[a\x01\x83`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81R` \x81\x90R`@\x90\x81\x90 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x01\xAB\x90a\x08\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xD7\x90a\x08\x98V[\x80\x15a\x02$W\x80`\x1F\x10a\x01\xF9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02$V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x07W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x02=\x90a\x08\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02i\x90a\x08\x98V[\x80\x15a\x02\xB6W\x80`\x1F\x10a\x02\x8BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xB6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x99W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x02\xCF\x90a\x08\x98V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xFB\x90a\x08\x98V[\x80\x15a\x03HW\x80`\x1F\x10a\x03\x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03HV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03+W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T`\xFF\x16[\x92\x91PPV[`\0a\x03\x88\x83\x83a\x03\x8FV[\x93\x92PPPV[`\0\x80\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x03\xB3\x93\x92\x91\x90a\x08\xD2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x83\x90R\x91 \x80T\x91\x92P\x90a\x03\xE5\x90a\x08\x98V[\x90P`\0\x03a\x04\x07W`@Qc\x1CbV\x0B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x041\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba\t\x15V[a\x03\xE8\x84`\x80\x01Qa\x04C\x91\x90a\t6V[\x11a\x04aW`@Qc\x0C\xB0/\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xEA\xC7\x08\xA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEA\xC7\x08\xA3\x90a\x04\xAF\x90\x87\x90\x87\x90`\x04\x01a\tXV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xC7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x04\xDBW=`\0\x80>=`\0\xFD[PPPP`\0a\x04\xEE\x84`\0\x01Qa\x05}V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x90\x91P\x15a\x05\x1AW`\0\x92PPPa\x03vV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x01` R`@\x90\x81\x90 \x84\x90U\x85Q\x90Q\x84\x92\x91\x7F\xBBMd(\xD5>\xA9$\xD9K\xE0H\x84s\xDE\xB5\xC0\xA7\x0C\x97\x9F\x84w\xD2\x19\x11\xCB[\xEE@7\xFD\x91a\x05j\x91\x90a\t\xECV[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0\x81Q`@\x14a\x05\xA1W`@Qc\xBD\x9C\x80\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q` \x90\x91\x01 \x90V[`\0` \x82\x84\x03\x12\x15a\x05\xBFW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x88W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x05\xE8W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x06\nW\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xF2V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x06+\x81` \x86\x01` \x86\x01a\x05\xEFV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0\x82Q``` \x84\x01Ra\x06[`\x80\x84\x01\x82a\x06\x13V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra\x06y\x83\x83a\x06\x13V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa\x06\x97\x82\x82a\x06\x13V[\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xB3W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\xFBWa\x06\xFBa\x06\xC2V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\x12W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07-Wa\x07-a\x06\xC2V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x07UWa\x07Ua\x06\xC2V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x07nW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xA1W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xB9W`\0\x80\xFD[a\x07\xC5\x86\x83\x87\x01a\x07\x01V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x07\xDBW`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\x07\xEFW`\0\x80\xFD[a\x07\xF7a\x06\xD8V[\x825\x82\x81\x11\x15a\x08\x06W`\0\x80\xFD[a\x08\x12\x88\x82\x86\x01a\x07\x01V[\x82RP` \x83\x015\x82\x81\x11\x15a\x08'W`\0\x80\xFD[a\x083\x88\x82\x86\x01a\x07\x01V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x08KW`\0\x80\xFD[a\x08W\x88\x82\x86\x01a\x07\x01V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\x08oW`\0\x80\xFD[a\x08{\x88\x82\x86\x01a\x07\x01V[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x08\xACW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x08\xCCWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x84Qa\x08\xE4\x81\x84` \x89\x01a\x05\xEFV[\x84Q\x90\x83\x01\x90a\x08\xF8\x81\x83` \x89\x01a\x05\xEFV[\x84Q\x91\x01\x90a\t\x0B\x81\x83` \x88\x01a\x05\xEFV[\x01\x95\x94PPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x03vWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\tSWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`@\x81R`\0a\tk`@\x83\x01\x85a\x06\x13V[\x82\x81\x03` \x84\x01R\x83Q`\xA0\x82Ra\t\x86`\xA0\x83\x01\x82a\x06\x13V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01Ra\t\x9F\x82\x82a\x06\x13V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra\t\xB9\x82\x82a\x06\x13V[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01Ra\t\xD3\x82\x82a\x06\x13V[\x91PP`\x80\x85\x01Q`\x80\x83\x01R\x80\x92PPP\x93\x92PPPV[` \x81R`\0a\x03\x88` \x83\x01\x84a\x06\x13V\xFE\xA2dipfsX\"\x12 \xE6lm\x0EW;xg\xCC\xA0\xC7\xD95\xC5H(\xD6a\xCA\xB4\x15'\xDFp%U\xCC\xE3J\x83\xEC\ndsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static ATTESTATIONAUTHER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct AttestationAuther<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AttestationAuther<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AttestationAuther<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AttestationAuther<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AttestationAuther<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AttestationAuther))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AttestationAuther<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ATTESTATIONAUTHER_ABI.clone(),
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
                ATTESTATIONAUTHER_ABI.clone(),
                ATTESTATIONAUTHER_BYTECODE.clone().into(),
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AttestationAutherEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for AttestationAuther<M>
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
    pub enum AttestationAutherErrors {
        AttestationAutherAttestationTooOld(AttestationAutherAttestationTooOld),
        AttestationAutherImageNotInFamily(AttestationAutherImageNotInFamily),
        AttestationAutherImageNotWhitelisted(AttestationAutherImageNotWhitelisted),
        AttestationAutherKeyNotVerified(AttestationAutherKeyNotVerified),
        AttestationAutherMismatchedLengths(AttestationAutherMismatchedLengths),
        AttestationAutherPCRsInvalid(AttestationAutherPCRsInvalid),
        AttestationAutherPubkeyLengthInvalid(AttestationAutherPubkeyLengthInvalid),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationAutherErrors {
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AttestationAutherErrors {
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
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AttestationAutherErrors {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AttestationAutherErrors {
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
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AttestationAutherErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AttestationAutherAttestationTooOld> for AttestationAutherErrors {
        fn from(value: AttestationAutherAttestationTooOld) -> Self {
            Self::AttestationAutherAttestationTooOld(value)
        }
    }
    impl ::core::convert::From<AttestationAutherImageNotInFamily> for AttestationAutherErrors {
        fn from(value: AttestationAutherImageNotInFamily) -> Self {
            Self::AttestationAutherImageNotInFamily(value)
        }
    }
    impl ::core::convert::From<AttestationAutherImageNotWhitelisted> for AttestationAutherErrors {
        fn from(value: AttestationAutherImageNotWhitelisted) -> Self {
            Self::AttestationAutherImageNotWhitelisted(value)
        }
    }
    impl ::core::convert::From<AttestationAutherKeyNotVerified> for AttestationAutherErrors {
        fn from(value: AttestationAutherKeyNotVerified) -> Self {
            Self::AttestationAutherKeyNotVerified(value)
        }
    }
    impl ::core::convert::From<AttestationAutherMismatchedLengths> for AttestationAutherErrors {
        fn from(value: AttestationAutherMismatchedLengths) -> Self {
            Self::AttestationAutherMismatchedLengths(value)
        }
    }
    impl ::core::convert::From<AttestationAutherPCRsInvalid> for AttestationAutherErrors {
        fn from(value: AttestationAutherPCRsInvalid) -> Self {
            Self::AttestationAutherPCRsInvalid(value)
        }
    }
    impl ::core::convert::From<AttestationAutherPubkeyLengthInvalid> for AttestationAutherErrors {
        fn from(value: AttestationAutherPubkeyLengthInvalid) -> Self {
            Self::AttestationAutherPubkeyLengthInvalid(value)
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
    pub enum AttestationAutherEvents {
        EnclaveImageAddedToFamilyFilter(EnclaveImageAddedToFamilyFilter),
        EnclaveImageRemovedFromFamilyFilter(EnclaveImageRemovedFromFamilyFilter),
        EnclaveImageRevokedFilter(EnclaveImageRevokedFilter),
        EnclaveImageWhitelistedFilter(EnclaveImageWhitelistedFilter),
        EnclaveKeyRevokedFilter(EnclaveKeyRevokedFilter),
        EnclaveKeyVerifiedFilter(EnclaveKeyVerifiedFilter),
        EnclaveKeyWhitelistedFilter(EnclaveKeyWhitelistedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AttestationAutherEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EnclaveImageAddedToFamilyFilter::decode_log(log) {
                return Ok(AttestationAutherEvents::EnclaveImageAddedToFamilyFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveImageRemovedFromFamilyFilter::decode_log(log) {
                return Ok(AttestationAutherEvents::EnclaveImageRemovedFromFamilyFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageRevokedFilter::decode_log(log) {
                return Ok(AttestationAutherEvents::EnclaveImageRevokedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageWhitelistedFilter::decode_log(log) {
                return Ok(AttestationAutherEvents::EnclaveImageWhitelistedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnclaveKeyRevokedFilter::decode_log(log) {
                return Ok(AttestationAutherEvents::EnclaveKeyRevokedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyVerifiedFilter::decode_log(log) {
                return Ok(AttestationAutherEvents::EnclaveKeyVerifiedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyWhitelistedFilter::decode_log(log) {
                return Ok(AttestationAutherEvents::EnclaveKeyWhitelistedFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AttestationAutherEvents {
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
    impl ::core::convert::From<EnclaveImageAddedToFamilyFilter> for AttestationAutherEvents {
        fn from(value: EnclaveImageAddedToFamilyFilter) -> Self {
            Self::EnclaveImageAddedToFamilyFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRemovedFromFamilyFilter> for AttestationAutherEvents {
        fn from(value: EnclaveImageRemovedFromFamilyFilter) -> Self {
            Self::EnclaveImageRemovedFromFamilyFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRevokedFilter> for AttestationAutherEvents {
        fn from(value: EnclaveImageRevokedFilter) -> Self {
            Self::EnclaveImageRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageWhitelistedFilter> for AttestationAutherEvents {
        fn from(value: EnclaveImageWhitelistedFilter) -> Self {
            Self::EnclaveImageWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyRevokedFilter> for AttestationAutherEvents {
        fn from(value: EnclaveKeyRevokedFilter) -> Self {
            Self::EnclaveKeyRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyVerifiedFilter> for AttestationAutherEvents {
        fn from(value: EnclaveKeyVerifiedFilter) -> Self {
            Self::EnclaveKeyVerifiedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyWhitelistedFilter> for AttestationAutherEvents {
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
    pub enum AttestationAutherCalls {
        AttestationMaxAge(AttestationMaxAgeCall),
        AttestationVerifier(AttestationVerifierCall),
        GetVerifiedKey(GetVerifiedKeyCall),
        GetWhitelistedImage(GetWhitelistedImageCall),
        IsImageInFamily(IsImageInFamilyCall),
        VerifyEnclaveKey(VerifyEnclaveKeyCall),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationAutherCalls {
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
    impl ::ethers::core::abi::AbiEncode for AttestationAutherCalls {
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
    impl ::core::fmt::Display for AttestationAutherCalls {
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
    impl ::core::convert::From<AttestationMaxAgeCall> for AttestationAutherCalls {
        fn from(value: AttestationMaxAgeCall) -> Self {
            Self::AttestationMaxAge(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierCall> for AttestationAutherCalls {
        fn from(value: AttestationVerifierCall) -> Self {
            Self::AttestationVerifier(value)
        }
    }
    impl ::core::convert::From<GetVerifiedKeyCall> for AttestationAutherCalls {
        fn from(value: GetVerifiedKeyCall) -> Self {
            Self::GetVerifiedKey(value)
        }
    }
    impl ::core::convert::From<GetWhitelistedImageCall> for AttestationAutherCalls {
        fn from(value: GetWhitelistedImageCall) -> Self {
            Self::GetWhitelistedImage(value)
        }
    }
    impl ::core::convert::From<IsImageInFamilyCall> for AttestationAutherCalls {
        fn from(value: IsImageInFamilyCall) -> Self {
            Self::IsImageInFamily(value)
        }
    }
    impl ::core::convert::From<VerifyEnclaveKeyCall> for AttestationAutherCalls {
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
