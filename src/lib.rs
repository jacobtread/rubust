///! This is a rust library for working with Java class files
pub mod io;
pub mod error;
mod class;

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::io::Readable;

    #[test]
    fn io_works() {
        let mut arr = vec![1u8, 0, 5];
        let mut cursor = Cursor::new(arr);
        let a = u16::read(&mut cursor).expect("Value exist");
        println!("{}", a)
    }
}