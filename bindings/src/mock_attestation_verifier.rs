pub use mock_attestation_verifier::*;
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
pub mod mock_attestation_verifier {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("verify"),
                ::std::vec![
                    ::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verify"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },
                    ::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verify"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("attestation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("enclaveKey"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },
                ],
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKATTESTATIONVERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\xA4\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x8Ev\n\xFE\x14a\0FW\x80c\xADHq9\x14a\0YW\x80c\xEA\xC7\x08\xA3\x14a\0oW[`\0\x80\xFD[a\0Wa\0T6`\x04a\x01MV[PV[\0[a\0Wa\0g6`\x04a\x01\x8AV[PPPPPPV[a\0Wa\0}6`\x04a\x02dV[PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\0\xBAWa\0\xBAa\0\x81V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\0\xD1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xECWa\0\xECa\0\x81V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x01\x14Wa\x01\x14a\0\x81V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x01-W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x01_W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01vW`\0\x80\xFD[a\x01\x82\x84\x82\x85\x01a\0\xC0V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x01\xA3W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xBBW`\0\x80\xFD[a\x01\xC7\x8A\x83\x8B\x01a\0\xC0V[\x97P` \x89\x015\x91P\x80\x82\x11\x15a\x01\xDDW`\0\x80\xFD[a\x01\xE9\x8A\x83\x8B\x01a\0\xC0V[\x96P`@\x89\x015\x91P\x80\x82\x11\x15a\x01\xFFW`\0\x80\xFD[a\x02\x0B\x8A\x83\x8B\x01a\0\xC0V[\x95P``\x89\x015\x91P\x80\x82\x11\x15a\x02!W`\0\x80\xFD[a\x02-\x8A\x83\x8B\x01a\0\xC0V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15a\x02CW`\0\x80\xFD[Pa\x02P\x89\x82\x8A\x01a\0\xC0V[\x92PP`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15a\x02wW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x8FW`\0\x80\xFD[a\x02\x9B\x86\x83\x87\x01a\0\xC0V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x02\xB1W`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\x02\xC5W`\0\x80\xFD[a\x02\xCDa\0\x97V[\x825\x82\x81\x11\x15a\x02\xDCW`\0\x80\xFD[a\x02\xE8\x88\x82\x86\x01a\0\xC0V[\x82RP` \x83\x015\x82\x81\x11\x15a\x02\xFDW`\0\x80\xFD[a\x03\t\x88\x82\x86\x01a\0\xC0V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x03!W`\0\x80\xFD[a\x03-\x88\x82\x86\x01a\0\xC0V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\x03EW`\0\x80\xFD[a\x03Q\x88\x82\x86\x01a\0\xC0V[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 pDu\xB9\x8A2\xAD\x0Bu\x98\xF8$9v\xEETz\x9B]\xD4+\x007\xBDa\xDC3\x05\xDF\xFD\xD7\x03dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static MOCKATTESTATIONVERIFIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x8Ev\n\xFE\x14a\0FW\x80c\xADHq9\x14a\0YW\x80c\xEA\xC7\x08\xA3\x14a\0oW[`\0\x80\xFD[a\0Wa\0T6`\x04a\x01MV[PV[\0[a\0Wa\0g6`\x04a\x01\x8AV[PPPPPPV[a\0Wa\0}6`\x04a\x02dV[PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\0\xBAWa\0\xBAa\0\x81V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\0\xD1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xECWa\0\xECa\0\x81V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x01\x14Wa\x01\x14a\0\x81V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x01-W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x01_W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01vW`\0\x80\xFD[a\x01\x82\x84\x82\x85\x01a\0\xC0V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x01\xA3W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xBBW`\0\x80\xFD[a\x01\xC7\x8A\x83\x8B\x01a\0\xC0V[\x97P` \x89\x015\x91P\x80\x82\x11\x15a\x01\xDDW`\0\x80\xFD[a\x01\xE9\x8A\x83\x8B\x01a\0\xC0V[\x96P`@\x89\x015\x91P\x80\x82\x11\x15a\x01\xFFW`\0\x80\xFD[a\x02\x0B\x8A\x83\x8B\x01a\0\xC0V[\x95P``\x89\x015\x91P\x80\x82\x11\x15a\x02!W`\0\x80\xFD[a\x02-\x8A\x83\x8B\x01a\0\xC0V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15a\x02CW`\0\x80\xFD[Pa\x02P\x89\x82\x8A\x01a\0\xC0V[\x92PP`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15a\x02wW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x8FW`\0\x80\xFD[a\x02\x9B\x86\x83\x87\x01a\0\xC0V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x02\xB1W`\0\x80\xFD[\x90\x84\x01\x90`\xA0\x82\x87\x03\x12\x15a\x02\xC5W`\0\x80\xFD[a\x02\xCDa\0\x97V[\x825\x82\x81\x11\x15a\x02\xDCW`\0\x80\xFD[a\x02\xE8\x88\x82\x86\x01a\0\xC0V[\x82RP` \x83\x015\x82\x81\x11\x15a\x02\xFDW`\0\x80\xFD[a\x03\t\x88\x82\x86\x01a\0\xC0V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x03!W`\0\x80\xFD[a\x03-\x88\x82\x86\x01a\0\xC0V[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a\x03EW`\0\x80\xFD[a\x03Q\x88\x82\x86\x01a\0\xC0V[``\x83\x01RP`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 pDu\xB9\x8A2\xAD\x0Bu\x98\xF8$9v\xEETz\x9B]\xD4+\x007\xBDa\xDC3\x05\xDF\xFD\xD7\x03dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKATTESTATIONVERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockAttestationVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockAttestationVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockAttestationVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockAttestationVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockAttestationVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockAttestationVerifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockAttestationVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKATTESTATIONVERIFIER_ABI.clone(),
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
                MOCKATTESTATIONVERIFIER_ABI.clone(),
                MOCKATTESTATIONVERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `verify` (0x8e760afe) function
        pub fn verify(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 118, 10, 254], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0xad487139) function
        pub fn verify_with_attestation(
            &self,
            attestation: ::ethers::core::types::Bytes,
            enclave_key: ::ethers::core::types::Bytes,
            pcr0: ::ethers::core::types::Bytes,
            pcr1: ::ethers::core::types::Bytes,
            pcr2: ::ethers::core::types::Bytes,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [173, 72, 113, 57],
                    (attestation, enclave_key, pcr0, pcr1, pcr2, timestamp),
                )
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for MockAttestationVerifier<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    pub struct VerifyCall(pub ::ethers::core::types::Bytes);
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes,bytes,bytes,bytes,bytes,uint256)` and selector `0xad487139`
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
    #[ethcall(name = "verify", abi = "verify(bytes,bytes,bytes,bytes,bytes,uint256)")]
    pub struct VerifyWithAttestationCall {
        pub attestation: ::ethers::core::types::Bytes,
        pub enclave_key: ::ethers::core::types::Bytes,
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
        pub timestamp: ::ethers::core::types::U256,
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
    pub enum MockAttestationVerifierCalls {
        Verify(VerifyCall),
        VerifyWithAttestation(VerifyWithAttestationCall),
        VerifyWithSignature(VerifyWithSignatureCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockAttestationVerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verify(decoded));
            }
            if let Ok(decoded) =
                <VerifyWithAttestationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyWithAttestation(decoded));
            }
            if let Ok(decoded) =
                <VerifyWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyWithSignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockAttestationVerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyWithAttestation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockAttestationVerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyWithAttestation(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyWithSignature(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<VerifyCall> for MockAttestationVerifierCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifyWithAttestationCall> for MockAttestationVerifierCalls {
        fn from(value: VerifyWithAttestationCall) -> Self {
            Self::VerifyWithAttestation(value)
        }
    }
    impl ::core::convert::From<VerifyWithSignatureCall> for MockAttestationVerifierCalls {
        fn from(value: VerifyWithSignatureCall) -> Self {
            Self::VerifyWithSignature(value)
        }
    }
}
