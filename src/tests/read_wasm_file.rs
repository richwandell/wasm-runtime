use crate::read_wasm_file;
use crate::sections::read_sections;

#[test]
#[cfg(feature = "print_in_tests")]
fn read_wasm_file() {
    read_wasm_file!(cursor, "wasm/code-br-table.wasm");

    let sections = read_sections(&mut cursor);
    println!("{:#?}", sections);
}
