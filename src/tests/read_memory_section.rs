use crate::sections::memory_section::{Memory, read_memory_section};
use crate::sections::Section;

#[test]
fn test_single_with_limit() {

    let section_bytes = vec![0x1, 0x1, 0x2, 0x5];
    let section = read_memory_section(section_bytes);

    let expected = Section::Memory {
        memories: vec![
            Memory {
                limits: vec![2, 5],
            }
        ]
    };

    assert_eq!(expected, section);
}