use crate::{ChainId, ChainInfo, ChainType, ResourceId, TreeId};

pub fn compute_chain_info(
    chain_id: ChainId,
    chain_type: ChainType,
) -> ChainInfo {
    let mut buf = [0u8; 8];

    buf[2..4].copy_from_slice(&chain_type);
    buf[4..].copy_from_slice(&chain_id.to_be_bytes());

    u64::from_be_bytes(buf)
}

pub fn derive_resource_id(chain_info: ChainInfo, tree_id: &[u8]) -> ResourceId {
    let mut r_id: ResourceId = [0; 32];

    // Last 6 bytes of chain id because chain[0] and chain[1] are 0.
    r_id[26..].copy_from_slice(&chain_info.to_be_bytes()[2..]);

    // Use at most 26 bytes
    let range = if tree_id.len() > 26 {
        26
    } else {
        tree_id.len()
    };
    for i in 0..range {
        // Ensure left padding for eth compatibility
        r_id[25 - i] = tree_id[range - i - 1];
    }

    r_id
}

pub fn parse_resource_id(resource_id: ResourceId) -> (TreeId, ChainInfo) {
    let mut tree_id = [0u8; 20];
    let mut chain_info = [0u8; 8];

    tree_id.copy_from_slice(&resource_id[6..26]);
    chain_info[2..8].copy_from_slice(&resource_id[26..]);

    (tree_id, u64::from_be_bytes(chain_info))
}
