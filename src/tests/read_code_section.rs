use crate::sections::code_section::{CodeSection, read_code_section};
use crate::sections::Section::Code;

#[test]
fn test_add() {
    use crate::instructions::Inst::{End, I32Add, LocalGet, Unreachable};

    let section_bytes = vec![
        0x1, // 1 code section
        0x7, // size of code section
        0x0, // trap
        0x20, // local.get
        0x0, // local index 0
        0x20, // local.get
        0x1, // local index 2
        0x6A, // i32.add
        0xB // end
    ];

    let section = read_code_section(section_bytes);

    let expected = Code {
        codes: vec![
            CodeSection {
                instructions: vec![
                    Unreachable,
                    LocalGet { x: 0 },
                    LocalGet { x: 1 },
                    I32Add,
                    End
                ]
            }
        ]
    };


    assert_eq!(section, expected);
}

#[test]
fn test_add_sub() {
    use crate::instructions::Inst::{End, I32Add, LocalGet, Unreachable, F32Sub};

    let section_bytes = vec![
        0x2, // two code sections
        0x7, // size of code section
        0x0, // trap
        0x20, // local get
        0x0, // index
        0x20, // local get
        0x1, // index
        0x6A, // i32.add
        0xB, // end
        0x7, // size of code section
        0x0, // trap
        0x20, // local get
        0x0, // index
        0x20, // local get
        0x1, // index
        0x93, // f32.sub
        0xB // end
    ];

    let section = read_code_section(section_bytes);

    let expected = Code {
        codes: vec![
            CodeSection {
                instructions: vec![
                    Unreachable,
                    LocalGet { x: 0 },
                    LocalGet { x: 1 },
                    I32Add,
                    End
                ]
            },
            CodeSection {
                instructions: vec![
                    Unreachable,
                    LocalGet { x: 0 },
                    LocalGet { x: 1 },
                    F32Sub,
                    End
                ]
            }
        ]
    };

    assert_eq!(section, expected);
}

