use crate::read_wasm_file;
use byteorder::{LittleEndian, ReadBytesExt};

#[test]
fn read_magic_and_version() {
    read_wasm_file!(magic, version, cursor, "wasm/add.wasm");
    let mut rdr = Cursor::new(&version);
    let v = rdr.read_u16::<LittleEndian>().unwrap();
    assert_eq!(magic, [0x0, 0x61, 0x73, 0x6D]);
    assert_eq!(v, 1);
}
