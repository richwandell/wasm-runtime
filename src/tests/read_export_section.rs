use crate::sections::export_section::ExportDesc::{Function, Memory};
use crate::sections::export_section::{read_export_section, ExportDesc, ExportSection};
use crate::sections::Section;
use crate::sections::Section::Export;

#[test]
fn test_single_function() {
    let section_bytes = vec![0x1, 0x3, 0x61, 0x64, 0x64, 0x0, 0x0];
    let section = read_export_section(section_bytes);

    let expected = Export {
        exports: vec![ExportSection {
            name: "add".to_string(),
            desc: Function { id: 0 },
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

    let expected = Export {
        exports: vec![
            ExportSection {
                name: "add".to_string(),
                desc: Function { id: 0 },
            },
            ExportSection {
                name: "sub".to_string(),
                desc: Function { id: 1 },
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

    let expected = Export {
        exports: vec![
            ExportSection {
                name: "add".to_string(),
                desc: Function { id: 0 },
            },
            ExportSection {
                name: "sub".to_string(),
                desc: Function { id: 1 },
            },
            ExportSection {
                name: "memory".to_string(),
                desc: Memory { id: 0 },
            },
        ],
    };

    assert_eq!(expected, section);
}
