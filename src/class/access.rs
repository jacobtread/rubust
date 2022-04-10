#![allow(dead_code)]

use std::fmt::{Debug, Formatter};
use std::io::Read;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::io::{Readable, ReadResult};

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
enum ClassAccessFlags {
    Public = 0x0001,
    Private = 0x0002,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
enum FieldAFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
enum MethodAccessFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

#[derive(Clone)]
pub struct AccessFlags(pub u16);

impl AccessFlags {
    pub fn new() -> Self { Self(0) }

    pub fn set(&mut self, mask: u16) {
        self.0 = self.0 | mask
    }

    pub fn is_set(&self, mask: u16) -> bool {
        self.0 & mask == mask
    }
}

trait AccessFlag {
    fn is_set(&self, flag: &AccessFlags) -> bool;
    fn set(self, flag: &mut AccessFlags);
}

impl AccessFlag for MethodAccessFlags {
    fn is_set(&self, flag: &AccessFlags) -> bool { flag.is_set((*self as u16).into()) }
    fn set(self, flag: &mut AccessFlags) { flag.set(self.into()) }
}

impl AccessFlag for ClassAccessFlags {
    fn is_set(&self, flag: &AccessFlags) -> bool { flag.is_set((*self as u16).into()) }
    fn set(self, flag: &mut AccessFlags) { flag.set(self.into()) }
}

impl AccessFlag for FieldAFlags{
    fn is_set(&self, flag: &AccessFlags) -> bool { flag.is_set((*self as u16).into()) }
    fn set(self, flag: &mut AccessFlags) { flag.set(self.into()) }
}

impl Debug for AccessFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:#06x}", self.0).as_str())
    }
}

impl Readable for AccessFlags {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
        Ok(AccessFlags(u16::read(i)?))
    }
}
