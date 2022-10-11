use std::convert::TryFrom;
use std::fs::{File, read};
use std::io::{BufRead, Cursor};
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

    loop {
        let mut section_id = [0; 1];
        cursor.read_exact(&mut section_id).expect("TODO: panic message");
        let length = leb128::read::unsigned(&mut cursor).expect("Should read number");

        println!("{}, {}", section_id[0], length);
        match Section::from(section_id[0]) {
            Section::Type => {
                println!("it's a type section");
                /*
                Section type bytes:
                0 - number of types of variables defined within the function
                1 - which type (60 = function type)
                2 - Number of input parameters
                3 -> 3+@2 - Input parameter types
                3+@2 - Number of output parameters
                3+@2 -> @(3+@2) - Result types
                */
                let mut section_rest = vec![0; length as usize];
                cursor.read_exact(&mut section_rest).expect("TODO: panic message");
                println!("{:X?}", section_rest);

                let number_of_types = section_rest[0];
                let which_type = section_rest[1];
                let number_of_parameters = section_rest[2];
                let function_parameter_types = &section_rest[3..(3 + number_of_parameters as usize)];
                let number_of_results = section_rest[3 + number_of_parameters as usize];
                let result_types = &section_rest[3 + number_of_parameters as usize + 1..];


                println!("{:X}, {:X}, {:X}, {:X?}, {:X}, {:X?}",
                         number_of_types,
                         which_type,
                         number_of_parameters,
                         function_parameter_types,
                         number_of_results,
                         result_types
                );
            }
            _ => panic!()
        }
        break;
    }


    // for item in &[["magic", magic], ["version", version]] {
    //
    // }
}