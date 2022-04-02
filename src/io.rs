use std::io::{Read, Write};

use anyhow::{Context, Result};
use byteorder::{BE, ReadBytesExt, WriteBytesExt};

pub trait Readable: Send + Sync {
    fn read<B: Read>(i: &mut B) -> Result<Self> where Self: Sized;
}

pub trait Writable: Send + Sync {
    fn write<B: Write>(&mut self, o: &mut B) -> Result<()>;
}

impl Writable for u8 {
    fn write<B: Write>(&mut self, o: &mut B) -> Result<()> {
        o.write_u8(*self)?;
        Ok(())
    }
}

impl Readable for u8 {
    fn read<B: Read>(i: &mut B) -> Result<Self> where Self: Sized {
        B::read_u8(i).map_err(anyhow::Error::from)
    }
}

impl Writable for i8 {
    fn write<B: Write>(&mut self, o: &mut B) -> Result<()> {
        o.write_i8(*self)?;
        Ok(())
    }
}

impl Readable for i8 {
    fn read<B: Read>(i: &mut B) -> Result<Self> where Self: Sized {
        B::read_i8(i).map_err(anyhow::Error::from)
    }
}

macro_rules! generate_rw {
    (
        $($type:ident: ($read_fn:ident, $write_fn:ident))*
    ) => {
        $(
            impl Writable for $type {
                fn write<B: Write>(&mut self, o: &mut B) -> Result<()> {
                    o.$write_fn::<BE>(*self)?;
                    Ok(())
                }
            }

            impl Readable for $type {
                fn read<B: Read>(i: &mut B) -> Result<Self> where Self: Sized {
                    i.$read_fn::<BE>()
                        .map_err(anyhow::Error::from)
                }
            }
        )*
    };
}

generate_rw! {
    u16: (read_u16, write_u16)
    u32: (read_u32, write_u32)
    u64: (read_u64, write_u64)

    i16: (read_i16, write_i16)
    i32: (read_i32, write_i32)
    i64: (read_i64, write_i64)

    f32: (read_f32, write_f32)
    f64: (read_f64, write_f64)
}


impl Readable for String {
    fn read<B: Read>(i: &mut B) -> Result<Self> where Self: Sized {
        let length = <u16>::read(i)? as usize;
        let mut bytes = Vec::with_capacity(length);
        unsafe { bytes.set_len(length) }
        i.read_exact(&mut bytes)
            .map_err(anyhow::Error::from)?;
        Ok(
            String::from_utf8(bytes)
                .context("string contained invalid utf-8 encoding")?
        )
    }
}

impl Writable for String {
    fn write<B: Write>(&mut self, o: &mut B) -> Result<()> {
        (self.len() as u16).write(o)?;
        o.write_all(self.as_bytes())?;
        Ok(())
    }
}