pub use priority_log::*;
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
pub mod priority_log {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("priorityStore"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("priorityStore"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum PriorityLog.Priority",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPriority"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPriority"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("priority"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum PriorityLog.Priority",),
                            ),
                        },],
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
    pub static PRIORITYLOG_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x01\x83\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x99\x1A4\xE7\x14a\0;W\x80c\xDB&6\xF6\x14a\0tW[`\0\x80\xFD[a\0^a\0I6`\x04a\0\xBEV[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\0k\x91\x90a\x01\x04V[`@Q\x80\x91\x03\x90\xF3[a\0\x87a\0\x826`\x04a\x01,V[a\0\x89V[\0[3`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x83\x91\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\0\xB5Wa\0\xB5a\0\xEEV[\x02\x17\x90UPPPV[`\0` \x82\x84\x03\x12\x15a\0\xD0W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xE7W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x04\x83\x10a\x01&WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a\x01>W`\0\x80\xFD[\x815`\x04\x81\x10a\0\xE7W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xF4\xAA\xF9\xC6\xE9r<\x7F\xF7?\xDE\xBC\xFD\xC4\xE3\x9F\xB9\x9D\xFCH\xD1k\xCA2\xD7\x93\xBB\x04\x1F\xE7\x1F\xFCdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static PRIORITYLOG_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x99\x1A4\xE7\x14a\0;W\x80c\xDB&6\xF6\x14a\0tW[`\0\x80\xFD[a\0^a\0I6`\x04a\0\xBEV[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\0k\x91\x90a\x01\x04V[`@Q\x80\x91\x03\x90\xF3[a\0\x87a\0\x826`\x04a\x01,V[a\0\x89V[\0[3`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x83\x91\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\0\xB5Wa\0\xB5a\0\xEEV[\x02\x17\x90UPPPV[`\0` \x82\x84\x03\x12\x15a\0\xD0W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xE7W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x04\x83\x10a\x01&WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a\x01>W`\0\x80\xFD[\x815`\x04\x81\x10a\0\xE7W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xF4\xAA\xF9\xC6\xE9r<\x7F\xF7?\xDE\xBC\xFD\xC4\xE3\x9F\xB9\x9D\xFCH\xD1k\xCA2\xD7\x93\xBB\x04\x1F\xE7\x1F\xFCdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static PRIORITYLOG_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PriorityLog<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PriorityLog<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PriorityLog<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PriorityLog<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PriorityLog<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PriorityLog))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PriorityLog<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PRIORITYLOG_ABI.clone(),
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
                PRIORITYLOG_ABI.clone(),
                PRIORITYLOG_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `priorityStore` (0x991a34e7) function
        pub fn priority_store(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([153, 26, 52, 231], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPriority` (0xdb2636f6) function
        pub fn set_priority(
            &self,
            priority: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 38, 54, 246], priority)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for PriorityLog<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `priorityStore` function with signature `priorityStore(address)` and selector `0x991a34e7`
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
    #[ethcall(name = "priorityStore", abi = "priorityStore(address)")]
    pub struct PriorityStoreCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `setPriority` function with signature `setPriority(uint8)` and selector `0xdb2636f6`
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
    #[ethcall(name = "setPriority", abi = "setPriority(uint8)")]
    pub struct SetPriorityCall {
        pub priority: u8,
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
    pub enum PriorityLogCalls {
        PriorityStore(PriorityStoreCall),
        SetPriority(SetPriorityCall),
    }
    impl ::ethers::core::abi::AbiDecode for PriorityLogCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <PriorityStoreCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PriorityStore(decoded));
            }
            if let Ok(decoded) = <SetPriorityCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPriority(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PriorityLogCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::PriorityStore(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPriority(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PriorityLogCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PriorityStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPriority(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PriorityStoreCall> for PriorityLogCalls {
        fn from(value: PriorityStoreCall) -> Self {
            Self::PriorityStore(value)
        }
    }
    impl ::core::convert::From<SetPriorityCall> for PriorityLogCalls {
        fn from(value: SetPriorityCall) -> Self {
            Self::SetPriority(value)
        }
    }
    ///Container type for all return fields from the `priorityStore` function with signature `priorityStore(address)` and selector `0x991a34e7`
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
    pub struct PriorityStoreReturn(pub u8);
}
