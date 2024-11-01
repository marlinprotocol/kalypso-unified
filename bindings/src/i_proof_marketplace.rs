pub use i_proof_marketplace::*;
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
pub mod i_proof_marketplace {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("slashGenerator"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("slashGenerator"),
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("askId"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },],
                    outputs: ::std::vec![],
                    constant: ::core::option::Option::None,
                    state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                },],
            )]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddExtraIVSImage"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddExtraIVSImage"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddExtraProverImage"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddExtraProverImage",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AskCancelled"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AskCancelled"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AskCreated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AskCreated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("askId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("hasPrivateInputs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("secret_data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("acl"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInputsDetected"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("InvalidInputsDetected",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MarketplaceCreated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MarketplaceCreated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("marketId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorFeeRewardAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorFeeRewardAdded",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeRewardAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorRewardShareSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorRewardShareSet",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("rewardShare"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProofCreated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ProofCreated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("askId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("proof"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProofNotGenerated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ProofNotGenerated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("askId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemoveExtraIVSImage"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RemoveExtraIVSImage",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemoveExtraProverImage"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RemoveExtraProverImage",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("marketId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("imageId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TaskCreated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TaskCreated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("askId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("generator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("new_acl"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransmitterFeeRewardAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TransmitterFeeRewardAdded",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeRewardAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UpdateCostPerBytes"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("UpdateCostPerBytes"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("secretType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("costPerInputBytes"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UpdateMinProvingTime"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("UpdateMinProvingTime",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("secretType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newProvingTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
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
    pub static IPROOFMARKETPLACE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IProofMarketplace<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IProofMarketplace<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IProofMarketplace<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IProofMarketplace<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IProofMarketplace<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IProofMarketplace))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IProofMarketplace<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IPROOFMARKETPLACE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `slashGenerator` (0xfcea3d75) function
        pub fn slash_generator(
            &self,
            ask_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 234, 61, 117], ask_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddExtraIVSImage` event
        pub fn add_extra_ivs_image_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddExtraIVSImageFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `AddExtraProverImage` event
        pub fn add_extra_prover_image_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddExtraProverImageFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `AskCancelled` event
        pub fn ask_cancelled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AskCancelledFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `AskCreated` event
        pub fn ask_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AskCreatedFilter> {
            self.0.event()
        }
        ///Gets the contract's `InvalidInputsDetected` event
        pub fn invalid_inputs_detected_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InvalidInputsDetectedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `MarketplaceCreated` event
        pub fn marketplace_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MarketplaceCreatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OperatorFeeRewardAdded` event
        pub fn operator_fee_reward_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorFeeRewardAddedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OperatorRewardShareSet` event
        pub fn operator_reward_share_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorRewardShareSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ProofCreated` event
        pub fn proof_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProofCreatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ProofNotGenerated` event
        pub fn proof_not_generated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProofNotGeneratedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RemoveExtraIVSImage` event
        pub fn remove_extra_ivs_image_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemoveExtraIVSImageFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RemoveExtraProverImage` event
        pub fn remove_extra_prover_image_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemoveExtraProverImageFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `TaskCreated` event
        pub fn task_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TaskCreatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `TransmitterFeeRewardAdded` event
        pub fn transmitter_fee_reward_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransmitterFeeRewardAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UpdateCostPerBytes` event
        pub fn update_cost_per_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpdateCostPerBytesFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `UpdateMinProvingTime` event
        pub fn update_min_proving_time_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpdateMinProvingTimeFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IProofMarketplaceEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IProofMarketplace<M>
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
    #[ethevent(name = "AddExtraIVSImage", abi = "AddExtraIVSImage(uint256,bytes32)")]
    pub struct AddExtraIVSImageFilter {
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
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
        name = "AddExtraProverImage",
        abi = "AddExtraProverImage(uint256,bytes32)"
    )]
    pub struct AddExtraProverImageFilter {
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
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
    #[ethevent(name = "AskCancelled", abi = "AskCancelled(uint256)")]
    pub struct AskCancelledFilter {
        #[ethevent(indexed)]
        pub ask_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "AskCreated", abi = "AskCreated(uint256,bool,bytes,bytes)")]
    pub struct AskCreatedFilter {
        #[ethevent(indexed)]
        pub ask_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub has_private_inputs: bool,
        pub secret_data: ::ethers::core::types::Bytes,
        pub acl: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "InvalidInputsDetected", abi = "InvalidInputsDetected(uint256)")]
    pub struct InvalidInputsDetectedFilter {
        #[ethevent(indexed)]
        pub ask_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "MarketplaceCreated", abi = "MarketplaceCreated(uint256)")]
    pub struct MarketplaceCreatedFilter {
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
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
        name = "OperatorFeeRewardAdded",
        abi = "OperatorFeeRewardAdded(address,uint256)"
    )]
    pub struct OperatorFeeRewardAddedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub fee_reward_amount: ::ethers::core::types::U256,
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
        name = "OperatorRewardShareSet",
        abi = "OperatorRewardShareSet(address,uint256)"
    )]
    pub struct OperatorRewardShareSetFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub reward_share: ::ethers::core::types::U256,
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
    #[ethevent(name = "ProofCreated", abi = "ProofCreated(uint256,bytes)")]
    pub struct ProofCreatedFilter {
        #[ethevent(indexed)]
        pub ask_id: ::ethers::core::types::U256,
        pub proof: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "ProofNotGenerated", abi = "ProofNotGenerated(uint256)")]
    pub struct ProofNotGeneratedFilter {
        #[ethevent(indexed)]
        pub ask_id: ::ethers::core::types::U256,
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
        name = "RemoveExtraIVSImage",
        abi = "RemoveExtraIVSImage(uint256,bytes32)"
    )]
    pub struct RemoveExtraIVSImageFilter {
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
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
        name = "RemoveExtraProverImage",
        abi = "RemoveExtraProverImage(uint256,bytes32)"
    )]
    pub struct RemoveExtraProverImageFilter {
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
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
    #[ethevent(name = "TaskCreated", abi = "TaskCreated(uint256,address,bytes)")]
    pub struct TaskCreatedFilter {
        #[ethevent(indexed)]
        pub ask_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub new_acl: ::ethers::core::types::Bytes,
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
        name = "TransmitterFeeRewardAdded",
        abi = "TransmitterFeeRewardAdded(address,uint256)"
    )]
    pub struct TransmitterFeeRewardAddedFilter {
        #[ethevent(indexed)]
        pub transmitter: ::ethers::core::types::Address,
        pub fee_reward_amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "UpdateCostPerBytes", abi = "UpdateCostPerBytes(uint8,uint256)")]
    pub struct UpdateCostPerBytesFilter {
        #[ethevent(indexed)]
        pub secret_type: u8,
        pub cost_per_input_bytes: ::ethers::core::types::U256,
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
        name = "UpdateMinProvingTime",
        abi = "UpdateMinProvingTime(uint8,uint256)"
    )]
    pub struct UpdateMinProvingTimeFilter {
        #[ethevent(indexed)]
        pub secret_type: u8,
        pub new_proving_time: ::ethers::core::types::U256,
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
    pub enum IProofMarketplaceEvents {
        AddExtraIVSImageFilter(AddExtraIVSImageFilter),
        AddExtraProverImageFilter(AddExtraProverImageFilter),
        AskCancelledFilter(AskCancelledFilter),
        AskCreatedFilter(AskCreatedFilter),
        InvalidInputsDetectedFilter(InvalidInputsDetectedFilter),
        MarketplaceCreatedFilter(MarketplaceCreatedFilter),
        OperatorFeeRewardAddedFilter(OperatorFeeRewardAddedFilter),
        OperatorRewardShareSetFilter(OperatorRewardShareSetFilter),
        ProofCreatedFilter(ProofCreatedFilter),
        ProofNotGeneratedFilter(ProofNotGeneratedFilter),
        RemoveExtraIVSImageFilter(RemoveExtraIVSImageFilter),
        RemoveExtraProverImageFilter(RemoveExtraProverImageFilter),
        TaskCreatedFilter(TaskCreatedFilter),
        TransmitterFeeRewardAddedFilter(TransmitterFeeRewardAddedFilter),
        UpdateCostPerBytesFilter(UpdateCostPerBytesFilter),
        UpdateMinProvingTimeFilter(UpdateMinProvingTimeFilter),
    }
    impl ::ethers::contract::EthLogDecode for IProofMarketplaceEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddExtraIVSImageFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::AddExtraIVSImageFilter(decoded));
            }
            if let Ok(decoded) = AddExtraProverImageFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::AddExtraProverImageFilter(decoded));
            }
            if let Ok(decoded) = AskCancelledFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::AskCancelledFilter(decoded));
            }
            if let Ok(decoded) = AskCreatedFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::AskCreatedFilter(decoded));
            }
            if let Ok(decoded) = InvalidInputsDetectedFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::InvalidInputsDetectedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = MarketplaceCreatedFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::MarketplaceCreatedFilter(decoded));
            }
            if let Ok(decoded) = OperatorFeeRewardAddedFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::OperatorFeeRewardAddedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorRewardShareSetFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::OperatorRewardShareSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ProofCreatedFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::ProofCreatedFilter(decoded));
            }
            if let Ok(decoded) = ProofNotGeneratedFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::ProofNotGeneratedFilter(decoded));
            }
            if let Ok(decoded) = RemoveExtraIVSImageFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::RemoveExtraIVSImageFilter(decoded));
            }
            if let Ok(decoded) = RemoveExtraProverImageFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::RemoveExtraProverImageFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TaskCreatedFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::TaskCreatedFilter(decoded));
            }
            if let Ok(decoded) = TransmitterFeeRewardAddedFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::TransmitterFeeRewardAddedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UpdateCostPerBytesFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::UpdateCostPerBytesFilter(decoded));
            }
            if let Ok(decoded) = UpdateMinProvingTimeFilter::decode_log(log) {
                return Ok(IProofMarketplaceEvents::UpdateMinProvingTimeFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IProofMarketplaceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddExtraIVSImageFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddExtraProverImageFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AskCancelledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AskCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInputsDetectedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketplaceCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorFeeRewardAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorRewardShareSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProofCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofNotGeneratedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveExtraIVSImageFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveExtraProverImageFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TaskCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransmitterFeeRewardAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateCostPerBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMinProvingTimeFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddExtraIVSImageFilter> for IProofMarketplaceEvents {
        fn from(value: AddExtraIVSImageFilter) -> Self {
            Self::AddExtraIVSImageFilter(value)
        }
    }
    impl ::core::convert::From<AddExtraProverImageFilter> for IProofMarketplaceEvents {
        fn from(value: AddExtraProverImageFilter) -> Self {
            Self::AddExtraProverImageFilter(value)
        }
    }
    impl ::core::convert::From<AskCancelledFilter> for IProofMarketplaceEvents {
        fn from(value: AskCancelledFilter) -> Self {
            Self::AskCancelledFilter(value)
        }
    }
    impl ::core::convert::From<AskCreatedFilter> for IProofMarketplaceEvents {
        fn from(value: AskCreatedFilter) -> Self {
            Self::AskCreatedFilter(value)
        }
    }
    impl ::core::convert::From<InvalidInputsDetectedFilter> for IProofMarketplaceEvents {
        fn from(value: InvalidInputsDetectedFilter) -> Self {
            Self::InvalidInputsDetectedFilter(value)
        }
    }
    impl ::core::convert::From<MarketplaceCreatedFilter> for IProofMarketplaceEvents {
        fn from(value: MarketplaceCreatedFilter) -> Self {
            Self::MarketplaceCreatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorFeeRewardAddedFilter> for IProofMarketplaceEvents {
        fn from(value: OperatorFeeRewardAddedFilter) -> Self {
            Self::OperatorFeeRewardAddedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRewardShareSetFilter> for IProofMarketplaceEvents {
        fn from(value: OperatorRewardShareSetFilter) -> Self {
            Self::OperatorRewardShareSetFilter(value)
        }
    }
    impl ::core::convert::From<ProofCreatedFilter> for IProofMarketplaceEvents {
        fn from(value: ProofCreatedFilter) -> Self {
            Self::ProofCreatedFilter(value)
        }
    }
    impl ::core::convert::From<ProofNotGeneratedFilter> for IProofMarketplaceEvents {
        fn from(value: ProofNotGeneratedFilter) -> Self {
            Self::ProofNotGeneratedFilter(value)
        }
    }
    impl ::core::convert::From<RemoveExtraIVSImageFilter> for IProofMarketplaceEvents {
        fn from(value: RemoveExtraIVSImageFilter) -> Self {
            Self::RemoveExtraIVSImageFilter(value)
        }
    }
    impl ::core::convert::From<RemoveExtraProverImageFilter> for IProofMarketplaceEvents {
        fn from(value: RemoveExtraProverImageFilter) -> Self {
            Self::RemoveExtraProverImageFilter(value)
        }
    }
    impl ::core::convert::From<TaskCreatedFilter> for IProofMarketplaceEvents {
        fn from(value: TaskCreatedFilter) -> Self {
            Self::TaskCreatedFilter(value)
        }
    }
    impl ::core::convert::From<TransmitterFeeRewardAddedFilter> for IProofMarketplaceEvents {
        fn from(value: TransmitterFeeRewardAddedFilter) -> Self {
            Self::TransmitterFeeRewardAddedFilter(value)
        }
    }
    impl ::core::convert::From<UpdateCostPerBytesFilter> for IProofMarketplaceEvents {
        fn from(value: UpdateCostPerBytesFilter) -> Self {
            Self::UpdateCostPerBytesFilter(value)
        }
    }
    impl ::core::convert::From<UpdateMinProvingTimeFilter> for IProofMarketplaceEvents {
        fn from(value: UpdateMinProvingTimeFilter) -> Self {
            Self::UpdateMinProvingTimeFilter(value)
        }
    }
    ///Container type for all input parameters for the `slashGenerator` function with signature `slashGenerator(uint256)` and selector `0xfcea3d75`
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
    #[ethcall(name = "slashGenerator", abi = "slashGenerator(uint256)")]
    pub struct SlashGeneratorCall {
        pub ask_id: ::ethers::core::types::U256,
    }
}
