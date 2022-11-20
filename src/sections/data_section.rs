use std::io::{Cursor};
use crate::instructions::{get_inst, Inst};
use crate::num_enum;
use crate::sections::Section;
use crate::utils::JustRead;

num_enum! {DataSegmentMode {
    ActiveWithExpression = 0x0,
    Passive = 0x1,
    ActiveSpecified = 0x2
}}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Data {
    ActiveExpression {
        expression: Vec<Inst>,
        data_string: Vec<u8>
    },
    Passive {
        data_string: Vec<u8>
    },
    ActiveMemoryOffset
}

pub(crate) fn read_data_section(section_rest: Vec<u8>) -> Section {
    let mut section_cursor = Cursor::new(&section_rest);
    let number_of_datas = section_cursor.leb_read();
    let mut segments = vec![];
    for _ in 0..number_of_datas {
        segments.push(match DataSegmentMode::from(section_cursor.just_read(1)[0]) {
            DataSegmentMode::ActiveWithExpression => {
                let mut expression = vec![];
                loop {
                    let instruction = get_inst(section_cursor.just_read(1)[0], &mut section_cursor);
                    match instruction {
                        Inst::End => {
                            expression.push(instruction);
                            break;
                        },
                        _ => {
                            expression.push(instruction);
                        }
                    }
                }
                let string_length = section_cursor.just_read(1)[0] as usize;
                Data::ActiveExpression {
                    expression,
                    data_string: section_cursor.just_read(string_length)
                }
            },
            DataSegmentMode::Passive => {
                let string_length = section_cursor.just_read(1)[0] as usize;
                Data::Passive {
                    data_string: section_cursor.just_read(string_length)
                }
            },
            DataSegmentMode::ActiveSpecified => {
                Data::ActiveMemoryOffset
            }
        });
    }
    Section::Data { segments }
}