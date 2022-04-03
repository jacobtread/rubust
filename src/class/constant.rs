use std::collections::HashMap;
use std::io::Read;

use anyhow::{anyhow, Result};

use crate::class::class::ClassPath;
use crate::class::constants::*;
use crate::io::Readable;

#[derive(Debug)]
pub struct ConstantPool {
    pub values: HashMap<u16, Constant>,
}


impl ConstantPool {
    pub fn get_class_path(&self, index: u16) -> Result<Option<ClassPath>> {
        Ok(if index != 0 {
            match self.values.get(&index) {
                Some(it) => match it {
                    Constant::Class(index) => match &self.values[&index] {
                        Constant::Utf8(value) => Some(ClassPath::from_string(&value)),
                        _ => Err(anyhow!("invalid class reference"))?
                    }
                    _ => Err(anyhow!("invalid class reference"))?
                }
                None => None
            }
        } else {
            None
        })
    }
}

impl ConstantPool {
    fn new(size: usize) -> Self {
        return ConstantPool {
            values: HashMap::with_capacity(size)
        };
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassItemRef {
    class_index: u16,
    name_and_type_info: u16,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct NameAndTypeConstant {
    name_index: u16,
    descriptor_index: u16,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MethodHandleConstant {
    reference_kind: u8,
    reference_index: u16,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DynamicConstant {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
}


#[derive(Debug)]
pub enum Constant {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    // name_index
    String(u16),
    // string_index
    FieldRef(ClassItemRef),
    MethodRef(ClassItemRef),
    InterfaceMethodRef(ClassItemRef),
    NameAndType(NameAndTypeConstant),
    MethodHandle(MethodHandleConstant),
    MethodType(u16),
    // descriptor_index
    Dynamic(DynamicConstant),
    InvokeDynamic(DynamicConstant),
    Module(u16),
    // name_index
    Package(u16), // name_index
}

impl Constant {
    pub fn tag(&self) -> u8 {
        match self {
            Constant::Utf8 { .. } => CONSTANT_UTF8,
            Constant::Integer { .. } => CONSTANT_INTEGER,
            Constant::Float { .. } => CONSTANT_FLOAT,
            Constant::Long { .. } => CONSTANT_LONG,
            Constant::Double { .. } => CONSTANT_DOUBLE,
            Constant::Class { .. } => CONSTANT_CLASS,
            Constant::String { .. } => CONSTANT_STRING,
            Constant::FieldRef { .. } => CONSTANT_FIELDREF,
            Constant::MethodRef { .. } => CONSTANT_METHODREF,
            Constant::InterfaceMethodRef { .. } => CONSTANT_INTERFACE_METHODREF,
            Constant::NameAndType { .. } => CONSTANT_NAME_AND_TYPE,
            Constant::MethodHandle { .. } => CONSTANT_METHOD_HANDLE,
            Constant::MethodType { .. } => CONSTANT_METHOD_TYPE,
            Constant::Dynamic { .. } => CONSTANT_DYNAMIC,
            Constant::InvokeDynamic { .. } => CONSTANT_INVOKE_DYNAMIC,
            Constant::Module { .. } => CONSTANT_MODULE,
            Constant::Package { .. } => CONSTANT_PACKAGE,
        }
    }
}


impl Readable for Constant {
    fn read<B: Read>(i: &mut B) -> Result<Self> where Self: Sized {
        let tag = <u8>::read(i)?;
        Ok(
            match tag {
                CONSTANT_UTF8 => Constant::Utf8(<String>::read(i)?),
                CONSTANT_INTEGER => Constant::Integer(<i32>::read(i)?),
                CONSTANT_FLOAT => Constant::Float(<f32>::read(i)?),
                CONSTANT_LONG => Constant::Long(<i64>::read(i)?),
                CONSTANT_DOUBLE => Constant::Double(<f64>::read(i)?),
                CONSTANT_CLASS => Constant::Class(<u16>::read(i)?),
                CONSTANT_STRING => Constant::String(<u16>::read(i)?),
                CONSTANT_FIELDREF => Constant::FieldRef(ClassItemRef {
                    class_index: <u16>::read(i)?,
                    name_and_type_info: <u16>::read(i)?,
                }),
                CONSTANT_METHODREF => Constant::MethodRef(ClassItemRef {
                    class_index: <u16>::read(i)?,
                    name_and_type_info: <u16>::read(i)?,
                }),
                CONSTANT_INTERFACE_METHODREF => Constant::MethodRef(ClassItemRef {
                    class_index: <u16>::read(i)?,
                    name_and_type_info: <u16>::read(i)?,
                }),
                CONSTANT_NAME_AND_TYPE => Constant::NameAndType(NameAndTypeConstant {
                    name_index: <u16>::read(i)?,
                    descriptor_index: <u16>::read(i)?,
                }),
                CONSTANT_METHOD_HANDLE => Constant::MethodHandle(MethodHandleConstant {
                    reference_kind: <u8>::read(i)?,
                    reference_index: <u16>::read(i)?,
                }),
                CONSTANT_METHOD_TYPE => Constant::MethodType(<u16>::read(i)?),
                CONSTANT_DYNAMIC => Constant::Dynamic(DynamicConstant {
                    bootstrap_method_attr_index: <u16>::read(i)?,
                    name_and_type_index: <u16>::read(i)?,
                }),
                CONSTANT_INVOKE_DYNAMIC => Constant::InvokeDynamic(DynamicConstant {
                    bootstrap_method_attr_index: <u16>::read(i)?,
                    name_and_type_index: <u16>::read(i)?,
                }),
                CONSTANT_MODULE => Constant::Module(<u16>::read(i)?),
                CONSTANT_PACKAGE => Constant::Package(<u16>::read(i)?),
                _ => { Err(anyhow!("unknown constant tag {}", tag))? }
            }
        )
    }
}

impl Readable for ConstantPool {
    fn read<B: Read>(i: &mut B) -> Result<Self> where Self: Sized {
        let size = <u16>::read(i)?;
        let mut pool = ConstantPool::new(size as usize);
        let mut x = 1;
        while x < size {
            let constant = Constant::read(i)?;
            let tag = constant.tag();
            pool.values.insert(x, constant);
            x += match tag {
                CONSTANT_LONG | CONSTANT_DOUBLE => 2,
                _ => 1
            }
        }
        Ok(pool)
    }
}
