pub use dispute::*;
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
pub mod dispute {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_er"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "contract EntityKeyRegistry",
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ENTITY_KEY_REGISTRY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ENTITY_KEY_REGISTRY",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract EntityKeyRegistry",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkDispute"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkDispute"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("askId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proverData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("invalidProofSignature",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expectedFamilyId"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CannotBeZero"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CannotBeZero"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DISPUTE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x06(8\x03\x80a\x06(\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x05\x97a\0\x91`\09`\0\x81\x81`@\x01Ra\x01\xD9\x01Ra\x05\x97`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cf\x1D\xE5\xAC\x14a\0;W\x80cs\x0F\xEC)\x14a\0\x7FW[`\0\x80\xFD[a\0b\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x92a\0\x8D6`\x04a\x04\x93V[a\0\xA2V[`@Q\x90\x15\x15\x81R` \x01a\0vV[`\0a\0\xE8\x87\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa\0\xF3\x91PPV[\x97\x96PPPPPPPV[`\0\x80\x86\x86\x86`@Q` \x01a\x01\x0B\x93\x92\x91\x90a\x05\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x01|\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90P`\0a\x01\x8A\x82\x87a\x02BV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01\xB3W`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcr\x10Z\xAF`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cr\x10Z\xAF\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02\x1BW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x02/W=`\0\x80>=`\0\xFD[P`\x01\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80a\x02R\x86\x86a\x02lV[\x92P\x92P\x92Pa\x02b\x82\x82a\x02\xB9V[P\x90\x94\x93PPPPV[`\0\x80`\0\x83Q`A\x03a\x02\xA6W` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa\x02\x98\x88\x82\x85\x85a\x03{V[\x95P\x95P\x95PPPPa\x02\xB2V[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a\x02\xCDWa\x02\xCDa\x05KV[\x03a\x02\xD6WPPV[`\x01\x82`\x03\x81\x11\x15a\x02\xEAWa\x02\xEAa\x05KV[\x03a\x03\x08W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x03\x1CWa\x03\x1Ca\x05KV[\x03a\x03BW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\x03\x82`\x03\x81\x11\x15a\x03VWa\x03Va\x05KV[\x03a\x03wW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x039V[PPV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x03\xB6WP`\0\x91P`\x03\x90P\x82a\x04@V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x04\nW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x046WP`\0\x92P`\x01\x91P\x82\x90Pa\x04@V[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x04\\W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04tW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x04\x8CW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x04\xACW`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xCBW`\0\x80\xFD[a\x04\xD7\x8A\x83\x8B\x01a\x04JV[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15a\x04\xF0W`\0\x80\xFD[Pa\x04\xFD\x89\x82\x8A\x01a\x04JV[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xF6\xDF\xA4\x80\xB8\xDD^\x8B/\xD4T\x08m\x1C\x90\xF6\x93\x12\xC4\xB3\xBF'\xD4\xB2\xB9\xBEo\xBD|\x1B=rdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static DISPUTE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cf\x1D\xE5\xAC\x14a\0;W\x80cs\x0F\xEC)\x14a\0\x7FW[`\0\x80\xFD[a\0b\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x92a\0\x8D6`\x04a\x04\x93V[a\0\xA2V[`@Q\x90\x15\x15\x81R` \x01a\0vV[`\0a\0\xE8\x87\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa\0\xF3\x91PPV[\x97\x96PPPPPPPV[`\0\x80\x86\x86\x86`@Q` \x01a\x01\x0B\x93\x92\x91\x90a\x05\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x01|\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90P`\0a\x01\x8A\x82\x87a\x02BV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01\xB3W`@Qc\x1E\x1D\n\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcr\x10Z\xAF`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cr\x10Z\xAF\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02\x1BW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x02/W=`\0\x80>=`\0\xFD[P`\x01\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80a\x02R\x86\x86a\x02lV[\x92P\x92P\x92Pa\x02b\x82\x82a\x02\xB9V[P\x90\x94\x93PPPPV[`\0\x80`\0\x83Q`A\x03a\x02\xA6W` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa\x02\x98\x88\x82\x85\x85a\x03{V[\x95P\x95P\x95PPPPa\x02\xB2V[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a\x02\xCDWa\x02\xCDa\x05KV[\x03a\x02\xD6WPPV[`\x01\x82`\x03\x81\x11\x15a\x02\xEAWa\x02\xEAa\x05KV[\x03a\x03\x08W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x03\x1CWa\x03\x1Ca\x05KV[\x03a\x03BW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\x03\x82`\x03\x81\x11\x15a\x03VWa\x03Va\x05KV[\x03a\x03wW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x039V[PPV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x03\xB6WP`\0\x91P`\x03\x90P\x82a\x04@V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x04\nW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x046WP`\0\x92P`\x01\x91P\x82\x90Pa\x04@V[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x04\\W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04tW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x04\x8CW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x04\xACW`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xCBW`\0\x80\xFD[a\x04\xD7\x8A\x83\x8B\x01a\x04JV[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15a\x04\xF0W`\0\x80\xFD[Pa\x04\xFD\x89\x82\x8A\x01a\x04JV[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[\x83\x81R`@` \x82\x01R\x81`@\x82\x01R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xF6\xDF\xA4\x80\xB8\xDD^\x8B/\xD4T\x08m\x1C\x90\xF6\x93\x12\xC4\xB3\xBF'\xD4\xB2\xB9\xBEo\xBD|\x1B=rdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static DISPUTE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Dispute<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Dispute<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Dispute<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Dispute<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Dispute<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Dispute))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Dispute<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                DISPUTE_ABI.clone(),
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
                DISPUTE_ABI.clone(),
                DISPUTE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ENTITY_KEY_REGISTRY` (0x661de5ac) function
        pub fn entity_key_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([102, 29, 229, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkDispute` (0x730fec29) function
        pub fn check_dispute(
            &self,
            ask_id: ::ethers::core::types::U256,
            prover_data: ::ethers::core::types::Bytes,
            invalid_proof_signature: ::ethers::core::types::Bytes,
            expected_family_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [115, 15, 236, 41],
                    (
                        ask_id,
                        prover_data,
                        invalid_proof_signature,
                        expected_family_id,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Dispute<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    pub enum DisputeErrors {
        CannotBeZero(CannotBeZero),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DisputeErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <CannotBeZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CannotBeZero(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DisputeErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CannotBeZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ECDSAInvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DisputeErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <CannotBeZero as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ECDSAInvalidSignature as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureLength as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureS as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DisputeErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CannotBeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignatureS(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DisputeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CannotBeZero> for DisputeErrors {
        fn from(value: CannotBeZero) -> Self {
            Self::CannotBeZero(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for DisputeErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength> for DisputeErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for DisputeErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    ///Container type for all input parameters for the `ENTITY_KEY_REGISTRY` function with signature `ENTITY_KEY_REGISTRY()` and selector `0x661de5ac`
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
    #[ethcall(name = "ENTITY_KEY_REGISTRY", abi = "ENTITY_KEY_REGISTRY()")]
    pub struct EntityKeyRegistryCall;
    ///Container type for all input parameters for the `checkDispute` function with signature `checkDispute(uint256,bytes,bytes,bytes32)` and selector `0x730fec29`
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
        name = "checkDispute",
        abi = "checkDispute(uint256,bytes,bytes,bytes32)"
    )]
    pub struct CheckDisputeCall {
        pub ask_id: ::ethers::core::types::U256,
        pub prover_data: ::ethers::core::types::Bytes,
        pub invalid_proof_signature: ::ethers::core::types::Bytes,
        pub expected_family_id: [u8; 32],
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
    pub enum DisputeCalls {
        EntityKeyRegistry(EntityKeyRegistryCall),
        CheckDispute(CheckDisputeCall),
    }
    impl ::ethers::core::abi::AbiDecode for DisputeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <EntityKeyRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EntityKeyRegistry(decoded));
            }
            if let Ok(decoded) = <CheckDisputeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckDispute(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DisputeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EntityKeyRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckDispute(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DisputeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EntityKeyRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckDispute(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EntityKeyRegistryCall> for DisputeCalls {
        fn from(value: EntityKeyRegistryCall) -> Self {
            Self::EntityKeyRegistry(value)
        }
    }
    impl ::core::convert::From<CheckDisputeCall> for DisputeCalls {
        fn from(value: CheckDisputeCall) -> Self {
            Self::CheckDispute(value)
        }
    }
    ///Container type for all return fields from the `ENTITY_KEY_REGISTRY` function with signature `ENTITY_KEY_REGISTRY()` and selector `0x661de5ac`
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
    pub struct EntityKeyRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `checkDispute` function with signature `checkDispute(uint256,bytes,bytes,bytes32)` and selector `0x730fec29`
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
    pub struct CheckDisputeReturn(pub bool);
}
