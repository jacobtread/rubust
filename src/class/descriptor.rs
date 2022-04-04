
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

    },
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
                } else {
                    Descriptor::Unknown(value.to_string())
                }
            }
        }
    }

    pub fn from_str(value: String) -> Vec<Descriptor> {
        let parts = value.split_inclusive(r"([BCDFIJSZ]|(L.*;)|(\[.*]))");
        parts.map(Descriptor::parse).collect()
    }
}
