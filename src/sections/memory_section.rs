use std::io::Cursor;
use crate::sections::Section;
use crate::types::{Bool};
use crate::types::Bool::{No, Yes};
use crate::utils::JustRead;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Memory {
    pub(crate) limits: Vec<u32>
}

pub(crate) fn read_memory_section(section_rest: Vec<u8>) -> Section {
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_memories = section_cursor.leb_read();
    let mut memories = vec![];
    for _ in 0..number_of_memories {
        memories.push(Memory {
            limits: match Bool::from(section_cursor.just_read(1)[0]) {
                No => vec![section_cursor.leb_read() as u32],
                Yes => vec![
                    section_cursor.leb_read() as u32,
                    section_cursor.leb_read() as u32,
                ],
            },
        });
    }
    Section::Memory { memories }
}