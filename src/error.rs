//! Our custom error types.

use std::error::Error;
use std::fmt;
use std::io;

use byteorder;

#[derive(Debug)]
pub enum SdcError {
    Byteorder(byteorder::Error),
    Io(io::Error),
}

impl Error for SdcError {
    fn description(&self) -> &str {
        match *self {
            SdcError::Byteorder(ref err) => err.description(),
            SdcError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SdcError::Byteorder(ref err) => Some(err),
            SdcError::Io(ref err) => Some(err),
        }
    }
}

impl fmt::Display for SdcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SdcError::Byteorder(ref err) => write!(f, "Byteorder error: {}", err),
            SdcError::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
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
