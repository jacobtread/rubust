// This file contains the all the possible errors that could be thrown
use std::io;

use thiserror::Error;

use crate::class::constant::PoolIndex;

#[derive(Error, Debug)]
pub enum WriteError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    DecompileError(#[from] DecompileError),
    #[error("method descriptor expected")]
    BadDescriptor,
    #[error("code attribute wasn't code attribute")]
    BadCodeAttribute,
}

#[derive(Error, Debug)]
pub enum DecompileError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    ReadErr(#[from] ReadError),
    #[error("unknown array type {0}")]
    UnknownArrayType(u8),
    #[error("unknown op code {0}")]
    UnknownInstruction(u8),
    #[error("expected method descriptor")]
    ExpectedMethodDescriptor,
    #[error(transparent)]
    InvalidConstant(#[from] ConstantError),
    #[error(transparent)]
    StackError(#[from] StackError),
}

#[derive(Error, Debug)]
pub enum StackError {
    #[error("nothing left on the stack")]
    Empty,
    #[error("stack still had {0} items remaining")]
    Remaining(usize),
    #[error("not enough items. needed {0} but only had {1}")]
    NotEnough(usize, usize),
}

#[derive(Error, Debug)]
pub enum ReadError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("unknown constant tag {0}")]
    UnknownConstantTag(u8),
    #[error("invalid class magic signature got 0x{0} expected 0xCAFEBABE")]
    InvalidMagic(u32),
    #[error(transparent)]
    InvalidConstant(#[from] ConstantError),
    #[error("class name was not found in constant pool")]
    NoClassName,
}

#[derive(Error, Debug)]
pub enum ConstantError {
    #[error("no constant found in pool at index {0}")]
    NotFound(PoolIndex),
    #[error("expected value at index {0} to be utf-8")]
    ExpectedUtf8(PoolIndex),
    #[error("expected value at index {0} to be class")]
    InvalidClassReference(PoolIndex),
    #[error("expected value at index {0} to be utf-8 class name")]
    InvalidClassReference2(PoolIndex),
    #[error("expected value at index {0} to be method/interface ref")]
    ExpectedMethodRef(PoolIndex),
    #[error("expected value at index {0} to be invoke dynamic")]
    ExpectedInvokeDynamic(PoolIndex),
}