//! An SDC point record.
//!
//! At this point, we're keeping it simple and only handling 5.0.

#[derive(Default)]
pub struct Point {
    pub time: f64,
    pub range: f32,
    pub theta: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub amplitude: u16,
    pub width: u16,
    pub target_type: TargetType,
    pub target: u8,
    pub num_target: u8,
    pub rg_index: u16,
    pub facet_number: u8,
    pub high_channel: bool,
}

impl Point {
    /// Creates a new, default point.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdc::point::Point;
    /// let point = Point::new();
    /// ```
    pub fn new() -> Point {
        Default::default()
    }

    /// Returns the channel description byte from this point.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdc::point::Point;
    /// let point = Point::new();
    /// let byte = point.channel_desc_byte();
    /// ```
    pub fn channel_desc_byte(&self) -> u8 {
        let mut byte = self.facet_number & 0x3;
        if self.high_channel {
            byte |= 0b01000000;
        }
        byte
    }
}

pub enum TargetType {
    CenterOfGravity,
    Parabola,
    Gaussian,
}

impl TargetType {
    /// Returns this target type as a `u8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdc::point::TargetType;
    /// assert_eq!(0, TargetType::CenterOfGravity.as_u8());
    /// assert_eq!(1, TargetType::Parabola.as_u8());
    /// assert_eq!(2, TargetType::Gaussian.as_u8());
    /// ```
    pub fn as_u8(&self) -> u8 {
        match *self {
            TargetType::CenterOfGravity => 0,
            TargetType::Parabola => 1,
            TargetType::Gaussian => 2,
        }
    }
}

impl Default for TargetType {
    fn default() -> TargetType {
        TargetType::CenterOfGravity
    }
}