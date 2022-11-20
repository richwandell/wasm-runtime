use std::io::{Cursor, Read};
use Bool::{No, Yes};

use crate::num_enum;
use crate::sections::Section;
use crate::types::{ValueType, Bool};
use crate::utils::JustRead;

num_enum! {ImportDescId {
    Function = 0x00,
    Table = 0x01,
    Memory = 0x02,
    Global = 0x03
}}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ImportDesc {
    Function {
        type_id: u32,
    },
    Table {
        ref_type: ValueType,
        limits: Vec<u32>,
    },
    Memory {
        limits: Vec<u32>,
    },
    Global {
        val_type: ValueType,
        mutable: bool,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Import {
    pub(crate) module_name: String,
    pub(crate) name: String,
    pub(crate) desc: ImportDesc,
}

pub(crate) fn read_import_section(section_rest: Vec<u8>) -> Section {
    use std::str;
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_imports = section_cursor.leb_read();
    let mut import_sections = vec![];
    for _ in 0..number_of_imports {
        let length_of_import_module_name = section_cursor.leb_read();
        let import_module_name = section_cursor.just_read(length_of_import_module_name as usize);

        let length_of_import_name_name = section_cursor.leb_read();
        let mut import_name_name = vec![0; length_of_import_name_name as usize];
        section_cursor
            .read_exact(&mut import_name_name)
            .expect("TODO: panic message");
        let description_type = section_cursor.just_read(1)[0];

        import_sections.push(Import {
            module_name: str::from_utf8(&import_module_name).unwrap().to_string(),
            name: str::from_utf8(&import_name_name).unwrap().to_string(),
            desc: match ImportDescId::from(description_type) {
                ImportDescId::Function => ImportDesc::Function {
                    type_id: section_cursor.leb_read() as u32,
                },
                ImportDescId::Table => ImportDesc::Table {
                    ref_type: ValueType::from(section_cursor.just_read(1)[0]),
                    limits: match Bool::from(section_cursor.just_read(1)[0]) {
                        No => vec![section_cursor.leb_read() as u32],
                        Yes => vec![
                            section_cursor.leb_read() as u32,
                            section_cursor.leb_read() as u32,
                        ],
                    },
                },
                ImportDescId::Memory => ImportDesc::Memory {
                    limits: match Bool::from(section_cursor.just_read(1)[0]) {
                        No => vec![section_cursor.leb_read() as u32],
                        Yes => vec![
                            section_cursor.leb_read() as u32,
                            section_cursor.leb_read() as u32,
                        ]
                    }
                },
                ImportDescId::Global => ImportDesc::Global {
                    val_type: ValueType::from(section_cursor.just_read(1)[0]),
                    mutable: match Bool::from(section_cursor.just_read(1)[0]) {
                        No => false,
                        Yes => true
                    }
                }
            }
        });
    }
    Section::Import {
        imports: import_sections,
    }
}
