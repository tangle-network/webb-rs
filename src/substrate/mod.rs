#![allow(
    clippy::just_underscores_and_digits,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::module_inception,
    clippy::large_enum_variant
)]

pub mod dkg_runtime;
pub mod protocol_substrate_runtime;
pub mod tangle_runtime;

pub use scale;
pub use scale_info;
pub use subxt;
