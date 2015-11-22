//! A simple library for reading and writing Riegl .sdc files.
//!
//! .sdc files are simple binary tables of discrete-return LiDAR data.

extern crate byteorder;

pub mod error;
pub mod point;
pub mod reader;
mod result;
pub mod writer;

pub use point::{Point, TargetType};
pub use reader::Reader;
pub use writer::Writer;
