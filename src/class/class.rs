use std::fmt::{Display, Formatter};
use std::io::Read;

use anyhow::{anyhow, Result};
use crate::class::attribute::Attribute;

use crate::class::constant::ConstantPool;
use crate::class::constants::AccessFlags;
use crate::class::member::Member;
use crate::io::Readable;

// Minor, Major
pub type Version = (u16, u16);

#[derive(Debug)]
pub struct Class {
    pub magic_number: u32,
    pub version: Version,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub class_name: ClassPath,
    pub super_name: Option<ClassPath>,
    pub interfaces: Vec<ClassPath>,
    pub fields: Vec<Member>,
    pub methods: Vec<Member>,
    pub attributes: Vec<Attribute>
}

pub const CLASS_SIGNATURE: u32 = 0xCAFEBABE;

impl Readable for Class {
    fn read<B: Read>(i: &mut B) -> Result<Self> where Self: Sized {
        let magic_number = <u32>::read(i)?;
        if magic_number != CLASS_SIGNATURE {
            Err(anyhow!("invalid class magic number got {}", magic_number))?
        }
        let minor_version = u16::read(i)?;
        let major_version = u16::read(i)?;
        let constant_pool = <ConstantPool>::read(i)?;
        let access_flags = AccessFlags( u16::read(i)?);

        let class_name_index = u16::read(i)?;
        let class_name = constant_pool.get_class_path(class_name_index)?
            .ok_or(anyhow!("missing class name"))?;

        let super_name_index = u16::read(i)?;
        let super_name = constant_pool.get_class_path(super_name_index)?;

        let interface_count = u16::read(i)?;
        let mut interfaces = Vec::with_capacity(interface_count as usize);

        for _ in 0..interface_count {
            let name_index = u16::read(i)?;
            let name = constant_pool.get_class_path(name_index)?
                .ok_or(anyhow!("invalid interface name reference"))?;
            interfaces.push(name)
        }

        let fields_count = u16::read(i)? as usize;
        let mut fields = Vec::with_capacity(fields_count );
        for _ in 0..fields_count {
            fields.push(Member::read(i, &constant_pool)?);
        }

        let methods_count = u16::read(i)? as usize;
        let mut methods = Vec::with_capacity(methods_count );
        for _ in 0..methods_count {
            methods.push(Member::read(i, &constant_pool)?);
        }

        let attributes_count = u16::read(i)? as usize;
        let mut attributes = Vec::with_capacity(attributes_count);

        for _ in 0..attributes_count {
            attributes.push(Attribute::read(i, &constant_pool)?);
        }

        return Ok(Class {
            magic_number,
            version: (minor_version, major_version),
            constant_pool,
            access_flags,
            class_name,
            super_name,
            interfaces,
            fields,
            methods,
            attributes
        });
    }
}

#[derive(Debug)]
pub struct ClassPath {
    pub package: Vec<String>,
    pub outer_classes: Vec<String>,
    pub name: String,
}

impl ClassPath {
    pub fn from_string(name: &str) -> ClassPath {
        let mut package: Vec<String> = name.split('/')
            .map(|s| s.to_string())
            .collect();
        let class = package.remove(package.len() - 1);
        let mut outer_classes: Vec<String> = class.split('$')
            .map(|s| s.to_string())
            .collect();
        let name = outer_classes.remove(outer_classes.len() - 1);
        ClassPath {
            package,
            outer_classes,
            name,
        }
    }

    pub fn package_path(&self) -> String {
        self.package.join(".")
    }

    pub fn jar_path(&self) -> String {
        let mut builder = self.package.join("/");
        if !self.outer_classes.is_empty() {
            builder += self.outer_classes.join("$").as_str();
            builder += "$"
        }
        builder += self.name.as_str();
        builder += ".class";
        builder
    }

    pub fn full_path(&self) -> String {
        let mut builder: String = self.package.join(".");
        if !builder.is_empty() {
            builder += ".";
        }
        for outer_c in &self.outer_classes {
            builder += format!("{}.", outer_c).as_str();
        }
        builder += self.name.to_string().as_str();
        builder
    }

    pub fn is_in_java_lang(&self) -> bool {
        if self.package.len() != 2 {
            return false;
        }
        self.package[0] == "java" && self.package[1] == "lang"
    }

    pub fn is_object(&self) -> bool {
        self.is_in_java_lang() && self.outer_classes.is_empty() && self.name == "Object"
    }
}

impl Display for ClassPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_path().as_str())
    }
}
