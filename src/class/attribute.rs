use std::io::{Cursor, Read};

use byteorder::ReadBytesExt;

use crate::class::access::AccessFlags;
use crate::class::constant::{ConstantPool, PoolIndex};
use crate::error::ReadError;
use crate::io::{Readable, VecReadableBytesSize, VecReadableFn, VecReadableSize};
use crate::readable_struct;

readable_struct! {
    struct ExceptionTableEntry {
        start_pc: u16,
        end_pc: u16,
        handler_pc: u16,
        catch_type: u16,
    }

    struct LineNumber {
        start_pc: u16,
        line_number: u16,
    }

    struct InnerClass {
        inner_class_info_index: PoolIndex,
        outer_class_info_index: PoolIndex,
        inner_name_index: PoolIndex,
        inner_class_access_flags: AccessFlags,
    }

    struct LocalVariable {
        start_pc: u16,
        length: u16,
        name_index: PoolIndex,
        descriptor_index: PoolIndex,
        index: u16,
    }

    struct LocalVariableType {
        start_pc: u16,
        length: u16,
        name_index: PoolIndex,
        signature_index: PoolIndex,
        index: u16,
    }

    struct MethodParameter {
        name_index: PoolIndex,
        access_flags: AccessFlags,
    }

    struct EnclosingMethod {
        class_index: PoolIndex,
        method_index: PoolIndex,
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
}

impl Attribute {
    pub fn read<B: Read>(
        i: &mut B,
        constant_pool: &ConstantPool,
    ) -> Result<Self, ReadError> where Self: Sized {
        let name_index = u16::read(i)?;
        let name = constant_pool.get_utf8(name_index)?;
        let data = u32::read_bytes(i)?;
        Ok(Attribute {
            name: name.clone(),
            value: AttributeValue::from_name(
                name.as_str(),
                data.as_slice(),
                constant_pool,
            )?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CodeAttr {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    // value_index
    ConstantValue(PoolIndex),
    Code(CodeAttr),
    Exceptions(Vec<PoolIndex>),
    // source-file_index
    SourceFile(PoolIndex),
    LineNumberTable(Vec<LineNumber>),
    LocalVariableTable(Vec<LocalVariable>),
    InnerClasses(Vec<InnerClass>),
    Synthetic,
    Depreciated,
    EnclosingMethod(EnclosingMethod),
    // signature_index
    Signature(String),
    SourceDebugExtension(Vec<u8>),
    LocalVariableTypeTable(Vec<LocalVariableType>),
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault(Vec<u8>),
    StackMapTable,
    BootstrapMethods,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    MethodParameters(Vec<MethodParameter>),
    Unknown(Vec<u8>),
}

impl AttributeValue {
    pub fn from_name(
        name: &str,
        data: &[u8],
        constant_pool: &ConstantPool,
    ) -> Result<AttributeValue, ReadError> {
        let c = &mut Cursor::new(data);
        Ok(match name {
            "Code" => AttributeValue::Code(CodeAttr {
                max_stack: u16::read(c)?,
                max_locals: u16::read(c)?,
                code: u32::read_vec(c)?,
                exception_table: u16::read_vec(c)?,
                attributes: u16::read_vec_closure(
                    c,
                    |r| Attribute::read(r, &constant_pool),
                )?,
            }),
            "ConstantValue" => AttributeValue::ConstantValue(PoolIndex::read(c)?),
            "Deprecated" => AttributeValue::Depreciated,
            "Exceptions" => AttributeValue::Exceptions(u16::read_vec(c)?),
            "InnerClasses" => AttributeValue::InnerClasses(u16::read_vec(c)?),
            "Signature" => {
                let id = u16::read(c)?;
                AttributeValue::Signature(constant_pool.get_utf8(id)?.clone())
            }
            "SourceDebugExtension" => AttributeValue::SourceDebugExtension(data.to_vec()),
            "LineNumberTable" => AttributeValue::LineNumberTable(u16::read_vec(c)?),
            "LocalVariableTable" => AttributeValue::LocalVariableTable(u16::read_vec(c)?),
            "SourceFile" => AttributeValue::SourceFile(PoolIndex::read(c)?),
            "Synthetic" => AttributeValue::Synthetic,
            "AnnotationDefault" => AttributeValue::AnnotationDefault(data.to_vec()),
            "EnclosingMethod" => AttributeValue::EnclosingMethod(EnclosingMethod::read(c)?),
            "LocalVariableTypeTable" => AttributeValue::LocalVariableTable(u16::read_vec(c)?),
            _ => AttributeValue::Unknown(data.to_vec())
        })
    }
}
