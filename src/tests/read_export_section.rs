use crate::sections::export_section::{ExportDesc, ExportSection, read_export_section};
use crate::sections::Section;

#[test]
fn test_read_export_section() {
    let section_bytes = vec![0x1, 0x3, 0x61, 0x64, 0x64, 0x0, 0x0];
    let section = read_export_section(section_bytes);

    let expected = Section::Export {
        exports: vec![ExportSection {
            export_name: "add".to_string(),
            export_desc: ExportDesc::Function { id: 0 }
        }]
    };

    assert_eq!(section, expected);
}