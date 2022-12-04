use std::io::Cursor;

use crate::sections::Section;
use crate::utils::JustRead;

pub(crate) fn read_start_section(section_rest: Vec<u8>) -> Section {
    let mut section_cursor = Cursor::new(&section_rest);
    Section::Start { function_index: section_cursor.leb_read() as usize }
}
