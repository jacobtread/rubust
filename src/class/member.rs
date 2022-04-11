use std::fmt::Debug;
use std::io::Read;

use crate::class::access::AccessFlags;
use crate::class::attribute::Attribute;
use crate::class::constant::ConstantPool;
use crate::class::descriptor::Descriptor;
use crate::io::{Readable, ReadResult, VecReadableFn};

#[derive(Debug,Clone)]
pub struct Member {
    pub access_flags: AccessFlags,
    pub name: String,
    pub descriptor: Descriptor,
    pub attributes: Vec<Attribute>,
}


impl Member {
    pub fn is_init(&self) -> bool { self.name == "<init>" }

    pub fn read<R: Read>(
        i: &mut R,
        constant_pool: &ConstantPool,
    ) -> ReadResult<Member> {
        let access_flags = AccessFlags::read(i)?;
        let name = constant_pool.read_utf8(i)?.clone();
        let raw_descriptor = constant_pool.read_utf8(i)?;
        let descriptor = Descriptor::parse(raw_descriptor);
        let attributes = u16::read_vec_closure(i, |r| Attribute::read(r, constant_pool))?;
        Ok(Member {
            access_flags,
            name,
            descriptor,
            attributes,
        })
    }
}

