pub use input_and_proof_format_registry::*;
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
pub mod input_and_proof_format_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_admin"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("inputArrayLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("inputArrayLength"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("inputs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("inputs"),
                        inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("proofArrayLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("proofArrayLength"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("proofs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("proofs"),
                        inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("setInputFormat"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setInputFormat"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("inputsFormat"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProofFormat"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setProofFormat"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proofFormat"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static INPUTANDPROOFFORMATREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x08P8\x03\x80a\x08P\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x07\xB7a\0\x99`\09`\0\x81\x81a\x015\x01R\x81\x81a\x023\x01Ra\x02\xF0\x01Ra\x07\xB7`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xC0\xA5\x94q\x11a\0[W\x80c\xC0\xA5\x94q\x14a\0\xF5W\x80c\xCC\xC1^I\x14a\x01\nW\x80c\xE8j\xA1\x90\x14a\x01\x1DW\x80c\xF8Q\xA4@\x14a\x010W`\0\x80\xFD[\x80c\x0C\xE4\x91L\x14a\0\x82W\x80c^\xF7yI\x14a\0\xB5W\x80c\xBE\xD7\x8C\xA3\x14a\0\xD5W[`\0\x80\xFD[a\0\xA2a\0\x906`\x04a\x04DV[`\x02` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC8a\0\xC36`\x04a\x04]V[a\x01oV[`@Qa\0\xAC\x91\x90a\x04\x7FV[a\0\xA2a\0\xE36`\x04a\x04DV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x01\x08a\x01\x036`\x04a\x05\x14V[a\x02(V[\0[a\0\xC8a\x01\x186`\x04a\x04]V[a\x02\xC9V[a\x01\x08a\x01+6`\x04a\x05\x14V[a\x02\xE5V[a\x01W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xACV[`\x01` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x01\x8BW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PP\x80Ta\x01\xA7\x90a\x068V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xD3\x90a\x068V[\x80\x15a\x02 W\x80`\x1F\x10a\x01\xF5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02 V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x03W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm7\xB76<\x900\xB26\xB4\xB7\x101\xB0\xB7`\x91\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x01` \x90\x81R`@\x90\x91 \x82Qa\x02\xB5\x92\x84\x01\x90a\x03\x7FV[PQ`\0\x91\x82R`\x03` R`@\x90\x91 UV[`\0` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x01\x8BW`\0\x80\xFD[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm7\xB76<\x900\xB26\xB4\xB7\x101\xB0\xB7`\x91\x1B`D\x82\x01R`d\x01a\x02\x8DV[`\0\x82\x81R` \x81\x81R`@\x90\x91 \x82Qa\x03k\x92\x84\x01\x90a\x03\x7FV[PQ`\0\x91\x82R`\x02` R`@\x90\x91 UV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x03\xC5W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x03\xC5W\x82Q\x82\x90a\x03\xB5\x90\x82a\x06\xC1V[P\x91` \x01\x91\x90`\x01\x01\x90a\x03\x9FV[Pa\x03\xD1\x92\x91Pa\x03\xD5V[P\x90V[\x80\x82\x11\x15a\x03\xD1W`\0a\x03\xE9\x82\x82a\x03\xF2V[P`\x01\x01a\x03\xD5V[P\x80Ta\x03\xFE\x90a\x068V[`\0\x82U\x80`\x1F\x10a\x04\x0EWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x04,\x91\x90a\x04/V[PV[[\x80\x82\x11\x15a\x03\xD1W`\0\x81U`\x01\x01a\x040V[`\0` \x82\x84\x03\x12\x15a\x04VW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x04pW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x04\xACW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x04\x90V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\x0CWa\x05\x0Ca\x04\xCDV[`@R\x91\x90PV[`\0\x80`@\x80\x84\x86\x03\x12\x15a\x05(W`\0\x80\xFD[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05HW`\0\x80\xFD[\x81\x87\x01\x91P`\x1F\x88\x81\x84\x01\x12a\x05]W`\0\x80\xFD[\x825\x82\x81\x11\x15a\x05oWa\x05oa\x04\xCDV[\x80`\x05\x1Ba\x05~\x86\x82\x01a\x04\xE3V[\x91\x82R\x84\x81\x01\x86\x01\x91\x86\x81\x01\x90\x8C\x84\x11\x15a\x05\x98W`\0\x80\xFD[\x87\x87\x01\x92P[\x83\x83\x10\x15a\x06%W\x825\x86\x81\x11\x15a\x05\xB6W`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x8E\x13a\x05\xC8W`\0\x80\x81\xFD[\x88\x81\x015\x87\x81\x11\x15a\x05\xDCWa\x05\xDCa\x04\xCDV[a\x05\xED\x81\x88\x01`\x1F\x19\x16\x8B\x01a\x04\xE3V[\x81\x81R\x8F\x8C\x83\x85\x01\x01\x11\x15a\x06\x02W`\0\x80\x81\xFD[\x81\x8C\x84\x01\x8C\x83\x017`\0\x91\x81\x01\x8B\x01\x91\x90\x91R\x83RP\x91\x87\x01\x91\x90\x87\x01\x90a\x05\x9EV[\x80\x99PPPPPPPPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06LW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06lWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x06\xBCW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x06\x99WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x06\xB8W\x82\x81U`\x01\x01a\x06\xA5V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xDBWa\x06\xDBa\x04\xCDV[a\x06\xEF\x81a\x06\xE9\x84Ta\x068V[\x84a\x06rV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x07$W`\0\x84\x15a\x07\x0CWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x06\xB8V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x07SW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x074V[P\x85\x82\x10\x15a\x07qW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \x01O\x89\xD1\x19\x80\xE9\x9B\xB0\x90J7\0\xAB@\xD5\x0F\xCDo\xF2f\\\x1Co\xED2v\x99B\x9A\x9C\x0FdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static INPUTANDPROOFFORMATREGISTRY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xC0\xA5\x94q\x11a\0[W\x80c\xC0\xA5\x94q\x14a\0\xF5W\x80c\xCC\xC1^I\x14a\x01\nW\x80c\xE8j\xA1\x90\x14a\x01\x1DW\x80c\xF8Q\xA4@\x14a\x010W`\0\x80\xFD[\x80c\x0C\xE4\x91L\x14a\0\x82W\x80c^\xF7yI\x14a\0\xB5W\x80c\xBE\xD7\x8C\xA3\x14a\0\xD5W[`\0\x80\xFD[a\0\xA2a\0\x906`\x04a\x04DV[`\x02` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC8a\0\xC36`\x04a\x04]V[a\x01oV[`@Qa\0\xAC\x91\x90a\x04\x7FV[a\0\xA2a\0\xE36`\x04a\x04DV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x01\x08a\x01\x036`\x04a\x05\x14V[a\x02(V[\0[a\0\xC8a\x01\x186`\x04a\x04]V[a\x02\xC9V[a\x01\x08a\x01+6`\x04a\x05\x14V[a\x02\xE5V[a\x01W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xACV[`\x01` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x01\x8BW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PP\x80Ta\x01\xA7\x90a\x068V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xD3\x90a\x068V[\x80\x15a\x02 W\x80`\x1F\x10a\x01\xF5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02 V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x03W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm7\xB76<\x900\xB26\xB4\xB7\x101\xB0\xB7`\x91\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x01` \x90\x81R`@\x90\x91 \x82Qa\x02\xB5\x92\x84\x01\x90a\x03\x7FV[PQ`\0\x91\x82R`\x03` R`@\x90\x91 UV[`\0` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x01\x8BW`\0\x80\xFD[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm7\xB76<\x900\xB26\xB4\xB7\x101\xB0\xB7`\x91\x1B`D\x82\x01R`d\x01a\x02\x8DV[`\0\x82\x81R` \x81\x81R`@\x90\x91 \x82Qa\x03k\x92\x84\x01\x90a\x03\x7FV[PQ`\0\x91\x82R`\x02` R`@\x90\x91 UV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x03\xC5W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x03\xC5W\x82Q\x82\x90a\x03\xB5\x90\x82a\x06\xC1V[P\x91` \x01\x91\x90`\x01\x01\x90a\x03\x9FV[Pa\x03\xD1\x92\x91Pa\x03\xD5V[P\x90V[\x80\x82\x11\x15a\x03\xD1W`\0a\x03\xE9\x82\x82a\x03\xF2V[P`\x01\x01a\x03\xD5V[P\x80Ta\x03\xFE\x90a\x068V[`\0\x82U\x80`\x1F\x10a\x04\x0EWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x04,\x91\x90a\x04/V[PV[[\x80\x82\x11\x15a\x03\xD1W`\0\x81U`\x01\x01a\x040V[`\0` \x82\x84\x03\x12\x15a\x04VW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x04pW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x04\xACW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x04\x90V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\x0CWa\x05\x0Ca\x04\xCDV[`@R\x91\x90PV[`\0\x80`@\x80\x84\x86\x03\x12\x15a\x05(W`\0\x80\xFD[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05HW`\0\x80\xFD[\x81\x87\x01\x91P`\x1F\x88\x81\x84\x01\x12a\x05]W`\0\x80\xFD[\x825\x82\x81\x11\x15a\x05oWa\x05oa\x04\xCDV[\x80`\x05\x1Ba\x05~\x86\x82\x01a\x04\xE3V[\x91\x82R\x84\x81\x01\x86\x01\x91\x86\x81\x01\x90\x8C\x84\x11\x15a\x05\x98W`\0\x80\xFD[\x87\x87\x01\x92P[\x83\x83\x10\x15a\x06%W\x825\x86\x81\x11\x15a\x05\xB6W`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x8E\x13a\x05\xC8W`\0\x80\x81\xFD[\x88\x81\x015\x87\x81\x11\x15a\x05\xDCWa\x05\xDCa\x04\xCDV[a\x05\xED\x81\x88\x01`\x1F\x19\x16\x8B\x01a\x04\xE3V[\x81\x81R\x8F\x8C\x83\x85\x01\x01\x11\x15a\x06\x02W`\0\x80\x81\xFD[\x81\x8C\x84\x01\x8C\x83\x017`\0\x91\x81\x01\x8B\x01\x91\x90\x91R\x83RP\x91\x87\x01\x91\x90\x87\x01\x90a\x05\x9EV[\x80\x99PPPPPPPPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06LW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06lWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x06\xBCW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x06\x99WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x06\xB8W\x82\x81U`\x01\x01a\x06\xA5V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xDBWa\x06\xDBa\x04\xCDV[a\x06\xEF\x81a\x06\xE9\x84Ta\x068V[\x84a\x06rV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x07$W`\0\x84\x15a\x07\x0CWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x06\xB8V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x07SW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x074V[P\x85\x82\x10\x15a\x07qW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \x01O\x89\xD1\x19\x80\xE9\x9B\xB0\x90J7\0\xAB@\xD5\x0F\xCDo\xF2f\\\x1Co\xED2v\x99B\x9A\x9C\x0FdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static INPUTANDPROOFFORMATREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct InputAndProofFormatRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InputAndProofFormatRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InputAndProofFormatRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InputAndProofFormatRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InputAndProofFormatRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(InputAndProofFormatRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InputAndProofFormatRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                INPUTANDPROOFFORMATREGISTRY_ABI.clone(),
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
                INPUTANDPROOFFORMATREGISTRY_ABI.clone(),
                INPUTANDPROOFFORMATREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inputArrayLength` (0x0ce4914c) function
        pub fn input_array_length(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([12, 228, 145, 76], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inputs` (0xccc15e49) function
        pub fn inputs(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([204, 193, 94, 73], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proofArrayLength` (0xbed78ca3) function
        pub fn proof_array_length(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([190, 215, 140, 163], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proofs` (0x5ef77949) function
        pub fn proofs(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([94, 247, 121, 73], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setInputFormat` (0xe86aa190) function
        pub fn set_input_format(
            &self,
            market_id: ::ethers::core::types::U256,
            inputs_format: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 106, 161, 144], (market_id, inputs_format))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProofFormat` (0xc0a59471) function
        pub fn set_proof_format(
            &self,
            market_id: ::ethers::core::types::U256,
            proof_format: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 165, 148, 113], (market_id, proof_format))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for InputAndProofFormatRegistry<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    ///Container type for all input parameters for the `inputArrayLength` function with signature `inputArrayLength(uint256)` and selector `0x0ce4914c`
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
    #[ethcall(name = "inputArrayLength", abi = "inputArrayLength(uint256)")]
    pub struct InputArrayLengthCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `inputs` function with signature `inputs(uint256,uint256)` and selector `0xccc15e49`
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
    #[ethcall(name = "inputs", abi = "inputs(uint256,uint256)")]
    pub struct InputsCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `proofArrayLength` function with signature `proofArrayLength(uint256)` and selector `0xbed78ca3`
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
    #[ethcall(name = "proofArrayLength", abi = "proofArrayLength(uint256)")]
    pub struct ProofArrayLengthCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `proofs` function with signature `proofs(uint256,uint256)` and selector `0x5ef77949`
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
    #[ethcall(name = "proofs", abi = "proofs(uint256,uint256)")]
    pub struct ProofsCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `setInputFormat` function with signature `setInputFormat(uint256,string[])` and selector `0xe86aa190`
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
    #[ethcall(name = "setInputFormat", abi = "setInputFormat(uint256,string[])")]
    pub struct SetInputFormatCall {
        pub market_id: ::ethers::core::types::U256,
        pub inputs_format: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all input parameters for the `setProofFormat` function with signature `setProofFormat(uint256,string[])` and selector `0xc0a59471`
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
    #[ethcall(name = "setProofFormat", abi = "setProofFormat(uint256,string[])")]
    pub struct SetProofFormatCall {
        pub market_id: ::ethers::core::types::U256,
        pub proof_format: ::std::vec::Vec<::std::string::String>,
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
    pub enum InputAndProofFormatRegistryCalls {
        Admin(AdminCall),
        InputArrayLength(InputArrayLengthCall),
        Inputs(InputsCall),
        ProofArrayLength(ProofArrayLengthCall),
        Proofs(ProofsCall),
        SetInputFormat(SetInputFormatCall),
        SetProofFormat(SetProofFormatCall),
    }
    impl ::ethers::core::abi::AbiDecode for InputAndProofFormatRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) =
                <InputArrayLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InputArrayLength(decoded));
            }
            if let Ok(decoded) = <InputsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Inputs(decoded));
            }
            if let Ok(decoded) =
                <ProofArrayLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProofArrayLength(decoded));
            }
            if let Ok(decoded) = <ProofsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Proofs(decoded));
            }
            if let Ok(decoded) =
                <SetInputFormatCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetInputFormat(decoded));
            }
            if let Ok(decoded) =
                <SetProofFormatCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProofFormat(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for InputAndProofFormatRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InputArrayLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Inputs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProofArrayLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Proofs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetInputFormat(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProofFormat(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for InputAndProofFormatRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::InputArrayLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Inputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofArrayLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Proofs(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetInputFormat(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProofFormat(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminCall> for InputAndProofFormatRegistryCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<InputArrayLengthCall> for InputAndProofFormatRegistryCalls {
        fn from(value: InputArrayLengthCall) -> Self {
            Self::InputArrayLength(value)
        }
    }
    impl ::core::convert::From<InputsCall> for InputAndProofFormatRegistryCalls {
        fn from(value: InputsCall) -> Self {
            Self::Inputs(value)
        }
    }
    impl ::core::convert::From<ProofArrayLengthCall> for InputAndProofFormatRegistryCalls {
        fn from(value: ProofArrayLengthCall) -> Self {
            Self::ProofArrayLength(value)
        }
    }
    impl ::core::convert::From<ProofsCall> for InputAndProofFormatRegistryCalls {
        fn from(value: ProofsCall) -> Self {
            Self::Proofs(value)
        }
    }
    impl ::core::convert::From<SetInputFormatCall> for InputAndProofFormatRegistryCalls {
        fn from(value: SetInputFormatCall) -> Self {
            Self::SetInputFormat(value)
        }
    }
    impl ::core::convert::From<SetProofFormatCall> for InputAndProofFormatRegistryCalls {
        fn from(value: SetProofFormatCall) -> Self {
            Self::SetProofFormat(value)
        }
    }
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
    ///Container type for all return fields from the `inputArrayLength` function with signature `inputArrayLength(uint256)` and selector `0x0ce4914c`
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
    pub struct InputArrayLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `inputs` function with signature `inputs(uint256,uint256)` and selector `0xccc15e49`
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
    pub struct InputsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `proofArrayLength` function with signature `proofArrayLength(uint256)` and selector `0xbed78ca3`
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
    pub struct ProofArrayLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proofs` function with signature `proofs(uint256,uint256)` and selector `0x5ef77949`
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
    pub struct ProofsReturn(pub ::std::string::String);
}
