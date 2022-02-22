#[cfg(test)]
mod tests;
mod chain_kind;
mod utils;

pub use chain_kind::*;
pub use utils::*;

pub type ResourceId = [u8; 32];
pub type TreeId = [u8; 20];
pub type ChainIdWithType = u64;
pub type ChainId = u32;
pub type ChainType = [u8; 2];
