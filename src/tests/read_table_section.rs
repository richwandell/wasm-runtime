use crate::sections::Section;
use crate::sections::table_section::{read_table_section, Table};
use crate::types::ValueType;

#[test]
fn test_table() {
    let section_bytes = vec![
        0x1, // number of tables
        0x70, // ref type
        0x0, // has maximum
        0x2, // minimum for limit
    ];
    let section = read_table_section(section_bytes);

    let expected = Section::Table {
        tables: vec![
            Table {
                ref_type: ValueType::FuncRef,
                limits: vec![2],
            }
        ]
    };

    assert_eq!(expected, section);
}

#[test]
fn test_table_with_limit() {
    let section_bytes = vec![0x1, 0x70, 0x1, 0x2, 0x5];
    let section = read_table_section(section_bytes);

    let expected = Section::Table {
        tables: vec![
            Table {
                ref_type: ValueType::FuncRef,
                limits: vec![2, 5],
            }
        ]
    };

    assert_eq!(expected, section);
}