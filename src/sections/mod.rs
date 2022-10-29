use std::io::{Cursor, Read};

use crate::{num_enum};
use crate::sections::code_section::read_code_section;
use crate::sections::export_section::{ExportSection, read_export_section};
use crate::sections::function_section::read_function_section;
use crate::sections::import_section::{ImportSection, read_import_section};
use crate::sections::type_section::{read_type_section, TypeSection};
use crate::utils::JustRead;

pub(crate) mod type_section;
pub(crate) mod import_section;
pub(crate) mod function_section;
pub(crate) mod export_section;
pub(crate) mod code_section;

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
    Type {
        types: Vec<TypeSection>
    },
    Import {
        imports: Vec<ImportSection>
    },
    Function,
    Custom,
    Export {
        exports: Vec<ExportSection>
    },
    Code,
    Memory,
    NotImplemented,
}


pub(crate) fn read_sections(cursor: &mut Cursor<&Vec<u8>>) {
    let mut sections = vec![];

    loop {
        let mut section_id = [0; 1];
        if let Err(_) = cursor.read_exact(&mut section_id) {
            break;
        }

        let length = cursor.leb_read();
        let section_rest = cursor.just_read(length as usize);

        sections.push(match SectionId::from(section_id[0]) {
            SectionId::Type => read_type_section(section_rest),
            SectionId::Import => read_import_section(section_rest),
            SectionId::Function => read_function_section(section_rest),
            SectionId::Custom => Section::Custom,
            SectionId::Export => read_export_section(section_rest),
            SectionId::Code => read_code_section(section_rest),
            SectionId::Memory => Section::Memory,
            _ => Section::NotImplemented
        })
    }
}