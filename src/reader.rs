//! Read points from an .sdc file.

use std::fs::File;
use std::iter::IntoIterator;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str;

use byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

use error::SdcError;
use point::{Point, TargetType};
use result::Result;

/// An object for readings .sdc points.
///
/// We don't just read them all into memory right away since .sdc files can be quite big.
#[derive(Debug)]
pub struct Reader<R: Read> {
    reader: R,
    version: (u16, u16),
    header_information: Vec<u8>,
}

impl Reader<BufReader<File>> {
    /// Creates a new reader for a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdc::reader::Reader;
    /// let reader = Reader::from_path("data/4-points.sdc").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<BufReader<File>>> {
        let reader = BufReader::new(try!(File::open(path)));
        Reader::new(reader)
    }
}

impl<R: Read> Reader<R> {
    /// Creates a new reader, consuimg a `Read`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use sdc::reader::Reader;
    /// let file = File::open("data/4-points.sdc").unwrap();
    /// let reader = Reader::new(file);
    /// ```
    pub fn new(mut reader: R) -> Result<Reader<R>> {
        let header_size = try!(reader.read_u32::<LittleEndian>());
        let major = try!(reader.read_u16::<LittleEndian>());
        let minor = try!(reader.read_u16::<LittleEndian>());
        let header_information_size = header_size - 8;
        let mut header_information = Vec::with_capacity(header_information_size as usize);
        if try!(reader.by_ref()
                      .take(header_information_size as u64)
                      .read_to_end(&mut header_information)) !=
           header_information_size as usize {
            return Err(SdcError::InvalidHeaderInformation);
        }
        Ok(Reader {
            reader: reader,
            version: (major, minor),
            header_information: header_information,
        })
    }

    /// Reads the next point from the underlying `Read`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdc::reader::Reader;
    /// let mut reader = Reader::from_path("data/4-points.sdc").unwrap();
    /// let point = reader.next_point();
    /// ```
    pub fn next_point(&mut self) -> Result<Option<Point>> {
        // Technically we should just check the first byte instead of the first four, but the work
        // required to do that doesn't seem worth it at the moment.
        let time = match self.reader.read_f64::<LittleEndian>() {
            Ok(time) => time,
            Err(byteorder::Error::UnexpectedEOF) => return Ok(None),
            Err(err) => return Err(SdcError::from(err)),
        };
        let range = try!(self.reader.read_f32::<LittleEndian>());
        let theta = try!(self.reader.read_f32::<LittleEndian>());
        let x = try!(self.reader.read_f32::<LittleEndian>());
        let y = try!(self.reader.read_f32::<LittleEndian>());
        let z = try!(self.reader.read_f32::<LittleEndian>());
        let amplitude = try!(self.reader.read_u16::<LittleEndian>());
        let width = try!(self.reader.read_u16::<LittleEndian>());
        let target_type = try!(TargetType::from_u8(try!(self.reader.read_u8())));
        let target = try!(self.reader.read_u8());
        let num_target = try!(self.reader.read_u8());
        let rg_index = try!(self.reader.read_u16::<LittleEndian>());
        let channel_desc_byte = try!(self.reader.read_u8());
        Ok(Some(Point {
            time: time,
            range: range,
            theta: theta,
            x: x,
            y: y,
            z: z,
            amplitude: amplitude,
            width: width,
            target_type: target_type,
            target: target,
            num_target: num_target,
            rg_index: rg_index,
            facet_number: channel_desc_byte & 0x3,
            high_channel: (channel_desc_byte & 0b01000000) == 0b01000000,
        }))
    }

    /// Returns this file's version as a `(u16, u16)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdc::reader::Reader;
    /// let reader = Reader::from_path("data/4-points.sdc").unwrap();
    /// let (major, minor) = reader.version();
    /// ```
    pub fn version(&self) -> (u16, u16) {
        self.version
    }

    /// Returns this file's header information, or an error if it is not valid ASCII.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdc::reader::Reader;
    /// let reader = Reader::from_path("data/4-points.sdc").unwrap();
    /// let header_information = reader.header_information_as_str();
    /// ```
    pub fn header_information_as_str(&self) -> Result<&str> {
        str::from_utf8(&self.header_information[..]).map_err(|e| SdcError::from(e))
    }
}

impl<R: Read> IntoIterator for Reader<R> {
    type Item = Point;
    type IntoIter = PointIterator<R>;
    fn into_iter(self) -> Self::IntoIter {
        PointIterator { reader: self }
    }
}

/// An iterator over a reader's points.
#[derive(Debug)]
pub struct PointIterator<R: Read> {
    reader: Reader<R>,
}

impl<R: Read> Iterator for PointIterator<R> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next_point().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_points() {
        let reader = Reader::from_path("data/4-points.sdc").unwrap();
        let points: Vec<_> = reader.into_iter().collect();
        assert_eq!(4, points.len());
    }
}
