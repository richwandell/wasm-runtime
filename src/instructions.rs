#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Inst {
    // 0x00
    Unreachable,
    // 0x01
    Nop,
    // 0x02
    Block,
    // 0x03
    Loop,
    // 0x04
    If,
    // 0x05
    Else,
    // 0x0b
    End,
    // 0x0c
    Br,
    // 0x0d
    BrIf,
    // 0x0e
    BrTable,
    // 0x0f
    Return,

    // 0x10
    Call { x: u8 },
    // 0x11
    CallIndirect { x: u8 },
    // 0x1a
    Drop,
    // 0x1b
    Select,

    // 0x20
    LocalGet { x: usize },
    // 0x21
    LocalSet { x: usize },
    // 0x22
    LocalTee { x: usize },
    // 0x23
    GlobalGet { x: usize },
    // 0x24
    GlobalSet { x: usize },
    // 0x28
    I32Load,
    // 0x29
    I64Load,
    // 0x2a
    F32Load,
    // 0x2b
    F64Load,
    // 0x2c
    I32Load8s,
    // 0x2d
    I23Load8u,

    // 0x38
    F32Store,
    // 0x39
    F64Store,
    // 0x34
    MemSize,
    // 0x40
    MemGrow,
    // 0x43
    F32Const,
    // 0x44
    F64Const,

    // 0x61
    F64Eq,
    // 0x62
    F64Ne,
    // 0x63
    F64Lt,
    // 0x64
    F64Gt,
    // 0x6A
    I32Add,

    // 0x93
    F32Sub
}
