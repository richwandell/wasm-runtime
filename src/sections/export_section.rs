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
    Function { index: usize },
    Table { index: usize },
    Memory { index: usize },
    Global { index: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Export {
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
        let index = section_cursor.leb_read() as usize;

        export_sections.push(Export {
            name: str::from_utf8(&export_name).unwrap().to_string(),
            desc: match ExportDescId::from(description_type) {
                ExportDescId::Function => ExportDesc::Function { index },
                ExportDescId::Table => ExportDesc::Table { index },
                ExportDescId::Memory => ExportDesc::Memory { index },
                ExportDescId::Global => ExportDesc::Global { index },
            },
        })
    }
    Section::Export {
        exports: export_sections,
    }
}
