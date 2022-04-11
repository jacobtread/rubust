use std::io::{Cursor, Read};

use crate::class::constant::PoolIndex;
use crate::error::DecompileError;
use crate::io::{Readable, ReadResult};

#[derive(Debug, Clone,Copy)]
pub enum ArrayType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
}

pub type Index = u16;
pub type BranchIndex = u16;

#[derive(Debug, Clone)]
pub enum Instr {
    SALoad, //
    TableSwitch {
        default: u32,
        low: u32,
        high: u32,
        offsets: Vec<u32>,
    },
    Swap, //
    SAStore, //
    BIPush(i8), //
    SIPush(i16), //
    NewArray(ArrayType), //
    Pop2,//
    IConst(i32),//
    FConst(f32),//
    DConst(f64),//
    LConst(i64),//
    IAdd,//
    FAdd,//
    InvokeSpecial(PoolIndex), //
    InvokeStatic(PoolIndex), //
    InvokeVirtual(PoolIndex), //
    InvokeInterface(PoolIndex),
    PutField(PoolIndex), //
    GetField(PoolIndex), //
    PutStatic(PoolIndex), //
    Return,//
    Dup, //
    DupX1,//
    DupX2,//
    Dup2,//
    Dup2X1,//
    Dup2X2,//
    Pop, //
    DAdd,//
    DDiv,//
    D2i,//
    D2f,//
    D2l,//
    AReturn, //
    CheckCast(PoolIndex), //
    F2i, //
    AConstNull, //
    LoadConst(PoolIndex), //
    DCmpL,
    DCmpG,
    ArrayLength, //
    AThrow,
    DALoad,//
    CALoad,//
    BALoad,//
    AALoad,//
    FALoad,//
    DAStore,//
    CAStore,//
    BAStore,//
    AAStore,//
    FAStore, //
    ANewArray(PoolIndex), //
    DMul, //
    DNeg,//
    DRem,
    DReturn, //
    FSub, //
    FMul, //
    FNeg,//
    FRem,
    FReturn,//
    FCmpL,//
    FCmpG,//
    DSub, //
    FDiv, //
    GetStatic(PoolIndex),//
    F2l,//
    F2d,//
    I2l,//
    I2d,//
    I2s,//
    I2c,//
    I2b,//
    I2f, //
    IALoad, //
    IAStore,
    IMul,//
    IDiv,//
    IAnd, //
    INeg,//
    InstanceOf(PoolIndex), //
    InvokeDynamic(PoolIndex),
    L2i,//
    L2d,//
    L2f,//
    LALoad,//
    LAStore,
    LAdd,//
    LAnd, //
    LOr,//
    LXOr,//
    LSub,//
    LMul,//
    LDiv,//
    ISub,//
    IRem,
    LNeg,//
    IShL,//
    IShR, //
    IUShR, //
    IOr,//
    IXOr,//
    LCmp, //
    IReturn,//
    LReturn, //
    LRem,
    LShL, //
    LShR, //
    LUShR, //
    LookupSwitch {
        default: u32,
        pairs: Vec<(i32, u32)>,
    },
    Nop, //
    MonitorEnter,//
    MonitorExit,//
    MultiANewArray {//
        index: u16,
        dimensions: u8,
    },
    New(PoolIndex), //
    Ret(Index),
    AStore(Index),//
    LStore(Index),//
    IStore(Index),//
    DStore(Index),//
    FStore(Index),//
    FLoad(Index),//
    ILoad(Index),//
    ALoad(Index),//
    DLoad(Index),//
    LLoad(Index),//
    IInc { index: u16, value: i16, }, //
    IfACmpEq(BranchIndex),
    IfACmpNe(BranchIndex),
    IfICmpEq(BranchIndex),
    IfICmpNe(BranchIndex),
    IfICmpLt(BranchIndex),
    IfICmpGe(BranchIndex),
    IfICmpGt(BranchIndex),
    IfICmpLe(BranchIndex),
    IfNull(BranchIndex),
    IfNonNull(BranchIndex),
    IfEq(BranchIndex),
    IfNe(BranchIndex),
    IfLt(BranchIndex),
    IfGe(BranchIndex),
    IfGt(BranchIndex),
    IfLe(BranchIndex),
    Goto(BranchIndex),
    JSr(BranchIndex),
}

impl Instr {
    fn read_instr(i: &mut Cursor<Vec<u8>>, wide: bool, pos: i32) -> Result<Self, DecompileError> where Self: Sized {
        let code = u8::read(i)?;
        Ok(match code {
            0x0 => Instr::Nop,
            0x1 => Instr::AConstNull,
            0x2 => Instr::IConst(-1),
            0x3 => Instr::IConst(0),
            0x4 => Instr::IConst(1),
            0x5 => Instr::IConst(2),
            0x6 => Instr::IConst(3),
            0x7 => Instr::IConst(4),
            0x8 => Instr::IConst(5),
            0x9 => Instr::LConst(0),
            0xa => Instr::LConst(1),
            0xb => Instr::FConst(0.0),
            0xc => Instr::FConst(1.0),
            0xd => Instr::FConst(2.0),
            0xe => Instr::DConst(0.0),
            0xf => Instr::DConst(1.0),
            0x10 => Instr::BIPush(u8::read(i)? as i8),
            0x11 => Instr::SIPush(u16::read(i)? as i16),
            0x12 => Instr::LoadConst(u8::read(i)? as u16),
            0x13 | 0x14 => Instr::LoadConst(u16::read(i)?),
            0x15 => Instr::ILoad(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0x16 => Instr::LLoad(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0x17 => Instr::FLoad(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0x18 => Instr::DLoad(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0x19 => Instr::ALoad(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0x1a => Instr::ILoad(0),
            0x1b => Instr::ILoad(1),
            0x1c => Instr::ILoad(2),
            0x1d => Instr::ILoad(3),
            0x1e => Instr::LLoad(0),
            0x1f => Instr::LLoad(1),
            0x20 => Instr::LLoad(2),
            0x21 => Instr::LLoad(3),
            0x22 => Instr::FLoad(0),
            0x23 => Instr::FLoad(1),
            0x24 => Instr::FLoad(2),
            0x25 => Instr::FLoad(3),
            0x26 => Instr::DLoad(0),
            0x27 => Instr::DLoad(1),
            0x28 => Instr::DLoad(2),
            0x29 => Instr::DLoad(3),
            0x2a => Instr::ALoad(0),
            0x2b => Instr::ALoad(1),
            0x2c => Instr::ALoad(2),
            0x2d => Instr::ALoad(3),
            0x2e => Instr::IALoad,
            0x2f => Instr::LALoad,
            0x30 => Instr::FALoad,
            0x31 => Instr::DALoad,
            0x32 => Instr::AALoad,
            0x33 => Instr::BALoad,
            0x34 => Instr::CALoad,
            0x35 => Instr::SALoad,
            0x36 => Instr::IStore(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0x37 => Instr::LStore(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0x38 => Instr::FStore(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0x39 => Instr::DStore(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0x3a => Instr::AStore(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0x3b => Instr::IStore(0),
            0x3c => Instr::IStore(1),
            0x3d => Instr::IStore(2),
            0x3e => Instr::IStore(3),
            0x3f => Instr::LStore(0),
            0x40 => Instr::LStore(1),
            0x41 => Instr::LStore(2),
            0x42 => Instr::LStore(3),
            0x43 => Instr::FStore(0),
            0x44 => Instr::FStore(1),
            0x45 => Instr::FStore(2),
            0x46 => Instr::FStore(3),
            0x47 => Instr::DStore(0),
            0x48 => Instr::DStore(1),
            0x49 => Instr::DStore(2),
            0x4a => Instr::DStore(3),
            0x4b => Instr::AStore(0),
            0x4c => Instr::AStore(1),
            0x4d => Instr::AStore(2),
            0x4e => Instr::AStore(3),
            0x4f => Instr::IAStore,
            0x50 => Instr::LAStore,
            0x51 => Instr::FAStore,
            0x52 => Instr::DAStore,
            0x53 => Instr::AAStore,
            0x54 => Instr::BAStore,
            0x55 => Instr::CAStore,
            0x56 => Instr::SAStore,
            0x57 => Instr::Pop,
            0x58 => Instr::Pop2,
            0x59 => Instr::Dup,
            0x5a => Instr::DupX1,
            0x5b => Instr::DupX2,
            0x5c => Instr::Dup2,
            0x5d => Instr::Dup2X1,
            0x5e => Instr::Dup2X2,
            0x5f => Instr::Swap,
            0x60 => Instr::IAdd,
            0x61 => Instr::LAdd,
            0x62 => Instr::FAdd,
            0x63 => Instr::DAdd,
            0x64 => Instr::ISub,
            0x65 => Instr::LSub,
            0x66 => Instr::FSub,
            0x67 => Instr::DSub,
            0x68 => Instr::IMul,
            0x69 => Instr::LMul,
            0x6a => Instr::FMul,
            0x6b => Instr::DMul,
            0x6c => Instr::IDiv,
            0x6d => Instr::LDiv,
            0x6e => Instr::FDiv,
            0x6f => Instr::DDiv,
            0x70 => Instr::IRem,
            0x71 => Instr::LRem,
            0x72 => Instr::FRem,
            0x73 => Instr::DRem,
            0x74 => Instr::INeg,
            0x75 => Instr::LNeg,
            0x76 => Instr::FNeg,
            0x77 => Instr::DNeg,
            0x78 => Instr::IShL,
            0x79 => Instr::LShL,
            0x7a => Instr::IShR,
            0x7b => Instr::LShR,
            0x7c => Instr::IUShR,
            0x7d => Instr::LUShR,
            0x7e => Instr::IAnd,
            0x7f => Instr::LAnd,
            0x80 => Instr::IOr,
            0x81 => Instr::LOr,
            0x82 => Instr::IXOr,
            0x83 => Instr::LXOr,
            0x84 => Instr::IInc {
                index: if wide { u16::read(i)? } else { u8::read(i)? as u16 },
                value: if wide { u16::read(i)? as i16 } else { (u8::read(i)? as i8) as i16 },
            },
            0x85 => Instr::I2l,
            0x86 => Instr::I2f,
            0x87 => Instr::I2d,
            0x88 => Instr::L2i,
            0x89 => Instr::L2f,
            0x8a => Instr::L2d,
            0x8b => Instr::F2i,
            0x8c => Instr::F2l,
            0x8d => Instr::F2d,
            0x8e => Instr::D2i,
            0x8f => Instr::D2l,
            0x90 => Instr::D2f,
            0x91 => Instr::I2b,
            0x92 => Instr::I2c,
            0x93 => Instr::I2s,
            0x94 => Instr::LCmp,
            0x95 => Instr::FCmpL,
            0x96 => Instr::FCmpG,
            0x97 => Instr::DCmpL,
            0x98 => Instr::DCmpG,
            0x99 => Instr::IfEq((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0x9a => Instr::IfNe((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0x9b => Instr::IfLt((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0x9c => Instr::IfGe((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0x9d => Instr::IfGt((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0x9e => Instr::IfLe((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0x9f => Instr::IfICmpEq((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xa0 => Instr::IfICmpNe((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xa1 => Instr::IfICmpLt((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xa2 => Instr::IfICmpGe((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xa3 => Instr::IfICmpGt((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xa4 => Instr::IfICmpLe((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xa5 => Instr::IfACmpEq((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xa6 => Instr::IfACmpNe((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xa7 => Instr::Goto((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xa8 => Instr::JSr((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xa9 => Instr::Ret(if wide { u16::read(i)? } else { u8::read(i)? as u16 }),
            0xaa => {
                let pad = (1 + ((i.position() - 1) / 4)) * 4 - i.position();
                for _ in 0..pad {
                    u8::read(i)?;
                }
                let default = (pos + (u32::read(i)? as i32)) as u32;
                let low = u32::read(i)?;
                let high = u32::read(i)?;
                let mut offsets = Vec::new();
                for _ in low..=high {
                    offsets.push((pos + (u32::read(i)? as i32)) as u32);
                }
                Instr::TableSwitch {
                    default,
                    low,
                    high,
                    offsets,
                }
            }
            0xab => {
                let pad = (1 + ((i.position() - 1) / 4)) * 4 - i.position();
                for _ in 0..pad {
                    u8::read(i)?;
                }
                let default = (pos + (u32::read(i)? as i32)) as u32;
                let count = u32::read(i)?;
                let mut pairs = Vec::new();
                for _ in 0..count {
                    pairs.push((
                        u32::read(i)? as i32,
                        (pos + (u32::read(i)? as i32)) as u32,
                    ));
                }
                Instr::LookupSwitch { default, pairs }
            }
            0xac => Instr::IReturn,
            0xad => Instr::LReturn,
            0xae => Instr::FReturn,
            0xaf => Instr::DReturn,
            0xb0 => Instr::AReturn,
            0xb1 => Instr::Return,
            0xb2 => Instr::GetStatic(PoolIndex::read(i)?),
            0xb3 => Instr::PutStatic(PoolIndex::read(i)?),
            0xb4 => Instr::GetField(PoolIndex::read(i)?),
            0xb5 => Instr::PutField(PoolIndex::read(i)?),
            0xb6 => Instr::InvokeVirtual(PoolIndex::read(i)?),
            0xb7 => Instr::InvokeSpecial(PoolIndex::read(i)?),
            0xb8 => Instr::InvokeStatic(PoolIndex::read(i)?),
            0xb9 => {
                let index = PoolIndex::read(i)?;
                u16::read(i)?;
                Instr::InvokeInterface(index)
            }
            0xba => {
                let index = PoolIndex::read(i)?;
                u16::read(i)?;
                Instr::InvokeDynamic(index)
            }
            0xbb => Instr::New(PoolIndex::read(i)?),
            0xbc => {
                let type_id = u8::read(i)?;
                let array_type = match type_id {
                    4 => ArrayType::Boolean,
                    5 => ArrayType::Char,
                    6 => ArrayType::Float,
                    7 => ArrayType::Double,
                    8 => ArrayType::Byte,
                    9 => ArrayType::Short,
                    10 => ArrayType::Int,
                    11 => ArrayType::Long,
                    _ => return Err(DecompileError::UnknownArrayType(type_id)),
                };
                Instr::NewArray(array_type)
            }
            0xbd => Instr::ANewArray(PoolIndex::read(i)?),
            0xbe => Instr::ArrayLength,
            0xbf => Instr::AThrow,
            0xc0 => Instr::CheckCast(PoolIndex::read(i)?),
            0xc1 => Instr::InstanceOf(PoolIndex::read(i)?),
            0xc2 => Instr::MonitorEnter,
            0xc3 => Instr::MonitorExit,
            0xc4 => Instr::read_instr(i, true, pos)?,
            0xc5 => Instr::MultiANewArray {
                index: u16::read(i)?,
                dimensions: u8::read(i)?,
            },
            0xc6 => Instr::IfNull((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xc7 => Instr::IfNonNull((pos + ((u16::read(i)? as i16) as i32)) as u16),
            0xc8 => Instr::Goto((pos + (u32::read(i)? as i32)) as u16),
            0xc9 => Instr::JSr((pos + (u32::read(i)? as i32)) as u16),
            _ => return Err(DecompileError::UnknownInstruction(code)),
        })
    }
}

pub type InstrSet = Vec<(u64, Instr)>;

pub fn parse_code(data: Vec<u8>) -> Result<InstrSet, DecompileError> {
    let length = data.len() as u64;
    let mut cursor = Cursor::new(data);
    let mut instructions = Vec::new();
    loop {
        let pos = cursor.position();
        let instr = Instr::read_instr(&mut cursor, false, pos as i32)?;
        instructions.push((pos, instr));
        if cursor.position() == length {
            break
        }
    }
    Ok(instructions)
}