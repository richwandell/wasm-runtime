use std::io::{Cursor, Read};
use crate::instructions::{get_inst, Inst};

use crate::sections::Section;
use crate::types::{Bool, ValueType};
use crate::utils::JustRead;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Global {
    pub(crate) value_type: ValueType,
    pub(crate) mutable: bool,
    pub(crate) expression: Vec<Inst>
}

pub(crate) fn read_global_section(section_rest: Vec<u8>) -> Section {
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_global_sections = section_cursor.leb_read();

    let mut globals = vec![];

    for _ in 0..number_of_global_sections {
        let value_type = ValueType::from(section_cursor.just_read(1)[0]);
        let mutable = match Bool::from(section_cursor.just_read(1)[0]) {
            Bool::Yes => true,
            Bool::No => false
        };
        let mut expression = vec![];
        loop {
            let instruction = get_inst(section_cursor.just_read(1)[0], &mut section_cursor);
            match instruction {
                Inst::End => {
                    expression.push(instruction);
                    break;
                },
                _ => {
                    expression.push(instruction);
                }
            }
        }
        globals.push(Global {
            value_type,
            mutable,
            expression
        })
    }

    Section::Global {
        globals
    }
}
