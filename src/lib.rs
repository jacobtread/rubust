pub mod io;
pub mod class;

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::class::class::Class;
    use crate::io::{Readable};

    #[test]
    fn it_works() {
        let mut bytes = include_bytes!("../Main.class");
        let mut cursor = Cursor::new(&mut bytes);
        let out = <Class>::read(&mut cursor);
        match out {
            Ok(t) => {
               println!("{:#?}", t.methods[0])
            }
            Err(t) => {
                println!("{:?}", t)
            }
        }
    }
}
