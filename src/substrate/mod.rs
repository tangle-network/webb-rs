#![allow(
    clippy::just_underscores_and_digits,
    clippy::type_complexity,
    clippy::module_inception
)]

pub mod dkg_runtime;

#[cfg(all(test, feature = "integration-tests"))]
mod tests {
    use super::*;
}
