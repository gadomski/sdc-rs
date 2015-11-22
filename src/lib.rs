//! A simple library for reading and writing Riegl .sdc files.
//!
//! .sdc files are simple binary tables of discrete-return LiDAR data.

extern crate byteorder;

pub mod error;
pub mod point;
mod result;
pub mod writer;

pub use writer::Writer;
pub use point::{Point, TargetType};
