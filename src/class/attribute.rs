use std::io::{Cursor, Read};

use crate::class::access::AccessFlags;
use crate::class::constant::{ConstantPool, PoolIndex};
use crate::error::ReadError;
use crate::io::{Readable, ReadByteVecExt};
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
        let name = constant_pool.get_string(name_index)?;
        let length = u32::read(i)? as usize;
        let data = i.read_bytes_vec(length)?;
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
            "Code" => {
                let max_stack = u16::read(c)?;
                let max_locals = u16::read(c)?;

                let code_length = u32::read(c)? as usize;
                let code = read_byte_vec(c, code_length)?;

                let exception_table_length = u16::read(c)? as usize;
                let exception_table = read_vec_from(c, exception_table_length)?;

                let attributes_count = u16::read(c)? as usize;
                let mut attributes = Vec::with_capacity(attributes_count);


                for _ in 0..attributes_count {
                    attributes.push(Attribute::read(c, &constant_pool)?);
                }

                AttributeValue::Code(CodeAttr {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                })
            }
            "ConstantValue" => AttributeValue::ConstantValue(PoolIndex::read(c)?),
            "Deprecated" => AttributeValue::Depreciated,
            "Exceptions" => {
                let exceptions_count = u16::read(c)? as usize;
                let exceptions = read_vec_from(c, exceptions_count)?;
                AttributeValue::Exceptions(exceptions)
            }
            "InnerClasses" => {
                let classes_count = u16::read(c)? as usize;
                let classes = read_vec_from(c, classes_count)?;
                AttributeValue::InnerClasses(classes)
            }
            "Signature" => {
                let id = u16::read(c)?;
                AttributeValue::Signature(constant_pool.get_string(id)?)
            }
            "SourceDebugExtension" => AttributeValue::SourceDebugExtension(data.to_vec()),
            "LineNumberTable" => {
                let ln_table_count = u16::read(c)? as usize;
                let table = read_vec_from(c, ln_table_count)?;
                AttributeValue::LineNumberTable(table)
            }
            "LocalVariableTable" => {
                let lv_table_count = u16::read(c)? as usize;
                let table = read_vec_from(c, lv_table_count)?;
                AttributeValue::LocalVariableTable(table)
            }
            "SourceFile" => AttributeValue::SourceFile(PoolIndex::read(c)?),
            "Synthetic" => AttributeValue::Synthetic,
            "AnnotationDefault" => AttributeValue::AnnotationDefault(data.to_vec()),
            "EnclosingMethod" => AttributeValue::EnclosingMethod(EnclosingMethod::read(c)?),
            "LocalVariableTypeTable" => {
                let lvt_table_count = u16::read(c)? as usize;
                let table = read_vec_from(c, lvt_table_count)?;
                AttributeValue::LocalVariableTable(table)
            }
            _ => AttributeValue::Unknown(data.to_vec())
        })
    }
}
