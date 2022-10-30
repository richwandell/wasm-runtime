use crate::sections::import_section::{ImportDesc, ImportRefType, ImportSection, read_import_section};
use crate::sections::Section;

#[test]
fn test_read_single_import_section() {
    let section_bytes = vec![
        0x1,// one import
        0xD, 0x77, 0x61, 0x73, 0x69, 0x5F, 0x75, 0x6E, 0x73, 0x74, 0x61, 0x62, 0x6C, 0x65, // wasi_unstable
        0x7, 0x66, 0x64, 0x5F, 0x72, 0x65, 0x61, 0x64, // fd_read
        0x0, 0x0, // function with id 0
    ];
    let section = read_import_section(section_bytes);

    let expected = Section::Import {
        imports: vec![
            ImportSection {
                import_module_name: "wasi_unstable".to_string(),
                import_name: "fd_read".to_string(),
                import_desc: ImportDesc::Function {
                    id: 0
                },
            }
        ]
    };

    assert_eq!(section, expected);
}

#[test]
fn test_read_table_import_and_memory_import() {
    let section_bytes = vec![
        0x2, // 2 imports
        0x2, 0x6A, 0x73, 0x3, 0x74, 0x62, 0x6C, 0x1, 0x70, 0x0, 0x2, // table
        0x2, 0x6A, 0x73, 0x3, 0x6D, 0x65, 0x6D, 0x2, 0x0, 0x1, // memory
    ];
    let section = read_import_section(section_bytes);

    let expected = Section::Import {
        imports: vec![
            ImportSection {
                import_module_name: "js".to_string(),
                import_name: "tbl".to_string(),
                import_desc: ImportDesc::Table {
                    ref_type: ImportRefType::FuncRef,
                    limits: vec![2],
                },
            },
            ImportSection {
                import_module_name: "js".to_string(),
                import_name: "mem".to_string(),
                import_desc: ImportDesc::Memory {
                    limits: vec![1]
                },
            },
        ]
    };


    assert_eq!(section, expected);
}

#[test]
fn test_read_table_import_limit_import() {
    let section_bytes = vec![
        0x2,
        0x2, 0x6A, 0x73, 0x3, 0x74, 0x62, 0x6C, 0x1, 0x70, 0x1, 0x2, 0x2,
        0x2, 0x6A, 0x73, 0x3, 0x6D, 0x65, 0x6D, 0x2, 0x1, 0x1, 0x2 // memory
    ];
    let section = read_import_section(section_bytes);

    let expected = Section::Import {
        imports: vec![
            ImportSection {
                import_module_name: "js".to_string(),
                import_name: "tbl".to_string(),
                import_desc: ImportDesc::Table {
                    ref_type: ImportRefType::FuncRef,
                    limits: vec![2, 2]
                }
            },
            ImportSection {
                import_module_name: "js".to_string(),
                import_name: "mem".to_string(),
                import_desc: ImportDesc::Memory {
                    limits: vec![1, 2]
                },
            },
        ]
    };

    assert_eq!(section, expected);
}