use crate::sections::import_section::{ImportDesc, ImportSection, read_import_section};
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