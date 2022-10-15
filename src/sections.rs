use std::io::{Cursor, Read};
use crate::{leb_read, num_enum};
use crate::utils::JustRead;

num_enum! {Section {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12
}}

fn read_type_section(section_rest: Vec<u8>) {
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_types = section_cursor.leb_read();
    let which_type = section_cursor.leb_read();
    let number_of_parameters = section_cursor.leb_read();
    let mut function_parameter_types = vec![];
    for _ in 0..number_of_parameters {
        function_parameter_types.push(section_cursor.leb_read());
    }
    let number_of_results = section_cursor.leb_read();
    let mut result_types = vec![];
    for _ in 0..number_of_results {
        result_types.push(section_cursor.leb_read());
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

fn read_import_section(section_rest: Vec<u8>) {
    use std::str;
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_imports = leb_read!(section_cursor);
    for _ in 0..number_of_imports {
        let length_of_import_module_name = leb_read!(section_cursor);
        let import_module_name = section_cursor.just_read(length_of_import_module_name as usize);

        let length_of_import_name_name = leb_read!(section_cursor);
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

fn read_function_section(section_rest: Vec<u8>) {
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_functions = leb_read!(section_cursor);
    let mut function_indexes = vec![];
    for _ in 0..number_of_functions {
        function_indexes.push(leb_read!(section_cursor))
    }

    println!(
        "function section: {:X}, {:X?}",
        number_of_functions, function_indexes
    );
}

fn read_export_section(section_rest: Vec<u8>) {
    use std::str;
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_exports = leb_read!(section_cursor);
    for _ in 0..number_of_exports {
        let length_of_export_name = leb_read!(section_cursor);

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

fn read_code_section(section_rest: Vec<u8>) {
    let mut section_cursor = Cursor::new(&section_rest);
    let code_section_id = leb_read!(section_cursor);
    let code_section_size = leb_read!(section_cursor);
    let number_of_functions = leb_read!(section_cursor);

    if number_of_functions > 0 {
        for _ in 0..number_of_functions {
            let function_body_size = leb_read!(section_cursor);
            let number_of_local_declarations = leb_read!(section_cursor);
            println!("code section function: {:X}, {:X}", function_body_size, number_of_local_declarations);
        }
    } else {
        let mut code = vec![0; (code_section_size - 1) as usize];
        section_cursor.read_exact(&mut code).expect("");
        println!("code section: {:X}, {:X}, {:X}, {:X?}", code_section_id, code_section_size, number_of_functions, code);
    }
}


pub(crate) fn read_sections(cursor: &mut Cursor<&Vec<u8>>) {
    loop {
        let mut section_id = [0; 1];
        if let Err(_) = cursor.read_exact(&mut section_id) {
            break;
        }

        let length = cursor.leb_read();
        let section_rest = cursor.just_read(length as usize);

        match Section::from(section_id[0]) {
            Section::Type => read_type_section(section_rest),
            Section::Import => read_import_section(section_rest),
            Section::Function => read_function_section(section_rest),
            Section::Custom => println!("custom section: {:X?}", section_rest),
            Section::Export => read_export_section(section_rest),
            Section::Code => read_code_section(section_rest),
            Section::Memory => println!("code section: {:X?}", section_rest),
            _ => panic!("{}", section_id[0])
        }
    }
}