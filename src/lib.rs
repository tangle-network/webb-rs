#![deny(
    trivial_casts,
    trivial_numeric_casts,
    stable_features,
    non_shorthand_field_patterns,
    renamed_and_removed_lints,
    unsafe_code,
    clippy::exhaustive_enums
)]

#[cfg(feature = "evm-runtime")]
pub mod evm;
#[cfg(feature = "substrate-runtime")]
pub mod substrate;
