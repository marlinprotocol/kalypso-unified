pub use plonk_verifier_wrapper::*;
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
pub mod plonk_verifier_wrapper {
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
                            ::std::borrow::ToOwned::to_owned("contract i_plonk_vk"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proof"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
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
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32[]"),
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
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("iverifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("iverifier"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract i_plonk_vk"),
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
    pub static PLONK_VERIFIER_WRAPPER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x12b8\x03\x80b\0\x12b\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01-V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\x80R`\x01b\0\0O\x83\x82b\0\x02FV[P`\x02b\0\0^\x82\x82b\0\x02FV[PPPPb\0\x03\x12V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\0\x90W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\0\xADWb\0\0\xADb\0\0hV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\0\xD8Wb\0\0\xD8b\0\0hV[\x81`@R\x83\x81R` \x92P\x86\x83\x85\x88\x01\x01\x11\x15b\0\0\xF5W`\0\x80\xFD[`\0\x91P[\x83\x82\x10\x15b\0\x01\x19W\x85\x82\x01\x83\x01Q\x81\x83\x01\x84\x01R\x90\x82\x01\x90b\0\0\xFAV[`\0\x93\x81\x01\x90\x92\x01\x92\x90\x92R\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01CW`\0\x80\xFD[\x83Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01[W`\0\x80\xFD[` \x85\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01yW`\0\x80\xFD[b\0\x01\x87\x87\x83\x88\x01b\0\0~V[\x93P`@\x86\x01Q\x91P\x80\x82\x11\x15b\0\x01\x9EW`\0\x80\xFD[Pb\0\x01\xAD\x86\x82\x87\x01b\0\0~V[\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01\xCCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01\xEDWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02AW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02\x1CWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02=W\x82\x81U`\x01\x01b\0\x02(V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x02bWb\0\x02bb\0\0hV[b\0\x02z\x81b\0\x02s\x84Tb\0\x01\xB7V[\x84b\0\x01\xF3V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x02\xB2W`\0\x84\x15b\0\x02\x99WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02=V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02\xE3W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02\xC2V[P\x85\x82\x10\x15b\0\x03\x02W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Qa\x0F-b\0\x035`\09`\0\x81\x81a\x01\xD8\x01Ra\x04\xA4\x01Ra\x0F-`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c}\x8A\xD4+\x11a\0\x8CW\x80c\xA6\xDF\xBC\x7F\x11a\0fW\x80c\xA6\xDF\xBC\x7F\x14a\x01\xA5W\x80c\xA7l\x05Q\x14a\x01\xB8W\x80c\xD2#^\xAC\x14a\x01\xC0W\x80c\xE7\xF5\xB8\x1D\x14a\x01\xD3W`\0\x80\xFD[\x80c}\x8A\xD4+\x14a\x01_W\x80c\x81\xC4\\p\x14a\x01gW\x80c\x8Ev\n\xFE\x14a\x01\x92W`\0\x80\xFD[\x80c\x02\xF7}\x19\x14a\0\xD4W\x80c\x05m\xE7\x04\x14a\0\xFCW\x80c\x10\xA5By\x14a\x01\x11W\x80c?\xA0\xCF\xBF\x14a\x01\x19W\x80cL\xFF\x91%\x14a\x019W\x80cp\xF9\xDF\xCA\x14a\x01LW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x07wV[a\x01\xFAV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x0Fa\x01\n6`\x04a\x07\xCBV[a\x022V[\0[a\0\xE7a\x02\xA0V[a\x01,a\x01'6`\x04a\x08qV[a\x03:V[`@Qa\0\xF3\x91\x90a\x08\xF5V[a\x01,a\x01G6`\x04a\x07wV[a\x03cV[a\x01,a\x01Z6`\x04a\t\x08V[a\x03vV[a\x01,a\x03\xB2V[`\0Ta\x01z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[a\0\xE7a\x01\xA06`\x04a\x07wV[a\x04@V[a\0\xE7a\x01\xB36`\x04a\t\xB3V[a\x05&V[a\x01,a\x05>V[a\x01\x0Fa\x01\xCE6`\x04a\t\xF4V[a\x05KV[a\x01z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\x01\x83`@Q` \x01a\x02\x11\x92\x91\x90a\n\xE2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x02+\x81a\x04@V[\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10[\x1C\x99XY\x1EH\x14\xD9]`\xAA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x035`\x02\x80Ta\x02\xB2\x90a\n\xA8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xDE\x90a\n\xA8V[\x80\x15a\x03+W\x80`\x1F\x10a\x03\0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03+V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x0EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x01\xFAV[\x90P\x90V[``\x81`@Q` \x01a\x03M\x91\x90a\x0B\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[``\x81`@Q` \x01a\x03M\x91\x90a\x08\xF5V[``a\x03\x81\x83a\x03:V[a\x03\x8A\x83a\x03cV[`@Q` \x01a\x03\x9B\x92\x91\x90a\x0B\xF0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\x01\x80Ta\x03\xBF\x90a\n\xA8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xEB\x90a\n\xA8V[\x80\x15a\x048W\x80`\x1F\x10a\x04\rWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x048V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x1BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0``\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x04\\\x91\x90a\x0CZV[\x91P\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x04t\x91\x90a\x0C\xB3V[\x93P\x80\x80` \x01\x90Q\x81\x01\x90a\x04\x8A\x91\x90a\rCV[`@Qc:\x9449`\xE2\x1B\x81R\x90\x93P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEAP\xD0\xE4\x90a\x04\xDB\x90\x86\x90\x88\x90`\x04\x01a\rwV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x1C\x91\x90a\r\x9CV[\x96\x95PPPPPPV[`\0a\x054\x82\x84\x01\x84a\x08qV[P`\x01\x93\x92PPPV[`\x02\x80Ta\x03\xBF\x90a\n\xA8V[`\0`@Q\x80`\xE0\x01`@R\x80\x88`\0\x015\x81R` \x01\x88` \x015\x81R` \x01\x88`@\x015\x81R` \x01\x88``\x015\x81R` \x01\x88`\x80\x015\x81R` \x01\x88`\xA0\x01` \x81\x01\x90a\x05\x9D\x91\x90a\x07\xCBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x05\xC3a\x01'a\x05\xBE`\xC0\x8C\x01\x8Ca\r\xBEV[a\x06;V[\x90R`\0T`@Qc8)\xC7\xE5`\xE1\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cpS\x8F\xCA\x90a\x06\0\x90\x84\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01a\x0EOV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x1AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06.W=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[``a\x06G\x83\x83a\x05&V[a\x06\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FPlonk Verifier Wrapper: Invalid `D\x82\x01Rk\x1A[\x9C\x1D]\x08\x19\x9B\xDC\x9BX]`\xA2\x1B`d\x82\x01R`\x84\x01a\x02uV[a\x02+\x82\x84\x01\x84a\x08qV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x06\xF2Wa\x06\xF2a\x06\xB4V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x07\x13Wa\x07\x13a\x06\xB4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x072W`\0\x80\xFD[\x815a\x07Ea\x07@\x82a\x06\xFAV[a\x06\xCAV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x07ZW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x07\x89W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x9FW`\0\x80\xFD[a\x07\xAB\x84\x82\x85\x01a\x07!V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xC8W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x07\xDDW`\0\x80\xFD[\x815a\x02+\x81a\x07\xB3V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x08\x01Wa\x08\x01a\x06\xB4V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x08\x1CW`\0\x80\xFD[\x815` a\x08,a\x07@\x83a\x07\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x08KW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x08fW\x805\x83R\x91\x83\x01\x91\x83\x01a\x08OV[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x08\x83W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x99W`\0\x80\xFD[a\x07\xAB\x84\x82\x85\x01a\x08\x0BV[`\0[\x83\x81\x10\x15a\x08\xC0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\xA8V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x08\xE1\x81` \x86\x01` \x86\x01a\x08\xA5V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x02+` \x83\x01\x84a\x08\xC9V[`\0\x80`@\x83\x85\x03\x12\x15a\t\x1BW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\t2W`\0\x80\xFD[a\t>\x86\x83\x87\x01a\x08\x0BV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\tTW`\0\x80\xFD[Pa\ta\x85\x82\x86\x01a\x07!V[\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\t}W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\x94W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\t\xACW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\t\xC6W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xDCW`\0\x80\xFD[a\t\xE8\x85\x82\x86\x01a\tkV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\n\rW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\n$W`\0\x80\xFD[\x90\x88\x01\x90`\xE0\x82\x8B\x03\x12\x15a\n8W`\0\x80\xFD[\x90\x96P` \x88\x015\x90`\x03\x82\x10a\nNW`\0\x80\xFD[\x90\x95P`@\x88\x015\x90\x80\x82\x11\x15a\ndW`\0\x80\xFD[a\np\x8A\x83\x8B\x01a\tkV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a\n\x89W`\0\x80\xFD[Pa\n\x96\x89\x82\x8A\x01a\tkV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\n\xBCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\n\xDCWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0\x80\x84T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a\x0B\x02W`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\x0B!WcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[`@\x88\x01\x84\x90R``\x88\x01\x82\x80\x15a\x0B@W`\x01\x81\x14a\x0BVWa\x0B\x81V[`\xFF\x19\x87\x16\x82R\x85\x15\x15`\x05\x1B\x82\x01\x97Pa\x0B\x81V[`\0\x8C\x81R` \x90 `\0[\x87\x81\x10\x15a\x0B{W\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a\x0BbV[\x83\x01\x98PP[PP\x87\x86\x03\x81\x89\x01RPPPPPa\x0B\x99\x81\x85a\x08\xC9V[\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x0B\xD2W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0B\xB6V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x02+` \x83\x01\x84a\x0B\xA2V[`@\x81R`\0a\x0C\x03`@\x83\x01\x85a\x08\xC9V[\x82\x81\x03` \x84\x01Ra\x0B\x99\x81\x85a\x08\xC9V[`\0\x82`\x1F\x83\x01\x12a\x0C&W`\0\x80\xFD[\x81Qa\x0C4a\x07@\x82a\x06\xFAV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0CIW`\0\x80\xFD[a\x07\xAB\x82` \x83\x01` \x87\x01a\x08\xA5V[`\0\x80`@\x83\x85\x03\x12\x15a\x0CmW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0C\x84W`\0\x80\xFD[a\x0C\x90\x86\x83\x87\x01a\x0C\x15V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\x0C\xA6W`\0\x80\xFD[Pa\ta\x85\x82\x86\x01a\x0C\x15V[`\0` \x80\x83\x85\x03\x12\x15a\x0C\xC6W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xDCW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0C\xEDW`\0\x80\xFD[\x80Qa\x0C\xFBa\x07@\x82a\x07\xE8V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\r\x1AW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\r8W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\r\x1FV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\rUW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\rkW`\0\x80\xFD[a\x07\xAB\x84\x82\x85\x01a\x0C\x15V[`@\x81R`\0a\r\x8A`@\x83\x01\x85a\x08\xC9V[\x82\x81\x03` \x84\x01Ra\x0B\x99\x81\x85a\x0B\xA2V[`\0` \x82\x84\x03\x12\x15a\r\xAEW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02+W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\r\xD5W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\r\xEFW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\t\xACW`\0\x80\xFD[`\x03\x81\x10a\x0E\"WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x80\x81R\x86Q`\x80\x82\x01R` \x87\x01Q`\xA0\x82\x01R`@\x87\x01Q`\xC0\x82\x01R``\x87\x01Q`\xE0\x82\x01R`\x80\x87\x01Qa\x01\0\x82\x01R`\x01\x80`\xA0\x1B\x03`\xA0\x88\x01Q\x16a\x01 \x82\x01R`\0`\xC0\x88\x01Q`\xE0a\x01@\x84\x01Ra\x0E\xB3a\x01`\x84\x01\x82a\x08\xC9V[\x90Pa\x0E\xC2` \x84\x01\x89a\x0E\x04V[\x82\x81\x03`@\x84\x01Ra\x0E\xD5\x81\x87\x89a\x0E&V[\x90P\x82\x81\x03``\x84\x01Ra\x0E\xEA\x81\x85\x87a\x0E&V[\x99\x98PPPPPPPPPV\xFE\xA2dipfsX\"\x12 \xB9\x08e]\xD6\xD6\x9E7\xA5\x1E \x19\x0F\x9C{\x16\xB5J\xCFV\x15\x8By\xF2o\xD4\xFA\xFE\x9F\xC3\x1A\xDBdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static PLONK_VERIFIER_WRAPPER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c}\x8A\xD4+\x11a\0\x8CW\x80c\xA6\xDF\xBC\x7F\x11a\0fW\x80c\xA6\xDF\xBC\x7F\x14a\x01\xA5W\x80c\xA7l\x05Q\x14a\x01\xB8W\x80c\xD2#^\xAC\x14a\x01\xC0W\x80c\xE7\xF5\xB8\x1D\x14a\x01\xD3W`\0\x80\xFD[\x80c}\x8A\xD4+\x14a\x01_W\x80c\x81\xC4\\p\x14a\x01gW\x80c\x8Ev\n\xFE\x14a\x01\x92W`\0\x80\xFD[\x80c\x02\xF7}\x19\x14a\0\xD4W\x80c\x05m\xE7\x04\x14a\0\xFCW\x80c\x10\xA5By\x14a\x01\x11W\x80c?\xA0\xCF\xBF\x14a\x01\x19W\x80cL\xFF\x91%\x14a\x019W\x80cp\xF9\xDF\xCA\x14a\x01LW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x07wV[a\x01\xFAV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x0Fa\x01\n6`\x04a\x07\xCBV[a\x022V[\0[a\0\xE7a\x02\xA0V[a\x01,a\x01'6`\x04a\x08qV[a\x03:V[`@Qa\0\xF3\x91\x90a\x08\xF5V[a\x01,a\x01G6`\x04a\x07wV[a\x03cV[a\x01,a\x01Z6`\x04a\t\x08V[a\x03vV[a\x01,a\x03\xB2V[`\0Ta\x01z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[a\0\xE7a\x01\xA06`\x04a\x07wV[a\x04@V[a\0\xE7a\x01\xB36`\x04a\t\xB3V[a\x05&V[a\x01,a\x05>V[a\x01\x0Fa\x01\xCE6`\x04a\t\xF4V[a\x05KV[a\x01z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80`\x01\x83`@Q` \x01a\x02\x11\x92\x91\x90a\n\xE2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x02+\x81a\x04@V[\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x10[\x1C\x99XY\x1EH\x14\xD9]`\xAA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x035`\x02\x80Ta\x02\xB2\x90a\n\xA8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xDE\x90a\n\xA8V[\x80\x15a\x03+W\x80`\x1F\x10a\x03\0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03+V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x0EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x01\xFAV[\x90P\x90V[``\x81`@Q` \x01a\x03M\x91\x90a\x0B\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[``\x81`@Q` \x01a\x03M\x91\x90a\x08\xF5V[``a\x03\x81\x83a\x03:V[a\x03\x8A\x83a\x03cV[`@Q` \x01a\x03\x9B\x92\x91\x90a\x0B\xF0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\x01\x80Ta\x03\xBF\x90a\n\xA8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xEB\x90a\n\xA8V[\x80\x15a\x048W\x80`\x1F\x10a\x04\rWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x048V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x1BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0``\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x04\\\x91\x90a\x0CZV[\x91P\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x04t\x91\x90a\x0C\xB3V[\x93P\x80\x80` \x01\x90Q\x81\x01\x90a\x04\x8A\x91\x90a\rCV[`@Qc:\x9449`\xE2\x1B\x81R\x90\x93P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEAP\xD0\xE4\x90a\x04\xDB\x90\x86\x90\x88\x90`\x04\x01a\rwV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x1C\x91\x90a\r\x9CV[\x96\x95PPPPPPV[`\0a\x054\x82\x84\x01\x84a\x08qV[P`\x01\x93\x92PPPV[`\x02\x80Ta\x03\xBF\x90a\n\xA8V[`\0`@Q\x80`\xE0\x01`@R\x80\x88`\0\x015\x81R` \x01\x88` \x015\x81R` \x01\x88`@\x015\x81R` \x01\x88``\x015\x81R` \x01\x88`\x80\x015\x81R` \x01\x88`\xA0\x01` \x81\x01\x90a\x05\x9D\x91\x90a\x07\xCBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x05\xC3a\x01'a\x05\xBE`\xC0\x8C\x01\x8Ca\r\xBEV[a\x06;V[\x90R`\0T`@Qc8)\xC7\xE5`\xE1\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cpS\x8F\xCA\x90a\x06\0\x90\x84\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01a\x0EOV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x1AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06.W=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[``a\x06G\x83\x83a\x05&V[a\x06\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FPlonk Verifier Wrapper: Invalid `D\x82\x01Rk\x1A[\x9C\x1D]\x08\x19\x9B\xDC\x9BX]`\xA2\x1B`d\x82\x01R`\x84\x01a\x02uV[a\x02+\x82\x84\x01\x84a\x08qV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x06\xF2Wa\x06\xF2a\x06\xB4V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x07\x13Wa\x07\x13a\x06\xB4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x072W`\0\x80\xFD[\x815a\x07Ea\x07@\x82a\x06\xFAV[a\x06\xCAV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x07ZW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x07\x89W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x9FW`\0\x80\xFD[a\x07\xAB\x84\x82\x85\x01a\x07!V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xC8W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x07\xDDW`\0\x80\xFD[\x815a\x02+\x81a\x07\xB3V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x08\x01Wa\x08\x01a\x06\xB4V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x08\x1CW`\0\x80\xFD[\x815` a\x08,a\x07@\x83a\x07\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x08KW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x08fW\x805\x83R\x91\x83\x01\x91\x83\x01a\x08OV[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x08\x83W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x99W`\0\x80\xFD[a\x07\xAB\x84\x82\x85\x01a\x08\x0BV[`\0[\x83\x81\x10\x15a\x08\xC0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\xA8V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x08\xE1\x81` \x86\x01` \x86\x01a\x08\xA5V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x02+` \x83\x01\x84a\x08\xC9V[`\0\x80`@\x83\x85\x03\x12\x15a\t\x1BW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\t2W`\0\x80\xFD[a\t>\x86\x83\x87\x01a\x08\x0BV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\tTW`\0\x80\xFD[Pa\ta\x85\x82\x86\x01a\x07!V[\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\t}W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\x94W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\t\xACW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\t\xC6W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xDCW`\0\x80\xFD[a\t\xE8\x85\x82\x86\x01a\tkV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\n\rW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\n$W`\0\x80\xFD[\x90\x88\x01\x90`\xE0\x82\x8B\x03\x12\x15a\n8W`\0\x80\xFD[\x90\x96P` \x88\x015\x90`\x03\x82\x10a\nNW`\0\x80\xFD[\x90\x95P`@\x88\x015\x90\x80\x82\x11\x15a\ndW`\0\x80\xFD[a\np\x8A\x83\x8B\x01a\tkV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a\n\x89W`\0\x80\xFD[Pa\n\x96\x89\x82\x8A\x01a\tkV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\n\xBCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\n\xDCWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0\x80\x84T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a\x0B\x02W`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\x0B!WcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[`@\x88\x01\x84\x90R``\x88\x01\x82\x80\x15a\x0B@W`\x01\x81\x14a\x0BVWa\x0B\x81V[`\xFF\x19\x87\x16\x82R\x85\x15\x15`\x05\x1B\x82\x01\x97Pa\x0B\x81V[`\0\x8C\x81R` \x90 `\0[\x87\x81\x10\x15a\x0B{W\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a\x0BbV[\x83\x01\x98PP[PP\x87\x86\x03\x81\x89\x01RPPPPPa\x0B\x99\x81\x85a\x08\xC9V[\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x0B\xD2W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0B\xB6V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x02+` \x83\x01\x84a\x0B\xA2V[`@\x81R`\0a\x0C\x03`@\x83\x01\x85a\x08\xC9V[\x82\x81\x03` \x84\x01Ra\x0B\x99\x81\x85a\x08\xC9V[`\0\x82`\x1F\x83\x01\x12a\x0C&W`\0\x80\xFD[\x81Qa\x0C4a\x07@\x82a\x06\xFAV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0CIW`\0\x80\xFD[a\x07\xAB\x82` \x83\x01` \x87\x01a\x08\xA5V[`\0\x80`@\x83\x85\x03\x12\x15a\x0CmW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0C\x84W`\0\x80\xFD[a\x0C\x90\x86\x83\x87\x01a\x0C\x15V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\x0C\xA6W`\0\x80\xFD[Pa\ta\x85\x82\x86\x01a\x0C\x15V[`\0` \x80\x83\x85\x03\x12\x15a\x0C\xC6W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xDCW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0C\xEDW`\0\x80\xFD[\x80Qa\x0C\xFBa\x07@\x82a\x07\xE8V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\r\x1AW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\r8W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\r\x1FV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\rUW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\rkW`\0\x80\xFD[a\x07\xAB\x84\x82\x85\x01a\x0C\x15V[`@\x81R`\0a\r\x8A`@\x83\x01\x85a\x08\xC9V[\x82\x81\x03` \x84\x01Ra\x0B\x99\x81\x85a\x0B\xA2V[`\0` \x82\x84\x03\x12\x15a\r\xAEW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02+W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\r\xD5W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\r\xEFW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\t\xACW`\0\x80\xFD[`\x03\x81\x10a\x0E\"WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x80\x81R\x86Q`\x80\x82\x01R` \x87\x01Q`\xA0\x82\x01R`@\x87\x01Q`\xC0\x82\x01R``\x87\x01Q`\xE0\x82\x01R`\x80\x87\x01Qa\x01\0\x82\x01R`\x01\x80`\xA0\x1B\x03`\xA0\x88\x01Q\x16a\x01 \x82\x01R`\0`\xC0\x88\x01Q`\xE0a\x01@\x84\x01Ra\x0E\xB3a\x01`\x84\x01\x82a\x08\xC9V[\x90Pa\x0E\xC2` \x84\x01\x89a\x0E\x04V[\x82\x81\x03`@\x84\x01Ra\x0E\xD5\x81\x87\x89a\x0E&V[\x90P\x82\x81\x03``\x84\x01Ra\x0E\xEA\x81\x85\x87a\x0E&V[\x99\x98PPPPPPPPPV\xFE\xA2dipfsX\"\x12 \xB9\x08e]\xD6\xD6\x9E7\xA5\x1E \x19\x0F\x9C{\x16\xB5J\xCFV\x15\x8By\xF2o\xD4\xFA\xFE\x9F\xC3\x1A\xDBdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static PLONK_VERIFIER_WRAPPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct plonk_verifier_wrapper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for plonk_verifier_wrapper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for plonk_verifier_wrapper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for plonk_verifier_wrapper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for plonk_verifier_wrapper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(plonk_verifier_wrapper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> plonk_verifier_wrapper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PLONK_VERIFIER_WRAPPER_ABI.clone(),
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
                PLONK_VERIFIER_WRAPPER_ABI.clone(),
                PLONK_VERIFIER_WRAPPER_BYTECODE.clone().into(),
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
        ///Calls the contract's `encodeInputAndProofForVerification` (0x70f9dfca) function
        pub fn encode_input_and_proof_for_verification(
            &self,
            inputs: ::std::vec::Vec<[u8; 32]>,
            proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([112, 249, 223, 202], (inputs, proof))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeInputs` (0x3fa0cfbf) function
        pub fn encode_inputs(
            &self,
            inputs: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([63, 160, 207, 191], inputs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeProof` (0x4cff9125) function
        pub fn encode_proof(
            &self,
            proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([76, 255, 145, 37], proof)
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
        for plonk_verifier_wrapper<M>
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
    ///Container type for all input parameters for the `encodeInputAndProofForVerification` function with signature `encodeInputAndProofForVerification(bytes32[],bytes)` and selector `0x70f9dfca`
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
        abi = "encodeInputAndProofForVerification(bytes32[],bytes)"
    )]
    pub struct EncodeInputAndProofForVerificationCall {
        pub inputs: ::std::vec::Vec<[u8; 32]>,
        pub proof: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `encodeInputs` function with signature `encodeInputs(bytes32[])` and selector `0x3fa0cfbf`
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
    #[ethcall(name = "encodeInputs", abi = "encodeInputs(bytes32[])")]
    pub struct EncodeInputsCall {
        pub inputs: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `encodeProof` function with signature `encodeProof(bytes)` and selector `0x4cff9125`
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
    #[ethcall(name = "encodeProof", abi = "encodeProof(bytes)")]
    pub struct EncodeProofCall {
        pub proof: ::ethers::core::types::Bytes,
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
    pub enum plonk_verifier_wrapperCalls {
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
    impl ::ethers::core::abi::AbiDecode for plonk_verifier_wrapperCalls {
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
    impl ::ethers::core::abi::AbiEncode for plonk_verifier_wrapperCalls {
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
    impl ::core::fmt::Display for plonk_verifier_wrapperCalls {
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
    impl ::core::convert::From<CheckSampleInputsAndProofCall> for plonk_verifier_wrapperCalls {
        fn from(value: CheckSampleInputsAndProofCall) -> Self {
            Self::CheckSampleInputsAndProof(value)
        }
    }
    impl ::core::convert::From<CreateRequestCall> for plonk_verifier_wrapperCalls {
        fn from(value: CreateRequestCall) -> Self {
            Self::CreateRequest(value)
        }
    }
    impl ::core::convert::From<EncodeInputAndProofForVerificationCall> for plonk_verifier_wrapperCalls {
        fn from(value: EncodeInputAndProofForVerificationCall) -> Self {
            Self::EncodeInputAndProofForVerification(value)
        }
    }
    impl ::core::convert::From<EncodeInputsCall> for plonk_verifier_wrapperCalls {
        fn from(value: EncodeInputsCall) -> Self {
            Self::EncodeInputs(value)
        }
    }
    impl ::core::convert::From<EncodeProofCall> for plonk_verifier_wrapperCalls {
        fn from(value: EncodeProofCall) -> Self {
            Self::EncodeProof(value)
        }
    }
    impl ::core::convert::From<IverifierCall> for plonk_verifier_wrapperCalls {
        fn from(value: IverifierCall) -> Self {
            Self::Iverifier(value)
        }
    }
    impl ::core::convert::From<ProofMarketplaceCall> for plonk_verifier_wrapperCalls {
        fn from(value: ProofMarketplaceCall) -> Self {
            Self::ProofMarketplace(value)
        }
    }
    impl ::core::convert::From<SampleInputCall> for plonk_verifier_wrapperCalls {
        fn from(value: SampleInputCall) -> Self {
            Self::SampleInput(value)
        }
    }
    impl ::core::convert::From<SampleProofCall> for plonk_verifier_wrapperCalls {
        fn from(value: SampleProofCall) -> Self {
            Self::SampleProof(value)
        }
    }
    impl ::core::convert::From<SetProofMarketplaceContractCall> for plonk_verifier_wrapperCalls {
        fn from(value: SetProofMarketplaceContractCall) -> Self {
            Self::SetProofMarketplaceContract(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for plonk_verifier_wrapperCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifyAgainstSampleInputsCall> for plonk_verifier_wrapperCalls {
        fn from(value: VerifyAgainstSampleInputsCall) -> Self {
            Self::VerifyAgainstSampleInputs(value)
        }
    }
    impl ::core::convert::From<VerifyInputsCall> for plonk_verifier_wrapperCalls {
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
    ///Container type for all return fields from the `encodeInputAndProofForVerification` function with signature `encodeInputAndProofForVerification(bytes32[],bytes)` and selector `0x70f9dfca`
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
    ///Container type for all return fields from the `encodeInputs` function with signature `encodeInputs(bytes32[])` and selector `0x3fa0cfbf`
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
    ///Container type for all return fields from the `encodeProof` function with signature `encodeProof(bytes)` and selector `0x4cff9125`
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
