#![allow(dead_code)]

use std::io::Read;
use crate::io::{Readable, ReadResult};

pub const ACC_PUBLIC: u16 = 0x0001;
pub const ACC_PRIVATE: u16 = 0x0002;
pub const ACC_PROTECTED: u16 = 0x0004;
pub const ACC_STATIC: u16 = 0x0008;
pub const ACC_FINAL: u16 = 0x0010;
pub const ACC_SYNCHRONIZED: u8 = 0x0020;
pub const ACC_OPEN: u16 = 0x0020;
pub const ACC_NATIVE: u16 = 0x0100;
pub const ACC_ABSTRACT: u16 = 0x0400;
pub const ACC_STRICT: u16 = 0x0800;
pub const ACC_VOLATILE: u16 = 0x0040;
pub const ACC_BRIDGE: u16 = 0x0040;
pub const ACC_TRANSIENT: u16 = 0x0080;
pub const ACC_VARARGS: u16 = 0x0080;
pub const ACC_SYNTHETIC: u16 = 0x1000;
pub const ACC_ANNOTATION: u16 = 0x2000;
pub const ACC_ENUM: u16 = 0x4000;
pub const ACC_MANDATED: u16 = 0x8000;
pub const ACC_MODULE: u16 = 0x8000;
pub const ACC_SUPER: u16 = 0x0020;
pub const ACC_INTERFACE: u16 = 0x0200;

#[derive(Debug, Clone)]
pub struct AccessFlags(pub u16);

impl Readable for AccessFlags {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
        Ok(AccessFlags(u16::read(i)?))
    }
}

impl AccessFlags {
    pub fn contains(&self, flag: u16) -> bool {
        self.0.clone() & flag == flag
    }
}
