pub use i_symbiotic_staking_reward::*;
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
pub mod i_symbiotic_staking_reward {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("claimReward"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("claimReward"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onSnapshotSubmission"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onSnapshotSubmission",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_vault"),
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
                    ::std::borrow::ToOwned::to_owned("rewardAccrued"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rewardAccrued"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_rewardToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_vault"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("rewardPerTokenPaid"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rewardPerTokenPaid"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_rewardToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_vault"),
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
                    ::std::borrow::ToOwned::to_owned("rewardPerTokenStored"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rewardPerTokenStored",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_rewardToken"),
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
                                name: ::std::borrow::ToOwned::to_owned("_amount"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FeeRewardTokenSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FeeRewardTokenSet"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("feeRewardToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("JobManagerSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("JobManagerSet"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("jobManager"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardAccrued"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RewardAccrued"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("rewardToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("vault"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardClaimed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RewardClaimed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardDistributed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RewardDistributed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardPerTokenUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RewardPerTokenUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stakeToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("rewardToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "rewardPerTokenStoredUpdated",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("rewardPerTokenAdded",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakingPoolSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakingPoolSet"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("stakingPool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SymbioticStakingSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SymbioticStakingSet",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("symbioticStaking"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ISYMBIOTICSTAKINGREWARD_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct ISymbioticStakingReward<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISymbioticStakingReward<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISymbioticStakingReward<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISymbioticStakingReward<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISymbioticStakingReward<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ISymbioticStakingReward))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISymbioticStakingReward<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ISYMBIOTICSTAKINGREWARD_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `claimReward` (0xd279c191) function
        pub fn claim_reward(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 121, 193, 145], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onSnapshotSubmission` (0x45a3fb1e) function
        pub fn on_snapshot_submission(
            &self,
            vault: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 163, 251, 30], (vault, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardAccrued` (0x0bda5be1) function
        pub fn reward_accrued(
            &self,
            reward_token: ::ethers::core::types::Address,
            vault: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([11, 218, 91, 225], (reward_token, vault))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerTokenPaid` (0x4899b77a) function
        pub fn reward_per_token_paid(
            &self,
            stake_token: ::ethers::core::types::Address,
            reward_token: ::ethers::core::types::Address,
            vault: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [72, 153, 183, 122],
                    (stake_token, reward_token, vault, operator),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerTokenStored` (0x9a5b3430) function
        pub fn reward_per_token_stored(
            &self,
            stake_token: ::ethers::core::types::Address,
            reward_token: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 91, 52, 48], (stake_token, reward_token, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFeeReward` (0x2f9a96a0) function
        pub fn update_fee_reward(
            &self,
            stake_token: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 154, 150, 160], (stake_token, operator, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FeeRewardTokenSet` event
        pub fn fee_reward_token_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeRewardTokenSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `JobManagerSet` event
        pub fn job_manager_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, JobManagerSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RewardAccrued` event
        pub fn reward_accrued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RewardAccruedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RewardClaimed` event
        pub fn reward_claimed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RewardClaimedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RewardDistributed` event
        pub fn reward_distributed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RewardDistributedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RewardPerTokenUpdated` event
        pub fn reward_per_token_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RewardPerTokenUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakingPoolSet` event
        pub fn staking_pool_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakingPoolSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SymbioticStakingSet` event
        pub fn symbiotic_staking_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SymbioticStakingSetFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ISymbioticStakingRewardEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ISymbioticStakingReward<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "FeeRewardTokenSet", abi = "FeeRewardTokenSet(address)")]
    pub struct FeeRewardTokenSetFilter {
        #[ethevent(indexed)]
        pub fee_reward_token: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "JobManagerSet", abi = "JobManagerSet(address)")]
    pub struct JobManagerSetFilter {
        #[ethevent(indexed)]
        pub job_manager: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "RewardAccrued", abi = "RewardAccrued(address,address,uint256)")]
    pub struct RewardAccruedFilter {
        #[ethevent(indexed)]
        pub reward_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub vault: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "RewardClaimed", abi = "RewardClaimed(address,uint256)")]
    pub struct RewardClaimedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "RewardDistributed",
        abi = "RewardDistributed(address,address,uint256)"
    )]
    pub struct RewardDistributedFilter {
        #[ethevent(indexed)]
        pub stake_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "RewardPerTokenUpdated",
        abi = "RewardPerTokenUpdated(address,address,address,uint256,uint256)"
    )]
    pub struct RewardPerTokenUpdatedFilter {
        #[ethevent(indexed)]
        pub stake_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub reward_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub reward_per_token_stored_updated: ::ethers::core::types::U256,
        pub reward_per_token_added: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "StakingPoolSet", abi = "StakingPoolSet(address)")]
    pub struct StakingPoolSetFilter {
        #[ethevent(indexed)]
        pub staking_pool: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SymbioticStakingSet", abi = "SymbioticStakingSet(address)")]
    pub struct SymbioticStakingSetFilter {
        #[ethevent(indexed)]
        pub symbiotic_staking: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
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
    pub enum ISymbioticStakingRewardEvents {
        FeeRewardTokenSetFilter(FeeRewardTokenSetFilter),
        JobManagerSetFilter(JobManagerSetFilter),
        RewardAccruedFilter(RewardAccruedFilter),
        RewardClaimedFilter(RewardClaimedFilter),
        RewardDistributedFilter(RewardDistributedFilter),
        RewardPerTokenUpdatedFilter(RewardPerTokenUpdatedFilter),
        StakingPoolSetFilter(StakingPoolSetFilter),
        SymbioticStakingSetFilter(SymbioticStakingSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for ISymbioticStakingRewardEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FeeRewardTokenSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingRewardEvents::FeeRewardTokenSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = JobManagerSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingRewardEvents::JobManagerSetFilter(decoded));
            }
            if let Ok(decoded) = RewardAccruedFilter::decode_log(log) {
                return Ok(ISymbioticStakingRewardEvents::RewardAccruedFilter(decoded));
            }
            if let Ok(decoded) = RewardClaimedFilter::decode_log(log) {
                return Ok(ISymbioticStakingRewardEvents::RewardClaimedFilter(decoded));
            }
            if let Ok(decoded) = RewardDistributedFilter::decode_log(log) {
                return Ok(ISymbioticStakingRewardEvents::RewardDistributedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RewardPerTokenUpdatedFilter::decode_log(log) {
                return Ok(ISymbioticStakingRewardEvents::RewardPerTokenUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StakingPoolSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingRewardEvents::StakingPoolSetFilter(decoded));
            }
            if let Ok(decoded) = SymbioticStakingSetFilter::decode_log(log) {
                return Ok(ISymbioticStakingRewardEvents::SymbioticStakingSetFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ISymbioticStakingRewardEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FeeRewardTokenSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::JobManagerSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardAccruedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardClaimedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardDistributedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardPerTokenUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakingPoolSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SymbioticStakingSetFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FeeRewardTokenSetFilter> for ISymbioticStakingRewardEvents {
        fn from(value: FeeRewardTokenSetFilter) -> Self {
            Self::FeeRewardTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<JobManagerSetFilter> for ISymbioticStakingRewardEvents {
        fn from(value: JobManagerSetFilter) -> Self {
            Self::JobManagerSetFilter(value)
        }
    }
    impl ::core::convert::From<RewardAccruedFilter> for ISymbioticStakingRewardEvents {
        fn from(value: RewardAccruedFilter) -> Self {
            Self::RewardAccruedFilter(value)
        }
    }
    impl ::core::convert::From<RewardClaimedFilter> for ISymbioticStakingRewardEvents {
        fn from(value: RewardClaimedFilter) -> Self {
            Self::RewardClaimedFilter(value)
        }
    }
    impl ::core::convert::From<RewardDistributedFilter> for ISymbioticStakingRewardEvents {
        fn from(value: RewardDistributedFilter) -> Self {
            Self::RewardDistributedFilter(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenUpdatedFilter> for ISymbioticStakingRewardEvents {
        fn from(value: RewardPerTokenUpdatedFilter) -> Self {
            Self::RewardPerTokenUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<StakingPoolSetFilter> for ISymbioticStakingRewardEvents {
        fn from(value: StakingPoolSetFilter) -> Self {
            Self::StakingPoolSetFilter(value)
        }
    }
    impl ::core::convert::From<SymbioticStakingSetFilter> for ISymbioticStakingRewardEvents {
        fn from(value: SymbioticStakingSetFilter) -> Self {
            Self::SymbioticStakingSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `claimReward` function with signature `claimReward(address)` and selector `0xd279c191`
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
    #[ethcall(name = "claimReward", abi = "claimReward(address)")]
    pub struct ClaimRewardCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `onSnapshotSubmission` function with signature `onSnapshotSubmission(address,address)` and selector `0x45a3fb1e`
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
        name = "onSnapshotSubmission",
        abi = "onSnapshotSubmission(address,address)"
    )]
    pub struct OnSnapshotSubmissionCall {
        pub vault: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rewardAccrued` function with signature `rewardAccrued(address,address)` and selector `0x0bda5be1`
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
    #[ethcall(name = "rewardAccrued", abi = "rewardAccrued(address,address)")]
    pub struct RewardAccruedCall {
        pub reward_token: ::ethers::core::types::Address,
        pub vault: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rewardPerTokenPaid` function with signature `rewardPerTokenPaid(address,address,address,address)` and selector `0x4899b77a`
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
        name = "rewardPerTokenPaid",
        abi = "rewardPerTokenPaid(address,address,address,address)"
    )]
    pub struct RewardPerTokenPaidCall {
        pub stake_token: ::ethers::core::types::Address,
        pub reward_token: ::ethers::core::types::Address,
        pub vault: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rewardPerTokenStored` function with signature `rewardPerTokenStored(address,address,address)` and selector `0x9a5b3430`
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
        name = "rewardPerTokenStored",
        abi = "rewardPerTokenStored(address,address,address)"
    )]
    pub struct RewardPerTokenStoredCall {
        pub stake_token: ::ethers::core::types::Address,
        pub reward_token: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
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
        pub amount: ::ethers::core::types::U256,
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
    pub enum ISymbioticStakingRewardCalls {
        ClaimReward(ClaimRewardCall),
        OnSnapshotSubmission(OnSnapshotSubmissionCall),
        RewardAccrued(RewardAccruedCall),
        RewardPerTokenPaid(RewardPerTokenPaidCall),
        RewardPerTokenStored(RewardPerTokenStoredCall),
        UpdateFeeReward(UpdateFeeRewardCall),
    }
    impl ::ethers::core::abi::AbiDecode for ISymbioticStakingRewardCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ClaimRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimReward(decoded));
            }
            if let Ok(decoded) =
                <OnSnapshotSubmissionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnSnapshotSubmission(decoded));
            }
            if let Ok(decoded) = <RewardAccruedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RewardAccrued(decoded));
            }
            if let Ok(decoded) =
                <RewardPerTokenPaidCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RewardPerTokenPaid(decoded));
            }
            if let Ok(decoded) =
                <RewardPerTokenStoredCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RewardPerTokenStored(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeRewardCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateFeeReward(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISymbioticStakingRewardCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClaimReward(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnSnapshotSubmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardAccrued(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardPerTokenPaid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardPerTokenStored(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFeeReward(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ISymbioticStakingRewardCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnSnapshotSubmission(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardAccrued(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardPerTokenPaid(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardPerTokenStored(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFeeReward(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimRewardCall> for ISymbioticStakingRewardCalls {
        fn from(value: ClaimRewardCall) -> Self {
            Self::ClaimReward(value)
        }
    }
    impl ::core::convert::From<OnSnapshotSubmissionCall> for ISymbioticStakingRewardCalls {
        fn from(value: OnSnapshotSubmissionCall) -> Self {
            Self::OnSnapshotSubmission(value)
        }
    }
    impl ::core::convert::From<RewardAccruedCall> for ISymbioticStakingRewardCalls {
        fn from(value: RewardAccruedCall) -> Self {
            Self::RewardAccrued(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenPaidCall> for ISymbioticStakingRewardCalls {
        fn from(value: RewardPerTokenPaidCall) -> Self {
            Self::RewardPerTokenPaid(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenStoredCall> for ISymbioticStakingRewardCalls {
        fn from(value: RewardPerTokenStoredCall) -> Self {
            Self::RewardPerTokenStored(value)
        }
    }
    impl ::core::convert::From<UpdateFeeRewardCall> for ISymbioticStakingRewardCalls {
        fn from(value: UpdateFeeRewardCall) -> Self {
            Self::UpdateFeeReward(value)
        }
    }
    ///Container type for all return fields from the `rewardAccrued` function with signature `rewardAccrued(address,address)` and selector `0x0bda5be1`
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
    pub struct RewardAccruedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardPerTokenPaid` function with signature `rewardPerTokenPaid(address,address,address,address)` and selector `0x4899b77a`
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
    pub struct RewardPerTokenPaidReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardPerTokenStored` function with signature `rewardPerTokenStored(address,address,address)` and selector `0x9a5b3430`
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
    pub struct RewardPerTokenStoredReturn(pub ::ethers::core::types::U256);
}
