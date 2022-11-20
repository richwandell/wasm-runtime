use std::io::{Cursor, Read};

use crate::num_enum;
use crate::sections::Section;
use crate::utils::JustRead;

num_enum! {ExportDescId {
    Function = 0x00,
    Table = 0x01,
    Memory = 0x02,
    Global = 0x03
}}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ExportDesc {
    Function { id: u32 },
    Table { id: u32 },
    Memory { id: u32 },
    Global { id: u32 },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExportSection {
    pub(crate) name: String,
    pub(crate) desc: ExportDesc,
}

pub(crate) fn read_export_section(section_rest: Vec<u8>) -> Section {
    use std::str;
    let mut section_cursor = Cursor::new(&section_rest);

    let mut export_sections = vec![];

    let number_of_exports = section_cursor.leb_read();
    for _ in 0..number_of_exports {
        let length_of_export_name = section_cursor.leb_read();

        let mut export_name = vec![0; length_of_export_name as usize];
        section_cursor
            .read_exact(&mut export_name)
            .expect("TODO: panic message");

        let description_type = section_cursor.just_read(1)[0];
        let description_id = section_cursor.leb_read() as u32;

        export_sections.push(ExportSection {
            name: str::from_utf8(&export_name).unwrap().to_string(),
            desc: match ExportDescId::from(description_type) {
                ExportDescId::Function => ExportDesc::Function { id: description_id },
                ExportDescId::Table => ExportDesc::Table { id: description_id },
                ExportDescId::Memory => ExportDesc::Memory { id: description_id },
                ExportDescId::Global => ExportDesc::Global { id: description_id },
            },
        })
    }
    Section::Export {
        exports: export_sections,
    }
}
