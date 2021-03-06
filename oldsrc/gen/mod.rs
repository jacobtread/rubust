use std::io::{Cursor, stdout, Write};

use anyhow::{anyhow, Result};

use crate::class::attribute::{AttributeValue, CodeAttr};
use crate::class::class::Class;
use crate::class::constants::{ACC_ANNOTATION, ACC_ENUM, ACC_FINAL, ACC_INTERFACE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC, AccessFlags};
use crate::class::descriptor::Descriptor;
use crate::class::member::Member;
use crate::class::op::{Instruction, OpCodes};
use crate::io::Readable;

mod core;

pub struct ClassWriter {}

impl ClassWriter {
    pub fn write_class<B: Write>(&self, class: &Class, o: &mut B) -> Result<()> {
        let package_path = class.class_name.package_path();
        if !package_path.is_empty() {
            write!(o, "package {};\n\n", package_path)?;
        }

        let imports = class.collect_imports();
        if !imports.is_empty() {
            for import in imports {
                let package = import.package.join(".");
                write!(o, "import {}.{};\n", package, import.name)?;
            }
            write!(o, "\n")?;
        }

        self.write_access_flags(&class.access_flags, o)?;

        if class.access_flags.contains(ACC_ENUM) {
            write!(o, "enum ")?;
        } else if class.access_flags.contains(ACC_INTERFACE) {
            write!(o, "interface ")?;
        } else if class.access_flags.contains(ACC_ANNOTATION) {
            write!(o, "@interface ")?;
        } else {
            write!(o, "class ")?;
        }


        let class_name = class.class_name.clone();
        write!(o, "{} ", class_name.name)?;

        if class.super_name.is_some() && !class.super_name.as_ref().unwrap().is_object() {
            write!(o, "extends {} ", class.super_name.as_ref().unwrap().name)?;
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


    pub fn write_code<B: Write>(&self, class: &Class, code_attr: &CodeAttr, o: &mut B) -> Result<()> {
        let code = &code_attr.code;
        let mut c = Cursor::new(code);
        while (c.position() as usize) < code.len() {
            let instr = Instruction::read(&mut c)?;
            write!(o, "      {}", instr.op.get_name())?;
            for x in instr.args {
                write!(o, " {}", x)?;
            }
            writeln!(o)?;
        }
        Ok(())
    }

    pub fn write_method<B: Write>(&self, class: &Class, method: &Member, o: &mut B) -> Result<()> {
        write!(o, "    ")?;
        self.write_access_flags(&method.access_flags, o)?;
        let desc = match &method.descriptor {
            Descriptor::Method(method) => method,
            _ => Err(anyhow!("expected method descriptor for method"))?
        };
        let c = method.is_constructor();
        if c {
            write!(o, "{}(", class.class_name.name)?;
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
                self.write_code(class, code_attr_value, o)?;
            }
            _ => Err(anyhow!("code expected code attribute"))?
        }


        write!(o, "    }}\n")?;

        Ok(())
    }
}