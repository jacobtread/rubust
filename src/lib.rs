///! This is a rust library for working with Java class files
pub mod io;
pub mod error;
pub mod class;
pub mod decomp;

#[cfg(test)]
mod tests {
    use std::io::{Cursor, stdout};

    use crate::class::access::{AccessFlag, AccessFlags};
    use crate::class::class::Class;
    use crate::decomp::writer::JavaWriter;
    use crate::io::Readable;

    #[test]
    fn io_works() {
        let arr = include_bytes!("../Test.class");

        let mut cursor = Cursor::new(arr);
        let v = Class::read(&mut cursor);

        match v {
            Ok(value) => {
                println!("{:#?}", value)
            }
            Err(err) => {
                println!("{:?}", err)
            }
        }
    }

    #[test]
    fn access_flag() {
        let ac = &mut AccessFlags::new();
        ac.set(AccessFlag::Public);
        assert!(ac.is_set(AccessFlag::Public))
    }

    #[test]
    fn writer_test() {
        let arr = include_bytes!("../Test.class");

        let mut cursor = Cursor::new(arr);
        let v = Class::read(&mut cursor);

        match v {
            Ok(value) => {
                let writer = JavaWriter {};
                match writer.write_class(&value, &mut stdout()) {
                    Ok(_) => { println!() }
                    Err(err) => { println!("{:?}",err) }
                }
            }
            Err(err) => {
                println!("{:?}", err)
            }
        }
    }
}