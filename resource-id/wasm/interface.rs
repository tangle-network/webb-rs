use crate::compute_chain_info as compute_chain_info_rs;

#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug)]
pub struct ResourceId {
	#[wasm_bindgen]
    pub value: [u8; 32],
}

#[wasm_bindgen]
impl ResourceId {
	#[wasm_bindgen(js_name = computeChainInfo)]
	#[wasm_bindgen(getter)]
	pub fn compute_chain_info(&self, chain_id: u32, chain_type: [u8; 2]) -> JsValue {
        compute_chain_info_rs(chain_id, chain_type)
	}
}
