//! A simple library for reading and writing Riegl .sdc files.
//!
//! .sdc files are simple binary tables of discrete-return LiDAR data.

#![deny(box_pointers, fat_ptr_transmutes, missing_copy_implementations, missing_debug_implementations, missing_docs, trivial_casts, trivial_numeric_casts, unsafe_code, unused_extern_crates, unused_import_braces, unused_qualifications, unused_results, variant_size_differences)]

extern crate byteorder;

pub mod error;
pub mod point;
pub mod reader;
mod result;
pub mod writer;

pub use error::Error;
pub use point::{Point, TargetType};
pub use reader::Reader;
pub use writer::Writer;
