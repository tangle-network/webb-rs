//! ### Proposals Implemented:
//! - [x] `AnchorUpdateProposal`
//! - [x] `TokenAddProposal`
//! - [x] `TokenRemoveProposal`
//! - [x] `WrappingFeeUpdateProposal`
//! - [x] `ResourceIdUpdateProposal`
//! - [x] `FeeRecipientUpdateProposal`
//! - [x] `MaxDepositLimitProposal`
//! - [x] `MinWithdrawalLimitProposal`
//! - [x] `RescueTokensProposal`

mod anchor_update;
mod fee_recipient_update;
mod fungible_asset_add_proposal;
mod max_deposit_limit;
mod min_withdrawal_limit;
mod rescue_tokens;
mod resource_id_update;
mod token_add;
mod token_remove;
mod wrapping_fee_update;

pub use anchor_update::*;
pub use fee_recipient_update::*;
pub use fungible_asset_add_proposal::*;
pub use max_deposit_limit::*;
pub use min_withdrawal_limit::*;
pub use rescue_tokens::*;
pub use resource_id_update::*;
pub use token_add::*;
pub use token_remove::*;
pub use wrapping_fee_update::*;
