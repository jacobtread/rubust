use std::fmt::{Debug, Display, Formatter};
use std::io::Read;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::class::access::AccessFlags;
use crate::class::attribute::Attribute;
use crate::class::constant::{ConstantPool, PoolIndex};
use crate::class::member::Member;
use crate::error::{ConstantError, ReadError};
use crate::io::{Readable, ReadResult, VecReadableFn};

pub struct SourceVersion {
    minor: u16,
    major: MajorVersion,
}

impl Debug for SourceVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        MajorVersion::fmt(&self.major, f)?;
        f.write_str(format!(", {}", self.minor).as_str())
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum MajorVersion {
    JavaLE4 = 48,
    Java5 = 49,
    Java6 = 50,
    Java7 = 51,
    Java8 = 52,
    Java9 = 53,
    Java10 = 54,
    Java11 = 55,
    Java12 = 56,
    Java13 = 57,
    Java14 = 58,
    Java15 = 59,
    Java16 = 60,
    Java17 = 61,
    #[num_enum(default)]
    Unknown,
}

impl Debug for MajorVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MajorVersion::JavaLE4 => "Java LE 4",
            MajorVersion::Java5 => "Java 5",
            MajorVersion::Java6 => "Java 6",
            MajorVersion::Java7 => "Java 7",
            MajorVersion::Java8 => "Java 8",
            MajorVersion::Java9 => "Java 9",
            MajorVersion::Java10 => "Java 10",
            MajorVersion::Java11 => "Java 11",
            MajorVersion::Java12 => "Java 12",
            MajorVersion::Java13 => "Java 13",
            MajorVersion::Java14 => "Java 14",
            MajorVersion::Java15 => "Java 15",
            MajorVersion::Java16 => "Java 16",
            MajorVersion::Java17 => "Java 17",
            MajorVersion::Unknown => "Unknown"
        })
    }
}

pub const CLASS_SIGNATURE: u32 = 0xCAFEBABE;

#[derive(Debug)]
pub struct Class {
    pub version: SourceVersion,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub class_path: ClassPath,
    pub super_class_path: Option<ClassPath>,
    pub interfaces: Vec<ClassPath>,
    pub fields: Vec<Member>,
    pub methods: Vec<Member>,
    pub attributes: Vec<Attribute>,
}

impl Readable for Class {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
        let magic_number = u32::read(i)?;
        if magic_number != CLASS_SIGNATURE {
            Err(ReadError::InvalidMagic(magic_number))?;
        }

        let minor_version = u16::read(i)?;
        let major_version = MajorVersion::try_from(u16::read(i)?)
            .unwrap_or(MajorVersion::Unknown);

        let version = SourceVersion { minor: minor_version, major: major_version };
        let constant_pool = ConstantPool::read(i)?;

        let access_flags = AccessFlags::read(i)?;

        let class_path = constant_pool.get_class_path(u16::read(i)?)?
            .ok_or(ReadError::NoClassName)?;
        let super_class_path = constant_pool.get_class_path(u16::read(i)?)?;

        let interfaces = u16::read_vec_closure(i, |r| -> ReadResult<ClassPath> {
            let name_index = PoolIndex::read(r)?;
            constant_pool.get_class_path(name_index)?
                .ok_or(ConstantError::InvalidClassReference(name_index))
                .map_err(ReadError::from)
        })?;

        let fields = u16::read_vec_closure(i,|r|Member::read(r, &constant_pool))?;
        let methods = u16::read_vec_closure(i,|r|Member::read(r, &constant_pool))?;
        let attributes = u16::read_vec_closure(i, |r|Attribute::read(r, &constant_pool))?;

        Ok(Class {
            version,
            constant_pool,
            access_flags,
            class_path,
            super_class_path,
            interfaces,
            fields,
            methods,
            attributes
        })
    }
}


/// Represents a path to a class includes outer classes,
/// the packages list and the class name
#[derive(Clone)]
pub struct ClassPath {
    pub name: String,
    pub package: Vec<String>,
    pub outer_classes: Vec<String>,
}

impl ClassPath {
    pub fn from(value: &str) -> Self {
        // Package components are split using slashes
        let mut package: Vec<String> = value.split('/')
            .map(|s| s.to_string())
            .collect();
        // Class is the last value of the packages list
        let class = package.remove(package.len() - 1);
        // If this class is a child class the name will be divided
        // using the $ symbol
        let mut outer_classes: Vec<String> = class.split('$')
            .map(|s| s.to_string())
            .collect();
        let name = outer_classes.remove(outer_classes.len() - 1);
        ClassPath { package, outer_classes, name }
    }

    pub fn is_object(&self) -> bool { self.is_java_lang() && self.outer_classes.is_empty() && self.name == "Object" }
    pub fn is_java_lang(&self) -> bool { self.package.len() >= 2 && self.package[0] == "java" && self.package[1] == "lang" }
    pub fn package_str(&self) -> String { self.package.join(".") }
    pub fn jar_path(&self) -> String {
        let mut out = self.package_str();
        if !self.outer_classes.is_empty() {
            out += self.outer_classes.join("$").as_str();
            out += "$";
        }
        out += self.name.as_str();
        out += ".class";
        out
    }
    pub fn full_path(&self) -> String {
        let mut out = self.package_str();
        if !out.is_empty() {
            out += ".";
        }
        if !self.outer_classes.is_empty() {
            out += self.outer_classes.join(".").as_str();
            out += ".";
        }
        out += self.name.as_str();
        out
    }
    pub fn internal_path(&self) -> String {
        let mut out = self.package_str();
        if !out.is_empty() {
            out += "/";
        }
        if !self.outer_classes.is_empty() {
            out += self.outer_classes.join("$").as_str();
            out += "$";
        }
        out += self.name.as_str();
        out
    }
}

impl Debug for ClassPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_path().as_str())
    }
}

impl Display for ClassPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_path().as_str())
    }
}