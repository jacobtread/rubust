///! This is a rust library for working with Java class files
pub mod io;
pub mod error;
mod class;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::class::constant::ConstantValue;
    use crate::io::{Readable, ReadResult};

    #[test]
    fn io_works() {
        let mut arr = vec![1u8, 3, 2, 23, 2, 2, 32, 3, 13, 21];

        let mut cursor = Cursor::new(arr);
        let v = u8::read(&mut cursor).expect("");
        println!("{}",v);
        let v = ConstantValue::read(&mut cursor);
        match v {
            Ok(value) => {
                println!("{:?}", value)
            }
            Err(err) => {
                println!("{:?}",err)
            }
        }
    }
}