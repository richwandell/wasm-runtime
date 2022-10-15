use crate::{read_wasm_file};
use crate::sections::read_sections;

#[test]
fn add() {
    read_wasm_file!(cursor, "wasm/add.wasm");

    read_sections(&mut cursor);
}
