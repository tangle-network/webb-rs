#![no_std]
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
    clippy::pedantic
)]
//! # Webb Protocol Proposals Specification
//!
//! A Reference Implementation of the Webb Protocol Proposals.
//!
//! ## Introduction
//!
//! Proposals are encoded in a simple binary format that describes the changes
//! happened to a state of a system (for example, a smart contract) and we need
//! to apply these changes to other connected systems.
//!
//! Each proposal is a sequence of bytes that fully describes the following:
//! 1. The Target System that we need to apply the proposal to.
//! 2. The Source System that is sending the proposal (_optinally_).
//! 3. Which function of the target system we need to call.
//! 4. The arguments of the function.
//!
//! The First 3 points described above are called the [`ProposalHeader`] and
//! they are (by definition) the first `40` bytes of the proposal. The remaining
//! bytes are the body and they are the rest of the proposal bytes and the
//! length could vary depending on what the proposal does.
//! Here is diagram of the proposal:
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
//! - The [`ResourceId`] which is 32 bytes that uniquely identifies the target
//!   system.
//! - The [`FunctionSignature`] which is 4 bytes that uniquely identifies the
//!   function of the target system.
//! - 3. The [`Nonce`] which is 4 bytes that is used to prevent replay attacks.
//!
//! ## The `ResourceId`
//!
//! The `ResourceId` is the first 32 bytes of the proposal header, and it
//! contains the following:
//! 1. The [`TargetSystem`] which is a the first 26 bytes of the `ResourceId`.
//! The [`TargetSystem`] could be one of the following (depending on the target
//! system):
//!    - A [`TargetSystem::ContractAddress`] which is actually a 20 bytes but
//!      will be left padded with zeros (in this case, 6 zeros).
//!    - A [`TargetSystem::TreeId`] which is actually a 4 bytes but will be left
//!      padded with zeros (in this case, 22 zeros).
//! 2. The [`ChainType`] which is a 2 bytes that identifies the chain type of
//! the target system. whcih could be one of the following:
//!    - A [`ChainType::Evm`] which is `0x0100`.
//!    - A [`ChainType::Substrate`] which is `0x0200`.
//!    - A [`ChainType::PolkadotRelayChain`] which is `0x0301`.
//!    - A [`ChainType::KusamaRelayChain`] which is `0x0302`.
//!    - A [`ChainType::Cosmos`] which is `0x0400`.
//!    - A [`ChainType::Solana`] which is `0x0500`.
//! 3. The [`ChainId`] which is a 4 bytes that identifies the chain of the
//! target system.
//!
//! ## The `FunctionSignature`
//!
//! The `FunctionSignature` is the next 4 bytes after the [`ResourceId`], and it
//! is used to identify the function of the target system.
//!
//! ## The `Nonce`
//!
//! The `Nonce` is the next 4 bytes after the [`FunctionSignature`], and it is
//! used to prevent replay attacks.
//!
//! ## The `Body`
//!
//! The `Body` is the rest of the proposal bytes, and the length could vary
//! depending on what the purpose of the proposal. See each proposal type and
//! the body structure.

mod header;
mod proposal;

pub use header::*;
pub use proposal::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
