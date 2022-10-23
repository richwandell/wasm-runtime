use std::io::{Cursor, Read};
use crate::sections::Section;

use crate::utils::JustRead;

pub(crate) fn read_import_section(section_rest: Vec<u8>) -> Section {
    use std::str;
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_imports = section_cursor.leb_read();
    for _ in 0..number_of_imports {
        let length_of_import_module_name = section_cursor.leb_read();
        let import_module_name = section_cursor.just_read(length_of_import_module_name as usize);

        let length_of_import_name_name = section_cursor.leb_read();
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
    Section::Import
}