use std::io::{Cursor, Read};
use crate::instructions::Inst;

use crate::sections::Section;
use crate::utils::JustRead;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CodeSection {
    pub(crate) instructions: Vec<Inst>,
}

pub(crate) fn read_code_section(section_rest: Vec<u8>) -> Section {
    use Inst::*;
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_code_sections = section_cursor.leb_read();

    let mut code_sections = vec![];

    for _ in 0..number_of_code_sections {
        let code_section_size = section_cursor.leb_read();
        let mut instructions = vec![];
        let mut code_section_cursor = Cursor::new(section_cursor.just_read(code_section_size as usize));

        loop {
            let mut instruction_buffer = vec![0; 1];
            if let Err(_) = code_section_cursor.read_exact(&mut instruction_buffer) {
                break;
            }
            instructions.push(match instruction_buffer[0] {
                0x0 => Unreachable,
                0x01 => Nop,
                0x02 => Block,
                0x03 => Loop,
                0x04 => If,
                0x05 => Else,
                0x0b => End,
                0x20 => LocalGet {
                    x: code_section_cursor.just_read(1)[0] as usize
                },
                0x21 => LocalSet {
                    x: code_section_cursor.just_read(1)[0] as usize
                },
                0x22 => LocalTee {
                    x: code_section_cursor.just_read(1)[0] as usize
                },
                0x6A => I32Add,
                0x93 => F32Sub,
                _ => Nop
            });
        }
        code_sections.push(CodeSection {
            instructions
        })
    }

    Section::Code {
        codes: code_sections
    }
}
