use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::error::ReadError;

pub type ReadResult<A> = Result<A, ReadError>;

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

// Trait for reading vec of a runtime known size
pub trait VecReadableSize: Sized {
    fn read_vec<C: Readable, R: Read>(r: &mut R) -> ReadResult<Vec<C>>;
}

// Trait for reading vec of a runtime known size
pub trait VecReadableBytesSize: Sized {
    fn read_bytes<R: Read>(r: &mut R) -> ReadResult<Vec<u8>>;
}

// Trait for read a vec of values of a runtime size using a closure for reading
pub trait VecReadableFn: Sized {
    fn read_vec_closure<C, R: Read, F: Fn(&mut R) -> ReadResult<C>>(r: &mut R, f: F) -> ReadResult<Vec<C>>;
}

macro_rules! impl_vec_readable {
    ($($type:ty),*) => {
        $(
            impl VecReadableSize for $type {
                 fn read_vec<C: Readable, R: Read>(r: &mut R) -> ReadResult<Vec<C>> {
                    let length = <$type>::read(r)? as usize;
                    let mut out = Vec::with_capacity(length);
                    for _ in 0..length {
                        out.push(C::read(r)?)
                    }
                    Ok(out)
                 }
            }

            impl VecReadableBytesSize for $type {
                 fn read_bytes<R: Read>(r: &mut R) -> ReadResult<Vec<u8>> {
                    let length = <$type>::read(r)? as usize;
                    let mut buffer = Vec::with_capacity(length);
                    unsafe { buffer.set_len(length) }
                    r.read_exact(&mut buffer).map_err(ReadError::from)?;
                    Ok(buffer)
                 }
            }

            impl VecReadableFn for $type {
                fn read_vec_closure<C, R: Read, F: Fn(&mut R) -> ReadResult<C>>(r: &mut R, f: F) -> ReadResult<Vec<C>> {
                    let length = <$type>::read(r)? as usize;
                    let mut out = Vec::with_capacity(length);
                    for _ in 0..length {
                        out.push(f(r)?)
                    }
                    Ok(out)
                }
            }
        )*
    };
}

impl_vec_readable!(u8,u16,u32);


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
    i16 (read_i16), u16 (read_u16),
    u32 (read_u32), i32 (read_i32),
    i64 (read_i64), u64 (read_u64),
    f32 (read_f32), f64 (read_f64)
);

impl Readable for String {
    fn read<R: Read>(i: &mut R) -> ReadResult<Self> where Self: Sized {
        let bytes = u16::read_bytes(i)?;
        Ok(
            String::from_utf8(bytes)
                .map_err(ReadError::from)?
        )
    }
}

#[macro_export]
macro_rules! readable_struct {
    (
        $(
            struct $name:ident {
                $($field:ident: $type:ty,)*
            }
        )*
    ) => {
        $(
            #[derive(Debug, Clone)]
            #[allow(dead_code)]
            pub struct $name {
                pub $($field: $type,)*
            }

            impl Readable for $name {
                fn read<R: Read>(i: &mut R) -> $crate::io::ReadResult<Self> where Self: Sized {
                    Ok(Self {
                        $($field: <$type>::read(i)?),*
                    })
                }
            }

        )*
    };
}