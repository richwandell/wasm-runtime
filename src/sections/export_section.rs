use std::io::{Cursor, Read};

use crate::num_enum;
use crate::sections::Section;
use crate::utils::JustRead;

/*
// println!(
        //     "export section: {:X}, {:X}, {:X?}, {:X?}, {}",
        //     number_of_exports,
        //     length_of_export_name,
        //     export_name,
        //     description,
        //     str::from_utf8(&export_name).unwrap()
        // );
 */

num_enum! {ExportDescId {
    Function = 0x00,
    Table = 0x01,
    Memory = 0x02,
    Global = 0x03
}}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ExportDesc {
    Function {
        id: u32
    },
    Table {
        id: u32
    },
    Memory {
        id: u32
    },
    Global {
        id: u32
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct  ExportSection {
    pub(crate) export_name: String,
    pub(crate) export_desc: ExportDesc
}

pub(crate) fn read_export_section(section_rest: Vec<u8>) -> Section {
    use std::str;
    let mut section_cursor = Cursor::new(&section_rest);

    let mut export_sections = vec![];

    let number_of_exports = section_cursor.leb_read();
    for _ in 0..number_of_exports {
        let length_of_export_name = section_cursor.leb_read();

        let mut export_name = vec![0; length_of_export_name as usize];
        section_cursor.read_exact(&mut export_name).expect("TODO: panic message");

        let mut description_type = 0x0;
        let mut description_id = 0x0;
        if number_of_exports > 1 {
            description_type = section_cursor.just_read(1)[0];
            description_id = section_cursor.leb_read() as u32;
        }

        println!("export section: {:X}, {:X}, {:X?}, [{:X}, {}], {}",
            number_of_exports,
            length_of_export_name,
            export_name,
            description_type,
            description_id,
            str::from_utf8(&export_name).unwrap()
        );

        export_sections.push(ExportSection {
            export_name: str::from_utf8(&export_name).unwrap().to_string(),
            export_desc: match ExportDescId::from(description_type) {
                ExportDescId::Function => ExportDesc::Function {
                    id: description_id
                },
                ExportDescId::Table => ExportDesc::Table {
                    id: description_id
                },
                ExportDescId::Memory => ExportDesc::Memory {
                    id: description_id
                },
                ExportDescId::Global => ExportDesc::Global {
                    id: description_id
                }
            }
        })
    }
    Section::Export {
        exports: export_sections
    }
}