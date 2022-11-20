use crate::instructions::Inst::I32Const;
use crate::sections::data_section::{Data, read_data_section};
use crate::sections::Section;

#[test]
fn test_single() {
    use crate::instructions::Inst::{I32Const, End};

    let section_bytes = vec![
        0x1, // 1 data segment
        0x0, // active memory mode 0 with expression
        0x41, // i32.const
        0x8, // 8
        0xB, // end expression
        0xB, // 11 chars
        0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64 // hello world
    ];

    let section = read_data_section(section_bytes);

    let expected = Section::Data {
        segments: vec![
            Data::ActiveExpression {
                expression: vec![
                    I32Const {
                        n: 8
                    },
                    End
                ],
                data_string: "hello world".as_bytes().to_vec()
            }
        ]
    };


    assert_eq!(section, expected);
}

#[test]
fn test_two() {
    use crate::instructions::Inst::{End};

    let section_bytes = vec![
        0x2,// two data segments
        0x0, // active memory mode with expression
        0x41, // i32.const
        0x8, // 8
        0xB, // end
        0xB, // 11 chars
        0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, // hello world
        0x0, // active memory mode with expression
        0x41, // i32.const
        0x13, // 13
        0xB, // end
        0x7, // 7 chars
        0x68, 0x69, 0x20, 0x72, 0x69, 0x63, 0x68 // hi rich
    ];

    let section = read_data_section(section_bytes);

    let expected = Section::Data {
        segments: vec![
            Data::ActiveExpression {
                expression: vec![
                    I32Const {
                        n: 8
                    },
                    End
                ],
                data_string: "hello world".as_bytes().to_vec()
            },
            Data::ActiveExpression {
                expression: vec![
                    I32Const {
                        n: 19
                    },
                    End
                ],
                data_string: "hi rich".as_bytes().to_vec()
            }
        ]
    };

    assert_eq!(section, expected);
}

#[test]
fn test_passive() {
    let section_bytes = vec![
        0x1, // one data
        0x1, // passive mode
        0x6, // six bytes
        0x74, 0x68, 0x69, 0x6E, 0x67, 0x73 // things
    ];

    let section = read_data_section(section_bytes);

    let expected = Section::Data {
        segments: vec![
            Data::Passive {
                data_string: "things".as_bytes().to_vec()
            }
        ]
    };

    assert_eq!(section, expected);
}

