use std::io::{Cursor, Read};

use crate::num_enum;
use crate::sections::Section;
use crate::utils::JustRead;

pub(crate) fn read_code_section(section_rest: Vec<u8>) -> Section {
    let mut section_cursor = Cursor::new(&section_rest);
    let code_section_id = section_cursor.leb_read();
    let code_section_size = section_cursor.leb_read();
    let number_of_functions = section_cursor.leb_read();

    if number_of_functions > 0 {
        for _ in 0..number_of_functions {
            let function_body_size = section_cursor.leb_read();
            let number_of_local_declarations = section_cursor.leb_read();
            println!("code section function: {:X}, {:X}", function_body_size, number_of_local_declarations);
        }
    } else {
        let mut code = vec![0; (code_section_size - 1) as usize];
        section_cursor.read_exact(&mut code).expect("");
        println!("code section: {:X}, {:X}, {:X}, {:X?}", code_section_id, code_section_size, number_of_functions, code);
    }
    Section::Code
}