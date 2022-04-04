use std::path::Display;

#[derive(Debug, Clone)]
pub enum Descriptor {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    ClassReference(String),
    Short,
    Boolean,
    Array {
        dimensions: u8,
        descriptor: Box<Descriptor>,
    },
    Function {
        parameters: Vec<Descriptor>,
        return_type: Box<Descriptor>,
    },
    Void,
    Unknown(String),
}

impl Descriptor {
    fn parse(value: &str) -> Descriptor {
        match value {
            "B" => Descriptor::Byte,
            "C" => Descriptor::Char,
            "D" => Descriptor::Double,
            "F" => Descriptor::Float,
            "I" => Descriptor::Int,
            "J" => Descriptor::Long,
            "V" => Descriptor::Void,
            _ => {
                if value.starts_with('L') {
                    let mut name = value.split_at(value.len() - 1);
                    name = name.0.split_at(1);
                    Descriptor::ClassReference(String::from(name.1))
                } else if value.starts_with('[') {
                    let name = value.trim_start_matches("[");
                    let dimensions = (value.len() - name.len()) as u8;
                    Descriptor::Array {
                        dimensions,
                        descriptor: Box::new(Descriptor::parse(name)),
                    }
                } else if value.starts_with('(') {
                    if let Some(end) = value.rfind(')') {
                        let parts = value.split_at(end);
                        let parameters;
                        if parts.0.len() != 1 {
                            let raw_params = parts.0.split_at(1).1;
                            parameters = Descriptor::from_str(String::from(raw_params))
                        } else {
                            parameters = Vec::with_capacity(0);
                        }
                        let return_type_raw = parts.1.split_at(1).1;
                        let return_type = Descriptor::parse(return_type_raw);
                        Descriptor::Function {
                            parameters,
                            return_type: Box::new(return_type),
                        }
                    } else {
                        Descriptor::Unknown(value.to_string())
                    }
                } else {
                    Descriptor::Unknown(value.to_string())
                }
            }
        }
    }

    pub fn from_str(value: String) -> Vec<Descriptor> {
        let parts = value.split_inclusive(r"([BCDFIJSZV]|(L.*;)|(\[.*]))");
        parts.map(Descriptor::parse).collect()
    }
}
