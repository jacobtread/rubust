use std::io::Write;

use anyhow::{anyhow, Result};

use crate::class::class::Class;
use crate::class::constants::{ACC_ANNOTATION, ACC_ENUM, ACC_FINAL, ACC_INTERFACE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC, AccessFlags};
use crate::class::descriptor::Descriptor;
use crate::class::member::Member;

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

    pub fn write_access_flags<B: Write>(&self, access_flags: &AccessFlags, o: &mut B) -> Result<()> {
        if access_flags.contains(ACC_PUBLIC) {
            write!(o, "public ")?;
        } else if access_flags.contains(ACC_PROTECTED) {
            write!(o, "protected ")?;
        } else if access_flags.contains(ACC_PRIVATE) {
            write!(o, "private ")?;
        }

        if access_flags.contains(ACC_STATIC) {
            write!(o, "static ")?;
        }

        if access_flags.contains(ACC_FINAL) {
            write!(o, "final ")?;
        }

        Ok(())
    }

    pub fn write_field<B: Write>(&self, field: &Member, o: &mut B) -> Result<()> {
        write!(o, "    ")?;

        self.write_access_flags(&field.access_flags, o)?;
        self.write_descriptor(&field.descriptor, o)?;

        write!(o, " {};\n", field.name)?;

        Ok(())
    }

    pub fn write_descriptor<B: Write>(&self, descriptor: &Descriptor, o: &mut B) -> Result<()> {
        match descriptor {
            Descriptor::Int => write!(o, "int")?,
            Descriptor::Long => write!(o, "long")?,
            Descriptor::Float => write!(o, "float")?,
            Descriptor::Double => write!(o, "double")?,
            Descriptor::Char => write!(o, "char")?,
            Descriptor::Byte => write!(o, "byte")?,
            Descriptor::Boolean => write!(o, "boolean")?,
            Descriptor::Short => write!(o, "short")?,
            Descriptor::Void => write!(o, "void")?,
            Descriptor::Array(array) => {
                self.write_descriptor(&*array.descriptor, o)?;
                write!(o, "{}", "[]".repeat(array.dimensions as usize))?;
            }
            Descriptor::ClassReference(class) => write!(o, "{}", class.name)?,
            _ => write!(o, "/* Failed to parse type*/")?,
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
            println!("{:?}", method);
        } else {
            self.write_descriptor(&*desc.return_type, o)?;
            write!(o, " {}(", method.name);
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


        write!(o, "    }}\n")?;

        Ok(())
    }
}