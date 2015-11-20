//! Our custom result types.

use std::result;

use error::SdcError;

pub type Result<T> = result::Result<T, SdcError>;
