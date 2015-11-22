//! Object for creating .sdc files.

use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};

use point::Point;
use result::Result;

/// An .sdc writer.
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
    /// use std::fs::remove_file;
    /// use sdc::writer::Writer;
    /// {
    ///     let writer = Writer::from_path("temp.sdc").unwrap();
    /// }
    /// remove_file("temp.sdc").unwrap();
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Writer<BufWriter<fs::File>>> {
        let writer = BufWriter::new(try!(fs::File::create(path)));
        let mut writer = Writer {
            writer: writer,
        };
        try!(writer.write_header());
        Ok(writer)
    }
}

impl<W: Write> Writer<W> {
    fn write_header(&mut self) -> Result<()> {
        try!(self.writer.write_u32::<LittleEndian>(8));
        // TODO hardcoded version, make this smahtar.
        try!(self.writer.write_u16::<LittleEndian>(5));
        try!(self.writer.write_u16::<LittleEndian>(0));
        Ok(())
    }

    /// Writes a point to this SDC file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs::remove_file;
    /// use sdc::writer::Writer;
    /// use sdc::point::Point;
    /// let ref point = Point::new();
    /// {
    ///     let mut file = Writer::from_path("temp.sdc").unwrap();
    ///     file.write_point(point).unwrap();
    /// }
    /// # remove_file("temp.sdc").unwrap();
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
