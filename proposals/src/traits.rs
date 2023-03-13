#[cfg(feature = "substrate")]
mod substrate {
    use crate::Proposal;
    use frame_support::pallet_prelude::Get;

    /// Trait to be used for handling signed proposals
    pub trait OnSignedProposal<E, MaxLength: Get<u32>> {
        /// On a signed proposal, this method is called.
        /// It returns a result `()` and otherwise an error of type `E`.
        ///
        /// ## Errors
        ///
        /// 1. If the proposal is not signed.
        /// 2. If the proposal is not valid.
        fn on_signed_proposal(proposal: Proposal<MaxLength>) -> Result<(), E>;
    }

    impl<E, MaxLength: Get<u32>> OnSignedProposal<E, MaxLength> for () {
        fn on_signed_proposal(_: Proposal<MaxLength>) -> Result<(), E> {
            Ok(())
        }
    }
}

#[cfg(feature = "substrate")]
pub use substrate::*;
