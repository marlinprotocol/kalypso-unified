pub use zksync_verifier_wrapper::*;
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
pub mod zksync_verifier_wrapper {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_iverifier"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract i_zksync_verifier",),
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
                    ::std::borrow::ToOwned::to_owned("iverifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("iverifier"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract i_zksync_verifier",),
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
    pub static ZKSYNC_VERIFIER_WRAPPER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x11\xF68\x03\x80b\0\x11\xF6\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x03WV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\x80R`\0b\0\0O\x83\x82b\0\x04rV[P`\x01b\0\0^\x82\x82b\0\x04rV[Pb\0\0ib\0\0\xB7V[b\0\0\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x10\xD8[\x89\xDD\x08\x18\x99H\x19\x19\\\x1B\x1B\xDEYY`z\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[PPPb\0\x08BV[`\0b\0\x01W`\x01\x80Tb\0\0\xCC\x90b\0\x03\xE1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\0\xFA\x90b\0\x03\xE1V[\x80\x15b\0\x01KW\x80`\x1F\x10b\0\x01\x1FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x01KV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x01-W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPb\0\x01\\\x92PPPV[\x90P\x90V[`\0\x80`\0\x83`@Q` \x01b\0\x01u\x92\x91\x90b\0\x05lV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90Pb\0\x01\x92\x81b\0\x01\x99V[\x93\x92PPPV[`\0```\0\x80\x84\x80` \x01\x90Q\x81\x01\x90b\0\x01\xB6\x91\x90b\0\x06\x18V[\x91P\x91P\x81\x80` \x01\x90Q\x81\x01\x90b\0\x01\xD0\x91\x90b\0\x06\xFFV[\x92P``\x80\x82\x80` \x01\x90Q\x81\x01\x90b\0\x01\xEB\x91\x90b\0\x077V[`\x80Q`@Qc\x87\xD9\xD0#`\xE0\x1B\x81R\x92\x94P\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\x87\xD9\xD0#\x90b\0\x02%\x90\x88\x90\x86\x90\x86\x90`\x04\x01b\0\x07\xD5V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x02CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02i\x91\x90b\0\x08\x1EV[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x02\xB5Wb\0\x02\xB5b\0\x02tV[`@R\x91\x90PV[`\0[\x83\x81\x10\x15b\0\x02\xDAW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x02\xC0V[PP`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12b\0\x02\xF5W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03\x11Wb\0\x03\x11b\0\x02tV[b\0\x03&`\x1F\x82\x01`\x1F\x19\x16` \x01b\0\x02\x8AV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15b\0\x03<W`\0\x80\xFD[b\0\x03O\x82` \x83\x01` \x87\x01b\0\x02\xBDV[\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x03mW`\0\x80\xFD[\x83Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x03\x85W`\0\x80\xFD[` \x85\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x03\xA3W`\0\x80\xFD[b\0\x03\xB1\x87\x83\x88\x01b\0\x02\xE3V[\x93P`@\x86\x01Q\x91P\x80\x82\x11\x15b\0\x03\xC8W`\0\x80\xFD[Pb\0\x03\xD7\x86\x82\x87\x01b\0\x02\xE3V[\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x03\xF6W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x04\x17WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x04mW`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x04HWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x04iW\x82\x81U`\x01\x01b\0\x04TV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x04\x8EWb\0\x04\x8Eb\0\x02tV[b\0\x04\xA6\x81b\0\x04\x9F\x84Tb\0\x03\xE1V[\x84b\0\x04\x1DV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x04\xDEW`\0\x84\x15b\0\x04\xC5WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x04iV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x05\x0FW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x04\xEEV[P\x85\x82\x10\x15b\0\x05.W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x81Q\x80\x84Rb\0\x05X\x81` \x86\x01` \x86\x01b\0\x02\xBDV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0\x80\x84Tb\0\x05\x80\x81b\0\x03\xE1V[\x80`@\x86\x01R```\x01\x80\x84\x16`\0\x81\x14b\0\x05\xA5W`\x01\x81\x14b\0\x05\xC2Wb\0\x05\xF6V[`\xFF\x19\x85\x16``\x89\x01R``\x84\x15\x15`\x05\x1B\x89\x01\x01\x95Pb\0\x05\xF6V[\x89`\0R` \x80`\0 `\0[\x86\x81\x10\x15b\0\x05\xECW\x81T\x8B\x82\x01\x87\x01R\x90\x84\x01\x90\x82\x01b\0\x05\xCFV[\x8A\x01``\x01\x97PPP[PPPPP\x82\x81\x03` \x84\x01Rb\0\x06\x0F\x81\x85b\0\x05>V[\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x06,W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x06DW`\0\x80\xFD[b\0\x06R\x86\x83\x87\x01b\0\x02\xE3V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15b\0\x06iW`\0\x80\xFD[Pb\0\x06x\x85\x82\x86\x01b\0\x02\xE3V[\x91PP\x92P\x92\x90PV[`\0\x82`\x1F\x83\x01\x12b\0\x06\x94W`\0\x80\xFD[\x81Q` `\x01`\x01`@\x1B\x03\x82\x11\x15b\0\x06\xB2Wb\0\x06\xB2b\0\x02tV[\x81`\x05\x1Bb\0\x06\xC3\x82\x82\x01b\0\x02\x8AV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15b\0\x06\xDEW`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15b\0\x02iW\x82Q\x82R\x91\x83\x01\x91\x90\x83\x01\x90b\0\x06\xE4V[`\0` \x82\x84\x03\x12\x15b\0\x07\x12W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x07)W`\0\x80\xFD[b\0\x03O\x84\x82\x85\x01b\0\x06\x82V[`\0\x80`@\x83\x85\x03\x12\x15b\0\x07KW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x07cW`\0\x80\xFD[b\0\x07q\x86\x83\x87\x01b\0\x06\x82V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15b\0\x07\x88W`\0\x80\xFD[Pb\0\x06x\x85\x82\x86\x01b\0\x06\x82V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15b\0\x07\xCAW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01b\0\x07\xACV[P\x94\x95\x94PPPPPV[``\x81R`\0b\0\x07\xEA``\x83\x01\x86b\0\x07\x97V[\x82\x81\x03` \x84\x01Rb\0\x07\xFE\x81\x86b\0\x07\x97V[\x90P\x82\x81\x03`@\x84\x01Rb\0\x08\x14\x81\x85b\0\x07\x97V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15b\0\x081W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x01\x92W`\0\x80\xFD[`\x80Qa\t\x92b\0\x08d`\09`\0\x81\x81`\xFA\x01Ra\x02\xFC\x01Ra\t\x92`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8Ev\n\xFE\x11a\0[W\x80c\x8Ev\n\xFE\x14a\0\xC7W\x80c\xA6\xDF\xBC\x7F\x14a\0\xDAW\x80c\xA7l\x05Q\x14a\0\xEDW\x80c\xE7\xF5\xB8\x1D\x14a\0\xF5W`\0\x80\xFD[\x80c\x02\xF7}\x19\x14a\0\x82W\x80c\x10\xA5By\x14a\0\xAAW\x80c}\x8A\xD4+\x14a\0\xB2W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x04\x15V[a\x014V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x95a\x01lV[a\0\xBAa\x02\x06V[`@Qa\0\xA1\x91\x90a\x04\xE5V[a\0\x95a\0\xD56`\x04a\x04\x15V[a\x02\x94V[a\0\x95a\0\xE86`\x04a\x04\xF8V[a\x03\x81V[a\0\xBAa\x03\x99V[a\x01\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xA1V[`\0\x80`\0\x83`@Q` \x01a\x01K\x92\x91\x90a\x05\xA4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x01e\x81a\x02\x94V[\x93\x92PPPV[`\0a\x02\x01`\x01\x80Ta\x01~\x90a\x05jV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xAA\x90a\x05jV[\x80\x15a\x01\xF7W\x80`\x1F\x10a\x01\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xF7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\xDAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x014V[\x90P\x90V[`\0\x80Ta\x02\x13\x90a\x05jV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02?\x90a\x05jV[\x80\x15a\x02\x8CW\x80`\x1F\x10a\x02aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x8CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02oW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0```\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x02\xAF\x91\x90a\x06\xB4V[\x91P\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x02\xC7\x91\x90a\x07\xA6V[\x92P``\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x02\xE0\x91\x90a\x07\xDBV[`@Qc\x87\xD9\xD0#`\xE0\x1B\x81R\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87\xD9\xD0#\x90a\x035\x90\x88\x90\x86\x90\x86\x90`\x04\x01a\x08qV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03v\x91\x90a\x08\xB4V[\x97\x96PPPPPPPV[`\0a\x03\x8F\x82\x84\x01\x84a\x08\xD6V[P`\x01\x93\x92PPPV[`\x01\x80Ta\x02\x13\x90a\x05jV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xE5Wa\x03\xE5a\x03\xA6V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04\x07Wa\x04\x07a\x03\xA6V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a\x04'W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04>W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x04OW`\0\x80\xFD[\x805a\x04ba\x04]\x82a\x03\xEDV[a\x03\xBCV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x04wW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\0[\x83\x81\x10\x15a\x04\xB0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\x98V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x04\xD1\x81` \x86\x01` \x86\x01a\x04\x95V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x01e` \x83\x01\x84a\x04\xB9V[`\0\x80` \x83\x85\x03\x12\x15a\x05\x0BW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05#W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x057W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05FW`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x05XW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x05~W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x05\x9EWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0\x80\x84T\x81`\x01\x82`\x01\x1C\x91P`\x01\x83\x16\x80a\x05\xC6W`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\x05\xE5WcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[`@\x88\x01\x84\x90R``\x88\x01\x82\x80\x15a\x06\x04W`\x01\x81\x14a\x06\x1AWa\x06EV[`\xFF\x19\x87\x16\x82R\x85\x15\x15`\x05\x1B\x82\x01\x97Pa\x06EV[`\0\x8C\x81R` \x90 `\0[\x87\x81\x10\x15a\x06?W\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a\x06&V[\x83\x01\x98PP[PPPPPPP\x82\x81\x03` \x84\x01Ra\x06^\x81\x85a\x04\xB9V[\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a\x06xW`\0\x80\xFD[\x81Qa\x06\x86a\x04]\x82a\x03\xEDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06\x9BW`\0\x80\xFD[a\x06\xAC\x82` \x83\x01` \x87\x01a\x04\x95V[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xC7W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xDFW`\0\x80\xFD[a\x06\xEB\x86\x83\x87\x01a\x06gV[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\x07\x01W`\0\x80\xFD[Pa\x07\x0E\x85\x82\x86\x01a\x06gV[\x91PP\x92P\x92\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x072Wa\x072a\x03\xA6V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x07MW`\0\x80\xFD[\x81Q` a\x07]a\x04]\x83a\x07\x18V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x07\x7FW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x07\x9BW\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x07\x84V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x07\xB8W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xCFW`\0\x80\xFD[a\x06\xAC\x84\x82\x85\x01a\x07<V[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xEEW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x06W`\0\x80\xFD[a\x08\x12\x86\x83\x87\x01a\x07<V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\x08(W`\0\x80\xFD[Pa\x07\x0E\x85\x82\x86\x01a\x07<V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x08fW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x08JV[P\x94\x95\x94PPPPPV[``\x81R`\0a\x08\x84``\x83\x01\x86a\x085V[\x82\x81\x03` \x84\x01Ra\x08\x96\x81\x86a\x085V[\x90P\x82\x81\x03`@\x84\x01Ra\x08\xAA\x81\x85a\x085V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x08\xC6W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x01eW`\0\x80\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x08\xE9W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\0W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\t\x11W`\0\x80\xFD[\x805a\t\x1Fa\x04]\x82a\x07\x18V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\t>W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x03vW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\tCV\xFE\xA2dipfsX\"\x12 4I\xD3\x9A\xDD/\x19\xA2\xFC\0\x12\xEE\x8A\xB1\x0BLy;Z\xFAV\x19\xE1\xB5b3\xB5\x94\xB6\xE9\xEA{dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static ZKSYNC_VERIFIER_WRAPPER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8Ev\n\xFE\x11a\0[W\x80c\x8Ev\n\xFE\x14a\0\xC7W\x80c\xA6\xDF\xBC\x7F\x14a\0\xDAW\x80c\xA7l\x05Q\x14a\0\xEDW\x80c\xE7\xF5\xB8\x1D\x14a\0\xF5W`\0\x80\xFD[\x80c\x02\xF7}\x19\x14a\0\x82W\x80c\x10\xA5By\x14a\0\xAAW\x80c}\x8A\xD4+\x14a\0\xB2W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x04\x15V[a\x014V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x95a\x01lV[a\0\xBAa\x02\x06V[`@Qa\0\xA1\x91\x90a\x04\xE5V[a\0\x95a\0\xD56`\x04a\x04\x15V[a\x02\x94V[a\0\x95a\0\xE86`\x04a\x04\xF8V[a\x03\x81V[a\0\xBAa\x03\x99V[a\x01\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xA1V[`\0\x80`\0\x83`@Q` \x01a\x01K\x92\x91\x90a\x05\xA4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x01e\x81a\x02\x94V[\x93\x92PPPV[`\0a\x02\x01`\x01\x80Ta\x01~\x90a\x05jV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xAA\x90a\x05jV[\x80\x15a\x01\xF7W\x80`\x1F\x10a\x01\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xF7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\xDAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x014V[\x90P\x90V[`\0\x80Ta\x02\x13\x90a\x05jV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02?\x90a\x05jV[\x80\x15a\x02\x8CW\x80`\x1F\x10a\x02aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x8CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02oW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0```\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x02\xAF\x91\x90a\x06\xB4V[\x91P\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x02\xC7\x91\x90a\x07\xA6V[\x92P``\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x02\xE0\x91\x90a\x07\xDBV[`@Qc\x87\xD9\xD0#`\xE0\x1B\x81R\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87\xD9\xD0#\x90a\x035\x90\x88\x90\x86\x90\x86\x90`\x04\x01a\x08qV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03v\x91\x90a\x08\xB4V[\x97\x96PPPPPPPV[`\0a\x03\x8F\x82\x84\x01\x84a\x08\xD6V[P`\x01\x93\x92PPPV[`\x01\x80Ta\x02\x13\x90a\x05jV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xE5Wa\x03\xE5a\x03\xA6V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04\x07Wa\x04\x07a\x03\xA6V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a\x04'W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04>W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x04OW`\0\x80\xFD[\x805a\x04ba\x04]\x82a\x03\xEDV[a\x03\xBCV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x04wW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\0[\x83\x81\x10\x15a\x04\xB0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\x98V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x04\xD1\x81` \x86\x01` \x86\x01a\x04\x95V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x01e` \x83\x01\x84a\x04\xB9V[`\0\x80` \x83\x85\x03\x12\x15a\x05\x0BW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05#W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x057W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05FW`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x05XW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x05~W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x05\x9EWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0\x80\x84T\x81`\x01\x82`\x01\x1C\x91P`\x01\x83\x16\x80a\x05\xC6W`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\x05\xE5WcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[`@\x88\x01\x84\x90R``\x88\x01\x82\x80\x15a\x06\x04W`\x01\x81\x14a\x06\x1AWa\x06EV[`\xFF\x19\x87\x16\x82R\x85\x15\x15`\x05\x1B\x82\x01\x97Pa\x06EV[`\0\x8C\x81R` \x90 `\0[\x87\x81\x10\x15a\x06?W\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a\x06&V[\x83\x01\x98PP[PPPPPPP\x82\x81\x03` \x84\x01Ra\x06^\x81\x85a\x04\xB9V[\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a\x06xW`\0\x80\xFD[\x81Qa\x06\x86a\x04]\x82a\x03\xEDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06\x9BW`\0\x80\xFD[a\x06\xAC\x82` \x83\x01` \x87\x01a\x04\x95V[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xC7W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xDFW`\0\x80\xFD[a\x06\xEB\x86\x83\x87\x01a\x06gV[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\x07\x01W`\0\x80\xFD[Pa\x07\x0E\x85\x82\x86\x01a\x06gV[\x91PP\x92P\x92\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x072Wa\x072a\x03\xA6V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x07MW`\0\x80\xFD[\x81Q` a\x07]a\x04]\x83a\x07\x18V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x07\x7FW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x07\x9BW\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x07\x84V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x07\xB8W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xCFW`\0\x80\xFD[a\x06\xAC\x84\x82\x85\x01a\x07<V[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xEEW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x06W`\0\x80\xFD[a\x08\x12\x86\x83\x87\x01a\x07<V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\x08(W`\0\x80\xFD[Pa\x07\x0E\x85\x82\x86\x01a\x07<V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x08fW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x08JV[P\x94\x95\x94PPPPPV[``\x81R`\0a\x08\x84``\x83\x01\x86a\x085V[\x82\x81\x03` \x84\x01Ra\x08\x96\x81\x86a\x085V[\x90P\x82\x81\x03`@\x84\x01Ra\x08\xAA\x81\x85a\x085V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x08\xC6W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x01eW`\0\x80\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x08\xE9W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\0W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\t\x11W`\0\x80\xFD[\x805a\t\x1Fa\x04]\x82a\x07\x18V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\t>W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x03vW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\tCV\xFE\xA2dipfsX\"\x12 4I\xD3\x9A\xDD/\x19\xA2\xFC\0\x12\xEE\x8A\xB1\x0BLy;Z\xFAV\x19\xE1\xB5b3\xB5\x94\xB6\xE9\xEA{dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static ZKSYNC_VERIFIER_WRAPPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct zksync_verifier_wrapper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for zksync_verifier_wrapper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for zksync_verifier_wrapper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for zksync_verifier_wrapper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for zksync_verifier_wrapper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(zksync_verifier_wrapper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> zksync_verifier_wrapper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ZKSYNC_VERIFIER_WRAPPER_ABI.clone(),
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
                ZKSYNC_VERIFIER_WRAPPER_ABI.clone(),
                ZKSYNC_VERIFIER_WRAPPER_BYTECODE.clone().into(),
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
        ///Calls the contract's `iverifier` (0xe7f5b81d) function
        pub fn iverifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([231, 245, 184, 29], ())
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
        for zksync_verifier_wrapper<M>
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
    pub enum zksync_verifier_wrapperCalls {
        CheckSampleInputsAndProof(CheckSampleInputsAndProofCall),
        Iverifier(IverifierCall),
        SampleInput(SampleInputCall),
        SampleProof(SampleProofCall),
        Verify(VerifyCall),
        VerifyAgainstSampleInputs(VerifyAgainstSampleInputsCall),
        VerifyInputs(VerifyInputsCall),
    }
    impl ::ethers::core::abi::AbiDecode for zksync_verifier_wrapperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CheckSampleInputsAndProofCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSampleInputsAndProof(decoded));
            }
            if let Ok(decoded) = <IverifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Iverifier(decoded));
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
            if let Ok(decoded) = <VerifyInputsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyInputs(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for zksync_verifier_wrapperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckSampleInputsAndProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Iverifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SampleInput(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SampleProof(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyAgainstSampleInputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyInputs(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for zksync_verifier_wrapperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckSampleInputsAndProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::Iverifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::SampleInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::SampleProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyAgainstSampleInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyInputs(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckSampleInputsAndProofCall> for zksync_verifier_wrapperCalls {
        fn from(value: CheckSampleInputsAndProofCall) -> Self {
            Self::CheckSampleInputsAndProof(value)
        }
    }
    impl ::core::convert::From<IverifierCall> for zksync_verifier_wrapperCalls {
        fn from(value: IverifierCall) -> Self {
            Self::Iverifier(value)
        }
    }
    impl ::core::convert::From<SampleInputCall> for zksync_verifier_wrapperCalls {
        fn from(value: SampleInputCall) -> Self {
            Self::SampleInput(value)
        }
    }
    impl ::core::convert::From<SampleProofCall> for zksync_verifier_wrapperCalls {
        fn from(value: SampleProofCall) -> Self {
            Self::SampleProof(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for zksync_verifier_wrapperCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifyAgainstSampleInputsCall> for zksync_verifier_wrapperCalls {
        fn from(value: VerifyAgainstSampleInputsCall) -> Self {
            Self::VerifyAgainstSampleInputs(value)
        }
    }
    impl ::core::convert::From<VerifyInputsCall> for zksync_verifier_wrapperCalls {
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
