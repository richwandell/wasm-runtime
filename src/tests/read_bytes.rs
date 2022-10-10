use std::convert::TryFrom;
use std::fs::{File, read};
use std::io::Cursor;
use std::io::Read;
use std::iter::FromIterator;
use std::str;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use leb128;
use crate::sections::Section;

#[test]
fn read_bytes() {

    let mut f = File::open("wasm/add.wasm").unwrap();

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("TODO: panic message");
    let mut cursor = Cursor::new(&buffer);

    let mut magic = [0; 4];
    cursor.read_exact(&mut magic).expect("TODO: panic message");
    let mut version = [0; 4];
    cursor.read_exact(&mut version).expect("TODO: panic message");

    println!("magic number: {}, {}", format!("{:X?}", magic), str::from_utf8(&magic).unwrap());
    let mut rdr = Cursor::new(&version);
    let v = rdr.read_u16::<LittleEndian>().unwrap();
    println!("version {}, {}", format!("{:X?}", version), v);

    let mut section_start = 8;
    loop {
        let mut section_id = [0; 1];
        cursor.read_exact(&mut section_id).expect("TODO: panic message");

        let length = leb128::read::signed(&mut cursor).expect("Should read number");
        let mut buf = Vec::new();
        cursor.read_to_end(&mut buf);

        println!("{}, {}, {:X?}", section_id[0], length, buf.to_vec());
        match Section::try_from(section_id[0]).unwrap() {
            Section::Type => {
                println!("it's a type");
            }
            Section::Custom => {}
            Section::Import => {}
            Section::Function => {}
            Section::Table => {}
            Section::Memory => {}
            Section::Global => {}
            Section::Export => {}
            Section::Start => {}
            Section::Element => {}
            Section::Code => {}
            Section::Data => {}
            Section::DataCount => {}
        }
        break;
    }



    // for item in &[["magic", magic], ["version", version]] {
    //
    // }

}