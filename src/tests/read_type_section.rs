use crate::sections::Section;
use crate::sections::type_section::{read_type_section, TypeSection, ValueType};

#[test]
fn test_read_type_section() {
    let type_section_bytes = vec![0x1, 0x60, 0x2, 0x7F, 0x7F, 0x1, 0x7F];
    let type_section = read_type_section(type_section_bytes);
    let expected = Section::Type {
        types: vec![TypeSection::Function {
            params: vec![ValueType::I32, ValueType::I32],
            results: vec![ValueType::I32]
        }]
    };
    assert_eq!(type_section, expected);
}