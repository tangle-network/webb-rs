//! ### Proposals Implemented:
//! - [x] `AnchorUpdateProposal`
//! - [x] `TokenAddProposal`
//! - [x] `TokenRemoveProposal`
//! - [x] `WrappingFeeUpdateProposal`

mod anchor_update;
mod token_add;
mod token_remove;
mod wrapping_fee_update;

pub use anchor_update::*;
pub use token_add::*;
pub use token_remove::*;
pub use wrapping_fee_update::*;
