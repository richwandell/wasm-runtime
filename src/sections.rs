use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum Section {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12
}

impl TryFrom<u8> for Section {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x >= Section::Custom as u8 && x <= Section::DataCount as u8 =>
                Ok(unsafe { std::mem::transmute(x) }),
            _ => Err(()),
        }
    }
}