pub use treasury_contract::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod treasury_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_treasuryHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("rescueTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rescueTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToRescue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("setHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setHandler"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newHandler"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("TreasuryHandlerUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TreasuryHandlerUpdated",
                            ),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TREASURYCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0B\x0F8\x03\x80a\x0B\x0F\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xBCV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\0\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTreasury: Treasury Handler can't`D\x82\x01Rd\x02\x06&R\x03`\xDC\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\xECV[`\0` \x82\x84\x03\x12\x15a\0\xCEW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xE5W`\0\x80\xFD[\x93\x92PPPV[a\n\x14\x80a\0\xFB`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0CW`\x005`\xE0\x1C\x80c\x0B'\xFB\x9A\x14a\0OW\x80cb,w\xD9\x14a\0rW\x80cr\xC1\xAD\x03\x14a\0\x94W\x80c\xCC<t\xA1\x14a\0\xB4W`\0\x80\xFD[6a\0JW\0[`\0\x80\xFD[4\x80\x15a\0[W`\0\x80\xFD[P`\0T[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0~W`\0\x80\xFD[Pa\0\x92a\0\x8D6`\x04a\x07\x7FV[a\0\xCAV[\0[4\x80\x15a\0\xA0W`\0\x80\xFD[Pa\0\x92a\0\xAF6`\x04a\x07\xD0V[a\x03eV[4\x80\x15a\0\xC0W`\0\x80\xFD[Pa\0``\0T\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08\x05V[`@Q\x80\x91\x03\x90\xFD[\x80c\xFF\xFF\xFF\xFF\x16\x80`\0T\x10a\x01%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08bV[`\0Ta\x013\x90`\x01a\x08\xA5V[\x81\x11\x15a\x01RW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08\xCCV[`\0\x81\x90U`\x01`\x01`\xA0\x1B\x03\x84\x16a\x01\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FTreasury: Cannot send liquidity `D\x82\x01Rnto zero address`\x88\x1B`d\x82\x01R`\x84\x01a\0\xF4V[0`\x01`\x01`\xA0\x1B\x03\x86\x16\x03a\x02+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTreasury: Cannot rescue wrapped `D\x82\x01Rd\x18\\\xDC\xD9]`\xDA\x1B`d\x82\x01R`\x84\x01a\0\xF4V[`\x01`\x01`\xA0\x1B\x03\x85\x16a\x02\xBBWG\x83\x81\x10a\x02}W`@Q`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x85\x15a\x08\xFC\x02\x90\x86\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02wW=`\0\x80>=`\0\xFD[Pa\x02\xB5V[`@Q`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02\xB3W=`\0\x80>=`\0\xFD[P[Pa\x03^V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03&\x91\x90a\t)V[\x90P\x83\x81\x10a\x03HWa\x03C`\x01`\x01`\xA0\x1B\x03\x87\x16\x86\x86a\x04\x95V[a\x03\\V[a\x03\\`\x01`\x01`\xA0\x1B\x03\x87\x16\x86\x83a\x04\x95V[P[PPPPPV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08\x05V[\x80c\xFF\xFF\xFF\xFF\x16\x80`\0T\x10a\x03\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08bV[`\0Ta\x03\xC5\x90`\x01a\x08\xA5V[\x81\x11\x15a\x03\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08\xCCV[`\0\x81\x90U`\x01`\x01`\xA0\x1B\x03\x83\x16a\x04?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FTreasury: Handler cannot be 0\0\0\0`D\x82\x01R`d\x01a\0\xF4V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xD6G\tGK+u\x11\xA8\0\x9EVQ\x85\xFB9\x03\x15T\xA6\xEB\x99b\xEFx\\\x97\xD8u\x9F\x11\xD7\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x04\xE7\x90\x84\x90a\x04\xECV[PPPV[`\0a\x05A\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x05\xBE\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x04\xE7W\x80\x80` \x01\x90Q\x81\x01\x90a\x05_\x91\x90a\tBV[a\x04\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\0\xF4V[``a\x05\xCD\x84\x84`\0\x85a\x05\xD5V[\x94\x93PPPPV[``\x82G\x10\x15a\x066W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\0\xF4V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x06R\x91\x90a\t\x8FV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x06\x8FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\x94V[``\x91P[P\x91P\x91Pa\x06\xA5\x87\x83\x83\x87a\x06\xB0V[\x97\x96PPPPPPPV[``\x83\x15a\x07\x1FW\x82Q`\0\x03a\x07\x18W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x07\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\0\xF4V[P\x81a\x05\xCDV[a\x05\xCD\x83\x83\x81Q\x15a\x074W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x91\x90a\t\xABV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07cW`\0\x80\xFD[PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07zW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x07\x95W`\0\x80\xFD[\x845a\x07\xA0\x81a\x07NV[\x93P` \x85\x015a\x07\xB0\x81a\x07NV[\x92P`@\x85\x015\x91Pa\x07\xC5``\x86\x01a\x07fV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xE3W`\0\x80\xFD[\x825a\x07\xEE\x81a\x07NV[\x91Pa\x07\xFC` \x84\x01a\x07fV[\x90P\x92P\x92\x90PV[` \x80\x82R`9\x90\x82\x01R\x7FTreasury: Function can only be c`@\x82\x01R\x7Falled by treasury handler\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`#\x90\x82\x01R\x7FProposalNonceTracker: Invalid no`@\x82\x01Rbnce`\xE8\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x08\xC6WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[` \x80\x82R`:\x90\x82\x01R\x7FProposalNonceTracker: Nonce must`@\x82\x01R\x7F not increment more than 1\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\t;W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\tTW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\tdW`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\t\x86W\x81\x81\x01Q\x83\x82\x01R` \x01a\tnV[PP`\0\x91\x01RV[`\0\x82Qa\t\xA1\x81\x84` \x87\x01a\tkV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\t\xCA\x81`@\x85\x01` \x87\x01a\tkV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 $\xA5\x99\x18 \x81$Cq\x9AGO\xB9\x8A\x87\xF8\xAE\x9F\x0E\xA5\xEA4\x96\xEC8\xFA\xB1\xAF\x87\xE16\x1FdsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static TREASURYCONTRACT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0CW`\x005`\xE0\x1C\x80c\x0B'\xFB\x9A\x14a\0OW\x80cb,w\xD9\x14a\0rW\x80cr\xC1\xAD\x03\x14a\0\x94W\x80c\xCC<t\xA1\x14a\0\xB4W`\0\x80\xFD[6a\0JW\0[`\0\x80\xFD[4\x80\x15a\0[W`\0\x80\xFD[P`\0T[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0~W`\0\x80\xFD[Pa\0\x92a\0\x8D6`\x04a\x07\x7FV[a\0\xCAV[\0[4\x80\x15a\0\xA0W`\0\x80\xFD[Pa\0\x92a\0\xAF6`\x04a\x07\xD0V[a\x03eV[4\x80\x15a\0\xC0W`\0\x80\xFD[Pa\0``\0T\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08\x05V[`@Q\x80\x91\x03\x90\xFD[\x80c\xFF\xFF\xFF\xFF\x16\x80`\0T\x10a\x01%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08bV[`\0Ta\x013\x90`\x01a\x08\xA5V[\x81\x11\x15a\x01RW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08\xCCV[`\0\x81\x90U`\x01`\x01`\xA0\x1B\x03\x84\x16a\x01\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FTreasury: Cannot send liquidity `D\x82\x01Rnto zero address`\x88\x1B`d\x82\x01R`\x84\x01a\0\xF4V[0`\x01`\x01`\xA0\x1B\x03\x86\x16\x03a\x02+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTreasury: Cannot rescue wrapped `D\x82\x01Rd\x18\\\xDC\xD9]`\xDA\x1B`d\x82\x01R`\x84\x01a\0\xF4V[`\x01`\x01`\xA0\x1B\x03\x85\x16a\x02\xBBWG\x83\x81\x10a\x02}W`@Q`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x85\x15a\x08\xFC\x02\x90\x86\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02wW=`\0\x80>=`\0\xFD[Pa\x02\xB5V[`@Q`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02\xB3W=`\0\x80>=`\0\xFD[P[Pa\x03^V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03&\x91\x90a\t)V[\x90P\x83\x81\x10a\x03HWa\x03C`\x01`\x01`\xA0\x1B\x03\x87\x16\x86\x86a\x04\x95V[a\x03\\V[a\x03\\`\x01`\x01`\xA0\x1B\x03\x87\x16\x86\x83a\x04\x95V[P[PPPPPV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08\x05V[\x80c\xFF\xFF\xFF\xFF\x16\x80`\0T\x10a\x03\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08bV[`\0Ta\x03\xC5\x90`\x01a\x08\xA5V[\x81\x11\x15a\x03\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x90a\x08\xCCV[`\0\x81\x90U`\x01`\x01`\xA0\x1B\x03\x83\x16a\x04?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FTreasury: Handler cannot be 0\0\0\0`D\x82\x01R`d\x01a\0\xF4V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xD6G\tGK+u\x11\xA8\0\x9EVQ\x85\xFB9\x03\x15T\xA6\xEB\x99b\xEFx\\\x97\xD8u\x9F\x11\xD7\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x04\xE7\x90\x84\x90a\x04\xECV[PPPV[`\0a\x05A\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x05\xBE\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x04\xE7W\x80\x80` \x01\x90Q\x81\x01\x90a\x05_\x91\x90a\tBV[a\x04\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\0\xF4V[``a\x05\xCD\x84\x84`\0\x85a\x05\xD5V[\x94\x93PPPPV[``\x82G\x10\x15a\x066W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\0\xF4V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x06R\x91\x90a\t\x8FV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x06\x8FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\x94V[``\x91P[P\x91P\x91Pa\x06\xA5\x87\x83\x83\x87a\x06\xB0V[\x97\x96PPPPPPPV[``\x83\x15a\x07\x1FW\x82Q`\0\x03a\x07\x18W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x07\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\0\xF4V[P\x81a\x05\xCDV[a\x05\xCD\x83\x83\x81Q\x15a\x074W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xF4\x91\x90a\t\xABV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07cW`\0\x80\xFD[PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07zW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x07\x95W`\0\x80\xFD[\x845a\x07\xA0\x81a\x07NV[\x93P` \x85\x015a\x07\xB0\x81a\x07NV[\x92P`@\x85\x015\x91Pa\x07\xC5``\x86\x01a\x07fV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xE3W`\0\x80\xFD[\x825a\x07\xEE\x81a\x07NV[\x91Pa\x07\xFC` \x84\x01a\x07fV[\x90P\x92P\x92\x90PV[` \x80\x82R`9\x90\x82\x01R\x7FTreasury: Function can only be c`@\x82\x01R\x7Falled by treasury handler\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`#\x90\x82\x01R\x7FProposalNonceTracker: Invalid no`@\x82\x01Rbnce`\xE8\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x08\xC6WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[` \x80\x82R`:\x90\x82\x01R\x7FProposalNonceTracker: Nonce must`@\x82\x01R\x7F not increment more than 1\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\t;W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\tTW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\tdW`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\t\x86W\x81\x81\x01Q\x83\x82\x01R` \x01a\tnV[PP`\0\x91\x01RV[`\0\x82Qa\t\xA1\x81\x84` \x87\x01a\tkV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\t\xCA\x81`@\x85\x01` \x87\x01a\tkV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 $\xA5\x99\x18 \x81$Cq\x9AGO\xB9\x8A\x87\xF8\xAE\x9F\x0E\xA5\xEA4\x96\xEC8\xFA\xB1\xAF\x87\xE16\x1FdsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static TREASURYCONTRACT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TreasuryContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TreasuryContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TreasuryContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TreasuryContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TreasuryContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TreasuryContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TreasuryContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TREASURYCONTRACT_ABI.clone(),
                    client,
                ),
            )
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
                TREASURYCONTRACT_ABI.clone(),
                TREASURYCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getProposalNonce` (0x0b27fb9a) function
        pub fn get_proposal_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([11, 39, 251, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalNonce` (0xcc3c74a1) function
        pub fn proposal_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([204, 60, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rescueTokens` (0x622c77d9) function
        pub fn rescue_tokens(
            &self,
            token_address: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount_to_rescue: ::ethers::core::types::U256,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [98, 44, 119, 217],
                    (token_address, to, amount_to_rescue, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHandler` (0x72c1ad03) function
        pub fn set_handler(
            &self,
            new_handler: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 193, 173, 3], (new_handler, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `TreasuryHandlerUpdated` event
        pub fn treasury_handler_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TreasuryHandlerUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TreasuryHandlerUpdatedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TreasuryContract<M> {
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
        Hash
    )]
    #[ethevent(name = "TreasuryHandlerUpdated", abi = "TreasuryHandlerUpdated(address)")]
    pub struct TreasuryHandlerUpdatedFilter {
        pub handler: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethcall(name = "getProposalNonce", abi = "getProposalNonce()")]
    pub struct GetProposalNonceCall;
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
        Hash
    )]
    #[ethcall(name = "proposalNonce", abi = "proposalNonce()")]
    pub struct ProposalNonceCall;
    ///Container type for all input parameters for the `rescueTokens` function with signature `rescueTokens(address,address,uint256,uint32)` and selector `0x622c77d9`
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
        Hash
    )]
    #[ethcall(
        name = "rescueTokens",
        abi = "rescueTokens(address,address,uint256,uint32)"
    )]
    pub struct RescueTokensCall {
        pub token_address: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount_to_rescue: ::ethers::core::types::U256,
        pub nonce: u32,
    }
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
        Hash
    )]
    #[ethcall(name = "setHandler", abi = "setHandler(address,uint32)")]
    pub struct SetHandlerCall {
        pub new_handler: ::ethers::core::types::Address,
        pub nonce: u32,
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
        Hash
    )]
    pub enum TreasuryContractCalls {
        GetProposalNonce(GetProposalNonceCall),
        ProposalNonce(ProposalNonceCall),
        RescueTokens(RescueTokensCall),
        SetHandler(SetHandlerCall),
    }
    impl ::ethers::core::abi::AbiDecode for TreasuryContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProposalNonce(decoded));
            }
            if let Ok(decoded) = <ProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposalNonce(decoded));
            }
            if let Ok(decoded) = <RescueTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RescueTokens(decoded));
            }
            if let Ok(decoded) = <SetHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetHandler(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TreasuryContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RescueTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TreasuryContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetProposalNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::RescueTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetHandler(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetProposalNonceCall> for TreasuryContractCalls {
        fn from(value: GetProposalNonceCall) -> Self {
            Self::GetProposalNonce(value)
        }
    }
    impl ::core::convert::From<ProposalNonceCall> for TreasuryContractCalls {
        fn from(value: ProposalNonceCall) -> Self {
            Self::ProposalNonce(value)
        }
    }
    impl ::core::convert::From<RescueTokensCall> for TreasuryContractCalls {
        fn from(value: RescueTokensCall) -> Self {
            Self::RescueTokens(value)
        }
    }
    impl ::core::convert::From<SetHandlerCall> for TreasuryContractCalls {
        fn from(value: SetHandlerCall) -> Self {
            Self::SetHandler(value)
        }
    }
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
        Hash
    )]
    pub struct GetProposalNonceReturn(pub ::ethers::core::types::U256);
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
        Hash
    )]
    pub struct ProposalNonceReturn(pub ::ethers::core::types::U256);
}
