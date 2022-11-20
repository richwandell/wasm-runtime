use std::io::{Cursor, Read};
use crate::instructions::{get_inst, Inst};

use crate::sections::Section;
use crate::utils::JustRead;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Code {
    pub(crate) instructions: Vec<Inst>,
}

pub(crate) fn read_code_section(section_rest: Vec<u8>) -> Section {
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_code_sections = section_cursor.leb_read();

    let mut code_sections = vec![];

    for _ in 0..number_of_code_sections {
        let code_section_size = section_cursor.leb_read();
        let mut instructions = vec![];
        let code_section_bytes = &section_cursor.just_read(code_section_size as usize);
        let mut code_section_cursor = Cursor::new(code_section_bytes);

        loop {
            let mut instruction_buffer = vec![0; 1];
            if let Err(_) = code_section_cursor.read_exact(&mut instruction_buffer) {
                break;
            }
            instructions.push(get_inst(instruction_buffer[0], &mut code_section_cursor));
        }
        code_sections.push(Code {
            instructions
        })
    }

    Section::Code {
        codes: code_sections
    }
}
