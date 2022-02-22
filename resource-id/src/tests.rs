use crate::{compute_chain_id_with_type, derive_resource_id, parse_resource_id};


#[test]
fn derive_parse_resource_ids() {
    let tree_id = 0u32;
    let chain_id = 2000u32;

    let updated_chain_id_with_type: u64 = compute_chain_id_with_type(chain_id, [2, 0]);

    let resource_id = derive_resource_id(updated_chain_id_with_type, &tree_id.to_be_bytes());

    let (tree_id2, chain_id2): ([u8; 20], u64) = parse_resource_id(resource_id);

    assert_eq!(updated_chain_id_with_type as u64, chain_id2 as u64);

    assert_eq!(tree_id.to_be_bytes(), &tree_id2[16..]);
}
