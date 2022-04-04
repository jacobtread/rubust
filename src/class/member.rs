use std::f32::consts::E;
use std::io::Read;

use crate::class::attribute::Attribute;
use crate::class::constant::ConstantPool;
use crate::class::constants::AccessFlags;
use crate::class::descriptor::Descriptor;
use crate::io::Readable;

#[derive(Debug, Clone)]
pub struct Member {
    pub access_flags: AccessFlags,
    pub name: String,
    pub descriptor: Descriptor,
    pub attributes: Vec<Attribute>,
}

impl Member {
    pub fn is_constructor(&self) -> bool {
        self.name == String::from("<init>")
    }

    pub fn read<B: Read>(
        i: &mut B,
        constant_pool: &ConstantPool,
    ) -> anyhow::Result<Self> where Self: Sized {
        let access_flags = AccessFlags(u16::read(i)?);
        let name_index = u16::read(i)?;
        let name = constant_pool.get_string(name_index)?;

        let desc_index = u16::read(i)?;
        let raw_descriptor = constant_pool.get_string(desc_index)?;

        let descriptor = Descriptor::parse(raw_descriptor.as_str());

        println!("{:?}", descriptor);

        let attr_count = u16::read(i)? as usize;
        let mut attributes = Vec::with_capacity(attr_count);
        for _ in 0..attr_count {
            attributes.push(Attribute::read(i, constant_pool)?);
        }

        Ok(Member {
            access_flags,
            name,
            descriptor,
            attributes,
        })
    }
}