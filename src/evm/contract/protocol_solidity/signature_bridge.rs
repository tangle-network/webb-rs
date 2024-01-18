pub use signature_bridge_contract::*;
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
pub mod signature_bridge_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialGovernor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("nonce"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EVM_CHAIN_ID_TYPE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("EVM_CHAIN_ID_TYPE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        2usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes2"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_resourceIdToHandlerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_resourceIdToHandlerAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("adminSetResourceWithSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "adminSetResourceWithSignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resourceID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("functionSig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newResourceID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("handlerAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("averageSessionLengthInMillisecs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "averageSessionLengthInMillisecs",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "batchAdminSetResourceWithSignature",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "batchAdminSetResourceWithSignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resourceID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("functionSig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonces"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newResourceIDs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("handlerAddresses"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hashedData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "batchExecuteProposalsWithSignature",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "batchExecuteProposalsWithSignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createVote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createVote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_leafIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proposedGovernor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_siblingPathNodes"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Vote"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("currentVotingPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentVotingPeriod",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "executeManyProposalsWithSignature",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeManyProposalsWithSignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeProposalWithSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeProposalWithSignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getChainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChainId"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getChainIdType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChainIdType"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProposalNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getProposalNonce"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("governor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("governor"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isCorrectExecutionChain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isCorrectExecutionChain",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resourceID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isCorrectExecutionContext"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isCorrectExecutionContext",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resourceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isGovernor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isGovernor"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isSignatureFromGovernor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isSignatureFromGovernor",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isSignatureFromGovernorPrehashed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isSignatureFromGovernorPrehashed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hashedData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastGovernorUpdateTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastGovernorUpdateTime",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseChainIdFromResourceId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "parseChainIdFromResourceId",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_resourceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalNonce"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recover"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("refreshNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("refreshNonce"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sessionLengthMultiplier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "sessionLengthMultiplier",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnershipWithSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "transferOwnershipWithSignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_voterMerkleRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_averageSessionLengthInMillisecs",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_voterCount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_publicKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("voteInFavorForceSetGovernor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "voteInFavorForceSetGovernor",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Vote"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "voteInFavorForceSetGovernorWithSig",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "voteInFavorForceSetGovernorWithSig",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("votes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Vote[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("voterCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("voterCount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("voterMerkleRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("voterMerkleRoot"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("GovernanceOwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "GovernanceOwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RecoveredAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RecoveredAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recovered"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SIGNATUREBRIDGECONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16\x81U`\x03\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x17\x90U`\x02`\x04U`\x06U4\x80\x15b\0\0BW`\0\x80\xFD[P`@Qb\0.\x9F8\x03\x80b\0.\x9F\x839\x81\x01`@\x81\x90Rb\0\0e\x91b\0\0\xD9V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x91\x16\x81\x17`\x01`\xA0\x1Bc\xFF\xFF\xFF\xFF\x85\x16\x02\x17\x82UB`\x01\x81\x90U`@Q\x90\x81R\x84\x92\x84\x92\x91\x7F\x95\x1A(5-\x03~\x92O\xC8)\xDAp\xE1w\xEE\x80\xCA\xA8[\x07\x0Ez[\x98\xB8\x1F\xA2#\xED\xCA\x94\x90` \x01`@Q\x80\x91\x03\x90\xA3PPPPb\0\x01+V[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\xEDW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x05W`\0\x80\xFD[` \x84\x01Q\x90\x92Pc\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x01 W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[a-d\x80b\0\x01;`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xF0W`\x005`\xE0\x1C\x80c\x8E\xBF\xBF\xA9\x11a\x01\x0FW\x80c\xC3gya\x11a\0\xA2W\x80c\xE7\x86\xD3]\x11a\0qW\x80c\xE7\x86\xD3]\x14a\x04>W\x80c\xF2\xDD\x0B\xB7\x14a\x04QW\x80c\xF5\xFC=k\x14a\x04zW\x80c\xFE\xCB\x95S\x14a\x04\x99W`\0\x80\xFD[\x80c\xC3gya\x14a\x03\xFCW\x80c\xC7\xAF3R\x14a\x04\x0FW\x80c\xCC<t\xA1\x14a\x04\"W\x80c\xD5k\x80\x14\x14a\x04+W`\0\x80\xFD[\x80c\xA2\x04\x03\xE9\x11a\0\xDEW\x80c\xA2\x04\x03\xE9\x14a\x03\xB4W\x80c\xA6\xE9L\x91\x14a\x03\xC7W\x80c\xBD\xFA\xDC\x84\x14a\x03\xDAW\x80c\xC2#\rn\x14a\x03\xE3W`\0\x80\xFD[\x80c\x8E\xBF\xBF\xA9\x14a\x03rW\x80c\x95{I\xA1\x14a\x03\x85W\x80c\x9D+\x1E\xD7\x14a\x03\x98W\x80c\x9E\tX<\x14a\x03\xABW`\0\x80\xFD[\x80cB\x16\x9EH\x11a\x01\x87W\x80cqP\x18\xA6\x11a\x01VW\x80cqP\x18\xA6\x14a\x03 W\x80c\x83\x0B/W\x14a\x03(W\x80c\x87U\xBC\xAD\x14a\x03;W\x80c\x8B~\x87\x82\x14a\x03NW`\0\x80\xFD[\x80cB\x16\x9EH\x14a\x02\xB9W\x80cL\x83\x0C\xBD\x14a\x02\xC9W\x80cbE\xE5a\x14a\x02\xE8W\x80cloHF\x14a\x02\xFDW`\0\x80\xFD[\x80c\x1E\xD1=\x1B\x11a\x01\xC3W\x80c\x1E\xD1=\x1B\x14a\x02\x8EW\x80c*i\xFBF\x14a\x02\xA1W\x80c4\x08\xE4p\x14a\x02\xAAW\x80c:\x04\x9E\x02\x14a\x02\xB0W`\0\x80\xFD[\x80c\x01g7\xBB\x14a\x01\xF5W\x80c\x0B'\xFB\x9A\x14a\x02%W\x80c\x0C4\n$\x14a\x027W\x80c\x13\xCB\x01\xF9\x14a\x02bW[`\0\x80\xFD[`\x03Ta\x02\x08\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\tT[`@Q\x90\x81R` \x01a\x02\x1CV[`\0Ta\x02J\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x1CV[`\0Ta\x02y\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x1CV[a\x02Ja\x02\x9C6`\x04a\x1D\xA9V[a\x04\xFEV[a\x02)`\x02T\x81V[Fa\x02)V[a\x02)`\x06T\x81V[`\x05Ta\x02y\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xD1a\x05\x1EV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x1CV[a\x02\xFBa\x02\xF66`\x04a\x1ETV[a\x05lV[\0[a\x03\x10a\x03\x0B6`\x04a\x1E\xD4V[a\x065V[`@Q\x90\x15\x15\x81R` \x01a\x02\x1CV[a\x02\xFBa\x06[V[a\x03\x10a\x0366`\x04a\x1F\x10V[a\x074V[a\x03\x10a\x03I6`\x04a\x1D\xA9V[a\x07XV[a\x03Y`\x01`\xF8\x1B\x81V[`@Q`\x01`\x01`\xF0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x02\x1CV[a\x02\xFBa\x03\x806`\x04a /V[a\x07\x87V[a\x02\xFBa\x03\x936`\x04a!\x83V[a\x08\xC4V[a\x02\xFBa\x03\xA66`\x04a\"LV[a\n\xC4V[a\x02)`\x01T\x81V[a\x02\xFBa\x03\xC26`\x04a\"\xD5V[a\x0B5V[a\x02\xFBa\x03\xD56`\x04a#\x11V[a\x0C\"V[a\x02)`\x04T\x81V[a\x02\x08a\x03\xF16`\x04a\x1F\x10V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\xFBa\x04\n6`\x04a#DV[a\x0C|V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x10V[a\x02)`\tT\x81V[a\x02\xFBa\x0496`\x04a$\xA2V[a\x0E*V[a\x02\xFBa\x04L6`\x04a%rV[a\x11\xDDV[a\x02Ja\x04_6`\x04a\x1F\x10V[`\n` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x10a\x04\x886`\x04a\x1F\x10V[`0\x1C`\x01`\x01`\xA0\x1B\x03\x160\x14\x90V[a\x04\xF1a\x04\xA76`\x04a%\xC3V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91RP`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x84\x01R\x90\x82\x01R\x90V[`@Qa\x02\x1C\x91\x90a&\x16V[\x81Q` \x83\x01 `\0\x90\x81a\x05\x13\x82\x85a\x12XV[\x92PPP[\x92\x91PPV[`@\x80Q`\x01`\xF8\x1B` \x82\x01\x81\x90RF`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\"\x84\x01R\x83Q\x80\x84\x03`\x06\x01\x81R`&\x90\x93\x01\x90\x93R`\0\x92\x91a\x05a\x81a&~V[`\xD0\x1C\x93PPPP\x90V[\x83c\xFF\xFF\xFF\xFF\x16\x80`\tT\x10a\x05\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a&\xB5V[`@Q\x80\x91\x03\x90\xFD[`\tTa\x05\xAB\x90`\x01a'\x0EV[\x81\x11\x15a\x05\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'!V[`\t\x81\x90U`@Qa\x05\xE8\x90\x88\x90\x88\x90\x88\x90\x88\x90\x88\x90` \x01a'~V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82a\x06\x02\x82\x82a\x07XV[a\x06\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'\xC5V[a\x06*\x89\x89\x88\x88a\x12|V[PPPPPPPPPV[`\0\x80a\x06B\x84\x84a\x12XV[`\0T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x94\x93PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a(\x11V[`\0`\x02\x81\x90U`\x03\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\x80\0\0\0\0\0\0\0\x17\x90U`\x05\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90UTa\x06\xCD\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a(WV[`\0\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`\x01`\xA0\x1Bc\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x92\x90\x92\x02`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x81U`@QB\x81R\x81\x90\x7F\x95\x1A(5-\x03~\x92O\xC8)\xDAp\xE1w\xEE\x80\xCA\xA8[\x07\x0Ez[\x98\xB8\x1F\xA2#\xED\xCA\x94\x90` \x01`@Q\x80\x91\x03\x90\xA3V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80a\x07Ha\x05\x1EV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x93\x92PPPV[\x81Q` \x83\x01 `\0\x90\x81a\x07m\x82\x85a\x12XV[`\0T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x95\x94PPPPPV[a\x07\x91\x82\x84a({V[\x81\x80Q\x82Q\x14a\x07\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FSignatureBridge: Data and sig le`D\x82\x01Ro\r\xCC\xEE\x8D\x0Ed\r\xAE\xAEn\x84\r\xAC.\x8Cm`\x83\x1B`d\x82\x01R`\x84\x01a\x05\x94V[`\0[\x82Q\x81\x10\x15a\x08rWa\x08D\x83\x82\x81Q\x81\x10a\x08\x1DWa\x08\x1Da(\x88V[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10a\x087Wa\x087a(\x88V[` \x02` \x01\x01Qa\x07XV[a\x08`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'\xC5V[\x80a\x08j\x81a(\x9EV[\x91PPa\x07\xFFV[P`\0[\x84\x81\x10\x15a\x08\xBCWa\x08\xAA\x86\x86\x83\x81\x81\x10a\x08\x93Wa\x08\x93a(\x88V[\x90P` \x02\x81\x01\x90a\x08\xA5\x91\x90a(\xB7V[a\x15\x87V[\x80a\x08\xB4\x81a(\x9EV[\x91PPa\x08vV[PPPPPPV[`\x03T`\x04Ta\x03\xE8\x91a\x08\xE3\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90a(\xFDV[a\x08\xED\x91\x90a)*V[`\x01Ta\x08\xFA\x91\x90a'\x0EV[B\x10\x15a\t\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a)>V[\x80Q\x82Q\x14a\t\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FGovernable: Invalid number of vo`D\x82\x01Rqtes and signatures`p\x1B`d\x82\x01R`\x84\x01a\x05\x94V[`\0[\x82Q\x81\x10\x15a\n\xBFW`\0`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\t\xAEWa\t\xAEa(\x88V[` \x02` \x01\x01Q` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\t\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a)\x7FV[`\0a\n=\x84\x83\x81Q\x81\x10a\t\xF7Wa\t\xF7a(\x88V[` \x02` \x01\x01Q`@Q` \x01a\n\x0F\x91\x90a&\x16V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84\x84\x81Q\x81\x10a\n0Wa\n0a(\x88V[` \x02` \x01\x01Qa\x04\xFEV[\x90Pa\n\x84\x84\x83\x81Q\x81\x10a\nTWa\nTa(\x88V[` \x02` \x01\x01Q`@\x01Q\x82\x86\x85\x81Q\x81\x10a\nsWa\nsa(\x88V[` \x02` \x01\x01Q`\0\x01Qa\x16\xD9V[\x15a\n\xACWa\n\xAC\x84\x83\x81Q\x81\x10a\n\x9EWa\n\x9Ea(\x88V[` \x02` \x01\x01Q\x82a\x18\x14V[P\x80a\n\xB7\x81a(\x9EV[\x91PPa\t\x88V[PPPV[\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x84\x92Pa\x0B\x08\x91P\x83\x90P\x82a\x07XV[a\x0B$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'\xC5V[a\x0B.\x85\x85a\x15\x87V[PPPPPV[`\x03T`\x04Ta\x03\xE8\x91a\x0BT\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90a(\xFDV[a\x0B^\x91\x90a)*V[`\x01Ta\x0Bk\x91\x90a'\x0EV[B\x10\x15a\x0B\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a)>V[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x0B\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a)\x7FV[`@\x81\x01Q\x81Q3\x91a\x0B\xC8\x91\x83\x90a\x16\xD9V[a\x0C\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FGovernable: Invalid merkle proof`D\x82\x01R`d\x01a\x05\x94V[a\x0C\x1E\x82\x82a\x18\x14V[PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0CLW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a(\x11V[a\x0CU\x82a\x19QV[`\0\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UPV[`\0Tc\xFF\xFF\xFF\xFF\x80\x85\x16`\x01`\xA0\x1B\x90\x92\x04\x16\x10a\x0C\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FGovernable: Invalid nonce\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x94V[`\0Ta\x0C\xF8\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a(WV[c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15a\raW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FGovernable: Nonce must increment`D\x82\x01Rd by 1`\xD8\x1B`d\x82\x01R`\x84\x01a\x05\x94V[\x81Q` \x80\x84\x01\x91\x90\x91 `@Q\x90\x91\x82\x91a\r\xA0\x91a\r\x8B\x91\x8B\x91\x8B\x91\x8B\x91\x8B\x91\x8B\x91\x01a*\0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84a\x07XV[a\r\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a(\x11V[`\x02\x88\x90U`\x03\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x89\x16\x17\x90U`\x05\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x88\x81\x16\x91\x90\x91\x17\x90\x91U`\0\x80Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x92\x88\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90Ua\x0E \x81a\x19QV[PPPPPPPPV[\x84`\0[\x81Q\x81\x10\x15a\x0E\xFCW\x81\x81\x81Q\x81\x10a\x0EIWa\x0EIa(\x88V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16`\tT\x10a\x0EwW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a&\xB5V[`\tTa\x0E\x85\x90`\x01a'\x0EV[\x82\x82\x81Q\x81\x10a\x0E\x97Wa\x0E\x97a(\x88V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0E\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'!V[\x81\x81\x81Q\x81\x10a\x0E\xD5Wa\x0E\xD5a(\x88V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16`\t\x81\x90UP\x80\x80a\x0E\xF4\x90a(\x9EV[\x91PPa\x0E.V[P\x82\x82a\x0F\t\x82\x82a\x065V[a\x0F%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'\xC5V[\x86Q\x88Q\x14\x80\x15a\x0F7WP\x85Q\x87Q\x14[a\x0F\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FSignatureBridge::batchAdminSetRe`D\x82\x01R\x7FsourceWithSignature: Array lengt`d\x82\x01Rl\r\x0Ed\r\xAE\xAEn\x84\r\xAC.\x8Cm`\x9B\x1B`\x84\x82\x01R`\xA4\x01a\x05\x94V[`\0\x88Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xDAWa\x0F\xDAa\x1C\xF4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\rW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0F\xF8W\x90P[P\x90P`\0[\x89Q\x81\x10\x15a\x10\xBFW\x8B\x8B\x8B\x83\x81Q\x81\x10a\x100Wa\x100a(\x88V[` \x02` \x01\x01Q\x8B\x84\x81Q\x81\x10a\x10JWa\x10Ja(\x88V[` \x02` \x01\x01Q\x8B\x85\x81Q\x81\x10a\x10dWa\x10da(\x88V[` \x02` \x01\x01Q`@Q` \x01a\x10\x80\x95\x94\x93\x92\x91\x90a'~V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82\x82\x81Q\x81\x10a\x10\xA1Wa\x10\xA1a(\x88V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x10\xB7\x90a(\x9EV[\x91PPa\x10\x13V[P\x85\x81`@Q` \x01a\x10\xD2\x91\x90a*]V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x11sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FSignatureBridge::batchAdminSetRe`D\x82\x01R\x7FsourceWithSignature: Hashed data`d\x82\x01Rn\x04\x0C\x8D\xEC\xAEd\r\xCD\xEE\x84\r\xAC.\x8Cm`\x8B\x1B`\x84\x82\x01R`\xA4\x01a\x05\x94V[`\0[\x89Q\x81\x10\x15a\x11\xCFWa\x11\xBD\x8C\x8C\x8B\x84\x81Q\x81\x10a\x11\x96Wa\x11\x96a(\x88V[` \x02` \x01\x01Q\x8B\x85\x81Q\x81\x10a\x11\xB0Wa\x11\xB0a(\x88V[` \x02` \x01\x01Qa\x12|V[\x80a\x11\xC7\x81a(\x9EV[\x91PPa\x11vV[PPPPPPPPPPPPV[\x82\x82`@Q` \x01a\x11\xF0\x92\x91\x90a+\0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81a\x12\n\x82\x82a\x07XV[a\x12&W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'\xC5V[`\0[\x84\x81\x10\x15a\x08\xBCWa\x12F\x86\x86\x83\x81\x81\x10a\x08\x93Wa\x08\x93a(\x88V[\x80a\x12P\x81a(\x9EV[\x91PPa\x12)V[`\0\x80`\0a\x12g\x85\x85a\x1AJV[\x91P\x91Pa\x12t\x81a\x1A\x8FV[P\x93\x92PPPV[`@Qc\x83\x0B/W`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R0\x90c\x83\x0B/W\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xDC\x91\x90a+\x9BV[a\x12\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a+\xBDV[`@Qc\x83\x0B/W`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R0\x90c\x83\x0B/W\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x134W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13X\x91\x90a+\x9BV[a\x13tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a+\xBDV[`@Qc\xF5\xFC=k`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R0\x90c\xF5\xFC=k\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD4\x91\x90a+\x9BV[a\x14XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FSignatureBridge::adminSetResourc`D\x82\x01R\x7FeWithSignature: Invalid executio`d\x82\x01Rh\x1B\x88\x18\xDB\xDB\x9D\x19^\x1D`\xBA\x1B`\x84\x82\x01R`\xA4\x01a\x05\x94V[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16cbE\xE5a`\xE0\x1B\x14a\x14\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FSignatureBridge::adminSetResourc`D\x82\x01R\x7FeWithSignature: Invalid function`d\x82\x01Ri signature`\xB0\x1B`\x84\x82\x01R`\xA4\x01a\x05\x94V[`\0\x82\x81R`\n` R`@\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x90\x92U\x91Qc\\}\x1B\x9B`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`0\x85\x90\x1C\x90\x91\x16`$\x82\x01\x81\x90R\x83\x92\x90\x91c\xB8\xFA76\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15gW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15{W=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\0a\x15\x96` \x82\x84\x86a,+V[a\x15\x9F\x91a,UV[`@Qc\x83\x0B/W`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P0\x90c\x83\x0B/W\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x02\x91\x90a+\x9BV[a\x16fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSignatureBridge: Batch Executing`D\x82\x01Rn\x107\xB7\x10;\xB97\xB73\x901\xB40\xB4\xB7`\x89\x1B`d\x82\x01R`\x84\x01a\x05\x94V[`\0\x81\x81R`\n` R`@\x90\x81\x90 T\x90Qcq$g\xF9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x81\x90\x81\x90c\xE2H\xCF\xF2\x90a\x16\xAB\x90\x86\x90\x89\x90\x89\x90`\x04\x01a,sV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xC5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06*W=`\0\x80>=`\0\xFD[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x90\x1B\x16` \x82\x01R`\0\x90\x81\x90`4\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P\x80\x83`\0[\x87Q\x81`\xFF\x16\x10\x15a\x18\x05Wa\x173`\x02\x83a,\x96V[c\xFF\xFF\xFF\xFF\x16`\0\x03a\x17\x95W\x82\x88\x82`\xFF\x16\x81Q\x81\x10a\x17VWa\x17Va(\x88V[` \x02` \x01\x01Q`@Q` \x01a\x17x\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92Pa\x17\xE6V[\x87\x81`\xFF\x16\x81Q\x81\x10a\x17\xAAWa\x17\xAAa(\x88V[` \x02` \x01\x01Q\x83`@Q` \x01a\x17\xCD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92P[a\x17\xF1`\x02\x83a,\xB9V[\x91P\x80a\x17\xFD\x81a,\xDCV[\x91PPa\x17\x1CV[PP`\x02T\x14\x95\x94PPPPPV[`\x06T`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x85R\x92R\x90\x91 T\x16\x15a\x18\xB5W`\x06T`\0\x81\x81R`\x07` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x85R\x90\x83R\x81\x84 T\x94\x84R`\x08\x83R\x81\x84 \x94\x16\x80\x84R\x93\x90\x91R\x81 \x80T`\x01\x92\x90a\x18\x95\x90\x84\x90c\xFF\xFF\xFF\xFF\x16a,\xFBV[\x92Pa\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP[` \x82\x81\x01\x80Q`\x06\x80T`\0\x90\x81R`\x07\x85R`@\x80\x82 `\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x84R\x90\x87R\x81\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x95\x82\x16\x95\x90\x95\x17\x90\x94U\x91T\x81R`\x08\x85R\x81\x81 \x93Q\x90\x92\x16\x82R\x91\x90\x92R\x81 \x80T`\x01\x92\x90a\x19&\x90\x84\x90c\xFF\xFF\xFF\xFF\x16a(WV[\x92Pa\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x0C\x1E\x82` \x01Qa\x1B\xDCV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x19\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FGovernable: New governor is the `D\x82\x01Rkzero address`\xA0\x1B`d\x82\x01R`\x84\x01a\x05\x94V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x17\x83UB`\x01U`\x06\x80T\x91\x90\x92\x16\x92\x90\x91\x90a\x19\xF2\x83a(\x9EV[\x91\x90PUP\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x95\x1A(5-\x03~\x92O\xC8)\xDAp\xE1w\xEE\x80\xCA\xA8[\x07\x0Ez[\x98\xB8\x1F\xA2#\xED\xCA\x94`\x01T`@Qa\x1A>\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPV[`\0\x80\x82Q`A\x03a\x1A\x80W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1At\x87\x82\x85\x85a\x1C0V[\x94P\x94PPPPa\x1A\x88V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x1A\xA3Wa\x1A\xA3a-\x18V[\x03a\x1A\xABWPV[`\x01\x81`\x04\x81\x11\x15a\x1A\xBFWa\x1A\xBFa-\x18V[\x03a\x1B\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x94V[`\x02\x81`\x04\x81\x11\x15a\x1B Wa\x1B a-\x18V[\x03a\x1BmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\x94V[`\x03\x81`\x04\x81\x11\x15a\x1B\x81Wa\x1B\x81a-\x18V[\x03a\x1B\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\x94V[PV[`\x05Ta\x1B\xF1\x90`\x02\x90c\xFF\xFF\xFF\xFF\x16a,\xB9V[`\x06T`\0\x90\x81R`\x08` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 Tc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x11\x15a\x1B\xD9Wa\x1B\xD9\x81a\x19QV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1CgWP`\0\x90P`\x03a\x1C\xEBV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1C\xBBW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\xE4W`\0`\x01\x92P\x92PPa\x1C\xEBV[\x91P`\0\x90P[\x94P\x94\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D2Wa\x1D2a\x1C\xF4V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1DKW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DdWa\x1Dda\x1C\xF4V[a\x1Dw`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1D\nV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1D\x8CW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\xBCW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1D\xD3W`\0\x80\xFD[a\x1D\xDF\x86\x83\x87\x01a\x1D:V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x1D\xF5W`\0\x80\xFD[Pa\x1E\x02\x85\x82\x86\x01a\x1D:V[\x91PP\x92P\x92\x90PV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1E$W`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1E$W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E$W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1EmW`\0\x80\xFD[\x865\x95Pa\x1E}` \x88\x01a\x1E\x0CV[\x94Pa\x1E\x8B`@\x88\x01a\x1E)V[\x93P``\x87\x015\x92Pa\x1E\xA0`\x80\x88\x01a\x1E=V[\x91P`\xA0\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xBBW`\0\x80\xFD[a\x1E\xC7\x89\x82\x8A\x01a\x1D:V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\xE7W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x04W`\0\x80\xFD[a\x1E\x02\x85\x82\x86\x01a\x1D:V[`\0` \x82\x84\x03\x12\x15a\x1F\"W`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1F;W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FRW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1A\x88W`\0\x80\xFD[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1F\x86Wa\x1F\x86a\x1C\xF4V[P`\x05\x1B` \x01\x90V[`\0a\x1F\xA3a\x1F\x9E\x84a\x1FmV[a\x1D\nV[\x83\x81R\x90P` \x80\x82\x01\x90`\x05\x85\x90\x1B\x84\x01\x86\x81\x11\x15a\x1F\xC2W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x1F\xFDW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xE3W`\0\x80\x81\xFD[a\x1F\xEF\x89\x82\x89\x01a\x1D:V[\x85RP\x92\x82\x01\x92\x82\x01a\x1F\xC4V[PPPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a \x19W`\0\x80\xFD[a (\x83\x835` \x85\x01a\x1F\x90V[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a DW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a [W`\0\x80\xFD[a g\x87\x83\x88\x01a\x1F)V[\x90\x95P\x93P` \x86\x015\x91P\x80\x82\x11\x15a \x80W`\0\x80\xFD[Pa \x8D\x86\x82\x87\x01a \x08V[\x91PP\x92P\x92P\x92V[`\0\x82`\x1F\x83\x01\x12a \xA8W`\0\x80\xFD[\x815` a \xB8a\x1F\x9E\x83a\x1FmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a \xD7W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xF2W\x805\x83R\x91\x83\x01\x91\x83\x01a \xDBV[P\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15a!\x0FW`\0\x80\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x82\x82\x10\x81\x83\x11\x17\x15a!2Wa!2a\x1C\xF4V[\x81`@R\x82\x93Pa!B\x85a\x1E)V[\x83Ra!P` \x86\x01a\x1E=V[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a!iW`\0\x80\xFD[Pa!v\x85\x82\x86\x01a \x97V[`@\x83\x01RPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a!\x96W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a!\xADW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a!\xC1W`\0\x80\xFD[\x815` a!\xD1a\x1F\x9E\x83a\x1FmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a!\xF0W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\"(W\x805\x86\x81\x11\x15a\"\x0CW`\0\x80\x81\xFD[a\"\x1A\x8C\x86\x83\x8B\x01\x01a \xFDV[\x84RP\x91\x83\x01\x91\x83\x01a!\xF4V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\"?W`\0\x80\xFD[Pa\x1E\x02\x85\x82\x86\x01a \x08V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"aW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"xW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\"\x8CW`\0\x80\xFD[\x815\x81\x81\x11\x15a\"\x9BW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\"\xADW`\0\x80\xFD[` \x92\x83\x01\x95P\x93P\x90\x85\x015\x90\x80\x82\x11\x15a\"\xC8W`\0\x80\xFD[Pa \x8D\x86\x82\x87\x01a\x1D:V[`\0` \x82\x84\x03\x12\x15a\"\xE7W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xFDW`\0\x80\xFD[a#\t\x84\x82\x85\x01a \xFDV[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a#$W`\0\x80\xFD[a#-\x83a\x1E=V[\x91Pa#;` \x84\x01a\x1E)V[\x90P\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a#]W`\0\x80\xFD[\x865\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x16\x82\x14a#|W`\0\x80\xFD[\x81\x96Pa#\x8B`@\x8A\x01a\x1E)V[\x95Pa#\x99``\x8A\x01a\x1E)V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15a#\xAFW`\0\x80\xFD[a#\xBB\x8A\x83\x8B\x01a\x1D:V[\x93P`\xA0\x89\x015\x91P\x80\x82\x11\x15a#\xD1W`\0\x80\xFD[Pa\x1E\xC7\x89\x82\x8A\x01a\x1D:V[`\0\x82`\x1F\x83\x01\x12a#\xEFW`\0\x80\xFD[\x815` a#\xFFa\x1F\x9E\x83a\x1FmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a$\x1EW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xF2Wa$3\x81a\x1E)V[\x83R\x91\x83\x01\x91\x83\x01a$\"V[`\0\x82`\x1F\x83\x01\x12a$QW`\0\x80\xFD[\x815` a$aa\x1F\x9E\x83a\x1FmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a$\x80W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xF2Wa$\x95\x81a\x1E=V[\x83R\x91\x83\x01\x91\x83\x01a$\x84V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a$\xBDW`\0\x80\xFD[\x875\x96Pa$\xCD` \x89\x01a\x1E\x0CV[\x95P`@\x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a$\xE9W`\0\x80\xFD[a$\xF5\x8B\x83\x8C\x01a#\xDEV[\x96P``\x8A\x015\x91P\x80\x82\x11\x15a%\x0BW`\0\x80\xFD[a%\x17\x8B\x83\x8C\x01a \x97V[\x95P`\x80\x8A\x015\x91P\x80\x82\x11\x15a%-W`\0\x80\xFD[a%9\x8B\x83\x8C\x01a$@V[\x94P`\xA0\x8A\x015\x93P`\xC0\x8A\x015\x91P\x80\x82\x11\x15a%VW`\0\x80\xFD[Pa%c\x8A\x82\x8B\x01a\x1D:V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a%\x87W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%\x9EW`\0\x80\xFD[a%\xAA\x87\x83\x88\x01a\x1F)V[\x90\x95P\x93P` \x86\x015\x91P\x80\x82\x11\x15a\"\xC8W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a%\xD8W`\0\x80\xFD[a%\xE1\x84a\x1E)V[\x92Pa%\xEF` \x85\x01a\x1E=V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\nW`\0\x80\xFD[a \x8D\x86\x82\x87\x01a \x97V[` \x80\x82R\x82Qc\xFF\xFF\xFF\xFF\x16\x82\x82\x01R\x82\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16`@\x80\x84\x01\x91\x90\x91R\x83\x01Q``\x80\x84\x01R\x80Q`\x80\x84\x01\x81\x90R`\0\x92\x91\x82\x01\x90\x83\x90`\xA0\x86\x01\x90[\x80\x83\x10\x15a \xF2W\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a&^V[\x80Q` \x82\x01Q`\x01`\x01`\xD0\x1B\x03\x19\x80\x82\x16\x92\x91\x90`\x06\x83\x10\x15a&\xADW\x80\x81\x84`\x06\x03`\x03\x1B\x1B\x83\x16\x16\x93P[PPP\x91\x90PV[` \x80\x82R`#\x90\x82\x01R\x7FProposalNonceTracker: Invalid no`@\x82\x01Rbnce`\xE8\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05\x18Wa\x05\x18a&\xF8V[` \x80\x82R`:\x90\x82\x01R\x7FProposalNonceTracker: Nonce must`@\x82\x01R\x7F not increment more than 1\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x94\x85R`\x01`\x01`\xE0\x1B\x03\x19\x93\x84\x16` \x86\x01R`\xE0\x92\x90\x92\x1B\x90\x92\x16`$\x84\x01R`(\x83\x01\x91\x90\x91R``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`H\x82\x01R`\\\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FSignatureBridge: Not valid sig f`@\x82\x01Rk97\xB6\x903\xB7\xBB2\xB977\xB9`\xA1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`&\x90\x82\x01R\x7FGovernable: caller is not the go`@\x82\x01Re;2\xB977\xB9`\xD1\x1B``\x82\x01R`\x80\x01\x90V[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a(tWa(ta&\xF8V[P\x92\x91PPV[`\0a (6\x84\x84a\x1F\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a(\xB0Wa(\xB0a&\xF8V[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a(\xCEW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a(\xE8W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1A\x88W`\0\x80\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x18Wa\x05\x18a&\xF8V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)9Wa)9a)\x14V[P\x04\x90V[` \x80\x82R`!\x90\x82\x01R\x7FGovernable: Invalid time for vot`@\x82\x01R`e`\xF8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`8\x90\x82\x01R\x7FGovernable: Proposed governor ca`@\x82\x01R\x7Fnnot be the zero address\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0[\x83\x81\x10\x15a)\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a)\xDFV[PP`\0\x91\x01RV[\x85\x81R`\x01`\x01`@\x1B\x03`\xC0\x1B\x85`\xC0\x1B\x16` \x82\x01R`\0c\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x86`\xE0\x1B\x16`(\x84\x01R\x80\x85`\xE0\x1B\x16`,\x84\x01RP\x82Qa*L\x81`0\x85\x01` \x87\x01a)\xDCV[\x91\x90\x91\x01`0\x01\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a*\xCAW\x87\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87Ra*\xAB\x81\x89\x89\x01\x8A\x85\x01a)\xDCV[`\x1F\x01`\x1F\x19\x16\x95\x90\x95\x01\x86\x01\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a*\x84V[P\x92\x97\x96PPPPPPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@`\x05\x85\x90\x1B\x84\x01\x81\x01\x90\x84\x01\x86\x84[\x87\x81\x10\x15a+\x8EW\x86\x84\x03`?\x19\x01\x83R\x8156\x8A\x90\x03`\x1E\x19\x01\x81\x12a+EW`\0\x80\xFD[\x89\x01\x85\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a+`W`\0\x80\xFD[\x806\x03\x82\x13\x15a+oW`\0\x80\xFD[a+z\x86\x82\x84a*\xD7V[\x95PPP\x91\x84\x01\x91\x90\x84\x01\x90`\x01\x01a+\x1FV[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a+\xADW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a (W`\0\x80\xFD[` \x80\x82R`H\x90\x82\x01R\x7FSignatureBridge::adminSetResourc`@\x82\x01R\x7FeWithSignature: Executing on wro``\x82\x01Rg73\x901\xB40\xB4\xB7`\xC1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x80\x85\x85\x11\x15a,;W`\0\x80\xFD[\x83\x86\x11\x15a,HW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\x05\x18W`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[\x83\x81R`@` \x82\x01R`\0a,\x8D`@\x83\x01\x84\x86a*\xD7V[\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a,\xADWa,\xADa)\x14V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a,\xD0Wa,\xD0a)\x14V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\xFF\x82\x16`\xFF\x81\x03a,\xF2Wa,\xF2a&\xF8V[`\x01\x01\x92\x91PPV[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a(tWa(ta&\xF8V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD5\x81\xF1\xE62\x8B+\x1A\x84\x9B\xB3&\xCD\xF7l\xB9hU\x0F\x18\xB8-a\xF3\xF2\xD9/!\x05\x89\x99\xCAdsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static SIGNATUREBRIDGECONTRACT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xF0W`\x005`\xE0\x1C\x80c\x8E\xBF\xBF\xA9\x11a\x01\x0FW\x80c\xC3gya\x11a\0\xA2W\x80c\xE7\x86\xD3]\x11a\0qW\x80c\xE7\x86\xD3]\x14a\x04>W\x80c\xF2\xDD\x0B\xB7\x14a\x04QW\x80c\xF5\xFC=k\x14a\x04zW\x80c\xFE\xCB\x95S\x14a\x04\x99W`\0\x80\xFD[\x80c\xC3gya\x14a\x03\xFCW\x80c\xC7\xAF3R\x14a\x04\x0FW\x80c\xCC<t\xA1\x14a\x04\"W\x80c\xD5k\x80\x14\x14a\x04+W`\0\x80\xFD[\x80c\xA2\x04\x03\xE9\x11a\0\xDEW\x80c\xA2\x04\x03\xE9\x14a\x03\xB4W\x80c\xA6\xE9L\x91\x14a\x03\xC7W\x80c\xBD\xFA\xDC\x84\x14a\x03\xDAW\x80c\xC2#\rn\x14a\x03\xE3W`\0\x80\xFD[\x80c\x8E\xBF\xBF\xA9\x14a\x03rW\x80c\x95{I\xA1\x14a\x03\x85W\x80c\x9D+\x1E\xD7\x14a\x03\x98W\x80c\x9E\tX<\x14a\x03\xABW`\0\x80\xFD[\x80cB\x16\x9EH\x11a\x01\x87W\x80cqP\x18\xA6\x11a\x01VW\x80cqP\x18\xA6\x14a\x03 W\x80c\x83\x0B/W\x14a\x03(W\x80c\x87U\xBC\xAD\x14a\x03;W\x80c\x8B~\x87\x82\x14a\x03NW`\0\x80\xFD[\x80cB\x16\x9EH\x14a\x02\xB9W\x80cL\x83\x0C\xBD\x14a\x02\xC9W\x80cbE\xE5a\x14a\x02\xE8W\x80cloHF\x14a\x02\xFDW`\0\x80\xFD[\x80c\x1E\xD1=\x1B\x11a\x01\xC3W\x80c\x1E\xD1=\x1B\x14a\x02\x8EW\x80c*i\xFBF\x14a\x02\xA1W\x80c4\x08\xE4p\x14a\x02\xAAW\x80c:\x04\x9E\x02\x14a\x02\xB0W`\0\x80\xFD[\x80c\x01g7\xBB\x14a\x01\xF5W\x80c\x0B'\xFB\x9A\x14a\x02%W\x80c\x0C4\n$\x14a\x027W\x80c\x13\xCB\x01\xF9\x14a\x02bW[`\0\x80\xFD[`\x03Ta\x02\x08\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\tT[`@Q\x90\x81R` \x01a\x02\x1CV[`\0Ta\x02J\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x1CV[`\0Ta\x02y\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x1CV[a\x02Ja\x02\x9C6`\x04a\x1D\xA9V[a\x04\xFEV[a\x02)`\x02T\x81V[Fa\x02)V[a\x02)`\x06T\x81V[`\x05Ta\x02y\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xD1a\x05\x1EV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x1CV[a\x02\xFBa\x02\xF66`\x04a\x1ETV[a\x05lV[\0[a\x03\x10a\x03\x0B6`\x04a\x1E\xD4V[a\x065V[`@Q\x90\x15\x15\x81R` \x01a\x02\x1CV[a\x02\xFBa\x06[V[a\x03\x10a\x0366`\x04a\x1F\x10V[a\x074V[a\x03\x10a\x03I6`\x04a\x1D\xA9V[a\x07XV[a\x03Y`\x01`\xF8\x1B\x81V[`@Q`\x01`\x01`\xF0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x02\x1CV[a\x02\xFBa\x03\x806`\x04a /V[a\x07\x87V[a\x02\xFBa\x03\x936`\x04a!\x83V[a\x08\xC4V[a\x02\xFBa\x03\xA66`\x04a\"LV[a\n\xC4V[a\x02)`\x01T\x81V[a\x02\xFBa\x03\xC26`\x04a\"\xD5V[a\x0B5V[a\x02\xFBa\x03\xD56`\x04a#\x11V[a\x0C\"V[a\x02)`\x04T\x81V[a\x02\x08a\x03\xF16`\x04a\x1F\x10V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\xFBa\x04\n6`\x04a#DV[a\x0C|V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x10V[a\x02)`\tT\x81V[a\x02\xFBa\x0496`\x04a$\xA2V[a\x0E*V[a\x02\xFBa\x04L6`\x04a%rV[a\x11\xDDV[a\x02Ja\x04_6`\x04a\x1F\x10V[`\n` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x10a\x04\x886`\x04a\x1F\x10V[`0\x1C`\x01`\x01`\xA0\x1B\x03\x160\x14\x90V[a\x04\xF1a\x04\xA76`\x04a%\xC3V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91RP`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x84\x01R\x90\x82\x01R\x90V[`@Qa\x02\x1C\x91\x90a&\x16V[\x81Q` \x83\x01 `\0\x90\x81a\x05\x13\x82\x85a\x12XV[\x92PPP[\x92\x91PPV[`@\x80Q`\x01`\xF8\x1B` \x82\x01\x81\x90RF`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\"\x84\x01R\x83Q\x80\x84\x03`\x06\x01\x81R`&\x90\x93\x01\x90\x93R`\0\x92\x91a\x05a\x81a&~V[`\xD0\x1C\x93PPPP\x90V[\x83c\xFF\xFF\xFF\xFF\x16\x80`\tT\x10a\x05\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a&\xB5V[`@Q\x80\x91\x03\x90\xFD[`\tTa\x05\xAB\x90`\x01a'\x0EV[\x81\x11\x15a\x05\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'!V[`\t\x81\x90U`@Qa\x05\xE8\x90\x88\x90\x88\x90\x88\x90\x88\x90\x88\x90` \x01a'~V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82a\x06\x02\x82\x82a\x07XV[a\x06\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'\xC5V[a\x06*\x89\x89\x88\x88a\x12|V[PPPPPPPPPV[`\0\x80a\x06B\x84\x84a\x12XV[`\0T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x94\x93PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a(\x11V[`\0`\x02\x81\x90U`\x03\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\x80\0\0\0\0\0\0\0\x17\x90U`\x05\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90UTa\x06\xCD\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a(WV[`\0\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`\x01`\xA0\x1Bc\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x92\x90\x92\x02`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x81U`@QB\x81R\x81\x90\x7F\x95\x1A(5-\x03~\x92O\xC8)\xDAp\xE1w\xEE\x80\xCA\xA8[\x07\x0Ez[\x98\xB8\x1F\xA2#\xED\xCA\x94\x90` \x01`@Q\x80\x91\x03\x90\xA3V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80a\x07Ha\x05\x1EV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x93\x92PPPV[\x81Q` \x83\x01 `\0\x90\x81a\x07m\x82\x85a\x12XV[`\0T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x95\x94PPPPPV[a\x07\x91\x82\x84a({V[\x81\x80Q\x82Q\x14a\x07\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FSignatureBridge: Data and sig le`D\x82\x01Ro\r\xCC\xEE\x8D\x0Ed\r\xAE\xAEn\x84\r\xAC.\x8Cm`\x83\x1B`d\x82\x01R`\x84\x01a\x05\x94V[`\0[\x82Q\x81\x10\x15a\x08rWa\x08D\x83\x82\x81Q\x81\x10a\x08\x1DWa\x08\x1Da(\x88V[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10a\x087Wa\x087a(\x88V[` \x02` \x01\x01Qa\x07XV[a\x08`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'\xC5V[\x80a\x08j\x81a(\x9EV[\x91PPa\x07\xFFV[P`\0[\x84\x81\x10\x15a\x08\xBCWa\x08\xAA\x86\x86\x83\x81\x81\x10a\x08\x93Wa\x08\x93a(\x88V[\x90P` \x02\x81\x01\x90a\x08\xA5\x91\x90a(\xB7V[a\x15\x87V[\x80a\x08\xB4\x81a(\x9EV[\x91PPa\x08vV[PPPPPPV[`\x03T`\x04Ta\x03\xE8\x91a\x08\xE3\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90a(\xFDV[a\x08\xED\x91\x90a)*V[`\x01Ta\x08\xFA\x91\x90a'\x0EV[B\x10\x15a\t\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a)>V[\x80Q\x82Q\x14a\t\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FGovernable: Invalid number of vo`D\x82\x01Rqtes and signatures`p\x1B`d\x82\x01R`\x84\x01a\x05\x94V[`\0[\x82Q\x81\x10\x15a\n\xBFW`\0`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\t\xAEWa\t\xAEa(\x88V[` \x02` \x01\x01Q` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\t\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a)\x7FV[`\0a\n=\x84\x83\x81Q\x81\x10a\t\xF7Wa\t\xF7a(\x88V[` \x02` \x01\x01Q`@Q` \x01a\n\x0F\x91\x90a&\x16V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84\x84\x81Q\x81\x10a\n0Wa\n0a(\x88V[` \x02` \x01\x01Qa\x04\xFEV[\x90Pa\n\x84\x84\x83\x81Q\x81\x10a\nTWa\nTa(\x88V[` \x02` \x01\x01Q`@\x01Q\x82\x86\x85\x81Q\x81\x10a\nsWa\nsa(\x88V[` \x02` \x01\x01Q`\0\x01Qa\x16\xD9V[\x15a\n\xACWa\n\xAC\x84\x83\x81Q\x81\x10a\n\x9EWa\n\x9Ea(\x88V[` \x02` \x01\x01Q\x82a\x18\x14V[P\x80a\n\xB7\x81a(\x9EV[\x91PPa\t\x88V[PPPV[\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x84\x92Pa\x0B\x08\x91P\x83\x90P\x82a\x07XV[a\x0B$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'\xC5V[a\x0B.\x85\x85a\x15\x87V[PPPPPV[`\x03T`\x04Ta\x03\xE8\x91a\x0BT\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90a(\xFDV[a\x0B^\x91\x90a)*V[`\x01Ta\x0Bk\x91\x90a'\x0EV[B\x10\x15a\x0B\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a)>V[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x0B\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a)\x7FV[`@\x81\x01Q\x81Q3\x91a\x0B\xC8\x91\x83\x90a\x16\xD9V[a\x0C\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FGovernable: Invalid merkle proof`D\x82\x01R`d\x01a\x05\x94V[a\x0C\x1E\x82\x82a\x18\x14V[PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0CLW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a(\x11V[a\x0CU\x82a\x19QV[`\0\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UPV[`\0Tc\xFF\xFF\xFF\xFF\x80\x85\x16`\x01`\xA0\x1B\x90\x92\x04\x16\x10a\x0C\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FGovernable: Invalid nonce\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x94V[`\0Ta\x0C\xF8\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a(WV[c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15a\raW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FGovernable: Nonce must increment`D\x82\x01Rd by 1`\xD8\x1B`d\x82\x01R`\x84\x01a\x05\x94V[\x81Q` \x80\x84\x01\x91\x90\x91 `@Q\x90\x91\x82\x91a\r\xA0\x91a\r\x8B\x91\x8B\x91\x8B\x91\x8B\x91\x8B\x91\x8B\x91\x01a*\0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x84a\x07XV[a\r\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a(\x11V[`\x02\x88\x90U`\x03\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x89\x16\x17\x90U`\x05\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x88\x81\x16\x91\x90\x91\x17\x90\x91U`\0\x80Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x92\x88\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90Ua\x0E \x81a\x19QV[PPPPPPPPV[\x84`\0[\x81Q\x81\x10\x15a\x0E\xFCW\x81\x81\x81Q\x81\x10a\x0EIWa\x0EIa(\x88V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16`\tT\x10a\x0EwW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a&\xB5V[`\tTa\x0E\x85\x90`\x01a'\x0EV[\x82\x82\x81Q\x81\x10a\x0E\x97Wa\x0E\x97a(\x88V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0E\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'!V[\x81\x81\x81Q\x81\x10a\x0E\xD5Wa\x0E\xD5a(\x88V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16`\t\x81\x90UP\x80\x80a\x0E\xF4\x90a(\x9EV[\x91PPa\x0E.V[P\x82\x82a\x0F\t\x82\x82a\x065V[a\x0F%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'\xC5V[\x86Q\x88Q\x14\x80\x15a\x0F7WP\x85Q\x87Q\x14[a\x0F\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FSignatureBridge::batchAdminSetRe`D\x82\x01R\x7FsourceWithSignature: Array lengt`d\x82\x01Rl\r\x0Ed\r\xAE\xAEn\x84\r\xAC.\x8Cm`\x9B\x1B`\x84\x82\x01R`\xA4\x01a\x05\x94V[`\0\x88Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xDAWa\x0F\xDAa\x1C\xF4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\rW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0F\xF8W\x90P[P\x90P`\0[\x89Q\x81\x10\x15a\x10\xBFW\x8B\x8B\x8B\x83\x81Q\x81\x10a\x100Wa\x100a(\x88V[` \x02` \x01\x01Q\x8B\x84\x81Q\x81\x10a\x10JWa\x10Ja(\x88V[` \x02` \x01\x01Q\x8B\x85\x81Q\x81\x10a\x10dWa\x10da(\x88V[` \x02` \x01\x01Q`@Q` \x01a\x10\x80\x95\x94\x93\x92\x91\x90a'~V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82\x82\x81Q\x81\x10a\x10\xA1Wa\x10\xA1a(\x88V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x10\xB7\x90a(\x9EV[\x91PPa\x10\x13V[P\x85\x81`@Q` \x01a\x10\xD2\x91\x90a*]V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x11sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FSignatureBridge::batchAdminSetRe`D\x82\x01R\x7FsourceWithSignature: Hashed data`d\x82\x01Rn\x04\x0C\x8D\xEC\xAEd\r\xCD\xEE\x84\r\xAC.\x8Cm`\x8B\x1B`\x84\x82\x01R`\xA4\x01a\x05\x94V[`\0[\x89Q\x81\x10\x15a\x11\xCFWa\x11\xBD\x8C\x8C\x8B\x84\x81Q\x81\x10a\x11\x96Wa\x11\x96a(\x88V[` \x02` \x01\x01Q\x8B\x85\x81Q\x81\x10a\x11\xB0Wa\x11\xB0a(\x88V[` \x02` \x01\x01Qa\x12|V[\x80a\x11\xC7\x81a(\x9EV[\x91PPa\x11vV[PPPPPPPPPPPPV[\x82\x82`@Q` \x01a\x11\xF0\x92\x91\x90a+\0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81a\x12\n\x82\x82a\x07XV[a\x12&W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a'\xC5V[`\0[\x84\x81\x10\x15a\x08\xBCWa\x12F\x86\x86\x83\x81\x81\x10a\x08\x93Wa\x08\x93a(\x88V[\x80a\x12P\x81a(\x9EV[\x91PPa\x12)V[`\0\x80`\0a\x12g\x85\x85a\x1AJV[\x91P\x91Pa\x12t\x81a\x1A\x8FV[P\x93\x92PPPV[`@Qc\x83\x0B/W`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R0\x90c\x83\x0B/W\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xDC\x91\x90a+\x9BV[a\x12\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a+\xBDV[`@Qc\x83\x0B/W`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R0\x90c\x83\x0B/W\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x134W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13X\x91\x90a+\x9BV[a\x13tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x94\x90a+\xBDV[`@Qc\xF5\xFC=k`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R0\x90c\xF5\xFC=k\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD4\x91\x90a+\x9BV[a\x14XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FSignatureBridge::adminSetResourc`D\x82\x01R\x7FeWithSignature: Invalid executio`d\x82\x01Rh\x1B\x88\x18\xDB\xDB\x9D\x19^\x1D`\xBA\x1B`\x84\x82\x01R`\xA4\x01a\x05\x94V[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16cbE\xE5a`\xE0\x1B\x14a\x14\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FSignatureBridge::adminSetResourc`D\x82\x01R\x7FeWithSignature: Invalid function`d\x82\x01Ri signature`\xB0\x1B`\x84\x82\x01R`\xA4\x01a\x05\x94V[`\0\x82\x81R`\n` R`@\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x90\x92U\x91Qc\\}\x1B\x9B`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`0\x85\x90\x1C\x90\x91\x16`$\x82\x01\x81\x90R\x83\x92\x90\x91c\xB8\xFA76\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15gW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15{W=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\0a\x15\x96` \x82\x84\x86a,+V[a\x15\x9F\x91a,UV[`@Qc\x83\x0B/W`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P0\x90c\x83\x0B/W\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x02\x91\x90a+\x9BV[a\x16fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSignatureBridge: Batch Executing`D\x82\x01Rn\x107\xB7\x10;\xB97\xB73\x901\xB40\xB4\xB7`\x89\x1B`d\x82\x01R`\x84\x01a\x05\x94V[`\0\x81\x81R`\n` R`@\x90\x81\x90 T\x90Qcq$g\xF9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x81\x90\x81\x90c\xE2H\xCF\xF2\x90a\x16\xAB\x90\x86\x90\x89\x90\x89\x90`\x04\x01a,sV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xC5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06*W=`\0\x80>=`\0\xFD[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x90\x1B\x16` \x82\x01R`\0\x90\x81\x90`4\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P\x80\x83`\0[\x87Q\x81`\xFF\x16\x10\x15a\x18\x05Wa\x173`\x02\x83a,\x96V[c\xFF\xFF\xFF\xFF\x16`\0\x03a\x17\x95W\x82\x88\x82`\xFF\x16\x81Q\x81\x10a\x17VWa\x17Va(\x88V[` \x02` \x01\x01Q`@Q` \x01a\x17x\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92Pa\x17\xE6V[\x87\x81`\xFF\x16\x81Q\x81\x10a\x17\xAAWa\x17\xAAa(\x88V[` \x02` \x01\x01Q\x83`@Q` \x01a\x17\xCD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92P[a\x17\xF1`\x02\x83a,\xB9V[\x91P\x80a\x17\xFD\x81a,\xDCV[\x91PPa\x17\x1CV[PP`\x02T\x14\x95\x94PPPPPV[`\x06T`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x85R\x92R\x90\x91 T\x16\x15a\x18\xB5W`\x06T`\0\x81\x81R`\x07` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x85R\x90\x83R\x81\x84 T\x94\x84R`\x08\x83R\x81\x84 \x94\x16\x80\x84R\x93\x90\x91R\x81 \x80T`\x01\x92\x90a\x18\x95\x90\x84\x90c\xFF\xFF\xFF\xFF\x16a,\xFBV[\x92Pa\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP[` \x82\x81\x01\x80Q`\x06\x80T`\0\x90\x81R`\x07\x85R`@\x80\x82 `\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x84R\x90\x87R\x81\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x95\x82\x16\x95\x90\x95\x17\x90\x94U\x91T\x81R`\x08\x85R\x81\x81 \x93Q\x90\x92\x16\x82R\x91\x90\x92R\x81 \x80T`\x01\x92\x90a\x19&\x90\x84\x90c\xFF\xFF\xFF\xFF\x16a(WV[\x92Pa\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x0C\x1E\x82` \x01Qa\x1B\xDCV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x19\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FGovernable: New governor is the `D\x82\x01Rkzero address`\xA0\x1B`d\x82\x01R`\x84\x01a\x05\x94V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x17\x83UB`\x01U`\x06\x80T\x91\x90\x92\x16\x92\x90\x91\x90a\x19\xF2\x83a(\x9EV[\x91\x90PUP\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x95\x1A(5-\x03~\x92O\xC8)\xDAp\xE1w\xEE\x80\xCA\xA8[\x07\x0Ez[\x98\xB8\x1F\xA2#\xED\xCA\x94`\x01T`@Qa\x1A>\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPV[`\0\x80\x82Q`A\x03a\x1A\x80W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1At\x87\x82\x85\x85a\x1C0V[\x94P\x94PPPPa\x1A\x88V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x1A\xA3Wa\x1A\xA3a-\x18V[\x03a\x1A\xABWPV[`\x01\x81`\x04\x81\x11\x15a\x1A\xBFWa\x1A\xBFa-\x18V[\x03a\x1B\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x94V[`\x02\x81`\x04\x81\x11\x15a\x1B Wa\x1B a-\x18V[\x03a\x1BmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\x94V[`\x03\x81`\x04\x81\x11\x15a\x1B\x81Wa\x1B\x81a-\x18V[\x03a\x1B\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\x94V[PV[`\x05Ta\x1B\xF1\x90`\x02\x90c\xFF\xFF\xFF\xFF\x16a,\xB9V[`\x06T`\0\x90\x81R`\x08` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 Tc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x11\x15a\x1B\xD9Wa\x1B\xD9\x81a\x19QV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1CgWP`\0\x90P`\x03a\x1C\xEBV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1C\xBBW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\xE4W`\0`\x01\x92P\x92PPa\x1C\xEBV[\x91P`\0\x90P[\x94P\x94\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D2Wa\x1D2a\x1C\xF4V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1DKW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DdWa\x1Dda\x1C\xF4V[a\x1Dw`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1D\nV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1D\x8CW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\xBCW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1D\xD3W`\0\x80\xFD[a\x1D\xDF\x86\x83\x87\x01a\x1D:V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x1D\xF5W`\0\x80\xFD[Pa\x1E\x02\x85\x82\x86\x01a\x1D:V[\x91PP\x92P\x92\x90PV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1E$W`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1E$W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E$W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1EmW`\0\x80\xFD[\x865\x95Pa\x1E}` \x88\x01a\x1E\x0CV[\x94Pa\x1E\x8B`@\x88\x01a\x1E)V[\x93P``\x87\x015\x92Pa\x1E\xA0`\x80\x88\x01a\x1E=V[\x91P`\xA0\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xBBW`\0\x80\xFD[a\x1E\xC7\x89\x82\x8A\x01a\x1D:V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\xE7W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x04W`\0\x80\xFD[a\x1E\x02\x85\x82\x86\x01a\x1D:V[`\0` \x82\x84\x03\x12\x15a\x1F\"W`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1F;W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FRW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1A\x88W`\0\x80\xFD[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1F\x86Wa\x1F\x86a\x1C\xF4V[P`\x05\x1B` \x01\x90V[`\0a\x1F\xA3a\x1F\x9E\x84a\x1FmV[a\x1D\nV[\x83\x81R\x90P` \x80\x82\x01\x90`\x05\x85\x90\x1B\x84\x01\x86\x81\x11\x15a\x1F\xC2W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x1F\xFDW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xE3W`\0\x80\x81\xFD[a\x1F\xEF\x89\x82\x89\x01a\x1D:V[\x85RP\x92\x82\x01\x92\x82\x01a\x1F\xC4V[PPPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a \x19W`\0\x80\xFD[a (\x83\x835` \x85\x01a\x1F\x90V[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a DW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a [W`\0\x80\xFD[a g\x87\x83\x88\x01a\x1F)V[\x90\x95P\x93P` \x86\x015\x91P\x80\x82\x11\x15a \x80W`\0\x80\xFD[Pa \x8D\x86\x82\x87\x01a \x08V[\x91PP\x92P\x92P\x92V[`\0\x82`\x1F\x83\x01\x12a \xA8W`\0\x80\xFD[\x815` a \xB8a\x1F\x9E\x83a\x1FmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a \xD7W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xF2W\x805\x83R\x91\x83\x01\x91\x83\x01a \xDBV[P\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15a!\x0FW`\0\x80\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x82\x82\x10\x81\x83\x11\x17\x15a!2Wa!2a\x1C\xF4V[\x81`@R\x82\x93Pa!B\x85a\x1E)V[\x83Ra!P` \x86\x01a\x1E=V[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a!iW`\0\x80\xFD[Pa!v\x85\x82\x86\x01a \x97V[`@\x83\x01RPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a!\x96W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a!\xADW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a!\xC1W`\0\x80\xFD[\x815` a!\xD1a\x1F\x9E\x83a\x1FmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a!\xF0W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\"(W\x805\x86\x81\x11\x15a\"\x0CW`\0\x80\x81\xFD[a\"\x1A\x8C\x86\x83\x8B\x01\x01a \xFDV[\x84RP\x91\x83\x01\x91\x83\x01a!\xF4V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\"?W`\0\x80\xFD[Pa\x1E\x02\x85\x82\x86\x01a \x08V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"aW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"xW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\"\x8CW`\0\x80\xFD[\x815\x81\x81\x11\x15a\"\x9BW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\"\xADW`\0\x80\xFD[` \x92\x83\x01\x95P\x93P\x90\x85\x015\x90\x80\x82\x11\x15a\"\xC8W`\0\x80\xFD[Pa \x8D\x86\x82\x87\x01a\x1D:V[`\0` \x82\x84\x03\x12\x15a\"\xE7W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xFDW`\0\x80\xFD[a#\t\x84\x82\x85\x01a \xFDV[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a#$W`\0\x80\xFD[a#-\x83a\x1E=V[\x91Pa#;` \x84\x01a\x1E)V[\x90P\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a#]W`\0\x80\xFD[\x865\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x16\x82\x14a#|W`\0\x80\xFD[\x81\x96Pa#\x8B`@\x8A\x01a\x1E)V[\x95Pa#\x99``\x8A\x01a\x1E)V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15a#\xAFW`\0\x80\xFD[a#\xBB\x8A\x83\x8B\x01a\x1D:V[\x93P`\xA0\x89\x015\x91P\x80\x82\x11\x15a#\xD1W`\0\x80\xFD[Pa\x1E\xC7\x89\x82\x8A\x01a\x1D:V[`\0\x82`\x1F\x83\x01\x12a#\xEFW`\0\x80\xFD[\x815` a#\xFFa\x1F\x9E\x83a\x1FmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a$\x1EW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xF2Wa$3\x81a\x1E)V[\x83R\x91\x83\x01\x91\x83\x01a$\"V[`\0\x82`\x1F\x83\x01\x12a$QW`\0\x80\xFD[\x815` a$aa\x1F\x9E\x83a\x1FmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a$\x80W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xF2Wa$\x95\x81a\x1E=V[\x83R\x91\x83\x01\x91\x83\x01a$\x84V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a$\xBDW`\0\x80\xFD[\x875\x96Pa$\xCD` \x89\x01a\x1E\x0CV[\x95P`@\x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a$\xE9W`\0\x80\xFD[a$\xF5\x8B\x83\x8C\x01a#\xDEV[\x96P``\x8A\x015\x91P\x80\x82\x11\x15a%\x0BW`\0\x80\xFD[a%\x17\x8B\x83\x8C\x01a \x97V[\x95P`\x80\x8A\x015\x91P\x80\x82\x11\x15a%-W`\0\x80\xFD[a%9\x8B\x83\x8C\x01a$@V[\x94P`\xA0\x8A\x015\x93P`\xC0\x8A\x015\x91P\x80\x82\x11\x15a%VW`\0\x80\xFD[Pa%c\x8A\x82\x8B\x01a\x1D:V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a%\x87W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a%\x9EW`\0\x80\xFD[a%\xAA\x87\x83\x88\x01a\x1F)V[\x90\x95P\x93P` \x86\x015\x91P\x80\x82\x11\x15a\"\xC8W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a%\xD8W`\0\x80\xFD[a%\xE1\x84a\x1E)V[\x92Pa%\xEF` \x85\x01a\x1E=V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\nW`\0\x80\xFD[a \x8D\x86\x82\x87\x01a \x97V[` \x80\x82R\x82Qc\xFF\xFF\xFF\xFF\x16\x82\x82\x01R\x82\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16`@\x80\x84\x01\x91\x90\x91R\x83\x01Q``\x80\x84\x01R\x80Q`\x80\x84\x01\x81\x90R`\0\x92\x91\x82\x01\x90\x83\x90`\xA0\x86\x01\x90[\x80\x83\x10\x15a \xF2W\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a&^V[\x80Q` \x82\x01Q`\x01`\x01`\xD0\x1B\x03\x19\x80\x82\x16\x92\x91\x90`\x06\x83\x10\x15a&\xADW\x80\x81\x84`\x06\x03`\x03\x1B\x1B\x83\x16\x16\x93P[PPP\x91\x90PV[` \x80\x82R`#\x90\x82\x01R\x7FProposalNonceTracker: Invalid no`@\x82\x01Rbnce`\xE8\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05\x18Wa\x05\x18a&\xF8V[` \x80\x82R`:\x90\x82\x01R\x7FProposalNonceTracker: Nonce must`@\x82\x01R\x7F not increment more than 1\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x94\x85R`\x01`\x01`\xE0\x1B\x03\x19\x93\x84\x16` \x86\x01R`\xE0\x92\x90\x92\x1B\x90\x92\x16`$\x84\x01R`(\x83\x01\x91\x90\x91R``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`H\x82\x01R`\\\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FSignatureBridge: Not valid sig f`@\x82\x01Rk97\xB6\x903\xB7\xBB2\xB977\xB9`\xA1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`&\x90\x82\x01R\x7FGovernable: caller is not the go`@\x82\x01Re;2\xB977\xB9`\xD1\x1B``\x82\x01R`\x80\x01\x90V[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a(tWa(ta&\xF8V[P\x92\x91PPV[`\0a (6\x84\x84a\x1F\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a(\xB0Wa(\xB0a&\xF8V[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a(\xCEW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a(\xE8W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1A\x88W`\0\x80\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x18Wa\x05\x18a&\xF8V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)9Wa)9a)\x14V[P\x04\x90V[` \x80\x82R`!\x90\x82\x01R\x7FGovernable: Invalid time for vot`@\x82\x01R`e`\xF8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`8\x90\x82\x01R\x7FGovernable: Proposed governor ca`@\x82\x01R\x7Fnnot be the zero address\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0[\x83\x81\x10\x15a)\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a)\xDFV[PP`\0\x91\x01RV[\x85\x81R`\x01`\x01`@\x1B\x03`\xC0\x1B\x85`\xC0\x1B\x16` \x82\x01R`\0c\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x86`\xE0\x1B\x16`(\x84\x01R\x80\x85`\xE0\x1B\x16`,\x84\x01RP\x82Qa*L\x81`0\x85\x01` \x87\x01a)\xDCV[\x91\x90\x91\x01`0\x01\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a*\xCAW\x87\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87Ra*\xAB\x81\x89\x89\x01\x8A\x85\x01a)\xDCV[`\x1F\x01`\x1F\x19\x16\x95\x90\x95\x01\x86\x01\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a*\x84V[P\x92\x97\x96PPPPPPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@`\x05\x85\x90\x1B\x84\x01\x81\x01\x90\x84\x01\x86\x84[\x87\x81\x10\x15a+\x8EW\x86\x84\x03`?\x19\x01\x83R\x8156\x8A\x90\x03`\x1E\x19\x01\x81\x12a+EW`\0\x80\xFD[\x89\x01\x85\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a+`W`\0\x80\xFD[\x806\x03\x82\x13\x15a+oW`\0\x80\xFD[a+z\x86\x82\x84a*\xD7V[\x95PPP\x91\x84\x01\x91\x90\x84\x01\x90`\x01\x01a+\x1FV[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a+\xADW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a (W`\0\x80\xFD[` \x80\x82R`H\x90\x82\x01R\x7FSignatureBridge::adminSetResourc`@\x82\x01R\x7FeWithSignature: Executing on wro``\x82\x01Rg73\x901\xB40\xB4\xB7`\xC1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x80\x85\x85\x11\x15a,;W`\0\x80\xFD[\x83\x86\x11\x15a,HW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\x05\x18W`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[\x83\x81R`@` \x82\x01R`\0a,\x8D`@\x83\x01\x84\x86a*\xD7V[\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a,\xADWa,\xADa)\x14V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a,\xD0Wa,\xD0a)\x14V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\xFF\x82\x16`\xFF\x81\x03a,\xF2Wa,\xF2a&\xF8V[`\x01\x01\x92\x91PPV[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a(tWa(ta&\xF8V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD5\x81\xF1\xE62\x8B+\x1A\x84\x9B\xB3&\xCD\xF7l\xB9hU\x0F\x18\xB8-a\xF3\xF2\xD9/!\x05\x89\x99\xCAdsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static SIGNATUREBRIDGECONTRACT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct SignatureBridgeContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SignatureBridgeContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SignatureBridgeContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SignatureBridgeContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SignatureBridgeContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SignatureBridgeContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SignatureBridgeContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SIGNATUREBRIDGECONTRACT_ABI.clone(),
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
                SIGNATUREBRIDGECONTRACT_ABI.clone(),
                SIGNATUREBRIDGECONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `EVM_CHAIN_ID_TYPE` (0x8b7e8782) function
        pub fn evm_chain_id_type(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 2]> {
            self.0
                .method_hash([139, 126, 135, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_resourceIdToHandlerAddress` (0xf2dd0bb7) function
        pub fn resource_id_to_handler_address(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([242, 221, 11, 183], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `adminSetResourceWithSignature` (0x6245e561) function
        pub fn admin_set_resource_with_signature(
            &self,
            resource_id: [u8; 32],
            function_sig: [u8; 4],
            nonce: u32,
            new_resource_id: [u8; 32],
            handler_address: ::ethers::core::types::Address,
            sig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [98, 69, 229, 97],
                    (
                        resource_id,
                        function_sig,
                        nonce,
                        new_resource_id,
                        handler_address,
                        sig,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `averageSessionLengthInMillisecs` (0x016737bb) function
        pub fn average_session_length_in_millisecs(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([1, 103, 55, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchAdminSetResourceWithSignature` (0xd56b8014) function
        pub fn batch_admin_set_resource_with_signature(
            &self,
            resource_id: [u8; 32],
            function_sig: [u8; 4],
            nonces: ::std::vec::Vec<u32>,
            new_resource_i_ds: ::std::vec::Vec<[u8; 32]>,
            handler_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
            hashed_data: [u8; 32],
            sig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 107, 128, 20],
                    (
                        resource_id,
                        function_sig,
                        nonces,
                        new_resource_i_ds,
                        handler_addresses,
                        hashed_data,
                        sig,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchExecuteProposalsWithSignature` (0xe786d35d) function
        pub fn batch_execute_proposals_with_signature(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
            sig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 134, 211, 93], (data, sig))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createVote` (0xfecb9553) function
        pub fn create_vote(
            &self,
            leaf_index: u32,
            proposed_governor: ::ethers::core::types::Address,
            sibling_path_nodes: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, Vote> {
            self.0
                .method_hash(
                    [254, 203, 149, 83],
                    (leaf_index, proposed_governor, sibling_path_nodes),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentVotingPeriod` (0x3a049e02) function
        pub fn current_voting_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([58, 4, 158, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeManyProposalsWithSignature` (0x8ebfbfa9) function
        pub fn execute_many_proposals_with_signature(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
            sig: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 191, 191, 169], (data, sig))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeProposalWithSignature` (0x9d2b1ed7) function
        pub fn execute_proposal_with_signature(
            &self,
            data: ::ethers::core::types::Bytes,
            sig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 43, 30, 215], (data, sig))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainIdType` (0x4c830cbd) function
        pub fn get_chain_id_type(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([76, 131, 12, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProposalNonce` (0x0b27fb9a) function
        pub fn get_proposal_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([11, 39, 251, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `governor` (0x0c340a24) function
        pub fn governor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([12, 52, 10, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCorrectExecutionChain` (0x830b2f57) function
        pub fn is_correct_execution_chain(
            &self,
            resource_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([131, 11, 47, 87], resource_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCorrectExecutionContext` (0xf5fc3d6b) function
        pub fn is_correct_execution_context(
            &self,
            resource_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([245, 252, 61, 107], resource_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isGovernor` (0xc7af3352) function
        pub fn is_governor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([199, 175, 51, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSignatureFromGovernor` (0x8755bcad) function
        pub fn is_signature_from_governor(
            &self,
            data: ::ethers::core::types::Bytes,
            sig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([135, 85, 188, 173], (data, sig))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSignatureFromGovernorPrehashed` (0x6c6f4846) function
        pub fn is_signature_from_governor_prehashed(
            &self,
            hashed_data: [u8; 32],
            sig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([108, 111, 72, 70], (hashed_data, sig))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastGovernorUpdateTime` (0x9e09583c) function
        pub fn last_governor_update_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([158, 9, 88, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseChainIdFromResourceId` (0xc2230d6e) function
        pub fn parse_chain_id_from_resource_id(
            &self,
            resource_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([194, 35, 13, 110], resource_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalNonce` (0xcc3c74a1) function
        pub fn proposal_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([204, 60, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recover` (0x1ed13d1b) function
        pub fn recover(
            &self,
            data: ::ethers::core::types::Bytes,
            sig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([30, 209, 61, 27], (data, sig))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refreshNonce` (0x13cb01f9) function
        pub fn refresh_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([19, 203, 1, 249], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sessionLengthMultiplier` (0xbdfadc84) function
        pub fn session_length_multiplier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([189, 250, 220, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xa6e94c91) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 233, 76, 145], (new_owner, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnershipWithSignature` (0xc3677961) function
        pub fn transfer_ownership_with_signature(
            &self,
            voter_merkle_root: [u8; 32],
            average_session_length_in_millisecs: u64,
            voter_count: u32,
            nonce: u32,
            public_key: ::ethers::core::types::Bytes,
            sig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [195, 103, 121, 97],
                    (
                        voter_merkle_root,
                        average_session_length_in_millisecs,
                        voter_count,
                        nonce,
                        public_key,
                        sig,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voteInFavorForceSetGovernor` (0xa20403e9) function
        pub fn vote_in_favor_force_set_governor(
            &self,
            vote: Vote,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 4, 3, 233], (vote,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voteInFavorForceSetGovernorWithSig` (0x957b49a1) function
        pub fn vote_in_favor_force_set_governor_with_sig(
            &self,
            votes: ::std::vec::Vec<Vote>,
            sigs: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 123, 73, 161], (votes, sigs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voterCount` (0x42169e48) function
        pub fn voter_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([66, 22, 158, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voterMerkleRoot` (0x2a69fb46) function
        pub fn voter_merkle_root(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([42, 105, 251, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `GovernanceOwnershipTransferred` event
        pub fn governance_ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GovernanceOwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RecoveredAddress` event
        pub fn recovered_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RecoveredAddressFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SignatureBridgeContractEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for SignatureBridgeContract<M>
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
    #[ethevent(
        name = "GovernanceOwnershipTransferred",
        abi = "GovernanceOwnershipTransferred(address,address,uint256)"
    )]
    pub struct GovernanceOwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
        pub timestamp: ::ethers::core::types::U256,
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
    #[ethevent(name = "RecoveredAddress", abi = "RecoveredAddress(address)")]
    pub struct RecoveredAddressFilter {
        #[ethevent(indexed)]
        pub recovered: ::ethers::core::types::Address,
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
    pub enum SignatureBridgeContractEvents {
        GovernanceOwnershipTransferredFilter(
            GovernanceOwnershipTransferredFilter,
        ),
        RecoveredAddressFilter(RecoveredAddressFilter),
    }
    impl ::ethers::contract::EthLogDecode for SignatureBridgeContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) =
                GovernanceOwnershipTransferredFilter::decode_log(log)
            {
                return Ok(
                    SignatureBridgeContractEvents::GovernanceOwnershipTransferredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RecoveredAddressFilter::decode_log(log) {
                return Ok(
                    SignatureBridgeContractEvents::RecoveredAddressFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SignatureBridgeContractEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::GovernanceOwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RecoveredAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GovernanceOwnershipTransferredFilter>
        for SignatureBridgeContractEvents
    {
        fn from(value: GovernanceOwnershipTransferredFilter) -> Self {
            Self::GovernanceOwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RecoveredAddressFilter>
        for SignatureBridgeContractEvents
    {
        fn from(value: RecoveredAddressFilter) -> Self {
            Self::RecoveredAddressFilter(value)
        }
    }
    ///Container type for all input parameters for the `EVM_CHAIN_ID_TYPE` function with signature `EVM_CHAIN_ID_TYPE()` and selector `0x8b7e8782`
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
    #[ethcall(name = "EVM_CHAIN_ID_TYPE", abi = "EVM_CHAIN_ID_TYPE()")]
    pub struct EvmChainIdTypeCall;
    ///Container type for all input parameters for the `_resourceIdToHandlerAddress` function with signature `_resourceIdToHandlerAddress(bytes32)` and selector `0xf2dd0bb7`
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
        name = "_resourceIdToHandlerAddress",
        abi = "_resourceIdToHandlerAddress(bytes32)"
    )]
    pub struct ResourceIdToHandlerAddressCall(pub [u8; 32]);
    ///Container type for all input parameters for the `adminSetResourceWithSignature` function with signature `adminSetResourceWithSignature(bytes32,bytes4,uint32,bytes32,address,bytes)` and selector `0x6245e561`
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
        name = "adminSetResourceWithSignature",
        abi = "adminSetResourceWithSignature(bytes32,bytes4,uint32,bytes32,address,bytes)"
    )]
    pub struct AdminSetResourceWithSignatureCall {
        pub resource_id: [u8; 32],
        pub function_sig: [u8; 4],
        pub nonce: u32,
        pub new_resource_id: [u8; 32],
        pub handler_address: ::ethers::core::types::Address,
        pub sig: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `averageSessionLengthInMillisecs` function with signature `averageSessionLengthInMillisecs()` and selector `0x016737bb`
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
        name = "averageSessionLengthInMillisecs",
        abi = "averageSessionLengthInMillisecs()"
    )]
    pub struct AverageSessionLengthInMillisecsCall;
    ///Container type for all input parameters for the `batchAdminSetResourceWithSignature` function with signature `batchAdminSetResourceWithSignature(bytes32,bytes4,uint32[],bytes32[],address[],bytes32,bytes)` and selector `0xd56b8014`
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
        name = "batchAdminSetResourceWithSignature",
        abi = "batchAdminSetResourceWithSignature(bytes32,bytes4,uint32[],bytes32[],address[],bytes32,bytes)"
    )]
    pub struct BatchAdminSetResourceWithSignatureCall {
        pub resource_id: [u8; 32],
        pub function_sig: [u8; 4],
        pub nonces: ::std::vec::Vec<u32>,
        pub new_resource_i_ds: ::std::vec::Vec<[u8; 32]>,
        pub handler_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub hashed_data: [u8; 32],
        pub sig: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `batchExecuteProposalsWithSignature` function with signature `batchExecuteProposalsWithSignature(bytes[],bytes)` and selector `0xe786d35d`
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
        name = "batchExecuteProposalsWithSignature",
        abi = "batchExecuteProposalsWithSignature(bytes[],bytes)"
    )]
    pub struct BatchExecuteProposalsWithSignatureCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub sig: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `createVote` function with signature `createVote(uint32,address,bytes32[])` and selector `0xfecb9553`
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
        name = "createVote",
        abi = "createVote(uint32,address,bytes32[])"
    )]
    pub struct CreateVoteCall {
        pub leaf_index: u32,
        pub proposed_governor: ::ethers::core::types::Address,
        pub sibling_path_nodes: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `currentVotingPeriod` function with signature `currentVotingPeriod()` and selector `0x3a049e02`
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
    #[ethcall(name = "currentVotingPeriod", abi = "currentVotingPeriod()")]
    pub struct CurrentVotingPeriodCall;
    ///Container type for all input parameters for the `executeManyProposalsWithSignature` function with signature `executeManyProposalsWithSignature(bytes[],bytes[])` and selector `0x8ebfbfa9`
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
        name = "executeManyProposalsWithSignature",
        abi = "executeManyProposalsWithSignature(bytes[],bytes[])"
    )]
    pub struct ExecuteManyProposalsWithSignatureCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub sig: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `executeProposalWithSignature` function with signature `executeProposalWithSignature(bytes,bytes)` and selector `0x9d2b1ed7`
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
        name = "executeProposalWithSignature",
        abi = "executeProposalWithSignature(bytes,bytes)"
    )]
    pub struct ExecuteProposalWithSignatureCall {
        pub data: ::ethers::core::types::Bytes,
        pub sig: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    ///Container type for all input parameters for the `getChainIdType` function with signature `getChainIdType()` and selector `0x4c830cbd`
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
    #[ethcall(name = "getChainIdType", abi = "getChainIdType()")]
    pub struct GetChainIdTypeCall;
    ///Container type for all input parameters for the `getProposalNonce` function with signature `getProposalNonce()` and selector `0x0b27fb9a`
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
    #[ethcall(name = "getProposalNonce", abi = "getProposalNonce()")]
    pub struct GetProposalNonceCall;
    ///Container type for all input parameters for the `governor` function with signature `governor()` and selector `0x0c340a24`
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
    #[ethcall(name = "governor", abi = "governor()")]
    pub struct GovernorCall;
    ///Container type for all input parameters for the `isCorrectExecutionChain` function with signature `isCorrectExecutionChain(bytes32)` and selector `0x830b2f57`
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
        name = "isCorrectExecutionChain",
        abi = "isCorrectExecutionChain(bytes32)"
    )]
    pub struct IsCorrectExecutionChainCall {
        pub resource_id: [u8; 32],
    }
    ///Container type for all input parameters for the `isCorrectExecutionContext` function with signature `isCorrectExecutionContext(bytes32)` and selector `0xf5fc3d6b`
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
        name = "isCorrectExecutionContext",
        abi = "isCorrectExecutionContext(bytes32)"
    )]
    pub struct IsCorrectExecutionContextCall {
        pub resource_id: [u8; 32],
    }
    ///Container type for all input parameters for the `isGovernor` function with signature `isGovernor()` and selector `0xc7af3352`
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
    #[ethcall(name = "isGovernor", abi = "isGovernor()")]
    pub struct IsGovernorCall;
    ///Container type for all input parameters for the `isSignatureFromGovernor` function with signature `isSignatureFromGovernor(bytes,bytes)` and selector `0x8755bcad`
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
        name = "isSignatureFromGovernor",
        abi = "isSignatureFromGovernor(bytes,bytes)"
    )]
    pub struct IsSignatureFromGovernorCall {
        pub data: ::ethers::core::types::Bytes,
        pub sig: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `isSignatureFromGovernorPrehashed` function with signature `isSignatureFromGovernorPrehashed(bytes32,bytes)` and selector `0x6c6f4846`
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
        name = "isSignatureFromGovernorPrehashed",
        abi = "isSignatureFromGovernorPrehashed(bytes32,bytes)"
    )]
    pub struct IsSignatureFromGovernorPrehashedCall {
        pub hashed_data: [u8; 32],
        pub sig: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `lastGovernorUpdateTime` function with signature `lastGovernorUpdateTime()` and selector `0x9e09583c`
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
        name = "lastGovernorUpdateTime",
        abi = "lastGovernorUpdateTime()"
    )]
    pub struct LastGovernorUpdateTimeCall;
    ///Container type for all input parameters for the `parseChainIdFromResourceId` function with signature `parseChainIdFromResourceId(bytes32)` and selector `0xc2230d6e`
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
        name = "parseChainIdFromResourceId",
        abi = "parseChainIdFromResourceId(bytes32)"
    )]
    pub struct ParseChainIdFromResourceIdCall {
        pub resource_id: [u8; 32],
    }
    ///Container type for all input parameters for the `proposalNonce` function with signature `proposalNonce()` and selector `0xcc3c74a1`
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
    #[ethcall(name = "proposalNonce", abi = "proposalNonce()")]
    pub struct ProposalNonceCall;
    ///Container type for all input parameters for the `recover` function with signature `recover(bytes,bytes)` and selector `0x1ed13d1b`
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
    #[ethcall(name = "recover", abi = "recover(bytes,bytes)")]
    pub struct RecoverCall {
        pub data: ::ethers::core::types::Bytes,
        pub sig: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `refreshNonce` function with signature `refreshNonce()` and selector `0x13cb01f9`
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
    #[ethcall(name = "refreshNonce", abi = "refreshNonce()")]
    pub struct RefreshNonceCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `sessionLengthMultiplier` function with signature `sessionLengthMultiplier()` and selector `0xbdfadc84`
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
        name = "sessionLengthMultiplier",
        abi = "sessionLengthMultiplier()"
    )]
    pub struct SessionLengthMultiplierCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address,uint32)` and selector `0xa6e94c91`
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
        name = "transferOwnership",
        abi = "transferOwnership(address,uint32)"
    )]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `transferOwnershipWithSignature` function with signature `transferOwnershipWithSignature(bytes32,uint64,uint32,uint32,bytes,bytes)` and selector `0xc3677961`
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
        name = "transferOwnershipWithSignature",
        abi = "transferOwnershipWithSignature(bytes32,uint64,uint32,uint32,bytes,bytes)"
    )]
    pub struct TransferOwnershipWithSignatureCall {
        pub voter_merkle_root: [u8; 32],
        pub average_session_length_in_millisecs: u64,
        pub voter_count: u32,
        pub nonce: u32,
        pub public_key: ::ethers::core::types::Bytes,
        pub sig: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `voteInFavorForceSetGovernor` function with signature `voteInFavorForceSetGovernor((uint32,address,bytes32[]))` and selector `0xa20403e9`
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
        name = "voteInFavorForceSetGovernor",
        abi = "voteInFavorForceSetGovernor((uint32,address,bytes32[]))"
    )]
    pub struct VoteInFavorForceSetGovernorCall {
        pub vote: Vote,
    }
    ///Container type for all input parameters for the `voteInFavorForceSetGovernorWithSig` function with signature `voteInFavorForceSetGovernorWithSig((uint32,address,bytes32[])[],bytes[])` and selector `0x957b49a1`
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
        name = "voteInFavorForceSetGovernorWithSig",
        abi = "voteInFavorForceSetGovernorWithSig((uint32,address,bytes32[])[],bytes[])"
    )]
    pub struct VoteInFavorForceSetGovernorWithSigCall {
        pub votes: ::std::vec::Vec<Vote>,
        pub sigs: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `voterCount` function with signature `voterCount()` and selector `0x42169e48`
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
    #[ethcall(name = "voterCount", abi = "voterCount()")]
    pub struct VoterCountCall;
    ///Container type for all input parameters for the `voterMerkleRoot` function with signature `voterMerkleRoot()` and selector `0x2a69fb46`
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
    #[ethcall(name = "voterMerkleRoot", abi = "voterMerkleRoot()")]
    pub struct VoterMerkleRootCall;
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
    pub enum SignatureBridgeContractCalls {
        EvmChainIdType(EvmChainIdTypeCall),
        ResourceIdToHandlerAddress(ResourceIdToHandlerAddressCall),
        AdminSetResourceWithSignature(AdminSetResourceWithSignatureCall),
        AverageSessionLengthInMillisecs(AverageSessionLengthInMillisecsCall),
        BatchAdminSetResourceWithSignature(
            BatchAdminSetResourceWithSignatureCall,
        ),
        BatchExecuteProposalsWithSignature(
            BatchExecuteProposalsWithSignatureCall,
        ),
        CreateVote(CreateVoteCall),
        CurrentVotingPeriod(CurrentVotingPeriodCall),
        ExecuteManyProposalsWithSignature(
            ExecuteManyProposalsWithSignatureCall,
        ),
        ExecuteProposalWithSignature(ExecuteProposalWithSignatureCall),
        GetChainId(GetChainIdCall),
        GetChainIdType(GetChainIdTypeCall),
        GetProposalNonce(GetProposalNonceCall),
        Governor(GovernorCall),
        IsCorrectExecutionChain(IsCorrectExecutionChainCall),
        IsCorrectExecutionContext(IsCorrectExecutionContextCall),
        IsGovernor(IsGovernorCall),
        IsSignatureFromGovernor(IsSignatureFromGovernorCall),
        IsSignatureFromGovernorPrehashed(IsSignatureFromGovernorPrehashedCall),
        LastGovernorUpdateTime(LastGovernorUpdateTimeCall),
        ParseChainIdFromResourceId(ParseChainIdFromResourceIdCall),
        ProposalNonce(ProposalNonceCall),
        Recover(RecoverCall),
        RefreshNonce(RefreshNonceCall),
        RenounceOwnership(RenounceOwnershipCall),
        SessionLengthMultiplier(SessionLengthMultiplierCall),
        TransferOwnership(TransferOwnershipCall),
        TransferOwnershipWithSignature(TransferOwnershipWithSignatureCall),
        VoteInFavorForceSetGovernor(VoteInFavorForceSetGovernorCall),
        VoteInFavorForceSetGovernorWithSig(
            VoteInFavorForceSetGovernorWithSigCall,
        ),
        VoterCount(VoterCountCall),
        VoterMerkleRoot(VoterMerkleRootCall),
    }
    impl ::ethers::core::abi::AbiDecode for SignatureBridgeContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <EvmChainIdTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::EvmChainIdType(decoded));
            }
            if let Ok(decoded) = <ResourceIdToHandlerAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResourceIdToHandlerAddress(decoded));
            }
            if let Ok(decoded) = <AdminSetResourceWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdminSetResourceWithSignature(decoded));
            }
            if let Ok(decoded) = <AverageSessionLengthInMillisecsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AverageSessionLengthInMillisecs(decoded));
            }
            if let Ok(decoded) = <BatchAdminSetResourceWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchAdminSetResourceWithSignature(decoded));
            }
            if let Ok(decoded) = <BatchExecuteProposalsWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchExecuteProposalsWithSignature(decoded));
            }
            if let Ok(decoded) =
                <CreateVoteCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateVote(decoded));
            }
            if let Ok(decoded) = <CurrentVotingPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrentVotingPeriod(decoded));
            }
            if let Ok(decoded) = <ExecuteManyProposalsWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteManyProposalsWithSignature(decoded));
            }
            if let Ok(decoded) = <ExecuteProposalWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteProposalWithSignature(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetChainIdType(decoded));
            }
            if let Ok(decoded) =
                <GetProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <GovernorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Governor(decoded));
            }
            if let Ok(decoded) = <IsCorrectExecutionChainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsCorrectExecutionChain(decoded));
            }
            if let Ok(decoded) = <IsCorrectExecutionContextCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsCorrectExecutionContext(decoded));
            }
            if let Ok(decoded) =
                <IsGovernorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsGovernor(decoded));
            }
            if let Ok(decoded) = <IsSignatureFromGovernorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsSignatureFromGovernor(decoded));
            }
            if let Ok(decoded) = <IsSignatureFromGovernorPrehashedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsSignatureFromGovernorPrehashed(decoded));
            }
            if let Ok(decoded) = <LastGovernorUpdateTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastGovernorUpdateTime(decoded));
            }
            if let Ok(decoded) = <ParseChainIdFromResourceIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseChainIdFromResourceId(decoded));
            }
            if let Ok(decoded) =
                <ProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <RecoverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Recover(decoded));
            }
            if let Ok(decoded) =
                <RefreshNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RefreshNonce(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SessionLengthMultiplierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SessionLengthMultiplier(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnershipWithSignature(decoded));
            }
            if let Ok(decoded) = <VoteInFavorForceSetGovernorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VoteInFavorForceSetGovernor(decoded));
            }
            if let Ok(decoded) = <VoteInFavorForceSetGovernorWithSigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VoteInFavorForceSetGovernorWithSig(decoded));
            }
            if let Ok(decoded) =
                <VoterCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VoterCount(decoded));
            }
            if let Ok(decoded) =
                <VoterMerkleRootCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::VoterMerkleRoot(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SignatureBridgeContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EvmChainIdType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResourceIdToHandlerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminSetResourceWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AverageSessionLengthInMillisecs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchAdminSetResourceWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchExecuteProposalsWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateVote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentVotingPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteManyProposalsWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteProposalWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainIdType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Governor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsCorrectExecutionChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsCorrectExecutionContext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsGovernor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSignatureFromGovernor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSignatureFromGovernorPrehashed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastGovernorUpdateTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseChainIdFromResourceId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Recover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RefreshNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SessionLengthMultiplier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnershipWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VoteInFavorForceSetGovernor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VoteInFavorForceSetGovernorWithSig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VoterCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VoterMerkleRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SignatureBridgeContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::EvmChainIdType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResourceIdToHandlerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdminSetResourceWithSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AverageSessionLengthInMillisecs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BatchAdminSetResourceWithSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BatchExecuteProposalsWithSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateVote(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurrentVotingPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteManyProposalsWithSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteProposalWithSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainIdType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Governor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsCorrectExecutionChain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsCorrectExecutionContext(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsGovernor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsSignatureFromGovernor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsSignatureFromGovernorPrehashed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastGovernorUpdateTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseChainIdFromResourceId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Recover(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefreshNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SessionLengthMultiplier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnershipWithSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoteInFavorForceSetGovernor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoteInFavorForceSetGovernorWithSig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterMerkleRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EvmChainIdTypeCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: EvmChainIdTypeCall) -> Self {
            Self::EvmChainIdType(value)
        }
    }
    impl ::core::convert::From<ResourceIdToHandlerAddressCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: ResourceIdToHandlerAddressCall) -> Self {
            Self::ResourceIdToHandlerAddress(value)
        }
    }
    impl ::core::convert::From<AdminSetResourceWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: AdminSetResourceWithSignatureCall) -> Self {
            Self::AdminSetResourceWithSignature(value)
        }
    }
    impl ::core::convert::From<AverageSessionLengthInMillisecsCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: AverageSessionLengthInMillisecsCall) -> Self {
            Self::AverageSessionLengthInMillisecs(value)
        }
    }
    impl ::core::convert::From<BatchAdminSetResourceWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: BatchAdminSetResourceWithSignatureCall) -> Self {
            Self::BatchAdminSetResourceWithSignature(value)
        }
    }
    impl ::core::convert::From<BatchExecuteProposalsWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: BatchExecuteProposalsWithSignatureCall) -> Self {
            Self::BatchExecuteProposalsWithSignature(value)
        }
    }
    impl ::core::convert::From<CreateVoteCall> for SignatureBridgeContractCalls {
        fn from(value: CreateVoteCall) -> Self {
            Self::CreateVote(value)
        }
    }
    impl ::core::convert::From<CurrentVotingPeriodCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: CurrentVotingPeriodCall) -> Self {
            Self::CurrentVotingPeriod(value)
        }
    }
    impl ::core::convert::From<ExecuteManyProposalsWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: ExecuteManyProposalsWithSignatureCall) -> Self {
            Self::ExecuteManyProposalsWithSignature(value)
        }
    }
    impl ::core::convert::From<ExecuteProposalWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: ExecuteProposalWithSignatureCall) -> Self {
            Self::ExecuteProposalWithSignature(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for SignatureBridgeContractCalls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<GetChainIdTypeCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: GetChainIdTypeCall) -> Self {
            Self::GetChainIdType(value)
        }
    }
    impl ::core::convert::From<GetProposalNonceCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: GetProposalNonceCall) -> Self {
            Self::GetProposalNonce(value)
        }
    }
    impl ::core::convert::From<GovernorCall> for SignatureBridgeContractCalls {
        fn from(value: GovernorCall) -> Self {
            Self::Governor(value)
        }
    }
    impl ::core::convert::From<IsCorrectExecutionChainCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: IsCorrectExecutionChainCall) -> Self {
            Self::IsCorrectExecutionChain(value)
        }
    }
    impl ::core::convert::From<IsCorrectExecutionContextCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: IsCorrectExecutionContextCall) -> Self {
            Self::IsCorrectExecutionContext(value)
        }
    }
    impl ::core::convert::From<IsGovernorCall> for SignatureBridgeContractCalls {
        fn from(value: IsGovernorCall) -> Self {
            Self::IsGovernor(value)
        }
    }
    impl ::core::convert::From<IsSignatureFromGovernorCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: IsSignatureFromGovernorCall) -> Self {
            Self::IsSignatureFromGovernor(value)
        }
    }
    impl ::core::convert::From<IsSignatureFromGovernorPrehashedCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: IsSignatureFromGovernorPrehashedCall) -> Self {
            Self::IsSignatureFromGovernorPrehashed(value)
        }
    }
    impl ::core::convert::From<LastGovernorUpdateTimeCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: LastGovernorUpdateTimeCall) -> Self {
            Self::LastGovernorUpdateTime(value)
        }
    }
    impl ::core::convert::From<ParseChainIdFromResourceIdCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: ParseChainIdFromResourceIdCall) -> Self {
            Self::ParseChainIdFromResourceId(value)
        }
    }
    impl ::core::convert::From<ProposalNonceCall> for SignatureBridgeContractCalls {
        fn from(value: ProposalNonceCall) -> Self {
            Self::ProposalNonce(value)
        }
    }
    impl ::core::convert::From<RecoverCall> for SignatureBridgeContractCalls {
        fn from(value: RecoverCall) -> Self {
            Self::Recover(value)
        }
    }
    impl ::core::convert::From<RefreshNonceCall> for SignatureBridgeContractCalls {
        fn from(value: RefreshNonceCall) -> Self {
            Self::RefreshNonce(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SessionLengthMultiplierCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: SessionLengthMultiplierCall) -> Self {
            Self::SessionLengthMultiplier(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: TransferOwnershipWithSignatureCall) -> Self {
            Self::TransferOwnershipWithSignature(value)
        }
    }
    impl ::core::convert::From<VoteInFavorForceSetGovernorCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: VoteInFavorForceSetGovernorCall) -> Self {
            Self::VoteInFavorForceSetGovernor(value)
        }
    }
    impl ::core::convert::From<VoteInFavorForceSetGovernorWithSigCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: VoteInFavorForceSetGovernorWithSigCall) -> Self {
            Self::VoteInFavorForceSetGovernorWithSig(value)
        }
    }
    impl ::core::convert::From<VoterCountCall> for SignatureBridgeContractCalls {
        fn from(value: VoterCountCall) -> Self {
            Self::VoterCount(value)
        }
    }
    impl ::core::convert::From<VoterMerkleRootCall>
        for SignatureBridgeContractCalls
    {
        fn from(value: VoterMerkleRootCall) -> Self {
            Self::VoterMerkleRoot(value)
        }
    }
    ///Container type for all return fields from the `EVM_CHAIN_ID_TYPE` function with signature `EVM_CHAIN_ID_TYPE()` and selector `0x8b7e8782`
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
    pub struct EvmChainIdTypeReturn(pub [u8; 2]);
    ///Container type for all return fields from the `_resourceIdToHandlerAddress` function with signature `_resourceIdToHandlerAddress(bytes32)` and selector `0xf2dd0bb7`
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
    pub struct ResourceIdToHandlerAddressReturn(
        pub ::ethers::core::types::Address,
    );
    ///Container type for all return fields from the `averageSessionLengthInMillisecs` function with signature `averageSessionLengthInMillisecs()` and selector `0x016737bb`
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
    pub struct AverageSessionLengthInMillisecsReturn(pub u64);
    ///Container type for all return fields from the `createVote` function with signature `createVote(uint32,address,bytes32[])` and selector `0xfecb9553`
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
    pub struct CreateVoteReturn(pub Vote);
    ///Container type for all return fields from the `currentVotingPeriod` function with signature `currentVotingPeriod()` and selector `0x3a049e02`
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
    pub struct CurrentVotingPeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    pub struct GetChainIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getChainIdType` function with signature `getChainIdType()` and selector `0x4c830cbd`
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
    pub struct GetChainIdTypeReturn(pub u64);
    ///Container type for all return fields from the `getProposalNonce` function with signature `getProposalNonce()` and selector `0x0b27fb9a`
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
    pub struct GetProposalNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `governor` function with signature `governor()` and selector `0x0c340a24`
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
    pub struct GovernorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isCorrectExecutionChain` function with signature `isCorrectExecutionChain(bytes32)` and selector `0x830b2f57`
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
    pub struct IsCorrectExecutionChainReturn(pub bool);
    ///Container type for all return fields from the `isCorrectExecutionContext` function with signature `isCorrectExecutionContext(bytes32)` and selector `0xf5fc3d6b`
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
    pub struct IsCorrectExecutionContextReturn(pub bool);
    ///Container type for all return fields from the `isGovernor` function with signature `isGovernor()` and selector `0xc7af3352`
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
    pub struct IsGovernorReturn(pub bool);
    ///Container type for all return fields from the `isSignatureFromGovernor` function with signature `isSignatureFromGovernor(bytes,bytes)` and selector `0x8755bcad`
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
    pub struct IsSignatureFromGovernorReturn(pub bool);
    ///Container type for all return fields from the `isSignatureFromGovernorPrehashed` function with signature `isSignatureFromGovernorPrehashed(bytes32,bytes)` and selector `0x6c6f4846`
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
    pub struct IsSignatureFromGovernorPrehashedReturn(pub bool);
    ///Container type for all return fields from the `lastGovernorUpdateTime` function with signature `lastGovernorUpdateTime()` and selector `0x9e09583c`
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
    pub struct LastGovernorUpdateTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `parseChainIdFromResourceId` function with signature `parseChainIdFromResourceId(bytes32)` and selector `0xc2230d6e`
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
    pub struct ParseChainIdFromResourceIdReturn(pub u64);
    ///Container type for all return fields from the `proposalNonce` function with signature `proposalNonce()` and selector `0xcc3c74a1`
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
    pub struct ProposalNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `recover` function with signature `recover(bytes,bytes)` and selector `0x1ed13d1b`
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
    pub struct RecoverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `refreshNonce` function with signature `refreshNonce()` and selector `0x13cb01f9`
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
    pub struct RefreshNonceReturn(pub u32);
    ///Container type for all return fields from the `sessionLengthMultiplier` function with signature `sessionLengthMultiplier()` and selector `0xbdfadc84`
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
    pub struct SessionLengthMultiplierReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `voterCount` function with signature `voterCount()` and selector `0x42169e48`
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
    pub struct VoterCountReturn(pub u32);
    ///Container type for all return fields from the `voterMerkleRoot` function with signature `voterMerkleRoot()` and selector `0x2a69fb46`
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
    pub struct VoterMerkleRootReturn(pub [u8; 32]);
    ///`Vote(uint32,address,bytes32[])`
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
    pub struct Vote {
        pub leaf_index: u32,
        pub proposed_governor: ::ethers::core::types::Address,
        pub sibling_path_nodes: ::std::vec::Vec<[u8; 32]>,
    }
}
