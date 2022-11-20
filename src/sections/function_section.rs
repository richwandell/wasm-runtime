use std::io::Cursor;

use crate::sections::Section;
use crate::utils::JustRead;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FunctionSection {
    pub(crate) type_index: usize,
}

pub(crate) fn read_function_section(section_rest: Vec<u8>) -> Section {
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_functions = section_cursor.leb_read();
    let mut functions = vec![];
    for _ in 0..number_of_functions {
        functions.push(FunctionSection {
            type_index: section_cursor.leb_read() as usize
        })
    }
    Section::Function { functions }
}
