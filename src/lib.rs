use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn zkmain() -> i64 {
    rlp()
}

pub fn rlp() -> i64 {
    let data = vec![0x83, b'c', b'a', b't'];
    let _animal: String = rlp::decode(&data).unwrap();
    // assert_eq!(animal, "cat".to_owned());
    0
}

#[cfg(test)]
mod tests {
    use crate::rlp;

    #[test]
    fn test_rlp() {
        // for ut test
        rlp();
    }
}
