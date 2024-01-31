pub use v_anchor_encode_inputs_contract::*;
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
pub mod v_anchor_encode_inputs_contract {
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
                    ::std::borrow::ToOwned::to_owned("_encodeInputs16"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_encodeInputs16"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_args"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxEdges"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("_encodeInputs2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_encodeInputs2"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_args"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxEdges"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static VANCHORENCODEINPUTSCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x16ka\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0aW`\x005`\xE0\x1C\x80c4\x08\xE4p\x14a\0fW\x80cL\x83\x0C\xBD\x14a\0yW\x80c}l\\\xEB\x14a\0\x98W\x80c\x8B~\x87\x82\x14a\0\xB9W\x80c\xAB\x14\x9F\xD8\x14a\0\xDDW[`\0\x80\xFD[`@QF\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x81a\0\xF0V[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0pV[a\0\xABa\0\xA66`\x04a\x12vV[a\x01>V[`@Qa\0p\x92\x91\x90a\x13\x88V[a\0\xC4`\x01`\xF8\x1B\x81V[`@Q`\x01`\x01`\xF0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\0pV[a\0\xABa\0\xEB6`\x04a\x12vV[a\x0B\xF5V[`@\x80Q`\x01`\xF8\x1B` \x82\x01\x81\x90RF`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\"\x84\x01R\x83Q\x80\x84\x03`\x06\x01\x81R`&\x90\x93\x01\x90\x93R`\0\x92\x91a\x013\x81a\x14\x1EV[`\xD0\x1C\x93PPPP\x90V[``\x80`\0a\x01Ka\0\xF0V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0a\x01b\x85`\x01a\x14UV[`\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01}Wa\x01}a\x10\x8DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xA6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P``\x85`\xFF\x16`\x01\x03a\x06\x18Wa\x01\xBEa\x10\x11V[`\0\x89`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\x01\xD8\x91\x90a\x14\x82V[\x90P\x80`\0` \x02\x01Q\x84`\0\x81Q\x81\x10a\x01\xF5Wa\x01\xF5a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x01` \x02\x01Q\x84`\x01\x81Q\x81\x10a\x02\x1BWa\x02\x1Ba\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`\x80\x8B\x01Q\x83R`\xA0\x8B\x01Q\x90\x83\x01R`@\x8A\x01Q\x80Q`\0\x90a\x02PWa\x02Pa\x14\xD7V[` \x02` \x01\x01Q\x82`\x02`\x17\x81\x10a\x02kWa\x02ka\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x01\x90\x81\x10a\x02\x88Wa\x02\x88a\x14\xD7V[` \x02` \x01\x01Q\x82`\x03`\x17\x81\x10a\x02\xA3Wa\x02\xA3a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x02\x90\x81\x10a\x02\xC0Wa\x02\xC0a\x14\xD7V[` \x02` \x01\x01Q\x82`\x04`\x17\x81\x10a\x02\xDBWa\x02\xDBa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x03\x90\x81\x10a\x02\xF8Wa\x02\xF8a\x14\xD7V[` \x02` \x01\x01Q\x82`\x05`\x17\x81\x10a\x03\x13Wa\x03\x13a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x04\x90\x81\x10a\x030Wa\x030a\x14\xD7V[` \x02` \x01\x01Q\x82`\x06`\x17\x81\x10a\x03KWa\x03Ka\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x05\x90\x81\x10a\x03hWa\x03ha\x14\xD7V[` \x02` \x01\x01Q\x82`\x07`\x17\x81\x10a\x03\x83Wa\x03\x83a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x06\x90\x81\x10a\x03\xA0Wa\x03\xA0a\x14\xD7V[` \x02` \x01\x01Q\x82`\x08`\x17\x81\x10a\x03\xBBWa\x03\xBBa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x07\x90\x81\x10a\x03\xD8Wa\x03\xD8a\x14\xD7V[` \x02` \x01\x01Q\x82`\t`\x17\x81\x10a\x03\xF3Wa\x03\xF3a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x08\x90\x81\x10a\x04\x10Wa\x04\x10a\x14\xD7V[` \x02` \x01\x01Q\x82`\n`\x17\x81\x10a\x04+Wa\x04+a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\t\x90\x81\x10a\x04HWa\x04Ha\x14\xD7V[` \x02` \x01\x01Q\x82`\x0B`\x17\x81\x10a\x04cWa\x04ca\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\n\x90\x81\x10a\x04\x80Wa\x04\x80a\x14\xD7V[` \x02` \x01\x01Q\x82`\x0C`\x17\x81\x10a\x04\x9BWa\x04\x9Ba\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0B\x90\x81\x10a\x04\xB8Wa\x04\xB8a\x14\xD7V[` \x02` \x01\x01Q\x82`\r`\x17\x81\x10a\x04\xD3Wa\x04\xD3a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0C\x90\x81\x10a\x04\xF0Wa\x04\xF0a\x14\xD7V[` \x02` \x01\x01Q\x82`\x0E`\x17\x81\x10a\x05\x0BWa\x05\x0Ba\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\r\x90\x81\x10a\x05(Wa\x05(a\x14\xD7V[` \x02` \x01\x01Q\x82`\x0F`\x17\x81\x10a\x05CWa\x05Ca\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0E\x90\x81\x10a\x05`Wa\x05`a\x14\xD7V[` \x02` \x01\x01Q\x82`\x10`\x17\x81\x10a\x05{Wa\x05{a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0F\x90\x81\x10a\x05\x98Wa\x05\x98a\x14\xD7V[` \x02` \x01\x01Q\x82`\x11`\x17\x81\x10a\x05\xB3Wa\x05\xB3a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x91\x90\x91R``\x8B\x01\x80QQa\x02@\x85\x01RQ\x81\x01Qa\x02`\x84\x01Ra\x02\x80\x83\x01\x86\x90R\x81Qa\x02\xA0\x84\x01R\x81\x81\x01Qa\x02\xC0\x84\x01R`@Qa\x06\0\x91\x84\x91\x01a\x14\xEDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPPa\x0B\xE9V[\x85`\xFF\x16`\x07\x03a\x0B\x92Wa\x06+a\x100V[`\0\x89`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\x06E\x91\x90a\x15\"V[\x90P\x80`\0` \x02\x01Q\x84`\0\x81Q\x81\x10a\x06bWa\x06ba\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x01` \x02\x01Q\x84`\x01\x81Q\x81\x10a\x06\x88Wa\x06\x88a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x02` \x02\x01Q\x84`\x02\x81Q\x81\x10a\x06\xAEWa\x06\xAEa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x03` \x02\x01Q\x84`\x03\x81Q\x81\x10a\x06\xD4Wa\x06\xD4a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x04` \x02\x01Q\x84`\x04\x81Q\x81\x10a\x06\xFAWa\x06\xFAa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x05` \x02\x01Q\x84`\x05\x81Q\x81\x10a\x07 Wa\x07 a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x06` \x02\x01Q\x84`\x06\x81Q\x81\x10a\x07FWa\x07Fa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x07` \x02\x01Q\x84`\x07\x81Q\x81\x10a\x07lWa\x07la\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`\x80\x8B\x01Q\x83R`\xA0\x8B\x01Q\x90\x83\x01R`@\x8A\x01Q\x80Q`\0\x90a\x07\xA1Wa\x07\xA1a\x14\xD7V[` \x02` \x01\x01Q\x82`\x02`\x1D\x81\x10a\x07\xBCWa\x07\xBCa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x01\x90\x81\x10a\x07\xD9Wa\x07\xD9a\x14\xD7V[` \x02` \x01\x01Q\x82`\x03`\x1D\x81\x10a\x07\xF4Wa\x07\xF4a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x02\x90\x81\x10a\x08\x11Wa\x08\x11a\x14\xD7V[` \x02` \x01\x01Q\x82`\x04`\x1D\x81\x10a\x08,Wa\x08,a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x03\x90\x81\x10a\x08IWa\x08Ia\x14\xD7V[` \x02` \x01\x01Q\x82`\x05`\x1D\x81\x10a\x08dWa\x08da\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x04\x90\x81\x10a\x08\x81Wa\x08\x81a\x14\xD7V[` \x02` \x01\x01Q\x82`\x06`\x1D\x81\x10a\x08\x9CWa\x08\x9Ca\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x05\x90\x81\x10a\x08\xB9Wa\x08\xB9a\x14\xD7V[` \x02` \x01\x01Q\x82`\x07`\x1D\x81\x10a\x08\xD4Wa\x08\xD4a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x06\x90\x81\x10a\x08\xF1Wa\x08\xF1a\x14\xD7V[` \x02` \x01\x01Q\x82`\x08`\x1D\x81\x10a\t\x0CWa\t\x0Ca\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x07\x90\x81\x10a\t)Wa\t)a\x14\xD7V[` \x02` \x01\x01Q\x82`\t`\x1D\x81\x10a\tDWa\tDa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x08\x90\x81\x10a\taWa\taa\x14\xD7V[` \x02` \x01\x01Q\x82`\n`\x1D\x81\x10a\t|Wa\t|a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\t\x90\x81\x10a\t\x99Wa\t\x99a\x14\xD7V[` \x02` \x01\x01Q\x82`\x0B`\x1D\x81\x10a\t\xB4Wa\t\xB4a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\n\x90\x81\x10a\t\xD1Wa\t\xD1a\x14\xD7V[` \x02` \x01\x01Q\x82`\x0C`\x1D\x81\x10a\t\xECWa\t\xECa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0B\x90\x81\x10a\n\tWa\n\ta\x14\xD7V[` \x02` \x01\x01Q\x82`\r`\x1D\x81\x10a\n$Wa\n$a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0C\x90\x81\x10a\nAWa\nAa\x14\xD7V[` \x02` \x01\x01Q\x82`\x0E`\x1D\x81\x10a\n\\Wa\n\\a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\r\x90\x81\x10a\nyWa\nya\x14\xD7V[` \x02` \x01\x01Q\x82`\x0F`\x1D\x81\x10a\n\x94Wa\n\x94a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0E\x90\x81\x10a\n\xB1Wa\n\xB1a\x14\xD7V[` \x02` \x01\x01Q\x82`\x10`\x1D\x81\x10a\n\xCCWa\n\xCCa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0F\x90\x81\x10a\n\xE9Wa\n\xE9a\x14\xD7V[` \x02` \x01\x01Q\x82`\x11`\x1D\x81\x10a\x0B\x04Wa\x0B\x04a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x91\x90\x91R``\x8B\x81\x01\x80QQa\x02@\x86\x01RQ\x82\x01Qa\x02`\x85\x01Ra\x02\x80\x84\x01\x87\x90R\x82Qa\x02\xA0\x85\x01R\x82\x82\x01Qa\x02\xC0\x85\x01R`@\x80\x84\x01Qa\x02\xE0\x86\x01R\x90\x83\x01Qa\x03\0\x85\x01R`\x80\x83\x01Qa\x03 \x85\x01R`\xA0\x83\x01Qa\x03@\x85\x01R`\xC0\x83\x01Qa\x03`\x85\x01R`\xE0\x83\x01Qa\x03\x80\x85\x01RQa\x06\0\x91\x84\x91\x01a\x15\x96V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FVAnchorEncodeInputs: Invalid edg`D\x82\x01Raes`\xF0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x97\x90\x96P\x94PPPPPV[``\x80`\0a\x0C\x02a\0\xF0V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0a\x0C\x19\x85`\x01a\x14UV[`\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C4Wa\x0C4a\x10\x8DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C]W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P``\x85`\xFF\x16`\x01\x03a\r\xA3Wa\x0Cua\x10OV[`\0\x89`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\x0C\x8F\x91\x90a\x14\x82V[\x90P\x80`\0` \x02\x01Q\x84`\0\x81Q\x81\x10a\x0C\xACWa\x0C\xACa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x01` \x02\x01Q\x84`\x01\x81Q\x81\x10a\x0C\xD2Wa\x0C\xD2a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`\x80\x8B\x01Q\x83R`\xA0\x8B\x01Q\x90\x83\x01R`@\x8A\x01Q\x80Q`\0\x90a\r\x07Wa\r\x07a\x14\xD7V[` \x02` \x01\x01Q\x82`\x02`\t\x81\x10a\r\"Wa\r\"a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x01\x90\x81\x10a\r?Wa\r?a\x14\xD7V[` \x02` \x01\x01Q\x82`\x03`\t\x81\x10a\rZWa\rZa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x91\x90\x91R``\x8B\x01\x80QQ`\x80\x85\x01RQ\x81\x01Q`\xA0\x84\x01R`\xC0\x83\x01\x86\x90R\x81Q`\xE0\x84\x01R\x81\x81\x01Qa\x01\0\x84\x01R`@Qa\x06\0\x91\x84\x91\x01a\x15\xCBV[\x85`\xFF\x16`\x07\x03a\x0B\x92Wa\r\xB6a\x10nV[`\0\x89`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\r\xD0\x91\x90a\x15\"V[\x90P\x80`\0` \x02\x01Q\x84`\0\x81Q\x81\x10a\r\xEDWa\r\xEDa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x01` \x02\x01Q\x84`\x01\x81Q\x81\x10a\x0E\x13Wa\x0E\x13a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x02` \x02\x01Q\x84`\x02\x81Q\x81\x10a\x0E9Wa\x0E9a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x03` \x02\x01Q\x84`\x03\x81Q\x81\x10a\x0E_Wa\x0E_a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x04` \x02\x01Q\x84`\x04\x81Q\x81\x10a\x0E\x85Wa\x0E\x85a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x05` \x02\x01Q\x84`\x05\x81Q\x81\x10a\x0E\xABWa\x0E\xABa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x06` \x02\x01Q\x84`\x06\x81Q\x81\x10a\x0E\xD1Wa\x0E\xD1a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x07` \x02\x01Q\x84`\x07\x81Q\x81\x10a\x0E\xF7Wa\x0E\xF7a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`\x80\x8B\x01Q\x83R`\xA0\x8B\x01Q\x90\x83\x01R`@\x8A\x01Q\x80Q`\0\x90a\x0F,Wa\x0F,a\x14\xD7V[` \x02` \x01\x01Q\x82`\x02`\x0F\x81\x10a\x0FGWa\x0FGa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x01\x90\x81\x10a\x0FdWa\x0Fda\x14\xD7V[` \x02` \x01\x01Q\x82`\x03`\x0F\x81\x10a\x0F\x7FWa\x0F\x7Fa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x91\x90\x91R``\x8B\x81\x01\x80QQ`\x80\x80\x87\x01\x91\x90\x91R\x90Q\x83\x01Q`\xA0\x80\x87\x01\x91\x90\x91R`\xC0\x80\x87\x01\x8A\x90R\x85Q`\xE0\x80\x89\x01\x91\x90\x91R\x86\x86\x01Qa\x01\0\x89\x01R`@\x80\x88\x01Qa\x01 \x8A\x01R\x94\x87\x01Qa\x01@\x89\x01R\x92\x86\x01Qa\x01`\x88\x01R\x90\x85\x01Qa\x01\x80\x87\x01R\x84\x01Qa\x01\xA0\x86\x01R\x83\x01Qa\x01\xC0\x85\x01RQa\x06\0\x91\x84\x91\x01a\x16\0V[`@Q\x80a\x02\xE0\x01`@R\x80`\x17\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x03\xA0\x01`@R\x80`\x1D\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01 \x01`@R\x80`\t\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01\xE0\x01`@R\x80`\x0F\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xC6Wa\x10\xC6a\x10\x8DV[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xC6Wa\x10\xC6a\x10\x8DV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11\x18Wa\x11\x18a\x10\x8DV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x111W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11KWa\x11Ka\x10\x8DV[a\x11^`\x1F\x82\x01`\x1F\x19\x16` \x01a\x10\xEFV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x11sW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x11\xA1W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\xBDWa\x11\xBDa\x10\x8DV[\x81`\x05\x1Ba\x11\xCC\x82\x82\x01a\x10\xEFV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x11\xE6W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x12\x05W\x825\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x11\xECV[\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x12!W`\0\x80\xFD[a\x12)a\x10\xCCV[\x80`@\x84\x01\x85\x81\x11\x15a\x12;W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x12UW\x805\x84R` \x93\x84\x01\x93\x01a\x12=V[P\x90\x95\x94PPPPPV[\x805`\xFF\x81\x16\x81\x14a\x12qW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\x8BW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\xA3W`\0\x80\xFD[\x90\x85\x01\x90`\xE0\x82\x88\x03\x12\x15a\x12\xB7W`\0\x80\xFD[a\x12\xBFa\x10\xA3V[\x825\x82\x81\x11\x15a\x12\xCEW`\0\x80\xFD[a\x12\xDA\x89\x82\x86\x01a\x11 V[\x82RP` \x83\x015\x82\x81\x11\x15a\x12\xEFW`\0\x80\xFD[a\x12\xFB\x89\x82\x86\x01a\x11 V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x13\x13W`\0\x80\xFD[a\x13\x1F\x89\x82\x86\x01a\x11\x90V[`@\x83\x01RPa\x132\x88``\x85\x01a\x12\x10V[``\x82\x01R`\xA0\x83\x015`\x80\x82\x01R`\xC0\x83\x015`\xA0\x82\x01R\x80\x95PP` \x86\x015\x91P\x80\x82\x11\x15a\x13cW`\0\x80\xFD[Pa\x13p\x86\x82\x87\x01a\x11 V[\x92PPa\x13\x7F`@\x85\x01a\x12`V[\x90P\x92P\x92P\x92V[`@\x81R`\0\x83Q\x80`@\x84\x01R`\0[\x81\x81\x10\x15a\x13\xB6W` \x81\x87\x01\x81\x01Q``\x86\x84\x01\x01R\x01a\x13\x99V[P`\0``\x82\x85\x01\x01R`\x1F\x19`\x1F\x82\x01\x16\x83\x01\x90P``\x81\x01` ``\x85\x84\x03\x01\x81\x86\x01R\x81\x86Q\x80\x84R`\x80\x85\x01\x91P\x82\x88\x01\x94P`\0\x93P[\x80\x84\x10\x15a\x14\x12W\x84Q\x82R\x93\x82\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a\x13\xF2V[P\x97\x96PPPPPPPV[\x80Q` \x82\x01Q`\x01`\x01`\xD0\x1B\x03\x19\x80\x82\x16\x92\x91\x90`\x06\x83\x10\x15a\x14MW\x80\x81\x84`\x06\x03`\x03\x1B\x1B\x83\x16\x16\x93P[PPP\x91\x90PV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x14|WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x14\x94W`\0\x80\xFD[\x82`\x1F\x83\x01\x12a\x14\xA3W`\0\x80\xFD[a\x14\xABa\x10\xCCV[\x80`@\x84\x01\x85\x81\x11\x15a\x14\xBDW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x12UW\x80Q\x84R` \x93\x84\x01\x93\x01a\x14\xBFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81\x83\x82[`\x17\x81\x10\x15a\x15\x12W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x14\xF3V[PPPa\x02\xE0\x82\x01\x90P\x92\x91PPV[`\0a\x01\0\x80\x83\x85\x03\x12\x15a\x156W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x15EW`\0\x80\xFD[`@Q\x81\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x15gWa\x15ga\x10\x8DV[`@R\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x15|W`\0\x80\xFD[\x84[\x83\x81\x10\x15a\x12UW\x80Q\x82R` \x91\x82\x01\x91\x01a\x15~V[`\0\x81\x83\x82[`\x1D\x81\x10\x15a\x15\xBBW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x15\x9CV[PPPa\x03\xA0\x82\x01\x90P\x92\x91PPV[`\0\x81\x83\x82[`\t\x81\x10\x15a\x15\xF0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x15\xD1V[PPPa\x01 \x82\x01\x90P\x92\x91PPV[`\0\x81\x83\x82[`\x0F\x81\x10\x15a\x16%W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x16\x06V[PPPa\x01\xE0\x82\x01\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xC4$\x04\x01=\x15_\x86\xAF\xB8:\xC12hH<\xA33\x83\xFC\x01\x85a\xB9\x9E\xDC\xDA1\xE0>N\x01dsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static VANCHORENCODEINPUTSCONTRACT_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0aW`\x005`\xE0\x1C\x80c4\x08\xE4p\x14a\0fW\x80cL\x83\x0C\xBD\x14a\0yW\x80c}l\\\xEB\x14a\0\x98W\x80c\x8B~\x87\x82\x14a\0\xB9W\x80c\xAB\x14\x9F\xD8\x14a\0\xDDW[`\0\x80\xFD[`@QF\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x81a\0\xF0V[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0pV[a\0\xABa\0\xA66`\x04a\x12vV[a\x01>V[`@Qa\0p\x92\x91\x90a\x13\x88V[a\0\xC4`\x01`\xF8\x1B\x81V[`@Q`\x01`\x01`\xF0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\0pV[a\0\xABa\0\xEB6`\x04a\x12vV[a\x0B\xF5V[`@\x80Q`\x01`\xF8\x1B` \x82\x01\x81\x90RF`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\"\x84\x01R\x83Q\x80\x84\x03`\x06\x01\x81R`&\x90\x93\x01\x90\x93R`\0\x92\x91a\x013\x81a\x14\x1EV[`\xD0\x1C\x93PPPP\x90V[``\x80`\0a\x01Ka\0\xF0V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0a\x01b\x85`\x01a\x14UV[`\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01}Wa\x01}a\x10\x8DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xA6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P``\x85`\xFF\x16`\x01\x03a\x06\x18Wa\x01\xBEa\x10\x11V[`\0\x89`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\x01\xD8\x91\x90a\x14\x82V[\x90P\x80`\0` \x02\x01Q\x84`\0\x81Q\x81\x10a\x01\xF5Wa\x01\xF5a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x01` \x02\x01Q\x84`\x01\x81Q\x81\x10a\x02\x1BWa\x02\x1Ba\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`\x80\x8B\x01Q\x83R`\xA0\x8B\x01Q\x90\x83\x01R`@\x8A\x01Q\x80Q`\0\x90a\x02PWa\x02Pa\x14\xD7V[` \x02` \x01\x01Q\x82`\x02`\x17\x81\x10a\x02kWa\x02ka\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x01\x90\x81\x10a\x02\x88Wa\x02\x88a\x14\xD7V[` \x02` \x01\x01Q\x82`\x03`\x17\x81\x10a\x02\xA3Wa\x02\xA3a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x02\x90\x81\x10a\x02\xC0Wa\x02\xC0a\x14\xD7V[` \x02` \x01\x01Q\x82`\x04`\x17\x81\x10a\x02\xDBWa\x02\xDBa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x03\x90\x81\x10a\x02\xF8Wa\x02\xF8a\x14\xD7V[` \x02` \x01\x01Q\x82`\x05`\x17\x81\x10a\x03\x13Wa\x03\x13a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x04\x90\x81\x10a\x030Wa\x030a\x14\xD7V[` \x02` \x01\x01Q\x82`\x06`\x17\x81\x10a\x03KWa\x03Ka\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x05\x90\x81\x10a\x03hWa\x03ha\x14\xD7V[` \x02` \x01\x01Q\x82`\x07`\x17\x81\x10a\x03\x83Wa\x03\x83a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x06\x90\x81\x10a\x03\xA0Wa\x03\xA0a\x14\xD7V[` \x02` \x01\x01Q\x82`\x08`\x17\x81\x10a\x03\xBBWa\x03\xBBa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x07\x90\x81\x10a\x03\xD8Wa\x03\xD8a\x14\xD7V[` \x02` \x01\x01Q\x82`\t`\x17\x81\x10a\x03\xF3Wa\x03\xF3a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x08\x90\x81\x10a\x04\x10Wa\x04\x10a\x14\xD7V[` \x02` \x01\x01Q\x82`\n`\x17\x81\x10a\x04+Wa\x04+a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\t\x90\x81\x10a\x04HWa\x04Ha\x14\xD7V[` \x02` \x01\x01Q\x82`\x0B`\x17\x81\x10a\x04cWa\x04ca\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\n\x90\x81\x10a\x04\x80Wa\x04\x80a\x14\xD7V[` \x02` \x01\x01Q\x82`\x0C`\x17\x81\x10a\x04\x9BWa\x04\x9Ba\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0B\x90\x81\x10a\x04\xB8Wa\x04\xB8a\x14\xD7V[` \x02` \x01\x01Q\x82`\r`\x17\x81\x10a\x04\xD3Wa\x04\xD3a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0C\x90\x81\x10a\x04\xF0Wa\x04\xF0a\x14\xD7V[` \x02` \x01\x01Q\x82`\x0E`\x17\x81\x10a\x05\x0BWa\x05\x0Ba\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\r\x90\x81\x10a\x05(Wa\x05(a\x14\xD7V[` \x02` \x01\x01Q\x82`\x0F`\x17\x81\x10a\x05CWa\x05Ca\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0E\x90\x81\x10a\x05`Wa\x05`a\x14\xD7V[` \x02` \x01\x01Q\x82`\x10`\x17\x81\x10a\x05{Wa\x05{a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0F\x90\x81\x10a\x05\x98Wa\x05\x98a\x14\xD7V[` \x02` \x01\x01Q\x82`\x11`\x17\x81\x10a\x05\xB3Wa\x05\xB3a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x91\x90\x91R``\x8B\x01\x80QQa\x02@\x85\x01RQ\x81\x01Qa\x02`\x84\x01Ra\x02\x80\x83\x01\x86\x90R\x81Qa\x02\xA0\x84\x01R\x81\x81\x01Qa\x02\xC0\x84\x01R`@Qa\x06\0\x91\x84\x91\x01a\x14\xEDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPPa\x0B\xE9V[\x85`\xFF\x16`\x07\x03a\x0B\x92Wa\x06+a\x100V[`\0\x89`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\x06E\x91\x90a\x15\"V[\x90P\x80`\0` \x02\x01Q\x84`\0\x81Q\x81\x10a\x06bWa\x06ba\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x01` \x02\x01Q\x84`\x01\x81Q\x81\x10a\x06\x88Wa\x06\x88a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x02` \x02\x01Q\x84`\x02\x81Q\x81\x10a\x06\xAEWa\x06\xAEa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x03` \x02\x01Q\x84`\x03\x81Q\x81\x10a\x06\xD4Wa\x06\xD4a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x04` \x02\x01Q\x84`\x04\x81Q\x81\x10a\x06\xFAWa\x06\xFAa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x05` \x02\x01Q\x84`\x05\x81Q\x81\x10a\x07 Wa\x07 a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x06` \x02\x01Q\x84`\x06\x81Q\x81\x10a\x07FWa\x07Fa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x07` \x02\x01Q\x84`\x07\x81Q\x81\x10a\x07lWa\x07la\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`\x80\x8B\x01Q\x83R`\xA0\x8B\x01Q\x90\x83\x01R`@\x8A\x01Q\x80Q`\0\x90a\x07\xA1Wa\x07\xA1a\x14\xD7V[` \x02` \x01\x01Q\x82`\x02`\x1D\x81\x10a\x07\xBCWa\x07\xBCa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x01\x90\x81\x10a\x07\xD9Wa\x07\xD9a\x14\xD7V[` \x02` \x01\x01Q\x82`\x03`\x1D\x81\x10a\x07\xF4Wa\x07\xF4a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x02\x90\x81\x10a\x08\x11Wa\x08\x11a\x14\xD7V[` \x02` \x01\x01Q\x82`\x04`\x1D\x81\x10a\x08,Wa\x08,a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x03\x90\x81\x10a\x08IWa\x08Ia\x14\xD7V[` \x02` \x01\x01Q\x82`\x05`\x1D\x81\x10a\x08dWa\x08da\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x04\x90\x81\x10a\x08\x81Wa\x08\x81a\x14\xD7V[` \x02` \x01\x01Q\x82`\x06`\x1D\x81\x10a\x08\x9CWa\x08\x9Ca\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x05\x90\x81\x10a\x08\xB9Wa\x08\xB9a\x14\xD7V[` \x02` \x01\x01Q\x82`\x07`\x1D\x81\x10a\x08\xD4Wa\x08\xD4a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x06\x90\x81\x10a\x08\xF1Wa\x08\xF1a\x14\xD7V[` \x02` \x01\x01Q\x82`\x08`\x1D\x81\x10a\t\x0CWa\t\x0Ca\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x07\x90\x81\x10a\t)Wa\t)a\x14\xD7V[` \x02` \x01\x01Q\x82`\t`\x1D\x81\x10a\tDWa\tDa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x08\x90\x81\x10a\taWa\taa\x14\xD7V[` \x02` \x01\x01Q\x82`\n`\x1D\x81\x10a\t|Wa\t|a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\t\x90\x81\x10a\t\x99Wa\t\x99a\x14\xD7V[` \x02` \x01\x01Q\x82`\x0B`\x1D\x81\x10a\t\xB4Wa\t\xB4a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\n\x90\x81\x10a\t\xD1Wa\t\xD1a\x14\xD7V[` \x02` \x01\x01Q\x82`\x0C`\x1D\x81\x10a\t\xECWa\t\xECa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0B\x90\x81\x10a\n\tWa\n\ta\x14\xD7V[` \x02` \x01\x01Q\x82`\r`\x1D\x81\x10a\n$Wa\n$a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0C\x90\x81\x10a\nAWa\nAa\x14\xD7V[` \x02` \x01\x01Q\x82`\x0E`\x1D\x81\x10a\n\\Wa\n\\a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\r\x90\x81\x10a\nyWa\nya\x14\xD7V[` \x02` \x01\x01Q\x82`\x0F`\x1D\x81\x10a\n\x94Wa\n\x94a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0E\x90\x81\x10a\n\xB1Wa\n\xB1a\x14\xD7V[` \x02` \x01\x01Q\x82`\x10`\x1D\x81\x10a\n\xCCWa\n\xCCa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x0F\x90\x81\x10a\n\xE9Wa\n\xE9a\x14\xD7V[` \x02` \x01\x01Q\x82`\x11`\x1D\x81\x10a\x0B\x04Wa\x0B\x04a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x91\x90\x91R``\x8B\x81\x01\x80QQa\x02@\x86\x01RQ\x82\x01Qa\x02`\x85\x01Ra\x02\x80\x84\x01\x87\x90R\x82Qa\x02\xA0\x85\x01R\x82\x82\x01Qa\x02\xC0\x85\x01R`@\x80\x84\x01Qa\x02\xE0\x86\x01R\x90\x83\x01Qa\x03\0\x85\x01R`\x80\x83\x01Qa\x03 \x85\x01R`\xA0\x83\x01Qa\x03@\x85\x01R`\xC0\x83\x01Qa\x03`\x85\x01R`\xE0\x83\x01Qa\x03\x80\x85\x01RQa\x06\0\x91\x84\x91\x01a\x15\x96V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FVAnchorEncodeInputs: Invalid edg`D\x82\x01Raes`\xF0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x97\x90\x96P\x94PPPPPV[``\x80`\0a\x0C\x02a\0\xF0V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0a\x0C\x19\x85`\x01a\x14UV[`\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C4Wa\x0C4a\x10\x8DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C]W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P``\x85`\xFF\x16`\x01\x03a\r\xA3Wa\x0Cua\x10OV[`\0\x89`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\x0C\x8F\x91\x90a\x14\x82V[\x90P\x80`\0` \x02\x01Q\x84`\0\x81Q\x81\x10a\x0C\xACWa\x0C\xACa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x01` \x02\x01Q\x84`\x01\x81Q\x81\x10a\x0C\xD2Wa\x0C\xD2a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`\x80\x8B\x01Q\x83R`\xA0\x8B\x01Q\x90\x83\x01R`@\x8A\x01Q\x80Q`\0\x90a\r\x07Wa\r\x07a\x14\xD7V[` \x02` \x01\x01Q\x82`\x02`\t\x81\x10a\r\"Wa\r\"a\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x01\x90\x81\x10a\r?Wa\r?a\x14\xD7V[` \x02` \x01\x01Q\x82`\x03`\t\x81\x10a\rZWa\rZa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x91\x90\x91R``\x8B\x01\x80QQ`\x80\x85\x01RQ\x81\x01Q`\xA0\x84\x01R`\xC0\x83\x01\x86\x90R\x81Q`\xE0\x84\x01R\x81\x81\x01Qa\x01\0\x84\x01R`@Qa\x06\0\x91\x84\x91\x01a\x15\xCBV[\x85`\xFF\x16`\x07\x03a\x0B\x92Wa\r\xB6a\x10nV[`\0\x89`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\r\xD0\x91\x90a\x15\"V[\x90P\x80`\0` \x02\x01Q\x84`\0\x81Q\x81\x10a\r\xEDWa\r\xEDa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x01` \x02\x01Q\x84`\x01\x81Q\x81\x10a\x0E\x13Wa\x0E\x13a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x02` \x02\x01Q\x84`\x02\x81Q\x81\x10a\x0E9Wa\x0E9a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x03` \x02\x01Q\x84`\x03\x81Q\x81\x10a\x0E_Wa\x0E_a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x04` \x02\x01Q\x84`\x04\x81Q\x81\x10a\x0E\x85Wa\x0E\x85a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x05` \x02\x01Q\x84`\x05\x81Q\x81\x10a\x0E\xABWa\x0E\xABa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x06` \x02\x01Q\x84`\x06\x81Q\x81\x10a\x0E\xD1Wa\x0E\xD1a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80`\x07` \x02\x01Q\x84`\x07\x81Q\x81\x10a\x0E\xF7Wa\x0E\xF7a\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`\x80\x8B\x01Q\x83R`\xA0\x8B\x01Q\x90\x83\x01R`@\x8A\x01Q\x80Q`\0\x90a\x0F,Wa\x0F,a\x14\xD7V[` \x02` \x01\x01Q\x82`\x02`\x0F\x81\x10a\x0FGWa\x0FGa\x14\xD7V[` \x02\x01R`@\x8A\x01Q\x80Q`\x01\x90\x81\x10a\x0FdWa\x0Fda\x14\xD7V[` \x02` \x01\x01Q\x82`\x03`\x0F\x81\x10a\x0F\x7FWa\x0F\x7Fa\x14\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x91\x90\x91R``\x8B\x81\x01\x80QQ`\x80\x80\x87\x01\x91\x90\x91R\x90Q\x83\x01Q`\xA0\x80\x87\x01\x91\x90\x91R`\xC0\x80\x87\x01\x8A\x90R\x85Q`\xE0\x80\x89\x01\x91\x90\x91R\x86\x86\x01Qa\x01\0\x89\x01R`@\x80\x88\x01Qa\x01 \x8A\x01R\x94\x87\x01Qa\x01@\x89\x01R\x92\x86\x01Qa\x01`\x88\x01R\x90\x85\x01Qa\x01\x80\x87\x01R\x84\x01Qa\x01\xA0\x86\x01R\x83\x01Qa\x01\xC0\x85\x01RQa\x06\0\x91\x84\x91\x01a\x16\0V[`@Q\x80a\x02\xE0\x01`@R\x80`\x17\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x03\xA0\x01`@R\x80`\x1D\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01 \x01`@R\x80`\t\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01\xE0\x01`@R\x80`\x0F\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xC6Wa\x10\xC6a\x10\x8DV[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xC6Wa\x10\xC6a\x10\x8DV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11\x18Wa\x11\x18a\x10\x8DV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x111W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11KWa\x11Ka\x10\x8DV[a\x11^`\x1F\x82\x01`\x1F\x19\x16` \x01a\x10\xEFV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x11sW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x11\xA1W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\xBDWa\x11\xBDa\x10\x8DV[\x81`\x05\x1Ba\x11\xCC\x82\x82\x01a\x10\xEFV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x11\xE6W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x12\x05W\x825\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x11\xECV[\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x12!W`\0\x80\xFD[a\x12)a\x10\xCCV[\x80`@\x84\x01\x85\x81\x11\x15a\x12;W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x12UW\x805\x84R` \x93\x84\x01\x93\x01a\x12=V[P\x90\x95\x94PPPPPV[\x805`\xFF\x81\x16\x81\x14a\x12qW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\x8BW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\xA3W`\0\x80\xFD[\x90\x85\x01\x90`\xE0\x82\x88\x03\x12\x15a\x12\xB7W`\0\x80\xFD[a\x12\xBFa\x10\xA3V[\x825\x82\x81\x11\x15a\x12\xCEW`\0\x80\xFD[a\x12\xDA\x89\x82\x86\x01a\x11 V[\x82RP` \x83\x015\x82\x81\x11\x15a\x12\xEFW`\0\x80\xFD[a\x12\xFB\x89\x82\x86\x01a\x11 V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x13\x13W`\0\x80\xFD[a\x13\x1F\x89\x82\x86\x01a\x11\x90V[`@\x83\x01RPa\x132\x88``\x85\x01a\x12\x10V[``\x82\x01R`\xA0\x83\x015`\x80\x82\x01R`\xC0\x83\x015`\xA0\x82\x01R\x80\x95PP` \x86\x015\x91P\x80\x82\x11\x15a\x13cW`\0\x80\xFD[Pa\x13p\x86\x82\x87\x01a\x11 V[\x92PPa\x13\x7F`@\x85\x01a\x12`V[\x90P\x92P\x92P\x92V[`@\x81R`\0\x83Q\x80`@\x84\x01R`\0[\x81\x81\x10\x15a\x13\xB6W` \x81\x87\x01\x81\x01Q``\x86\x84\x01\x01R\x01a\x13\x99V[P`\0``\x82\x85\x01\x01R`\x1F\x19`\x1F\x82\x01\x16\x83\x01\x90P``\x81\x01` ``\x85\x84\x03\x01\x81\x86\x01R\x81\x86Q\x80\x84R`\x80\x85\x01\x91P\x82\x88\x01\x94P`\0\x93P[\x80\x84\x10\x15a\x14\x12W\x84Q\x82R\x93\x82\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a\x13\xF2V[P\x97\x96PPPPPPPV[\x80Q` \x82\x01Q`\x01`\x01`\xD0\x1B\x03\x19\x80\x82\x16\x92\x91\x90`\x06\x83\x10\x15a\x14MW\x80\x81\x84`\x06\x03`\x03\x1B\x1B\x83\x16\x16\x93P[PPP\x91\x90PV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x14|WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x14\x94W`\0\x80\xFD[\x82`\x1F\x83\x01\x12a\x14\xA3W`\0\x80\xFD[a\x14\xABa\x10\xCCV[\x80`@\x84\x01\x85\x81\x11\x15a\x14\xBDW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x12UW\x80Q\x84R` \x93\x84\x01\x93\x01a\x14\xBFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81\x83\x82[`\x17\x81\x10\x15a\x15\x12W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x14\xF3V[PPPa\x02\xE0\x82\x01\x90P\x92\x91PPV[`\0a\x01\0\x80\x83\x85\x03\x12\x15a\x156W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x15EW`\0\x80\xFD[`@Q\x81\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x15gWa\x15ga\x10\x8DV[`@R\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x15|W`\0\x80\xFD[\x84[\x83\x81\x10\x15a\x12UW\x80Q\x82R` \x91\x82\x01\x91\x01a\x15~V[`\0\x81\x83\x82[`\x1D\x81\x10\x15a\x15\xBBW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x15\x9CV[PPPa\x03\xA0\x82\x01\x90P\x92\x91PPV[`\0\x81\x83\x82[`\t\x81\x10\x15a\x15\xF0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x15\xD1V[PPPa\x01 \x82\x01\x90P\x92\x91PPV[`\0\x81\x83\x82[`\x0F\x81\x10\x15a\x16%W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x16\x06V[PPPa\x01\xE0\x82\x01\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xC4$\x04\x01=\x15_\x86\xAF\xB8:\xC12hH<\xA33\x83\xFC\x01\x85a\xB9\x9E\xDC\xDA1\xE0>N\x01dsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static VANCHORENCODEINPUTSCONTRACT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct VAnchorEncodeInputsContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for VAnchorEncodeInputsContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for VAnchorEncodeInputsContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for VAnchorEncodeInputsContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for VAnchorEncodeInputsContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(VAnchorEncodeInputsContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> VAnchorEncodeInputsContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                VANCHORENCODEINPUTSCONTRACT_ABI.clone(),
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
                VANCHORENCODEINPUTSCONTRACT_ABI.clone(),
                VANCHORENCODEINPUTSCONTRACT_BYTECODE.clone().into(),
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
        ///Calls the contract's `_encodeInputs16` (0x62506d38) function
        pub fn encode_inputs_16(
            &self,
            args: PublicInputs,
            p1: ::ethers::core::types::Bytes,
            max_edges: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Bytes,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([98, 80, 109, 56], (args, p1, max_edges))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_encodeInputs2` (0xd41d6d1e) function
        pub fn encode_inputs_2(
            &self,
            args: PublicInputs,
            p1: ::ethers::core::types::Bytes,
            max_edges: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Bytes,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([212, 29, 109, 30], (args, p1, max_edges))
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
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>>
        for VAnchorEncodeInputsContract<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    ///Container type for all input parameters for the `_encodeInputs16` function with signature `_encodeInputs16((bytes,bytes,uint256[],uint256[2],uint256,uint256),bytes,uint8)` and selector `0x62506d38`
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
        name = "_encodeInputs16",
        abi = "_encodeInputs16((bytes,bytes,uint256[],uint256[2],uint256,uint256),bytes,uint8)"
    )]
    pub struct EncodeInputs16Call {
        pub args: PublicInputs,
        pub p1: ::ethers::core::types::Bytes,
        pub max_edges: u8,
    }
    ///Container type for all input parameters for the `_encodeInputs2` function with signature `_encodeInputs2((bytes,bytes,uint256[],uint256[2],uint256,uint256),bytes,uint8)` and selector `0xd41d6d1e`
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
        name = "_encodeInputs2",
        abi = "_encodeInputs2((bytes,bytes,uint256[],uint256[2],uint256,uint256),bytes,uint8)"
    )]
    pub struct EncodeInputs2Call {
        pub args: PublicInputs,
        pub p1: ::ethers::core::types::Bytes,
        pub max_edges: u8,
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
    pub enum VAnchorEncodeInputsContractCalls {
        EvmChainIdType(EvmChainIdTypeCall),
        EncodeInputs16(EncodeInputs16Call),
        EncodeInputs2(EncodeInputs2Call),
        GetChainId(GetChainIdCall),
        GetChainIdType(GetChainIdTypeCall),
    }
    impl ::ethers::core::abi::AbiDecode for VAnchorEncodeInputsContractCalls {
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
                <EncodeInputs16Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::EncodeInputs16(decoded));
            }
            if let Ok(decoded) =
                <EncodeInputs2Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::EncodeInputs2(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VAnchorEncodeInputsContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EvmChainIdType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeInputs16(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeInputs2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainIdType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for VAnchorEncodeInputsContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::EvmChainIdType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeInputs16(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeInputs2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainIdType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EvmChainIdTypeCall>
        for VAnchorEncodeInputsContractCalls
    {
        fn from(value: EvmChainIdTypeCall) -> Self {
            Self::EvmChainIdType(value)
        }
    }
    impl ::core::convert::From<EncodeInputs16Call>
        for VAnchorEncodeInputsContractCalls
    {
        fn from(value: EncodeInputs16Call) -> Self {
            Self::EncodeInputs16(value)
        }
    }
    impl ::core::convert::From<EncodeInputs2Call>
        for VAnchorEncodeInputsContractCalls
    {
        fn from(value: EncodeInputs2Call) -> Self {
            Self::EncodeInputs2(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall>
        for VAnchorEncodeInputsContractCalls
    {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<GetChainIdTypeCall>
        for VAnchorEncodeInputsContractCalls
    {
        fn from(value: GetChainIdTypeCall) -> Self {
            Self::GetChainIdType(value)
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
    ///Container type for all return fields from the `_encodeInputs16` function with signature `_encodeInputs16((bytes,bytes,uint256[],uint256[2],uint256,uint256),bytes,uint8)` and selector `0x62506d38`
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
    pub struct EncodeInputs16Return(
        pub ::ethers::core::types::Bytes,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `_encodeInputs2` function with signature `_encodeInputs2((bytes,bytes,uint256[],uint256[2],uint256,uint256),bytes,uint8)` and selector `0xd41d6d1e`
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
    pub struct EncodeInputs2Return(
        pub ::ethers::core::types::Bytes,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
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
}
