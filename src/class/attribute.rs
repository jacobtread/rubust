use std::io::{Cursor, Read};

use anyhow::{anyhow, Context, Result};

use crate::class::constant::{Constant, ConstantPool};
use crate::io::{read_byte_vec, read_vec_from, Readable};
use crate::rstruct;

rstruct! {
    ExceptionTableEntry {
        start_pc: u16,
        end_pc: u16,
        handler_pc: u16,
        catch_type: u16,
    }

    LineNumber {
        start_pc: u16,
        line_number: u16,
    }

    InnerClass {
        inner_class_info_index: u16,
        outer_class_info_index: u16,
        inner_name_index: u16,
        inner_class_access_flags: u16,
    }

    LocalVariable {
        start_pc: u16,
        length: u16,
        name_index: u16,
        descriptor_index: u16,
        index: u16,
    }

    LocalVariableType {
        start_pc: u16,
        length: u16,
        name_index: u16,
        signature_index: u16,
        index: u16,
    }

    MethodParameter {
        name_index: u16,
        access_flags: u16,
    }
}

pub struct Annotation {
    type_index: u16,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
}

pub struct CodeAttr {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<Attribute>,
}

pub struct EnclosingMethodAttr {
    class_index: u16,
    method_index: u16,
}

pub enum AttributeValue {
    // value_index
    ConstantValue(u16),
    Code(CodeAttr),
    Exceptions(Vec<u16>),
    // source-file_index
    SourceFile(u16),
    LineNumberTable(Vec<LineNumber>),
    LocalVariableTable(Vec<LocalVariable>),
    InnerClasses(Vec<InnerClass>),
    Synthetic,
    Depreciated,
    EnclosingMethod(EnclosingMethodAttr),
    // signature_index
    Signature(String),
    SourceDebugExtension,
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
}

impl AttributeValue {
    pub fn from_name(
        name: &str,
        data: &[u8],
        constant_pool: &ConstantPool,
    ) -> Result<AttributeValue> {
        let c = &mut Cursor::new(data);

        Ok(match name {
            "Code" => {
                let max_stack = u16::read(c)?;
                let max_locals = u16::read(c)?;

                let code_length = u32::read(c)?;
                let code = read_byte_vec(c, code_length as usize)?;

                let exception_table_length = u16::read(c)? as usize;
                let exception_table =
                    read_vec_from::<ExceptionTableEntry, Cursor<&[u8]>>(c, exception_table_length)?;

                let attributes_count = u16::read(c)? as usize;
                let attributes =
                    read_vec_from::<Attribute, Cursor<&[u8]>>(c, attributes_count)?;

                AttributeValue::Code(CodeAttr {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                })
            }
            "ConstantValue" => AttributeValue::ConstantValue(u16::read(c)?),
            "Deprecated" => AttributeValue::Depreciated,
            "Exceptions" => {
                let exceptions_count = u16::read(c)? as usize;
                let exceptions =
                    read_vec_from::<u16, Cursor<&[u8]>>(c, exceptions_count)?;
                AttributeValue::Exceptions(exceptions)
            }
            "InnerClasses" => {
                let classes_count = u16::read(c)? as usize;
                let classes =
                    read_vec_from::<InnerClass, Cursor<&[u8]>>(c, classes_count)?;
                AttributeValue::InnerClasses(classes)
            }
            "Signature" => {
                let id = u16::read(c)?;
                match constant_pool.values
                    .get(&id)
                    .ok_or(anyhow!("missing constant value {}", id))?
                {
                    Constant::Utf8(value) => AttributeValue::Signature(value.clone())m,
                    _ => Err(anyhow!("invalid signature constant. wasn't utf8"))
                }
            }
        })
    }
}