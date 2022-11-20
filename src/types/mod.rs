use crate::num_enum;

num_enum! {ValueType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
    V128 = 0x7B,
    FuncRef = 0x70,
    ExternRef = 0x6F
}}

num_enum! {Bool {
    No = 0x0,
    Yes = 0x1
}}
