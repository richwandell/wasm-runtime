use crate::sections::global_section::{Global, read_global_section};
use crate::sections::Section;


#[test]
fn test_global_i32_const() {
    use crate::instructions::Inst::{End, I32Const};
    use crate::types::ValueType::I32;

    let section_bytes = vec![
        0x1, // one global section
        0x7F, // i32
        0x0, // not mutable
        0x41, // i32.const
        0x1, // 1
        0xB, // end
    ];

    let section = read_global_section(section_bytes);

    let expected = Section::Global {
        globals: vec![
            Global {
                value_type: I32,
                mutable: false,
                expression: vec![
                    I32Const { n: 1 },
                    End
                ]
            }
        ]
    };


    assert_eq!(section, expected);
}

#[test]
fn test_global_mutable_i32_const() {
    use crate::instructions::Inst::{End, I32Const};
    use crate::types::ValueType::I32;
    let section_bytes = vec![
        0x2, // two globals
        0x7F, // i32
        0x0, // not mutable
        0x41, // i32.const
        0x1, // 1
        0xB, // end
        0x7F, // i32
        0x1, // mutable
        0x41, // i32.const
        0x1, // 1
        0xB, // end
    ];

    let section = read_global_section(section_bytes);

    let expected = Section::Global {
        globals: vec![
            Global {
                value_type: I32,
                mutable: false,
                expression: vec![
                    I32Const { n: 1 },
                    End
                ]
            },
            Global {
                value_type: I32,
                mutable: true,
                expression: vec![
                    I32Const { n: 1 },
                    End
                ]
            }
        ]
    };


    assert_eq!(section, expected);
}

