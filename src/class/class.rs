use std::fmt::{Display, Formatter};

pub const CLASS_SIGNATURE: u32 = 0xCAFEBABE;

/// Represents a path to a class includes outer classes,
/// the packages list and the class name
#[derive(Debug, Clone)]
pub struct ClassPath {
    pub name: String,
    pub package: Vec<String>,
    pub outer_classes: Vec<String>,
}

impl From<&str> for ClassPath {
    fn from(value: &str) -> Self {
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
}

impl ClassPath {
    pub fn is_object(&self) -> bool { self.is_java_lang() && self.outer_classes.is_empty() && self.name == "Object" }
    pub fn is_java_lang(&self) -> bool { self.package.len() >= 2 && self.package[0] == "java" && self.package[1] == "lang" }
    pub fn package_str(&self) -> String { self.package.join(".") }
    pub fn jar_path(&self) -> String {
        let mut out = self.package_str();
        if !self.outer_classes.is_empty() {
            out += self.outer_classes.join("$").as_str() + "$"
        }
        out += self.name.as_str() + ".class";
        out
    }
    pub fn full_path(&self) -> String {
        let mut out = self.package_str();
        if !out.is_empty() {
            out += "."
        }
        if !self.outer_classes.is_empty() {
            out += self.outer_classes.join(".").as_str() + "."
        }
        out += self.name.as_str();
        out
    }
}

impl Display for ClassPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_path().as_str())
    }
}