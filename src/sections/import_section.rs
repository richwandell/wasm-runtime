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

num_enum! {ImportRefType {
    FuncRef = 0x70,
    ExternRef = 0x6F
}}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ImportDesc {
    Function {
        id: u32
    },
    Table {
        ref_type: ImportRefType,
        limits: Vec<u32>
    },
    Memory {
        id: u32
    },
    Global {
        id: u32
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ImportSection {
    pub(crate) import_module_name: String,
    pub(crate) import_name: String,
    pub(crate) import_desc: ImportDesc,
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


        let mut description_type = section_cursor.just_read(1)[0];


        import_sections.push(ImportSection {
            import_module_name: str::from_utf8(&import_module_name).unwrap().to_string(),
            import_name: str::from_utf8(&import_name_name).unwrap().to_string(),
            import_desc: match ImportDescId::from(description_type) {
                ImportDescId::Function => ImportDesc::Function {
                    id:  section_cursor.leb_read() as u32
                },
                ImportDescId::Table => {
                    let ref_type = ImportRefType::from(section_cursor.just_read(1)[0]);
                    let limits = match section_cursor.just_read(1)[0] {
                        0x0 => {
                            vec![section_cursor.leb_read() as u32]
                        }
                        0x1 => {
                            vec![section_cursor.leb_read() as u32, section_cursor.leb_read() as u32]
                        }
                        _ => {
                            vec![]
                        }
                    };
                    ImportDesc::Table { ref_type, limits }
                },
                ImportDescId::Memory => ImportDesc::Memory {
                    id: section_cursor.leb_read() as u32
                },
                ImportDescId::Global => ImportDesc::Global {
                    id: section_cursor.leb_read() as u32
                }
            },
        });

        // println!(
        //     "import section: {:X}, {:X}, {:X?}, {:X}, {:X?}, {:X}, {:X}, {}, {}",
        //     number_of_imports,
        //     length_of_import_module_name,
        //     import_module_name,
        //     length_of_import_name_name,
        //     import_name_name,
        //     description_type,
        //     description_id,
        //     str::from_utf8(&import_module_name).unwrap(),
        //     str::from_utf8(&import_name_name).unwrap()
        // );
    }
    Section::Import {
        imports: import_sections
    }
}