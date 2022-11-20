use crate::num_enum;
use crate::sections::Section;
use crate::types::ValueType;
use std::io::Cursor;

use crate::utils::JustRead;

num_enum! {SectionTypeId {
    Function = 0x60
}}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Type {
    Function {
        params: Vec<ValueType>,
        results: Vec<ValueType>,
    },
}

pub(crate) fn read_type_section(section_rest: Vec<u8>) -> Section {
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_types = section_cursor.leb_read();

    let mut types = vec![];
    for _ in 0..number_of_types {
        let which_type = section_cursor.leb_read();
        let number_of_parameters = section_cursor.leb_read();
        let mut function_parameter_types = vec![];
        for _ in 0..number_of_parameters {
            function_parameter_types.push(ValueType::from(section_cursor.leb_read()));
        }
        let number_of_results = section_cursor.leb_read();
        let mut result_types = vec![];
        for _ in 0..number_of_results {
            result_types.push(ValueType::from(section_cursor.leb_read()));
        }
        types.push(match SectionTypeId::from(which_type) {
            SectionTypeId::Function => Type::Function {
                params: function_parameter_types,
                results: result_types,
            },
        });
    }

    Section::Type { types }
}
