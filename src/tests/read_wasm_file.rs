use crate::read_wasm_file;
use crate::sections::read_sections;

#[test]
fn read_wasm_file() {
    read_wasm_file!(cursor, "wasm/simple-import.wasm");

    read_sections(&mut cursor);
}
