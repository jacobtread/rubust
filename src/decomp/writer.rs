use std::io::Write;
use crate::class::access::AccessFlag;

use crate::class::class::Class;
use crate::error::WriteError;

pub struct JavaWriter;

type WriterError = Result<(), WriteError>;

impl JavaWriter {
    pub fn write_class<W: Write>(&self, class: &Class, o: &mut W) -> WriterError {
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

        write!(o, "}}")?;
        Ok(())
    }

}