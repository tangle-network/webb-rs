use crate::Proposal;

/// Trait to be used for handling signed proposals
pub trait OnSignedProposal<E> {
    /// On a signed proposal, this method is called.
    /// It returns a result `()` and otherwise an error of type `E`.
    ///
    /// ## Errors
    ///
    /// 1. If the proposal is not signed.
    /// 2. If the proposal is not valid.
    fn on_signed_proposal(proposal: Proposal) -> Result<(), E>;
}

impl<E> OnSignedProposal<E> for () {
    fn on_signed_proposal(_: Proposal) -> Result<(), E> {
        Ok(())
    }
}

