use std::io::Write;
use std::process::id;

use anyhow::Result;

use crate::class::class::Class;
use crate::class::constants::{ACC_ANNOTATION, ACC_ENUM, ACC_FINAL, ACC_INTERFACE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC};
use crate::class::member::Member;

mod core;


pub struct ClassWriter {}

impl ClassWriter {
    pub fn write_class<B: Write>(&self, class: &Class, o: &mut B) -> Result<()> {
        let package_path = class.class_name.package_path();
        if !package_path.is_empty() {
            write!(o, "package {};\n\n", package_path)?;
        }

        if class.access_flags.contains(ACC_PUBLIC) {
            write!(o, "public ")?;
        } else if class.access_flags.contains(ACC_PROTECTED) {
            write!(o, "protected ")?;
        } else if class.access_flags.contains(ACC_PRIVATE) {
            write!(o, "private ")?;
        }

        if class.access_flags.contains(ACC_FINAL) {
            write!(o, "final ")?;
        }

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
        }

        write!(o, "}}")?;

        Ok(())
    }
    pub fn write_field<B: Write>(&self, field: &Member, o: &mut B) -> Result<()> {
        write!(o, "{}", "    ")?;

        if field.access_flags.contains(ACC_PUBLIC) {
            write!(o, "public ")?;
        } else if field.access_flags.contains(ACC_PROTECTED) {
            write!(o, "protected ")?;
        } else if field.access_flags.contains(ACC_PRIVATE) {
            write!(o, "private ")?;
        }

        if field.access_flags.contains(ACC_STATIC) {
            write!(o, "static ")?;
        }

        if field.access_flags.contains(ACC_FINAL) {
            write!(o, "final ")?;
        }

        write!(o, "{} {};\n", field.descriptor, field.name)?;

        println!("{:?}", field);
        Ok(())
    }
}