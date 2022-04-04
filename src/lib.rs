pub mod io;
pub mod class;
pub mod gen;

#[cfg(test)]
mod tests {
    use std::io::{Cursor, stdout};

    use crate::class::class::Class;
    use crate::gen::ClassWriter;
    use crate::io::Readable;

    #[test]
    fn it_works() {
        let mut bytes = include_bytes!("../Test.class");
        let mut cursor = Cursor::new(&mut bytes);
        let out = Class::read(&mut cursor);
        match out {
            Ok(t) => {
                // println!("{:#?}", t.methods[1]);

                let w = ClassWriter {};
                w.write_class(&t, &mut stdout());
            }
            Err(t) => {
                println!("{:?}", t)
            }
        }
    }
}
