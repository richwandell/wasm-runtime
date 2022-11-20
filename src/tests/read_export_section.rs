use crate::sections::export_section::ExportDesc::{Function, Global, Memory, Table};
use crate::sections::export_section::{read_export_section, Export};
use crate::sections::Section;

#[test]
fn test_single_function() {
    let section_bytes = vec![0x1, 0x3, 0x61, 0x64, 0x64, 0x0, 0x0];
    let section = read_export_section(section_bytes);

    let expected = Section::Export {
        exports: vec![Export {
            name: "add".to_string(),
            desc: Function { index: 0 },
        }],
    };

    assert_eq!(section, expected);
}

#[test]
fn test_two_functions() {
    let section_bytes = vec![
        0x2, // 2 exports
        0x3, 0x61, 0x64, 0x64, // add
        0x0, 0x0, // function id 0
        0x3, 0x73, 0x75, 0x62, // sub
        0x0, 0x1, // function with id 1
    ];

    let section = read_export_section(section_bytes);

    let expected = Section::Export {
        exports: vec![
            Export {
                name: "add".to_string(),
                desc: Function { index: 0 },
            },
            Export {
                name: "sub".to_string(),
                desc: Function { index: 1 },
            },
        ],
    };

    assert_eq!(expected, section);
}

#[test]
fn test_three_exports_with_memory() {
    let section_bytes = vec![
        0x3, // three exports
        0x3, 0x61, 0x64, 0x64, // add
        0x0, 0x0, // function with ID 0
        0x3, 0x73, 0x75, 0x62,
        0x0, 0x1, // function with id 1
        0x6, 0x6D, 0x65, 0x6D, 0x6F, 0x72, 0x79, // memory
        0x2, 0x0, // memory with id 0
    ];
    let section = read_export_section(section_bytes);

    let expected = Section::Export {
        exports: vec![
            Export {
                name: "add".to_string(),
                desc: Function { index: 0 },
            },
            Export {
                name: "sub".to_string(),
                desc: Function { index: 1 },
            },
            Export {
                name: "memory".to_string(),
                desc: Memory { index: 0 },
            },
        ],
    };

    assert_eq!(expected, section);
}

#[test]
fn test_global() {
    let section_bytes = vec![0x1, 0xA, 0x74, 0x68, 0x65, 0x5F, 0x67, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x3, 0x0];
    let section = read_export_section(section_bytes);

    let expected = Section::Export {
        exports: vec![
            Export {
                name: "the_global".to_string(),
                desc: Global { index: 0 }
            }
        ]
    };

    assert_eq!(expected, section);
}

#[test]
fn test_table() {
    let section_bytes = vec![0x1, 0x9, 0x74, 0x68, 0x65, 0x5F, 0x74, 0x61, 0x62, 0x6C, 0x65, 0x1, 0x0];
    let section = read_export_section(section_bytes);

    let expected = Section::Export {
        exports: vec![
            Export {
                name: "the_table".to_string(),
                desc: Table { index: 0 }
            }
        ]
    };

    assert_eq!(expected, section);
}
