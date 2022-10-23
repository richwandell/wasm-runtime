use crate::sections::export_section::{ExportDesc, ExportSection, read_export_section};
use crate::sections::export_section::ExportDesc::{Function, Memory};
use crate::sections::Section;
use crate::sections::Section::Export;

#[test]
fn test_read_single_export_section() {
    let section_bytes = vec![0x1, 0x3, 0x61, 0x64, 0x64, 0x0, 0x0];
    let section = read_export_section(section_bytes);

    let expected = Section::Export {
        exports: vec![ExportSection {
            export_name: "add".to_string(),
            export_desc: ExportDesc::Function { id: 0 },
        }]
    };

    assert_eq!(section, expected);
}

#[test]
fn test_read_two_function_exports() {
    let section_bytes = vec![
        0x2, // 2 exports
        0x3, 0x61, 0x64, 0x64, 0x0, 0x0, // add
        0x3, 0x73, 0x75, 0x62, 0x0, 0x1, // sub
    ];

    let section = read_export_section(section_bytes);

    let expected = Export {
        exports: vec![
            ExportSection {
                export_name: "add".to_string(),
                export_desc: Function {
                    id: 0
                },
            },
            ExportSection {
                export_name: "sub".to_string(),
                export_desc: Function {
                    id: 1
                }
            }
        ]
    };

    assert_eq!(expected, section);
}

#[test]
fn test_read_three_exports_with_memory() {
    let section_bytes = vec![
        0x3, // three exports
        0x3, 0x61, 0x64, 0x64, 0x0, 0x0,
        0x3, 0x73, 0x75, 0x62, 0x0, 0x1,
        0x6, 0x6D, 0x65, 0x6D, 0x6F, 0x72, 0x79, 0x2, 0x0,
    ];
    let section = read_export_section(section_bytes);

    let expected = Export {
        exports: vec![
            ExportSection {
                export_name: "add".to_string(),
                export_desc: Function {
                    id: 0
                }
            },
            ExportSection {
                export_name: "sub".to_string(),
                export_desc: Function {
                    id: 1
                }
            },
            ExportSection {
                export_name: "memory".to_string(),
                export_desc: Memory {
                    id: 0
                }
            }
        ]
    };

    assert_eq!(expected, section);
}