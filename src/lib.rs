#![deny(unsafe_code)]

#[cfg(feature = "evm-runtime")]
pub mod evm;
#[cfg(feature = "substrate-runtime")]
pub mod pallet;
#[cfg(feature = "substrate-runtime")]
pub mod substrate;
