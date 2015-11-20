//! A simple library for reading and writing Riegl .sdc files.
//!
//! .sdc files are simple binary tables of discrete-return LiDAR data.

extern crate byteorder;

mod error;
pub mod file;
pub mod point;
mod result;

pub use file::File;