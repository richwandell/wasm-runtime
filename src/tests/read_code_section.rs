use crate::sections::code_section::{Code, read_code_section};
use crate::sections::Section;

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

    let expected = Section::Code {
        codes: vec![
            Code {
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

    let expected = Section::Code {
        codes: vec![
            Code {
                instructions: vec![
                    Unreachable,
                    LocalGet { x: 0 },
                    LocalGet { x: 1 },
                    I32Add,
                    End
                ]
            },
            Code {
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

#[test]
fn test_br_table() {
    use crate::instructions::Inst::{End, I32Add, LocalGet, Unreachable, F32Sub, I32Const, BrTable};

    let section_bytes = vec![
        0x1, // one code section
        0x7, // size of code section
        0x0, // trap
        0x41, // i32.const
        0x1, // 1
        0xE, // br_table
        0x0, // 0
        0x0, // 0
        0xB // end
    ];

    let section = read_code_section(section_bytes);

    println!("{:#?}", section);

    let expected = Section::Code {
        codes: vec![
            Code {
                instructions: vec![
                    Unreachable,
                    I32Const { n: 1 },
                    BrTable {
                        labels: vec![],
                        label_id: 0
                    },
                    End
                ]
            }
        ]
    };

    assert_eq!(section, expected);
}

#[test]
fn test_read_zero_page() {
    use crate::instructions::Inst::{Drop, Loop, LocalSet, I32LtS, End, I32Add, LocalGet, Unreachable, Block, I32Const, BrTable, BrIf, Br, I32Eq, If};
    use crate::instructions::BlockType::{Empty};

    let section_bytes = vec![
        0x3, // three code sections
        0x13, // size of code section
        0x0, // trap
        0x2, // block
        0x40, // block type empty
        0x20, // local.get
        0x0, // index
        0x41, // i32.const
        0xE4, // leb128 100
        0x0, // leb128 0
        0x46, // i32.eq
        0x4, // if
        0x40, // block type empty
        0xC, // br
        0x1, // label 1
        0xB, // end
        0x20, // local.get
        0x0, // index
        0x1A, 0xB, 0xB, 0x16, 0x0, 0x3, 0x40, 0x20, 0x0, 0x41, 0x1, 0x6A, 0x21, 0x0,
        0x20, 0x0, 0x1A, 0x20, 0x0, 0x41, 0xA, 0x48, 0xD, 0x0, 0xB, 0xB, 0x7, 0x0, 0x41, 0x1, 0xE,
        0x0, 0x0, 0xB
    ];

    let section = read_code_section(section_bytes);

    println!("{:#X?}", section);
    let expected = Section::Code {
        codes: vec![
            Code {
                instructions: vec![
                    Unreachable,
                    Block {
                        block_type: Empty,
                    },
                    LocalGet {
                        x: 0x0,
                    },
                    I32Const {
                        n: 0x64,
                    },
                    I32Eq,
                    If {
                        block_type: Empty,
                    },
                    Br {
                        l: 0x1,
                    },
                    End,
                    LocalGet {
                        x: 0x0,
                    },
                    Drop,
                    End,
                    End,
                ],
            },
            Code {
                instructions: vec![
                    Unreachable,
                    Loop {
                        block_type: Empty,
                    },
                    LocalGet {
                        x: 0x0,
                    },
                    I32Const {
                        n: 0x1,
                    },
                    I32Add,
                    LocalSet {
                        x: 0x0,
                    },
                    LocalGet {
                        x: 0x0,
                    },
                    Drop,
                    LocalGet {
                        x: 0x0,
                    },
                    I32Const {
                        n: 0xA,
                    },
                    I32LtS,
                    BrIf {
                        l: 0x0,
                    },
                    End,
                    End,
                ],
            },
            Code {
                instructions: vec![
                    Unreachable,
                    I32Const {
                        n: 0x1,
                    },
                    BrTable {
                        labels: vec![],
                        label_id: 0x0,
                    },
                    End,
                ],
            },
        ],
    };
    assert_eq!(section, expected);
}

