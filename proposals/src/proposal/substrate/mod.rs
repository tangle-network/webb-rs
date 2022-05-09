//! ### Proposals Implemented:
//! - [x] `AnchorUpdateProposal`
//! - [x] `TokenAddProposal`
//! - [x] `TokenRemoveProposal`
//! - [ ] `WrappingFeeUpdateProposal`

mod anchor_update;
mod token_add;
mod token_remove;

pub use anchor_update::*;
pub use token_add::*;
pub use token_remove::*;
