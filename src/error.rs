// This file contains the all the possible errors that could be thrown
use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("unknown constant tag {0}")]
    UnknownConstantTag(u8)
}