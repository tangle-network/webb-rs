#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs,
    rustdoc::broken_intra_doc_links,
    unused_results,
    clippy::all,
    clippy::pedantic,
    clippy::exhaustive_enums
)]

//! # Webb Protocol Proposals Specification
//!
//! A Reference Implementation of the Webb Protocol Proposals.
//!
//! ## Introduction
//!
//! Proposals are encoded as a sequence of bytes that describe the proposed
//! changes to the state of a system. An example of such a state is a storage
//! value on a smart contract.
//!
//! Each proposal contains the following:
//! 1. The target system we need to apply the proposal to.
//! 2. The source system that is sending the proposal (_optionally_).
//! 3. Which function on the target system we need to call.
//! 4. The arguments of this function.
//!
//! The first 3 points described above are called the [`ProposalHeader`] and
//! they are (by definition) the first `40` bytes of the proposal. The remaining
//! bytes are the body. The length of the body varies depending on what the
//! proposal does. Here is a diagram of a proposal:
//! ```text
//! ┌───────────────────┬──────────────┬───────────────────┐
//! │                   │              │                   │
//! │    Target 26B     │ ChainType 2B │    ChainId 4B     │
//! │                   │              │                   │
//! └───────────────┬───┴──────────────┴───────────────────┘
//!                 │
//!                 │
//!                 │
//!    ┌────────────▼───────────┬─────────────┬───────────┬──────────┐
//!    │                        │             │           │          │
//!    │     ResourceId 32B     │ FuncSig 4B  │ Nonce 4B  │ Body nB  │
//!    │                        │             │           │          │
//!    └────────────────────────┴─────────────┴───────────┴──────────┘
//! ```
//!
//! ## Proposal Header
//!
//! The proposal header is the first 40 bytes of the proposal. It contains the
//! following:
//! - The [`ResourceId`] which is a 32 byte value that uniquely identifies the
//!   target system.
//! - The [`FunctionSignature`] which is a 4 byte value that uniquely identifies
//!   the function to be executed on the target system.
//! - The [`Nonce`] which is a 4 byte value that is used to prevent replay
//!   attacks.
//!
//! ## The `ResourceId`
//!
//! The `ResourceId` is the first 32 bytes of the proposal header, and it
//! contains the following:
//! 1. The [`TargetSystem`] which is contained within the first 26 bytes of the
//! `ResourceId`. The [`TargetSystem`] could be one of the following (depending
//! on the target system):
//!    - A [`TargetSystem::ContractAddress`] which is actually 20 bytes but is
//!      left padded with zeroes (in this case, 6 bytes of zeroes).
#![cfg_attr(
    features = "substrate",
    doc = "    - A [`TargetSystem::Substrate`] which is the target system for Substrate based systems"
)]
//!      padded with zeroes (in this case, 22 bytes of zeroes).
//! 2. The `ChainType` which is a 2 byte value that identifies the chain type
//! of the target system. It can be one of the following:
//!    - A `ChainType::Evm` which is `0x0100`.
//!    - A `ChainType::Substrate` which is `0x0200`.
//!    - A `ChainType::PolkadotRelayChain` which is `0x0301`.
//!    - A `ChainType::KusamaRelayChain` which is `0x0302`.
//!    - A `ChainType::Cosmos` which is `0x0400`.
//!    - A `ChainType::Solana` which is `0x0500`.
//! 3. The `ChainId` which is a 4 byte value that identifies the chain of the
//! target system.
//!
//! ## The `FunctionSignature`
//!
//! The `FunctionSignature` is the next 4 bytes after the [`ResourceId`], and it
//! is used to identify the function to be executed on the target system.
//!
//! ## The `Nonce`
//!
//! The `Nonce` is the next 4 bytes after the [`FunctionSignature`], and it is
//! used to prevent replay attacks.
//!
//! ## The `Body`
//!
//! The `Body` is the rest of the proposal bytes, and the length could vary
//! depending on what the purpose of the proposal. See each proposal type for
//! the body structure.
//!
//! ## Proposals Implementations
//!
//! The following proposals are implemented for EVM-based chains:
//! - `AnchorUpdateProposal`
//! - `TokenAddProposal`
//! - `TokenRemoveProposal`
//! - `WrappingFeeUpdateProposal`
//! - `MinWithdrawalLimitProposal`
//! - `MaxDepositLimitProposal`
//! - `ResourceIdUpdateProposal`
//! - `SetTreasuryHandlerProposal`
//! - `SetVerifierProposal`
//! - `FeeRecipientUpdateProposal`
//! - `RescueTokensProposal`
//!
//! ## Feature Flags
//!
//! The following feature flags are used to enable or disable certain features:
//! - `scale`: Enables Implementation of the SCALE encoding and decoding
//!   (disabled by default).
//! - `std`: Enables the use of the standard library (enabled by default).
//! - `evm`: Enables EVM proposals (disabled by default).
//! - `substrate`: Enables Substrate proposals (disabled by default).

#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub extern crate alloc;

mod header;
mod nonce;
mod proposal;
mod target_system;
mod traits;

pub use header::*;
pub use nonce::*;
pub use proposal::*;
pub use target_system::*;
pub use traits::*;
