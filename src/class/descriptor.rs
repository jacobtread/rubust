use std::fmt::{Debug, Formatter};
use regex::Regex;

use crate::class::class::ClassPath;

#[derive(Debug, Clone)]
pub struct MethodDescriptor {
    pub parameters: Vec<Descriptor>,
    pub return_type: Box<Descriptor>,
}

#[derive(Debug, Clone)]
pub struct ArrayDescriptor {
    pub dimensions: u8,
    pub descriptor: Box<Descriptor>,
}

#[derive(Clone)]
pub enum Descriptor {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Class(ClassPath),
    Short,
    Boolean,
    Array(ArrayDescriptor),
    Method(MethodDescriptor),
    Void,
    Unknown(String),
}

impl Debug for Descriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_java().as_str())?;
        Ok(())
    }
}

impl Descriptor {
    pub fn parse_all(value: &str) -> Vec<Descriptor> {
        let regex = Regex::new(r"([BCDFIJSZV]|(L.*;)|(\[.*))")
            .expect("invalid regex");
        regex.find_iter(value)
            .map(|v| Descriptor::parse(v.as_str())).collect()
    }

    pub fn ito_java(&self) -> &str {
        match self {
            Descriptor::Byte => "B",
            Descriptor::Char => "C",
            Descriptor::Double => "D",
            Descriptor::Float => "F",
            Descriptor::Int => "I",
            Descriptor::Long => "J",
            Descriptor::Short => "S",
            Descriptor::Boolean => "Z",
            Descriptor::Void => "V",
            _ => "",
        }
    }

    pub fn to_java(&self) -> String {
        match self {
            Descriptor::Class(clazz) => format!("L{};", clazz.internal_path()),
            Descriptor::Array(arr) => {
                format!("{}{}", "[".repeat(arr.dimensions as usize), arr.descriptor.to_java())
            }
            Descriptor::Method(met) => {
                let mut out = String::from('(');
                for x in &met.parameters {
                    out.push_str(x.to_java().as_str());
                }
                out.push_str(")");
                out.push_str(met.return_type.to_java().as_str());
                out
            }
            Descriptor::Unknown(v) => v.clone(),
            el => String::from(el.ito_java())
        }
    }

    pub fn parse(value: &str) -> Descriptor {
        match value {
            "B" => Descriptor::Byte,
            "C" => Descriptor::Char,
            "D" => Descriptor::Double,
            "F" => Descriptor::Float,
            "I" => Descriptor::Int,
            "J" => Descriptor::Long,
            "V" => Descriptor::Void,
            "S" => Descriptor::Short,
            "Z" => Descriptor::Boolean,
            _ => {
                if value.starts_with('L') {
                    let mut name = value.split_at(value.len() - 1);
                    name = name.0.split_at(1);
                    Descriptor::Class(ClassPath::from(name.1))
                } else if value.starts_with('[') {
                    let name = value.trim_start_matches("[");
                    let dimensions = (value.len() - name.len()) as u8;
                    Descriptor::Array(ArrayDescriptor {
                        dimensions,
                        descriptor: Box::new(Descriptor::parse(name)),
                    })
                } else if value.starts_with('(') {
                    if let Some(end) = value.rfind(')') {
                        let parts = value.split_at(end);
                        let parameters;
                        if parts.0.len() != 1 {
                            let raw_params = parts.0.split_at(1).1;
                            parameters = Descriptor::parse_all(raw_params)
                        } else {
                            parameters = Vec::with_capacity(0);
                        }
                        let return_type_raw = parts.1.split_at(1).1;
                        let return_type = Descriptor::parse(return_type_raw);
                        Descriptor::Method(MethodDescriptor {
                            parameters,
                            return_type: Box::new(return_type),
                        })
                    } else {
                        Descriptor::Unknown(value.to_string())
                    }
                } else {
                    Descriptor::Unknown(value.to_string())
                }
            }
        }
    }
}