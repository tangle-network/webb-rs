pub use fungible_token_wrapper_contract::*;
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
pub mod fungible_token_wrapper_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_symbol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
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
                (
                    ::std::borrow::ToOwned::to_owned("MINTER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MINTER_ROLE"),
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
                (
                    ::std::borrow::ToOwned::to_owned("PAUSER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PAUSER_ROLE"),
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
                (
                    ::std::borrow::ToOwned::to_owned("add"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("add"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
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
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("burnFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subtractedValue"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feePercentage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feePercentage"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeRecipient"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountToWrap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountToWrap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deposit"),
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
                    ::std::borrow::ToOwned::to_owned("getFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFee"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFeeFromAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFeeFromAmount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountToWrap"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMember"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMember"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
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
                    ::std::borrow::ToOwned::to_owned("getTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTokens"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("historicalTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("historicalTokens"),
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
                    ::std::borrow::ToOwned::to_owned("historicallyValid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("historicallyValid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addedValue"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    name: ::std::borrow::ToOwned::to_owned("_feePercentage"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feeRecipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_handler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_limit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_isNativeAllowed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("isNativeAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isNativeAllowed"),
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
                    ::std::borrow::ToOwned::to_owned("isValidToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isValidToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
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
                    ::std::borrow::ToOwned::to_owned("remove"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("remove"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
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
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("setFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feePercentage"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("setFeeRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeRecipient"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feeRecipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setNativeAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setNativeAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_isNativeAllowed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unwrap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unwrap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("unwrapAndSendTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unwrapAndSendTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("unwrapFor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unwrapFor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("updateLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateLimit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_limit"),
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
                    ::std::borrow::ToOwned::to_owned("valid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("valid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("wrap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wrap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("wrapFor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wrapFor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("wrapForAndSendTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wrapForAndSendTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("wrappingLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wrappingLimit"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("FeeRecipientUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FeeRecipientUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_feeRecipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FeeUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_feePercentage"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HandlerUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("HandlerUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_handler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NativeAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NativeAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_isNativeAllowed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenRemoved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unwrapping"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unwrapping"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("Wrapping"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Wrapping"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("wrappingFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("afterFeeAmount"),
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
                    ::std::borrow::ToOwned::to_owned("WrappingLimitUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WrappingLimitUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_limit"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FUNGIBLETOKENWRAPPERCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0G\x888\x03\x80b\0G\x88\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x03\x05V[\x81\x81\x81\x81\x81\x81`\x05b\0\0H\x83\x82b\0\x03\xFDV[P`\x06b\0\0W\x82\x82b\0\x03\xFDV[PP`\x07\x80T`\xFF\x19\x16\x90UPb\0\0q`\x003b\0\0\xDBV[b\0\0\x9D\x7F\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA63b\0\0\xDBV[b\0\0\xC9\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*3b\0\0\xDBV[PP`\x01`\x08UPb\0\x04\xC9\x92PPPV[b\0\0\xE7\x82\x82b\0\0\xEBV[PPV[b\0\x01\x02\x82\x82b\0\x01.` \x1Bb\0$\xA3\x17` \x1CV[`\0\x82\x81R`\x01` \x90\x81R`@\x90\x91 b\0\x01)\x91\x83\x90b\0%'b\0\x01\xCE\x82\x1B\x17\x90\x1CV[PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\0\xE7W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01\x8A3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0b\0\x01\xE5\x83`\x01`\x01`\xA0\x1B\x03\x84\x16b\0\x01\xEEV[\x90P[\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Tb\0\x027WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ub\0\x01\xE8V[P`\0b\0\x01\xE8V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\x02hW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x02\x85Wb\0\x02\x85b\0\x02@V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x02\xB0Wb\0\x02\xB0b\0\x02@V[\x81`@R\x83\x81R` \x92P\x86\x83\x85\x88\x01\x01\x11\x15b\0\x02\xCDW`\0\x80\xFD[`\0\x91P[\x83\x82\x10\x15b\0\x02\xF1W\x85\x82\x01\x83\x01Q\x81\x83\x01\x84\x01R\x90\x82\x01\x90b\0\x02\xD2V[`\0\x93\x81\x01\x90\x92\x01\x92\x90\x92R\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x03\x19W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x031W`\0\x80\xFD[b\0\x03?\x86\x83\x87\x01b\0\x02VV[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15b\0\x03VW`\0\x80\xFD[Pb\0\x03e\x85\x82\x86\x01b\0\x02VV[\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x03\x84W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x03\xA5WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x01)W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x03\xD4WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x03\xF5W\x82\x81U`\x01\x01b\0\x03\xE0V[PPPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x04\x19Wb\0\x04\x19b\0\x02@V[b\0\x041\x81b\0\x04*\x84Tb\0\x03oV[\x84b\0\x03\xABV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x04iW`\0\x84\x15b\0\x04PWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x03\xF5V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x04\x9AW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x04yV[P\x85\x82\x10\x15b\0\x04\xB9W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[aB\xAF\x80b\0\x04\xD9`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x03\x8CW`\x005`\xE0\x1C\x80c\x85\xC0\n\xE8\x11a\x01\xDCW\x80c\xBA\xC4&\xD0\x11a\x01\x02W\x80c\xCE\xD7/\x87\x11a\0\xA0W\x80c\xE6:\xB1\xE9\x11a\0oW\x80c\xE6:\xB1\xE9\x14a\nsW\x80c\xF6>\xBBE\x14a\n\x95W\x80c\xFA\xE0\x95\x9A\x14a\n\xB5W\x80c\xFC\x97\xA6R\x14a\n\xD5W`\0\x80\xFD[\x80c\xCE\xD7/\x87\x14a\t\xF8W\x80c\xD59\x13\x93\x14a\n\x11W\x80c\xD5Gt\x1F\x14a\n3W\x80c\xDDb\xED>\x14a\nSW`\0\x80\xFD[\x80c\xC2\xAEG \x11a\0\xDCW\x80c\xC2\xAEG \x14a\t\x82W\x80c\xC8\t\x16\xD4\x14a\t\xA2W\x80c\xCA\x15\xC8s\x14a\t\xC2W\x80c\xCC<t\xA1\x14a\t\xE2W`\0\x80\xFD[\x80c\xBA\xC4&\xD0\x14a\t/W\x80c\xBF7lz\x14a\tOW\x80c\xC1\x87dS\x14a\tbW`\0\x80\xFD[\x80c\xA0\x01\xEC\xDD\x11a\x01zW\x80c\xAAl\xA8\x08\x11a\x01IW\x80c\xAAl\xA8\x08\x14a\x08\x93W\x80c\xAC\x8A&\x0C\x14a\x08\xB5W\x80c\xB1\xCB\xA2X\x14a\x08\xE5W\x80c\xB3\xE4\x08?\x14a\t\x15W`\0\x80\xFD[\x80c\xA0\x01\xEC\xDD\x14a\x08\x10W\x80c\xA2\x17\xFD\xDF\x14a\x08>W\x80c\xA4W\xC2\xD7\x14a\x08SW\x80c\xA9\x05\x9C\xBB\x14a\x08sW`\0\x80\xFD[\x80c\x90\x10\xD0|\x11a\x01\xB6W\x80c\x90\x10\xD0|\x14a\x07\x9BW\x80c\x91\xD1HT\x14a\x07\xBBW\x80c\x95\xD8\x9BA\x14a\x07\xDBW\x80c\x96\xCDM\xFE\x14a\x07\xF0W`\0\x80\xFD[\x80c\x85\xC0\n\xE8\x14a\x07;W\x80c\x85\xD1H4\x14a\x07[W\x80c\x8BTx\xB9\x14a\x07{W`\0\x80\xFD[\x80c1<\xE5g\x11a\x02\xC1W\x80cF\x90H@\x11a\x02_W\x80cp\xA0\x821\x11a\x02.W\x80cp\xA0\x821\x14a\x06\xBDW\x80cy\xCCg\x90\x14a\x06\xF3W\x80c{.0\xD6\x14a\x07\x13W\x80c\x84V\xCBY\x14a\x07&W`\0\x80\xFD[\x80cF\x90H@\x14a\x06'W\x80cH\x08(^\x14a\x06eW\x80cOd\xB2\xBE\x14a\x06\x85W\x80c\\\x97Z\xBB\x14a\x06\xA5W`\0\x80\xFD[\x80c9\xF4v\x93\x11a\x02\x9BW\x80c9\xF4v\x93\x14a\x05\xB2W\x80c?K\xA8:\x14a\x05\xD2W\x80c@\xC1\x0F\x19\x14a\x05\xE7W\x80cB\x96lh\x14a\x06\x07W`\0\x80\xFD[\x80c1<\xE5g\x14a\x05VW\x80c6V\x8A\xBE\x14a\x05rW\x80c9P\x93Q\x14a\x05\x92W`\0\x80\xFD[\x80c\x1CJ\x146\x11a\x03.W\x80c$\x8A\x9C\xA3\x11a\x03\x08W\x80c$\x8A\x9C\xA3\x14a\x04\xD3W\x80c&\x1C\x80\xB6\x14a\x05\x03W\x80c,\xA6\x93\x88\x14a\x05#W\x80c//\xF1]\x14a\x056W`\0\x80\xFD[\x80c\x1CJ\x146\x14a\x04}W\x80c\x1F\x91C\x82\x14a\x04\x9DW\x80c#\xB8r\xDD\x14a\x04\xB3W`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x03jW\x80c\t^\xA7\xB3\x14a\x04\nW\x80c\x0B'\xFB\x9A\x14a\x04*W\x80c\x15\x8E\xF9>\x14a\x04IW\x80c\x18\x16\r\xDD\x14a\x04hW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x03\x91W\x80c\x06\xFD\xDE\x03\x14a\x03\xC6W\x80c\x07\x18O\x1C\x14a\x03\xE8W[`\0\x80\xFD[4\x80\x15a\x03\x9DW`\0\x80\xFD[Pa\x03\xB1a\x03\xAC6`\x04a7\x87V[a\n\xF5V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xD2W`\0\x80\xFD[Pa\x03\xDBa\x0B V[`@Qa\x03\xBD\x91\x90a7\xD5V[4\x80\x15a\x03\xF4W`\0\x80\xFD[Pa\x04\x08a\x04\x036`\x04a81V[a\x0B\xB2V[\0[4\x80\x15a\x04\x16W`\0\x80\xFD[Pa\x03\xB1a\x04%6`\x04a8fV[a\rCV[4\x80\x15a\x046W`\0\x80\xFD[P`\nT[`@Q\x90\x81R` \x01a\x03\xBDV[4\x80\x15a\x04UW`\0\x80\xFD[P`\x07Ta\x03\xB1\x90a\x01\0\x90\x04`\xFF\x16\x81V[4\x80\x15a\x04tW`\0\x80\xFD[P`\x04Ta\x04;V[4\x80\x15a\x04\x89W`\0\x80\xFD[Pa\x04\x08a\x04\x986`\x04a81V[a\r[V[4\x80\x15a\x04\xA9W`\0\x80\xFD[Pa\x04;`\x11T\x81V[4\x80\x15a\x04\xBFW`\0\x80\xFD[Pa\x03\xB1a\x04\xCE6`\x04a8\x92V[a\x0F\xACV[4\x80\x15a\x04\xDFW`\0\x80\xFD[Pa\x04;a\x04\xEE6`\x04a8\xD3V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x05\x0FW`\0\x80\xFD[Pa\x04\x08a\x05\x1E6`\x04a8\x92V[a\x0F\xD0V[a\x04\x08a\x0516`\x04a8\x92V[a\x11gV[4\x80\x15a\x05BW`\0\x80\xFD[Pa\x04\x08a\x05Q6`\x04a8\xECV[a\x12\xE8V[4\x80\x15a\x05bW`\0\x80\xFD[P`@Q`\x12\x81R` \x01a\x03\xBDV[4\x80\x15a\x05~W`\0\x80\xFD[Pa\x04\x08a\x05\x8D6`\x04a8\xECV[a\x13\rV[4\x80\x15a\x05\x9EW`\0\x80\xFD[Pa\x03\xB1a\x05\xAD6`\x04a8fV[a\x13\x8BV[4\x80\x15a\x05\xBEW`\0\x80\xFD[Pa\x04\x08a\x05\xCD6`\x04a8fV[a\x13\xADV[4\x80\x15a\x05\xDEW`\0\x80\xFD[Pa\x04\x08a\x15\x0BV[4\x80\x15a\x05\xF3W`\0\x80\xFD[Pa\x04\x08a\x06\x026`\x04a8fV[a\x15\x9FV[4\x80\x15a\x06\x13W`\0\x80\xFD[Pa\x04\x08a\x06\"6`\x04a8\xD3V[a\x16,V[4\x80\x15a\x063W`\0\x80\xFD[P`\tTa\x06M\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xBDV[4\x80\x15a\x06qW`\0\x80\xFD[Pa\x04\x08a\x06\x806`\x04a9\x1CV[a\x169V[4\x80\x15a\x06\x91W`\0\x80\xFD[Pa\x06Ma\x06\xA06`\x04a8\xD3V[a\x17\x8BV[4\x80\x15a\x06\xB1W`\0\x80\xFD[P`\x07T`\xFF\x16a\x03\xB1V[4\x80\x15a\x06\xC9W`\0\x80\xFD[Pa\x04;a\x06\xD86`\x04a9^V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 T\x90V[4\x80\x15a\x06\xFFW`\0\x80\xFD[Pa\x04\x08a\x07\x0E6`\x04a8fV[a\x17\xB5V[a\x04\x08a\x07!6`\x04a9{V[a\x17\xCAV[4\x80\x15a\x072W`\0\x80\xFD[Pa\x04\x08a\x19QV[4\x80\x15a\x07GW`\0\x80\xFD[Pa\x04;a\x07V6`\x04a8\xD3V[a\x19\xE3V[4\x80\x15a\x07gW`\0\x80\xFD[Pa\x06Ma\x07v6`\x04a8\xD3V[a\x1A\x05V[4\x80\x15a\x07\x87W`\0\x80\xFD[Pa\x04\x08a\x07\x966`\x04a9\xDCV[a\x1A\x15V[4\x80\x15a\x07\xA7W`\0\x80\xFD[Pa\x06Ma\x07\xB66`\x04a9\xF9V[a\x1A\xAEV[4\x80\x15a\x07\xC7W`\0\x80\xFD[Pa\x03\xB1a\x07\xD66`\x04a8\xECV[a\x1A\xCDV[4\x80\x15a\x07\xE7W`\0\x80\xFD[Pa\x03\xDBa\x1A\xF6V[4\x80\x15a\x07\xFCW`\0\x80\xFD[Pa\x04;a\x08\x0B6`\x04a8\xD3V[a\x1B\x05V[4\x80\x15a\x08\x1CW`\0\x80\xFD[P`\tTa\x08+\x90a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xBDV[4\x80\x15a\x08JW`\0\x80\xFD[Pa\x04;`\0\x81V[4\x80\x15a\x08_W`\0\x80\xFD[Pa\x03\xB1a\x08n6`\x04a8fV[a\x1B+V[4\x80\x15a\x08\x7FW`\0\x80\xFD[Pa\x03\xB1a\x08\x8E6`\x04a8fV[a\x1B\xA6V[4\x80\x15a\x08\x9FW`\0\x80\xFD[Pa\x08\xA8a\x1B\xB4V[`@Qa\x03\xBD\x91\x90a:\x1BV[4\x80\x15a\x08\xC1W`\0\x80\xFD[Pa\x03\xB1a\x08\xD06`\x04a9^V[`\x0E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x08\xF1W`\0\x80\xFD[Pa\x03\xB1a\t\x006`\x04a9^V[`\x0F` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\t!W`\0\x80\xFD[P`\x10Ta\x03\xB1\x90`\xFF\x16\x81V[4\x80\x15a\t;W`\0\x80\xFD[Pa\x04\x08a\tJ6`\x04a9^V[a\x1C\x15V[a\x04\x08a\t]6`\x04a8fV[a\x1C\xDAV[4\x80\x15a\tnW`\0\x80\xFD[Pa\x03\xB1a\t}6`\x04a9^V[a\x1E'V[4\x80\x15a\t\x8EW`\0\x80\xFD[Pa\x04\x08a\t\x9D6`\x04a:zV[a\x1EhV[4\x80\x15a\t\xAEW`\0\x80\xFD[P`\x0BTa\x06M\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\t\xCEW`\0\x80\xFD[Pa\x04;a\t\xDD6`\x04a8\xD3V[a\x1F\xC1V[4\x80\x15a\t\xEEW`\0\x80\xFD[Pa\x04;`\nT\x81V[4\x80\x15a\n\x04W`\0\x80\xFD[P`\tTa\xFF\xFF\x16a\x08+V[4\x80\x15a\n\x1DW`\0\x80\xFD[Pa\x04;`\0\x80Q` aBZ\x839\x81Q\x91R\x81V[4\x80\x15a\n?W`\0\x80\xFD[Pa\x04\x08a\nN6`\x04a8\xECV[a\x1F\xD8V[4\x80\x15a\n_W`\0\x80\xFD[Pa\x04;a\nn6`\x04a:\x96V[a\x1F\xFDV[4\x80\x15a\n\x7FW`\0\x80\xFD[Pa\x04;`\0\x80Q` aB:\x839\x81Q\x91R\x81V[4\x80\x15a\n\xA1W`\0\x80\xFD[Pa\x04\x08a\n\xB06`\x04a:\xC4V[a (V[4\x80\x15a\n\xC1W`\0\x80\xFD[Pa\x04\x08a\n\xD06`\x04a8\xD3V[a\"\x01V[4\x80\x15a\n\xE1W`\0\x80\xFD[Pa\x04\x08a\n\xF06`\x04a81V[a\"`V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x0B\x1AWPa\x0B\x1A\x82a%<V[\x92\x91PPV[```\x05\x80Ta\x0B/\x90a;;V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B[\x90a;;V[\x80\x15a\x0B\xA8W\x80`\x1F\x10a\x0B}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`@Q\x80\x91\x03\x90\xFD[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x0C\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x80c\xFF\xFF\xFF\xFF\x16\x80`\nT\x10a\x0C4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<\tV[`\nTa\x0CB\x90`\x01a<bV[\x81\x11\x15a\x0CaW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<uV[`\n\x81\x90U`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0C\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FFungibleTokenWrapper: Fee Recipi`D\x82\x01R\x7Fent cannot be zero address\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xDCV[`\t\x80Tb\x01\0\0`\x01`\xB0\x1B\x03\x19\x16b\x01\0\0`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x90\x81R\x7Fz{Z\n\x13/\x9E\x05\x81\xEB\x85'\xF6n\xAE\x9E\xE8\x9C*>y\xD4\xAC~A\xA1\xF1\xF4\xD4\x8A\x7F\xC2\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPPV[`\x003a\rQ\x81\x85\x85a%qV[P`\x01\x93\x92PPPV[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\r\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x80c\xFF\xFF\xFF\xFF\x16\x80`\nT\x10a\r\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<\tV[`\nTa\r\xE2\x90`\x01a<bV[\x81\x11\x15a\x0E\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<uV[`\n\x81\x90U`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16a\x0E\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FFungibleTokenWrapper: Token shou`D\x82\x01Rj\x1B\x19\x08\x18\x99H\x1D\x98[\x1AY`\xAA\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\0\x80[`\x0CT\x81\x10\x15a\x0E\xE1W\x84`\x01`\x01`\xA0\x1B\x03\x16`\x0C\x82\x81T\x81\x10a\x0E\xADWa\x0E\xADa<\xD2V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0E\xCFW\x80\x91Pa\x0E\xE1V[\x80a\x0E\xD9\x81a<\xE8V[\x91PPa\x0E\x86V[P`\x0CT\x81\x10a\x0FAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FFungibleTokenWrapper: Token not `D\x82\x01Rd\x19\x9B\xDD[\x99`\xDA\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x0E` R`@\x90 \x80T`\xFF\x19\x16\x90Ua\x0Fj\x81a&\x95V[`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R\x7FL\x91\x0Bi\xFEe\xA6\x1Fu1\xB9\xC5\x04+#)\xCAqy\xC7r\x90\xAA~.\xB3\xAF\xA3\xC8Q\x1F\xD3\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[`\x003a\x0F\xBA\x85\x82\x85a'EV[a\x0F\xC5\x85\x85\x85a'\xB9V[P`\x01\x94\x93PPPPV[a\x0F\xD8a)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x0F\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[a\x10\x17`\0\x80Q` aBZ\x839\x81Q\x91R3a\x1A\xCDV[a\x103W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x01V[\x81\x81`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10\x8AW\x80G\x10\x15a\x10cW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=DV[`\x10T`\xFF\x16a\x10\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x8DV[a\x11JV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xF4\x91\x90a=\xF8V[\x10\x15a\x11\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x11JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>YV[a\x11V\x85\x85\x85\x88a)\xC8V[PPa\x11b`\x01`\x08UV[PPPV[a\x11oa)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x11\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[a\x11\xAE`\0\x80Q` aBZ\x839\x81Q\x91R3a\x1A\xCDV[a\x11\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x01V[`\tT\x82\x90`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x91\x04\x81\x16\x90\x83\x90\x83\x16a\x12.W\x80\x15a\x12\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\xA7V[`\x10T`\xFF\x16a\x12)W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\x04V[a\x12\x84V[4\x15a\x12LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?mV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16a\x12\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x12\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xFDV[a\x12\xB3\x81a*\x8DV[a\x12\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a@OV[a\x12\xDB\x86\x86\x86\x89a+\x06V[PPPa\x11b`\x01`\x08UV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x13\x03\x81a,QV[a\x11b\x83\x83a,[V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x13}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a\x13\x87\x82\x82a,}V[PPV[`\x003a\rQ\x81\x85\x85a\x13\x9E\x83\x83a\x1F\xFDV[a\x13\xA8\x91\x90a<bV[a%qV[a\x13\xB5a)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x13\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x81\x81`\x01`\x01`\xA0\x1B\x03\x82\x16a\x143W\x80G\x10\x15a\x14\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=DV[`\x10T`\xFF\x16a\x14.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x8DV[a\x14\xF3V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x9D\x91\x90a=\xF8V[\x10\x15a\x14\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x14\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>YV[a\x14\xFF3\x85\x853a)\xC8V[PPa\x13\x87`\x01`\x08UV[a\x15#`\0\x80Q` aB:\x839\x81Q\x91R3a\x1A\xCDV[a\x15\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FERC20PresetMinterPauser: must ha`D\x82\x01R\x7Fve pauser role to unpause\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xDCV[a\x15\x9Da,\x9FV[V[a\x15\xB7`\0\x80Q` aBZ\x839\x81Q\x91R3a\x1A\xCDV[a\x16\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FERC20PresetMinterPauser: must ha`D\x82\x01Ru\x1D\x99H\x1BZ[\x9D\x19\\\x88\x1C\x9B\xDB\x19H\x1D\x1B\xC8\x1BZ[\x9D`R\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a\x13\x87\x82\x82a,\xF1V[a\x1663\x82a-\xBEV[PV[a\x16Aa)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x16hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x82\x82`\x01`\x01`\xA0\x1B\x03\x82\x16a\x16\xBFW\x80G\x10\x15a\x16\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=DV[`\x10T`\xFF\x16a\x16\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x8DV[a\x17\x7FV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17)\x91\x90a=\xF8V[\x10\x15a\x17GW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x17\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>YV[a\x11V3\x86\x86\x86a)\xC8V[`\x0C\x81\x81T\x81\x10a\x17\x9BW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[a\x17\xC0\x823\x83a'EV[a\x13\x87\x82\x82a-\xBEV[a\x17\xD2a)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x17\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[a\x18\x11`\0\x80Q` aBZ\x839\x81Q\x91R3a\x1A\xCDV[a\x18-W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x01V[`\tT\x83\x90`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x91\x04\x81\x16\x90\x84\x90\x83\x16a\x18\x91W\x80\x15a\x18jW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\xA7V[`\x10T`\xFF\x16a\x18\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\x04V[a\x18\xE7V[4\x15a\x18\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?mV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16a\x18\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x19\rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xFDV[a\x19\x16\x81a*\x8DV[a\x192W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a@OV[a\x19>\x87\x87\x87\x87a+\x06V[PPPa\x19K`\x01`\x08UV[PPPPV[a\x19i`\0\x80Q` aB:\x839\x81Q\x91R3a\x1A\xCDV[a\x19\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FERC20PresetMinterPauser: must ha`D\x82\x01R\x7Fve pauser role to pause\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xDCV[a\x15\x9Da.\xFEV[`\tT`\0\x90a'\x10\x90a\x19\xFB\x90a\xFF\xFF\x16\x84a@\x91V[a\x0B\x1A\x91\x90a@\xA8V[`\r\x81\x81T\x81\x10a\x17\x9BW`\0\x80\xFD[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A?W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x1AfW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[`\x10\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x0E\xE34\x89\xA3p\x8D\xD3vY\xF7\x9F\xF3\xC6\x8B8_\xAD\xF2p\xBA\x08\xB0\xBDu\xBA\x88*\x9C\xBD\xCE\xAB\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\0\x82\x81R`\x01` R`@\x81 a\x1A\xC6\x90\x83a/;V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[```\x06\x80Ta\x0B/\x90a;;V[`\tT`\0\x90a\x1B\x1B\x90a\xFF\xFF\x16a'\x10a@\xCAV[a\xFF\xFF\x16a\x19\xFB\x83a'\x10a@\x91V[`\x003\x81a\x1B9\x82\x86a\x1F\xFDV[\x90P\x83\x81\x10\x15a\x1B\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a\x0F\xC5\x82\x86\x86\x84\x03a%qV[`\x003a\rQ\x81\x85\x85a'\xB9V[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\xA8W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1B\xEEWPPPPP\x90P\x90V[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C?W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x1CfW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a@\xE5V[`\x0B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xFE\x14\x98C\xA4@KCi\x9DDl\x99\xC9\xBE-z[\xFC\x8B\xD6n\x15\xCAL\xFA\xD5\xCA(\x11\xDD\x9B\x90` \x01a\x1A\xA3V[a\x1C\xE2a)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x1D\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[`\tT\x82\x90`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x91\x04\x81\x16\x90\x83\x90\x83\x16a\x1DmW\x80\x15a\x1DFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\xA7V[`\x10T`\xFF\x16a\x1DhW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\x04V[a\x1D\xC3V[4\x15a\x1D\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?mV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16a\x1D\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1D\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xFDV[a\x1D\xF2\x81a*\x8DV[a\x1E\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a@OV[a\x1E\x1A3\x86\x863a+\x06V[PPPa\x13\x87`\x01`\x08UV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1EBW`\x10T`\xFF\x16a\x0B\x1AV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16a\x0B\x1AV[\x91\x90PV[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x1E\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x80c\xFF\xFF\xFF\xFF\x16\x80`\nT\x10a\x1E\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<\tV[`\nTa\x1E\xEF\x90`\x01a<bV[\x81\x11\x15a\x1F\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<uV[`\n\x81\x90Ua'\x10a\xFF\xFF\x84\x16\x10a\x1F}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFungibleTokenWrapper: Invalid fe`D\x82\x01Rke percentage`\xA0\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\t\x80Ta\xFF\xFF\x19\x16a\xFF\xFF\x85\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xC8\xFC\xF8\xEE\x14%\xE7\xE6\x0B\x8A\xF875\xE1\xEBQm[\x9E\xF0[\xFDn\xEC\xE5R\xEB\xAE\xB7\xC7[H\x90` \x01a\r6V[`\0\x81\x81R`\x01` R`@\x81 a\x0B\x1A\x90a/GV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x1F\xF3\x81a,QV[a\x11b\x83\x83a,}V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[`\x07Ta\x01\0\x90\x04`\xFF\x16\x15a \x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInitialized: Already initialized`D\x82\x01R`d\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x85\x16a \xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FFungibleTokenWrapper: Fee Recipi`D\x82\x01Ru\x06V\xE7B\x04\x16FG&W72\x066\x16\xE2wB\x06&R\x03`T\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x84\x16a!\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a@\xE5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FFungibleTokenWrapper: Admin Addr`D\x82\x01Rm\x06W72\x066\x16\xE2wB\x06&R\x03`\x94\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a!\x91\x81a/QV[P`\x07\x80Ta\x01\0a\xFF\0\x19\x90\x91\x16\x17\x90U`\t\x80Ta\xFF\xFF\x96\x90\x96\x16`\x01`\x01`\xB0\x1B\x03\x19\x90\x96\x16\x95\x90\x95\x17b\x01\0\0`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x02\x17\x90\x94U`\x0B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x91U`\x11U`\x10\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\"+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x11\x81\x90U`@Q\x81\x81R\x7F\x14\xC1\x92\xA3{\xA7NZ&\x1A\x88m\xC0\xEAH\xAFB\xEF\x0F\x8A\xAF\x19\xC3\xAD\xFBa\xA2\x1C\xFE\xD3\x87\x1A\x90` \x01a\x1A\xA3V[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\"\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x80c\xFF\xFF\xFF\xFF\x16\x80`\nT\x10a\"\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<\tV[`\nTa\"\xE7\x90`\x01a<bV[\x81\x11\x15a#\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<uV[`\n\x81\x90U`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16\x15a#\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FFungibleTokenWrapper: Token shou`D\x82\x01Rn\x1B\x19\x08\x1B\x9B\xDD\x08\x18\x99H\x1D\x98[\x1AY`\x8A\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x0C\x80T`\x01\x81\x01\x90\x91U\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a$OW`\r\x80T`\x01\x80\x82\x01\x90\x92U\x7F\xD7\xB6\x99\x01\x05q\x91\x01\xDA\xBE\xB7qD\xF2\xA38\\\x803\xAC\xD3\xAF\x97\xE9B:i^\x81\xAD\x1E\xB5\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x81\x17\x90\x91U`\0\x90\x81R`\x0F` R`@\x90 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x0E` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90Q\x91\x82R\x7FxL\x8FM\xBF\x0F\xFE\xDDnr\xC7e\x01\xC5E\xA7\x0F\x8B ;0\xA2l\xE5B\xBF\x92\xBA\x87\xC2H\xA4\x91\x01a\r6V[a$\xAD\x82\x82a\x1A\xCDV[a\x13\x87W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua$\xE33\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x1A\xC6\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a/\x8CV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x0B\x1AWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x0B\x1AV[`\x01`\x01`\xA0\x1B\x03\x83\x16a%\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x82\x16a&4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x0C\x80Ta&\xA5\x90`\x01\x90aA5V[\x81T\x81\x10a&\xB5Wa&\xB5a<\xD2V[`\0\x91\x82R` \x90\x91 \x01T`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x83\x90\x81\x10a&\xE1Wa&\xE1a<\xD2V[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\x0C\x80T\x80a' Wa' aAHV[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPV[`\0a'Q\x84\x84a\x1F\xFDV[\x90P`\0\x19\x81\x14a\x19KW\x81\x81\x10\x15a'\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x0B\xDCV[a\x19K\x84\x84\x84\x84\x03a%qV[`\x01`\x01`\xA0\x1B\x03\x83\x16a(\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x82\x16a(\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a(\x8A\x83\x83\x83a/\xDBV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a)\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a)b\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x19KV[`\x02`\x08T\x03a)\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B\xDCV[`\x02`\x08UV[a)\xD2\x84\x83a-\xBEV[`\x01`\x01`\xA0\x1B\x03\x83\x16a*\x1CW`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x83\x15a\x08\xFC\x02\x90\x84\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a*\x16W=`\0\x80>=`\0\xFD[Pa*0V[a*0`\x01`\x01`\xA0\x1B\x03\x84\x16\x82\x84a/\xE6V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x7F\xC7\xCF2j\xE4\xD9F\"\x11v\xA4W\xCC0\x9D],\x15\xF3Tq\x11\xD8\xCC\xA3iQ\x07n\xEEM\x08\x85`@Qa*\x7F\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPV[`\0`\x11T0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xF4\x91\x90a=\xF8V[a*\xFE\x90\x84a<bV[\x11\x15\x92\x91PPV[`\0a+&`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a+ W\x83a\x19\xE3V[4a\x19\xE3V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a+HWa+C\x82\x85aA5V[a+RV[a+R\x824aA5V[\x90P`\x01`\x01`\xA0\x1B\x03\x85\x16a+\xA7W`\tT`@Qb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90a\x08\xFC\x84\x15\x02\x90\x84\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a+\xA1W=`\0\x80>=`\0\xFD[Pa+\xDFV[a+\xBC`\x01`\x01`\xA0\x1B\x03\x86\x16\x870\x84a0IV[`\tTa+\xDF\x90`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x89\x91b\x01\0\0\x90\x91\x04\x16\x85a0IV[a+\xE9\x83\x82a,\xF1V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F3\xB7F\xCF\xD2\x0B\x94@E\xB43\n\xE6\xF0\xC7me\xEBK99\xA9/l\n]\xCEN\x9E\x9F\x99\xD9\x85\x85`@Qa,A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPPV[a\x166\x813a0\x81V[a,e\x82\x82a$\xA3V[`\0\x82\x81R`\x01` R`@\x90 a\x11b\x90\x82a%'V[a,\x87\x82\x82a0\xDAV[`\0\x82\x81R`\x01` R`@\x90 a\x11b\x90\x82a1?V[a,\xA7a1TV[`\x07\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\x01`\x01`\xA0\x1B\x03\x82\x16a-GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x0B\xDCV[a-S`\0\x83\x83a/\xDBV[\x80`\x04`\0\x82\x82Ta-e\x91\x90a<bV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a.\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a.*\x82`\0\x83a/\xDBV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a.\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`\x04\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a/\x06a1\x9DV[`\x07\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa,\xD43\x90V[`\0a\x1A\xC6\x83\x83a1\xE3V[`\0a\x0B\x1A\x82T\x90V[a/i`\0\x80Q` aBZ\x839\x81Q\x91R\x82a2\rV[a/t`\0\x82a2\rV[a\x166`\0\x80Q` aB:\x839\x81Q\x91R\x82a2\rV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta/\xD3WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x0B\x1AV[P`\0a\x0B\x1AV[a\x11b\x83\x83\x83a2\x17V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x11b\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra2}V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x19K\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01a0\x12V[a0\x8B\x82\x82a\x1A\xCDV[a\x13\x87Wa0\x98\x81a3OV[a0\xA3\x83` a3aV[`@Q` \x01a0\xB4\x92\x91\x90aA^V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x0B\xDC\x91`\x04\x01a7\xD5V[a0\xE4\x82\x82a\x1A\xCDV[\x15a\x13\x87W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x1A\xC6\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a4\xFDV[`\x07T`\xFF\x16a\x15\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x18]\\\xD8X\x9B\x19N\x88\x1B\x9B\xDD\x08\x1C\x18]\\\xD9Y`b\x1B`D\x82\x01R`d\x01a\x0B\xDCV[`\x07T`\xFF\x16\x15a\x15\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x14\x18]\\\xD8X\x9B\x19N\x88\x1C\x18]\\\xD9Y`\x82\x1B`D\x82\x01R`d\x01a\x0B\xDCV[`\0\x82`\0\x01\x82\x81T\x81\x10a1\xFAWa1\xFAa<\xD2V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[a\x13\x87\x82\x82a,[V[`\x07T`\xFF\x16\x15a\x11bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FERC20Pausable: token transfer wh`D\x82\x01Ri\x1A[\x19H\x1C\x18]\\\xD9Y`\xB2\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\0a2\xD2\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a5\xF7\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x11bW\x80\x80` \x01\x90Q\x81\x01\x90a2\xF0\x91\x90aA\xD3V[a\x11bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[``a\x0B\x1A`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a3p\x83`\x02a@\x91V[a3{\x90`\x02a<bV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\x93Wa3\x93aA\xF0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a3\xBDW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a3\xD8Wa3\xD8a<\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a4\x07Wa4\x07a<\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a4+\x84`\x02a@\x91V[a46\x90`\x01a<bV[\x90P[`\x01\x81\x11\x15a4\xAEWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a4jWa4ja<\xD2V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a4\x80Wa4\x80a<\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a4\xA7\x81aB\x06V[\x90Pa49V[P\x83\x15a\x1A\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x0B\xDCV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a5\xE6W`\0a5!`\x01\x83aA5V[\x85T\x90\x91P`\0\x90a55\x90`\x01\x90aA5V[\x90P\x81\x81\x14a5\x9AW`\0\x86`\0\x01\x82\x81T\x81\x10a5UWa5Ua<\xD2V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a5xWa5xa<\xD2V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a5\xABWa5\xABaAHV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x0B\x1AV[`\0\x91PPa\x0B\x1AV[P\x92\x91PPV[``a6\x06\x84\x84`\0\x85a6\x0EV[\x94\x93PPPPV[``\x82G\x10\x15a6oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa6\x8B\x91\x90aB\x1DV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a6\xC8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a6\xCDV[``\x91P[P\x91P\x91Pa6\xDE\x87\x83\x83\x87a6\xE9V[\x97\x96PPPPPPPV[``\x83\x15a7XW\x82Q`\0\x03a7QW`\x01`\x01`\xA0\x1B\x03\x85\x16;a7QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0B\xDCV[P\x81a6\x06V[a6\x06\x83\x83\x81Q\x15a7mW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x91\x90a7\xD5V[`\0` \x82\x84\x03\x12\x15a7\x99W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1A\xC6W`\0\x80\xFD[`\0[\x83\x81\x10\x15a7\xCCW\x81\x81\x01Q\x83\x82\x01R` \x01a7\xB4V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra7\xF4\x81`@\x85\x01` \x87\x01a7\xB1V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x166W`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1EcW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a8DW`\0\x80\xFD[\x825a8O\x81a8\x08V[\x91Pa8]` \x84\x01a8\x1DV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a8yW`\0\x80\xFD[\x825a8\x84\x81a8\x08V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a8\xA7W`\0\x80\xFD[\x835a8\xB2\x81a8\x08V[\x92P` \x84\x015a8\xC2\x81a8\x08V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a8\xE5W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a8\xFFW`\0\x80\xFD[\x825\x91P` \x83\x015a9\x11\x81a8\x08V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a91W`\0\x80\xFD[\x835a9<\x81a8\x08V[\x92P` \x84\x015\x91P`@\x84\x015a9S\x81a8\x08V[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a9pW`\0\x80\xFD[\x815a\x1A\xC6\x81a8\x08V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a9\x91W`\0\x80\xFD[\x845a9\x9C\x81a8\x08V[\x93P` \x85\x015a9\xAC\x81a8\x08V[\x92P`@\x85\x015\x91P``\x85\x015a9\xC3\x81a8\x08V[\x93\x96\x92\x95P\x90\x93PPV[\x80\x15\x15\x81\x14a\x166W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a9\xEEW`\0\x80\xFD[\x815a\x1A\xC6\x81a9\xCEV[`\0\x80`@\x83\x85\x03\x12\x15a:\x0CW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a:\\W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a:7V[P\x90\x96\x95PPPPPPV[\x805a\xFF\xFF\x81\x16\x81\x14a\x1EcW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a:\x8DW`\0\x80\xFD[a8O\x83a:hV[`\0\x80`@\x83\x85\x03\x12\x15a:\xA9W`\0\x80\xFD[\x825a:\xB4\x81a8\x08V[\x91P` \x83\x015a9\x11\x81a8\x08V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a:\xDDW`\0\x80\xFD[a:\xE6\x87a:hV[\x95P` \x87\x015a:\xF6\x81a8\x08V[\x94P`@\x87\x015a;\x06\x81a8\x08V[\x93P``\x87\x015\x92P`\x80\x87\x015a;\x1D\x81a9\xCEV[\x91P`\xA0\x87\x015a;-\x81a8\x08V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01\x81\x81\x1C\x90\x82\x16\x80a;OW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a;oWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`9\x90\x82\x01R\x7FFungibleTokenWrapper: Only handl`@\x82\x01R\x7Fer can call this function\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x1C\x90\x82\x01R\x7FInitialized: Not initialized\0\0\0\0`@\x82\x01R``\x01\x90V[` \x80\x82R`#\x90\x82\x01R\x7FProposalNonceTracker: Invalid no`@\x82\x01Rbnce`\xE8\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\x1AWa\x0B\x1Aa<LV[` \x80\x82R`:\x90\x82\x01R\x7FProposalNonceTracker: Nonce must`@\x82\x01R\x7F not increment more than 1\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a<\xFAWa<\xFAa<LV[P`\x01\x01\x90V[` \x80\x82R`#\x90\x82\x01R\x7FTokenWrapper: must have minter r`@\x82\x01Rbole`\xE8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`)\x90\x82\x01R\x7FTokenWrapper: Insufficient nativ`@\x82\x01Rhe balance`\xB8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`E\x90\x82\x01R\x7FTokenWrapper: Native unwrapping `@\x82\x01R\x7Fis not allowed for this token wr``\x82\x01Rd0\xB882\xB9`\xD9\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0` \x82\x84\x03\x12\x15a>\nW`\0\x80\xFD[PQ\x91\x90PV[` \x80\x82R`(\x90\x82\x01R\x7FTokenWrapper: Insufficient ERC20`@\x82\x01Rg balance`\xC0\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`.\x90\x82\x01R\x7FTokenWrapper: Invalid historical`@\x82\x01Rm token address`\x90\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`9\x90\x82\x01R\x7FTokenWrapper: Invalid amount pro`@\x82\x01R\x7Fvided for native wrapping\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`C\x90\x82\x01R\x7FTokenWrapper: Native wrapping is`@\x82\x01R\x7F not allowed for this token wrap``\x82\x01Rb82\xB9`\xE9\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`-\x90\x82\x01R\x7FTokenWrapper: Invalid value sent`@\x82\x01Rl for wrapping`\x98\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`#\x90\x82\x01R\x7FTokenWrapper: Invalid token addr`@\x82\x01Rbess`\xE8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`2\x90\x82\x01R\x7FTokenWrapper: Fee Recipient cann`@\x82\x01Rqot be zero address`p\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\"\x90\x82\x01R\x7FTokenWrapper: Invalid token amou`@\x82\x01Ra\x1B\x9D`\xF2\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0B\x1AWa\x0B\x1Aa<LV[`\0\x82a@\xC5WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[a\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a5\xF0Wa5\xF0a<LV[` \x80\x82R`0\x90\x82\x01R\x7FFungibleTokenWrapper: Handler Ad`@\x82\x01Ro\x06G&W72\x066\x16\xE2wB\x06&R\x03`\x84\x1B``\x82\x01R`\x80\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0B\x1AWa\x0B\x1Aa<LV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83QaA\x96\x81`\x17\x85\x01` \x88\x01a7\xB1V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83QaA\xC7\x81`(\x84\x01` \x88\x01a7\xB1V[\x01`(\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aA\xE5W`\0\x80\xFD[\x81Qa\x1A\xC6\x81a9\xCEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x81aB\x15WaB\x15a<LV[P`\0\x19\x01\x90V[`\0\x82QaB/\x81\x84` \x87\x01a7\xB1V[\x91\x90\x91\x01\x92\x91PPV\xFEe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6\xA2dipfsX\"\x12 \xCF\xA9<\x06]\x8D\xF0\x89\xFF\x1B0\xD8\x08\xB8O\x1F&`3[O\x1F\x8F\x02\xED\x96\xA4\xDD\x1C\x86\x110dsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static FUNGIBLETOKENWRAPPERCONTRACT_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x03\x8CW`\x005`\xE0\x1C\x80c\x85\xC0\n\xE8\x11a\x01\xDCW\x80c\xBA\xC4&\xD0\x11a\x01\x02W\x80c\xCE\xD7/\x87\x11a\0\xA0W\x80c\xE6:\xB1\xE9\x11a\0oW\x80c\xE6:\xB1\xE9\x14a\nsW\x80c\xF6>\xBBE\x14a\n\x95W\x80c\xFA\xE0\x95\x9A\x14a\n\xB5W\x80c\xFC\x97\xA6R\x14a\n\xD5W`\0\x80\xFD[\x80c\xCE\xD7/\x87\x14a\t\xF8W\x80c\xD59\x13\x93\x14a\n\x11W\x80c\xD5Gt\x1F\x14a\n3W\x80c\xDDb\xED>\x14a\nSW`\0\x80\xFD[\x80c\xC2\xAEG \x11a\0\xDCW\x80c\xC2\xAEG \x14a\t\x82W\x80c\xC8\t\x16\xD4\x14a\t\xA2W\x80c\xCA\x15\xC8s\x14a\t\xC2W\x80c\xCC<t\xA1\x14a\t\xE2W`\0\x80\xFD[\x80c\xBA\xC4&\xD0\x14a\t/W\x80c\xBF7lz\x14a\tOW\x80c\xC1\x87dS\x14a\tbW`\0\x80\xFD[\x80c\xA0\x01\xEC\xDD\x11a\x01zW\x80c\xAAl\xA8\x08\x11a\x01IW\x80c\xAAl\xA8\x08\x14a\x08\x93W\x80c\xAC\x8A&\x0C\x14a\x08\xB5W\x80c\xB1\xCB\xA2X\x14a\x08\xE5W\x80c\xB3\xE4\x08?\x14a\t\x15W`\0\x80\xFD[\x80c\xA0\x01\xEC\xDD\x14a\x08\x10W\x80c\xA2\x17\xFD\xDF\x14a\x08>W\x80c\xA4W\xC2\xD7\x14a\x08SW\x80c\xA9\x05\x9C\xBB\x14a\x08sW`\0\x80\xFD[\x80c\x90\x10\xD0|\x11a\x01\xB6W\x80c\x90\x10\xD0|\x14a\x07\x9BW\x80c\x91\xD1HT\x14a\x07\xBBW\x80c\x95\xD8\x9BA\x14a\x07\xDBW\x80c\x96\xCDM\xFE\x14a\x07\xF0W`\0\x80\xFD[\x80c\x85\xC0\n\xE8\x14a\x07;W\x80c\x85\xD1H4\x14a\x07[W\x80c\x8BTx\xB9\x14a\x07{W`\0\x80\xFD[\x80c1<\xE5g\x11a\x02\xC1W\x80cF\x90H@\x11a\x02_W\x80cp\xA0\x821\x11a\x02.W\x80cp\xA0\x821\x14a\x06\xBDW\x80cy\xCCg\x90\x14a\x06\xF3W\x80c{.0\xD6\x14a\x07\x13W\x80c\x84V\xCBY\x14a\x07&W`\0\x80\xFD[\x80cF\x90H@\x14a\x06'W\x80cH\x08(^\x14a\x06eW\x80cOd\xB2\xBE\x14a\x06\x85W\x80c\\\x97Z\xBB\x14a\x06\xA5W`\0\x80\xFD[\x80c9\xF4v\x93\x11a\x02\x9BW\x80c9\xF4v\x93\x14a\x05\xB2W\x80c?K\xA8:\x14a\x05\xD2W\x80c@\xC1\x0F\x19\x14a\x05\xE7W\x80cB\x96lh\x14a\x06\x07W`\0\x80\xFD[\x80c1<\xE5g\x14a\x05VW\x80c6V\x8A\xBE\x14a\x05rW\x80c9P\x93Q\x14a\x05\x92W`\0\x80\xFD[\x80c\x1CJ\x146\x11a\x03.W\x80c$\x8A\x9C\xA3\x11a\x03\x08W\x80c$\x8A\x9C\xA3\x14a\x04\xD3W\x80c&\x1C\x80\xB6\x14a\x05\x03W\x80c,\xA6\x93\x88\x14a\x05#W\x80c//\xF1]\x14a\x056W`\0\x80\xFD[\x80c\x1CJ\x146\x14a\x04}W\x80c\x1F\x91C\x82\x14a\x04\x9DW\x80c#\xB8r\xDD\x14a\x04\xB3W`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x03jW\x80c\t^\xA7\xB3\x14a\x04\nW\x80c\x0B'\xFB\x9A\x14a\x04*W\x80c\x15\x8E\xF9>\x14a\x04IW\x80c\x18\x16\r\xDD\x14a\x04hW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x03\x91W\x80c\x06\xFD\xDE\x03\x14a\x03\xC6W\x80c\x07\x18O\x1C\x14a\x03\xE8W[`\0\x80\xFD[4\x80\x15a\x03\x9DW`\0\x80\xFD[Pa\x03\xB1a\x03\xAC6`\x04a7\x87V[a\n\xF5V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xD2W`\0\x80\xFD[Pa\x03\xDBa\x0B V[`@Qa\x03\xBD\x91\x90a7\xD5V[4\x80\x15a\x03\xF4W`\0\x80\xFD[Pa\x04\x08a\x04\x036`\x04a81V[a\x0B\xB2V[\0[4\x80\x15a\x04\x16W`\0\x80\xFD[Pa\x03\xB1a\x04%6`\x04a8fV[a\rCV[4\x80\x15a\x046W`\0\x80\xFD[P`\nT[`@Q\x90\x81R` \x01a\x03\xBDV[4\x80\x15a\x04UW`\0\x80\xFD[P`\x07Ta\x03\xB1\x90a\x01\0\x90\x04`\xFF\x16\x81V[4\x80\x15a\x04tW`\0\x80\xFD[P`\x04Ta\x04;V[4\x80\x15a\x04\x89W`\0\x80\xFD[Pa\x04\x08a\x04\x986`\x04a81V[a\r[V[4\x80\x15a\x04\xA9W`\0\x80\xFD[Pa\x04;`\x11T\x81V[4\x80\x15a\x04\xBFW`\0\x80\xFD[Pa\x03\xB1a\x04\xCE6`\x04a8\x92V[a\x0F\xACV[4\x80\x15a\x04\xDFW`\0\x80\xFD[Pa\x04;a\x04\xEE6`\x04a8\xD3V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x05\x0FW`\0\x80\xFD[Pa\x04\x08a\x05\x1E6`\x04a8\x92V[a\x0F\xD0V[a\x04\x08a\x0516`\x04a8\x92V[a\x11gV[4\x80\x15a\x05BW`\0\x80\xFD[Pa\x04\x08a\x05Q6`\x04a8\xECV[a\x12\xE8V[4\x80\x15a\x05bW`\0\x80\xFD[P`@Q`\x12\x81R` \x01a\x03\xBDV[4\x80\x15a\x05~W`\0\x80\xFD[Pa\x04\x08a\x05\x8D6`\x04a8\xECV[a\x13\rV[4\x80\x15a\x05\x9EW`\0\x80\xFD[Pa\x03\xB1a\x05\xAD6`\x04a8fV[a\x13\x8BV[4\x80\x15a\x05\xBEW`\0\x80\xFD[Pa\x04\x08a\x05\xCD6`\x04a8fV[a\x13\xADV[4\x80\x15a\x05\xDEW`\0\x80\xFD[Pa\x04\x08a\x15\x0BV[4\x80\x15a\x05\xF3W`\0\x80\xFD[Pa\x04\x08a\x06\x026`\x04a8fV[a\x15\x9FV[4\x80\x15a\x06\x13W`\0\x80\xFD[Pa\x04\x08a\x06\"6`\x04a8\xD3V[a\x16,V[4\x80\x15a\x063W`\0\x80\xFD[P`\tTa\x06M\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xBDV[4\x80\x15a\x06qW`\0\x80\xFD[Pa\x04\x08a\x06\x806`\x04a9\x1CV[a\x169V[4\x80\x15a\x06\x91W`\0\x80\xFD[Pa\x06Ma\x06\xA06`\x04a8\xD3V[a\x17\x8BV[4\x80\x15a\x06\xB1W`\0\x80\xFD[P`\x07T`\xFF\x16a\x03\xB1V[4\x80\x15a\x06\xC9W`\0\x80\xFD[Pa\x04;a\x06\xD86`\x04a9^V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 T\x90V[4\x80\x15a\x06\xFFW`\0\x80\xFD[Pa\x04\x08a\x07\x0E6`\x04a8fV[a\x17\xB5V[a\x04\x08a\x07!6`\x04a9{V[a\x17\xCAV[4\x80\x15a\x072W`\0\x80\xFD[Pa\x04\x08a\x19QV[4\x80\x15a\x07GW`\0\x80\xFD[Pa\x04;a\x07V6`\x04a8\xD3V[a\x19\xE3V[4\x80\x15a\x07gW`\0\x80\xFD[Pa\x06Ma\x07v6`\x04a8\xD3V[a\x1A\x05V[4\x80\x15a\x07\x87W`\0\x80\xFD[Pa\x04\x08a\x07\x966`\x04a9\xDCV[a\x1A\x15V[4\x80\x15a\x07\xA7W`\0\x80\xFD[Pa\x06Ma\x07\xB66`\x04a9\xF9V[a\x1A\xAEV[4\x80\x15a\x07\xC7W`\0\x80\xFD[Pa\x03\xB1a\x07\xD66`\x04a8\xECV[a\x1A\xCDV[4\x80\x15a\x07\xE7W`\0\x80\xFD[Pa\x03\xDBa\x1A\xF6V[4\x80\x15a\x07\xFCW`\0\x80\xFD[Pa\x04;a\x08\x0B6`\x04a8\xD3V[a\x1B\x05V[4\x80\x15a\x08\x1CW`\0\x80\xFD[P`\tTa\x08+\x90a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xBDV[4\x80\x15a\x08JW`\0\x80\xFD[Pa\x04;`\0\x81V[4\x80\x15a\x08_W`\0\x80\xFD[Pa\x03\xB1a\x08n6`\x04a8fV[a\x1B+V[4\x80\x15a\x08\x7FW`\0\x80\xFD[Pa\x03\xB1a\x08\x8E6`\x04a8fV[a\x1B\xA6V[4\x80\x15a\x08\x9FW`\0\x80\xFD[Pa\x08\xA8a\x1B\xB4V[`@Qa\x03\xBD\x91\x90a:\x1BV[4\x80\x15a\x08\xC1W`\0\x80\xFD[Pa\x03\xB1a\x08\xD06`\x04a9^V[`\x0E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x08\xF1W`\0\x80\xFD[Pa\x03\xB1a\t\x006`\x04a9^V[`\x0F` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\t!W`\0\x80\xFD[P`\x10Ta\x03\xB1\x90`\xFF\x16\x81V[4\x80\x15a\t;W`\0\x80\xFD[Pa\x04\x08a\tJ6`\x04a9^V[a\x1C\x15V[a\x04\x08a\t]6`\x04a8fV[a\x1C\xDAV[4\x80\x15a\tnW`\0\x80\xFD[Pa\x03\xB1a\t}6`\x04a9^V[a\x1E'V[4\x80\x15a\t\x8EW`\0\x80\xFD[Pa\x04\x08a\t\x9D6`\x04a:zV[a\x1EhV[4\x80\x15a\t\xAEW`\0\x80\xFD[P`\x0BTa\x06M\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\t\xCEW`\0\x80\xFD[Pa\x04;a\t\xDD6`\x04a8\xD3V[a\x1F\xC1V[4\x80\x15a\t\xEEW`\0\x80\xFD[Pa\x04;`\nT\x81V[4\x80\x15a\n\x04W`\0\x80\xFD[P`\tTa\xFF\xFF\x16a\x08+V[4\x80\x15a\n\x1DW`\0\x80\xFD[Pa\x04;`\0\x80Q` aBZ\x839\x81Q\x91R\x81V[4\x80\x15a\n?W`\0\x80\xFD[Pa\x04\x08a\nN6`\x04a8\xECV[a\x1F\xD8V[4\x80\x15a\n_W`\0\x80\xFD[Pa\x04;a\nn6`\x04a:\x96V[a\x1F\xFDV[4\x80\x15a\n\x7FW`\0\x80\xFD[Pa\x04;`\0\x80Q` aB:\x839\x81Q\x91R\x81V[4\x80\x15a\n\xA1W`\0\x80\xFD[Pa\x04\x08a\n\xB06`\x04a:\xC4V[a (V[4\x80\x15a\n\xC1W`\0\x80\xFD[Pa\x04\x08a\n\xD06`\x04a8\xD3V[a\"\x01V[4\x80\x15a\n\xE1W`\0\x80\xFD[Pa\x04\x08a\n\xF06`\x04a81V[a\"`V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x0B\x1AWPa\x0B\x1A\x82a%<V[\x92\x91PPV[```\x05\x80Ta\x0B/\x90a;;V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B[\x90a;;V[\x80\x15a\x0B\xA8W\x80`\x1F\x10a\x0B}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`@Q\x80\x91\x03\x90\xFD[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x0C\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x80c\xFF\xFF\xFF\xFF\x16\x80`\nT\x10a\x0C4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<\tV[`\nTa\x0CB\x90`\x01a<bV[\x81\x11\x15a\x0CaW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<uV[`\n\x81\x90U`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0C\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FFungibleTokenWrapper: Fee Recipi`D\x82\x01R\x7Fent cannot be zero address\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xDCV[`\t\x80Tb\x01\0\0`\x01`\xB0\x1B\x03\x19\x16b\x01\0\0`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x90\x81R\x7Fz{Z\n\x13/\x9E\x05\x81\xEB\x85'\xF6n\xAE\x9E\xE8\x9C*>y\xD4\xAC~A\xA1\xF1\xF4\xD4\x8A\x7F\xC2\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPPV[`\x003a\rQ\x81\x85\x85a%qV[P`\x01\x93\x92PPPV[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\r\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x80c\xFF\xFF\xFF\xFF\x16\x80`\nT\x10a\r\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<\tV[`\nTa\r\xE2\x90`\x01a<bV[\x81\x11\x15a\x0E\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<uV[`\n\x81\x90U`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16a\x0E\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FFungibleTokenWrapper: Token shou`D\x82\x01Rj\x1B\x19\x08\x18\x99H\x1D\x98[\x1AY`\xAA\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\0\x80[`\x0CT\x81\x10\x15a\x0E\xE1W\x84`\x01`\x01`\xA0\x1B\x03\x16`\x0C\x82\x81T\x81\x10a\x0E\xADWa\x0E\xADa<\xD2V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0E\xCFW\x80\x91Pa\x0E\xE1V[\x80a\x0E\xD9\x81a<\xE8V[\x91PPa\x0E\x86V[P`\x0CT\x81\x10a\x0FAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FFungibleTokenWrapper: Token not `D\x82\x01Rd\x19\x9B\xDD[\x99`\xDA\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x0E` R`@\x90 \x80T`\xFF\x19\x16\x90Ua\x0Fj\x81a&\x95V[`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R\x7FL\x91\x0Bi\xFEe\xA6\x1Fu1\xB9\xC5\x04+#)\xCAqy\xC7r\x90\xAA~.\xB3\xAF\xA3\xC8Q\x1F\xD3\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[`\x003a\x0F\xBA\x85\x82\x85a'EV[a\x0F\xC5\x85\x85\x85a'\xB9V[P`\x01\x94\x93PPPPV[a\x0F\xD8a)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x0F\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[a\x10\x17`\0\x80Q` aBZ\x839\x81Q\x91R3a\x1A\xCDV[a\x103W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x01V[\x81\x81`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10\x8AW\x80G\x10\x15a\x10cW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=DV[`\x10T`\xFF\x16a\x10\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x8DV[a\x11JV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xF4\x91\x90a=\xF8V[\x10\x15a\x11\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x11JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>YV[a\x11V\x85\x85\x85\x88a)\xC8V[PPa\x11b`\x01`\x08UV[PPPV[a\x11oa)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x11\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[a\x11\xAE`\0\x80Q` aBZ\x839\x81Q\x91R3a\x1A\xCDV[a\x11\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x01V[`\tT\x82\x90`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x91\x04\x81\x16\x90\x83\x90\x83\x16a\x12.W\x80\x15a\x12\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\xA7V[`\x10T`\xFF\x16a\x12)W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\x04V[a\x12\x84V[4\x15a\x12LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?mV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16a\x12\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x12\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xFDV[a\x12\xB3\x81a*\x8DV[a\x12\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a@OV[a\x12\xDB\x86\x86\x86\x89a+\x06V[PPPa\x11b`\x01`\x08UV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x13\x03\x81a,QV[a\x11b\x83\x83a,[V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x13}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a\x13\x87\x82\x82a,}V[PPV[`\x003a\rQ\x81\x85\x85a\x13\x9E\x83\x83a\x1F\xFDV[a\x13\xA8\x91\x90a<bV[a%qV[a\x13\xB5a)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x13\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x81\x81`\x01`\x01`\xA0\x1B\x03\x82\x16a\x143W\x80G\x10\x15a\x14\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=DV[`\x10T`\xFF\x16a\x14.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x8DV[a\x14\xF3V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x9D\x91\x90a=\xF8V[\x10\x15a\x14\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x14\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>YV[a\x14\xFF3\x85\x853a)\xC8V[PPa\x13\x87`\x01`\x08UV[a\x15#`\0\x80Q` aB:\x839\x81Q\x91R3a\x1A\xCDV[a\x15\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FERC20PresetMinterPauser: must ha`D\x82\x01R\x7Fve pauser role to unpause\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xDCV[a\x15\x9Da,\x9FV[V[a\x15\xB7`\0\x80Q` aBZ\x839\x81Q\x91R3a\x1A\xCDV[a\x16\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FERC20PresetMinterPauser: must ha`D\x82\x01Ru\x1D\x99H\x1BZ[\x9D\x19\\\x88\x1C\x9B\xDB\x19H\x1D\x1B\xC8\x1BZ[\x9D`R\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a\x13\x87\x82\x82a,\xF1V[a\x1663\x82a-\xBEV[PV[a\x16Aa)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x16hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x82\x82`\x01`\x01`\xA0\x1B\x03\x82\x16a\x16\xBFW\x80G\x10\x15a\x16\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=DV[`\x10T`\xFF\x16a\x16\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x8DV[a\x17\x7FV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17)\x91\x90a=\xF8V[\x10\x15a\x17GW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a\x17\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>YV[a\x11V3\x86\x86\x86a)\xC8V[`\x0C\x81\x81T\x81\x10a\x17\x9BW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[a\x17\xC0\x823\x83a'EV[a\x13\x87\x82\x82a-\xBEV[a\x17\xD2a)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x17\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[a\x18\x11`\0\x80Q` aBZ\x839\x81Q\x91R3a\x1A\xCDV[a\x18-W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a=\x01V[`\tT\x83\x90`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x91\x04\x81\x16\x90\x84\x90\x83\x16a\x18\x91W\x80\x15a\x18jW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\xA7V[`\x10T`\xFF\x16a\x18\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\x04V[a\x18\xE7V[4\x15a\x18\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?mV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16a\x18\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x19\rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xFDV[a\x19\x16\x81a*\x8DV[a\x192W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a@OV[a\x19>\x87\x87\x87\x87a+\x06V[PPPa\x19K`\x01`\x08UV[PPPPV[a\x19i`\0\x80Q` aB:\x839\x81Q\x91R3a\x1A\xCDV[a\x19\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FERC20PresetMinterPauser: must ha`D\x82\x01R\x7Fve pauser role to pause\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xDCV[a\x15\x9Da.\xFEV[`\tT`\0\x90a'\x10\x90a\x19\xFB\x90a\xFF\xFF\x16\x84a@\x91V[a\x0B\x1A\x91\x90a@\xA8V[`\r\x81\x81T\x81\x10a\x17\x9BW`\0\x80\xFD[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A?W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x1AfW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[`\x10\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x0E\xE34\x89\xA3p\x8D\xD3vY\xF7\x9F\xF3\xC6\x8B8_\xAD\xF2p\xBA\x08\xB0\xBDu\xBA\x88*\x9C\xBD\xCE\xAB\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\0\x82\x81R`\x01` R`@\x81 a\x1A\xC6\x90\x83a/;V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[```\x06\x80Ta\x0B/\x90a;;V[`\tT`\0\x90a\x1B\x1B\x90a\xFF\xFF\x16a'\x10a@\xCAV[a\xFF\xFF\x16a\x19\xFB\x83a'\x10a@\x91V[`\x003\x81a\x1B9\x82\x86a\x1F\xFDV[\x90P\x83\x81\x10\x15a\x1B\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a\x0F\xC5\x82\x86\x86\x84\x03a%qV[`\x003a\rQ\x81\x85\x85a'\xB9V[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\xA8W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1B\xEEWPPPPP\x90P\x90V[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C?W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x1CfW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a@\xE5V[`\x0B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xFE\x14\x98C\xA4@KCi\x9DDl\x99\xC9\xBE-z[\xFC\x8B\xD6n\x15\xCAL\xFA\xD5\xCA(\x11\xDD\x9B\x90` \x01a\x1A\xA3V[a\x1C\xE2a)oV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x1D\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[`\tT\x82\x90`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x91\x04\x81\x16\x90\x83\x90\x83\x16a\x1DmW\x80\x15a\x1DFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a>\xA7V[`\x10T`\xFF\x16a\x1DhW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\x04V[a\x1D\xC3V[4\x15a\x1D\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?mV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16a\x1D\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1D\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a?\xFDV[a\x1D\xF2\x81a*\x8DV[a\x1E\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a@OV[a\x1E\x1A3\x86\x863a+\x06V[PPPa\x13\x87`\x01`\x08UV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1EBW`\x10T`\xFF\x16a\x0B\x1AV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16a\x0B\x1AV[\x91\x90PV[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\x1E\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x80c\xFF\xFF\xFF\xFF\x16\x80`\nT\x10a\x1E\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<\tV[`\nTa\x1E\xEF\x90`\x01a<bV[\x81\x11\x15a\x1F\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<uV[`\n\x81\x90Ua'\x10a\xFF\xFF\x84\x16\x10a\x1F}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFungibleTokenWrapper: Invalid fe`D\x82\x01Rke percentage`\xA0\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\t\x80Ta\xFF\xFF\x19\x16a\xFF\xFF\x85\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xC8\xFC\xF8\xEE\x14%\xE7\xE6\x0B\x8A\xF875\xE1\xEBQm[\x9E\xF0[\xFDn\xEC\xE5R\xEB\xAE\xB7\xC7[H\x90` \x01a\r6V[`\0\x81\x81R`\x01` R`@\x81 a\x0B\x1A\x90a/GV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x1F\xF3\x81a,QV[a\x11b\x83\x83a,}V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[`\x07Ta\x01\0\x90\x04`\xFF\x16\x15a \x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInitialized: Already initialized`D\x82\x01R`d\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x85\x16a \xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FFungibleTokenWrapper: Fee Recipi`D\x82\x01Ru\x06V\xE7B\x04\x16FG&W72\x066\x16\xE2wB\x06&R\x03`T\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x84\x16a!\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a@\xE5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FFungibleTokenWrapper: Admin Addr`D\x82\x01Rm\x06W72\x066\x16\xE2wB\x06&R\x03`\x94\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a!\x91\x81a/QV[P`\x07\x80Ta\x01\0a\xFF\0\x19\x90\x91\x16\x17\x90U`\t\x80Ta\xFF\xFF\x96\x90\x96\x16`\x01`\x01`\xB0\x1B\x03\x19\x90\x96\x16\x95\x90\x95\x17b\x01\0\0`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x02\x17\x90\x94U`\x0B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x91U`\x11U`\x10\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\"+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x11\x81\x90U`@Q\x81\x81R\x7F\x14\xC1\x92\xA3{\xA7NZ&\x1A\x88m\xC0\xEAH\xAFB\xEF\x0F\x8A\xAF\x19\xC3\xAD\xFBa\xA2\x1C\xFE\xD3\x87\x1A\x90` \x01a\x1A\xA3V[`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;uV[`\x07Ta\x01\0\x90\x04`\xFF\x16a\"\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a;\xD2V[\x80c\xFF\xFF\xFF\xFF\x16\x80`\nT\x10a\"\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<\tV[`\nTa\"\xE7\x90`\x01a<bV[\x81\x11\x15a#\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x90a<uV[`\n\x81\x90U`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0E` R`@\x90 T`\xFF\x16\x15a#\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FFungibleTokenWrapper: Token shou`D\x82\x01Rn\x1B\x19\x08\x1B\x9B\xDD\x08\x18\x99H\x1D\x98[\x1AY`\x8A\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x0C\x80T`\x01\x81\x01\x90\x91U\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`\0\x90\x81R`\x0F` R`@\x90 T`\xFF\x16a$OW`\r\x80T`\x01\x80\x82\x01\x90\x92U\x7F\xD7\xB6\x99\x01\x05q\x91\x01\xDA\xBE\xB7qD\xF2\xA38\\\x803\xAC\xD3\xAF\x97\xE9B:i^\x81\xAD\x1E\xB5\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x81\x17\x90\x91U`\0\x90\x81R`\x0F` R`@\x90 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x0E` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90Q\x91\x82R\x7FxL\x8FM\xBF\x0F\xFE\xDDnr\xC7e\x01\xC5E\xA7\x0F\x8B ;0\xA2l\xE5B\xBF\x92\xBA\x87\xC2H\xA4\x91\x01a\r6V[a$\xAD\x82\x82a\x1A\xCDV[a\x13\x87W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua$\xE33\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x1A\xC6\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a/\x8CV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x0B\x1AWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x0B\x1AV[`\x01`\x01`\xA0\x1B\x03\x83\x16a%\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x82\x16a&4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x0C\x80Ta&\xA5\x90`\x01\x90aA5V[\x81T\x81\x10a&\xB5Wa&\xB5a<\xD2V[`\0\x91\x82R` \x90\x91 \x01T`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x83\x90\x81\x10a&\xE1Wa&\xE1a<\xD2V[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\x0C\x80T\x80a' Wa' aAHV[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPV[`\0a'Q\x84\x84a\x1F\xFDV[\x90P`\0\x19\x81\x14a\x19KW\x81\x81\x10\x15a'\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x0B\xDCV[a\x19K\x84\x84\x84\x84\x03a%qV[`\x01`\x01`\xA0\x1B\x03\x83\x16a(\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x82\x16a(\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a(\x8A\x83\x83\x83a/\xDBV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a)\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a)b\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x19KV[`\x02`\x08T\x03a)\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B\xDCV[`\x02`\x08UV[a)\xD2\x84\x83a-\xBEV[`\x01`\x01`\xA0\x1B\x03\x83\x16a*\x1CW`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x83\x15a\x08\xFC\x02\x90\x84\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a*\x16W=`\0\x80>=`\0\xFD[Pa*0V[a*0`\x01`\x01`\xA0\x1B\x03\x84\x16\x82\x84a/\xE6V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x7F\xC7\xCF2j\xE4\xD9F\"\x11v\xA4W\xCC0\x9D],\x15\xF3Tq\x11\xD8\xCC\xA3iQ\x07n\xEEM\x08\x85`@Qa*\x7F\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPV[`\0`\x11T0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xF4\x91\x90a=\xF8V[a*\xFE\x90\x84a<bV[\x11\x15\x92\x91PPV[`\0a+&`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a+ W\x83a\x19\xE3V[4a\x19\xE3V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a+HWa+C\x82\x85aA5V[a+RV[a+R\x824aA5V[\x90P`\x01`\x01`\xA0\x1B\x03\x85\x16a+\xA7W`\tT`@Qb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90a\x08\xFC\x84\x15\x02\x90\x84\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a+\xA1W=`\0\x80>=`\0\xFD[Pa+\xDFV[a+\xBC`\x01`\x01`\xA0\x1B\x03\x86\x16\x870\x84a0IV[`\tTa+\xDF\x90`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x89\x91b\x01\0\0\x90\x91\x04\x16\x85a0IV[a+\xE9\x83\x82a,\xF1V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F3\xB7F\xCF\xD2\x0B\x94@E\xB43\n\xE6\xF0\xC7me\xEBK99\xA9/l\n]\xCEN\x9E\x9F\x99\xD9\x85\x85`@Qa,A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPPV[a\x166\x813a0\x81V[a,e\x82\x82a$\xA3V[`\0\x82\x81R`\x01` R`@\x90 a\x11b\x90\x82a%'V[a,\x87\x82\x82a0\xDAV[`\0\x82\x81R`\x01` R`@\x90 a\x11b\x90\x82a1?V[a,\xA7a1TV[`\x07\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\x01`\x01`\xA0\x1B\x03\x82\x16a-GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x0B\xDCV[a-S`\0\x83\x83a/\xDBV[\x80`\x04`\0\x82\x82Ta-e\x91\x90a<bV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a.\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[a.*\x82`\0\x83a/\xDBV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a.\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`\x04\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a/\x06a1\x9DV[`\x07\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa,\xD43\x90V[`\0a\x1A\xC6\x83\x83a1\xE3V[`\0a\x0B\x1A\x82T\x90V[a/i`\0\x80Q` aBZ\x839\x81Q\x91R\x82a2\rV[a/t`\0\x82a2\rV[a\x166`\0\x80Q` aB:\x839\x81Q\x91R\x82a2\rV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta/\xD3WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x0B\x1AV[P`\0a\x0B\x1AV[a\x11b\x83\x83\x83a2\x17V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x11b\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra2}V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x19K\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01a0\x12V[a0\x8B\x82\x82a\x1A\xCDV[a\x13\x87Wa0\x98\x81a3OV[a0\xA3\x83` a3aV[`@Q` \x01a0\xB4\x92\x91\x90aA^V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x0B\xDC\x91`\x04\x01a7\xD5V[a0\xE4\x82\x82a\x1A\xCDV[\x15a\x13\x87W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x1A\xC6\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a4\xFDV[`\x07T`\xFF\x16a\x15\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x18]\\\xD8X\x9B\x19N\x88\x1B\x9B\xDD\x08\x1C\x18]\\\xD9Y`b\x1B`D\x82\x01R`d\x01a\x0B\xDCV[`\x07T`\xFF\x16\x15a\x15\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x14\x18]\\\xD8X\x9B\x19N\x88\x1C\x18]\\\xD9Y`\x82\x1B`D\x82\x01R`d\x01a\x0B\xDCV[`\0\x82`\0\x01\x82\x81T\x81\x10a1\xFAWa1\xFAa<\xD2V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[a\x13\x87\x82\x82a,[V[`\x07T`\xFF\x16\x15a\x11bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FERC20Pausable: token transfer wh`D\x82\x01Ri\x1A[\x19H\x1C\x18]\\\xD9Y`\xB2\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\0a2\xD2\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a5\xF7\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x11bW\x80\x80` \x01\x90Q\x81\x01\x90a2\xF0\x91\x90aA\xD3V[a\x11bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[``a\x0B\x1A`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a3p\x83`\x02a@\x91V[a3{\x90`\x02a<bV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\x93Wa3\x93aA\xF0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a3\xBDW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a3\xD8Wa3\xD8a<\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a4\x07Wa4\x07a<\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a4+\x84`\x02a@\x91V[a46\x90`\x01a<bV[\x90P[`\x01\x81\x11\x15a4\xAEWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a4jWa4ja<\xD2V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a4\x80Wa4\x80a<\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a4\xA7\x81aB\x06V[\x90Pa49V[P\x83\x15a\x1A\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x0B\xDCV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a5\xE6W`\0a5!`\x01\x83aA5V[\x85T\x90\x91P`\0\x90a55\x90`\x01\x90aA5V[\x90P\x81\x81\x14a5\x9AW`\0\x86`\0\x01\x82\x81T\x81\x10a5UWa5Ua<\xD2V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a5xWa5xa<\xD2V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a5\xABWa5\xABaAHV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x0B\x1AV[`\0\x91PPa\x0B\x1AV[P\x92\x91PPV[``a6\x06\x84\x84`\0\x85a6\x0EV[\x94\x93PPPPV[``\x82G\x10\x15a6oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x0B\xDCV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa6\x8B\x91\x90aB\x1DV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a6\xC8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a6\xCDV[``\x91P[P\x91P\x91Pa6\xDE\x87\x83\x83\x87a6\xE9V[\x97\x96PPPPPPPV[``\x83\x15a7XW\x82Q`\0\x03a7QW`\x01`\x01`\xA0\x1B\x03\x85\x16;a7QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0B\xDCV[P\x81a6\x06V[a6\x06\x83\x83\x81Q\x15a7mW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xDC\x91\x90a7\xD5V[`\0` \x82\x84\x03\x12\x15a7\x99W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1A\xC6W`\0\x80\xFD[`\0[\x83\x81\x10\x15a7\xCCW\x81\x81\x01Q\x83\x82\x01R` \x01a7\xB4V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra7\xF4\x81`@\x85\x01` \x87\x01a7\xB1V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x166W`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1EcW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a8DW`\0\x80\xFD[\x825a8O\x81a8\x08V[\x91Pa8]` \x84\x01a8\x1DV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a8yW`\0\x80\xFD[\x825a8\x84\x81a8\x08V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a8\xA7W`\0\x80\xFD[\x835a8\xB2\x81a8\x08V[\x92P` \x84\x015a8\xC2\x81a8\x08V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a8\xE5W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a8\xFFW`\0\x80\xFD[\x825\x91P` \x83\x015a9\x11\x81a8\x08V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a91W`\0\x80\xFD[\x835a9<\x81a8\x08V[\x92P` \x84\x015\x91P`@\x84\x015a9S\x81a8\x08V[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a9pW`\0\x80\xFD[\x815a\x1A\xC6\x81a8\x08V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a9\x91W`\0\x80\xFD[\x845a9\x9C\x81a8\x08V[\x93P` \x85\x015a9\xAC\x81a8\x08V[\x92P`@\x85\x015\x91P``\x85\x015a9\xC3\x81a8\x08V[\x93\x96\x92\x95P\x90\x93PPV[\x80\x15\x15\x81\x14a\x166W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a9\xEEW`\0\x80\xFD[\x815a\x1A\xC6\x81a9\xCEV[`\0\x80`@\x83\x85\x03\x12\x15a:\x0CW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a:\\W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a:7V[P\x90\x96\x95PPPPPPV[\x805a\xFF\xFF\x81\x16\x81\x14a\x1EcW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a:\x8DW`\0\x80\xFD[a8O\x83a:hV[`\0\x80`@\x83\x85\x03\x12\x15a:\xA9W`\0\x80\xFD[\x825a:\xB4\x81a8\x08V[\x91P` \x83\x015a9\x11\x81a8\x08V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a:\xDDW`\0\x80\xFD[a:\xE6\x87a:hV[\x95P` \x87\x015a:\xF6\x81a8\x08V[\x94P`@\x87\x015a;\x06\x81a8\x08V[\x93P``\x87\x015\x92P`\x80\x87\x015a;\x1D\x81a9\xCEV[\x91P`\xA0\x87\x015a;-\x81a8\x08V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01\x81\x81\x1C\x90\x82\x16\x80a;OW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a;oWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`9\x90\x82\x01R\x7FFungibleTokenWrapper: Only handl`@\x82\x01R\x7Fer can call this function\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x1C\x90\x82\x01R\x7FInitialized: Not initialized\0\0\0\0`@\x82\x01R``\x01\x90V[` \x80\x82R`#\x90\x82\x01R\x7FProposalNonceTracker: Invalid no`@\x82\x01Rbnce`\xE8\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\x1AWa\x0B\x1Aa<LV[` \x80\x82R`:\x90\x82\x01R\x7FProposalNonceTracker: Nonce must`@\x82\x01R\x7F not increment more than 1\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a<\xFAWa<\xFAa<LV[P`\x01\x01\x90V[` \x80\x82R`#\x90\x82\x01R\x7FTokenWrapper: must have minter r`@\x82\x01Rbole`\xE8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`)\x90\x82\x01R\x7FTokenWrapper: Insufficient nativ`@\x82\x01Rhe balance`\xB8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`E\x90\x82\x01R\x7FTokenWrapper: Native unwrapping `@\x82\x01R\x7Fis not allowed for this token wr``\x82\x01Rd0\xB882\xB9`\xD9\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0` \x82\x84\x03\x12\x15a>\nW`\0\x80\xFD[PQ\x91\x90PV[` \x80\x82R`(\x90\x82\x01R\x7FTokenWrapper: Insufficient ERC20`@\x82\x01Rg balance`\xC0\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`.\x90\x82\x01R\x7FTokenWrapper: Invalid historical`@\x82\x01Rm token address`\x90\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`9\x90\x82\x01R\x7FTokenWrapper: Invalid amount pro`@\x82\x01R\x7Fvided for native wrapping\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`C\x90\x82\x01R\x7FTokenWrapper: Native wrapping is`@\x82\x01R\x7F not allowed for this token wrap``\x82\x01Rb82\xB9`\xE9\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`-\x90\x82\x01R\x7FTokenWrapper: Invalid value sent`@\x82\x01Rl for wrapping`\x98\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`#\x90\x82\x01R\x7FTokenWrapper: Invalid token addr`@\x82\x01Rbess`\xE8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`2\x90\x82\x01R\x7FTokenWrapper: Fee Recipient cann`@\x82\x01Rqot be zero address`p\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\"\x90\x82\x01R\x7FTokenWrapper: Invalid token amou`@\x82\x01Ra\x1B\x9D`\xF2\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0B\x1AWa\x0B\x1Aa<LV[`\0\x82a@\xC5WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[a\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a5\xF0Wa5\xF0a<LV[` \x80\x82R`0\x90\x82\x01R\x7FFungibleTokenWrapper: Handler Ad`@\x82\x01Ro\x06G&W72\x066\x16\xE2wB\x06&R\x03`\x84\x1B``\x82\x01R`\x80\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0B\x1AWa\x0B\x1Aa<LV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83QaA\x96\x81`\x17\x85\x01` \x88\x01a7\xB1V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83QaA\xC7\x81`(\x84\x01` \x88\x01a7\xB1V[\x01`(\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aA\xE5W`\0\x80\xFD[\x81Qa\x1A\xC6\x81a9\xCEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x81aB\x15WaB\x15a<LV[P`\0\x19\x01\x90V[`\0\x82QaB/\x81\x84` \x87\x01a7\xB1V[\x91\x90\x91\x01\x92\x91PPV\xFEe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x9F-\xF0\xFE\xD2\xC7vH\xDEX`\xA4\xCCP\x8C\xD0\x81\x8C\x85\xB8\xB8\xA1\xABL\xEE\xEF\x8D\x98\x1C\x89V\xA6\xA2dipfsX\"\x12 \xCF\xA9<\x06]\x8D\xF0\x89\xFF\x1B0\xD8\x08\xB8O\x1F&`3[O\x1F\x8F\x02\xED\x96\xA4\xDD\x1C\x86\x110dsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static FUNGIBLETOKENWRAPPERCONTRACT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FungibleTokenWrapperContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FungibleTokenWrapperContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FungibleTokenWrapperContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FungibleTokenWrapperContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FungibleTokenWrapperContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FungibleTokenWrapperContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FungibleTokenWrapperContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                FUNGIBLETOKENWRAPPERCONTRACT_ABI.clone(),
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
                FUNGIBLETOKENWRAPPERCONTRACT_ABI.clone(),
                FUNGIBLETOKENWRAPPERCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MINTER_ROLE` (0xd5391393) function
        pub fn minter_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([213, 57, 19, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PAUSER_ROLE` (0xe63ab1e9) function
        pub fn pauser_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([230, 58, 177, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `add` (0xfc97a652) function
        pub fn add(
            &self,
            token_address: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 151, 166, 82], (token_address, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68) function
        pub fn burn(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnFrom` (0x79cc6790) function
        pub fn burn_from(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 204, 103, 144], (account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance` (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feePercentage` (0xa001ecdd) function
        pub fn fee_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([160, 1, 236, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeRecipient` (0x46904840) function
        pub fn fee_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 144, 72, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountToWrap` (0x96cd4dfe) function
        pub fn get_amount_to_wrap(
            &self,
            deposit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([150, 205, 77, 254], deposit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFee` (0xced72f87) function
        pub fn get_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([206, 215, 47, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFeeFromAmount` (0x85c00ae8) function
        pub fn get_fee_from_amount(
            &self,
            amount_to_wrap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([133, 192, 10, 232], amount_to_wrap)
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
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMember` (0x9010d07c) function
        pub fn get_role_member(
            &self,
            role: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 16, 208, 124], (role, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMemberCount` (0xca15c873) function
        pub fn get_role_member_count(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([202, 21, 200, 115], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokens` (0xaa6ca808) function
        pub fn get_tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([170, 108, 168, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
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
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `historicalTokens` (0x85d14834) function
        pub fn historical_tokens(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([133, 209, 72, 52], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `historicallyValid` (0xb1cba258) function
        pub fn historically_valid(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 203, 162, 88], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xf63ebb45) function
        pub fn initialize(
            &self,
            fee_percentage: u16,
            fee_recipient: ::ethers::core::types::Address,
            handler: ::ethers::core::types::Address,
            limit: ::ethers::core::types::U256,
            is_native_allowed: bool,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [246, 62, 187, 69],
                    (
                        fee_percentage,
                        fee_recipient,
                        handler,
                        limit,
                        is_native_allowed,
                        admin,
                    ),
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
        ///Calls the contract's `isNativeAllowed` (0xb3e4083f) function
        pub fn is_native_allowed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([179, 228, 8, 63], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidToken` (0xc1876453) function
        pub fn is_valid_token(
            &self,
            token_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([193, 135, 100, 83], token_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x40c10f19) function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String>
        {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
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
        ///Calls the contract's `remove` (0x1c4a1436) function
        pub fn remove(
            &self,
            token_address: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 74, 20, 54], (token_address, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFee` (0xc2ae4720) function
        pub fn set_fee(
            &self,
            fee_percentage: u16,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 174, 71, 32], (fee_percentage, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeRecipient` (0x07184f1c) function
        pub fn set_fee_recipient(
            &self,
            fee_recipient: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 24, 79, 28], (fee_recipient, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHandler` (0xbac426d0) function
        pub fn set_handler(
            &self,
            handler: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 196, 38, 208], handler)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNativeAllowed` (0x8b5478b9) function
        pub fn set_native_allowed(
            &self,
            is_native_allowed: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 84, 120, 185], is_native_allowed)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String>
        {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokens` (0x4f64b2be) function
        pub fn tokens(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([79, 100, 178, 190], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrap` (0x39f47693) function
        pub fn unwrap(
            &self,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 244, 118, 147], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapAndSendTo` (0x4808285e) function
        pub fn unwrap_and_send_to(
            &self,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [72, 8, 40, 94],
                    (token_address, amount, recipient),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapFor` (0x261c80b6) function
        pub fn unwrap_for(
            &self,
            sender: ::ethers::core::types::Address,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 28, 128, 182],
                    (sender, token_address, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateLimit` (0xfae0959a) function
        pub fn update_limit(
            &self,
            limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 224, 149, 154], limit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `valid` (0xac8a260c) function
        pub fn valid(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([172, 138, 38, 12], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wrap` (0xbf376c7a) function
        pub fn wrap(
            &self,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 55, 108, 122], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wrapFor` (0x2ca69388) function
        pub fn wrap_for(
            &self,
            sender: ::ethers::core::types::Address,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 166, 147, 136],
                    (sender, token_address, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wrapForAndSendTo` (0x7b2e30d6) function
        pub fn wrap_for_and_send_to(
            &self,
            sender: ::ethers::core::types::Address,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [123, 46, 48, 214],
                    (sender, token_address, amount, recipient),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wrappingLimit` (0x1f914382) function
        pub fn wrapping_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([31, 145, 67, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FeeRecipientUpdated` event
        pub fn fee_recipient_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FeeRecipientUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FeeUpdated` event
        pub fn fee_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FeeUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `HandlerUpdated` event
        pub fn handler_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HandlerUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NativeAllowed` event
        pub fn native_allowed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NativeAllowedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PausedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenAdded` event
        pub fn token_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenRemoved` event
        pub fn token_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unwrapping` event
        pub fn unwrapping_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnwrappingFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Wrapping` event
        pub fn wrapping_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WrappingFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WrappingLimitUpdated` event
        pub fn wrapping_limit_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WrappingLimitUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FungibleTokenWrapperContractEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>>
        for FungibleTokenWrapperContract<M>
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
        name = "FeeRecipientUpdated",
        abi = "FeeRecipientUpdated(address)"
    )]
    pub struct FeeRecipientUpdatedFilter {
        pub fee_recipient: ::ethers::core::types::Address,
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
    #[ethevent(name = "FeeUpdated", abi = "FeeUpdated(uint16)")]
    pub struct FeeUpdatedFilter {
        pub fee_percentage: u16,
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
    #[ethevent(name = "HandlerUpdated", abi = "HandlerUpdated(address)")]
    pub struct HandlerUpdatedFilter {
        pub handler: ::ethers::core::types::Address,
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
    #[ethevent(name = "NativeAllowed", abi = "NativeAllowed(bool)")]
    pub struct NativeAllowedFilter {
        pub is_native_allowed: bool,
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
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ::ethers::core::types::Address,
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
        name = "RoleGranted",
        abi = "RoleGranted(bytes32,address,address)"
    )]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
        name = "RoleRevoked",
        abi = "RoleRevoked(bytes32,address,address)"
    )]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "TokenAdded", abi = "TokenAdded(address)")]
    pub struct TokenAddedFilter {
        pub token_address: ::ethers::core::types::Address,
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
    #[ethevent(name = "TokenRemoved", abi = "TokenRemoved(address)")]
    pub struct TokenRemovedFilter {
        pub token_address: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ::ethers::core::types::Address,
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
        name = "Unwrapping",
        abi = "Unwrapping(address,address,address,uint256)"
    )]
    pub struct UnwrappingFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_address: ::ethers::core::types::Address,
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
        name = "Wrapping",
        abi = "Wrapping(address,address,address,uint256,uint256)"
    )]
    pub struct WrappingFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_address: ::ethers::core::types::Address,
        pub wrapping_fee: ::ethers::core::types::U256,
        pub after_fee_amount: ::ethers::core::types::U256,
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
        name = "WrappingLimitUpdated",
        abi = "WrappingLimitUpdated(uint256)"
    )]
    pub struct WrappingLimitUpdatedFilter {
        pub limit: ::ethers::core::types::U256,
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
    pub enum FungibleTokenWrapperContractEvents {
        ApprovalFilter(ApprovalFilter),
        FeeRecipientUpdatedFilter(FeeRecipientUpdatedFilter),
        FeeUpdatedFilter(FeeUpdatedFilter),
        HandlerUpdatedFilter(HandlerUpdatedFilter),
        NativeAllowedFilter(NativeAllowedFilter),
        PausedFilter(PausedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        TokenAddedFilter(TokenAddedFilter),
        TokenRemovedFilter(TokenRemovedFilter),
        TransferFilter(TransferFilter),
        UnpausedFilter(UnpausedFilter),
        UnwrappingFilter(UnwrappingFilter),
        WrappingFilter(WrappingFilter),
        WrappingLimitUpdatedFilter(WrappingLimitUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for FungibleTokenWrapperContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::ApprovalFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = FeeRecipientUpdatedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::FeeRecipientUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = FeeUpdatedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::FeeUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = HandlerUpdatedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::HandlerUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = NativeAllowedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::NativeAllowedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::PausedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::RoleAdminChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::RoleGrantedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::RoleRevokedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = TokenAddedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::TokenAddedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = TokenRemovedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::TokenRemovedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::TransferFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::UnpausedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UnwrappingFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::UnwrappingFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = WrappingFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::WrappingFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = WrappingLimitUpdatedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::WrappingLimitUpdatedFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FungibleTokenWrapperContractEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeRecipientUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HandlerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NativeAllowedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnwrappingFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WrappingFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WrappingLimitUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<FeeRecipientUpdatedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: FeeRecipientUpdatedFilter) -> Self {
            Self::FeeRecipientUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<FeeUpdatedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: FeeUpdatedFilter) -> Self {
            Self::FeeUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<HandlerUpdatedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: HandlerUpdatedFilter) -> Self {
            Self::HandlerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<NativeAllowedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: NativeAllowedFilter) -> Self {
            Self::NativeAllowedFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<TokenAddedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: TokenAddedFilter) -> Self {
            Self::TokenAddedFilter(value)
        }
    }
    impl ::core::convert::From<TokenRemovedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: TokenRemovedFilter) -> Self {
            Self::TokenRemovedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    impl ::core::convert::From<UnwrappingFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: UnwrappingFilter) -> Self {
            Self::UnwrappingFilter(value)
        }
    }
    impl ::core::convert::From<WrappingFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: WrappingFilter) -> Self {
            Self::WrappingFilter(value)
        }
    }
    impl ::core::convert::From<WrappingLimitUpdatedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: WrappingLimitUpdatedFilter) -> Self {
            Self::WrappingLimitUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `MINTER_ROLE` function with signature `MINTER_ROLE()` and selector `0xd5391393`
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
    #[ethcall(name = "MINTER_ROLE", abi = "MINTER_ROLE()")]
    pub struct MinterRoleCall;
    ///Container type for all input parameters for the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`
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
    #[ethcall(name = "PAUSER_ROLE", abi = "PAUSER_ROLE()")]
    pub struct PauserRoleCall;
    ///Container type for all input parameters for the `add` function with signature `add(address,uint32)` and selector `0xfc97a652`
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
    #[ethcall(name = "add", abi = "add(address,uint32)")]
    pub struct AddCall {
        pub token_address: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `0x42966c68`
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
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `burnFrom` function with signature `burnFrom(address,uint256)` and selector `0x79cc6790`
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
    #[ethcall(name = "burnFrom", abi = "burnFrom(address,uint256)")]
    pub struct BurnFromCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
        name = "decreaseAllowance",
        abi = "decreaseAllowance(address,uint256)"
    )]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `feePercentage` function with signature `feePercentage()` and selector `0xa001ecdd`
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
    #[ethcall(name = "feePercentage", abi = "feePercentage()")]
    pub struct FeePercentageCall;
    ///Container type for all input parameters for the `feeRecipient` function with signature `feeRecipient()` and selector `0x46904840`
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
    #[ethcall(name = "feeRecipient", abi = "feeRecipient()")]
    pub struct FeeRecipientCall;
    ///Container type for all input parameters for the `getAmountToWrap` function with signature `getAmountToWrap(uint256)` and selector `0x96cd4dfe`
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
    #[ethcall(name = "getAmountToWrap", abi = "getAmountToWrap(uint256)")]
    pub struct GetAmountToWrapCall {
        pub deposit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getFee` function with signature `getFee()` and selector `0xced72f87`
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
    #[ethcall(name = "getFee", abi = "getFee()")]
    pub struct GetFeeCall;
    ///Container type for all input parameters for the `getFeeFromAmount` function with signature `getFeeFromAmount(uint256)` and selector `0x85c00ae8`
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
    #[ethcall(name = "getFeeFromAmount", abi = "getFeeFromAmount(uint256)")]
    pub struct GetFeeFromAmountCall {
        pub amount_to_wrap: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    #[ethcall(name = "getRoleMember", abi = "getRoleMember(bytes32,uint256)")]
    pub struct GetRoleMemberCall {
        pub role: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `getTokens` function with signature `getTokens()` and selector `0xaa6ca808`
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
    #[ethcall(name = "getTokens", abi = "getTokens()")]
    pub struct GetTokensCall;
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `historicalTokens` function with signature `historicalTokens(uint256)` and selector `0x85d14834`
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
    #[ethcall(name = "historicalTokens", abi = "historicalTokens(uint256)")]
    pub struct HistoricalTokensCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `historicallyValid` function with signature `historicallyValid(address)` and selector `0xb1cba258`
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
    #[ethcall(name = "historicallyValid", abi = "historicallyValid(address)")]
    pub struct HistoricallyValidCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
        name = "increaseAllowance",
        abi = "increaseAllowance(address,uint256)"
    )]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint16,address,address,uint256,bool,address)` and selector `0xf63ebb45`
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
        name = "initialize",
        abi = "initialize(uint16,address,address,uint256,bool,address)"
    )]
    pub struct InitializeCall {
        pub fee_percentage: u16,
        pub fee_recipient: ::ethers::core::types::Address,
        pub handler: ::ethers::core::types::Address,
        pub limit: ::ethers::core::types::U256,
        pub is_native_allowed: bool,
        pub admin: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `isNativeAllowed` function with signature `isNativeAllowed()` and selector `0xb3e4083f`
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
    #[ethcall(name = "isNativeAllowed", abi = "isNativeAllowed()")]
    pub struct IsNativeAllowedCall;
    ///Container type for all input parameters for the `isValidToken` function with signature `isValidToken(address)` and selector `0xc1876453`
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
    #[ethcall(name = "isValidToken", abi = "isValidToken(address)")]
    pub struct IsValidTokenCall {
        pub token_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
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
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
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
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
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
    ///Container type for all input parameters for the `remove` function with signature `remove(address,uint32)` and selector `0x1c4a1436`
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
    #[ethcall(name = "remove", abi = "remove(address,uint32)")]
    pub struct RemoveCall {
        pub token_address: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFee` function with signature `setFee(uint16,uint32)` and selector `0xc2ae4720`
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
    #[ethcall(name = "setFee", abi = "setFee(uint16,uint32)")]
    pub struct SetFeeCall {
        pub fee_percentage: u16,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `setFeeRecipient` function with signature `setFeeRecipient(address,uint32)` and selector `0x07184f1c`
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
        name = "setFeeRecipient",
        abi = "setFeeRecipient(address,uint32)"
    )]
    pub struct SetFeeRecipientCall {
        pub fee_recipient: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `setHandler` function with signature `setHandler(address)` and selector `0xbac426d0`
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
    #[ethcall(name = "setHandler", abi = "setHandler(address)")]
    pub struct SetHandlerCall {
        pub handler: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setNativeAllowed` function with signature `setNativeAllowed(bool)` and selector `0x8b5478b9`
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
    #[ethcall(name = "setNativeAllowed", abi = "setNativeAllowed(bool)")]
    pub struct SetNativeAllowedCall {
        pub is_native_allowed: bool,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokens` function with signature `tokens(uint256)` and selector `0x4f64b2be`
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
    #[ethcall(name = "tokens", abi = "tokens(uint256)")]
    pub struct TokensCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
        name = "transferFrom",
        abi = "transferFrom(address,address,uint256)"
    )]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
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
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `unwrap` function with signature `unwrap(address,uint256)` and selector `0x39f47693`
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
    #[ethcall(name = "unwrap", abi = "unwrap(address,uint256)")]
    pub struct UnwrapCall {
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `unwrapAndSendTo` function with signature `unwrapAndSendTo(address,uint256,address)` and selector `0x4808285e`
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
        name = "unwrapAndSendTo",
        abi = "unwrapAndSendTo(address,uint256,address)"
    )]
    pub struct UnwrapAndSendToCall {
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unwrapFor` function with signature `unwrapFor(address,address,uint256)` and selector `0x261c80b6`
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
    #[ethcall(name = "unwrapFor", abi = "unwrapFor(address,address,uint256)")]
    pub struct UnwrapForCall {
        pub sender: ::ethers::core::types::Address,
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateLimit` function with signature `updateLimit(uint256)` and selector `0xfae0959a`
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
    #[ethcall(name = "updateLimit", abi = "updateLimit(uint256)")]
    pub struct UpdateLimitCall {
        pub limit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `valid` function with signature `valid(address)` and selector `0xac8a260c`
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
    #[ethcall(name = "valid", abi = "valid(address)")]
    pub struct ValidCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `wrap` function with signature `wrap(address,uint256)` and selector `0xbf376c7a`
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
    #[ethcall(name = "wrap", abi = "wrap(address,uint256)")]
    pub struct WrapCall {
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `wrapFor` function with signature `wrapFor(address,address,uint256)` and selector `0x2ca69388`
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
    #[ethcall(name = "wrapFor", abi = "wrapFor(address,address,uint256)")]
    pub struct WrapForCall {
        pub sender: ::ethers::core::types::Address,
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `wrapForAndSendTo` function with signature `wrapForAndSendTo(address,address,uint256,address)` and selector `0x7b2e30d6`
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
        name = "wrapForAndSendTo",
        abi = "wrapForAndSendTo(address,address,uint256,address)"
    )]
    pub struct WrapForAndSendToCall {
        pub sender: ::ethers::core::types::Address,
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `wrappingLimit` function with signature `wrappingLimit()` and selector `0x1f914382`
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
    #[ethcall(name = "wrappingLimit", abi = "wrappingLimit()")]
    pub struct WrappingLimitCall;
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
    pub enum FungibleTokenWrapperContractCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        MinterRole(MinterRoleCall),
        PauserRole(PauserRoleCall),
        Add(AddCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        BurnFrom(BurnFromCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        FeePercentage(FeePercentageCall),
        FeeRecipient(FeeRecipientCall),
        GetAmountToWrap(GetAmountToWrapCall),
        GetFee(GetFeeCall),
        GetFeeFromAmount(GetFeeFromAmountCall),
        GetProposalNonce(GetProposalNonceCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GetTokens(GetTokensCall),
        GrantRole(GrantRoleCall),
        Handler(HandlerCall),
        HasRole(HasRoleCall),
        HistoricalTokens(HistoricalTokensCall),
        HistoricallyValid(HistoricallyValidCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Initialize(InitializeCall),
        Initialized(InitializedCall),
        IsNativeAllowed(IsNativeAllowedCall),
        IsValidToken(IsValidTokenCall),
        Mint(MintCall),
        Name(NameCall),
        Pause(PauseCall),
        Paused(PausedCall),
        ProposalNonce(ProposalNonceCall),
        Remove(RemoveCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetFee(SetFeeCall),
        SetFeeRecipient(SetFeeRecipientCall),
        SetHandler(SetHandlerCall),
        SetNativeAllowed(SetNativeAllowedCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        Tokens(TokensCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Unpause(UnpauseCall),
        Unwrap(UnwrapCall),
        UnwrapAndSendTo(UnwrapAndSendToCall),
        UnwrapFor(UnwrapForCall),
        UpdateLimit(UpdateLimitCall),
        Valid(ValidCall),
        Wrap(WrapCall),
        WrapFor(WrapForCall),
        WrapForAndSendTo(WrapForAndSendToCall),
        WrappingLimit(WrappingLimitCall),
    }
    impl ::ethers::core::abi::AbiDecode for FungibleTokenWrapperContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <MinterRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinterRole(decoded));
            }
            if let Ok(decoded) =
                <PauserRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PauserRole(decoded));
            }
            if let Ok(decoded) =
                <AddCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Add(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) =
                <BurnFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BurnFrom(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <FeePercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::FeePercentage(decoded));
            }
            if let Ok(decoded) =
                <FeeRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::FeeRecipient(decoded));
            }
            if let Ok(decoded) =
                <GetAmountToWrapCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetAmountToWrap(decoded));
            }
            if let Ok(decoded) =
                <GetFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFee(decoded));
            }
            if let Ok(decoded) =
                <GetFeeFromAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetFeeFromAmount(decoded));
            }
            if let Ok(decoded) =
                <GetProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetRoleMemberCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetRoleMember(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMemberCount(decoded));
            }
            if let Ok(decoded) =
                <GetTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTokens(decoded));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HandlerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) =
                <HistoricalTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::HistoricalTokens(decoded));
            }
            if let Ok(decoded) = <HistoricallyValidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HistoricallyValid(decoded));
            }
            if let Ok(decoded) = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseAllowance(decoded));
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
            if let Ok(decoded) =
                <IsNativeAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsNativeAllowed(decoded));
            }
            if let Ok(decoded) =
                <IsValidTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsValidToken(decoded));
            }
            if let Ok(decoded) =
                <MintCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) =
                <NameCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) =
                <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) =
                <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) =
                <ProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <RemoveCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Remove(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <SetFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFee(decoded));
            }
            if let Ok(decoded) =
                <SetFeeRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetFeeRecipient(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <SetNativeAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetNativeAllowed(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Tokens(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UnwrapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Unwrap(decoded));
            }
            if let Ok(decoded) =
                <UnwrapAndSendToCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UnwrapAndSendTo(decoded));
            }
            if let Ok(decoded) =
                <UnwrapForCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnwrapFor(decoded));
            }
            if let Ok(decoded) =
                <UpdateLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UpdateLimit(decoded));
            }
            if let Ok(decoded) =
                <ValidCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Valid(decoded));
            }
            if let Ok(decoded) =
                <WrapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Wrap(decoded));
            }
            if let Ok(decoded) =
                <WrapForCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WrapFor(decoded));
            }
            if let Ok(decoded) =
                <WrapForAndSendToCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::WrapForAndSendTo(decoded));
            }
            if let Ok(decoded) =
                <WrappingLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::WrappingLimit(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FungibleTokenWrapperContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PauserRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Add(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeePercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountToWrap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFeeFromAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMember(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Handler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HistoricalTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HistoricallyValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsNativeAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Paused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Remove(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetNativeAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Tokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unwrap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapAndSendTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Valid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Wrap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrapFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrapForAndSendTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrappingLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FungibleTokenWrapperContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinterRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PauserRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Add(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Decimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeePercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAmountToWrap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFeeFromAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleMember(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Handler(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HistoricalTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HistoricallyValid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsNativeAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsValidToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Remove(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetNativeAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::Tokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Transfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unwrap(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapAndSendTo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnwrapFor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Valid(element) => ::core::fmt::Display::fmt(element, f),
                Self::Wrap(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrapFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrapForAndSendTo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WrappingLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<MinterRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: MinterRoleCall) -> Self {
            Self::MinterRole(value)
        }
    }
    impl ::core::convert::From<PauserRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: PauserRoleCall) -> Self {
            Self::PauserRole(value)
        }
    }
    impl ::core::convert::From<AddCall> for FungibleTokenWrapperContractCalls {
        fn from(value: AddCall) -> Self {
            Self::Add(value)
        }
    }
    impl ::core::convert::From<AllowanceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for FungibleTokenWrapperContractCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for FungibleTokenWrapperContractCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<BurnFromCall> for FungibleTokenWrapperContractCalls {
        fn from(value: BurnFromCall) -> Self {
            Self::BurnFrom(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for FungibleTokenWrapperContractCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<FeePercentageCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: FeePercentageCall) -> Self {
            Self::FeePercentage(value)
        }
    }
    impl ::core::convert::From<FeeRecipientCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: FeeRecipientCall) -> Self {
            Self::FeeRecipient(value)
        }
    }
    impl ::core::convert::From<GetAmountToWrapCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetAmountToWrapCall) -> Self {
            Self::GetAmountToWrap(value)
        }
    }
    impl ::core::convert::From<GetFeeCall> for FungibleTokenWrapperContractCalls {
        fn from(value: GetFeeCall) -> Self {
            Self::GetFee(value)
        }
    }
    impl ::core::convert::From<GetFeeFromAmountCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetFeeFromAmountCall) -> Self {
            Self::GetFeeFromAmount(value)
        }
    }
    impl ::core::convert::From<GetProposalNonceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetProposalNonceCall) -> Self {
            Self::GetProposalNonce(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GetTokensCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetTokensCall) -> Self {
            Self::GetTokens(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HandlerCall> for FungibleTokenWrapperContractCalls {
        fn from(value: HandlerCall) -> Self {
            Self::Handler(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for FungibleTokenWrapperContractCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<HistoricalTokensCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: HistoricalTokensCall) -> Self {
            Self::HistoricalTokens(value)
        }
    }
    impl ::core::convert::From<HistoricallyValidCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: HistoricallyValidCall) -> Self {
            Self::HistoricallyValid(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<InitializeCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InitializedCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: InitializedCall) -> Self {
            Self::Initialized(value)
        }
    }
    impl ::core::convert::From<IsNativeAllowedCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: IsNativeAllowedCall) -> Self {
            Self::IsNativeAllowed(value)
        }
    }
    impl ::core::convert::From<IsValidTokenCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: IsValidTokenCall) -> Self {
            Self::IsValidToken(value)
        }
    }
    impl ::core::convert::From<MintCall> for FungibleTokenWrapperContractCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for FungibleTokenWrapperContractCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<PauseCall> for FungibleTokenWrapperContractCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for FungibleTokenWrapperContractCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<ProposalNonceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: ProposalNonceCall) -> Self {
            Self::ProposalNonce(value)
        }
    }
    impl ::core::convert::From<RemoveCall> for FungibleTokenWrapperContractCalls {
        fn from(value: RemoveCall) -> Self {
            Self::Remove(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetFeeCall> for FungibleTokenWrapperContractCalls {
        fn from(value: SetFeeCall) -> Self {
            Self::SetFee(value)
        }
    }
    impl ::core::convert::From<SetFeeRecipientCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: SetFeeRecipientCall) -> Self {
            Self::SetFeeRecipient(value)
        }
    }
    impl ::core::convert::From<SetHandlerCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: SetHandlerCall) -> Self {
            Self::SetHandler(value)
        }
    }
    impl ::core::convert::From<SetNativeAllowedCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: SetNativeAllowedCall) -> Self {
            Self::SetNativeAllowed(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for FungibleTokenWrapperContractCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokensCall> for FungibleTokenWrapperContractCalls {
        fn from(value: TokensCall) -> Self {
            Self::Tokens(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for FungibleTokenWrapperContractCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for FungibleTokenWrapperContractCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UnwrapCall> for FungibleTokenWrapperContractCalls {
        fn from(value: UnwrapCall) -> Self {
            Self::Unwrap(value)
        }
    }
    impl ::core::convert::From<UnwrapAndSendToCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: UnwrapAndSendToCall) -> Self {
            Self::UnwrapAndSendTo(value)
        }
    }
    impl ::core::convert::From<UnwrapForCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: UnwrapForCall) -> Self {
            Self::UnwrapFor(value)
        }
    }
    impl ::core::convert::From<UpdateLimitCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: UpdateLimitCall) -> Self {
            Self::UpdateLimit(value)
        }
    }
    impl ::core::convert::From<ValidCall> for FungibleTokenWrapperContractCalls {
        fn from(value: ValidCall) -> Self {
            Self::Valid(value)
        }
    }
    impl ::core::convert::From<WrapCall> for FungibleTokenWrapperContractCalls {
        fn from(value: WrapCall) -> Self {
            Self::Wrap(value)
        }
    }
    impl ::core::convert::From<WrapForCall> for FungibleTokenWrapperContractCalls {
        fn from(value: WrapForCall) -> Self {
            Self::WrapFor(value)
        }
    }
    impl ::core::convert::From<WrapForAndSendToCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: WrapForAndSendToCall) -> Self {
            Self::WrapForAndSendTo(value)
        }
    }
    impl ::core::convert::From<WrappingLimitCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: WrappingLimitCall) -> Self {
            Self::WrappingLimit(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MINTER_ROLE` function with signature `MINTER_ROLE()` and selector `0xd5391393`
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
    pub struct MinterRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`
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
    pub struct PauserRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    pub struct DecreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `feePercentage` function with signature `feePercentage()` and selector `0xa001ecdd`
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
    pub struct FeePercentageReturn(pub u16);
    ///Container type for all return fields from the `feeRecipient` function with signature `feeRecipient()` and selector `0x46904840`
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
    pub struct FeeRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAmountToWrap` function with signature `getAmountToWrap(uint256)` and selector `0x96cd4dfe`
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
    pub struct GetAmountToWrapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getFee` function with signature `getFee()` and selector `0xced72f87`
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
    pub struct GetFeeReturn(pub u16);
    ///Container type for all return fields from the `getFeeFromAmount` function with signature `getFeeFromAmount(uint256)` and selector `0x85c00ae8`
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
    pub struct GetFeeFromAmountReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    pub struct GetRoleMemberReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    pub struct GetRoleMemberCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTokens` function with signature `getTokens()` and selector `0xaa6ca808`
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
    pub struct GetTokensReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
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
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `historicalTokens` function with signature `historicalTokens(uint256)` and selector `0x85d14834`
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
    pub struct HistoricalTokensReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `historicallyValid` function with signature `historicallyValid(address)` and selector `0xb1cba258`
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
    pub struct HistoricallyValidReturn(pub bool);
    ///Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    pub struct IncreaseAllowanceReturn(pub bool);
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
    ///Container type for all return fields from the `isNativeAllowed` function with signature `isNativeAllowed()` and selector `0xb3e4083f`
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
    pub struct IsNativeAllowedReturn(pub bool);
    ///Container type for all return fields from the `isValidToken` function with signature `isValidToken(address)` and selector `0xc1876453`
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
    pub struct IsValidTokenReturn(pub bool);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    pub struct PausedReturn(pub bool);
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
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokens` function with signature `tokens(uint256)` and selector `0x4f64b2be`
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
    pub struct TokensReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
    ///Container type for all return fields from the `valid` function with signature `valid(address)` and selector `0xac8a260c`
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
    pub struct ValidReturn(pub bool);
    ///Container type for all return fields from the `wrappingLimit` function with signature `wrappingLimit()` and selector `0x1f914382`
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
    pub struct WrappingLimitReturn(pub ::ethers::core::types::U256);
}
