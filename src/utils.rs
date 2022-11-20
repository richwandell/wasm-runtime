use std::io::Cursor;
use std::io::Read;

#[macro_export]
macro_rules! leb_read {
    ($cursor:ident) => {{
        use leb128::read;
        read::unsigned(&mut $cursor).expect("Could not read leb128 number")
    }};
}

#[macro_export]
macro_rules! read_wasm_file {
    ($cursor:ident, $path:literal) => {
        use crate::utils::JustRead;
        use std::{fs::File, io::Cursor, io::Read};
        let mut file = File::open($path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("TODO: panic message");
        let mut $cursor = Cursor::new(&buffer);

        let _ = $cursor.just_read(4);
        let _ = $cursor.just_read(4);
    };

    ($magic:ident, $version:ident, $cursor:ident, $path:literal) => {
        use crate::utils::JustRead;
        use std::{fs::File, io::Cursor, io::Read};
        let mut file = File::open($path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("TODO: panic message");
        let mut $cursor = Cursor::new(&buffer);

        let $magic = $cursor.just_read(4);
        let $version = $cursor.just_read(4);
    };
}

pub(crate) trait JustRead {
    fn just_read(&mut self, len: usize) -> Vec<u8>;
    fn leb_read(&mut self) -> u64;
}

impl<T> JustRead for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn just_read(&mut self, len: usize) -> Vec<u8> {
        let mut buf = vec![0; len];
        if let Err(_) = self.read_exact(&mut buf) {
            panic!("Cannot read from cursor");
        }
        return buf;
    }

    fn leb_read(&mut self) -> u64 {
        {
            use leb128::read;
            return read::unsigned(self).expect("Could not read leb128 number");
        }
    }
}
