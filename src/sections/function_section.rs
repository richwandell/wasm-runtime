use std::io::Cursor;

use crate::num_enum;
use crate::sections::Section;
use crate::utils::JustRead;

pub(crate) fn read_function_section(section_rest: Vec<u8>) -> Section {
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_functions = section_cursor.leb_read();
    let mut function_indexes = vec![];
    for _ in 0..number_of_functions {
        function_indexes.push(section_cursor.leb_read())
    }
    Section::Function

    // println!(
    //     "function section: {:X}, {:X?}",
    //     number_of_functions, function_indexes
    // );
}