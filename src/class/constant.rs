use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::Read;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::class::class::ClassPath;
use crate::class::descriptor::Descriptor;
use crate::error::ConstantError;
use crate::error::ReadError::UnknownConstantTag;
use crate::io::{Readable, ReadResult};
use crate::readable_struct;

/// Represents an index in the pool.
/// Note: not every index of the pool contains a constant. Some contents
/// span across multiple indexes in the pool resulting in some indexes
/// being blank.
pub type PoolIndex = u16;

pub struct ConstantPool {
    pub inner: HashMap<PoolIndex, Constant>,
}

impl Debug for ConstantPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str("ConstantPool {\n")?;
            let mut keys: Vec<&u16> = self.inner.keys().collect();
            keys.sort(); // Obtain a sorted version of the keys
            for key in keys {
                let v = self.inner.get(key)
                    .expect("expected constant pool to contain index");
                f.write_str(format!("  {}: {:?}\n", key, v).as_str())?;
            }
            f.write_str("}")?;
        } else {
            f.write_str("ConstantPool { ")?;
            let mut keys: Vec<&u16> = self.inner.keys().collect();
            keys.sort(); // Obtain a sorted version of the keys
            let last = keys.len() - 1;
            for (index, key) in keys.iter().enumerate() {
                let v = self.inner.get(key)
                    .expect("expected constant pool to contain index");
                f.write_str(format!("{}: {:?}", key, v).as_str())?;
                if index != last {
                    f.write_str(", ")?;
                }
            }
            f.write_str(" }")?;
        }
        Ok(())
    }
}

impl Readable for ConstantPool {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
        let size = u16::read(i)?;
        let mut pool = ConstantPool { inner: HashMap::with_capacity(size as usize) };
        let mut index = 1;
        while index < size {
            let value = ConstantValue::read(i)?;
            pool.inner.insert(index, value.value);
            // Long and Double constants consume two indexes worth of data
            index += match value.tag {
                ConstantTag::Long | ConstantTag::Double => 2,
                _ => 1
            }
        }
        Ok(pool)
    }
}

impl ConstantPool {
    pub fn get_class_path(&self, index: PoolIndex) -> Result<Option<ClassPath>, ConstantError> {
        if index == 0 { return Ok(None); }
        match self.inner.get(&index) {
            Some(constant) => match constant {
                Constant::Class(v) => Ok(Some(ClassPath::from(
                    self.get_utf8(*v)
                        .map_err(|_| ConstantError::InvalidClassReference(index))?
                ))),
                _ => Err(ConstantError::InvalidClassReference(index))
            }
            None => Ok(None)
        }
    }

    pub fn get_class_path_required(&self, index: PoolIndex) -> Result<ClassPath, ConstantError> {
        match self.inner.get(&index) {
            Some(constant) => match constant {
                Constant::Class(v) => Ok(ClassPath::from(
                    self.get_utf8(*v)
                        .map_err(|_| ConstantError::InvalidClassReference(index))?
                )),
                _ => Err(ConstantError::InvalidClassReference(index))
            }
            None => Err(ConstantError::InvalidClassReference(index))
        }
    }

    pub fn get_utf8(&self, index: PoolIndex) -> Result<&String, ConstantError> {
        match self.inner.get(&index) {
            Some(constant) => match constant {
                Constant::Utf8(value) => Ok(value),
                _ => Err(ConstantError::ExpectedUtf8(index))
            }
            None => Err(ConstantError::NotFound(index))
        }
    }

    pub fn get_member_ref(&self, index: PoolIndex) -> Result<MemberReference, ConstantError> {
        match self.inner.get(&index) {
            Some(constant) => match constant {
                Constant::MethodRef(value) |
                Constant::FieldRef(value) |
                Constant::InterfaceMethodRef(value) => {
                    let class = self.get_class_path_required(value.class_index)?;
                    let name_and_type = self.get_name_and_type(value.name_and_type_info)?;
                    Ok(MemberReference {
                        class,
                        name_and_type
                    })
                },
                _ => Err(ConstantError::ExpectedMethodRef(index))
            }
            None => Err(ConstantError::NotFound(index))
        }
    }

    pub fn get_name_and_type(&self, index: PoolIndex) -> Result<NameAndType, ConstantError> {
        match self.inner.get(&index) {
            Some(constant) => match constant {
                Constant::NameAndType(value) => Ok(NameAndType {
                    name: self.get_utf8(value.name_index)?.clone(),
                    descriptor: Descriptor::parse(self.get_utf8(value.descriptor_index)?),
                }),
                _ => Err(ConstantError::ExpectedMethodRef(index))
            }
            None => Err(ConstantError::NotFound(index))
        }
    }

    pub fn read_utf8<R: Read>(&self, i: &mut R) -> ReadResult<&String> {
        let index = PoolIndex::read(i)?;
        Ok(self.get_utf8(index)?)
    }

    pub fn read_class_path<R: Read>(&self, i: &mut R) -> ReadResult<Option<ClassPath>> {
        let index = PoolIndex::read(i)?;
        Ok(self.get_class_path(index)?)
    }
}

#[derive(Debug, Clone)]
pub struct NameAndType {
    pub name: String,
    pub descriptor: Descriptor,
}

#[derive(Debug, Clone)]
pub struct MemberReference {
    pub class: ClassPath,
    pub name_and_type: NameAndType,
}

readable_struct! {
    struct MemberReferenceU {
        class_index: PoolIndex,
        name_and_type_info: PoolIndex,
    }

    struct NameAndTypeIndex {
        name_index: PoolIndex,
        descriptor_index: PoolIndex,
    }

    struct MethodHandle {
        reference_kind: u8,
        reference_index: PoolIndex,
    }

    struct DynamicConstant {
        bootstrap_method_attr_index: PoolIndex,
        name_and_type_index: PoolIndex,
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ConstantTag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

#[derive(Debug)]
pub enum Constant {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    // Class contains a pool reference to a Utf8 Constant which is the name of the class.
    Class(PoolIndex),
    // Strings contain a pool reference to a Utf8 Constant which is the value of the string.
    String(PoolIndex),
    FieldRef(MemberReferenceU),
    MethodRef(MemberReferenceU),
    InterfaceMethodRef(MemberReferenceU),
    NameAndType(NameAndTypeIndex),
    MethodHandle(MethodHandle),
    // MethodType contains a pool reference to a descriptor Ut8 constant
    MethodType(PoolIndex),
    Dynamic(DynamicConstant),
    InvokeDynamic(DynamicConstant),
    // Module contains a pool reference to a Utf8 Constant which  is the name of the module
    Module(PoolIndex),
    // Package contains a pool reference to a Utf8 Constant which  is the name of the module
    Package(PoolIndex),
}

impl From<Constant> for ConstantTag {
    fn from(value: Constant) -> Self {
        match value {
            Constant::Utf8 { .. } => ConstantTag::Utf8,
            Constant::Integer { .. } => ConstantTag::Integer,
            Constant::Float { .. } => ConstantTag::Float,
            Constant::Long { .. } => ConstantTag::Long,
            Constant::Double { .. } => ConstantTag::Double,
            Constant::Class { .. } => ConstantTag::Class,
            Constant::String { .. } => ConstantTag::String,
            Constant::FieldRef { .. } => ConstantTag::FieldRef,
            Constant::MethodRef { .. } => ConstantTag::MethodRef,
            Constant::InterfaceMethodRef { .. } => ConstantTag::InterfaceMethodRef,
            Constant::NameAndType { .. } => ConstantTag::NameAndType,
            Constant::MethodHandle { .. } => ConstantTag::MethodHandle,
            Constant::MethodType { .. } => ConstantTag::MethodType,
            Constant::Dynamic { .. } => ConstantTag::Dynamic,
            Constant::InvokeDynamic { .. } => ConstantTag::InvokeDynamic,
            Constant::Module { .. } => ConstantTag::Module,
            Constant::Package { .. } => ConstantTag::Package,
        }
    }
}

#[derive(Debug)]
pub struct ConstantValue {
    tag: ConstantTag,
    value: Constant,
}

impl Readable for ConstantValue {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
        let tag_raw = u8::read(i)?;
        let tag = ConstantTag::try_from(tag_raw)
            .map_err(|_| UnknownConstantTag(tag_raw))?;
        let value = match tag {
            ConstantTag::Utf8 => Constant::Utf8(String::read(i)?),
            ConstantTag::Integer => Constant::Integer(i32::read(i)?),
            ConstantTag::Float => Constant::Float(f32::read(i)?),
            ConstantTag::Long => Constant::Long(i64::read(i)?),
            ConstantTag::Double => Constant::Double(f64::read(i)?),
            ConstantTag::Class => Constant::Class(PoolIndex::read(i)?),
            ConstantTag::String => Constant::String(PoolIndex::read(i)?),
            ConstantTag::FieldRef => Constant::FieldRef(MemberReferenceU::read(i)?),
            ConstantTag::MethodRef => Constant::MethodRef(MemberReferenceU::read(i)?),
            ConstantTag::InterfaceMethodRef => Constant::InterfaceMethodRef(MemberReferenceU::read(i)?),
            ConstantTag::NameAndType => Constant::NameAndType(NameAndTypeIndex::read(i)?),
            ConstantTag::MethodHandle => Constant::MethodHandle(MethodHandle::read(i)?),
            ConstantTag::MethodType => Constant::MethodType(PoolIndex::read(i)?),
            ConstantTag::Dynamic => Constant::Dynamic(DynamicConstant::read(i)?),
            ConstantTag::InvokeDynamic => Constant::InvokeDynamic(DynamicConstant::read(i)?),
            ConstantTag::Module => Constant::Module(PoolIndex::read(i)?),
            ConstantTag::Package => Constant::Package(PoolIndex::read(i)?),
        };
        Ok(ConstantValue { tag, value })
    }
}

