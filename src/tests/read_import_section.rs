use crate::sections::import_section::{
    read_import_section, ImportDesc, Import,
};
use crate::sections::Section;
use crate::types::ValueType;

#[test]
fn test_read_single_function() {
    let section_bytes = vec![
        0x1, // one import
        0xD, 0x77, 0x61, 0x73, 0x69, 0x5F, 0x75, 0x6E, 0x73, 0x74, 0x61, 0x62, 0x6C,
        0x65, // wasi_unstable
        0x7, 0x66, 0x64, 0x5F, 0x72, 0x65, 0x61, 0x64, // fd_read
        0x0, 0x0, // function with id 0
    ];
    let section = read_import_section(section_bytes);

    let expected = Section::Import {
        imports: vec![Import {
            module_name: "wasi_unstable".to_string(),
            name: "fd_read".to_string(),
            desc: ImportDesc::Function { type_id: 0 },
        }],
    };

    assert_eq!(section, expected);
}

#[test]
fn test_read_two_functions_one_type() {
    let section_bytes = vec![
        0x2, // two imports
        0xD, 0x77, 0x61, 0x73, 0x69, 0x5F, 0x75, 0x6E, 0x73, 0x74, 0x61, 0x62, 0x6C,
        0x65, // wasi_unstable
        0x7, 0x66, 0x64, 0x5F, 0x72, 0x65, 0x61, 0x64, // fd_read
        0x0, 0x0, 0xD, 0x77, 0x61, 0x73, 0x69, 0x5F, 0x75, 0x6E, 0x73, 0x74, 0x61, 0x62, 0x6C,
        0x65, // wasi_unstable
        0x8, 0x66, 0x64, 0x5F, 0x77, 0x72, 0x69, 0x74, 0x65, // fd_write
        0x0, 0x0,
    ];
    let section = read_import_section(section_bytes);

    let expected = Section::Import {
        imports: vec![
            Import {
                module_name: "wasi_unstable".to_string(),
                name: "fd_read".to_string(),
                desc: ImportDesc::Function { type_id: 0 },
            },
            Import {
                module_name: "wasi_unstable".to_string(),
                name: "fd_write".to_string(),
                desc: ImportDesc::Function { type_id: 0 },
            },
        ],
    };

    assert_eq!(section, expected);
}

#[test]
fn test_read_two_functions_two_types() {
    let section_bytes = vec![
        0x2, 0x6, 0x66, 0x75, 0x6E, 0x63, 0x5F, 0x31, 0x3, 0x6F, 0x6E, 0x65, 0x0, 0x0, 0x6, 0x66,
        0x75, 0x6E, 0x63, 0x5F, 0x32, 0x3, 0x74, 0x77, 0x6F, 0x0, 0x1,
    ];
    let section = read_import_section(section_bytes);

    let expected = Section::Import {
        imports: vec![
            Import {
                module_name: "func_1".to_string(),
                name: "one".to_string(),
                desc: ImportDesc::Function { type_id: 0 },
            },
            Import {
                module_name: "func_2".to_string(),
                name: "two".to_string(),
                desc: ImportDesc::Function { type_id: 1 },
            },
        ],
    };

    assert_eq!(section, expected);
}

#[test]
fn test_read_table_import_and_memory() {
    let section_bytes = vec![
        0x2, // 2 imports
        0x2, 0x6A, 0x73, 0x3, 0x74, 0x62, 0x6C, 0x1, 0x70, 0x0, 0x2, // table
        0x2, 0x6A, 0x73, 0x3, 0x6D, 0x65, 0x6D, 0x2, 0x0, 0x1, // memory
    ];
    let section = read_import_section(section_bytes);

    let expected = Section::Import {
        imports: vec![
            Import {
                module_name: "js".to_string(),
                name: "tbl".to_string(),
                desc: ImportDesc::Table {
                    ref_type: ValueType::FuncRef,
                    limits: vec![2],
                },
            },
            Import {
                module_name: "js".to_string(),
                name: "mem".to_string(),
                desc: ImportDesc::Memory { limits: vec![1] },
            },
        ],
    };

    assert_eq!(section, expected);
}

#[test]
fn test_read_table_import_limit() {
    let section_bytes = vec![
        0x2, 0x2, 0x6A, 0x73, 0x3, 0x74, 0x62, 0x6C, 0x1, 0x70, 0x1, 0x2, 0x2, 0x2, 0x6A, 0x73,
        0x3, 0x6D, 0x65, 0x6D, 0x2, 0x1, 0x1, 0x2, // memory
    ];
    let section = read_import_section(section_bytes);

    let expected = Section::Import {
        imports: vec![
            Import {
                module_name: "js".to_string(),
                name: "tbl".to_string(),
                desc: ImportDesc::Table {
                    ref_type: ValueType::FuncRef,
                    limits: vec![2, 2],
                },
            },
            Import {
                module_name: "js".to_string(),
                name: "mem".to_string(),
                desc: ImportDesc::Memory { limits: vec![1, 2] },
            },
        ],
    };

    assert_eq!(section, expected);
}

#[test]
fn test_import_global() {
    let section_bytes = vec![
        0x1,
        0x5, 0x68, 0x65, 0x6C, 0x6C, 0x6F,
        0x5, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x3, 0x7F, 0x0];
    let section = read_import_section(section_bytes);

    let expected = Section::Import {
        imports: vec![
            Import {
                module_name: "hello".to_string(),
                name: "world".to_string(),
                desc: ImportDesc::Global {
                    val_type: ValueType::I32,
                    mutable: false
                }
            }
        ]
    };

    assert_eq!(section, expected);
}

#[test]
fn test_import_two_globals_one_mutable() {
    use ImportDesc::*;
    use ValueType::I32;
    let section_bytes = vec![
        0x2,
        0x5, 0x68, 0x65, 0x6C, 0x6C, 0x6F,
        0x5, 0x77, 0x6F, 0x72, 0x6C, 0x64,
        0x3, 0x7F, 0x0,
        0x6, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x31,
        0x6, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x31,
        0x3, 0x7F, 0x1
    ];
    let section = read_import_section(section_bytes);

    let expected = Section::Import {
        imports: vec![
            Import {
                module_name: "hello".to_string(),
                name: "world".to_string(),
                desc: Global {
                    val_type: I32,
                    mutable: false
                }
            },
            Import {
                module_name: "hello1".to_string(),
                name: "world1".to_string(),
                desc: Global {
                    val_type: I32,
                    mutable: true
                }
            }
        ]
    };

    assert_eq!(section, expected);
}
