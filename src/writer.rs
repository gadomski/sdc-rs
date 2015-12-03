//! Object for creating .sdc files.

use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};

use point::Point;
use result::Result;

/// An .sdc writer.
#[derive(Debug)]
pub struct Writer<W: Write> {
    writer: W,
}

impl Writer<BufWriter<fs::File>> {
    /// Creates a new SDC file and opens it for writing.
    ///
    /// The file will be closed when the file goes out of scope.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdc::writer::Writer;
    /// let writer = Writer::from_path("/dev/null").unwrap();
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Writer<BufWriter<fs::File>>> {
        let writer = BufWriter::new(try!(fs::File::create(path)));
        Writer::new(writer)
    }
}

impl<W: Write> Writer<W> {
    /// Creates a new writer, consuming the provided `Write`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use sdc::writer::Writer;
    /// let writer = File::create("/dev/null").unwrap();
    /// let writer = Writer::new(writer).unwrap();
    pub fn new(mut writer: W) -> Result<Writer<W>> {
        try!(writer.write_u32::<LittleEndian>(8));
        // TODO hardcoded version, make this smahtar.
        try!(writer.write_u16::<LittleEndian>(5));
        try!(writer.write_u16::<LittleEndian>(0));
        Ok(Writer { writer: writer })
    }

    /// Writes a point to this SDC file.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdc::writer::Writer;
    /// use sdc::point::Point;
    /// let ref point = Point::new();
    /// let mut file = Writer::from_path("/dev/null").unwrap();
    /// file.write_point(point).unwrap();
    pub fn write_point(&mut self, point: &Point) -> Result<()> {
        try!(self.writer.write_f64::<LittleEndian>(point.time));
        try!(self.writer.write_f32::<LittleEndian>(point.range));
        try!(self.writer.write_f32::<LittleEndian>(point.theta));
        try!(self.writer.write_f32::<LittleEndian>(point.x));
        try!(self.writer.write_f32::<LittleEndian>(point.y));
        try!(self.writer.write_f32::<LittleEndian>(point.z));
        try!(self.writer.write_u16::<LittleEndian>(point.amplitude));
        try!(self.writer.write_u16::<LittleEndian>(point.width));
        try!(self.writer.write_u8(point.target_type.as_u8()));
        try!(self.writer.write_u8(point.target));
        try!(self.writer.write_u8(point.num_target));
        try!(self.writer.write_u16::<LittleEndian>(point.rg_index));
        try!(self.writer.write_u8(point.channel_desc_byte()));
        Ok(())
    }
}
