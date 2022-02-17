use crate::{compute_chain_info, derive_resource_id, parse_resource_id};


#[test]
fn derive_parse_resource_ids() {
    let tree_id = 0u32;
    let chain_id = 2000u32;

    let updated_chain_info: u64 = compute_chain_info(chain_id, [2, 0]);

    let resource_id = derive_resource_id(updated_chain_info, &tree_id.to_be_bytes());

    let (tree_id2, chain_id2): ([u8; 20], u64) = parse_resource_id(resource_id);

    assert_eq!(updated_chain_info as u64, chain_id2 as u64);

    assert_eq!(tree_id.to_be_bytes(), &tree_id2[16..]);
}
