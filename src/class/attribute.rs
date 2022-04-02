use std::io::Read;

use anyhow::{anyhow, Context, Result};

use crate::io::Readable;
use crate::rstruct;

rstruct! {
    ExceptionTableEntry {
        start_pc: u16,
        end_pc: u16,
        handler_pc: u16,
        catch_type: u16,
    }
}

rstruct! {
    LineNumber {
        start_pc: u16,
        line_number: u16,
    }
}

rstruct! {
    InnerClass {
        inner_class_info_index: u16,
        outer_class_info_index: u16,
        inner_name_index: u16,
        inner_class_access_flags: u16,
    }
}

rstruct! {
    LocalVariable {
        start_pc: u16,
        length: u16,
        name_index: u16,
        descriptor_index: u16,
        index: u16,
    }
}

rstruct! {
    LocalVariableType {
        start_pc: u16,
        length: u16,
        name_index: u16,
        signature_index: u16,
        index: u16,
    }
}

pub struct Annotation {
    type_index: u16,
}

pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
}

pub struct CodeAttr {
    max_stack: u16,
    max_locals: u16,
    code_length: u32,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<Attribute>,
}

pub struct ExceptionsAttr {
    indexes: Vec<u16>,
}

pub struct LineNumberTableAttr {
    values: Vec<LineNumber>,
}

pub struct LocalVariableTableAttr {
    values: Vec<LocalVariable>,
}

pub struct InnerClassesAttr {
    classes: Vec<InnerClass>,
}

pub struct EnclosingMethodAttr {
    class_index: u16,
    method_index: u16,
}

pub enum AttributeValue {
    // value_index
    ConstantValue(u16),
    Code(CodeAttr),
    Exceptions(ExceptionsAttr),
    // source-file_index
    SourceFile(u16),
    LineNumberTable(LineNumberTableAttr),
    LocalVariableTable(LocalVariableTableAttr),
    InnerClasses(InnerClassesAttr),
    Synthetic,
    Depreciated,
    EnclosingMethod(EnclosingMethodAttr),
}
