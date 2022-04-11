///! This is a rust library for working with Java class files
pub mod io;
pub mod error;
mod class;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::class::access::{AccessFlag, AccessFlags};
    use crate::class::class::Class;
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
}