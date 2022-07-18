//! ### Proposals Implemented:
//! - [x] `AnchorUpdateProposal`
//! - [x] `TokenAddProposal`
//! - [x] `TokenRemoveProposal`
//! - [x] `WrappingFeeUpdateProposal`
//! - [x] `MinWithdrawalLimitProposal`
//! - [x] `MaxDepositLimitProposal`
//! - [x] `ResourceIdUpdateProposal`
//! - [x] `FeeRecipientUpdateProposal`
//! - [x] `SetTreasuryHandlerProposal`
//! - [x] `RescueTokensProposal`

mod anchor_update;
mod fee_recipient_update;
mod max_deposit_limit;
mod min_withdrawal_limit;
mod rescue_tokens;
mod resource_id_update;
mod set_treasury_handler;
mod token_add;
mod token_remove;
mod utils;
mod wrapping_fee_update;

pub use anchor_update::*;
pub use fee_recipient_update::*;
pub use max_deposit_limit::*;
pub use min_withdrawal_limit::*;
pub use rescue_tokens::*;
pub use resource_id_update::*;
pub use set_treasury_handler::*;
pub use token_add::*;
pub use token_remove::*;
pub use utils::*;
pub use wrapping_fee_update::*;
