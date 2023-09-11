use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn zkmain() -> i64 {
    let data = vec![0x83, b'c', b'a', b't'];
    let _animal: String = rlp::decode(&data).unwrap();
    // assert_eq!(animal, "cat".to_owned());

    0
}
