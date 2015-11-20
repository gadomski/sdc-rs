//! Our custom error types.

use std::io;

use byteorder;

#[derive(Debug)]
pub enum SdcError {
    Byteorder(byteorder::Error),
    Io(io::Error),
}

impl From<byteorder::Error> for SdcError {
    fn from(err: byteorder::Error) -> SdcError {
        SdcError::Byteorder(err)
    }
}

impl From<io::Error> for SdcError {
    fn from(err: io::Error) -> SdcError {
        SdcError::Io(err)
    }
}
