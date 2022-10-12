use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use leb128;
use std::convert::TryFrom;
use std::fs::{read, File};
use std::io::Cursor;
use std::io::Read;
use std::str;
use leb128::read;
use crate::sections::Section;


#[test]
fn read_bytes() {
    let mut f = File::open("wasm/simple-import.wasm").unwrap();

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
        if let Err(_) = cursor.read_exact(&mut section_id) {
            break;
        }

        let length = read::unsigned(&mut cursor).expect("Should read number");

        let mut section_rest = vec![0; length as usize];
        cursor
            .read_exact(&mut section_rest)
            .expect("TODO: panic message");

        match Section::from(section_id[0]) {
            Section::Type => {
                let mut section_cursor = Cursor::new(&section_rest);
                let number_of_types = read::unsigned(&mut section_cursor).expect("Should read number");
                let which_type = read::unsigned(&mut section_cursor).expect("Should read number");
                let number_of_parameters = read::unsigned(&mut section_cursor).expect("Should read number");
                let mut function_parameter_types = vec![];
                for _ in 0..number_of_parameters {
                    function_parameter_types.push(
                        read::unsigned(&mut section_cursor).expect("Should read number")
                    );
                }
                let number_of_results = read::unsigned(&mut section_cursor).expect("Should read number");
                let mut result_types = vec![];
                for _ in 0..number_of_results {
                    result_types.push(
                        read::unsigned(&mut section_cursor).expect("Should read number")
                    );
                }

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
            Section::Import => {
                let mut section_cursor = Cursor::new(&section_rest);
                let number_of_imports = read::unsigned(&mut section_cursor).expect("Should read number");
                for _ in 0..number_of_imports {
                    let length_of_import_module_name = read::unsigned(&mut section_cursor).expect("Should read number");
                    let mut import_module_name = vec![0; length_of_import_module_name as usize];
                    section_cursor.read_exact(&mut import_module_name).expect("TODO: panic message");

                    let length_of_import_name_name = read::unsigned(&mut section_cursor).expect("Should read number");
                    let mut import_name_name = vec![0; length_of_import_name_name as usize];
                    section_cursor.read_exact(&mut import_name_name).expect("TODO: panic message");

                    let mut description: [u8; 2] = [0; 2];
                    if number_of_imports > 1 {
                        section_cursor.read_exact(&mut description).expect("TODO: panic message");
                    }
                    println!(
                        "import section: {:X}, {:X}, {:X?}, {:X}, {:X?}, {:X?}, {}, {}",
                        number_of_imports,
                        length_of_import_module_name,
                        import_module_name,
                        length_of_import_name_name,
                        import_name_name,
                        description,
                        str::from_utf8(&import_module_name).unwrap(),
                        str::from_utf8(&import_name_name).unwrap()
                    );
                }
            }
            Section::Function => {
                let mut section_cursor = Cursor::new(&section_rest);
                let number_of_functions = read::unsigned(&mut section_cursor).expect("Should read number");
                let mut function_indexes = vec![];
                for _ in 0..number_of_functions {
                    function_indexes.push(
                        read::unsigned(&mut section_cursor).expect("Should read number")
                    )
                }

                println!(
                    "function section: {:X}, {:X?}",
                    number_of_functions, function_indexes
                );
            }
            Section::Custom => {
                println!("custom section: {:X?}", section_rest);
            }
            Section::Export => {
                let mut section_cursor = Cursor::new(&section_rest);
                let number_of_exports = read::unsigned(&mut section_cursor).expect("Should read number");
                for _ in 0..number_of_exports {
                    let length_of_export_name = read::unsigned(&mut section_cursor).expect("Should read number");

                    let mut export_name = vec![0; length_of_export_name as usize];
                    section_cursor.read_exact(&mut export_name).expect("TODO: panic message");

                    let mut description: [u8; 2] = [0; 2];
                    if number_of_exports > 1 {
                        section_cursor.read_exact(&mut description).expect("TODO: panic message");
                    }
                    println!(
                        "export section: {:X}, {:X}, {:X?}, {:X?}, {}",
                        number_of_exports,
                        length_of_export_name,
                        export_name,
                        description,
                        str::from_utf8(&export_name).unwrap()
                    );
                }
            }
            Section::Code => {
                println!("code section: {:X?}", section_rest);
            }
            Section::Memory => {
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
