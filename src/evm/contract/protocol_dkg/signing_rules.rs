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
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("MAX_VOTERS"), inputs :
                        ::std::vec![], outputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_proposals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("_proposals"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::borrow::ToOwned::to_owned("_status"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("enum ProposalStatus")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("_yesVotes"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("_yesVotesTotal"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint8")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("_proposedBlock"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint40")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("adminSetForwarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("adminSetForwarder"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("proposalId"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("forwarder"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Address, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("valid"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bool, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool")),
                        }], outputs : ::std::vec![], constant :
                        ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::NonPayable, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("admins"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("admins"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Address, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculatePhase1ProposalId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("calculatePhase1ProposalId"),
                        inputs : ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase1JobId"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase1JobDetails"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bytes, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::Pure, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculatePhase2JobHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("calculatePhase2JobHash"),
                        inputs : ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("proposalId"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase2JobDetails"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bytes, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::Pure, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("expiry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("expiry"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint40")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProposalState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("getProposalState"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase2JobHash"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("enum ProposalStatus")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProposalYesVotes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("getProposalYesVotes"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase2JobHash"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProposalYesVotesTotal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("getProposalYesVotesTotal"),
                        inputs : ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase2JobHash"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint8")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("initialize"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase1JobId"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase1JobDetails"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bytes, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("_threshold"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint8")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("_useDemocracy"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bool, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("_voters"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Address)),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address[]")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("_expiry"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint40")),
                        }], outputs : ::std::vec![], constant :
                        ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::NonPayable, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("initialized"), inputs :
                        ::std::vec![], outputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bool, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isValidForwarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("isValidForwarder"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Address, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bool, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("refreshVoters"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("refreshVoters"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase1ProposalId"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], outputs : ::std::vec![], constant :
                        ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::NonPayable, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitGovernanceProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("submitGovernanceProposal"),
                        inputs : ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase1JobId"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase1JobDetails"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bytes, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase2JobDetails"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bytes, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes")),
                        }], outputs : ::std::vec![], constant :
                        ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::NonPayable, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("threshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("threshold"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint8")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("useDemocracy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("useDemocracy"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bool, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("useValidators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("useValidators"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bool, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("voteProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("voteProposal"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase1JobId"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase1JobDetails"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bytes, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::borrow::ToOwned::to_owned("phase2JobDetails"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bytes, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes")),
                        }], outputs : ::std::vec![], constant :
                        ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::NonPayable, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("voters"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function { name :
                        ::std::borrow::ToOwned::to_owned("voters"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32")),
                        }, ::ethers::core::abi::ethabi::Param { name :
                        ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256")),
                        }], outputs : ::std::vec![::ethers::core::abi::ethabi::Param {
                        name : ::std::string::String::new(), kind :
                        ::ethers::core::abi::ethabi::ParamType::Address, internal_type :
                        ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address")),
                        }], constant : ::core::option::Option::None, state_mutability :
                        ::ethers::core::abi::ethabi::StateMutability::View, }
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FailedHandlerExecution"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event { name :
                        ::std::borrow::ToOwned::to_owned("FailedHandlerExecution"),
                        inputs : ::std::vec![::ethers::core::abi::ethabi::EventParam {
                        name : ::std::borrow::ToOwned::to_owned("lowLevelData"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Bytes, indexed : false,
                        }], anonymous : false, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposalEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event { name :
                        ::std::borrow::ToOwned::to_owned("ProposalEvent"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::EventParam { name :
                        ::std::borrow::ToOwned::to_owned("status"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize), indexed :
                        false, }, ::ethers::core::abi::ethabi::EventParam { name :
                        ::std::borrow::ToOwned::to_owned("proposalId"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        indexed : false, }, ::ethers::core::abi::ethabi::EventParam {
                        name : ::std::borrow::ToOwned::to_owned("phase2JobHash"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        indexed : false, }], anonymous : false, }
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposalVote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event { name :
                        ::std::borrow::ToOwned::to_owned("ProposalVote"), inputs :
                        ::std::vec![::ethers::core::abi::ethabi::EventParam { name :
                        ::std::borrow::ToOwned::to_owned("status"), kind :
                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize), indexed :
                        false, }, ::ethers::core::abi::ethabi::EventParam { name :
                        ::std::borrow::ToOwned::to_owned("proposalId"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        indexed : false, }, ::ethers::core::abi::ethabi::EventParam {
                        name : ::std::borrow::ToOwned::to_owned("phase2JobHash"), kind :
                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        indexed : false, }], anonymous : false, }
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
        ///Calls the contract's `adminSetForwarder` (0xa72cd030) function
        pub fn admin_set_forwarder(
            &self,
            proposal_id: [u8; 32],
            forwarder: ::ethers::core::types::Address,
            valid: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [167, 44, 208, 48],
                    (proposal_id, forwarder, valid),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admins` (0xd079661b) function
        pub fn admins(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([208, 121, 102, 27], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculatePhase1ProposalId` (0xd020a66f) function
        pub fn calculate_phase_1_proposal_id(
            &self,
            phase_1_job_id: [u8; 32],
            phase_1_job_details: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [208, 32, 166, 111],
                    (phase_1_job_id, phase_1_job_details),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculatePhase2JobHash` (0xa0a0c516) function
        pub fn calculate_phase_2_job_hash(
            &self,
            proposal_id: [u8; 32],
            phase_2_job_details: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [160, 160, 197, 22],
                    (proposal_id, phase_2_job_details),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expiry` (0xd2ec5fca) function
        pub fn expiry(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([210, 236, 95, 202], p0)
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
        ///Calls the contract's `initialize` (0xdf1a3f3a) function
        pub fn initialize(
            &self,
            phase_1_job_id: [u8; 32],
            phase_1_job_details: ::ethers::core::types::Bytes,
            threshold: u8,
            use_democracy: bool,
            voters: ::std::vec::Vec<::ethers::core::types::Address>,
            expiry: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [223, 26, 63, 58],
                    (
                        phase_1_job_id,
                        phase_1_job_details,
                        threshold,
                        use_democracy,
                        voters,
                        expiry,
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
        ///Calls the contract's `isValidForwarder` (0x22b87b97) function
        pub fn is_valid_forwarder(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([34, 184, 123, 151], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refreshVoters` (0x1cba5b04) function
        pub fn refresh_voters(
            &self,
            phase_1_proposal_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 186, 91, 4], phase_1_proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitGovernanceProposal` (0x6f85e71a) function
        pub fn submit_governance_proposal(
            &self,
            phase_1_job_id: [u8; 32],
            phase_1_job_details: ::ethers::core::types::Bytes,
            phase_2_job_details: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [111, 133, 231, 26],
                    (phase_1_job_id, phase_1_job_details, phase_2_job_details),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `threshold` (0x25605b6d) function
        pub fn threshold(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([37, 96, 91, 109], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `useDemocracy` (0xcf108ab9) function
        pub fn use_democracy(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([207, 16, 138, 185], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `useValidators` (0xbc20f44e) function
        pub fn use_validators(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([188, 32, 244, 78], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voteProposal` (0x2d1ef7f6) function
        pub fn vote_proposal(
            &self,
            phase_1_job_id: [u8; 32],
            phase_1_job_details: ::ethers::core::types::Bytes,
            phase_2_job_details: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [45, 30, 247, 246],
                    (phase_1_job_id, phase_1_job_details, phase_2_job_details),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voters` (0xef67b8ed) function
        pub fn voters(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([239, 103, 184, 237], (p0, p1))
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
        abi = "ProposalEvent(uint8,bytes32,bytes32)"
    )]
    pub struct ProposalEventFilter {
        pub status: u8,
        pub proposal_id: [u8; 32],
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
        abi = "ProposalVote(uint8,bytes32,bytes32)"
    )]
    pub struct ProposalVoteFilter {
        pub status: u8,
        pub proposal_id: [u8; 32],
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
    ///Container type for all input parameters for the `adminSetForwarder` function with signature `adminSetForwarder(bytes32,address,bool)` and selector `0xa72cd030`
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
        abi = "adminSetForwarder(bytes32,address,bool)"
    )]
    pub struct AdminSetForwarderCall {
        pub proposal_id: [u8; 32],
        pub forwarder: ::ethers::core::types::Address,
        pub valid: bool,
    }
    ///Container type for all input parameters for the `admins` function with signature `admins(bytes32)` and selector `0xd079661b`
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
    #[ethcall(name = "admins", abi = "admins(bytes32)")]
    pub struct AdminsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `calculatePhase1ProposalId` function with signature `calculatePhase1ProposalId(bytes32,bytes)` and selector `0xd020a66f`
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
        name = "calculatePhase1ProposalId",
        abi = "calculatePhase1ProposalId(bytes32,bytes)"
    )]
    pub struct CalculatePhase1ProposalIdCall {
        pub phase_1_job_id: [u8; 32],
        pub phase_1_job_details: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `calculatePhase2JobHash` function with signature `calculatePhase2JobHash(bytes32,bytes)` and selector `0xa0a0c516`
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
        abi = "calculatePhase2JobHash(bytes32,bytes)"
    )]
    pub struct CalculatePhase2JobHashCall {
        pub proposal_id: [u8; 32],
        pub phase_2_job_details: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `expiry` function with signature `expiry(bytes32)` and selector `0xd2ec5fca`
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
    #[ethcall(name = "expiry", abi = "expiry(bytes32)")]
    pub struct ExpiryCall(pub [u8; 32]);
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(bytes32,bytes,uint8,bool,address[],uint40)` and selector `0xdf1a3f3a`
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
        abi = "initialize(bytes32,bytes,uint8,bool,address[],uint40)"
    )]
    pub struct InitializeCall {
        pub phase_1_job_id: [u8; 32],
        pub phase_1_job_details: ::ethers::core::types::Bytes,
        pub threshold: u8,
        pub use_democracy: bool,
        pub voters: ::std::vec::Vec<::ethers::core::types::Address>,
        pub expiry: u64,
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
    ///Container type for all input parameters for the `isValidForwarder` function with signature `isValidForwarder(bytes32,address)` and selector `0x22b87b97`
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
        abi = "isValidForwarder(bytes32,address)"
    )]
    pub struct IsValidForwarderCall(
        pub [u8; 32],
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `refreshVoters` function with signature `refreshVoters(bytes32)` and selector `0x1cba5b04`
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
    #[ethcall(name = "refreshVoters", abi = "refreshVoters(bytes32)")]
    pub struct RefreshVotersCall {
        pub phase_1_proposal_id: [u8; 32],
    }
    ///Container type for all input parameters for the `submitGovernanceProposal` function with signature `submitGovernanceProposal(bytes32,bytes,bytes)` and selector `0x6f85e71a`
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
        abi = "submitGovernanceProposal(bytes32,bytes,bytes)"
    )]
    pub struct SubmitGovernanceProposalCall {
        pub phase_1_job_id: [u8; 32],
        pub phase_1_job_details: ::ethers::core::types::Bytes,
        pub phase_2_job_details: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `threshold` function with signature `threshold(bytes32)` and selector `0x25605b6d`
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
    #[ethcall(name = "threshold", abi = "threshold(bytes32)")]
    pub struct ThresholdCall(pub [u8; 32]);
    ///Container type for all input parameters for the `useDemocracy` function with signature `useDemocracy(bytes32)` and selector `0xcf108ab9`
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
    #[ethcall(name = "useDemocracy", abi = "useDemocracy(bytes32)")]
    pub struct UseDemocracyCall(pub [u8; 32]);
    ///Container type for all input parameters for the `useValidators` function with signature `useValidators(bytes32)` and selector `0xbc20f44e`
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
    #[ethcall(name = "useValidators", abi = "useValidators(bytes32)")]
    pub struct UseValidatorsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `voteProposal` function with signature `voteProposal(bytes32,bytes,bytes)` and selector `0x2d1ef7f6`
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
    #[ethcall(name = "voteProposal", abi = "voteProposal(bytes32,bytes,bytes)")]
    pub struct VoteProposalCall {
        pub phase_1_job_id: [u8; 32],
        pub phase_1_job_details: ::ethers::core::types::Bytes,
        pub phase_2_job_details: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `voters` function with signature `voters(bytes32,uint256)` and selector `0xef67b8ed`
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
    #[ethcall(name = "voters", abi = "voters(bytes32,uint256)")]
    pub struct VotersCall(pub [u8; 32], pub ::ethers::core::types::U256);
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
        CalculatePhase1ProposalId(CalculatePhase1ProposalIdCall),
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
            if let Ok(decoded)
                = <AdminSetForwarderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AdminSetForwarder(decoded));
            }
            if let Ok(decoded) =
                <AdminsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Admins(decoded));
            }
            if let Ok(decoded)
                = <CalculatePhase1ProposalIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculatePhase1ProposalId(decoded));
            }
            if let Ok(decoded)
                = <CalculatePhase2JobHashCall as ::ethers::core::abi::AbiDecode>::decode(
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
            if let Ok(decoded)
                = <GetProposalYesVotesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetProposalYesVotes(decoded));
            }
            if let Ok(decoded)
                = <GetProposalYesVotesTotalCall as ::ethers::core::abi::AbiDecode>::decode(
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
            if let Ok(decoded)
                = <SubmitGovernanceProposalCall as ::ethers::core::abi::AbiDecode>::decode(
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
                Self::CalculatePhase1ProposalId(element) => {
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
                Self::CalculatePhase1ProposalId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<CalculatePhase1ProposalIdCall>
        for SigningRulesContractCalls
    {
        fn from(value: CalculatePhase1ProposalIdCall) -> Self {
            Self::CalculatePhase1ProposalId(value)
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
    ///Container type for all return fields from the `admins` function with signature `admins(bytes32)` and selector `0xd079661b`
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
    ///Container type for all return fields from the `calculatePhase1ProposalId` function with signature `calculatePhase1ProposalId(bytes32,bytes)` and selector `0xd020a66f`
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
    pub struct CalculatePhase1ProposalIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculatePhase2JobHash` function with signature `calculatePhase2JobHash(bytes32,bytes)` and selector `0xa0a0c516`
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
    ///Container type for all return fields from the `expiry` function with signature `expiry(bytes32)` and selector `0xd2ec5fca`
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
    ///Container type for all return fields from the `isValidForwarder` function with signature `isValidForwarder(bytes32,address)` and selector `0x22b87b97`
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
    ///Container type for all return fields from the `threshold` function with signature `threshold(bytes32)` and selector `0x25605b6d`
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
    ///Container type for all return fields from the `useDemocracy` function with signature `useDemocracy(bytes32)` and selector `0xcf108ab9`
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
    ///Container type for all return fields from the `useValidators` function with signature `useValidators(bytes32)` and selector `0xbc20f44e`
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
    ///Container type for all return fields from the `voters` function with signature `voters(bytes32,uint256)` and selector `0xef67b8ed`
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
