use std::io::{Cursor, Read};

use crate::num_enum;
use crate::sections::code_section::{CodeSection, read_code_section};
use crate::sections::export_section::{read_export_section, ExportSection};
use crate::sections::function_section::{FunctionSection, read_function_section};
use crate::sections::import_section::{read_import_section, ImportSection};
use crate::sections::type_section::{read_type_section, TypeSection};
use crate::utils::JustRead;

pub(crate) mod code_section;
pub(crate) mod export_section;
pub(crate) mod function_section;
pub(crate) mod import_section;
pub(crate) mod type_section;

num_enum! {SectionId {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12
}}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Section {
    Type { types: Vec<TypeSection> },
    Import { imports: Vec<ImportSection> },
    Function { functions: Vec<FunctionSection>},
    Custom,
    Export { exports: Vec<ExportSection> },
    Code { codes: Vec<CodeSection>},
    Memory,
    Table,
    NotImplemented,
}

pub(crate) fn read_sections(cursor: &mut Cursor<&Vec<u8>>) -> Vec<Section> {
    let mut sections = vec![];

    loop {
        let mut section_id = [0; 1];
        if let Err(_) = cursor.read_exact(&mut section_id) {
            break;
        }

        let length = cursor.leb_read();
        let section_rest = cursor.just_read(length as usize);

        sections.push(match SectionId::from(section_id[0]) {
            SectionId::Type => {
                #[cfg(feature = "print_in_tests")]
                println!("type section: {:X?}", section_rest);
                read_type_section(section_rest)
            }
            SectionId::Import => {
                #[cfg(feature = "print_in_tests")]
                println!("import section: {:X?}", section_rest);
                read_import_section(section_rest)
            }
            SectionId::Function => {
                #[cfg(feature = "print_in_tests")]
                println!("function section: {:X?}", section_rest);
                read_function_section(section_rest)
            }
            SectionId::Custom => {
                #[cfg(feature = "print_in_tests")]
                println!("custom section: {:X?}", section_rest);
                Section::Custom
            }
            SectionId::Export => {
                #[cfg(feature = "print_in_tests")]
                println!("export section: {:X?}", section_rest);
                read_export_section(section_rest)
            }
            SectionId::Code => {
                #[cfg(feature = "print_in_tests")]
                println!("code section: {:X?}", section_rest);
                read_code_section(section_rest)
            }
            SectionId::Memory => {
                #[cfg(feature = "print_in_tests")]
                println!("memory section: {:X?}", section_rest);
                Section::Memory
            }
            SectionId::Table => {
                #[cfg(feature = "print_in_tests")]
                println!("memory section: {:X?}", section_rest);
                Section::Table
            },
            _ => {
                #[cfg(feature = "print_in_tests")]
                println!("not implemented: {:X}, {:X?}", section_id[0], section_rest);
                Section::NotImplemented
            }
        })
    }
    return sections;
}
