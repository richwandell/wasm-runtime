use std::io::Cursor;
use Bool::{No, Yes};
use crate::sections::Section;
use crate::types::{ValueType, Bool};
use crate::utils::JustRead;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Table {
    pub(crate) ref_type: ValueType,
    pub(crate) limits: Vec<u32>
}

pub(crate) fn read_table_section(section_rest: Vec<u8>) -> Section {
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_tables = section_cursor.leb_read();
    let mut tables = vec![];
    for _ in 0..number_of_tables {
        tables.push(Table {
            ref_type: ValueType::from(section_cursor.just_read(1)[0]),
            limits: match Bool::from(section_cursor.just_read(1)[0]) {
                No => vec![section_cursor.leb_read() as u32],
                Yes => vec![
                    section_cursor.leb_read() as u32,
                    section_cursor.leb_read() as u32,
                ],
            },
        });
    }
    Section::Table { tables }
}