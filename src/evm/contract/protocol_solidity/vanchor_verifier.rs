pub use v_anchor_verifier_contract::*;
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
pub mod v_anchor_verifier_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_verifier_2_2"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IVAnchorVerifier2_2",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_verifier_2_16"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IVAnchorVerifier2_16",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_verifier_8_2"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IVAnchorVerifier8_2",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_verifier_8_16"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IVAnchorVerifier8_16",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("v2_16"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("v2_16"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IVAnchorVerifier2_16",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("v2_2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("v2_2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IVAnchorVerifier2_2",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("v8_16"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("v8_16"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IVAnchorVerifier8_16",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("v8_2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("v8_2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IVAnchorVerifier8_2",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifyProof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifyProof"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
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
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                                    name: ::std::borrow::ToOwned::to_owned("c"),
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
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxEdges"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("smallInputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static VANCHORVERIFIERCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x08\xA48\x03\x80a\x08\xA4\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x9AV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x94\x86\x16\x94\x82\x16\x94\x90\x94\x17\x90\x93U`\x02\x80T\x92\x85\x16\x92\x84\x16\x92\x90\x92\x17\x90\x91U`\x03\x80T\x91\x90\x93\x16\x91\x16\x17\x90Ua\0\xF9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x97W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\0\xB0W`\0\x80\xFD[\x84Qa\0\xBB\x81a\0\x82V[` \x86\x01Q\x90\x94Pa\0\xCC\x81a\0\x82V[`@\x86\x01Q\x90\x93Pa\0\xDD\x81a\0\x82V[``\x86\x01Q\x90\x92Pa\0\xEE\x81a\0\x82V[\x93\x96\x92\x95P\x90\x93PPV[a\x07\x9C\x80a\x01\x08`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c.B\x86\x86\x14a\0\\W\x80c0\xBA\x9EM\x14a\0\x8CW\x80c\x80A\xCAS\x14a\0\x9FW\x80c\xE6_\x86\xAF\x14a\0\xC2W\x80c\xF8\xD5\x066\x14a\0\xD5W[`\0\x80\xFD[`\0Ta\0o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x02Ta\0o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xB2a\0\xAD6`\x04a\x03\xE5V[a\0\xE8V[`@Q\x90\x15\x15\x81R` \x01a\0\x83V[`\x03Ta\0o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01Ta\0o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x82`\xFF\x16`\x01\x03a\x01\xDEW\x81\x15a\x01\x91W`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x01\x11\x91\x90a\x04\xC1V[`\0T`@Qc\xC5B\xC9;`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC5B\xC9;\x90a\x01H\x90\x8B\x90\x8B\x90\x8B\x90\x87\x90`\x04\x01a\x05\xB6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x89\x91\x90a\x06\x13V[\x91PPa\x02\x8DV[`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x01\xA7\x91\x90a\x067V[`\x01T`@QcPaP\xCD`\xE1\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA0\xC2\xA1\x9A\x90a\x01H\x90\x8B\x90\x8B\x90\x8B\x90\x87\x90`\x04\x01a\x06KV[\x82`\xFF\x16`\x07\x03a\x02\x89W\x81\x15a\x02<W`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x02\x05\x91\x90a\x06\x9CV[`\x02T`@Qc\xF0T\xA9\xA3`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF0T\xA9\xA3\x90a\x01H\x90\x8B\x90\x8B\x90\x8B\x90\x87\x90`\x04\x01a\x06\xB0V[`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x02R\x91\x90a\x07\x01V[`\x03T`@Qc*\\P\xA3`\xE1\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cT\xB8\xA1F\x90a\x01H\x90\x8B\x90\x8B\x90\x8B\x90\x87\x90`\x04\x01a\x07\x15V[P`\0[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xD0Wa\x02\xD0a\x02\x97V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x02\xE7W`\0\x80\xFD[a\x02\xEFa\x02\xADV[\x80`@\x84\x01\x85\x81\x11\x15a\x03\x01W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x03\x1BW\x805\x84R` \x93\x84\x01\x93\x01a\x03\x03V[P\x90\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a\x037W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03RWa\x03Ra\x02\x97V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x03zWa\x03za\x02\x97V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x03\x93W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a\x03\xC4W`\0\x80\xFD[\x91\x90PV[\x80\x15\x15\x81\x14a\x03\xD7W`\0\x80\xFD[PV[\x805a\x03\xC4\x81a\x03\xC9V[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15a\x03\xFFW`\0\x80\xFD[a\x04\t\x88\x88a\x02\xD6V[\x95P`@\x88`_\x89\x01\x12a\x04\x1CW`\0\x80\xFD[a\x04$a\x02\xADV[\x80`\xC0\x8A\x01\x8B\x81\x11\x15a\x046W`\0\x80\xFD[\x83\x8B\x01[\x81\x81\x10\x15a\x04[Wa\x04L\x8D\x82a\x02\xD6V[\x84R` \x90\x93\x01\x92\x84\x01a\x04:V[P\x81\x98Pa\x04i\x8C\x82a\x02\xD6V[\x97PPPPPa\x01\0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x8AW`\0\x80\xFD[a\x04\x96\x89\x82\x8A\x01a\x03&V[\x93PPa\x04\xA6a\x01 \x88\x01a\x03\xB3V[\x91Pa\x04\xB5a\x01@\x88\x01a\x03\xDAV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0a\x01 \x80\x83\x85\x03\x12\x15a\x04\xD5W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x04\xE4W`\0\x80\xFD[`@Q\x81\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x05\x06Wa\x05\x06a\x02\x97V[`@R\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x05\x1BW`\0\x80\xFD[\x84[\x83\x81\x10\x15a\x03\x1BW\x80Q\x82R` \x91\x82\x01\x91\x01a\x05\x1DV[\x80`\0[`\x02\x81\x10\x15a\x05XW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x059V[PPPPV[\x80`\0\x80[`\x02\x80\x82\x10a\x05rWPa\x05\xAFV[\x83Q\x86\x84[\x83\x81\x10\x15a\x05\x95W\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x05wV[PPP`@\x95\x90\x95\x01\x94P` \x92\x90\x92\x01\x91`\x01\x01a\x05cV[PPPPPV[a\x02 \x81\x01a\x05\xC5\x82\x87a\x055V[a\x05\xD2`@\x83\x01\x86a\x05^V[a\x05\xDF`\xC0\x83\x01\x85a\x055V[a\x01\0\x82\x01\x83`\0[`\t\x81\x10\x15a\x06\x07W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x05\xE8V[PPP\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x06%W`\0\x80\xFD[\x81Qa\x060\x81a\x03\xC9V[\x93\x92PPPV[`\0a\x02\xE0\x80\x83\x85\x03\x12\x15a\x04\xD5W`\0\x80\xFD[a\x03\xE0\x81\x01a\x06Z\x82\x87a\x055V[a\x06g`@\x83\x01\x86a\x05^V[a\x06t`\xC0\x83\x01\x85a\x055V[a\x01\0\x82\x01\x83`\0[`\x17\x81\x10\x15a\x06\x07W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x06}V[`\0a\x01\xE0\x80\x83\x85\x03\x12\x15a\x04\xD5W`\0\x80\xFD[a\x02\xE0\x81\x01a\x06\xBF\x82\x87a\x055V[a\x06\xCC`@\x83\x01\x86a\x05^V[a\x06\xD9`\xC0\x83\x01\x85a\x055V[a\x01\0\x82\x01\x83`\0[`\x0F\x81\x10\x15a\x06\x07W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x06\xE2V[`\0a\x03\xA0\x80\x83\x85\x03\x12\x15a\x04\xD5W`\0\x80\xFD[a\x04\xA0\x81\x01a\x07$\x82\x87a\x055V[a\x071`@\x83\x01\x86a\x05^V[a\x07>`\xC0\x83\x01\x85a\x055V[a\x01\0\x82\x01\x83`\0[`\x1D\x81\x10\x15a\x06\x07W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x07GV\xFE\xA2dipfsX\"\x12 T\x8F\xD5\xD9\xAF\x99\x06Z\xD0\x9E\xFD\x01\x83\xEC\xD7U`\xE4\xDB\xE6\x9F'\x02\xB5\xC8$\xCA~\xE9.o\x12dsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static VANCHORVERIFIERCONTRACT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c.B\x86\x86\x14a\0\\W\x80c0\xBA\x9EM\x14a\0\x8CW\x80c\x80A\xCAS\x14a\0\x9FW\x80c\xE6_\x86\xAF\x14a\0\xC2W\x80c\xF8\xD5\x066\x14a\0\xD5W[`\0\x80\xFD[`\0Ta\0o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x02Ta\0o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xB2a\0\xAD6`\x04a\x03\xE5V[a\0\xE8V[`@Q\x90\x15\x15\x81R` \x01a\0\x83V[`\x03Ta\0o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01Ta\0o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0\x82`\xFF\x16`\x01\x03a\x01\xDEW\x81\x15a\x01\x91W`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x01\x11\x91\x90a\x04\xC1V[`\0T`@Qc\xC5B\xC9;`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC5B\xC9;\x90a\x01H\x90\x8B\x90\x8B\x90\x8B\x90\x87\x90`\x04\x01a\x05\xB6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x89\x91\x90a\x06\x13V[\x91PPa\x02\x8DV[`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x01\xA7\x91\x90a\x067V[`\x01T`@QcPaP\xCD`\xE1\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA0\xC2\xA1\x9A\x90a\x01H\x90\x8B\x90\x8B\x90\x8B\x90\x87\x90`\x04\x01a\x06KV[\x82`\xFF\x16`\x07\x03a\x02\x89W\x81\x15a\x02<W`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x02\x05\x91\x90a\x06\x9CV[`\x02T`@Qc\xF0T\xA9\xA3`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF0T\xA9\xA3\x90a\x01H\x90\x8B\x90\x8B\x90\x8B\x90\x87\x90`\x04\x01a\x06\xB0V[`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x02R\x91\x90a\x07\x01V[`\x03T`@Qc*\\P\xA3`\xE1\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cT\xB8\xA1F\x90a\x01H\x90\x8B\x90\x8B\x90\x8B\x90\x87\x90`\x04\x01a\x07\x15V[P`\0[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xD0Wa\x02\xD0a\x02\x97V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x02\xE7W`\0\x80\xFD[a\x02\xEFa\x02\xADV[\x80`@\x84\x01\x85\x81\x11\x15a\x03\x01W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x03\x1BW\x805\x84R` \x93\x84\x01\x93\x01a\x03\x03V[P\x90\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a\x037W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03RWa\x03Ra\x02\x97V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x03zWa\x03za\x02\x97V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x03\x93W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a\x03\xC4W`\0\x80\xFD[\x91\x90PV[\x80\x15\x15\x81\x14a\x03\xD7W`\0\x80\xFD[PV[\x805a\x03\xC4\x81a\x03\xC9V[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15a\x03\xFFW`\0\x80\xFD[a\x04\t\x88\x88a\x02\xD6V[\x95P`@\x88`_\x89\x01\x12a\x04\x1CW`\0\x80\xFD[a\x04$a\x02\xADV[\x80`\xC0\x8A\x01\x8B\x81\x11\x15a\x046W`\0\x80\xFD[\x83\x8B\x01[\x81\x81\x10\x15a\x04[Wa\x04L\x8D\x82a\x02\xD6V[\x84R` \x90\x93\x01\x92\x84\x01a\x04:V[P\x81\x98Pa\x04i\x8C\x82a\x02\xD6V[\x97PPPPPa\x01\0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x8AW`\0\x80\xFD[a\x04\x96\x89\x82\x8A\x01a\x03&V[\x93PPa\x04\xA6a\x01 \x88\x01a\x03\xB3V[\x91Pa\x04\xB5a\x01@\x88\x01a\x03\xDAV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0a\x01 \x80\x83\x85\x03\x12\x15a\x04\xD5W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x04\xE4W`\0\x80\xFD[`@Q\x81\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x05\x06Wa\x05\x06a\x02\x97V[`@R\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x05\x1BW`\0\x80\xFD[\x84[\x83\x81\x10\x15a\x03\x1BW\x80Q\x82R` \x91\x82\x01\x91\x01a\x05\x1DV[\x80`\0[`\x02\x81\x10\x15a\x05XW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x059V[PPPPV[\x80`\0\x80[`\x02\x80\x82\x10a\x05rWPa\x05\xAFV[\x83Q\x86\x84[\x83\x81\x10\x15a\x05\x95W\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x05wV[PPP`@\x95\x90\x95\x01\x94P` \x92\x90\x92\x01\x91`\x01\x01a\x05cV[PPPPPV[a\x02 \x81\x01a\x05\xC5\x82\x87a\x055V[a\x05\xD2`@\x83\x01\x86a\x05^V[a\x05\xDF`\xC0\x83\x01\x85a\x055V[a\x01\0\x82\x01\x83`\0[`\t\x81\x10\x15a\x06\x07W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x05\xE8V[PPP\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x06%W`\0\x80\xFD[\x81Qa\x060\x81a\x03\xC9V[\x93\x92PPPV[`\0a\x02\xE0\x80\x83\x85\x03\x12\x15a\x04\xD5W`\0\x80\xFD[a\x03\xE0\x81\x01a\x06Z\x82\x87a\x055V[a\x06g`@\x83\x01\x86a\x05^V[a\x06t`\xC0\x83\x01\x85a\x055V[a\x01\0\x82\x01\x83`\0[`\x17\x81\x10\x15a\x06\x07W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x06}V[`\0a\x01\xE0\x80\x83\x85\x03\x12\x15a\x04\xD5W`\0\x80\xFD[a\x02\xE0\x81\x01a\x06\xBF\x82\x87a\x055V[a\x06\xCC`@\x83\x01\x86a\x05^V[a\x06\xD9`\xC0\x83\x01\x85a\x055V[a\x01\0\x82\x01\x83`\0[`\x0F\x81\x10\x15a\x06\x07W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x06\xE2V[`\0a\x03\xA0\x80\x83\x85\x03\x12\x15a\x04\xD5W`\0\x80\xFD[a\x04\xA0\x81\x01a\x07$\x82\x87a\x055V[a\x071`@\x83\x01\x86a\x05^V[a\x07>`\xC0\x83\x01\x85a\x055V[a\x01\0\x82\x01\x83`\0[`\x1D\x81\x10\x15a\x06\x07W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x07GV\xFE\xA2dipfsX\"\x12 T\x8F\xD5\xD9\xAF\x99\x06Z\xD0\x9E\xFD\x01\x83\xEC\xD7U`\xE4\xDB\xE6\x9F'\x02\xB5\xC8$\xCA~\xE9.o\x12dsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static VANCHORVERIFIERCONTRACT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct VAnchorVerifierContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for VAnchorVerifierContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for VAnchorVerifierContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for VAnchorVerifierContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for VAnchorVerifierContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(VAnchorVerifierContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> VAnchorVerifierContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                VANCHORVERIFIERCONTRACT_ABI.clone(),
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
                VANCHORVERIFIERCONTRACT_ABI.clone(),
                VANCHORVERIFIERCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `v2_16` (0xf8d50636) function
        pub fn v_2_16(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 213, 6, 54], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `v2_2` (0x2e428686) function
        pub fn v_2_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([46, 66, 134, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `v8_16` (0xe65f86af) function
        pub fn v_8_16(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([230, 95, 134, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `v8_2` (0x30ba9e4d) function
        pub fn v_8_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([48, 186, 158, 77], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyProof` (0x8041ca53) function
        pub fn verify_proof(
            &self,
            a: [::ethers::core::types::U256; 2],
            b: [[::ethers::core::types::U256; 2]; 2],
            c: [::ethers::core::types::U256; 2],
            input: ::ethers::core::types::Bytes,
            max_edges: u8,
            small_inputs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [128, 65, 202, 83],
                    (a, b, c, input, max_edges, small_inputs),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for VAnchorVerifierContract<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `v2_16` function with signature `v2_16()` and selector `0xf8d50636`
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
    #[ethcall(name = "v2_16", abi = "v2_16()")]
    pub struct V216Call;
    ///Container type for all input parameters for the `v2_2` function with signature `v2_2()` and selector `0x2e428686`
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
    #[ethcall(name = "v2_2", abi = "v2_2()")]
    pub struct V22Call;
    ///Container type for all input parameters for the `v8_16` function with signature `v8_16()` and selector `0xe65f86af`
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
    #[ethcall(name = "v8_16", abi = "v8_16()")]
    pub struct V816Call;
    ///Container type for all input parameters for the `v8_2` function with signature `v8_2()` and selector `0x30ba9e4d`
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
    #[ethcall(name = "v8_2", abi = "v8_2()")]
    pub struct V82Call;
    ///Container type for all input parameters for the `verifyProof` function with signature `verifyProof(uint256[2],uint256[2][2],uint256[2],bytes,uint8,bool)` and selector `0x8041ca53`
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
        name = "verifyProof",
        abi = "verifyProof(uint256[2],uint256[2][2],uint256[2],bytes,uint8,bool)"
    )]
    pub struct VerifyProofCall {
        pub a: [::ethers::core::types::U256; 2],
        pub b: [[::ethers::core::types::U256; 2]; 2],
        pub c: [::ethers::core::types::U256; 2],
        pub input: ::ethers::core::types::Bytes,
        pub max_edges: u8,
        pub small_inputs: bool,
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
    pub enum VAnchorVerifierContractCalls {
        V216(V216Call),
        V22(V22Call),
        V816(V816Call),
        V82(V82Call),
        VerifyProof(VerifyProofCall),
    }
    impl ::ethers::core::abi::AbiDecode for VAnchorVerifierContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <V216Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::V216(decoded));
            }
            if let Ok(decoded) =
                <V22Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::V22(decoded));
            }
            if let Ok(decoded) =
                <V816Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::V816(decoded));
            }
            if let Ok(decoded) =
                <V82Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::V82(decoded));
            }
            if let Ok(decoded) =
                <VerifyProofCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::VerifyProof(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VAnchorVerifierContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::V216(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::V22(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::V816(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::V82(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for VAnchorVerifierContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::V216(element) => ::core::fmt::Display::fmt(element, f),
                Self::V22(element) => ::core::fmt::Display::fmt(element, f),
                Self::V816(element) => ::core::fmt::Display::fmt(element, f),
                Self::V82(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyProof(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<V216Call> for VAnchorVerifierContractCalls {
        fn from(value: V216Call) -> Self {
            Self::V216(value)
        }
    }
    impl ::core::convert::From<V22Call> for VAnchorVerifierContractCalls {
        fn from(value: V22Call) -> Self {
            Self::V22(value)
        }
    }
    impl ::core::convert::From<V816Call> for VAnchorVerifierContractCalls {
        fn from(value: V816Call) -> Self {
            Self::V816(value)
        }
    }
    impl ::core::convert::From<V82Call> for VAnchorVerifierContractCalls {
        fn from(value: V82Call) -> Self {
            Self::V82(value)
        }
    }
    impl ::core::convert::From<VerifyProofCall> for VAnchorVerifierContractCalls {
        fn from(value: VerifyProofCall) -> Self {
            Self::VerifyProof(value)
        }
    }
    ///Container type for all return fields from the `v2_16` function with signature `v2_16()` and selector `0xf8d50636`
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
    pub struct V216Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `v2_2` function with signature `v2_2()` and selector `0x2e428686`
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
    pub struct V22Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `v8_16` function with signature `v8_16()` and selector `0xe65f86af`
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
    pub struct V816Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `v8_2` function with signature `v8_2()` and selector `0x30ba9e4d`
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
    pub struct V82Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `verifyProof` function with signature `verifyProof(uint256[2],uint256[2][2],uint256[2],bytes,uint8,bool)` and selector `0x8041ca53`
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
    pub struct VerifyProofReturn {
        pub r: bool,
    }
}
