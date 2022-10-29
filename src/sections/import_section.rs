use std::io::{Cursor, Read};
use crate::num_enum;
use crate::sections::export_section::{ExportDesc, ExportDescId, ExportSection};
use crate::sections::Section;

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
pub(crate) struct  ImportSection {
    pub(crate) import_module_name: String,
    pub(crate) import_name: String,
    pub(crate) import_desc: ImportDesc
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
        section_cursor.read_exact(&mut import_name_name).expect("TODO: panic message");


        let mut description_type = 0x0;
        let mut description_id = 0x0;
        if number_of_imports > 1 {
            description_type = section_cursor.just_read(1)[0];
            description_id = section_cursor.leb_read() as u32;
        }

        import_sections.push(ImportSection {
            import_module_name: str::from_utf8(&import_module_name).unwrap().to_string(),
            import_name: str::from_utf8(&import_name_name).unwrap().to_string(),
            import_desc: match ImportDescId::from(description_type) {
                ImportDescId::Function => ImportDesc::Function {
                    id: description_id
                },
                ImportDescId::Table => ImportDesc::Table {
                    id: description_id
                },
                ImportDescId::Memory => ImportDesc::Memory {
                    id: description_id
                },
                ImportDescId::Global => ImportDesc::Global {
                    id: description_id
                }
            }
        });


        println!(
            "import section: {:X}, {:X}, {:X?}, {:X}, {:X?}, {:X?}, {}, {}, {}",
            number_of_imports,
            length_of_import_module_name,
            import_module_name,
            length_of_import_name_name,
            import_name_name,
            description_type,
            description_id,
            str::from_utf8(&import_module_name).unwrap(),
            str::from_utf8(&import_name_name).unwrap()
        );
    }
    Section::Import {
        imports: import_sections
    }
}