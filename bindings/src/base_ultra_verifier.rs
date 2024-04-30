pub use base_ultra_verifier::*;
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
pub mod base_ultra_verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getVerificationKeyHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getVerificationKeyHash",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verify"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_proof"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_publicInputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32[]"),
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
                    ::std::borrow::ToOwned::to_owned("EC_SCALAR_MUL_FAILURE"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("EC_SCALAR_MUL_FAILURE",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MOD_EXP_FAILURE"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MOD_EXP_FAILURE"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PROOF_FAILURE"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PROOF_FAILURE"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PUBLIC_INPUT_COUNT_INVALID"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PUBLIC_INPUT_COUNT_INVALID",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expected"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("actual"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PUBLIC_INPUT_GE_P"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PUBLIC_INPUT_GE_P"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PUBLIC_INPUT_INVALID_BN128_G1_POINT"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "PUBLIC_INPUT_INVALID_BN128_G1_POINT",
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
    pub static BASEULTRAVERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct BaseUltraVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BaseUltraVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BaseUltraVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BaseUltraVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BaseUltraVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BaseUltraVerifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BaseUltraVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                BASEULTRAVERIFIER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `getVerificationKeyHash` (0x937f6a10) function
        pub fn get_verification_key_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([147, 127, 106, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0xea50d0e4) function
        pub fn verify(
            &self,
            proof: ::ethers::core::types::Bytes,
            public_inputs: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([234, 80, 208, 228], (proof, public_inputs))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for BaseUltraVerifier<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `EC_SCALAR_MUL_FAILURE` with signature `EC_SCALAR_MUL_FAILURE()` and selector `0xf755f369`
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
    #[etherror(name = "EC_SCALAR_MUL_FAILURE", abi = "EC_SCALAR_MUL_FAILURE()")]
    pub struct EC_SCALAR_MUL_FAILURE;
    ///Custom Error type `MOD_EXP_FAILURE` with signature `MOD_EXP_FAILURE()` and selector `0xf894a7bc`
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
    #[etherror(name = "MOD_EXP_FAILURE", abi = "MOD_EXP_FAILURE()")]
    pub struct MOD_EXP_FAILURE;
    ///Custom Error type `PROOF_FAILURE` with signature `PROOF_FAILURE()` and selector `0x0711fcec`
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
    #[etherror(name = "PROOF_FAILURE", abi = "PROOF_FAILURE()")]
    pub struct PROOF_FAILURE;
    ///Custom Error type `PUBLIC_INPUT_COUNT_INVALID` with signature `PUBLIC_INPUT_COUNT_INVALID(uint256,uint256)` and selector `0x7667dc9b`
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
        name = "PUBLIC_INPUT_COUNT_INVALID",
        abi = "PUBLIC_INPUT_COUNT_INVALID(uint256,uint256)"
    )]
    pub struct PUBLIC_INPUT_COUNT_INVALID {
        pub expected: ::ethers::core::types::U256,
        pub actual: ::ethers::core::types::U256,
    }
    ///Custom Error type `PUBLIC_INPUT_GE_P` with signature `PUBLIC_INPUT_GE_P()` and selector `0x374a972f`
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
    #[etherror(name = "PUBLIC_INPUT_GE_P", abi = "PUBLIC_INPUT_GE_P()")]
    pub struct PUBLIC_INPUT_GE_P;
    ///Custom Error type `PUBLIC_INPUT_INVALID_BN128_G1_POINT` with signature `PUBLIC_INPUT_INVALID_BN128_G1_POINT()` and selector `0xeba9f4a6`
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
        name = "PUBLIC_INPUT_INVALID_BN128_G1_POINT",
        abi = "PUBLIC_INPUT_INVALID_BN128_G1_POINT()"
    )]
    pub struct PUBLIC_INPUT_INVALID_BN128_G1_POINT;
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
    pub enum BaseUltraVerifierErrors {
        EC_SCALAR_MUL_FAILURE(EC_SCALAR_MUL_FAILURE),
        MOD_EXP_FAILURE(MOD_EXP_FAILURE),
        PROOF_FAILURE(PROOF_FAILURE),
        PUBLIC_INPUT_COUNT_INVALID(PUBLIC_INPUT_COUNT_INVALID),
        PUBLIC_INPUT_GE_P(PUBLIC_INPUT_GE_P),
        PUBLIC_INPUT_INVALID_BN128_G1_POINT(PUBLIC_INPUT_INVALID_BN128_G1_POINT),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BaseUltraVerifierErrors {
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
                <EC_SCALAR_MUL_FAILURE as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EC_SCALAR_MUL_FAILURE(decoded));
            }
            if let Ok(decoded) = <MOD_EXP_FAILURE as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MOD_EXP_FAILURE(decoded));
            }
            if let Ok(decoded) = <PROOF_FAILURE as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PROOF_FAILURE(decoded));
            }
            if let Ok(decoded) =
                <PUBLIC_INPUT_COUNT_INVALID as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PUBLIC_INPUT_COUNT_INVALID(decoded));
            }
            if let Ok(decoded) = <PUBLIC_INPUT_GE_P as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PUBLIC_INPUT_GE_P(decoded));
            }
            if let Ok(decoded) =
                <PUBLIC_INPUT_INVALID_BN128_G1_POINT as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PUBLIC_INPUT_INVALID_BN128_G1_POINT(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BaseUltraVerifierErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::EC_SCALAR_MUL_FAILURE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MOD_EXP_FAILURE(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PROOF_FAILURE(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PUBLIC_INPUT_COUNT_INVALID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PUBLIC_INPUT_GE_P(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PUBLIC_INPUT_INVALID_BN128_G1_POINT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BaseUltraVerifierErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <EC_SCALAR_MUL_FAILURE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MOD_EXP_FAILURE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PROOF_FAILURE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PUBLIC_INPUT_COUNT_INVALID as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PUBLIC_INPUT_GE_P as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PUBLIC_INPUT_INVALID_BN128_G1_POINT as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for BaseUltraVerifierErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EC_SCALAR_MUL_FAILURE(element) => ::core::fmt::Display::fmt(element, f),
                Self::MOD_EXP_FAILURE(element) => ::core::fmt::Display::fmt(element, f),
                Self::PROOF_FAILURE(element) => ::core::fmt::Display::fmt(element, f),
                Self::PUBLIC_INPUT_COUNT_INVALID(element) => ::core::fmt::Display::fmt(element, f),
                Self::PUBLIC_INPUT_GE_P(element) => ::core::fmt::Display::fmt(element, f),
                Self::PUBLIC_INPUT_INVALID_BN128_G1_POINT(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for BaseUltraVerifierErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<EC_SCALAR_MUL_FAILURE> for BaseUltraVerifierErrors {
        fn from(value: EC_SCALAR_MUL_FAILURE) -> Self {
            Self::EC_SCALAR_MUL_FAILURE(value)
        }
    }
    impl ::core::convert::From<MOD_EXP_FAILURE> for BaseUltraVerifierErrors {
        fn from(value: MOD_EXP_FAILURE) -> Self {
            Self::MOD_EXP_FAILURE(value)
        }
    }
    impl ::core::convert::From<PROOF_FAILURE> for BaseUltraVerifierErrors {
        fn from(value: PROOF_FAILURE) -> Self {
            Self::PROOF_FAILURE(value)
        }
    }
    impl ::core::convert::From<PUBLIC_INPUT_COUNT_INVALID> for BaseUltraVerifierErrors {
        fn from(value: PUBLIC_INPUT_COUNT_INVALID) -> Self {
            Self::PUBLIC_INPUT_COUNT_INVALID(value)
        }
    }
    impl ::core::convert::From<PUBLIC_INPUT_GE_P> for BaseUltraVerifierErrors {
        fn from(value: PUBLIC_INPUT_GE_P) -> Self {
            Self::PUBLIC_INPUT_GE_P(value)
        }
    }
    impl ::core::convert::From<PUBLIC_INPUT_INVALID_BN128_G1_POINT> for BaseUltraVerifierErrors {
        fn from(value: PUBLIC_INPUT_INVALID_BN128_G1_POINT) -> Self {
            Self::PUBLIC_INPUT_INVALID_BN128_G1_POINT(value)
        }
    }
    ///Container type for all input parameters for the `getVerificationKeyHash` function with signature `getVerificationKeyHash()` and selector `0x937f6a10`
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
    #[ethcall(name = "getVerificationKeyHash", abi = "getVerificationKeyHash()")]
    pub struct GetVerificationKeyHashCall;
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes,bytes32[])` and selector `0xea50d0e4`
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
    #[ethcall(name = "verify", abi = "verify(bytes,bytes32[])")]
    pub struct VerifyCall {
        pub proof: ::ethers::core::types::Bytes,
        pub public_inputs: ::std::vec::Vec<[u8; 32]>,
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
    pub enum BaseUltraVerifierCalls {
        GetVerificationKeyHash(GetVerificationKeyHashCall),
        Verify(VerifyCall),
    }
    impl ::ethers::core::abi::AbiDecode for BaseUltraVerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <GetVerificationKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetVerificationKeyHash(decoded));
            }
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verify(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BaseUltraVerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetVerificationKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BaseUltraVerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetVerificationKeyHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetVerificationKeyHashCall> for BaseUltraVerifierCalls {
        fn from(value: GetVerificationKeyHashCall) -> Self {
            Self::GetVerificationKeyHash(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for BaseUltraVerifierCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    ///Container type for all return fields from the `getVerificationKeyHash` function with signature `getVerificationKeyHash()` and selector `0x937f6a10`
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
    pub struct GetVerificationKeyHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `verify` function with signature `verify(bytes,bytes32[])` and selector `0xea50d0e4`
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
}
