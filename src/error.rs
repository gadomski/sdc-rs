//! Our custom error types.

use std::error;
use std::fmt;
use std::io;
use std::str;

use byteorder;

/// Our custom error type.
#[derive(Debug)]
pub enum Error {
    /// Wrapper around a `byteorder::Error`.
    Byteorder(byteorder::Error),
    /// The header information is invalid.
    InvalidHeaderInformation,
    /// The given number cannot be converted to a target type.
    InvalidTargetType(u8),
    /// Wrapper around `std::io::Error`.
    Io(io::Error),
    /// Wrapper around `std::str::Error`.
    Utf8(str::Utf8Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Byteorder(ref err) => err.description(),
            Error::InvalidHeaderInformation => "invalid header information",
            Error::InvalidTargetType(_) => "invalid target type",
            Error::Io(ref err) => err.description(),
            Error::Utf8(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Byteorder(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Byteorder(ref err) => write!(f, "Byteorder error: {}", err),
            Error::InvalidHeaderInformation => write!(f, "Invalid header information"),
            Error::InvalidTargetType(n) => write!(f, "Invalid target type: {}", n),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Utf8(ref err) => write!(f, "Utf8 error: {}", err),
        }
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::Byteorder(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}
