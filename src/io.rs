use std::io;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};
use paste::paste;

use crate::error::ReadError;

type ReadResult<A> = Result<A, ReadError>;

pub trait Readable: Send + Sync {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized;
}

impl Readable for u8 {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
        i.read_u8().map_err(ReadError::from)
    }
}

impl Readable for i8 {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
        i.read_i8().map_err(ReadError::from)
    }
}

trait ReadByteVecExt: io::Read {
    #[inline]
    fn read_bytes_vec(&mut self, length: usize) -> ReadResult<Vec<u8>> {
        let mut buffer = Vec::with_capacity(length);
        unsafe { buffer.set_len(length) }
        self.read_exact(&mut buffer).map_err(ReadError::from)?;
        Ok(buffer)
    }
}

trait ReadVecExt: io::Read {
    #[inline]
    fn read_vec<C: Readable>(&mut self, length: usize) -> ReadResult<Vec<u8>> {
        let mut out = Vec::with_capacity(length);
        for _ in 0..length {
            out.push(C::read(self)?)
        }
        Ok(out)
    }
}

impl<R: io::Read> ReadByteVecExt for R {}
impl<R: io::Read> ReadVecExt for R {};

// Macro for implementing the readable trait on numbers
// that support the BigEndian encoding.
macro_rules! be_readable {
    (
        $($type:ident ($fn:ident)),*
    ) => {
        $(
            impl Readable for $type {
                fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
                     i.$fn::<BigEndian>().map_err(ReadError::from)
                }
            }
        )*
    };
}

be_readable!(
    u16 (read_u16), u32 (read_u32),
    i16 (read_i16), i32 (read_i32),
    f32 (read_f32), f64 (read_f64)
);

impl Readable for String {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
        let length = u16::read(i)? as usize;
        let bytes = i.read_bytes_vec(length)?;
        Ok(
            String::from_utf8(bytes)
                .map_err(ReadError::from)?
        )
    }
}

