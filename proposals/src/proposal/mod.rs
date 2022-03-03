//! ### Proposals Implemented:
//! - [x] `AnchorUpdateProposal`
//! - [x] `TokenAddProposal`
//! - [x] `TokenRemoveProposal`
//! - [x] `WrappingFeeUpdateProposal`
//! - [x] `MinWithdrawalLimitProposal`
//! - [x] `MaxDepositLimitProposal`
//! - [x] `ResourceIdUpdateProposal`
//! - [x] `SetTreasuryHandlerProposal`
//! - [x] `SetVerifierProposal`
//! - [x] `FeeRecipientUpdateProposal`
//! - [ ] `RescueTokensProposal`

mod anchor_update;
mod fee_recipient_update;
mod max_deposit_limit;
mod min_withdrawal_limit;
mod resource_id_update;
mod set_treasury_handler;
mod set_verifier;
mod token_add;
mod token_remove;
mod wrapping_fee_update;

pub use anchor_update::*;
pub use fee_recipient_update::*;
pub use max_deposit_limit::*;
pub use min_withdrawal_limit::*;
pub use resource_id_update::*;
pub use set_treasury_handler::*;
pub use set_verifier::*;
pub use token_add::*;
pub use token_remove::*;
pub use wrapping_fee_update::*;
