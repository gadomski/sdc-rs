//! SDC file management.

use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};

use point::Point;
use result::Result;

/// An SDC file.
pub struct File<W: Write> {
    writer: W,
}

impl File<BufWriter<fs::File>> {
    /// Creates a new SDC file and opens it for writing.
    ///
    /// The file will be closed when the file goes out of scope.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::remove_file;
    /// use sdc::file::File;
    /// {
    ///     let file = File::create("temp.sdc").unwrap();
    /// }
    /// remove_file("temp.sdc").unwrap();
    pub fn create<P: AsRef<Path>>(path: P) -> Result<File<BufWriter<fs::File>>> {
        let mut writer = BufWriter::new(try!(fs::File::create(path)));
        try!(writer.write_u32::<LittleEndian>(8));
        // TODO hardcoded version, make this smahtar.
        try!(writer.write_u16::<LittleEndian>(5));
        try!(writer.write_u16::<LittleEndian>(0));
        Ok(File { writer: writer })
    }
}

impl<W: Write> File<W> {
    /// Writes a point to this SDC file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs::remove_file;
    /// use sdc::file::File;
    /// use sdc::point::Point;
    /// let ref point = Point::new();
    /// {
    ///     let mut file = File::create("temp.sdc").unwrap();
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
