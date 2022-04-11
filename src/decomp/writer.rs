use std::io::{Cursor, Write};

use crate::class::access::{AccessFlag, AccessFlags};
use crate::class::attribute::{AttributeValue, CodeAttr};
use crate::class::class::Class;
use crate::class::descriptor::Descriptor;
use crate::class::member::Member;
use crate::class::op::parse_code;
use crate::decomp::ops::{AST, decompile_block, find_paths, gen_control_flow_graph};
use crate::error::WriteError;

pub struct JavaWriter;

type WriteResult = Result<(), WriteError>;

impl JavaWriter {
    pub fn write_class<W: Write>(&self, class: &Class, o: &mut W) -> WriteResult {
        let class_path = &class.class_path;
        let package_str = class_path.package_str();
        if !package_str.is_empty() {
            write!(o, "package {};\n\n", package_str)?;
        }

        let imports = class.collect_imports();
        if !imports.is_empty() {
            for import in imports {
                write!(o, "import {};\n", import.full_path())?;
            }
            write!(o, "\n")?;
        }

        let access = class.access_flags;
        self.write_access_psf(&access, o)?;
        if access.is_set(AccessFlag::Enum) {
            write!(o, "enum ")?;
        } else if access.is_set(AccessFlag::Interface) {
            write!(o, "interface ")?;
        } else if access.is_set(AccessFlag::Annotation) {
            write!(o, "@interface ")?;
        } else {
            write!(o, "class ")?;
        }

        write!(o, "{} ", class_path.name)?;

        if let Some(x) = &class.super_class_path {
            if !x.is_java_lang() || x.name != "Object" {
                write!(o, "extends {} ", x.name)?;
            }
        }

        if !class.interfaces.is_empty() {
            write!(o, "implements ")?;
            let last = class.interfaces.len() - 1;
            for (i, interface) in class.interfaces.iter().enumerate() {
                write!(o, "{}", interface.full_path())?;
                if i != last {
                    write!(o, ", ")?;
                }
            }
            write!(o, " {{\n")?;
        } else {
            write!(o, "{{\n")?;
        }

        if !class.fields.is_empty() {
            for field in class.fields.iter() {
                self.write_field(field, o)?;
            }
            write!(o, "\n")?;
        }

        if !class.methods.is_empty() {
            for method in class.methods.iter() {
                self.write_method(class, method, o)?;
            }
        }

        write!(o, "}}")?;
        Ok(())
    }

    fn i_write_descriptor<W: Write>(&self, descriptor: &Descriptor, o: &mut W) -> WriteResult {
        write!(o, "{}", match descriptor {
            Descriptor::Byte => "byte",
            Descriptor::Char => "char",
            Descriptor::Double => "double",
            Descriptor::Float => "float",
            Descriptor::Int => "int",
            Descriptor::Long => "long",
            Descriptor::Short => "short",
            Descriptor::Boolean => "boolean",
            Descriptor::Void => "void",
            _ => "/* invalid descriptor */"
        }).map_err(WriteError::from)
    }

    fn write_descriptor<W: Write>(&self, descriptor: &Descriptor, o: &mut W) -> WriteResult {
        match descriptor {
            Descriptor::Class(class) => write!(o, "{}", class.name.as_str())?,
            Descriptor::Array(array) => {
                self.write_descriptor(&*array.descriptor, o)?;
                write!(o, "{}", "[]".repeat(array.dimensions as usize).as_str())?;
            }
            Descriptor::Unknown(value) => write!(o, "/* unknown: {} */", value)?,
            _ => self.i_write_descriptor(descriptor, o)?
        }
        Ok(())
    }

    fn write_access_psf<W: Write>(&self, access: &AccessFlags, o: &mut W) -> WriteResult {
        if access.is_set(AccessFlag::Public) {
            write!(o, "public ")?;
        } else if access.is_set(AccessFlag::Protected) {
            write!(o, "protected ")?;
        } else if access.is_set(AccessFlag::Private) {
            write!(o, "private ")?;
        }
        if access.is_set(AccessFlag::Static) {
            write!(o, "static ")?;
        }
        if access.is_set(AccessFlag::Final) {
            write!(o, "final ")?;
        }
        Ok(())
    }

    fn write_field<W: Write>(&self, field: &Member, o: &mut W) -> WriteResult {
        let access = field.access_flags;
        write!(o, "    ")?;
        self.write_access_psf(&access, o)?;
        if access.is_set(AccessFlag::Volatile) {
            write!(o, "volatile ")?;
        }
        if access.is_set(AccessFlag::Transient) {
            write!(o, "transient ")?;
        }
        self.write_descriptor(&field.descriptor, o)?;
        write!(o, " {};\n", field.name)?;
        Ok(())
    }

    fn write_code<W: Write>(&self, class: &Class, method: &Member, code_attr: &CodeAttr, o: &mut W) -> WriteResult {
        let instr = parse_code(code_attr.code.clone())
            .map_err(|_| WriteError::BadCodeAttribute)?;
        let control_flow_graph = gen_control_flow_graph(&instr);
        // let paths = find_paths(&control_flow_graph, 0, Vec::new());
        let is_static = method.access_flags.is_set(AccessFlag::Static);
        for block in control_flow_graph.values() {
            let decompiled = decompile_block(block, &class.constant_pool)?;
            let length = decompiled.len();
            for (index, statement) in decompiled.iter().enumerate() {
                if index == length - 1 {
                    if let AST::VoidReturn = statement {
                        break;
                    }
                } else {
                    write!(o, "      ")?;
                    self.write_ast(statement, class, is_static, method.is_init(), o)?;
                    writeln!(o)?;
                }
            }
        }
        Ok(())
    }

    fn write_ast<W: Write>(&self, ast: &AST, class: &Class, is_static: bool, is_init: bool, o: &mut W) -> WriteResult {
        match ast {
            AST::Set { index, value } => {
                if *index == 0 && !is_static {
                    write!(o, "this = ");
                } else {
                    write!(o, "var{} = ", index);
                }
                self.write_ast(value, class, is_static, is_init, o)?;
                write!(o, ";")?;
            }
            AST::Variable { index, var_type: _ } => {
                if *index == 0 && !is_static {
                    write!(o, "this")?;
                } else {
                    write!(o, "var{}", index)?;
                }
            }
            AST::Call {
                method_data,
                reference,
                args,
            } => {
                self.write_ast(reference, class, is_static, is_init, o)?;
                let name = &method_data.name_and_type.name;
                write!(o, ".{}(", name)?;
                let len = args.len();
                for (i, value) in args.iter().enumerate() {
                    self.write_ast(value, class, is_static, is_init, o)?;
                    if i != len - 1 {
                        write!(o, ", ")?;
                    }
                }
                write!(o, ");")?;
            }
            AST::Mul { lhs, rhs } => {
                self.write_ast(lhs, class, is_static, is_init, o)?;
                write!(o, " * ")?;
                self.write_ast(rhs, class, is_static, is_init, o)?;
                writeln!(o)?;
            }
            AST::ConstInt(value) => write!(o, "{}", value)?,
            AST::ConstFloat(value) => write!(o, "{}", value)?,
            AST::ConstString(value) => write!(o, "\"{}\"", value)?,
            AST::VoidReturn => {
                writeln!(o, "return;\n")?
            }
            AST::BasicCast { cast_type, value } => {
                write!(o, "(({}) (", cast_type)?;
                self.write_ast(value, class, is_static, is_init, o)?;
                write!(o, "))")?;
            }
            AST::ClassCast { cast_type, value } => {
                write!(o, "(({}) (", cast_type.name);
                self.write_ast(value, class, is_static, is_init, o)?;
                write!(o, "))")?;
            }
            AST::Static(member) => {
                let ty = &member.name_and_type;

                write!(o, "{}.{}", member.class.name, ty.name)?;
            }
            v => write!(o, "/* unknown */ {:?}", v)?,
        }
        Ok(())
    }

    fn write_method<W: Write>(&self, class: &Class, method: &Member, o: &mut W) -> WriteResult {
        write!(o, "    ")?;
        self.write_access_psf(&method.access_flags, o)?;
        let desc = match &method.descriptor {
            Descriptor::Method(method) => method,
            _ => Err(WriteError::BadDescriptor)?
        };
        let c = method.is_init();
        if c {
            write!(o, "{}(", class.class_path.name)?;
        } else {
            self.write_descriptor(&*desc.return_type, o)?;
            write!(o, " {}(", method.name)?;
        }
        let mut p_num = 0;
        if !desc.parameters.is_empty() {
            let last = desc.parameters.len() - 1;
            for (i, parameter) in desc.parameters.iter().enumerate() {
                self.write_descriptor(parameter, o)?;
                write!(o, " p_{}", p_num)?;
                p_num += 1;
                if i != last {
                    write!(o, ", ")?;
                }
            }
        }

        write!(o, ") {{\n")?;

        let code_attr = &method.attributes
            .iter()
            .find(|a| a.name == "Code")
            .unwrap()
            .value;

        match code_attr {
            AttributeValue::Code(code_attr_value) => {
                self.write_code(class, method, code_attr_value, o)?;
            }
            _ => Err(WriteError::BadCodeAttribute)?
        }

        write!(o, "    }}\n")?;
        Ok(())
    }
}