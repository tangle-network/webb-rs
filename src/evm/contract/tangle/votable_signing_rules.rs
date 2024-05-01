pub use signing_rules_contract::*;
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
pub mod signing_rules_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_VOTERS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_VOTERS"),
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
                    ::std::borrow::ToOwned::to_owned("_proposals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_proposals"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ProposalStatus"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_yesVotes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_yesVotesTotal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proposedBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint40"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("adminSetForwarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("adminSetForwarder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase1JobId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forwarder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
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
                    ::std::borrow::ToOwned::to_owned("admins"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admins"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("calculatePhase2JobHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculatePhase2JobHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase1JobId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase2JobDetails"),
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
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("expiry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expiry"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProposalState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getProposalState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase2JobHash"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ProposalStatus"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProposalYesVotes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getProposalYesVotes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase2JobHash"),
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
                    ::std::borrow::ToOwned::to_owned("getProposalYesVotesTotal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getProposalYesVotesTotal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase2JobHash"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase1JobId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_useDemocracy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_voters"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_expiry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ttl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("isValidForwarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isValidForwarder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("refreshVoters"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("refreshVoters"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase1JobId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("submitGovernanceProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "submitGovernanceProposal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase1JobId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase2JobDetails"),
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
                    ::std::borrow::ToOwned::to_owned("threshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("threshold"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("ttl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ttl"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("useDemocracy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("useDemocracy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("useValidators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("useValidators"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("voteProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("voteProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase1JobId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phase2JobDetails"),
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
                    ::std::borrow::ToOwned::to_owned("voters"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("voters"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FailedHandlerExecution"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FailedHandlerExecution",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lowLevelData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposalEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposalEvent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("phase1JobId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("phase2JobHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposalVote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposalVote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("phase1JobId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("phase2JobHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
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
    pub static SIGNINGRULESCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x15\x80\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01,W`\x005`\xE0\x1C\x80c[\x9A^\xB7\x11a\0\xADW\x80c\xDBV\xB94\x11a\0qW\x80c\xDBV\xB94\x14a\x03YW\x80c\xEE\xA2+\xA2\x14a\x03lW\x80c\xF2\xE2\xAF\x94\x14a\x03\x7FW\x80c\xF7h\xB7M\x14a\x03\xD1W\x80c\xF8\xD4\x85\x8D\x14a\x03\xFAW`\0\x80\xFD[\x80c[\x9A^\xB7\x14a\x02\xACW\x80ch!s7\x14a\x02\xCFW\x80cj0\xED\x1C\x14a\x02\xE2W\x80c\x8A\x9A \x93\x14a\x03\x0BW\x80c\xA9\xB6-\x8D\x14a\x036W`\0\x80\xFD[\x80c\x1B5\xB0\xCC\x11a\0\xF4W\x80c\x1B5\xB0\xCC\x14a\x01\xF0W\x80c!}u\xAB\x14a\x02\x16W\x80c7\x18\xAC\xF8\x14a\x02WW\x80c<\xEE\xD6\x92\x14a\x02\x82W\x80cBlXc\x14a\x02\x99W`\0\x80\xFD[\x80c\x07\xFFA\x97\x14a\x011W\x80c\npOH\x14a\x01iW\x80c\x11\xF8\xDE\xC6\x14a\x01\x99W\x80c\x15\x8E\xF9>\x14a\x01\xCEW\x80c\x19\xBD\xDCa\x14a\x01\xDBW[`\0\x80\xFD[a\x01Ta\x01?6`\x04a\x10\x0BV[`\x07` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Ca\x01w6`\x04a\x10-V[`\0\x90\x81R`\x08` R`@\x90 T`\xFF\x16\x90V[`@Qa\x01`\x91\x90a\x10~V[a\x01\xBCa\x01\xA76`\x04a\x10\x0BV[`\x03` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01`V[`\tTa\x01T\x90`\xFF\x16\x81V[a\x01\xEEa\x01\xE96`\x04a\x10\x0BV[a\x04\rV[\0[a\x01\xBCa\x01\xFE6`\x04a\x10-V[`\0\x90\x81R`\x08` R`@\x90 `\x02\x01T`\xFF\x16\x90V[a\x02?a\x02$6`\x04a\x10\x0BV[`\x04` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01`V[a\x01Ta\x02e6`\x04a\x10\xA3V[`\0` \x81\x81R\x92\x81R`@\x80\x82 \x90\x93R\x90\x81R T`\xFF\x16\x81V[a\x02\x8Ba\x01\0\x81V[`@Q\x90\x81R` \x01a\x01`V[a\x01\xEEa\x02\xA76`\x04a\x11\x1CV[a\x04\\V[a\x01Ta\x02\xBA6`\x04a\x10\x0BV[`\x06` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\xEEa\x02\xDD6`\x04a\x11\xD1V[a\x04\xBCV[a\x02?a\x02\xF06`\x04a\x10\x0BV[`\x05` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x03\x1Ea\x03\x196`\x04a\x12\xD9V[a\x06AV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01`V[a\x02\x8Ba\x03D6`\x04a\x10-V[`\0\x90\x81R`\x08` R`@\x90 `\x01\x01T\x90V[a\x01\xEEa\x03g6`\x04a\x13\x03V[a\x06yV[a\x02\x8Ba\x03z6`\x04a\x11\x1CV[a\x06\xFAV[a\x03\xC1a\x03\x8D6`\x04a\x10-V[`\x08` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T`\xFF\x91\x82\x16\x92\x91\x81\x16\x90a\x01\0\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16\x84V[`@Qa\x01`\x94\x93\x92\x91\x90a\x13FV[a\x03\x1Ea\x03\xDF6`\x04a\x10\x0BV[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xEEa\x04\x086`\x04a\x11\x1CV[a\x07.V[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x81\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04XW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04O\x90a\x13yV[`@Q\x80\x91\x03\x90\xFD[PPV[a\x04f\x82\x82a\x08HV[a\x04\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FProposal must be votable\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04OV[a\x04X\x82\x82a\x08\xA4V[a\x01\0\x83Q\x11\x15a\x05\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnToo many voters`\x88\x1B`D\x82\x01R`d\x01a\x04OV[`\tT`\xFF\x16\x15a\x05JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x04OV[`\t\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x01`\x01`@\x1B\x03\x88\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x16`\xFF\x8D\x16\x17\x90U`\x06\x82R\x80\x83 \x80T\x90\x95\x16\x8A\x15\x15\x17\x90\x94U`\x04\x81R\x83\x82 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x89\x86\x16\x17\x90\x91U`\x05\x82R\x84\x83 \x80T\x90\x91\x16\x93\x87\x16\x93\x90\x93\x17\x90\x92U\x92\x90R\x90 \x80T3`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90U\x82Q\x15a\x06\x15W`\x01`\x01`@\x1B\x03\x86\x16`\0\x90\x81R`\x02` \x90\x81R`@\x90\x91 \x84Qa\x06\x0F\x92\x86\x01\x90a\x0FuV[Pa\x069V[`\x01`\x01`@\x1B\x03\x86\x16`\0\x90\x81R`\x07` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U[PPPPPPV[`\x02` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x06]W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`\x01`\x01`@\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x83\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04O\x90a\x13yV[P`\x01`\x01`@\x1B\x03\x90\x92\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x83R\x92\x90R \x80T\x91\x15\x15`\xFF\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x82\x82`@Q` \x01a\x07\x0F\x92\x91\x90a\x13\xDEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[`\0\x82\x82`@Q` \x01a\x07C\x92\x91\x90a\x13\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P`\x03`\0\x82\x81R`\x08` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x07\x82Wa\x07\x82a\x10FV[\x03a\x07\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FProposal must have been executed`D\x82\x01R`d\x01a\x04OV[`\x01`\x01`@\x1B\x03\x83\x16`\0\x90\x81R`\x06` R`@\x90 T`\xFF\x16a\x08CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FProposal must allow using govern`D\x82\x01Rcance`\xE0\x1B`d\x82\x01R`\x84\x01a\x04OV[PPPV[`\0\x81Q`\0\x03a\x08\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FJob details must be non-empty\0\0\0`D\x82\x01R`d\x01a\x04OV[P`\x01\x92\x91PPV[`\0\x82\x82`@Q` \x01a\x08\xB9\x92\x91\x90a\x13\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R`\x08\x90\x93R\x91 \x90\x91P`\x02\x81T`\xFF\x16`\x04\x81\x11\x15a\x08\xF8Wa\x08\xF8a\x10FV[\x03a\t\x0EWa\t\x08\x84\x83\x85a\r\x01V[PPPPV[`\0a\t\x19\x85a\x0E<V[\x82T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a\t5Wa\t5a\x10FV[\x11\x15a\t\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fproposal already executed/cancel`D\x82\x01Rb\x1B\x19Y`\xEA\x1B`d\x82\x01R`\x84\x01a\x04OV[a\t\x9A\x85\x84\x83a\x0E\x8EV[\x15a\t\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x99[\x18^Y\\\x88\x18[\x1C\x99XY\x1EH\x1D\x9B\xDD\x19Y`Z\x1B`D\x82\x01R`d\x01a\x04OV[`\0\x82T`\xFF\x16`\x04\x81\x11\x15a\t\xF7Wa\t\xF7a\x10FV[\x03a\n\xCDW`@\x80Q`\x80\x81\x01\x90\x91R\x80`\x01\x81R`\0` \x80\x83\x01\x82\x90R`@\x80\x84\x01\x83\x90Rd\xFF\xFF\xFF\xFF\xFFC\x16``\x90\x94\x01\x93\x90\x93R\x86\x82R`\x08\x90R \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a\nUWa\nUa\x10FV[\x02\x17\x90UP` \x82\x01Q`\x01\x80\x83\x01\x91\x90\x91U`@\x80\x84\x01Q`\x02\x90\x93\x01\x80T``\x90\x95\x01Qd\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16`\xFF\x90\x94\x16\x93\x90\x93\x17\x93\x90\x93\x17\x90\x91U\x90Q`\0\x80Q` a\x15+\x839\x81Q\x91R\x91a\n\xC0\x91\x88\x90\x87\x90a\x14\x13V[`@Q\x80\x91\x03\x90\xA1a\x0BHV[`\x01`\x01`@\x1B\x03\x85\x81\x16`\0\x90\x81R`\x04` R`@\x90 T`\x02\x84\x01T\x91\x16\x90a\x0B\x05\x90a\x01\0\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16Ca\x14QV[d\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x0BHW\x81T`\xFF\x19\x16`\x04\x90\x81\x17\x83U`@Q`\0\x80Q` a\x15+\x839\x81Q\x91R\x91a\x0B?\x91\x88\x90\x87\x90a\x14\x13V[`@Q\x80\x91\x03\x90\xA1[`\x04\x82T`\xFF\x16`\x04\x81\x11\x15a\x0B`Wa\x0B`a\x10FV[\x14a\x0CUWa\x0Bo\x85\x82a\x0E\xB8V[`\x01\x83\x01\x80T\x91\x90\x91\x17\x90U`\x02\x82\x01\x80T`\xFF\x16\x90`\0a\x0B\x90\x83a\x14dV[\x91\x90a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UPP\x7F \xFFJ\x81T,\x92M\x8Fe\xD7\xAB\x87'\xF9\xE3R\xB6O(\x8A@*\xCE\n^~\xDBh\xC2G\xDB\x82`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x86\x85`@Qa\x0B\xED\x93\x92\x91\x90a\x14\x13V[`@Q\x80\x91\x03\x90\xA1`\x01`\x01`@\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x90 T`\x02\x83\x01T`\xFF\x91\x82\x16\x91\x16\x10a\x0CUW\x81T`\xFF\x19\x16`\x02\x90\x81\x17\x83U`@Q`\0\x80Q` a\x15+\x839\x81Q\x91R\x91a\x0CL\x91\x88\x90\x87\x90a\x14\x13V[`@Q\x80\x91\x03\x90\xA1[`\0\x83\x81R`\x08` R`@\x90 \x82T\x81T\x84\x92\x91`\xFF\x16\x90\x82\x90`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a\x0C\x88Wa\x0C\x88a\x10FV[\x02\x17\x90UP`\x01\x82\x81\x01T\x90\x82\x01U`\x02\x91\x82\x01\x80T\x91\x83\x01\x80T`\xFF\x19\x81\x16`\xFF\x90\x94\x16\x93\x84\x17\x82U\x91Td\xFF\xFF\xFF\xFF\xFFa\x01\0\x91\x82\x90\x04\x16\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x90\x92\x17\x17\x90U\x82T`\xFF\x16`\x04\x81\x11\x15a\x0C\xEAWa\x0C\xEAa\x10FV[\x03a\x0C\xFAWa\x0C\xFA\x85\x84\x86a\r\x01V[PPPPPV[`\0\x82\x81R`\x08` R`@\x90 `\x02\x81T`\xFF\x16`\x04\x81\x11\x15a\r'Wa\r'a\x10FV[\x14a\rtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FProposal must have Passed status`D\x82\x01R`d\x01a\x04OV[`\x01`\x01`@\x1B\x03\x80\x85\x16`\0\x90\x81R`\x04` \x81\x81R`@\x80\x84 T`\x05\x83R\x81\x85 T\x82Q\x93\x84\x01\x83R\x94\x83R\x90Qc\x10u3A`\xE0\x1B\x81Ra\x08\x14\x95c\x10u3A\x95a\r\xCF\x95\x93\x82\x16\x94\x91\x16\x92\x8B\x92\x8A\x92\x91\x01a\x14\xAFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xE9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\xFDW=`\0\x80>=`\0\xFD[PP\x82T`\xFF\x19\x16`\x03\x90\x81\x17\x84U`@Q`\0\x80Q` a\x15+\x839\x81Q\x91R\x93Pa\x0E.\x92P\x87\x90\x87\x90a\x14\x13V[`@Q\x80\x91\x03\x90\xA1PPPPV[`\x003`\x146\x10\x80\x15\x90a\x0EzWP`\x01`\x01`@\x1B\x03\x83\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16[\x15a\x07(WPPP6`\x13\x19\x015``\x1C\x90V[`\0\x82\x81R`\x08` R`@\x81 `\x01\x81\x01T\x82\x90a\x0E\xAD\x87\x86a\x0E\xB8V[\x16\x11\x95\x94PPPPPV[`\0`\x01a\x0E\xC6\x84\x84a\x0E\xDBV[a\x0E\xD0\x91\x90a\x14QV[`\x01\x90\x1B\x93\x92PPPV[`\0\x80[`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x10\x15a\x0FjW`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90\x83\x90\x81\x10a\x0F5Wa\x0F5a\x15\x01V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0FbWa\x0FZ\x81`\x01a\x15\x17V[\x91PPa\x07(V[`\x01\x01a\x0E\xDFV[Pa\x01\0\x93\x92PPPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x0F\xCAW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x0F\xCAW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x0F\x95V[Pa\x0F\xD6\x92\x91Pa\x0F\xDAV[P\x90V[[\x80\x82\x11\x15a\x0F\xD6W`\0\x81U`\x01\x01a\x0F\xDBV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x10\x06W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10\x1DW`\0\x80\xFD[a\x10&\x82a\x0F\xEFV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x10?W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x05\x81\x10a\x10zWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x07(\x82\x84a\x10\\V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\x06W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xB6W`\0\x80\xFD[a\x10\xBF\x83a\x0F\xEFV[\x91Pa\x10\xCD` \x84\x01a\x10\x8CV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\x14Wa\x11\x14a\x10\xD6V[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x11/W`\0\x80\xFD[a\x118\x83a\x0F\xEFV[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11UW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x11iW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11{Wa\x11{a\x10\xD6V[a\x11\x8D`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x10\xECV[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x11\xA3W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x10\x06W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x11\xEAW`\0\x80\xFD[a\x11\xF3\x87a\x0F\xEFV[\x95P` \x80\x88\x015`\xFF\x81\x16\x81\x14a\x12\nW`\0\x80\xFD[\x95Pa\x12\x18`@\x89\x01a\x11\xC1V[\x94P``\x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x124W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x12HW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x12ZWa\x12Za\x10\xD6V[\x80`\x05\x1B\x91Pa\x12k\x84\x83\x01a\x10\xECV[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x8D\x84\x11\x15a\x12\x85W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x12\xAAWa\x12\x9B\x85a\x10\x8CV[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x12\x8AV[\x80\x98PPPPPPPa\x12\xBF`\x80\x88\x01a\x0F\xEFV[\x91Pa\x12\xCD`\xA0\x88\x01a\x0F\xEFV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15a\x12\xECW`\0\x80\xFD[a\x12\xF5\x83a\x0F\xEFV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\x18W`\0\x80\xFD[a\x13!\x84a\x0F\xEFV[\x92Pa\x13/` \x85\x01a\x10\x8CV[\x91Pa\x13=`@\x85\x01a\x11\xC1V[\x90P\x92P\x92P\x92V[`\x80\x81\x01a\x13T\x82\x87a\x10\\V[\x84` \x83\x01R`\xFF\x84\x16`@\x83\x01Rd\xFF\xFF\xFF\xFF\xFF\x83\x16``\x83\x01R\x95\x94PPPPPV[` \x80\x82R`!\x90\x82\x01R\x7FOnly admin can call this functio`@\x82\x01R`7`\xF9\x1B``\x82\x01R`\x80\x01\x90V[`\0[\x83\x81\x10\x15a\x13\xD5W\x81\x81\x01Q\x83\x82\x01R` \x01a\x13\xBDV[PP`\0\x91\x01RV[`\x01`\x01`@\x1B\x03`\xC0\x1B\x83`\xC0\x1B\x16\x81R`\0\x82Qa\x14\x05\x81`\x08\x85\x01` \x87\x01a\x13\xBAV[\x91\x90\x91\x01`\x08\x01\x93\x92PPPV[``\x81\x01a\x14!\x82\x86a\x10\\V[`\x01`\x01`@\x1B\x03\x93\x90\x93\x16` \x82\x01R`@\x01R\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07(Wa\x07(a\x14;V[`\0`\xFF\x82\x16`\xFF\x81\x03a\x14zWa\x14za\x14;V[`\x01\x01\x92\x91PPV[`\0\x81Q\x80\x84Ra\x14\x9B\x81` \x86\x01` \x86\x01a\x13\xBAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x88\x16\x83R\x80\x87\x16` \x84\x01R\x80\x86\x16`@\x84\x01RP`\xA0``\x83\x01Ra\x14\xE3`\xA0\x83\x01\x85a\x14\x83V[\x82\x81\x03`\x80\x84\x01Ra\x14\xF5\x81\x85a\x14\x83V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07(Wa\x07(a\x14;V\xFE\x1D\xA7Gsn6\xD4\xB2\x89\x19\xE5\x8C\xFC\xB8GuJv\x04\xD8\xAF\xB5\x1Ca#\xB3R\x14\xEE\xE0\xB4\x8B\xA2dipfsX\"\x12 U\x13\xAB\x92\xE5\xF3\xE7\xB9\x83\n\x1C]\xC0\xB0l\xEB>\xD6\x1B#\xEE\x94T\xB5C{\x83\x96\xB0\x9A\xD5,dsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static SIGNINGRULESCONTRACT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01,W`\x005`\xE0\x1C\x80c[\x9A^\xB7\x11a\0\xADW\x80c\xDBV\xB94\x11a\0qW\x80c\xDBV\xB94\x14a\x03YW\x80c\xEE\xA2+\xA2\x14a\x03lW\x80c\xF2\xE2\xAF\x94\x14a\x03\x7FW\x80c\xF7h\xB7M\x14a\x03\xD1W\x80c\xF8\xD4\x85\x8D\x14a\x03\xFAW`\0\x80\xFD[\x80c[\x9A^\xB7\x14a\x02\xACW\x80ch!s7\x14a\x02\xCFW\x80cj0\xED\x1C\x14a\x02\xE2W\x80c\x8A\x9A \x93\x14a\x03\x0BW\x80c\xA9\xB6-\x8D\x14a\x036W`\0\x80\xFD[\x80c\x1B5\xB0\xCC\x11a\0\xF4W\x80c\x1B5\xB0\xCC\x14a\x01\xF0W\x80c!}u\xAB\x14a\x02\x16W\x80c7\x18\xAC\xF8\x14a\x02WW\x80c<\xEE\xD6\x92\x14a\x02\x82W\x80cBlXc\x14a\x02\x99W`\0\x80\xFD[\x80c\x07\xFFA\x97\x14a\x011W\x80c\npOH\x14a\x01iW\x80c\x11\xF8\xDE\xC6\x14a\x01\x99W\x80c\x15\x8E\xF9>\x14a\x01\xCEW\x80c\x19\xBD\xDCa\x14a\x01\xDBW[`\0\x80\xFD[a\x01Ta\x01?6`\x04a\x10\x0BV[`\x07` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Ca\x01w6`\x04a\x10-V[`\0\x90\x81R`\x08` R`@\x90 T`\xFF\x16\x90V[`@Qa\x01`\x91\x90a\x10~V[a\x01\xBCa\x01\xA76`\x04a\x10\x0BV[`\x03` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01`V[`\tTa\x01T\x90`\xFF\x16\x81V[a\x01\xEEa\x01\xE96`\x04a\x10\x0BV[a\x04\rV[\0[a\x01\xBCa\x01\xFE6`\x04a\x10-V[`\0\x90\x81R`\x08` R`@\x90 `\x02\x01T`\xFF\x16\x90V[a\x02?a\x02$6`\x04a\x10\x0BV[`\x04` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01`V[a\x01Ta\x02e6`\x04a\x10\xA3V[`\0` \x81\x81R\x92\x81R`@\x80\x82 \x90\x93R\x90\x81R T`\xFF\x16\x81V[a\x02\x8Ba\x01\0\x81V[`@Q\x90\x81R` \x01a\x01`V[a\x01\xEEa\x02\xA76`\x04a\x11\x1CV[a\x04\\V[a\x01Ta\x02\xBA6`\x04a\x10\x0BV[`\x06` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\xEEa\x02\xDD6`\x04a\x11\xD1V[a\x04\xBCV[a\x02?a\x02\xF06`\x04a\x10\x0BV[`\x05` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x03\x1Ea\x03\x196`\x04a\x12\xD9V[a\x06AV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01`V[a\x02\x8Ba\x03D6`\x04a\x10-V[`\0\x90\x81R`\x08` R`@\x90 `\x01\x01T\x90V[a\x01\xEEa\x03g6`\x04a\x13\x03V[a\x06yV[a\x02\x8Ba\x03z6`\x04a\x11\x1CV[a\x06\xFAV[a\x03\xC1a\x03\x8D6`\x04a\x10-V[`\x08` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T`\xFF\x91\x82\x16\x92\x91\x81\x16\x90a\x01\0\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16\x84V[`@Qa\x01`\x94\x93\x92\x91\x90a\x13FV[a\x03\x1Ea\x03\xDF6`\x04a\x10\x0BV[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xEEa\x04\x086`\x04a\x11\x1CV[a\x07.V[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x81\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04XW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04O\x90a\x13yV[`@Q\x80\x91\x03\x90\xFD[PPV[a\x04f\x82\x82a\x08HV[a\x04\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FProposal must be votable\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04OV[a\x04X\x82\x82a\x08\xA4V[a\x01\0\x83Q\x11\x15a\x05\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnToo many voters`\x88\x1B`D\x82\x01R`d\x01a\x04OV[`\tT`\xFF\x16\x15a\x05JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x04OV[`\t\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x01`\x01`@\x1B\x03\x88\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x16`\xFF\x8D\x16\x17\x90U`\x06\x82R\x80\x83 \x80T\x90\x95\x16\x8A\x15\x15\x17\x90\x94U`\x04\x81R\x83\x82 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x89\x86\x16\x17\x90\x91U`\x05\x82R\x84\x83 \x80T\x90\x91\x16\x93\x87\x16\x93\x90\x93\x17\x90\x92U\x92\x90R\x90 \x80T3`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90U\x82Q\x15a\x06\x15W`\x01`\x01`@\x1B\x03\x86\x16`\0\x90\x81R`\x02` \x90\x81R`@\x90\x91 \x84Qa\x06\x0F\x92\x86\x01\x90a\x0FuV[Pa\x069V[`\x01`\x01`@\x1B\x03\x86\x16`\0\x90\x81R`\x07` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U[PPPPPPV[`\x02` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x06]W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`\x01`\x01`@\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x83\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04O\x90a\x13yV[P`\x01`\x01`@\x1B\x03\x90\x92\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x83R\x92\x90R \x80T\x91\x15\x15`\xFF\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x82\x82`@Q` \x01a\x07\x0F\x92\x91\x90a\x13\xDEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[`\0\x82\x82`@Q` \x01a\x07C\x92\x91\x90a\x13\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P`\x03`\0\x82\x81R`\x08` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x07\x82Wa\x07\x82a\x10FV[\x03a\x07\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FProposal must have been executed`D\x82\x01R`d\x01a\x04OV[`\x01`\x01`@\x1B\x03\x83\x16`\0\x90\x81R`\x06` R`@\x90 T`\xFF\x16a\x08CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FProposal must allow using govern`D\x82\x01Rcance`\xE0\x1B`d\x82\x01R`\x84\x01a\x04OV[PPPV[`\0\x81Q`\0\x03a\x08\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FJob details must be non-empty\0\0\0`D\x82\x01R`d\x01a\x04OV[P`\x01\x92\x91PPV[`\0\x82\x82`@Q` \x01a\x08\xB9\x92\x91\x90a\x13\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R`\x08\x90\x93R\x91 \x90\x91P`\x02\x81T`\xFF\x16`\x04\x81\x11\x15a\x08\xF8Wa\x08\xF8a\x10FV[\x03a\t\x0EWa\t\x08\x84\x83\x85a\r\x01V[PPPPV[`\0a\t\x19\x85a\x0E<V[\x82T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a\t5Wa\t5a\x10FV[\x11\x15a\t\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fproposal already executed/cancel`D\x82\x01Rb\x1B\x19Y`\xEA\x1B`d\x82\x01R`\x84\x01a\x04OV[a\t\x9A\x85\x84\x83a\x0E\x8EV[\x15a\t\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x99[\x18^Y\\\x88\x18[\x1C\x99XY\x1EH\x1D\x9B\xDD\x19Y`Z\x1B`D\x82\x01R`d\x01a\x04OV[`\0\x82T`\xFF\x16`\x04\x81\x11\x15a\t\xF7Wa\t\xF7a\x10FV[\x03a\n\xCDW`@\x80Q`\x80\x81\x01\x90\x91R\x80`\x01\x81R`\0` \x80\x83\x01\x82\x90R`@\x80\x84\x01\x83\x90Rd\xFF\xFF\xFF\xFF\xFFC\x16``\x90\x94\x01\x93\x90\x93R\x86\x82R`\x08\x90R \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a\nUWa\nUa\x10FV[\x02\x17\x90UP` \x82\x01Q`\x01\x80\x83\x01\x91\x90\x91U`@\x80\x84\x01Q`\x02\x90\x93\x01\x80T``\x90\x95\x01Qd\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16`\xFF\x90\x94\x16\x93\x90\x93\x17\x93\x90\x93\x17\x90\x91U\x90Q`\0\x80Q` a\x15+\x839\x81Q\x91R\x91a\n\xC0\x91\x88\x90\x87\x90a\x14\x13V[`@Q\x80\x91\x03\x90\xA1a\x0BHV[`\x01`\x01`@\x1B\x03\x85\x81\x16`\0\x90\x81R`\x04` R`@\x90 T`\x02\x84\x01T\x91\x16\x90a\x0B\x05\x90a\x01\0\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16Ca\x14QV[d\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x0BHW\x81T`\xFF\x19\x16`\x04\x90\x81\x17\x83U`@Q`\0\x80Q` a\x15+\x839\x81Q\x91R\x91a\x0B?\x91\x88\x90\x87\x90a\x14\x13V[`@Q\x80\x91\x03\x90\xA1[`\x04\x82T`\xFF\x16`\x04\x81\x11\x15a\x0B`Wa\x0B`a\x10FV[\x14a\x0CUWa\x0Bo\x85\x82a\x0E\xB8V[`\x01\x83\x01\x80T\x91\x90\x91\x17\x90U`\x02\x82\x01\x80T`\xFF\x16\x90`\0a\x0B\x90\x83a\x14dV[\x91\x90a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UPP\x7F \xFFJ\x81T,\x92M\x8Fe\xD7\xAB\x87'\xF9\xE3R\xB6O(\x8A@*\xCE\n^~\xDBh\xC2G\xDB\x82`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x86\x85`@Qa\x0B\xED\x93\x92\x91\x90a\x14\x13V[`@Q\x80\x91\x03\x90\xA1`\x01`\x01`@\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x90 T`\x02\x83\x01T`\xFF\x91\x82\x16\x91\x16\x10a\x0CUW\x81T`\xFF\x19\x16`\x02\x90\x81\x17\x83U`@Q`\0\x80Q` a\x15+\x839\x81Q\x91R\x91a\x0CL\x91\x88\x90\x87\x90a\x14\x13V[`@Q\x80\x91\x03\x90\xA1[`\0\x83\x81R`\x08` R`@\x90 \x82T\x81T\x84\x92\x91`\xFF\x16\x90\x82\x90`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a\x0C\x88Wa\x0C\x88a\x10FV[\x02\x17\x90UP`\x01\x82\x81\x01T\x90\x82\x01U`\x02\x91\x82\x01\x80T\x91\x83\x01\x80T`\xFF\x19\x81\x16`\xFF\x90\x94\x16\x93\x84\x17\x82U\x91Td\xFF\xFF\xFF\xFF\xFFa\x01\0\x91\x82\x90\x04\x16\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x90\x92\x17\x17\x90U\x82T`\xFF\x16`\x04\x81\x11\x15a\x0C\xEAWa\x0C\xEAa\x10FV[\x03a\x0C\xFAWa\x0C\xFA\x85\x84\x86a\r\x01V[PPPPPV[`\0\x82\x81R`\x08` R`@\x90 `\x02\x81T`\xFF\x16`\x04\x81\x11\x15a\r'Wa\r'a\x10FV[\x14a\rtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FProposal must have Passed status`D\x82\x01R`d\x01a\x04OV[`\x01`\x01`@\x1B\x03\x80\x85\x16`\0\x90\x81R`\x04` \x81\x81R`@\x80\x84 T`\x05\x83R\x81\x85 T\x82Q\x93\x84\x01\x83R\x94\x83R\x90Qc\x10u3A`\xE0\x1B\x81Ra\x08\x14\x95c\x10u3A\x95a\r\xCF\x95\x93\x82\x16\x94\x91\x16\x92\x8B\x92\x8A\x92\x91\x01a\x14\xAFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xE9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\xFDW=`\0\x80>=`\0\xFD[PP\x82T`\xFF\x19\x16`\x03\x90\x81\x17\x84U`@Q`\0\x80Q` a\x15+\x839\x81Q\x91R\x93Pa\x0E.\x92P\x87\x90\x87\x90a\x14\x13V[`@Q\x80\x91\x03\x90\xA1PPPPV[`\x003`\x146\x10\x80\x15\x90a\x0EzWP`\x01`\x01`@\x1B\x03\x83\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16[\x15a\x07(WPPP6`\x13\x19\x015``\x1C\x90V[`\0\x82\x81R`\x08` R`@\x81 `\x01\x81\x01T\x82\x90a\x0E\xAD\x87\x86a\x0E\xB8V[\x16\x11\x95\x94PPPPPV[`\0`\x01a\x0E\xC6\x84\x84a\x0E\xDBV[a\x0E\xD0\x91\x90a\x14QV[`\x01\x90\x1B\x93\x92PPPV[`\0\x80[`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x10\x15a\x0FjW`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90\x83\x90\x81\x10a\x0F5Wa\x0F5a\x15\x01V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0FbWa\x0FZ\x81`\x01a\x15\x17V[\x91PPa\x07(V[`\x01\x01a\x0E\xDFV[Pa\x01\0\x93\x92PPPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x0F\xCAW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x0F\xCAW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x0F\x95V[Pa\x0F\xD6\x92\x91Pa\x0F\xDAV[P\x90V[[\x80\x82\x11\x15a\x0F\xD6W`\0\x81U`\x01\x01a\x0F\xDBV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x10\x06W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10\x1DW`\0\x80\xFD[a\x10&\x82a\x0F\xEFV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x10?W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x05\x81\x10a\x10zWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x07(\x82\x84a\x10\\V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\x06W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xB6W`\0\x80\xFD[a\x10\xBF\x83a\x0F\xEFV[\x91Pa\x10\xCD` \x84\x01a\x10\x8CV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\x14Wa\x11\x14a\x10\xD6V[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x11/W`\0\x80\xFD[a\x118\x83a\x0F\xEFV[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11UW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x11iW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11{Wa\x11{a\x10\xD6V[a\x11\x8D`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x10\xECV[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x11\xA3W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x10\x06W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x11\xEAW`\0\x80\xFD[a\x11\xF3\x87a\x0F\xEFV[\x95P` \x80\x88\x015`\xFF\x81\x16\x81\x14a\x12\nW`\0\x80\xFD[\x95Pa\x12\x18`@\x89\x01a\x11\xC1V[\x94P``\x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x124W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x12HW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x12ZWa\x12Za\x10\xD6V[\x80`\x05\x1B\x91Pa\x12k\x84\x83\x01a\x10\xECV[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x8D\x84\x11\x15a\x12\x85W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x12\xAAWa\x12\x9B\x85a\x10\x8CV[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x12\x8AV[\x80\x98PPPPPPPa\x12\xBF`\x80\x88\x01a\x0F\xEFV[\x91Pa\x12\xCD`\xA0\x88\x01a\x0F\xEFV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15a\x12\xECW`\0\x80\xFD[a\x12\xF5\x83a\x0F\xEFV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\x18W`\0\x80\xFD[a\x13!\x84a\x0F\xEFV[\x92Pa\x13/` \x85\x01a\x10\x8CV[\x91Pa\x13=`@\x85\x01a\x11\xC1V[\x90P\x92P\x92P\x92V[`\x80\x81\x01a\x13T\x82\x87a\x10\\V[\x84` \x83\x01R`\xFF\x84\x16`@\x83\x01Rd\xFF\xFF\xFF\xFF\xFF\x83\x16``\x83\x01R\x95\x94PPPPPV[` \x80\x82R`!\x90\x82\x01R\x7FOnly admin can call this functio`@\x82\x01R`7`\xF9\x1B``\x82\x01R`\x80\x01\x90V[`\0[\x83\x81\x10\x15a\x13\xD5W\x81\x81\x01Q\x83\x82\x01R` \x01a\x13\xBDV[PP`\0\x91\x01RV[`\x01`\x01`@\x1B\x03`\xC0\x1B\x83`\xC0\x1B\x16\x81R`\0\x82Qa\x14\x05\x81`\x08\x85\x01` \x87\x01a\x13\xBAV[\x91\x90\x91\x01`\x08\x01\x93\x92PPPV[``\x81\x01a\x14!\x82\x86a\x10\\V[`\x01`\x01`@\x1B\x03\x93\x90\x93\x16` \x82\x01R`@\x01R\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07(Wa\x07(a\x14;V[`\0`\xFF\x82\x16`\xFF\x81\x03a\x14zWa\x14za\x14;V[`\x01\x01\x92\x91PPV[`\0\x81Q\x80\x84Ra\x14\x9B\x81` \x86\x01` \x86\x01a\x13\xBAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x88\x16\x83R\x80\x87\x16` \x84\x01R\x80\x86\x16`@\x84\x01RP`\xA0``\x83\x01Ra\x14\xE3`\xA0\x83\x01\x85a\x14\x83V[\x82\x81\x03`\x80\x84\x01Ra\x14\xF5\x81\x85a\x14\x83V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07(Wa\x07(a\x14;V\xFE\x1D\xA7Gsn6\xD4\xB2\x89\x19\xE5\x8C\xFC\xB8GuJv\x04\xD8\xAF\xB5\x1Ca#\xB3R\x14\xEE\xE0\xB4\x8B\xA2dipfsX\"\x12 U\x13\xAB\x92\xE5\xF3\xE7\xB9\x83\n\x1C]\xC0\xB0l\xEB>\xD6\x1B#\xEE\x94T\xB5C{\x83\x96\xB0\x9A\xD5,dsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static SIGNINGRULESCONTRACT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct SigningRulesContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SigningRulesContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SigningRulesContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SigningRulesContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SigningRulesContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SigningRulesContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SigningRulesContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SIGNINGRULESCONTRACT_ABI.clone(),
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
                SIGNINGRULESCONTRACT_ABI.clone(),
                SIGNINGRULESCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_VOTERS` (0x3ceed692) function
        pub fn max_voters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([60, 238, 214, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_proposals` (0xf2e2af94) function
        pub fn proposals(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, ::ethers::core::types::U256, u8, u64),
        > {
            self.0
                .method_hash([242, 226, 175, 148], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `adminSetForwarder` (0xdb56b934) function
        pub fn admin_set_forwarder(
            &self,
            phase_1_job_id: u64,
            forwarder: ::ethers::core::types::Address,
            valid: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [219, 86, 185, 52],
                    (phase_1_job_id, forwarder, valid),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admins` (0xf768b74d) function
        pub fn admins(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([247, 104, 183, 77], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculatePhase2JobHash` (0xeea22ba2) function
        pub fn calculate_phase_2_job_hash(
            &self,
            phase_1_job_id: u64,
            phase_2_job_details: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [238, 162, 43, 162],
                    (phase_1_job_id, phase_2_job_details),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expiry` (0x217d75ab) function
        pub fn expiry(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([33, 125, 117, 171], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProposalState` (0x0a704f48) function
        pub fn get_proposal_state(
            &self,
            phase_2_job_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([10, 112, 79, 72], phase_2_job_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProposalYesVotes` (0xa9b62d8d) function
        pub fn get_proposal_yes_votes(
            &self,
            phase_2_job_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([169, 182, 45, 141], phase_2_job_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProposalYesVotesTotal` (0x1b35b0cc) function
        pub fn get_proposal_yes_votes_total(
            &self,
            phase_2_job_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([27, 53, 176, 204], phase_2_job_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x68217337) function
        pub fn initialize(
            &self,
            phase_1_job_id: u64,
            threshold: u8,
            use_democracy: bool,
            voters: ::std::vec::Vec<::ethers::core::types::Address>,
            expiry: u64,
            ttl: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [104, 33, 115, 55],
                    (
                        phase_1_job_id,
                        threshold,
                        use_democracy,
                        voters,
                        expiry,
                        ttl,
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
        ///Calls the contract's `isValidForwarder` (0x3718acf8) function
        pub fn is_valid_forwarder(
            &self,
            p0: u64,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([55, 24, 172, 248], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refreshVoters` (0x19bddc61) function
        pub fn refresh_voters(
            &self,
            phase_1_job_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 189, 220, 97], phase_1_job_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitGovernanceProposal` (0xf8d4858d) function
        pub fn submit_governance_proposal(
            &self,
            phase_1_job_id: u64,
            phase_2_job_details: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 212, 133, 141],
                    (phase_1_job_id, phase_2_job_details),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `threshold` (0x11f8dec6) function
        pub fn threshold(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([17, 248, 222, 198], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ttl` (0x6a30ed1c) function
        pub fn ttl(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([106, 48, 237, 28], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `useDemocracy` (0x5b9a5eb7) function
        pub fn use_democracy(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([91, 154, 94, 183], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `useValidators` (0x07ff4197) function
        pub fn use_validators(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([7, 255, 65, 151], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voteProposal` (0x426c5863) function
        pub fn vote_proposal(
            &self,
            phase_1_job_id: u64,
            phase_2_job_details: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [66, 108, 88, 99],
                    (phase_1_job_id, phase_2_job_details),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voters` (0x8a9a2093) function
        pub fn voters(
            &self,
            p0: u64,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([138, 154, 32, 147], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FailedHandlerExecution` event
        pub fn failed_handler_execution_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FailedHandlerExecutionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalEvent` event
        pub fn proposal_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalEventFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalVote` event
        pub fn proposal_vote_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalVoteFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SigningRulesContractEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for SigningRulesContract<M>
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
        name = "FailedHandlerExecution",
        abi = "FailedHandlerExecution(bytes)"
    )]
    pub struct FailedHandlerExecutionFilter {
        pub low_level_data: ::ethers::core::types::Bytes,
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
        name = "ProposalEvent",
        abi = "ProposalEvent(uint8,uint64,bytes32)"
    )]
    pub struct ProposalEventFilter {
        pub status: u8,
        pub phase_1_job_id: u64,
        pub phase_2_job_hash: [u8; 32],
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
        name = "ProposalVote",
        abi = "ProposalVote(uint8,uint64,bytes32)"
    )]
    pub struct ProposalVoteFilter {
        pub status: u8,
        pub phase_1_job_id: u64,
        pub phase_2_job_hash: [u8; 32],
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
    pub enum SigningRulesContractEvents {
        FailedHandlerExecutionFilter(FailedHandlerExecutionFilter),
        ProposalEventFilter(ProposalEventFilter),
        ProposalVoteFilter(ProposalVoteFilter),
    }
    impl ::ethers::contract::EthLogDecode for SigningRulesContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FailedHandlerExecutionFilter::decode_log(log) {
                return Ok(
                    SigningRulesContractEvents::FailedHandlerExecutionFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = ProposalEventFilter::decode_log(log) {
                return Ok(SigningRulesContractEvents::ProposalEventFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ProposalVoteFilter::decode_log(log) {
                return Ok(SigningRulesContractEvents::ProposalVoteFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SigningRulesContractEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::FailedHandlerExecutionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalEventFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalVoteFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FailedHandlerExecutionFilter>
        for SigningRulesContractEvents
    {
        fn from(value: FailedHandlerExecutionFilter) -> Self {
            Self::FailedHandlerExecutionFilter(value)
        }
    }
    impl ::core::convert::From<ProposalEventFilter> for SigningRulesContractEvents {
        fn from(value: ProposalEventFilter) -> Self {
            Self::ProposalEventFilter(value)
        }
    }
    impl ::core::convert::From<ProposalVoteFilter> for SigningRulesContractEvents {
        fn from(value: ProposalVoteFilter) -> Self {
            Self::ProposalVoteFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_VOTERS` function with signature `MAX_VOTERS()` and selector `0x3ceed692`
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
    #[ethcall(name = "MAX_VOTERS", abi = "MAX_VOTERS()")]
    pub struct MaxVotersCall;
    ///Container type for all input parameters for the `_proposals` function with signature `_proposals(bytes32)` and selector `0xf2e2af94`
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
    #[ethcall(name = "_proposals", abi = "_proposals(bytes32)")]
    pub struct ProposalsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `adminSetForwarder` function with signature `adminSetForwarder(uint64,address,bool)` and selector `0xdb56b934`
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
        name = "adminSetForwarder",
        abi = "adminSetForwarder(uint64,address,bool)"
    )]
    pub struct AdminSetForwarderCall {
        pub phase_1_job_id: u64,
        pub forwarder: ::ethers::core::types::Address,
        pub valid: bool,
    }
    ///Container type for all input parameters for the `admins` function with signature `admins(uint64)` and selector `0xf768b74d`
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
    #[ethcall(name = "admins", abi = "admins(uint64)")]
    pub struct AdminsCall(pub u64);
    ///Container type for all input parameters for the `calculatePhase2JobHash` function with signature `calculatePhase2JobHash(uint64,bytes)` and selector `0xeea22ba2`
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
        name = "calculatePhase2JobHash",
        abi = "calculatePhase2JobHash(uint64,bytes)"
    )]
    pub struct CalculatePhase2JobHashCall {
        pub phase_1_job_id: u64,
        pub phase_2_job_details: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `expiry` function with signature `expiry(uint64)` and selector `0x217d75ab`
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
    #[ethcall(name = "expiry", abi = "expiry(uint64)")]
    pub struct ExpiryCall(pub u64);
    ///Container type for all input parameters for the `getProposalState` function with signature `getProposalState(bytes32)` and selector `0x0a704f48`
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
    #[ethcall(name = "getProposalState", abi = "getProposalState(bytes32)")]
    pub struct GetProposalStateCall {
        pub phase_2_job_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getProposalYesVotes` function with signature `getProposalYesVotes(bytes32)` and selector `0xa9b62d8d`
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
        name = "getProposalYesVotes",
        abi = "getProposalYesVotes(bytes32)"
    )]
    pub struct GetProposalYesVotesCall {
        pub phase_2_job_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getProposalYesVotesTotal` function with signature `getProposalYesVotesTotal(bytes32)` and selector `0x1b35b0cc`
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
        name = "getProposalYesVotesTotal",
        abi = "getProposalYesVotesTotal(bytes32)"
    )]
    pub struct GetProposalYesVotesTotalCall {
        pub phase_2_job_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint64,uint8,bool,address[],uint64,uint64)` and selector `0x68217337`
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
        abi = "initialize(uint64,uint8,bool,address[],uint64,uint64)"
    )]
    pub struct InitializeCall {
        pub phase_1_job_id: u64,
        pub threshold: u8,
        pub use_democracy: bool,
        pub voters: ::std::vec::Vec<::ethers::core::types::Address>,
        pub expiry: u64,
        pub ttl: u64,
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
    ///Container type for all input parameters for the `isValidForwarder` function with signature `isValidForwarder(uint64,address)` and selector `0x3718acf8`
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
        name = "isValidForwarder",
        abi = "isValidForwarder(uint64,address)"
    )]
    pub struct IsValidForwarderCall(
        pub u64,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `refreshVoters` function with signature `refreshVoters(uint64)` and selector `0x19bddc61`
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
    #[ethcall(name = "refreshVoters", abi = "refreshVoters(uint64)")]
    pub struct RefreshVotersCall {
        pub phase_1_job_id: u64,
    }
    ///Container type for all input parameters for the `submitGovernanceProposal` function with signature `submitGovernanceProposal(uint64,bytes)` and selector `0xf8d4858d`
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
        name = "submitGovernanceProposal",
        abi = "submitGovernanceProposal(uint64,bytes)"
    )]
    pub struct SubmitGovernanceProposalCall {
        pub phase_1_job_id: u64,
        pub phase_2_job_details: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `threshold` function with signature `threshold(uint64)` and selector `0x11f8dec6`
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
    #[ethcall(name = "threshold", abi = "threshold(uint64)")]
    pub struct ThresholdCall(pub u64);
    ///Container type for all input parameters for the `ttl` function with signature `ttl(uint64)` and selector `0x6a30ed1c`
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
    #[ethcall(name = "ttl", abi = "ttl(uint64)")]
    pub struct TtlCall(pub u64);
    ///Container type for all input parameters for the `useDemocracy` function with signature `useDemocracy(uint64)` and selector `0x5b9a5eb7`
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
    #[ethcall(name = "useDemocracy", abi = "useDemocracy(uint64)")]
    pub struct UseDemocracyCall(pub u64);
    ///Container type for all input parameters for the `useValidators` function with signature `useValidators(uint64)` and selector `0x07ff4197`
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
    #[ethcall(name = "useValidators", abi = "useValidators(uint64)")]
    pub struct UseValidatorsCall(pub u64);
    ///Container type for all input parameters for the `voteProposal` function with signature `voteProposal(uint64,bytes)` and selector `0x426c5863`
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
    #[ethcall(name = "voteProposal", abi = "voteProposal(uint64,bytes)")]
    pub struct VoteProposalCall {
        pub phase_1_job_id: u64,
        pub phase_2_job_details: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `voters` function with signature `voters(uint64,uint256)` and selector `0x8a9a2093`
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
    #[ethcall(name = "voters", abi = "voters(uint64,uint256)")]
    pub struct VotersCall(pub u64, pub ::ethers::core::types::U256);
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
    pub enum SigningRulesContractCalls {
        MaxVoters(MaxVotersCall),
        Proposals(ProposalsCall),
        AdminSetForwarder(AdminSetForwarderCall),
        Admins(AdminsCall),
        CalculatePhase2JobHash(CalculatePhase2JobHashCall),
        Expiry(ExpiryCall),
        GetProposalState(GetProposalStateCall),
        GetProposalYesVotes(GetProposalYesVotesCall),
        GetProposalYesVotesTotal(GetProposalYesVotesTotalCall),
        Initialize(InitializeCall),
        Initialized(InitializedCall),
        IsValidForwarder(IsValidForwarderCall),
        RefreshVoters(RefreshVotersCall),
        SubmitGovernanceProposal(SubmitGovernanceProposalCall),
        Threshold(ThresholdCall),
        Ttl(TtlCall),
        UseDemocracy(UseDemocracyCall),
        UseValidators(UseValidatorsCall),
        VoteProposal(VoteProposalCall),
        Voters(VotersCall),
    }
    impl ::ethers::core::abi::AbiDecode for SigningRulesContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <MaxVotersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxVoters(decoded));
            }
            if let Ok(decoded) =
                <ProposalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Proposals(decoded));
            }
            if let Ok(decoded) = <AdminSetForwarderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdminSetForwarder(decoded));
            }
            if let Ok(decoded) =
                <AdminsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Admins(decoded));
            }
            if let Ok(decoded) = <CalculatePhase2JobHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculatePhase2JobHash(decoded));
            }
            if let Ok(decoded) =
                <ExpiryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Expiry(decoded));
            }
            if let Ok(decoded) =
                <GetProposalStateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetProposalState(decoded));
            }
            if let Ok(decoded) = <GetProposalYesVotesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProposalYesVotes(decoded));
            }
            if let Ok(decoded) = <GetProposalYesVotesTotalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProposalYesVotesTotal(decoded));
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
                <IsValidForwarderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsValidForwarder(decoded));
            }
            if let Ok(decoded) =
                <RefreshVotersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RefreshVoters(decoded));
            }
            if let Ok(decoded) = <SubmitGovernanceProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitGovernanceProposal(decoded));
            }
            if let Ok(decoded) =
                <ThresholdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Threshold(decoded));
            }
            if let Ok(decoded) =
                <TtlCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Ttl(decoded));
            }
            if let Ok(decoded) =
                <UseDemocracyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UseDemocracy(decoded));
            }
            if let Ok(decoded) =
                <UseValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UseValidators(decoded));
            }
            if let Ok(decoded) =
                <VoteProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::VoteProposal(decoded));
            }
            if let Ok(decoded) =
                <VotersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Voters(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SigningRulesContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxVoters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Proposals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminSetForwarder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admins(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculatePhase2JobHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Expiry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposalState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposalYesVotes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposalYesVotesTotal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidForwarder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RefreshVoters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitGovernanceProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Threshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ttl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UseDemocracy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UseValidators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VoteProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SigningRulesContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::MaxVoters(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Proposals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdminSetForwarder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Admins(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculatePhase2JobHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Expiry(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProposalState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProposalYesVotes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProposalYesVotesTotal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsValidForwarder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RefreshVoters(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitGovernanceProposal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Threshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Ttl(element) => ::core::fmt::Display::fmt(element, f),
                Self::UseDemocracy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UseValidators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoteProposal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voters(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxVotersCall> for SigningRulesContractCalls {
        fn from(value: MaxVotersCall) -> Self {
            Self::MaxVoters(value)
        }
    }
    impl ::core::convert::From<ProposalsCall> for SigningRulesContractCalls {
        fn from(value: ProposalsCall) -> Self {
            Self::Proposals(value)
        }
    }
    impl ::core::convert::From<AdminSetForwarderCall>
        for SigningRulesContractCalls
    {
        fn from(value: AdminSetForwarderCall) -> Self {
            Self::AdminSetForwarder(value)
        }
    }
    impl ::core::convert::From<AdminsCall> for SigningRulesContractCalls {
        fn from(value: AdminsCall) -> Self {
            Self::Admins(value)
        }
    }
    impl ::core::convert::From<CalculatePhase2JobHashCall>
        for SigningRulesContractCalls
    {
        fn from(value: CalculatePhase2JobHashCall) -> Self {
            Self::CalculatePhase2JobHash(value)
        }
    }
    impl ::core::convert::From<ExpiryCall> for SigningRulesContractCalls {
        fn from(value: ExpiryCall) -> Self {
            Self::Expiry(value)
        }
    }
    impl ::core::convert::From<GetProposalStateCall> for SigningRulesContractCalls {
        fn from(value: GetProposalStateCall) -> Self {
            Self::GetProposalState(value)
        }
    }
    impl ::core::convert::From<GetProposalYesVotesCall>
        for SigningRulesContractCalls
    {
        fn from(value: GetProposalYesVotesCall) -> Self {
            Self::GetProposalYesVotes(value)
        }
    }
    impl ::core::convert::From<GetProposalYesVotesTotalCall>
        for SigningRulesContractCalls
    {
        fn from(value: GetProposalYesVotesTotalCall) -> Self {
            Self::GetProposalYesVotesTotal(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for SigningRulesContractCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InitializedCall> for SigningRulesContractCalls {
        fn from(value: InitializedCall) -> Self {
            Self::Initialized(value)
        }
    }
    impl ::core::convert::From<IsValidForwarderCall> for SigningRulesContractCalls {
        fn from(value: IsValidForwarderCall) -> Self {
            Self::IsValidForwarder(value)
        }
    }
    impl ::core::convert::From<RefreshVotersCall> for SigningRulesContractCalls {
        fn from(value: RefreshVotersCall) -> Self {
            Self::RefreshVoters(value)
        }
    }
    impl ::core::convert::From<SubmitGovernanceProposalCall>
        for SigningRulesContractCalls
    {
        fn from(value: SubmitGovernanceProposalCall) -> Self {
            Self::SubmitGovernanceProposal(value)
        }
    }
    impl ::core::convert::From<ThresholdCall> for SigningRulesContractCalls {
        fn from(value: ThresholdCall) -> Self {
            Self::Threshold(value)
        }
    }
    impl ::core::convert::From<TtlCall> for SigningRulesContractCalls {
        fn from(value: TtlCall) -> Self {
            Self::Ttl(value)
        }
    }
    impl ::core::convert::From<UseDemocracyCall> for SigningRulesContractCalls {
        fn from(value: UseDemocracyCall) -> Self {
            Self::UseDemocracy(value)
        }
    }
    impl ::core::convert::From<UseValidatorsCall> for SigningRulesContractCalls {
        fn from(value: UseValidatorsCall) -> Self {
            Self::UseValidators(value)
        }
    }
    impl ::core::convert::From<VoteProposalCall> for SigningRulesContractCalls {
        fn from(value: VoteProposalCall) -> Self {
            Self::VoteProposal(value)
        }
    }
    impl ::core::convert::From<VotersCall> for SigningRulesContractCalls {
        fn from(value: VotersCall) -> Self {
            Self::Voters(value)
        }
    }
    ///Container type for all return fields from the `MAX_VOTERS` function with signature `MAX_VOTERS()` and selector `0x3ceed692`
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
    pub struct MaxVotersReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_proposals` function with signature `_proposals(bytes32)` and selector `0xf2e2af94`
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
    pub struct ProposalsReturn {
        pub status: u8,
        pub yes_votes: ::ethers::core::types::U256,
        pub yes_votes_total: u8,
        pub proposed_block: u64,
    }
    ///Container type for all return fields from the `admins` function with signature `admins(uint64)` and selector `0xf768b74d`
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
    pub struct AdminsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `calculatePhase2JobHash` function with signature `calculatePhase2JobHash(uint64,bytes)` and selector `0xeea22ba2`
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
    pub struct CalculatePhase2JobHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `expiry` function with signature `expiry(uint64)` and selector `0x217d75ab`
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
    pub struct ExpiryReturn(pub u64);
    ///Container type for all return fields from the `getProposalState` function with signature `getProposalState(bytes32)` and selector `0x0a704f48`
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
    pub struct GetProposalStateReturn(pub u8);
    ///Container type for all return fields from the `getProposalYesVotes` function with signature `getProposalYesVotes(bytes32)` and selector `0xa9b62d8d`
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
    pub struct GetProposalYesVotesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getProposalYesVotesTotal` function with signature `getProposalYesVotesTotal(bytes32)` and selector `0x1b35b0cc`
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
    pub struct GetProposalYesVotesTotalReturn(pub u8);
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
    ///Container type for all return fields from the `isValidForwarder` function with signature `isValidForwarder(uint64,address)` and selector `0x3718acf8`
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
    pub struct IsValidForwarderReturn(pub bool);
    ///Container type for all return fields from the `threshold` function with signature `threshold(uint64)` and selector `0x11f8dec6`
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
    pub struct ThresholdReturn(pub u8);
    ///Container type for all return fields from the `ttl` function with signature `ttl(uint64)` and selector `0x6a30ed1c`
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
    pub struct TtlReturn(pub u64);
    ///Container type for all return fields from the `useDemocracy` function with signature `useDemocracy(uint64)` and selector `0x5b9a5eb7`
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
    pub struct UseDemocracyReturn(pub bool);
    ///Container type for all return fields from the `useValidators` function with signature `useValidators(uint64)` and selector `0x07ff4197`
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
    pub struct UseValidatorsReturn(pub bool);
    ///Container type for all return fields from the `voters` function with signature `voters(uint64,uint256)` and selector `0x8a9a2093`
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
    pub struct VotersReturn(pub ::ethers::core::types::Address);
}
