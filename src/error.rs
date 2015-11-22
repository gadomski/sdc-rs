//! Our custom error types.

use std::error::Error;
use std::fmt;
use std::io;
use std::str;

use byteorder;

#[derive(Debug)]
pub enum SdcError {
    Byteorder(byteorder::Error),
    InvalidHeaderInformation,
    InvalidTargetType(u8),
    Io(io::Error),
    Utf8(str::Utf8Error),
}

impl Error for SdcError {
    fn description(&self) -> &str {
        match *self {
            SdcError::Byteorder(ref err) => err.description(),
            SdcError::InvalidHeaderInformation => "invalid header information",
            SdcError::InvalidTargetType(_) => "invalid target type",
            SdcError::Io(ref err) => err.description(),
            SdcError::Utf8(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SdcError::Byteorder(ref err) => Some(err),
            SdcError::Io(ref err) => Some(err),
            SdcError::Utf8(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for SdcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SdcError::Byteorder(ref err) => write!(f, "Byteorder error: {}", err),
            SdcError::InvalidHeaderInformation => write!(f, "Invalid header information"),
            SdcError::InvalidTargetType(n) => write!(f, "Invalid target type: {}", n),
            SdcError::Io(ref err) => write!(f, "IO error: {}", err),
            SdcError::Utf8(ref err) => write!(f, "Utf8 error: {}", err),
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

impl From<str::Utf8Error> for SdcError {
    fn from(err: str::Utf8Error) -> SdcError {
        SdcError::Utf8(err)
    }
}
