pub use multi_asset_v_anchor_contract::*;
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
pub mod multi_asset_v_anchor_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                    ::std::borrow::ToOwned::to_owned("FIELD_SIZE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FIELD_SIZE"),
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
                    ::std::borrow::ToOwned::to_owned("MAX_EXT_AMOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_EXT_AMOUNT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_FEE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_FEE"),
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
                    ::std::borrow::ToOwned::to_owned("ROOT_HISTORY_SIZE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ROOT_HISTORY_SIZE"),
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
                    ::std::borrow::ToOwned::to_owned("UPDATE_MAX_LENGTH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("UPDATE_MAX_LENGTH"),
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
                    ::std::borrow::ToOwned::to_owned("ZERO_VALUE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ZERO_VALUE"),
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
                    ::std::borrow::ToOwned::to_owned("_genExtDataHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_genExtDataHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_auxPublicInputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_externalData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct CommonExtData"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_encryptions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Encryptions"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculatePublicAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculatePublicAmount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("commitments"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("commitments"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("configureMaximumDepositLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "configureMaximumDepositLimit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_maximumDepositAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
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
                    ::std::borrow::ToOwned::to_owned("configureMinimumWithdrawalLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "configureMinimumWithdrawalLimit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_minimumWithdrawalAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
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
                    ::std::borrow::ToOwned::to_owned("currentNeighborRootIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentNeighborRootIndex",
                            ),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("edgeExistsForChain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("edgeExistsForChain"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("edgeIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("edgeIndex"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("edgeList"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("edgeList"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("root"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("latestLeafIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("srcResourceID"),
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
                (
                    ::std::borrow::ToOwned::to_owned("filledSubtrees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("filledSubtrees"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("getHasher"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getHasher"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IHasher"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLastRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLastRoot"),
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
                    ::std::borrow::ToOwned::to_owned("getLatestNeighborEdges"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLatestNeighborEdges",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Edge[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLatestNeighborRoots"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLatestNeighborRoots",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLevels"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLevels"),
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
                    ::std::borrow::ToOwned::to_owned("getNextIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextIndex"),
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
                    ::std::borrow::ToOwned::to_owned("getZeroHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getZeroHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("handler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("handler"),
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
                    ::std::borrow::ToOwned::to_owned("hasEdge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasEdge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("hashLeftRight"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hashLeftRight"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_left"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_right"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_minimumWithdrawalAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_maximumDepositAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialized"),
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
                    ::std::borrow::ToOwned::to_owned("isKnownNeighborRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isKnownNeighborRoot",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_neighborChainID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_root"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("isKnownRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isKnownRoot"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("root"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("isSpent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isSpent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nullifierHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("isSpentArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isSpentArray"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nullifierHashes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isValidRoots"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isValidRoots"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_roots"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                    ::std::borrow::ToOwned::to_owned("lastBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastBalance"),
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
                    ::std::borrow::ToOwned::to_owned("maxEdges"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxEdges"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maximumDepositAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "maximumDepositAmount",
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
                    ::std::borrow::ToOwned::to_owned("minimumWithdrawalAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minimumWithdrawalAmount",
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
                    ::std::borrow::ToOwned::to_owned("neighborRoots"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("neighborRoots"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("nullifierHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nullifierHashes"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("outerLevels"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("outerLevels"),
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
                    ::std::borrow::ToOwned::to_owned("register"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("register"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct VAnchorBase.Account",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("registerAndTransact"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerAndTransact",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct VAnchorBase.Account",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_auxPublicInputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_externalData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct CommonExtData"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_publicInputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct PublicInputs"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_encryptions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Encryptions"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registry"),
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
                    ::std::borrow::ToOwned::to_owned("roots"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("roots"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("root"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("latestLeafindex"),
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
                    ::std::borrow::ToOwned::to_owned("setHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setHandler"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_handler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
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
                    ::std::borrow::ToOwned::to_owned("setVerifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setVerifier"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_verifier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
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
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_publicInputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SwapPublicInputs"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aliceEncryptions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Encryptions"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bobEncryptions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Encryptions"),
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
                    ::std::borrow::ToOwned::to_owned("transact"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transact"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_auxPublicInputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_externalData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct CommonExtData"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_publicInputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct PublicInputs"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_encryptions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Encryptions"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpackProof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpackProof"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        8usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[8]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        2usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[2]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                        ),
                                        2usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[2][2]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        2usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[2]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateEdge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateEdge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_root"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_leafIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcResourceID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifier"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IAnchorVerifier"),
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
                    ::std::borrow::ToOwned::to_owned("EdgeAddition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EdgeAddition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("latestLeafIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("merkleRoot"),
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
                    ::std::borrow::ToOwned::to_owned("EdgeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EdgeUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("latestLeafIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("merkleRoot"),
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
                    ::std::borrow::ToOwned::to_owned("Insertion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Insertion"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("commitment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("leafIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMerkleRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MaxDepositLimitUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MaxDepositLimitUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "maximumDepositAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinWithdrawalLimitUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MinWithdrawalLimitUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "minimumWithdrawalAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewCommitment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewCommitment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("commitment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subTreeIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("leafIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("encryptedOutput"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewNullifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewNullifier"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nullifier"),
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
                    ::std::borrow::ToOwned::to_owned("PublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PublicKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetHandler"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("handler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetVerifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetVerifier"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("verifier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
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
    pub static MULTIASSETVANCHORCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct MultiAssetVAnchorContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MultiAssetVAnchorContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MultiAssetVAnchorContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MultiAssetVAnchorContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MultiAssetVAnchorContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MultiAssetVAnchorContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MultiAssetVAnchorContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MULTIASSETVANCHORCONTRACT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `EVM_CHAIN_ID_TYPE` (0x8b7e8782) function
        pub fn evm_chain_id_type(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 2]> {
            self.0
                .method_hash([139, 126, 135, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FIELD_SIZE` (0x414a37ba) function
        pub fn field_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([65, 74, 55, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_EXT_AMOUNT` (0x7fe24ffe) function
        pub fn max_ext_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::I256,
        > {
            self.0
                .method_hash([127, 226, 79, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_FEE` (0xbc063e1a) function
        pub fn max_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([188, 6, 62, 26], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ROOT_HISTORY_SIZE` (0xcd87a3b4) function
        pub fn root_history_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([205, 135, 163, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UPDATE_MAX_LENGTH` (0xdbc9d395) function
        pub fn update_max_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([219, 201, 211, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ZERO_VALUE` (0xec732959) function
        pub fn zero_value(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([236, 115, 41, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_genExtDataHash` (0xd384534d) function
        pub fn gen_ext_data_hash(
            &self,
            aux_public_inputs: ::ethers::core::types::Bytes,
            external_data: CommonExtData,
            encryptions: Encryptions,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [211, 132, 83, 77],
                    (aux_public_inputs, external_data, encryptions),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculatePublicAmount` (0x2570b7b4) function
        pub fn calculate_public_amount(
            &self,
            ext_amount: ::ethers::core::types::I256,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([37, 112, 183, 180], (ext_amount, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitments` (0x49ce8997) function
        pub fn commitments(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 206, 137, 151], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configureMaximumDepositLimit` (0x8c832b13) function
        pub fn configure_maximum_deposit_limit(
            &self,
            maximum_deposit_amount: ::ethers::core::types::U256,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [140, 131, 43, 19],
                    (maximum_deposit_amount, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configureMinimumWithdrawalLimit` (0x0b97e0a8) function
        pub fn configure_minimum_withdrawal_limit(
            &self,
            minimum_withdrawal_amount: ::ethers::core::types::U256,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [11, 151, 224, 168],
                    (minimum_withdrawal_amount, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentNeighborRootIndex` (0x5d2d766c) function
        pub fn current_neighbor_root_index(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([93, 45, 118, 108], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `edgeExistsForChain` (0xfa731687) function
        pub fn edge_exists_for_chain(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 115, 22, 135], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `edgeIndex` (0xe70ea87c) function
        pub fn edge_index(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([231, 14, 168, 124], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `edgeList` (0xdbc916b8) function
        pub fn edge_list(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([219, 201, 22, 184], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `filledSubtrees` (0xf178e47c) function
        pub fn filled_subtrees(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([241, 120, 228, 124], p0)
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
        ///Calls the contract's `getHasher` (0xea495db0) function
        pub fn get_hasher(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([234, 73, 93, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLastRoot` (0xba70f757) function
        pub fn get_last_root(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([186, 112, 247, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatestNeighborEdges` (0x8c0d34d8) function
        pub fn get_latest_neighbor_edges(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Edge>>
        {
            self.0
                .method_hash([140, 13, 52, 216], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatestNeighborRoots` (0x1e627617) function
        pub fn get_latest_neighbor_roots(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([30, 98, 118, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLevels` (0x0c394a60) function
        pub fn get_levels(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([12, 57, 74, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextIndex` (0x0eb7606f) function
        pub fn get_next_index(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([14, 183, 96, 111], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getZeroHash` (0x305e9eac) function
        pub fn get_zero_hash(
            &self,
            index: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([48, 94, 158, 172], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `handler` (0xc80916d4) function
        pub fn handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 9, 22, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasEdge` (0x92156311) function
        pub fn has_edge(
            &self,
            chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 21, 99, 17], chain_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashLeftRight` (0x5bb93995) function
        pub fn hash_left_right(
            &self,
            left: ::ethers::core::types::U256,
            right: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([91, 185, 57, 149], (left, right))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xe4a30116) function
        pub fn initialize(
            &self,
            minimum_withdrawal_amount: ::ethers::core::types::U256,
            maximum_deposit_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [228, 163, 1, 22],
                    (minimum_withdrawal_amount, maximum_deposit_amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialized` (0x158ef93e) function
        pub fn initialized(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 142, 249, 62], ())
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
        ///Calls the contract's `isKnownNeighborRoot` (0x3bfa8d7a) function
        pub fn is_known_neighbor_root(
            &self,
            neighbor_chain_id: ::ethers::core::types::U256,
            root: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 250, 141, 122], (neighbor_chain_id, root))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isKnownRoot` (0xa6232a93) function
        pub fn is_known_root(
            &self,
            root: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 35, 42, 147], root)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSpent` (0x5a129efe) function
        pub fn is_spent(
            &self,
            nullifier_hash: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 18, 158, 254], nullifier_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSpentArray` (0xea65ba49) function
        pub fn is_spent_array(
            &self,
            nullifier_hashes: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>>
        {
            self.0
                .method_hash([234, 101, 186, 73], nullifier_hashes)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidRoots` (0xb75e6798) function
        pub fn is_valid_roots(
            &self,
            roots: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([183, 94, 103, 152], roots)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastBalance` (0x8f1c56bd) function
        pub fn last_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([143, 28, 86, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxEdges` (0x71523c32) function
        pub fn max_edges(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([113, 82, 60, 50], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maximumDepositAmount` (0x78abb49b) function
        pub fn maximum_deposit_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([120, 171, 180, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minimumWithdrawalAmount` (0x2b180646) function
        pub fn minimum_withdrawal_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([43, 24, 6, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `neighborRoots` (0x43e7119f) function
        pub fn neighbor_roots(
            &self,
            p0: ::ethers::core::types::U256,
            p1: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([67, 231, 17, 159], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nullifierHashes` (0x1f79a1e9) function
        pub fn nullifier_hashes(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([31, 121, 161, 233], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            operator: ::ethers::core::types::Address,
            from: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (operator, from, token_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `outerLevels` (0xbfbc0a39) function
        pub fn outer_levels(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([191, 188, 10, 57], ())
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
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([204, 60, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `register` (0xb2bc6e0f) function
        pub fn register(
            &self,
            account: Account,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 188, 110, 15], (account,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerAndTransact` (0x38a26a09) function
        pub fn register_and_transact(
            &self,
            account: Account,
            proof: ::ethers::core::types::Bytes,
            aux_public_inputs: ::ethers::core::types::Bytes,
            external_data: CommonExtData,
            public_inputs: PublicInputs,
            encryptions: Encryptions,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [56, 162, 106, 9],
                    (
                        account,
                        proof,
                        aux_public_inputs,
                        external_data,
                        public_inputs,
                        encryptions,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registry` (0x7b103999) function
        pub fn registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([123, 16, 57, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `roots` (0xc2b40ae4) function
        pub fn roots(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, u32),
        > {
            self.0
                .method_hash([194, 180, 10, 228], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHandler` (0x72c1ad03) function
        pub fn set_handler(
            &self,
            handler: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 193, 173, 3], (handler, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setVerifier` (0xa0d192f5) function
        pub fn set_verifier(
            &self,
            verifier: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 209, 146, 245], (verifier, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x236d421d) function
        pub fn swap(
            &self,
            proof: ::ethers::core::types::Bytes,
            public_inputs: SwapPublicInputs,
            alice_encryptions: Encryptions,
            bob_encryptions: Encryptions,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [35, 109, 66, 29],
                    (proof, public_inputs, alice_encryptions, bob_encryptions),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transact` (0xa38f76e8) function
        pub fn transact(
            &self,
            proof: ::ethers::core::types::Bytes,
            aux_public_inputs: ::ethers::core::types::Bytes,
            external_data: CommonExtData,
            public_inputs: PublicInputs,
            encryptions: Encryptions,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [163, 143, 118, 232],
                    (
                        proof,
                        aux_public_inputs,
                        external_data,
                        public_inputs,
                        encryptions,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpackProof` (0xf5ab0dd6) function
        pub fn unpack_proof(
            &self,
            proof: [::ethers::core::types::U256; 8],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                [::ethers::core::types::U256; 2],
                [[::ethers::core::types::U256; 2]; 2],
                [::ethers::core::types::U256; 2],
            ),
        > {
            self.0
                .method_hash([245, 171, 13, 214], proof)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateEdge` (0xc1922f9e) function
        pub fn update_edge(
            &self,
            root: ::ethers::core::types::U256,
            leaf_index: u32,
            src_resource_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [193, 146, 47, 158],
                    (root, leaf_index, src_resource_id),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifier` (0x2b7ac3f3) function
        pub fn verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([43, 122, 195, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EdgeAddition` event
        pub fn edge_addition_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EdgeAdditionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EdgeUpdate` event
        pub fn edge_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EdgeUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Insertion` event
        pub fn insertion_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InsertionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaxDepositLimitUpdated` event
        pub fn max_deposit_limit_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxDepositLimitUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinWithdrawalLimitUpdated` event
        pub fn min_withdrawal_limit_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinWithdrawalLimitUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewCommitment` event
        pub fn new_commitment_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewCommitmentFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewNullifier` event
        pub fn new_nullifier_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewNullifierFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PublicKey` event
        pub fn public_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PublicKeyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetHandler` event
        pub fn set_handler_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetHandlerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetVerifier` event
        pub fn set_verifier_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetVerifierFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MultiAssetVAnchorContractEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for MultiAssetVAnchorContract<M>
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
        name = "EdgeAddition",
        abi = "EdgeAddition(uint256,uint256,uint256)"
    )]
    pub struct EdgeAdditionFilter {
        pub chain_id: ::ethers::core::types::U256,
        pub latest_leaf_index: ::ethers::core::types::U256,
        pub merkle_root: ::ethers::core::types::U256,
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
        name = "EdgeUpdate",
        abi = "EdgeUpdate(uint256,uint256,uint256)"
    )]
    pub struct EdgeUpdateFilter {
        pub chain_id: ::ethers::core::types::U256,
        pub latest_leaf_index: ::ethers::core::types::U256,
        pub merkle_root: ::ethers::core::types::U256,
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
        name = "Insertion",
        abi = "Insertion(uint256,uint32,uint256,uint256)"
    )]
    pub struct InsertionFilter {
        #[ethevent(indexed)]
        pub commitment: ::ethers::core::types::U256,
        pub leaf_index: u32,
        pub timestamp: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub new_merkle_root: ::ethers::core::types::U256,
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
        name = "MaxDepositLimitUpdated",
        abi = "MaxDepositLimitUpdated(uint256,uint32)"
    )]
    pub struct MaxDepositLimitUpdatedFilter {
        pub maximum_deposit_amount: ::ethers::core::types::U256,
        pub nonce: u32,
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
        name = "MinWithdrawalLimitUpdated",
        abi = "MinWithdrawalLimitUpdated(uint256,uint32)"
    )]
    pub struct MinWithdrawalLimitUpdatedFilter {
        pub minimum_withdrawal_amount: ::ethers::core::types::U256,
        pub nonce: u32,
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
        name = "NewCommitment",
        abi = "NewCommitment(uint256,uint256,uint256,bytes)"
    )]
    pub struct NewCommitmentFilter {
        pub commitment: ::ethers::core::types::U256,
        pub sub_tree_index: ::ethers::core::types::U256,
        pub leaf_index: ::ethers::core::types::U256,
        pub encrypted_output: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "NewNullifier", abi = "NewNullifier(uint256)")]
    pub struct NewNullifierFilter {
        pub nullifier: ::ethers::core::types::U256,
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
    #[ethevent(name = "PublicKey", abi = "PublicKey(address,bytes)")]
    pub struct PublicKeyFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub key: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "SetHandler", abi = "SetHandler(address,uint32)")]
    pub struct SetHandlerFilter {
        pub handler: ::ethers::core::types::Address,
        pub nonce: u32,
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
    #[ethevent(name = "SetVerifier", abi = "SetVerifier(address,uint32)")]
    pub struct SetVerifierFilter {
        pub verifier: ::ethers::core::types::Address,
        pub nonce: u32,
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
    pub enum MultiAssetVAnchorContractEvents {
        EdgeAdditionFilter(EdgeAdditionFilter),
        EdgeUpdateFilter(EdgeUpdateFilter),
        InsertionFilter(InsertionFilter),
        MaxDepositLimitUpdatedFilter(MaxDepositLimitUpdatedFilter),
        MinWithdrawalLimitUpdatedFilter(MinWithdrawalLimitUpdatedFilter),
        NewCommitmentFilter(NewCommitmentFilter),
        NewNullifierFilter(NewNullifierFilter),
        PublicKeyFilter(PublicKeyFilter),
        SetHandlerFilter(SetHandlerFilter),
        SetVerifierFilter(SetVerifierFilter),
    }
    impl ::ethers::contract::EthLogDecode for MultiAssetVAnchorContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EdgeAdditionFilter::decode_log(log) {
                return Ok(
                    MultiAssetVAnchorContractEvents::EdgeAdditionFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = EdgeUpdateFilter::decode_log(log) {
                return Ok(MultiAssetVAnchorContractEvents::EdgeUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InsertionFilter::decode_log(log) {
                return Ok(MultiAssetVAnchorContractEvents::InsertionFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = MaxDepositLimitUpdatedFilter::decode_log(log) {
                return Ok(
                    MultiAssetVAnchorContractEvents::MaxDepositLimitUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                MinWithdrawalLimitUpdatedFilter::decode_log(log)
            {
                return Ok(
                    MultiAssetVAnchorContractEvents::MinWithdrawalLimitUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = NewCommitmentFilter::decode_log(log) {
                return Ok(
                    MultiAssetVAnchorContractEvents::NewCommitmentFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = NewNullifierFilter::decode_log(log) {
                return Ok(
                    MultiAssetVAnchorContractEvents::NewNullifierFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PublicKeyFilter::decode_log(log) {
                return Ok(MultiAssetVAnchorContractEvents::PublicKeyFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SetHandlerFilter::decode_log(log) {
                return Ok(MultiAssetVAnchorContractEvents::SetHandlerFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SetVerifierFilter::decode_log(log) {
                return Ok(MultiAssetVAnchorContractEvents::SetVerifierFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MultiAssetVAnchorContractEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::EdgeAdditionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EdgeUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsertionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxDepositLimitUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinWithdrawalLimitUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewCommitmentFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewNullifierFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PublicKeyFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetHandlerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetVerifierFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EdgeAdditionFilter>
        for MultiAssetVAnchorContractEvents
    {
        fn from(value: EdgeAdditionFilter) -> Self {
            Self::EdgeAdditionFilter(value)
        }
    }
    impl ::core::convert::From<EdgeUpdateFilter>
        for MultiAssetVAnchorContractEvents
    {
        fn from(value: EdgeUpdateFilter) -> Self {
            Self::EdgeUpdateFilter(value)
        }
    }
    impl ::core::convert::From<InsertionFilter>
        for MultiAssetVAnchorContractEvents
    {
        fn from(value: InsertionFilter) -> Self {
            Self::InsertionFilter(value)
        }
    }
    impl ::core::convert::From<MaxDepositLimitUpdatedFilter>
        for MultiAssetVAnchorContractEvents
    {
        fn from(value: MaxDepositLimitUpdatedFilter) -> Self {
            Self::MaxDepositLimitUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinWithdrawalLimitUpdatedFilter>
        for MultiAssetVAnchorContractEvents
    {
        fn from(value: MinWithdrawalLimitUpdatedFilter) -> Self {
            Self::MinWithdrawalLimitUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<NewCommitmentFilter>
        for MultiAssetVAnchorContractEvents
    {
        fn from(value: NewCommitmentFilter) -> Self {
            Self::NewCommitmentFilter(value)
        }
    }
    impl ::core::convert::From<NewNullifierFilter>
        for MultiAssetVAnchorContractEvents
    {
        fn from(value: NewNullifierFilter) -> Self {
            Self::NewNullifierFilter(value)
        }
    }
    impl ::core::convert::From<PublicKeyFilter>
        for MultiAssetVAnchorContractEvents
    {
        fn from(value: PublicKeyFilter) -> Self {
            Self::PublicKeyFilter(value)
        }
    }
    impl ::core::convert::From<SetHandlerFilter>
        for MultiAssetVAnchorContractEvents
    {
        fn from(value: SetHandlerFilter) -> Self {
            Self::SetHandlerFilter(value)
        }
    }
    impl ::core::convert::From<SetVerifierFilter>
        for MultiAssetVAnchorContractEvents
    {
        fn from(value: SetVerifierFilter) -> Self {
            Self::SetVerifierFilter(value)
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
    ///Container type for all input parameters for the `FIELD_SIZE` function with signature `FIELD_SIZE()` and selector `0x414a37ba`
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
    #[ethcall(name = "FIELD_SIZE", abi = "FIELD_SIZE()")]
    pub struct FieldSizeCall;
    ///Container type for all input parameters for the `MAX_EXT_AMOUNT` function with signature `MAX_EXT_AMOUNT()` and selector `0x7fe24ffe`
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
    #[ethcall(name = "MAX_EXT_AMOUNT", abi = "MAX_EXT_AMOUNT()")]
    pub struct MaxExtAmountCall;
    ///Container type for all input parameters for the `MAX_FEE` function with signature `MAX_FEE()` and selector `0xbc063e1a`
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
    #[ethcall(name = "MAX_FEE", abi = "MAX_FEE()")]
    pub struct MaxFeeCall;
    ///Container type for all input parameters for the `ROOT_HISTORY_SIZE` function with signature `ROOT_HISTORY_SIZE()` and selector `0xcd87a3b4`
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
    #[ethcall(name = "ROOT_HISTORY_SIZE", abi = "ROOT_HISTORY_SIZE()")]
    pub struct RootHistorySizeCall;
    ///Container type for all input parameters for the `UPDATE_MAX_LENGTH` function with signature `UPDATE_MAX_LENGTH()` and selector `0xdbc9d395`
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
    #[ethcall(name = "UPDATE_MAX_LENGTH", abi = "UPDATE_MAX_LENGTH()")]
    pub struct UpdateMaxLengthCall;
    ///Container type for all input parameters for the `ZERO_VALUE` function with signature `ZERO_VALUE()` and selector `0xec732959`
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
    #[ethcall(name = "ZERO_VALUE", abi = "ZERO_VALUE()")]
    pub struct ZeroValueCall;
    ///Container type for all input parameters for the `_genExtDataHash` function with signature `_genExtDataHash(bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes))` and selector `0xd384534d`
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
        name = "_genExtDataHash",
        abi = "_genExtDataHash(bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes))"
    )]
    pub struct GenExtDataHashCall {
        pub aux_public_inputs: ::ethers::core::types::Bytes,
        pub external_data: CommonExtData,
        pub encryptions: Encryptions,
    }
    ///Container type for all input parameters for the `calculatePublicAmount` function with signature `calculatePublicAmount(int256,uint256)` and selector `0x2570b7b4`
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
        name = "calculatePublicAmount",
        abi = "calculatePublicAmount(int256,uint256)"
    )]
    pub struct CalculatePublicAmountCall {
        pub ext_amount: ::ethers::core::types::I256,
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `commitments` function with signature `commitments(uint256)` and selector `0x49ce8997`
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
    #[ethcall(name = "commitments", abi = "commitments(uint256)")]
    pub struct CommitmentsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `configureMaximumDepositLimit` function with signature `configureMaximumDepositLimit(uint256,uint32)` and selector `0x8c832b13`
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
        name = "configureMaximumDepositLimit",
        abi = "configureMaximumDepositLimit(uint256,uint32)"
    )]
    pub struct ConfigureMaximumDepositLimitCall {
        pub maximum_deposit_amount: ::ethers::core::types::U256,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `configureMinimumWithdrawalLimit` function with signature `configureMinimumWithdrawalLimit(uint256,uint32)` and selector `0x0b97e0a8`
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
        name = "configureMinimumWithdrawalLimit",
        abi = "configureMinimumWithdrawalLimit(uint256,uint32)"
    )]
    pub struct ConfigureMinimumWithdrawalLimitCall {
        pub minimum_withdrawal_amount: ::ethers::core::types::U256,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `currentNeighborRootIndex` function with signature `currentNeighborRootIndex(uint256)` and selector `0x5d2d766c`
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
        name = "currentNeighborRootIndex",
        abi = "currentNeighborRootIndex(uint256)"
    )]
    pub struct CurrentNeighborRootIndexCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `edgeExistsForChain` function with signature `edgeExistsForChain(uint256)` and selector `0xfa731687`
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
    #[ethcall(name = "edgeExistsForChain", abi = "edgeExistsForChain(uint256)")]
    pub struct EdgeExistsForChainCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `edgeIndex` function with signature `edgeIndex(uint256)` and selector `0xe70ea87c`
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
    #[ethcall(name = "edgeIndex", abi = "edgeIndex(uint256)")]
    pub struct EdgeIndexCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `edgeList` function with signature `edgeList(uint256)` and selector `0xdbc916b8`
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
    #[ethcall(name = "edgeList", abi = "edgeList(uint256)")]
    pub struct EdgeListCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `filledSubtrees` function with signature `filledSubtrees(uint256)` and selector `0xf178e47c`
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
    #[ethcall(name = "filledSubtrees", abi = "filledSubtrees(uint256)")]
    pub struct FilledSubtreesCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `getHasher` function with signature `getHasher()` and selector `0xea495db0`
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
    #[ethcall(name = "getHasher", abi = "getHasher()")]
    pub struct GetHasherCall;
    ///Container type for all input parameters for the `getLastRoot` function with signature `getLastRoot()` and selector `0xba70f757`
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
    #[ethcall(name = "getLastRoot", abi = "getLastRoot()")]
    pub struct GetLastRootCall;
    ///Container type for all input parameters for the `getLatestNeighborEdges` function with signature `getLatestNeighborEdges()` and selector `0x8c0d34d8`
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
        name = "getLatestNeighborEdges",
        abi = "getLatestNeighborEdges()"
    )]
    pub struct GetLatestNeighborEdgesCall;
    ///Container type for all input parameters for the `getLatestNeighborRoots` function with signature `getLatestNeighborRoots()` and selector `0x1e627617`
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
        name = "getLatestNeighborRoots",
        abi = "getLatestNeighborRoots()"
    )]
    pub struct GetLatestNeighborRootsCall;
    ///Container type for all input parameters for the `getLevels` function with signature `getLevels()` and selector `0x0c394a60`
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
    #[ethcall(name = "getLevels", abi = "getLevels()")]
    pub struct GetLevelsCall;
    ///Container type for all input parameters for the `getNextIndex` function with signature `getNextIndex()` and selector `0x0eb7606f`
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
    #[ethcall(name = "getNextIndex", abi = "getNextIndex()")]
    pub struct GetNextIndexCall;
    ///Container type for all input parameters for the `getZeroHash` function with signature `getZeroHash(uint32)` and selector `0x305e9eac`
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
    #[ethcall(name = "getZeroHash", abi = "getZeroHash(uint32)")]
    pub struct GetZeroHashCall {
        pub index: u32,
    }
    ///Container type for all input parameters for the `handler` function with signature `handler()` and selector `0xc80916d4`
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
    #[ethcall(name = "handler", abi = "handler()")]
    pub struct HandlerCall;
    ///Container type for all input parameters for the `hasEdge` function with signature `hasEdge(uint256)` and selector `0x92156311`
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
    #[ethcall(name = "hasEdge", abi = "hasEdge(uint256)")]
    pub struct HasEdgeCall {
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `hashLeftRight` function with signature `hashLeftRight(uint256,uint256)` and selector `0x5bb93995`
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
    #[ethcall(name = "hashLeftRight", abi = "hashLeftRight(uint256,uint256)")]
    pub struct HashLeftRightCall {
        pub left: ::ethers::core::types::U256,
        pub right: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint256,uint256)` and selector `0xe4a30116`
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
    #[ethcall(name = "initialize", abi = "initialize(uint256,uint256)")]
    pub struct InitializeCall {
        pub minimum_withdrawal_amount: ::ethers::core::types::U256,
        pub maximum_deposit_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialized` function with signature `initialized()` and selector `0x158ef93e`
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
    #[ethcall(name = "initialized", abi = "initialized()")]
    pub struct InitializedCall;
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
    ///Container type for all input parameters for the `isKnownNeighborRoot` function with signature `isKnownNeighborRoot(uint256,uint256)` and selector `0x3bfa8d7a`
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
        name = "isKnownNeighborRoot",
        abi = "isKnownNeighborRoot(uint256,uint256)"
    )]
    pub struct IsKnownNeighborRootCall {
        pub neighbor_chain_id: ::ethers::core::types::U256,
        pub root: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `0xa6232a93`
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
    #[ethcall(name = "isKnownRoot", abi = "isKnownRoot(uint256)")]
    pub struct IsKnownRootCall {
        pub root: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isSpent` function with signature `isSpent(uint256)` and selector `0x5a129efe`
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
    #[ethcall(name = "isSpent", abi = "isSpent(uint256)")]
    pub struct IsSpentCall {
        pub nullifier_hash: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isSpentArray` function with signature `isSpentArray(uint256[])` and selector `0xea65ba49`
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
    #[ethcall(name = "isSpentArray", abi = "isSpentArray(uint256[])")]
    pub struct IsSpentArrayCall {
        pub nullifier_hashes: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `isValidRoots` function with signature `isValidRoots(uint256[])` and selector `0xb75e6798`
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
    #[ethcall(name = "isValidRoots", abi = "isValidRoots(uint256[])")]
    pub struct IsValidRootsCall {
        pub roots: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `lastBalance` function with signature `lastBalance()` and selector `0x8f1c56bd`
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
    #[ethcall(name = "lastBalance", abi = "lastBalance()")]
    pub struct LastBalanceCall;
    ///Container type for all input parameters for the `maxEdges` function with signature `maxEdges()` and selector `0x71523c32`
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
    #[ethcall(name = "maxEdges", abi = "maxEdges()")]
    pub struct MaxEdgesCall;
    ///Container type for all input parameters for the `maximumDepositAmount` function with signature `maximumDepositAmount()` and selector `0x78abb49b`
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
    #[ethcall(name = "maximumDepositAmount", abi = "maximumDepositAmount()")]
    pub struct MaximumDepositAmountCall;
    ///Container type for all input parameters for the `minimumWithdrawalAmount` function with signature `minimumWithdrawalAmount()` and selector `0x2b180646`
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
        name = "minimumWithdrawalAmount",
        abi = "minimumWithdrawalAmount()"
    )]
    pub struct MinimumWithdrawalAmountCall;
    ///Container type for all input parameters for the `neighborRoots` function with signature `neighborRoots(uint256,uint32)` and selector `0x43e7119f`
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
    #[ethcall(name = "neighborRoots", abi = "neighborRoots(uint256,uint32)")]
    pub struct NeighborRootsCall(pub ::ethers::core::types::U256, pub u32);
    ///Container type for all input parameters for the `nullifierHashes` function with signature `nullifierHashes(uint256)` and selector `0x1f79a1e9`
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
    #[ethcall(name = "nullifierHashes", abi = "nullifierHashes(uint256)")]
    pub struct NullifierHashesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall {
        pub operator: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `outerLevels` function with signature `outerLevels()` and selector `0xbfbc0a39`
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
    #[ethcall(name = "outerLevels", abi = "outerLevels()")]
    pub struct OuterLevelsCall;
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
    ///Container type for all input parameters for the `register` function with signature `register((address,bytes))` and selector `0xb2bc6e0f`
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
    #[ethcall(name = "register", abi = "register((address,bytes))")]
    pub struct RegisterCall {
        pub account: Account,
    }
    ///Container type for all input parameters for the `registerAndTransact` function with signature `registerAndTransact((address,bytes),bytes,bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes,uint256[],uint256[2],uint256,uint256),(bytes,bytes))` and selector `0x38a26a09`
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
        name = "registerAndTransact",
        abi = "registerAndTransact((address,bytes),bytes,bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes,uint256[],uint256[2],uint256,uint256),(bytes,bytes))"
    )]
    pub struct RegisterAndTransactCall {
        pub account: Account,
        pub proof: ::ethers::core::types::Bytes,
        pub aux_public_inputs: ::ethers::core::types::Bytes,
        pub external_data: CommonExtData,
        pub public_inputs: PublicInputs,
        pub encryptions: Encryptions,
    }
    ///Container type for all input parameters for the `registry` function with signature `registry()` and selector `0x7b103999`
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
    #[ethcall(name = "registry", abi = "registry()")]
    pub struct RegistryCall;
    ///Container type for all input parameters for the `roots` function with signature `roots(uint256)` and selector `0xc2b40ae4`
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
    #[ethcall(name = "roots", abi = "roots(uint256)")]
    pub struct RootsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `setHandler` function with signature `setHandler(address,uint32)` and selector `0x72c1ad03`
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
    #[ethcall(name = "setHandler", abi = "setHandler(address,uint32)")]
    pub struct SetHandlerCall {
        pub handler: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `setVerifier` function with signature `setVerifier(address,uint32)` and selector `0xa0d192f5`
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
    #[ethcall(name = "setVerifier", abi = "setVerifier(address,uint32)")]
    pub struct SetVerifierCall {
        pub verifier: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap(bytes,(bytes,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),(bytes,bytes),(bytes,bytes))` and selector `0x236d421d`
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
        name = "swap",
        abi = "swap(bytes,(bytes,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256),(bytes,bytes),(bytes,bytes))"
    )]
    pub struct SwapCall {
        pub proof: ::ethers::core::types::Bytes,
        pub public_inputs: SwapPublicInputs,
        pub alice_encryptions: Encryptions,
        pub bob_encryptions: Encryptions,
    }
    ///Container type for all input parameters for the `transact` function with signature `transact(bytes,bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes,uint256[],uint256[2],uint256,uint256),(bytes,bytes))` and selector `0xa38f76e8`
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
        name = "transact",
        abi = "transact(bytes,bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes,uint256[],uint256[2],uint256,uint256),(bytes,bytes))"
    )]
    pub struct TransactCall {
        pub proof: ::ethers::core::types::Bytes,
        pub aux_public_inputs: ::ethers::core::types::Bytes,
        pub external_data: CommonExtData,
        pub public_inputs: PublicInputs,
        pub encryptions: Encryptions,
    }
    ///Container type for all input parameters for the `unpackProof` function with signature `unpackProof(uint256[8])` and selector `0xf5ab0dd6`
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
    #[ethcall(name = "unpackProof", abi = "unpackProof(uint256[8])")]
    pub struct UnpackProofCall {
        pub proof: [::ethers::core::types::U256; 8],
    }
    ///Container type for all input parameters for the `updateEdge` function with signature `updateEdge(uint256,uint32,bytes32)` and selector `0xc1922f9e`
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
    #[ethcall(name = "updateEdge", abi = "updateEdge(uint256,uint32,bytes32)")]
    pub struct UpdateEdgeCall {
        pub root: ::ethers::core::types::U256,
        pub leaf_index: u32,
        pub src_resource_id: [u8; 32],
    }
    ///Container type for all input parameters for the `verifier` function with signature `verifier()` and selector `0x2b7ac3f3`
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
    #[ethcall(name = "verifier", abi = "verifier()")]
    pub struct VerifierCall;
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
    pub enum MultiAssetVAnchorContractCalls {
        EvmChainIdType(EvmChainIdTypeCall),
        FieldSize(FieldSizeCall),
        MaxExtAmount(MaxExtAmountCall),
        MaxFee(MaxFeeCall),
        RootHistorySize(RootHistorySizeCall),
        UpdateMaxLength(UpdateMaxLengthCall),
        ZeroValue(ZeroValueCall),
        GenExtDataHash(GenExtDataHashCall),
        CalculatePublicAmount(CalculatePublicAmountCall),
        Commitments(CommitmentsCall),
        ConfigureMaximumDepositLimit(ConfigureMaximumDepositLimitCall),
        ConfigureMinimumWithdrawalLimit(ConfigureMinimumWithdrawalLimitCall),
        CurrentNeighborRootIndex(CurrentNeighborRootIndexCall),
        EdgeExistsForChain(EdgeExistsForChainCall),
        EdgeIndex(EdgeIndexCall),
        EdgeList(EdgeListCall),
        FilledSubtrees(FilledSubtreesCall),
        GetChainId(GetChainIdCall),
        GetChainIdType(GetChainIdTypeCall),
        GetHasher(GetHasherCall),
        GetLastRoot(GetLastRootCall),
        GetLatestNeighborEdges(GetLatestNeighborEdgesCall),
        GetLatestNeighborRoots(GetLatestNeighborRootsCall),
        GetLevels(GetLevelsCall),
        GetNextIndex(GetNextIndexCall),
        GetZeroHash(GetZeroHashCall),
        Handler(HandlerCall),
        HasEdge(HasEdgeCall),
        HashLeftRight(HashLeftRightCall),
        Initialize(InitializeCall),
        Initialized(InitializedCall),
        IsCorrectExecutionChain(IsCorrectExecutionChainCall),
        IsCorrectExecutionContext(IsCorrectExecutionContextCall),
        IsKnownNeighborRoot(IsKnownNeighborRootCall),
        IsKnownRoot(IsKnownRootCall),
        IsSpent(IsSpentCall),
        IsSpentArray(IsSpentArrayCall),
        IsValidRoots(IsValidRootsCall),
        LastBalance(LastBalanceCall),
        MaxEdges(MaxEdgesCall),
        MaximumDepositAmount(MaximumDepositAmountCall),
        MinimumWithdrawalAmount(MinimumWithdrawalAmountCall),
        NeighborRoots(NeighborRootsCall),
        NullifierHashes(NullifierHashesCall),
        OnERC721Received(OnERC721ReceivedCall),
        OuterLevels(OuterLevelsCall),
        ParseChainIdFromResourceId(ParseChainIdFromResourceIdCall),
        ProposalNonce(ProposalNonceCall),
        Register(RegisterCall),
        RegisterAndTransact(RegisterAndTransactCall),
        Registry(RegistryCall),
        Roots(RootsCall),
        SetHandler(SetHandlerCall),
        SetVerifier(SetVerifierCall),
        Swap(SwapCall),
        Transact(TransactCall),
        UnpackProof(UnpackProofCall),
        UpdateEdge(UpdateEdgeCall),
        Verifier(VerifierCall),
    }
    impl ::ethers::core::abi::AbiDecode for MultiAssetVAnchorContractCalls {
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
            if let Ok(decoded) =
                <FieldSizeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FieldSize(decoded));
            }
            if let Ok(decoded) =
                <MaxExtAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::MaxExtAmount(decoded));
            }
            if let Ok(decoded) =
                <MaxFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxFee(decoded));
            }
            if let Ok(decoded) =
                <RootHistorySizeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RootHistorySize(decoded));
            }
            if let Ok(decoded) =
                <UpdateMaxLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UpdateMaxLength(decoded));
            }
            if let Ok(decoded) =
                <ZeroValueCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ZeroValue(decoded));
            }
            if let Ok(decoded) =
                <GenExtDataHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GenExtDataHash(decoded));
            }
            if let Ok(decoded) = <CalculatePublicAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculatePublicAmount(decoded));
            }
            if let Ok(decoded) =
                <CommitmentsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::Commitments(decoded));
            }
            if let Ok(decoded) = <ConfigureMaximumDepositLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConfigureMaximumDepositLimit(decoded));
            }
            if let Ok(decoded) = <ConfigureMinimumWithdrawalLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConfigureMinimumWithdrawalLimit(decoded));
            }
            if let Ok(decoded) = <CurrentNeighborRootIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrentNeighborRootIndex(decoded));
            }
            if let Ok(decoded) = <EdgeExistsForChainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EdgeExistsForChain(decoded));
            }
            if let Ok(decoded) =
                <EdgeIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EdgeIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeListCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EdgeList(decoded));
            }
            if let Ok(decoded) =
                <FilledSubtreesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::FilledSubtrees(decoded));
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
                <GetHasherCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHasher(decoded));
            }
            if let Ok(decoded) =
                <GetLastRootCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetLastRoot(decoded));
            }
            if let Ok(decoded) = <GetLatestNeighborEdgesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLatestNeighborEdges(decoded));
            }
            if let Ok(decoded) = <GetLatestNeighborRootsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLatestNeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <GetLevelsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLevels(decoded));
            }
            if let Ok(decoded) =
                <GetNextIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetNextIndex(decoded));
            }
            if let Ok(decoded) =
                <GetZeroHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetZeroHash(decoded));
            }
            if let Ok(decoded) =
                <HandlerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasEdgeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HasEdge(decoded));
            }
            if let Ok(decoded) =
                <HashLeftRightCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::HashLeftRight(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InitializedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::Initialized(decoded));
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
            if let Ok(decoded) = <IsKnownNeighborRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsKnownNeighborRoot(decoded));
            }
            if let Ok(decoded) =
                <IsKnownRootCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsKnownRoot(decoded));
            }
            if let Ok(decoded) =
                <IsSpentCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsSpent(decoded));
            }
            if let Ok(decoded) =
                <IsSpentArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsSpentArray(decoded));
            }
            if let Ok(decoded) =
                <IsValidRootsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsValidRoots(decoded));
            }
            if let Ok(decoded) =
                <LastBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LastBalance(decoded));
            }
            if let Ok(decoded) =
                <MaxEdgesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxEdges(decoded));
            }
            if let Ok(decoded) = <MaximumDepositAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaximumDepositAmount(decoded));
            }
            if let Ok(decoded) = <MinimumWithdrawalAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinimumWithdrawalAmount(decoded));
            }
            if let Ok(decoded) =
                <NeighborRootsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NullifierHashes(decoded));
            }
            if let Ok(decoded) =
                <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded) =
                <OuterLevelsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::OuterLevels(decoded));
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
                <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) = <RegisterAndTransactCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterAndTransact(decoded));
            }
            if let Ok(decoded) =
                <RegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Registry(decoded));
            }
            if let Ok(decoded) =
                <RootsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Roots(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <SetVerifierCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetVerifier(decoded));
            }
            if let Ok(decoded) =
                <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) =
                <TransactCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Transact(decoded));
            }
            if let Ok(decoded) =
                <UnpackProofCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UnpackProof(decoded));
            }
            if let Ok(decoded) =
                <UpdateEdgeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateEdge(decoded));
            }
            if let Ok(decoded) =
                <VerifierCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Verifier(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultiAssetVAnchorContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EvmChainIdType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FieldSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxExtAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RootHistorySize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMaxLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GenExtDataHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculatePublicAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Commitments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConfigureMaximumDepositLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConfigureMinimumWithdrawalLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentNeighborRootIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EdgeExistsForChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EdgeIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EdgeList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FilledSubtrees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainIdType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHasher(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLastRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestNeighborEdges(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestNeighborRoots(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLevels(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetZeroHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Handler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasEdge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashLeftRight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsCorrectExecutionChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsCorrectExecutionContext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsKnownNeighborRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsKnownRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSpent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSpentArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidRoots(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxEdges(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaximumDepositAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumWithdrawalAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NeighborRoots(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NullifierHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OuterLevels(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseChainIdFromResourceId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterAndTransact(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Registry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Roots(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetVerifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transact(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnpackProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateEdge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MultiAssetVAnchorContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::EvmChainIdType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FieldSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxExtAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::RootHistorySize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateMaxLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GenExtDataHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculatePublicAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Commitments(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfigureMaximumDepositLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfigureMinimumWithdrawalLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurrentNeighborRootIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EdgeExistsForChain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EdgeIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EdgeList(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FilledSubtrees(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainIdType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetHasher(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLastRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLatestNeighborEdges(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLatestNeighborRoots(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLevels(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNextIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetZeroHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Handler(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasEdge(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashLeftRight(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsCorrectExecutionChain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsCorrectExecutionContext(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsKnownNeighborRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsKnownRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsSpent(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSpentArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsValidRoots(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxEdges(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaximumDepositAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinimumWithdrawalAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NeighborRoots(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NullifierHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC721Received(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OuterLevels(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseChainIdFromResourceId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Register(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterAndTransact(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Registry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Roots(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetVerifier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transact(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpackProof(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateEdge(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Verifier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EvmChainIdTypeCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: EvmChainIdTypeCall) -> Self {
            Self::EvmChainIdType(value)
        }
    }
    impl ::core::convert::From<FieldSizeCall> for MultiAssetVAnchorContractCalls {
        fn from(value: FieldSizeCall) -> Self {
            Self::FieldSize(value)
        }
    }
    impl ::core::convert::From<MaxExtAmountCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: MaxExtAmountCall) -> Self {
            Self::MaxExtAmount(value)
        }
    }
    impl ::core::convert::From<MaxFeeCall> for MultiAssetVAnchorContractCalls {
        fn from(value: MaxFeeCall) -> Self {
            Self::MaxFee(value)
        }
    }
    impl ::core::convert::From<RootHistorySizeCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: RootHistorySizeCall) -> Self {
            Self::RootHistorySize(value)
        }
    }
    impl ::core::convert::From<UpdateMaxLengthCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: UpdateMaxLengthCall) -> Self {
            Self::UpdateMaxLength(value)
        }
    }
    impl ::core::convert::From<ZeroValueCall> for MultiAssetVAnchorContractCalls {
        fn from(value: ZeroValueCall) -> Self {
            Self::ZeroValue(value)
        }
    }
    impl ::core::convert::From<GenExtDataHashCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: GenExtDataHashCall) -> Self {
            Self::GenExtDataHash(value)
        }
    }
    impl ::core::convert::From<CalculatePublicAmountCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: CalculatePublicAmountCall) -> Self {
            Self::CalculatePublicAmount(value)
        }
    }
    impl ::core::convert::From<CommitmentsCall> for MultiAssetVAnchorContractCalls {
        fn from(value: CommitmentsCall) -> Self {
            Self::Commitments(value)
        }
    }
    impl ::core::convert::From<ConfigureMaximumDepositLimitCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: ConfigureMaximumDepositLimitCall) -> Self {
            Self::ConfigureMaximumDepositLimit(value)
        }
    }
    impl ::core::convert::From<ConfigureMinimumWithdrawalLimitCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: ConfigureMinimumWithdrawalLimitCall) -> Self {
            Self::ConfigureMinimumWithdrawalLimit(value)
        }
    }
    impl ::core::convert::From<CurrentNeighborRootIndexCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: CurrentNeighborRootIndexCall) -> Self {
            Self::CurrentNeighborRootIndex(value)
        }
    }
    impl ::core::convert::From<EdgeExistsForChainCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: EdgeExistsForChainCall) -> Self {
            Self::EdgeExistsForChain(value)
        }
    }
    impl ::core::convert::From<EdgeIndexCall> for MultiAssetVAnchorContractCalls {
        fn from(value: EdgeIndexCall) -> Self {
            Self::EdgeIndex(value)
        }
    }
    impl ::core::convert::From<EdgeListCall> for MultiAssetVAnchorContractCalls {
        fn from(value: EdgeListCall) -> Self {
            Self::EdgeList(value)
        }
    }
    impl ::core::convert::From<FilledSubtreesCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: FilledSubtreesCall) -> Self {
            Self::FilledSubtrees(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for MultiAssetVAnchorContractCalls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<GetChainIdTypeCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: GetChainIdTypeCall) -> Self {
            Self::GetChainIdType(value)
        }
    }
    impl ::core::convert::From<GetHasherCall> for MultiAssetVAnchorContractCalls {
        fn from(value: GetHasherCall) -> Self {
            Self::GetHasher(value)
        }
    }
    impl ::core::convert::From<GetLastRootCall> for MultiAssetVAnchorContractCalls {
        fn from(value: GetLastRootCall) -> Self {
            Self::GetLastRoot(value)
        }
    }
    impl ::core::convert::From<GetLatestNeighborEdgesCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: GetLatestNeighborEdgesCall) -> Self {
            Self::GetLatestNeighborEdges(value)
        }
    }
    impl ::core::convert::From<GetLatestNeighborRootsCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: GetLatestNeighborRootsCall) -> Self {
            Self::GetLatestNeighborRoots(value)
        }
    }
    impl ::core::convert::From<GetLevelsCall> for MultiAssetVAnchorContractCalls {
        fn from(value: GetLevelsCall) -> Self {
            Self::GetLevels(value)
        }
    }
    impl ::core::convert::From<GetNextIndexCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: GetNextIndexCall) -> Self {
            Self::GetNextIndex(value)
        }
    }
    impl ::core::convert::From<GetZeroHashCall> for MultiAssetVAnchorContractCalls {
        fn from(value: GetZeroHashCall) -> Self {
            Self::GetZeroHash(value)
        }
    }
    impl ::core::convert::From<HandlerCall> for MultiAssetVAnchorContractCalls {
        fn from(value: HandlerCall) -> Self {
            Self::Handler(value)
        }
    }
    impl ::core::convert::From<HasEdgeCall> for MultiAssetVAnchorContractCalls {
        fn from(value: HasEdgeCall) -> Self {
            Self::HasEdge(value)
        }
    }
    impl ::core::convert::From<HashLeftRightCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: HashLeftRightCall) -> Self {
            Self::HashLeftRight(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MultiAssetVAnchorContractCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InitializedCall> for MultiAssetVAnchorContractCalls {
        fn from(value: InitializedCall) -> Self {
            Self::Initialized(value)
        }
    }
    impl ::core::convert::From<IsCorrectExecutionChainCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: IsCorrectExecutionChainCall) -> Self {
            Self::IsCorrectExecutionChain(value)
        }
    }
    impl ::core::convert::From<IsCorrectExecutionContextCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: IsCorrectExecutionContextCall) -> Self {
            Self::IsCorrectExecutionContext(value)
        }
    }
    impl ::core::convert::From<IsKnownNeighborRootCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: IsKnownNeighborRootCall) -> Self {
            Self::IsKnownNeighborRoot(value)
        }
    }
    impl ::core::convert::From<IsKnownRootCall> for MultiAssetVAnchorContractCalls {
        fn from(value: IsKnownRootCall) -> Self {
            Self::IsKnownRoot(value)
        }
    }
    impl ::core::convert::From<IsSpentCall> for MultiAssetVAnchorContractCalls {
        fn from(value: IsSpentCall) -> Self {
            Self::IsSpent(value)
        }
    }
    impl ::core::convert::From<IsSpentArrayCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: IsSpentArrayCall) -> Self {
            Self::IsSpentArray(value)
        }
    }
    impl ::core::convert::From<IsValidRootsCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: IsValidRootsCall) -> Self {
            Self::IsValidRoots(value)
        }
    }
    impl ::core::convert::From<LastBalanceCall> for MultiAssetVAnchorContractCalls {
        fn from(value: LastBalanceCall) -> Self {
            Self::LastBalance(value)
        }
    }
    impl ::core::convert::From<MaxEdgesCall> for MultiAssetVAnchorContractCalls {
        fn from(value: MaxEdgesCall) -> Self {
            Self::MaxEdges(value)
        }
    }
    impl ::core::convert::From<MaximumDepositAmountCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: MaximumDepositAmountCall) -> Self {
            Self::MaximumDepositAmount(value)
        }
    }
    impl ::core::convert::From<MinimumWithdrawalAmountCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: MinimumWithdrawalAmountCall) -> Self {
            Self::MinimumWithdrawalAmount(value)
        }
    }
    impl ::core::convert::From<NeighborRootsCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: NeighborRootsCall) -> Self {
            Self::NeighborRoots(value)
        }
    }
    impl ::core::convert::From<NullifierHashesCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: NullifierHashesCall) -> Self {
            Self::NullifierHashes(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<OuterLevelsCall> for MultiAssetVAnchorContractCalls {
        fn from(value: OuterLevelsCall) -> Self {
            Self::OuterLevels(value)
        }
    }
    impl ::core::convert::From<ParseChainIdFromResourceIdCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: ParseChainIdFromResourceIdCall) -> Self {
            Self::ParseChainIdFromResourceId(value)
        }
    }
    impl ::core::convert::From<ProposalNonceCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: ProposalNonceCall) -> Self {
            Self::ProposalNonce(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for MultiAssetVAnchorContractCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<RegisterAndTransactCall>
        for MultiAssetVAnchorContractCalls
    {
        fn from(value: RegisterAndTransactCall) -> Self {
            Self::RegisterAndTransact(value)
        }
    }
    impl ::core::convert::From<RegistryCall> for MultiAssetVAnchorContractCalls {
        fn from(value: RegistryCall) -> Self {
            Self::Registry(value)
        }
    }
    impl ::core::convert::From<RootsCall> for MultiAssetVAnchorContractCalls {
        fn from(value: RootsCall) -> Self {
            Self::Roots(value)
        }
    }
    impl ::core::convert::From<SetHandlerCall> for MultiAssetVAnchorContractCalls {
        fn from(value: SetHandlerCall) -> Self {
            Self::SetHandler(value)
        }
    }
    impl ::core::convert::From<SetVerifierCall> for MultiAssetVAnchorContractCalls {
        fn from(value: SetVerifierCall) -> Self {
            Self::SetVerifier(value)
        }
    }
    impl ::core::convert::From<SwapCall> for MultiAssetVAnchorContractCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<TransactCall> for MultiAssetVAnchorContractCalls {
        fn from(value: TransactCall) -> Self {
            Self::Transact(value)
        }
    }
    impl ::core::convert::From<UnpackProofCall> for MultiAssetVAnchorContractCalls {
        fn from(value: UnpackProofCall) -> Self {
            Self::UnpackProof(value)
        }
    }
    impl ::core::convert::From<UpdateEdgeCall> for MultiAssetVAnchorContractCalls {
        fn from(value: UpdateEdgeCall) -> Self {
            Self::UpdateEdge(value)
        }
    }
    impl ::core::convert::From<VerifierCall> for MultiAssetVAnchorContractCalls {
        fn from(value: VerifierCall) -> Self {
            Self::Verifier(value)
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
    ///Container type for all return fields from the `FIELD_SIZE` function with signature `FIELD_SIZE()` and selector `0x414a37ba`
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
    pub struct FieldSizeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_EXT_AMOUNT` function with signature `MAX_EXT_AMOUNT()` and selector `0x7fe24ffe`
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
    pub struct MaxExtAmountReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `MAX_FEE` function with signature `MAX_FEE()` and selector `0xbc063e1a`
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
    pub struct MaxFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ROOT_HISTORY_SIZE` function with signature `ROOT_HISTORY_SIZE()` and selector `0xcd87a3b4`
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
    pub struct RootHistorySizeReturn(pub u32);
    ///Container type for all return fields from the `UPDATE_MAX_LENGTH` function with signature `UPDATE_MAX_LENGTH()` and selector `0xdbc9d395`
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
    pub struct UpdateMaxLengthReturn(pub u32);
    ///Container type for all return fields from the `ZERO_VALUE` function with signature `ZERO_VALUE()` and selector `0xec732959`
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
    pub struct ZeroValueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_genExtDataHash` function with signature `_genExtDataHash(bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes))` and selector `0xd384534d`
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
    pub struct GenExtDataHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculatePublicAmount` function with signature `calculatePublicAmount(int256,uint256)` and selector `0x2570b7b4`
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
    pub struct CalculatePublicAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `commitments` function with signature `commitments(uint256)` and selector `0x49ce8997`
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
    pub struct CommitmentsReturn(pub bool);
    ///Container type for all return fields from the `currentNeighborRootIndex` function with signature `currentNeighborRootIndex(uint256)` and selector `0x5d2d766c`
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
    pub struct CurrentNeighborRootIndexReturn(pub u32);
    ///Container type for all return fields from the `edgeExistsForChain` function with signature `edgeExistsForChain(uint256)` and selector `0xfa731687`
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
    pub struct EdgeExistsForChainReturn(pub bool);
    ///Container type for all return fields from the `edgeIndex` function with signature `edgeIndex(uint256)` and selector `0xe70ea87c`
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
    pub struct EdgeIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `edgeList` function with signature `edgeList(uint256)` and selector `0xdbc916b8`
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
    pub struct EdgeListReturn {
        pub chain_id: ::ethers::core::types::U256,
        pub root: ::ethers::core::types::U256,
        pub latest_leaf_index: ::ethers::core::types::U256,
        pub src_resource_id: [u8; 32],
    }
    ///Container type for all return fields from the `filledSubtrees` function with signature `filledSubtrees(uint256)` and selector `0xf178e47c`
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
    pub struct FilledSubtreesReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `getHasher` function with signature `getHasher()` and selector `0xea495db0`
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
    pub struct GetHasherReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getLastRoot` function with signature `getLastRoot()` and selector `0xba70f757`
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
    pub struct GetLastRootReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLatestNeighborEdges` function with signature `getLatestNeighborEdges()` and selector `0x8c0d34d8`
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
    pub struct GetLatestNeighborEdgesReturn(pub ::std::vec::Vec<Edge>);
    ///Container type for all return fields from the `getLatestNeighborRoots` function with signature `getLatestNeighborRoots()` and selector `0x1e627617`
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
    pub struct GetLatestNeighborRootsReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `getLevels` function with signature `getLevels()` and selector `0x0c394a60`
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
    pub struct GetLevelsReturn(pub u32);
    ///Container type for all return fields from the `getNextIndex` function with signature `getNextIndex()` and selector `0x0eb7606f`
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
    pub struct GetNextIndexReturn(pub u32);
    ///Container type for all return fields from the `getZeroHash` function with signature `getZeroHash(uint32)` and selector `0x305e9eac`
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
    pub struct GetZeroHashReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `handler` function with signature `handler()` and selector `0xc80916d4`
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
    pub struct HandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hasEdge` function with signature `hasEdge(uint256)` and selector `0x92156311`
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
    pub struct HasEdgeReturn(pub bool);
    ///Container type for all return fields from the `hashLeftRight` function with signature `hashLeftRight(uint256,uint256)` and selector `0x5bb93995`
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
    pub struct HashLeftRightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `initialized` function with signature `initialized()` and selector `0x158ef93e`
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
    pub struct InitializedReturn(pub bool);
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
    ///Container type for all return fields from the `isKnownNeighborRoot` function with signature `isKnownNeighborRoot(uint256,uint256)` and selector `0x3bfa8d7a`
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
    pub struct IsKnownNeighborRootReturn(pub bool);
    ///Container type for all return fields from the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `0xa6232a93`
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
    pub struct IsKnownRootReturn(pub bool);
    ///Container type for all return fields from the `isSpent` function with signature `isSpent(uint256)` and selector `0x5a129efe`
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
    pub struct IsSpentReturn(pub bool);
    ///Container type for all return fields from the `isSpentArray` function with signature `isSpentArray(uint256[])` and selector `0xea65ba49`
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
    pub struct IsSpentArrayReturn(pub ::std::vec::Vec<bool>);
    ///Container type for all return fields from the `isValidRoots` function with signature `isValidRoots(uint256[])` and selector `0xb75e6798`
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
    pub struct IsValidRootsReturn(pub bool);
    ///Container type for all return fields from the `lastBalance` function with signature `lastBalance()` and selector `0x8f1c56bd`
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
    pub struct LastBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxEdges` function with signature `maxEdges()` and selector `0x71523c32`
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
    pub struct MaxEdgesReturn(pub u8);
    ///Container type for all return fields from the `maximumDepositAmount` function with signature `maximumDepositAmount()` and selector `0x78abb49b`
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
    pub struct MaximumDepositAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minimumWithdrawalAmount` function with signature `minimumWithdrawalAmount()` and selector `0x2b180646`
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
    pub struct MinimumWithdrawalAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `neighborRoots` function with signature `neighborRoots(uint256,uint32)` and selector `0x43e7119f`
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
    pub struct NeighborRootsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nullifierHashes` function with signature `nullifierHashes(uint256)` and selector `0x1f79a1e9`
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
    pub struct NullifierHashesReturn(pub bool);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `outerLevels` function with signature `outerLevels()` and selector `0xbfbc0a39`
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
    pub struct OuterLevelsReturn(pub u32);
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
    pub struct ProposalNonceReturn(pub u32);
    ///Container type for all return fields from the `registry` function with signature `registry()` and selector `0x7b103999`
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
    pub struct RegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `roots` function with signature `roots(uint256)` and selector `0xc2b40ae4`
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
    pub struct RootsReturn {
        pub root: ::ethers::core::types::U256,
        pub latest_leafindex: u32,
    }
    ///Container type for all return fields from the `unpackProof` function with signature `unpackProof(uint256[8])` and selector `0xf5ab0dd6`
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
    pub struct UnpackProofReturn(
        pub [::ethers::core::types::U256; 2],
        pub [[::ethers::core::types::U256; 2]; 2],
        pub [::ethers::core::types::U256; 2],
    );
    ///Container type for all return fields from the `verifier` function with signature `verifier()` and selector `0x2b7ac3f3`
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
    pub struct VerifierReturn(pub ::ethers::core::types::Address);
    ///`CommonExtData(address,int256,address,uint256,uint256,address)`
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
    pub struct CommonExtData {
        pub recipient: ::ethers::core::types::Address,
        pub ext_amount: ::ethers::core::types::I256,
        pub relayer: ::ethers::core::types::Address,
        pub fee: ::ethers::core::types::U256,
        pub refund: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
    }
    ///`Edge(uint256,uint256,uint256,bytes32)`
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
    pub struct Edge {
        pub chain_id: ::ethers::core::types::U256,
        pub root: ::ethers::core::types::U256,
        pub latest_leaf_index: ::ethers::core::types::U256,
        pub src_resource_id: [u8; 32],
    }
    ///`Encryptions(bytes,bytes)`
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
    pub struct Encryptions {
        pub encrypted_output_1: ::ethers::core::types::Bytes,
        pub encrypted_output_2: ::ethers::core::types::Bytes,
    }
    ///`PublicInputs(bytes,bytes,uint256[],uint256[2],uint256,uint256)`
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
    pub struct PublicInputs {
        pub roots: ::ethers::core::types::Bytes,
        pub extension_roots: ::ethers::core::types::Bytes,
        pub input_nullifiers: ::std::vec::Vec<::ethers::core::types::U256>,
        pub output_commitments: [::ethers::core::types::U256; 2],
        pub public_amount: ::ethers::core::types::U256,
        pub ext_data_hash: ::ethers::core::types::U256,
    }
    ///`SwapPublicInputs(bytes,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)`
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
    pub struct SwapPublicInputs {
        pub roots: ::ethers::core::types::Bytes,
        pub alice_spend_nullifier: ::ethers::core::types::U256,
        pub bob_spend_nullifier: ::ethers::core::types::U256,
        pub swap_chain_id: ::ethers::core::types::U256,
        pub current_timestamp: ::ethers::core::types::U256,
        pub alice_change_record: ::ethers::core::types::U256,
        pub bob_change_record: ::ethers::core::types::U256,
        pub alice_receive_record: ::ethers::core::types::U256,
        pub bob_receive_record: ::ethers::core::types::U256,
    }
    ///`Account(address,bytes)`
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
    pub struct Account {
        pub owner: ::ethers::core::types::Address,
        pub key_data: ::ethers::core::types::Bytes,
    }
}
