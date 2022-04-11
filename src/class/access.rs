#![allow(dead_code)]

use std::fmt::{Debug, Formatter, Write};
use std::io::Read;

use crate::class::constant::ConstantTag::Class;
use crate::io::{Readable, ReadResult};

macro_rules! access_flags {
    (
        $(
            $FieldName:ident = $FieldValue:literal
        ),* $(,)?
    ) => {
        #[derive(Copy, Clone)]
        pub enum AccessFlag {
            $($FieldName,)*
        }

        impl Debug for AccessFlag {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.name())
            }
        }

        impl AccessFlag {
            fn name(&self) -> &'static str {
                match self {
                    $(AccessFlag::$FieldName => stringify!($FieldName),)*
                }
            }

            fn value(&self) -> u16 {
                match self {
                    $(AccessFlag::$FieldName => $FieldValue,)*
                }
            }

            fn get_values(value: &AccessFlags) -> Vec<Self> where Self: Sized {
                let v = value.0;
                let mut out = Vec::new();
                $(
                    if v & $FieldValue == $FieldValue {
                        out.push(AccessFlag::$FieldName);
                    }
                )*
                out
            }
        }
    };
}

#[derive(Copy, Clone)]
pub struct AccessFlags(pub u16);

impl Readable for AccessFlags {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
        Ok(AccessFlags(u16::read(i)?))
    }
}

impl Debug for AccessFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let values: Vec<&'static str> = AccessFlag::get_values(self).iter()
            .map(|x| x.name()).collect();
        f.write_str(format!("AccessFlags ({:#06x}, [", self.0).as_str())?;
        f.write_str(values.join(", ").as_str())?;
        f.write_str("])")
    }
}

impl AccessFlags {
    pub fn new() -> AccessFlags { AccessFlags(0) }

    pub fn set(&mut self, value: AccessFlag) {
        let sv = value.value();
        self.0 = self.0 & sv
    }

    pub fn is_set(&self, value: AccessFlag) -> bool {
        let sv = value.value();
        self.0 | sv == sv
    }
}


access_flags! {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Super = 0x0020,
    Synchronized = 0x0020,
    Interface = 0x0200,
    Volatile = 0x0040,
    Bridge = 0x0040,
    Transient = 0x0080,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}