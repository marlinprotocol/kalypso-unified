pub use i_reward_distributor::*;
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
pub mod i_reward_distributor {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("onClaimReward"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onClaimReward"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onSlash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onSlash"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onStakeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onStakeUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setStakeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setStakeToken"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakingPool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_isSupported"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateFeeReward"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateFeeReward"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_rewardAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateInflationReward"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateInflationReward",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_rewardAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
    pub static IREWARDDISTRIBUTOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IRewardDistributor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IRewardDistributor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IRewardDistributor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IRewardDistributor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IRewardDistributor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IRewardDistributor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IRewardDistributor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IREWARDDISTRIBUTOR_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `onClaimReward` (0x73bc3503) function
        pub fn on_claim_reward(
            &self,
            account: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 188, 53, 3], (account, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onSlash` (0xc4a7385d) function
        pub fn on_slash(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 167, 56, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onStakeUpdate` (0x8a28089e) function
        pub fn on_stake_update(
            &self,
            account: ::ethers::core::types::Address,
            stake_token: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 40, 8, 158], (account, stake_token, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStakeToken` (0x038798a0) function
        pub fn set_stake_token(
            &self,
            staking_pool: ::ethers::core::types::Address,
            is_supported: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 135, 152, 160], (staking_pool, is_supported))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFeeReward` (0x2f9a96a0) function
        pub fn update_fee_reward(
            &self,
            stake_token: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
            reward_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 154, 150, 160], (stake_token, operator, reward_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateInflationReward` (0x8700e1a8) function
        pub fn update_inflation_reward(
            &self,
            operator: ::ethers::core::types::Address,
            reward_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 0, 225, 168], (operator, reward_amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IRewardDistributor<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `onClaimReward` function with signature `onClaimReward(address,address)` and selector `0x73bc3503`
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
    #[ethcall(name = "onClaimReward", abi = "onClaimReward(address,address)")]
    pub struct OnClaimRewardCall {
        pub account: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `onSlash` function with signature `onSlash()` and selector `0xc4a7385d`
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
    #[ethcall(name = "onSlash", abi = "onSlash()")]
    pub struct OnSlashCall;
    ///Container type for all input parameters for the `onStakeUpdate` function with signature `onStakeUpdate(address,address,address)` and selector `0x8a28089e`
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
    #[ethcall(name = "onStakeUpdate", abi = "onStakeUpdate(address,address,address)")]
    pub struct OnStakeUpdateCall {
        pub account: ::ethers::core::types::Address,
        pub stake_token: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setStakeToken` function with signature `setStakeToken(address,bool)` and selector `0x038798a0`
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
    #[ethcall(name = "setStakeToken", abi = "setStakeToken(address,bool)")]
    pub struct SetStakeTokenCall {
        pub staking_pool: ::ethers::core::types::Address,
        pub is_supported: bool,
    }
    ///Container type for all input parameters for the `updateFeeReward` function with signature `updateFeeReward(address,address,uint256)` and selector `0x2f9a96a0`
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
        name = "updateFeeReward",
        abi = "updateFeeReward(address,address,uint256)"
    )]
    pub struct UpdateFeeRewardCall {
        pub stake_token: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateInflationReward` function with signature `updateInflationReward(address,uint256)` and selector `0x8700e1a8`
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
        name = "updateInflationReward",
        abi = "updateInflationReward(address,uint256)"
    )]
    pub struct UpdateInflationRewardCall {
        pub operator: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
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
    pub enum IRewardDistributorCalls {
        OnClaimReward(OnClaimRewardCall),
        OnSlash(OnSlashCall),
        OnStakeUpdate(OnStakeUpdateCall),
        SetStakeToken(SetStakeTokenCall),
        UpdateFeeReward(UpdateFeeRewardCall),
        UpdateInflationReward(UpdateInflationRewardCall),
    }
    impl ::ethers::core::abi::AbiDecode for IRewardDistributorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <OnClaimRewardCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnClaimReward(decoded));
            }
            if let Ok(decoded) = <OnSlashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnSlash(decoded));
            }
            if let Ok(decoded) = <OnStakeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnStakeUpdate(decoded));
            }
            if let Ok(decoded) = <SetStakeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetStakeToken(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeRewardCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateFeeReward(decoded));
            }
            if let Ok(decoded) =
                <UpdateInflationRewardCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateInflationReward(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IRewardDistributorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::OnClaimReward(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnSlash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnStakeUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetStakeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateFeeReward(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateInflationReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IRewardDistributorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OnClaimReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnSlash(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnStakeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStakeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFeeReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateInflationReward(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OnClaimRewardCall> for IRewardDistributorCalls {
        fn from(value: OnClaimRewardCall) -> Self {
            Self::OnClaimReward(value)
        }
    }
    impl ::core::convert::From<OnSlashCall> for IRewardDistributorCalls {
        fn from(value: OnSlashCall) -> Self {
            Self::OnSlash(value)
        }
    }
    impl ::core::convert::From<OnStakeUpdateCall> for IRewardDistributorCalls {
        fn from(value: OnStakeUpdateCall) -> Self {
            Self::OnStakeUpdate(value)
        }
    }
    impl ::core::convert::From<SetStakeTokenCall> for IRewardDistributorCalls {
        fn from(value: SetStakeTokenCall) -> Self {
            Self::SetStakeToken(value)
        }
    }
    impl ::core::convert::From<UpdateFeeRewardCall> for IRewardDistributorCalls {
        fn from(value: UpdateFeeRewardCall) -> Self {
            Self::UpdateFeeReward(value)
        }
    }
    impl ::core::convert::From<UpdateInflationRewardCall> for IRewardDistributorCalls {
        fn from(value: UpdateInflationRewardCall) -> Self {
            Self::UpdateInflationReward(value)
        }
    }
}
