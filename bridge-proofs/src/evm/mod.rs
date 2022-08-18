use ethers::types::{EIP1186ProofResponse, H256};
use scale_codec::{Decode, Encode};
use webb_proposals::{Nonce, ResourceId, TargetSystem, TypedChainId};

trait BlockHeaderVerifier {
    type BlockHeaderHash;
    type Proof;
    type Proposal;
    type ProposalResult: Encode + Decode;

    fn verify_header(
        &self,
        chain_id: TypedChainId,
        header_hash: &Self::BlockHeaderHash,
        proof: &Self::Proof,
    ) -> Result<bool, String>;

    fn verify_header_and_return_results(
        &self,
        chain_id: TypedChainId,
        header_hash: &Self::BlockHeaderHash,
        proof: &Self::Proof,
    ) -> Result<Self::ProposalResult, String>;

    fn generate_unsigned_proposals(
        &self,
        header_hash: &Self::BlockHeaderHash,
        proof: &Self::Proof,
        resource_ids: &[ResourceId],
    ) -> Result<Vec<Self::Proposal>, String>;
}

#[derive(Encode, Decode)]
struct AnchorUpdateProposalResult {
    r_id: ResourceId,
    nonce: Nonce,
    merkle_root: [u8; 32],
}
struct EVMBlockHeaderVerifier;
impl BlockHeaderVerifier for EVMBlockHeaderVerifier {
    type BlockHeaderHash = H256;
    type Proof = EIP1186ProofResponse;
    type Proposal = webb_proposals::evm::AnchorUpdateProposal;
    type ProposalResult = AnchorUpdateProposalResult;

    fn verify_header(
        &self,
        chain_id: TypedChainId,
        header: &Self::BlockHeaderHash,
        proof: &Self::Proof,
    ) -> Result<bool, String> {
        Ok(true)
    }

    fn generate_unsigned_proposals(
        &self,
        header_hash: &Self::BlockHeaderHash,
        proof: &Self::Proof,
        resource_ids: &[ResourceId],
    ) -> Result<Vec<Self::Proposal>, String> {
        Ok(vec![])
    }

    fn verify_header_and_return_results(
        &self,
        chain_id: TypedChainId,
        header_hash: &Self::BlockHeaderHash,
        proof: &Self::Proof,
    ) -> Result<Self::ProposalResult, String> {
        let success = self.verify_header(chain_id, header_hash, proof)?;
        if success {
            let address = proof.address.as_bytes();
            let nonce = proof.storage.proof[0].value;
            let merkle_root = proof.storage.proof[1].value;
            return Ok(AnchorUpdateProposalResult {
                r_id: ResourceId::new(
                    TargetSystem::ContractAddress(address),
                    chain_id,
                ),
                nonce,
                merkle_root,
            });
        } else {
            Err("Failure".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use ethers::types::EIP1186ProofResponse;
    use serde::de::Deserialize;

    #[test]
    fn test_verify_and_generate_proposals() {
        let hardcoded_merkle_trie_proof = "....";
        let decoded_proof_response: EIP1186ProofResponse =
            EIP1186ProofResponse::deserialize(hardcoded_merkle_trie_proof)
                .unwrap();
    }
}
