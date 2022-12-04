use std::io::Cursor;

use crate::instructions::Inst::{Drop, Block, Else, End, F32Sub, I32Add, I32Const, I32Store, If, LocalGet, LocalSet, LocalTee, Loop, MemGrow, Nop, Unreachable, Call};
use crate::utils::JustRead;

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
    // 0x1c
    SelectT { t: u8 },

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
    // 0x25
    TableGet { x: usize },
    // 0x26
    TableSet { x: usize },
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
    // 0x2e
    I32Load16s,
    // 0x2f
    I32Load16u,

    // 0x30
    I64Load8s,
    // 0x31
    I64Load8u,
    // 0x32
    I64Load16s,
    // 0x33
    I64Load16u,
    // 0x34
    I64Load32s,
    // 0x35
    I64Load32u,
    //0x36
    I32Store {
        offset: u32,
        align: u32,
    },
    // 0x37
    I64Store {
        offset: u32,
        align: u32
    },
    // 0x38
    F32Store,
    // 0x39
    F64Store,
    // 0x3A
    I32Store8,
    // 0x3B
    I32Store16,
    // 0x3C
    I64Store8,
    // 0x3D
    I64Store16,
    // 0x3F
    MemSize,
    // 0x40
    MemGrow,
    // 0x41
    I32Const { n: i32 },
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
    F32Sub,
}

pub(crate) fn get_inst(inst: u8, cursor: &mut Cursor<&Vec<u8>>) -> Inst {
    match inst {
        0x0 => Unreachable,
        0x01 => Nop,
        0x02 => Block,
        0x03 => Loop,
        0x04 => If,
        0x05 => Else,
        0x0b => End,
        0x10 => Call {
            x: cursor.just_read(1)[0]
        },
        0x1a => Drop,
        0x20 => LocalGet {
            x: cursor.just_read(1)[0] as usize
        },
        0x21 => LocalSet {
            x: cursor.just_read(1)[0] as usize
        },
        0x22 => LocalTee {
            x: cursor.just_read(1)[0] as usize
        },
        0x36 => I32Store {
            offset: cursor.leb_read() as u32,
            align: cursor.leb_read() as u32,
        },
        0x40 => MemGrow,
        0x41 => I32Const {
            n: cursor.leb_read() as i32
        },
        0x6A => I32Add,
        0x93 => F32Sub,
        _ => {
            println!("unimplemented instruction {:X}", inst);
            Nop
        }
    }
}