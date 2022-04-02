use std::io::Read;
use crate::class::constant::ConstantPool;
use crate::io::Readable;
use anyhow::{ Result};

// Minor, Major
pub type Version = (u16, u16);

#[derive(Debug)]
pub struct Class {
    pub magic_number: u32,
    pub version: Version,
    pub pool: ConstantPool
}

impl Readable for Class {
    fn read<B: Read>(i: &mut B) -> Result<Self> where Self: Sized {
        return Ok(Class {
            magic_number: <u32>::read(i)?,
            version: (<u16>::read(i)?, <u16>::read(i)?),
            pool: <ConstantPool>::read(i)?
        })
    }
}