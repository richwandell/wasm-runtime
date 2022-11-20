use crate::sections::function_section::{FunctionSection, read_function_section};
use crate::sections::Section;

#[test]
fn test_two_functions() {
    use Section::Function;
    let section_bytes = vec![0x2, 0x0, 0x1];
    let section = read_function_section(section_bytes);

    let expected = Function {
        functions: vec![
            FunctionSection { type_index: 0 },
            FunctionSection { type_index: 1 }
        ]
    };

    assert_eq!(section, expected);
}