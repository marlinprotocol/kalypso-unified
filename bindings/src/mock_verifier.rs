pub use mock_verifier::*;
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
pub mod mock_verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKVERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\\\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x02\xF7}\x19\x14a\0gW\x80c\x10\xA5By\x14a\0\x90W\x80c}\x8A\xD4+\x14a\0\x97W\x80c\x8Ev\n\xFE\x14a\0\xACW\x80c\xA6\xDF\xBC\x7F\x14a\0\xACW\x80c\xA7l\x05Q\x14a\0\xC2W[`\0\x80\xFD[a\0{a\0u6`\x04a\x01{V[P`\x01\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01a\0{V[a\0\x9Fa\0\xCAV[`@Qa\0\x87\x91\x90a\x02,V[a\0{a\0\xBA6`\x04a\x02zV[`\x01\x92\x91PPV[a\0\x9Fa\x01XV[`\0\x80Ta\0\xD7\x90a\x02\xECV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\x03\x90a\x02\xECV[\x80\x15a\x01PW\x80`\x1F\x10a\x01%Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01PV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x013W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x01\x80Ta\0\xD7\x90a\x02\xECV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x01\x8DW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xA5W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x01\xB9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\xCBWa\x01\xCBa\x01eV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x01\xF3Wa\x01\xF3a\x01eV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x02\x0CW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x02YW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x02=V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80` \x83\x85\x03\x12\x15a\x02\x8DW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xA5W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02\xB9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\xC8W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x02\xDAW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x03\0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x03 WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xAC\xB4z\xB1\"O\xE9%\x1E\xBEj\x10\x0F\xC9X\x08\xDB\0\x0E\x9Bt+\"\xE8\x8Ej\xBD\x04#!\xA9\xBDdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static MOCKVERIFIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x02\xF7}\x19\x14a\0gW\x80c\x10\xA5By\x14a\0\x90W\x80c}\x8A\xD4+\x14a\0\x97W\x80c\x8Ev\n\xFE\x14a\0\xACW\x80c\xA6\xDF\xBC\x7F\x14a\0\xACW\x80c\xA7l\x05Q\x14a\0\xC2W[`\0\x80\xFD[a\0{a\0u6`\x04a\x01{V[P`\x01\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01a\0{V[a\0\x9Fa\0\xCAV[`@Qa\0\x87\x91\x90a\x02,V[a\0{a\0\xBA6`\x04a\x02zV[`\x01\x92\x91PPV[a\0\x9Fa\x01XV[`\0\x80Ta\0\xD7\x90a\x02\xECV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\x03\x90a\x02\xECV[\x80\x15a\x01PW\x80`\x1F\x10a\x01%Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01PV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x013W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x01\x80Ta\0\xD7\x90a\x02\xECV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x01\x8DW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xA5W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x01\xB9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\xCBWa\x01\xCBa\x01eV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x01\xF3Wa\x01\xF3a\x01eV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x02\x0CW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x02YW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x02=V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\0\x80` \x83\x85\x03\x12\x15a\x02\x8DW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xA5W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02\xB9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\xC8W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x02\xDAW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x03\0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x03 WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xAC\xB4z\xB1\"O\xE9%\x1E\xBEj\x10\x0F\xC9X\x08\xDB\0\x0E\x9Bt+\"\xE8\x8Ej\xBD\x04#!\xA9\xBDdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKVERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockVerifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKVERIFIER_ABI.clone(),
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
                MOCKVERIFIER_ABI.clone(),
                MOCKVERIFIER_BYTECODE.clone().into(),
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
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([142, 118, 10, 254], p0)
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
        ///Calls the contract's `verifyInputs` (0xa6dfbc7f) function
        pub fn verify_inputs(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 223, 188, 127], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockVerifier<M> {
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
    pub struct VerifyCall(pub ::ethers::core::types::Bytes);
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
    pub enum MockVerifierCalls {
        CheckSampleInputsAndProof(CheckSampleInputsAndProofCall),
        SampleInput(SampleInputCall),
        SampleProof(SampleProofCall),
        Verify(VerifyCall),
        VerifyAgainstSampleInputs(VerifyAgainstSampleInputsCall),
        VerifyInputs(VerifyInputsCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockVerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CheckSampleInputsAndProofCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSampleInputsAndProof(decoded));
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
    impl ::ethers::core::abi::AbiEncode for MockVerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckSampleInputsAndProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::core::fmt::Display for MockVerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckSampleInputsAndProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::SampleInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::SampleProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyAgainstSampleInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyInputs(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckSampleInputsAndProofCall> for MockVerifierCalls {
        fn from(value: CheckSampleInputsAndProofCall) -> Self {
            Self::CheckSampleInputsAndProof(value)
        }
    }
    impl ::core::convert::From<SampleInputCall> for MockVerifierCalls {
        fn from(value: SampleInputCall) -> Self {
            Self::SampleInput(value)
        }
    }
    impl ::core::convert::From<SampleProofCall> for MockVerifierCalls {
        fn from(value: SampleProofCall) -> Self {
            Self::SampleProof(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for MockVerifierCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifyAgainstSampleInputsCall> for MockVerifierCalls {
        fn from(value: VerifyAgainstSampleInputsCall) -> Self {
            Self::VerifyAgainstSampleInputs(value)
        }
    }
    impl ::core::convert::From<VerifyInputsCall> for MockVerifierCalls {
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
