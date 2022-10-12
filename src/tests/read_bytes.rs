use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use leb128;
use std::convert::TryFrom;
use std::fs::{read, File};
use std::io::Cursor;
use std::io::Read;
use std::str;
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
    cursor
        .read_exact(&mut version)
        .expect("TODO: panic message");

    println!(
        "magic number: {}, {}",
        format!("{:X?}", magic),
        str::from_utf8(&magic).unwrap()
    );
    let mut rdr = Cursor::new(&version);
    let v = rdr.read_u16::<LittleEndian>().unwrap();
    println!("version {}, {}", format!("{:X?}", version), v);

    loop {
        let mut section_id = [0; 1];
        match cursor.read_exact(&mut section_id) {
            Err(_) => {
                break
            }
            _ => {}
        }

        let length = leb128::read::unsigned(&mut cursor).expect("Should read number");

        let mut section_rest = vec![0; length as usize];
        cursor
            .read_exact(&mut section_rest)
            .expect("TODO: panic message");

        match Section::from(section_id[0]) {
            Section::Type => {
                let number_of_types = section_rest[0];
                let which_type = section_rest[1];
                let number_of_parameters = section_rest[2];
                let function_parameter_types =
                    &section_rest[3..(3 + number_of_parameters as usize)];
                let number_of_results = section_rest[3 + number_of_parameters as usize];
                let result_types = &section_rest[3 + number_of_parameters as usize + 1..];

                println!(
                    "type section: {:X}, {:X}, {:X}, {:X?}, {:X}, {:X?}",
                    number_of_types,
                    which_type,
                    number_of_parameters,
                    function_parameter_types,
                    number_of_results,
                    result_types
                );
            }
            Section::Function => {
                let number_of_functions = section_rest[0];
                let function_indexes = &section_rest[1..];

                println!(
                    "function section: {:X}, {:X?}",
                    number_of_functions, function_indexes
                );
            }
            Section::Custom => {
                println!("custom section: {:X?}", section_rest);
            }
            Section::Export => {
                let number_of_exports = section_rest[0];
                let length_of_export_name = section_rest[1];
                let export_name = &section_rest[2..(2 + length_of_export_name as usize)];
                println!(
                    "export section: {:X}, {:X}, {:X?}, {}",
                    number_of_exports,
                    length_of_export_name,
                    export_name,
                    str::from_utf8(&export_name).unwrap()
                );
            }
            Section::Code => {
                println!("code section: {:X?}", section_rest);
            }
            _ => {
                println!("{}", section_id[0]);
                panic!()
            }
        }
    }

    // for item in &[["magic", magic], ["version", version]] {
    //
    // }
}
