use std::ops::{Add, Mul};

/// Defines the size of something in 2 dimensions.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Size2d {
    width: u32,
    height: u32,
}

pub const ZERO: Size2d = Size2d {
    width: 0,
    height: 0,
};

impl Size2d {
    /// Creates a new Size2d
    pub fn new(width: u32, height: u32) -> Size2d {
        Size2d { width, height }
    }

    /// Returns the number of tiles covered by an area of this size
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.get_tiles(), 6);
    /// ```
    pub fn get_tiles(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// Returns the size along the x-axis
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.width(), 2);
    /// ```
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the size along the y-axis
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.height(), 3);
    /// ```
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Converts an index to the x-coordinate of the equivalent point
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_x(5), 1);
    /// ```
    pub fn to_x(&self, index: usize) -> u32 {
        index as u32 % self.width
    }

    /// Converts an index to the y-coordinate of the equivalent point
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_y(5), 2);
    /// ```
    pub fn to_y(&self, index: usize) -> u32 {
        index as u32 / self.width
    }

    /// Converts an index to the equivalent point
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_point(5), [1,2]);
    /// ```
    pub fn to_point(&self, index: usize) -> [u32; 2] {
        [self.to_x(index), self.to_y(index)]
    }

    /// Converts a point to the equivalent index
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_index(1, 2), 5);
    /// ```
    pub fn to_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
}

/// Adds 2 sizes
///
/// ```
///# use rust_roguelike_core::math::size2d::Size2d;
/// let a = Size2d::new(2, 3);
/// let b = Size2d::new(10, 40);
/// assert_eq!(a + b, Size2d::new(12, 43));
/// ```
impl Add for Size2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Size2d {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

/// Multiplies 2 sizes
///
/// ```
///# use rust_roguelike_core::math::size2d::Size2d;
/// let a = Size2d::new(2, 3);
/// let b = Size2d::new(10, 40);
/// assert_eq!(a * b, Size2d::new(20, 120));
/// ```
impl Mul for Size2d {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Size2d {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}
