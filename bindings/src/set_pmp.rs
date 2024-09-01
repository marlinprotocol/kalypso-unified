pub use set_pmp::*;
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
pub mod set_pmp {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SETPMP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct SetPmp<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SetPmp<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SetPmp<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SetPmp<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SetPmp<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SetPmp))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SetPmp<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SETPMP_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `proofMarketplace` (0x81c45c70) function
        pub fn proof_marketplace(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([129, 196, 92, 112], ())
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for SetPmp<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    pub enum SetPmpCalls {
        ProofMarketplace(ProofMarketplaceCall),
        SetProofMarketplaceContract(SetProofMarketplaceContractCall),
    }
    impl ::ethers::core::abi::AbiDecode for SetPmpCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ProofMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProofMarketplace(decoded));
            }
            if let Ok(decoded) =
                <SetProofMarketplaceContractCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProofMarketplaceContract(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SetPmpCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ProofMarketplace(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProofMarketplaceContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SetPmpCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ProofMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProofMarketplaceContract(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ProofMarketplaceCall> for SetPmpCalls {
        fn from(value: ProofMarketplaceCall) -> Self {
            Self::ProofMarketplace(value)
        }
    }
    impl ::core::convert::From<SetProofMarketplaceContractCall> for SetPmpCalls {
        fn from(value: SetProofMarketplaceContractCall) -> Self {
            Self::SetProofMarketplaceContract(value)
        }
    }
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
}
