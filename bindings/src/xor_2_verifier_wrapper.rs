pub use xor_2_verifier_wrapper::*;
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
pub mod xor_2_verifier_wrapper {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_iverifier"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract i_xor2_verifier"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_sampleInput"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_sampleProof"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("createRequest"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createRequest"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ask"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ProofMarketplace.Ask",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("secretType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum ProofMarketplace.SecretType",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("secret_inputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("acl"),
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
                    ::std::borrow::ToOwned::to_owned("encodeInputAndProofForVerification"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "encodeInputAndProofForVerification",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("inputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                    1usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[1]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("a"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                    2usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[2]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("b"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),
                                            ),
                                            2usize,
                                        ),
                                    ),
                                    2usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[2][2]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("c"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                    2usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[2]"),
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
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                                1usize,
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[1]"),
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
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("a"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                    2usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[2]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("b"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),
                                            ),
                                            2usize,
                                        ),
                                    ),
                                    2usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[2][2]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("c"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                    2usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[2]"),
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
                    ::std::borrow::ToOwned::to_owned("iverifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("iverifier"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract i_xor2_verifier"),
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
                                ::std::borrow::ToOwned::to_owned("contract ProofMarketplace",),
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
                    ::std::borrow::ToOwned::to_owned("setProofMarketplaceContract"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setProofMarketplaceContract",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_proofMarketplace"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ProofMarketplace",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                            name: ::std::borrow::ToOwned::to_owned("encodedProof"),
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
                    ::std::borrow::ToOwned::to_owned("verifyInputs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifyInputs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("inputs"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static XOR2_VERIFIER_WRAPPER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x15\xB88\x03\x80b\0\x15\xB8\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01-V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\x80R`\x01b\0\0O\x83\x82b\0\x02FV[P`\x02b\0\0^\x82\x82b\0\x02FV[PPPPb\0\x03\x12V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\0\x90W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\0\xADWb\0\0\xADb\0\0hV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\0\xD8Wb\0\0\xD8b\0\0hV[\x81`@R\x83\x81R` \x92P\x86\x83\x85\x88\x01\x01\x11\x15b\0\0\xF5W`\0\x80\xFD[`\0\x91P[\x83\x82\x10\x15b\0\x01\x19W\x85\x82\x01\x83\x01Q\x81\x83\x01\x84\x01R\x90\x82\x01\x90b\0\0\xFAV[`\0\x93\x81\x01\x90\x92\x01\x92\x90\x92R\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01CW`\0\x80\xFD[\x83Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01[W`\0\x80\xFD[` \x85\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01yW`\0\x80\xFD[b\0\x01\x87\x87\x83\x88\x01b\0\0~V[\x93P`@\x86\x01Q\x91P\x80\x82\x11\x15b\0\x01\x9EW`\0\x80\xFD[Pb\0\x01\xAD\x86\x82\x87\x01b\0\0~V[\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01\xCCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01\xEDWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02AW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02\x1CWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02=W\x82\x81U`\x01\x01b\0\x02(V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x02bWb\0\x02bb\0\0hV[b\0\x02z\x81b\0\x02s\x84Tb\0\x01\xB7V[\x84b\0\x01\xF3V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x02\xB2W`\0\x84\x15b\0\x02\x99WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02=V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02\xE3W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02\xC2V[P\x85\x82\x10\x15b\0\x03\x02W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Qa\x12\x83b\0\x035`\09`\0\x81\x81a\x01\xC8\x01Ra\x04\x92\x01Ra\x12\x83`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80c\x8Ev\n\xFE\x11a\0\x8CW\x80c\xA7l\x05Q\x11a\0fW\x80c\xA7l\x05Q\x14a\x01\xA8W\x80c\xD2#^\xAC\x14a\x01\xB0W\x80c\xE7\xF5\xB8\x1D\x14a\x01\xC3W\x80c\xEC#\xA9\xED\x14a\x01\xEAW`\0\x80\xFD[\x80c\x8Ev\n\xFE\x14a\x01oW\x80c\x9B\xBC\xADx\x14a\x01\x82W\x80c\xA6\xDF\xBC\x7F\x14a\x01\x95W`\0\x80\xFD[\x80bkO\xB0\x14a\0\xD3W\x80c\x02\xF7}\x19\x14a\0\xFCW\x80c\x05m\xE7\x04\x14a\x01\x1FW\x80c\x10\xA5By\x14a\x014W\x80c}\x8A\xD4+\x14a\x01<W\x80c\x81\xC4\\p\x14a\x01DW[`\0\x80\xFD[a\0\xE6a\0\xE16`\x04a\t7V[a\x01\xFDV[`@Qa\0\xF3\x91\x90a\t\xE0V[`@Q\x80\x91\x03\x90\xF3[a\x01\x0Fa\x01\n6`\x04a\n\x1BV[a\x02>V[`@Q\x90\x15\x15\x81R` \x01a\0\xF3V[a\x012a\x01-6`\x04a\n\xB3V[a\x02vV[\0[a\x01\x0Fa\x02\xE4V[a\0\xE6a\x03~V[`\0Ta\x01W\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[a\x01\x0Fa\x01}6`\x04a\n\x1BV[a\x04\x0CV[a\0\xE6a\x01\x906`\x04a\n\xD0V[a\x05\x1AV[a\x01\x0Fa\x01\xA36`\x04a\x0B5V[a\x05CV[a\0\xE6a\x05]V[a\x012a\x01\xBE6`\x04a\x0BwV[a\x05jV[a\x01W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xE6a\x01\xF86`\x04a\x0C,V[a\x06\x9CV[``a\x02\x08\x85a\x05\x1AV[a\x02\x13\x85\x85\x85a\x06\x9CV[`@Q` \x01a\x02$\x92\x91\x90a\x0CsV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[\x94\x93PPPPV[`\0\x80`\x01\x83`@Q` \x01a\x02U\x92\x91\x90a\x0C\xDBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x02o\x81a\x04\x0CV[\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10[\x1C\x99XY\x1EH\x14\xD9]`\xAA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x03y`\x02\x80Ta\x02\xF6\x90a\x0C\xA1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\"\x90a\x0C\xA1V[\x80\x15a\x03oW\x80`\x1F\x10a\x03DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03oV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03RW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x02>V[\x90P\x90V[`\x01\x80Ta\x03\x8B\x90a\x0C\xA1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xB7\x90a\x0C\xA1V[\x80\x15a\x04\x04W\x80`\x1F\x10a\x03\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\x04V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0a\x04\x16a\x07KV[a\x04\x1Ea\x07iV[a\x04&a\x07KV[a\x04.a\x07\x96V[`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x04E\x91\x90a\r\xD7V[\x91P\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x04]\x91\x90a\x0E;V[\x92P\x80\x80` \x01\x90Q\x81\x01\x90a\x04s\x91\x90a\x0E\xE0V[`@QcCu;M`\xE0\x1B\x81R\x92\x98P\x90\x96P\x94P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cCu;M\x90a\x04\xCD\x90\x89\x90\x89\x90\x89\x90\x89\x90`\x04\x01a\x0F\xE7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x0E\x91\x90a\x10\x1EV[\x98\x97PPPPPPPPV[``\x81`@Q` \x01a\x05-\x91\x90a\x10@V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a\x05Q\x82\x84\x01\x84a\x10NV[P`\x01\x90P[\x92\x91PPV[`\x02\x80Ta\x03\x8B\x90a\x0C\xA1V[`\0`@Q\x80`\xE0\x01`@R\x80\x88`\0\x015\x81R` \x01\x88` \x015\x81R` \x01\x88`@\x015\x81R` \x01\x88``\x015\x81R` \x01\x88`\x80\x015\x81R` \x01\x88`\xA0\x01` \x81\x01\x90a\x05\xBC\x91\x90a\n\xB3V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x05\xE2a\x01\x90a\x05\xDD`\xC0\x8C\x01\x8Ca\x10\xE8V[a\x06\xCBV[\x90R`\0T`@Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cpS\x8F\xCA\x90\x83\x90\x89\x90a\x06\x11\x90\x8A\x90\x8A\x90` \x01a\x11/V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x87\x87`@Q` \x01a\x063\x92\x91\x90a\x11/V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06a\x94\x93\x92\x91\x90a\x11\x80V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06{W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x8FW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[``\x83\x83\x83`@Q` \x01a\x06\xB3\x93\x92\x91\x90a\x12$V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x93\x92PPPV[a\x06\xD3a\x07\x96V[a\x06\xDD\x83\x83a\x05CV[a\x07?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FCircom Verifier Wrapper: Invalid`D\x82\x01Rl\x08\x1A[\x9C\x1D]\x08\x19\x9B\xDC\x9BX]`\x9A\x1B`d\x82\x01R`\x84\x01a\x02\xB9V[a\x02o\x82\x84\x01\x84a\n\xD0V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90[a\x07\x80a\x07KV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07xW\x90PP\x90V[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q` \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xEDWa\x07\xEDa\x07\xB4V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xEDWa\x07\xEDa\x07\xB4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08?Wa\x08?a\x07\xB4V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x08XW`\0\x80\xFD[a\x08`a\x07\xCAV[\x80` \x80\x85\x01\x86\x81\x11\x15a\x08sW`\0\x80\xFD[\x85[\x81\x81\x10\x15a\x08\x8CW\x805\x85R\x93\x82\x01\x93\x82\x01a\x08uV[P\x91\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x08\xA9W`\0\x80\xFD[a\x08\xB1a\x07\xF3V[\x80`@\x84\x01\x85\x81\x11\x15a\x08\xC3W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x08\xDDW\x805\x84R` \x93\x84\x01\x93\x01a\x08\xC5V[P\x90\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a\x08\xF9W`\0\x80\xFD[a\t\x01a\x07\xF3V[\x80`\x80\x84\x01\x85\x81\x11\x15a\t\x13W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x08\xDDWa\t'\x87\x82a\x08\x98V[\x84R` \x90\x93\x01\x92`@\x01a\t\x15V[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a\tNW`\0\x80\xFD[a\tX\x86\x86a\x08GV[\x93Pa\tg\x86` \x87\x01a\x08\x98V[\x92Pa\tv\x86``\x87\x01a\x08\xE8V[\x91Pa\t\x85\x86`\xE0\x87\x01a\x08\x98V[\x90P\x92\x95\x91\x94P\x92PV[`\0[\x83\x81\x10\x15a\t\xABW\x81\x81\x01Q\x83\x82\x01R` \x01a\t\x93V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\t\xCC\x81` \x86\x01` \x86\x01a\t\x90V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x02o` \x83\x01\x84a\t\xB4V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\rWa\n\ra\x07\xB4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a\n-W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nDW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\nUW`\0\x80\xFD[\x805a\nha\nc\x82a\t\xF3V[a\x08\x16V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\n}W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xB0W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\n\xC5W`\0\x80\xFD[\x815a\x02o\x81a\n\x9BV[`\0` \x82\x84\x03\x12\x15a\n\xE2W`\0\x80\xFD[a\x02o\x83\x83a\x08GV[`\0\x80\x83`\x1F\x84\x01\x12a\n\xFEW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x16W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0B.W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x0BHW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B_W`\0\x80\xFD[a\x0Bk\x85\x82\x86\x01a\n\xECV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x0B\x90W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xA8W`\0\x80\xFD[\x90\x88\x01\x90`\xE0\x82\x8B\x03\x12\x15a\x0B\xBCW`\0\x80\xFD[\x90\x96P` \x88\x015\x90`\x03\x82\x10a\x0B\xD2W`\0\x80\xFD[\x90\x95P`@\x88\x015\x90\x80\x82\x11\x15a\x0B\xE8W`\0\x80\xFD[a\x0B\xF4\x8A\x83\x8B\x01a\n\xECV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a\x0C\rW`\0\x80\xFD[Pa\x0C\x1A\x89\x82\x8A\x01a\n\xECV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a\x0CBW`\0\x80\xFD[a\x0CL\x85\x85a\x08\x98V[\x92Pa\x0C[\x85`@\x86\x01a\x08\xE8V[\x91Pa\x0Cj\x85`\xC0\x86\x01a\x08\x98V[\x90P\x92P\x92P\x92V[`@\x81R`\0a\x0C\x86`@\x83\x01\x85a\t\xB4V[\x82\x81\x03` \x84\x01Ra\x0C\x98\x81\x85a\t\xB4V[\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\xB5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C\xD5WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0\x80\x84T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a\x0C\xFBW`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\r\x1AWcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[`@\x88\x01\x84\x90R``\x88\x01\x82\x80\x15a\r9W`\x01\x81\x14a\rOWa\rzV[`\xFF\x19\x87\x16\x82R\x85\x15\x15`\x05\x1B\x82\x01\x97Pa\rzV[`\0\x8C\x81R` \x90 `\0[\x87\x81\x10\x15a\rtW\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a\r[V[\x83\x01\x98PP[PP\x87\x86\x03\x81\x89\x01RPPPPPa\x0C\x98\x81\x85a\t\xB4V[`\0\x82`\x1F\x83\x01\x12a\r\xA3W`\0\x80\xFD[\x81Qa\r\xB1a\nc\x82a\t\xF3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\r\xC6W`\0\x80\xFD[a\x026\x82` \x83\x01` \x87\x01a\t\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\r\xEAW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x02W`\0\x80\xFD[a\x0E\x0E\x86\x83\x87\x01a\r\x92V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\x0E$W`\0\x80\xFD[Pa\x0E1\x85\x82\x86\x01a\r\x92V[\x91PP\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x0ENW`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x0E]W`\0\x80\xFD[a\x0Eea\x07\xCAV[\x80\x82\x85\x01\x86\x81\x11\x15a\x0EvW`\0\x80\xFD[\x85[\x81\x81\x10\x15a\x0E\x8FW\x80Q\x84R\x92\x84\x01\x92\x84\x01a\x0ExV[P\x90\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x0E\xACW`\0\x80\xFD[a\x0E\xB4a\x07\xF3V[\x80`@\x84\x01\x85\x81\x11\x15a\x0E\xC6W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x08\xDDW\x80Q\x84R` \x93\x84\x01\x93\x01a\x0E\xC8V[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a\x0E\xF6W`\0\x80\xFD[a\x0F\0\x85\x85a\x0E\x9BV[\x92P`@\x85`_\x86\x01\x12a\x0F\x13W`\0\x80\xFD[a\x0F\x1Ba\x07\xF3V[\x80`\xC0\x87\x01\x88\x81\x11\x15a\x0F-W`\0\x80\xFD[\x83\x88\x01[\x81\x81\x10\x15a\x0FRWa\x0FC\x8A\x82a\x0E\x9BV[\x84R` \x90\x93\x01\x92\x84\x01a\x0F1V[P\x81\x95Pa\x0F`\x89\x82a\x0E\x9BV[\x94PPPPP\x92P\x92P\x92V[\x80`\0[`\x02\x81\x10\x15a\x0F\x90W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x0FqV[PPPPV[\x80`\0[`\x02\x81\x10\x15a\x0F\x90Wa\x0F\xAE\x84\x83Qa\x0FmV[`@\x93\x90\x93\x01\x92` \x91\x90\x91\x01\x90`\x01\x01a\x0F\x9AV[\x80`\0[`\x01\x81\x10\x15a\x0F\x90W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x0F\xC8V[a\x01 \x81\x01a\x0F\xF6\x82\x87a\x0FmV[a\x10\x03`@\x83\x01\x86a\x0F\x96V[a\x10\x10`\xC0\x83\x01\x85a\x0FmV[a\x0C\x98a\x01\0\x83\x01\x84a\x0F\xC4V[`\0` \x82\x84\x03\x12\x15a\x100W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02oW`\0\x80\xFD[` \x81\x01a\x05W\x82\x84a\x0F\xC4V[`\0` \x80\x83\x85\x03\x12\x15a\x10aW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10yW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x10\x8DW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10\x9FWa\x10\x9Fa\x07\xB4V[\x80`\x05\x1B\x91Pa\x10\xB0\x84\x83\x01a\x08\x16V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\x10\xCAW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x05\x0EW\x845\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x10\xCFV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x10\xFFW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\x1AW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0B.W`\0\x80\xFD[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\x03\x81\x10a\x11|WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`\x80\x81R\x84Q`\x80\x82\x01R` \x85\x01Q`\xA0\x82\x01R`@\x85\x01Q`\xC0\x82\x01R``\x85\x01Q`\xE0\x82\x01R`\x80\x85\x01Qa\x01\0\x82\x01R`\x01\x80`\xA0\x1B\x03`\xA0\x86\x01Q\x16a\x01 \x82\x01R`\0`\xC0\x86\x01Q`\xE0a\x01@\x84\x01Ra\x11\xE4a\x01`\x84\x01\x82a\t\xB4V[\x90Pa\x11\xF3` \x84\x01\x87a\x11^V[\x82\x81\x03`@\x84\x01Ra\x12\x05\x81\x86a\t\xB4V[\x90P\x82\x81\x03``\x84\x01Ra\x12\x19\x81\x85a\t\xB4V[\x97\x96PPPPPPPV[a\x01\0\x81\x01a\x123\x82\x86a\x0FmV[a\x12@`@\x83\x01\x85a\x0F\x96V[a\x026`\xC0\x83\x01\x84a\x0FmV\xFE\xA2dipfsX\"\x12 K\xB4\xB1\xDCw3\x14cO\x10\x83\x1D(\xFF\x8C\x1C\x96\xC3\x1B\xF2\x87\x87\xEF\xCD\xB8\xE1\x026\x14\xE2\xED:dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static XOR2_VERIFIER_WRAPPER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80c\x8Ev\n\xFE\x11a\0\x8CW\x80c\xA7l\x05Q\x11a\0fW\x80c\xA7l\x05Q\x14a\x01\xA8W\x80c\xD2#^\xAC\x14a\x01\xB0W\x80c\xE7\xF5\xB8\x1D\x14a\x01\xC3W\x80c\xEC#\xA9\xED\x14a\x01\xEAW`\0\x80\xFD[\x80c\x8Ev\n\xFE\x14a\x01oW\x80c\x9B\xBC\xADx\x14a\x01\x82W\x80c\xA6\xDF\xBC\x7F\x14a\x01\x95W`\0\x80\xFD[\x80bkO\xB0\x14a\0\xD3W\x80c\x02\xF7}\x19\x14a\0\xFCW\x80c\x05m\xE7\x04\x14a\x01\x1FW\x80c\x10\xA5By\x14a\x014W\x80c}\x8A\xD4+\x14a\x01<W\x80c\x81\xC4\\p\x14a\x01DW[`\0\x80\xFD[a\0\xE6a\0\xE16`\x04a\t7V[a\x01\xFDV[`@Qa\0\xF3\x91\x90a\t\xE0V[`@Q\x80\x91\x03\x90\xF3[a\x01\x0Fa\x01\n6`\x04a\n\x1BV[a\x02>V[`@Q\x90\x15\x15\x81R` \x01a\0\xF3V[a\x012a\x01-6`\x04a\n\xB3V[a\x02vV[\0[a\x01\x0Fa\x02\xE4V[a\0\xE6a\x03~V[`\0Ta\x01W\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[a\x01\x0Fa\x01}6`\x04a\n\x1BV[a\x04\x0CV[a\0\xE6a\x01\x906`\x04a\n\xD0V[a\x05\x1AV[a\x01\x0Fa\x01\xA36`\x04a\x0B5V[a\x05CV[a\0\xE6a\x05]V[a\x012a\x01\xBE6`\x04a\x0BwV[a\x05jV[a\x01W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xE6a\x01\xF86`\x04a\x0C,V[a\x06\x9CV[``a\x02\x08\x85a\x05\x1AV[a\x02\x13\x85\x85\x85a\x06\x9CV[`@Q` \x01a\x02$\x92\x91\x90a\x0CsV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[\x94\x93PPPPV[`\0\x80`\x01\x83`@Q` \x01a\x02U\x92\x91\x90a\x0C\xDBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x02o\x81a\x04\x0CV[\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10[\x1C\x99XY\x1EH\x14\xD9]`\xAA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x03y`\x02\x80Ta\x02\xF6\x90a\x0C\xA1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\"\x90a\x0C\xA1V[\x80\x15a\x03oW\x80`\x1F\x10a\x03DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03oV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03RW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x02>V[\x90P\x90V[`\x01\x80Ta\x03\x8B\x90a\x0C\xA1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xB7\x90a\x0C\xA1V[\x80\x15a\x04\x04W\x80`\x1F\x10a\x03\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\x04V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0a\x04\x16a\x07KV[a\x04\x1Ea\x07iV[a\x04&a\x07KV[a\x04.a\x07\x96V[`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x04E\x91\x90a\r\xD7V[\x91P\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x04]\x91\x90a\x0E;V[\x92P\x80\x80` \x01\x90Q\x81\x01\x90a\x04s\x91\x90a\x0E\xE0V[`@QcCu;M`\xE0\x1B\x81R\x92\x98P\x90\x96P\x94P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cCu;M\x90a\x04\xCD\x90\x89\x90\x89\x90\x89\x90\x89\x90`\x04\x01a\x0F\xE7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x0E\x91\x90a\x10\x1EV[\x98\x97PPPPPPPPV[``\x81`@Q` \x01a\x05-\x91\x90a\x10@V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a\x05Q\x82\x84\x01\x84a\x10NV[P`\x01\x90P[\x92\x91PPV[`\x02\x80Ta\x03\x8B\x90a\x0C\xA1V[`\0`@Q\x80`\xE0\x01`@R\x80\x88`\0\x015\x81R` \x01\x88` \x015\x81R` \x01\x88`@\x015\x81R` \x01\x88``\x015\x81R` \x01\x88`\x80\x015\x81R` \x01\x88`\xA0\x01` \x81\x01\x90a\x05\xBC\x91\x90a\n\xB3V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x05\xE2a\x01\x90a\x05\xDD`\xC0\x8C\x01\x8Ca\x10\xE8V[a\x06\xCBV[\x90R`\0T`@Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cpS\x8F\xCA\x90\x83\x90\x89\x90a\x06\x11\x90\x8A\x90\x8A\x90` \x01a\x11/V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x87\x87`@Q` \x01a\x063\x92\x91\x90a\x11/V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06a\x94\x93\x92\x91\x90a\x11\x80V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06{W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x8FW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[``\x83\x83\x83`@Q` \x01a\x06\xB3\x93\x92\x91\x90a\x12$V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x93\x92PPPV[a\x06\xD3a\x07\x96V[a\x06\xDD\x83\x83a\x05CV[a\x07?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FCircom Verifier Wrapper: Invalid`D\x82\x01Rl\x08\x1A[\x9C\x1D]\x08\x19\x9B\xDC\x9BX]`\x9A\x1B`d\x82\x01R`\x84\x01a\x02\xB9V[a\x02o\x82\x84\x01\x84a\n\xD0V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90[a\x07\x80a\x07KV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07xW\x90PP\x90V[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q` \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xEDWa\x07\xEDa\x07\xB4V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xEDWa\x07\xEDa\x07\xB4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08?Wa\x08?a\x07\xB4V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x08XW`\0\x80\xFD[a\x08`a\x07\xCAV[\x80` \x80\x85\x01\x86\x81\x11\x15a\x08sW`\0\x80\xFD[\x85[\x81\x81\x10\x15a\x08\x8CW\x805\x85R\x93\x82\x01\x93\x82\x01a\x08uV[P\x91\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x08\xA9W`\0\x80\xFD[a\x08\xB1a\x07\xF3V[\x80`@\x84\x01\x85\x81\x11\x15a\x08\xC3W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x08\xDDW\x805\x84R` \x93\x84\x01\x93\x01a\x08\xC5V[P\x90\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a\x08\xF9W`\0\x80\xFD[a\t\x01a\x07\xF3V[\x80`\x80\x84\x01\x85\x81\x11\x15a\t\x13W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x08\xDDWa\t'\x87\x82a\x08\x98V[\x84R` \x90\x93\x01\x92`@\x01a\t\x15V[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a\tNW`\0\x80\xFD[a\tX\x86\x86a\x08GV[\x93Pa\tg\x86` \x87\x01a\x08\x98V[\x92Pa\tv\x86``\x87\x01a\x08\xE8V[\x91Pa\t\x85\x86`\xE0\x87\x01a\x08\x98V[\x90P\x92\x95\x91\x94P\x92PV[`\0[\x83\x81\x10\x15a\t\xABW\x81\x81\x01Q\x83\x82\x01R` \x01a\t\x93V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\t\xCC\x81` \x86\x01` \x86\x01a\t\x90V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x02o` \x83\x01\x84a\t\xB4V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\rWa\n\ra\x07\xB4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a\n-W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nDW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\nUW`\0\x80\xFD[\x805a\nha\nc\x82a\t\xF3V[a\x08\x16V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\n}W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xB0W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\n\xC5W`\0\x80\xFD[\x815a\x02o\x81a\n\x9BV[`\0` \x82\x84\x03\x12\x15a\n\xE2W`\0\x80\xFD[a\x02o\x83\x83a\x08GV[`\0\x80\x83`\x1F\x84\x01\x12a\n\xFEW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x16W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0B.W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x0BHW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B_W`\0\x80\xFD[a\x0Bk\x85\x82\x86\x01a\n\xECV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x0B\x90W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xA8W`\0\x80\xFD[\x90\x88\x01\x90`\xE0\x82\x8B\x03\x12\x15a\x0B\xBCW`\0\x80\xFD[\x90\x96P` \x88\x015\x90`\x03\x82\x10a\x0B\xD2W`\0\x80\xFD[\x90\x95P`@\x88\x015\x90\x80\x82\x11\x15a\x0B\xE8W`\0\x80\xFD[a\x0B\xF4\x8A\x83\x8B\x01a\n\xECV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a\x0C\rW`\0\x80\xFD[Pa\x0C\x1A\x89\x82\x8A\x01a\n\xECV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a\x0CBW`\0\x80\xFD[a\x0CL\x85\x85a\x08\x98V[\x92Pa\x0C[\x85`@\x86\x01a\x08\xE8V[\x91Pa\x0Cj\x85`\xC0\x86\x01a\x08\x98V[\x90P\x92P\x92P\x92V[`@\x81R`\0a\x0C\x86`@\x83\x01\x85a\t\xB4V[\x82\x81\x03` \x84\x01Ra\x0C\x98\x81\x85a\t\xB4V[\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\xB5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C\xD5WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0\x80\x84T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a\x0C\xFBW`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\r\x1AWcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[`@\x88\x01\x84\x90R``\x88\x01\x82\x80\x15a\r9W`\x01\x81\x14a\rOWa\rzV[`\xFF\x19\x87\x16\x82R\x85\x15\x15`\x05\x1B\x82\x01\x97Pa\rzV[`\0\x8C\x81R` \x90 `\0[\x87\x81\x10\x15a\rtW\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a\r[V[\x83\x01\x98PP[PP\x87\x86\x03\x81\x89\x01RPPPPPa\x0C\x98\x81\x85a\t\xB4V[`\0\x82`\x1F\x83\x01\x12a\r\xA3W`\0\x80\xFD[\x81Qa\r\xB1a\nc\x82a\t\xF3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\r\xC6W`\0\x80\xFD[a\x026\x82` \x83\x01` \x87\x01a\t\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\r\xEAW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x02W`\0\x80\xFD[a\x0E\x0E\x86\x83\x87\x01a\r\x92V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\x0E$W`\0\x80\xFD[Pa\x0E1\x85\x82\x86\x01a\r\x92V[\x91PP\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x0ENW`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x0E]W`\0\x80\xFD[a\x0Eea\x07\xCAV[\x80\x82\x85\x01\x86\x81\x11\x15a\x0EvW`\0\x80\xFD[\x85[\x81\x81\x10\x15a\x0E\x8FW\x80Q\x84R\x92\x84\x01\x92\x84\x01a\x0ExV[P\x90\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x0E\xACW`\0\x80\xFD[a\x0E\xB4a\x07\xF3V[\x80`@\x84\x01\x85\x81\x11\x15a\x0E\xC6W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x08\xDDW\x80Q\x84R` \x93\x84\x01\x93\x01a\x0E\xC8V[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a\x0E\xF6W`\0\x80\xFD[a\x0F\0\x85\x85a\x0E\x9BV[\x92P`@\x85`_\x86\x01\x12a\x0F\x13W`\0\x80\xFD[a\x0F\x1Ba\x07\xF3V[\x80`\xC0\x87\x01\x88\x81\x11\x15a\x0F-W`\0\x80\xFD[\x83\x88\x01[\x81\x81\x10\x15a\x0FRWa\x0FC\x8A\x82a\x0E\x9BV[\x84R` \x90\x93\x01\x92\x84\x01a\x0F1V[P\x81\x95Pa\x0F`\x89\x82a\x0E\x9BV[\x94PPPPP\x92P\x92P\x92V[\x80`\0[`\x02\x81\x10\x15a\x0F\x90W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x0FqV[PPPPV[\x80`\0[`\x02\x81\x10\x15a\x0F\x90Wa\x0F\xAE\x84\x83Qa\x0FmV[`@\x93\x90\x93\x01\x92` \x91\x90\x91\x01\x90`\x01\x01a\x0F\x9AV[\x80`\0[`\x01\x81\x10\x15a\x0F\x90W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x0F\xC8V[a\x01 \x81\x01a\x0F\xF6\x82\x87a\x0FmV[a\x10\x03`@\x83\x01\x86a\x0F\x96V[a\x10\x10`\xC0\x83\x01\x85a\x0FmV[a\x0C\x98a\x01\0\x83\x01\x84a\x0F\xC4V[`\0` \x82\x84\x03\x12\x15a\x100W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02oW`\0\x80\xFD[` \x81\x01a\x05W\x82\x84a\x0F\xC4V[`\0` \x80\x83\x85\x03\x12\x15a\x10aW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10yW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x10\x8DW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10\x9FWa\x10\x9Fa\x07\xB4V[\x80`\x05\x1B\x91Pa\x10\xB0\x84\x83\x01a\x08\x16V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\x10\xCAW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x05\x0EW\x845\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x10\xCFV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x10\xFFW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\x1AW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0B.W`\0\x80\xFD[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\x03\x81\x10a\x11|WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`\x80\x81R\x84Q`\x80\x82\x01R` \x85\x01Q`\xA0\x82\x01R`@\x85\x01Q`\xC0\x82\x01R``\x85\x01Q`\xE0\x82\x01R`\x80\x85\x01Qa\x01\0\x82\x01R`\x01\x80`\xA0\x1B\x03`\xA0\x86\x01Q\x16a\x01 \x82\x01R`\0`\xC0\x86\x01Q`\xE0a\x01@\x84\x01Ra\x11\xE4a\x01`\x84\x01\x82a\t\xB4V[\x90Pa\x11\xF3` \x84\x01\x87a\x11^V[\x82\x81\x03`@\x84\x01Ra\x12\x05\x81\x86a\t\xB4V[\x90P\x82\x81\x03``\x84\x01Ra\x12\x19\x81\x85a\t\xB4V[\x97\x96PPPPPPPV[a\x01\0\x81\x01a\x123\x82\x86a\x0FmV[a\x12@`@\x83\x01\x85a\x0F\x96V[a\x026`\xC0\x83\x01\x84a\x0FmV\xFE\xA2dipfsX\"\x12 K\xB4\xB1\xDCw3\x14cO\x10\x83\x1D(\xFF\x8C\x1C\x96\xC3\x1B\xF2\x87\x87\xEF\xCD\xB8\xE1\x026\x14\xE2\xED:dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static XOR2_VERIFIER_WRAPPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct xor2_verifier_wrapper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for xor2_verifier_wrapper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for xor2_verifier_wrapper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for xor2_verifier_wrapper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for xor2_verifier_wrapper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(xor2_verifier_wrapper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> xor2_verifier_wrapper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                XOR2_VERIFIER_WRAPPER_ABI.clone(),
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
                XOR2_VERIFIER_WRAPPER_ABI.clone(),
                XOR2_VERIFIER_WRAPPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkSampleInputsAndProof` (0x10a54279) function
        pub fn check_sample_inputs_and_proof(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([16, 165, 66, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createRequest` (0xd2235eac) function
        pub fn create_request(
            &self,
            ask: Ask,
            secret_type: u8,
            secret_inputs: ::ethers::core::types::Bytes,
            acl: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 35, 94, 172], (ask, secret_type, secret_inputs, acl))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeInputAndProofForVerification` (0x006b4fb0) function
        pub fn encode_input_and_proof_for_verification(
            &self,
            inputs: [::ethers::core::types::U256; 1],
            a: [::ethers::core::types::U256; 2],
            b: [[::ethers::core::types::U256; 2]; 2],
            c: [::ethers::core::types::U256; 2],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([0, 107, 79, 176], (inputs, a, b, c))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeInputs` (0x9bbcad78) function
        pub fn encode_inputs(
            &self,
            inputs: [::ethers::core::types::U256; 1],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([155, 188, 173, 120], inputs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeProof` (0xec23a9ed) function
        pub fn encode_proof(
            &self,
            a: [::ethers::core::types::U256; 2],
            b: [[::ethers::core::types::U256; 2]; 2],
            c: [::ethers::core::types::U256; 2],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([236, 35, 169, 237], (a, b, c))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `iverifier` (0xe7f5b81d) function
        pub fn iverifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([231, 245, 184, 29], ())
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
        ///Calls the contract's `setProofMarketplaceContract` (0x056de704) function
        pub fn set_proof_marketplace_contract(
            &self,
            proof_marketplace: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 109, 231, 4], proof_marketplace)
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
            encoded_proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([2, 247, 125, 25], encoded_proof)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyInputs` (0xa6dfbc7f) function
        pub fn verify_inputs(
            &self,
            inputs: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 223, 188, 127], inputs)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for xor2_verifier_wrapper<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all input parameters for the `createRequest` function with signature `createRequest((uint256,uint256,uint256,uint256,uint256,address,bytes),uint8,bytes,bytes)` and selector `0xd2235eac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "createRequest",
        abi = "createRequest((uint256,uint256,uint256,uint256,uint256,address,bytes),uint8,bytes,bytes)"
    )]
    pub struct CreateRequestCall {
        pub ask: Ask,
        pub secret_type: u8,
        pub secret_inputs: ::ethers::core::types::Bytes,
        pub acl: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `encodeInputAndProofForVerification` function with signature `encodeInputAndProofForVerification(uint256[1],uint256[2],uint256[2][2],uint256[2])` and selector `0x006b4fb0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        abi = "encodeInputAndProofForVerification(uint256[1],uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct EncodeInputAndProofForVerificationCall {
        pub inputs: [::ethers::core::types::U256; 1],
        pub a: [::ethers::core::types::U256; 2],
        pub b: [[::ethers::core::types::U256; 2]; 2],
        pub c: [::ethers::core::types::U256; 2],
    }
    ///Container type for all input parameters for the `encodeInputs` function with signature `encodeInputs(uint256[1])` and selector `0x9bbcad78`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "encodeInputs", abi = "encodeInputs(uint256[1])")]
    pub struct EncodeInputsCall {
        pub inputs: [::ethers::core::types::U256; 1],
    }
    ///Container type for all input parameters for the `encodeProof` function with signature `encodeProof(uint256[2],uint256[2][2],uint256[2])` and selector `0xec23a9ed`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "encodeProof",
        abi = "encodeProof(uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct EncodeProofCall {
        pub a: [::ethers::core::types::U256; 2],
        pub b: [[::ethers::core::types::U256; 2]; 2],
        pub c: [::ethers::core::types::U256; 2],
    }
    ///Container type for all input parameters for the `iverifier` function with signature `iverifier()` and selector `0xe7f5b81d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "iverifier", abi = "iverifier()")]
    pub struct IverifierCall;
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
    ///Container type for all input parameters for the `setProofMarketplaceContract` function with signature `setProofMarketplaceContract(address)` and selector `0x056de704`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "setProofMarketplaceContract",
        abi = "setProofMarketplaceContract(address)"
    )]
    pub struct SetProofMarketplaceContractCall {
        pub proof_marketplace: ::ethers::core::types::Address,
    }
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
    pub struct VerifyAgainstSampleInputsCall {
        pub encoded_proof: ::ethers::core::types::Bytes,
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
    pub struct VerifyInputsCall {
        pub inputs: ::ethers::core::types::Bytes,
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
    pub enum xor2_verifier_wrapperCalls {
        CheckSampleInputsAndProof(CheckSampleInputsAndProofCall),
        CreateRequest(CreateRequestCall),
        EncodeInputAndProofForVerification(EncodeInputAndProofForVerificationCall),
        EncodeInputs(EncodeInputsCall),
        EncodeProof(EncodeProofCall),
        Iverifier(IverifierCall),
        ProofMarketplace(ProofMarketplaceCall),
        SampleInput(SampleInputCall),
        SampleProof(SampleProofCall),
        SetProofMarketplaceContract(SetProofMarketplaceContractCall),
        Verify(VerifyCall),
        VerifyAgainstSampleInputs(VerifyAgainstSampleInputsCall),
        VerifyInputs(VerifyInputsCall),
    }
    impl ::ethers::core::abi::AbiDecode for xor2_verifier_wrapperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CheckSampleInputsAndProofCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSampleInputsAndProof(decoded));
            }
            if let Ok(decoded) = <CreateRequestCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateRequest(decoded));
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
            if let Ok(decoded) = <IverifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Iverifier(decoded));
            }
            if let Ok(decoded) =
                <ProofMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProofMarketplace(decoded));
            }
            if let Ok(decoded) = <SampleInputCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SampleInput(decoded));
            }
            if let Ok(decoded) = <SampleProofCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SampleProof(decoded));
            }
            if let Ok(decoded) =
                <SetProofMarketplaceContractCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProofMarketplaceContract(decoded));
            }
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verify(decoded));
            }
            if let Ok(decoded) =
                <VerifyAgainstSampleInputsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyAgainstSampleInputs(decoded));
            }
            if let Ok(decoded) = <VerifyInputsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyInputs(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for xor2_verifier_wrapperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckSampleInputsAndProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateRequest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EncodeInputAndProofForVerification(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeInputs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EncodeProof(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Iverifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProofMarketplace(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SampleInput(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SampleProof(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProofMarketplaceContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyAgainstSampleInputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyInputs(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for xor2_verifier_wrapperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckSampleInputsAndProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeInputAndProofForVerification(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::Iverifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::SampleInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::SampleProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProofMarketplaceContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyAgainstSampleInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyInputs(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckSampleInputsAndProofCall> for xor2_verifier_wrapperCalls {
        fn from(value: CheckSampleInputsAndProofCall) -> Self {
            Self::CheckSampleInputsAndProof(value)
        }
    }
    impl ::core::convert::From<CreateRequestCall> for xor2_verifier_wrapperCalls {
        fn from(value: CreateRequestCall) -> Self {
            Self::CreateRequest(value)
        }
    }
    impl ::core::convert::From<EncodeInputAndProofForVerificationCall> for xor2_verifier_wrapperCalls {
        fn from(value: EncodeInputAndProofForVerificationCall) -> Self {
            Self::EncodeInputAndProofForVerification(value)
        }
    }
    impl ::core::convert::From<EncodeInputsCall> for xor2_verifier_wrapperCalls {
        fn from(value: EncodeInputsCall) -> Self {
            Self::EncodeInputs(value)
        }
    }
    impl ::core::convert::From<EncodeProofCall> for xor2_verifier_wrapperCalls {
        fn from(value: EncodeProofCall) -> Self {
            Self::EncodeProof(value)
        }
    }
    impl ::core::convert::From<IverifierCall> for xor2_verifier_wrapperCalls {
        fn from(value: IverifierCall) -> Self {
            Self::Iverifier(value)
        }
    }
    impl ::core::convert::From<ProofMarketplaceCall> for xor2_verifier_wrapperCalls {
        fn from(value: ProofMarketplaceCall) -> Self {
            Self::ProofMarketplace(value)
        }
    }
    impl ::core::convert::From<SampleInputCall> for xor2_verifier_wrapperCalls {
        fn from(value: SampleInputCall) -> Self {
            Self::SampleInput(value)
        }
    }
    impl ::core::convert::From<SampleProofCall> for xor2_verifier_wrapperCalls {
        fn from(value: SampleProofCall) -> Self {
            Self::SampleProof(value)
        }
    }
    impl ::core::convert::From<SetProofMarketplaceContractCall> for xor2_verifier_wrapperCalls {
        fn from(value: SetProofMarketplaceContractCall) -> Self {
            Self::SetProofMarketplaceContract(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for xor2_verifier_wrapperCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifyAgainstSampleInputsCall> for xor2_verifier_wrapperCalls {
        fn from(value: VerifyAgainstSampleInputsCall) -> Self {
            Self::VerifyAgainstSampleInputs(value)
        }
    }
    impl ::core::convert::From<VerifyInputsCall> for xor2_verifier_wrapperCalls {
        fn from(value: VerifyInputsCall) -> Self {
            Self::VerifyInputs(value)
        }
    }
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
    ///Container type for all return fields from the `encodeInputAndProofForVerification` function with signature `encodeInputAndProofForVerification(uint256[1],uint256[2],uint256[2][2],uint256[2])` and selector `0x006b4fb0`
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
    ///Container type for all return fields from the `encodeInputs` function with signature `encodeInputs(uint256[1])` and selector `0x9bbcad78`
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
    ///Container type for all return fields from the `encodeProof` function with signature `encodeProof(uint256[2],uint256[2][2],uint256[2])` and selector `0xec23a9ed`
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
    ///Container type for all return fields from the `iverifier` function with signature `iverifier()` and selector `0xe7f5b81d`
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
    pub struct IverifierReturn(pub ::ethers::core::types::Address);
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
}
