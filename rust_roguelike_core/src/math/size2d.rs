use std::ops::{Add, Mul};

/// Defines the size of something in 2 dimensions.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Size2d {
    x: u32,
    y: u32,
}

pub const ZERO: Size2d = Size2d { x: 0, y: 0 };

impl Size2d {
    /// Creates a new Size2d
    pub fn new(x: u32, y: u32) -> Size2d {
        Size2d { x, y }
    }

    /// Returns the size along the x-axis
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.x(), 2);
    /// ```
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Returns the size along the y-axis
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.y(), 3);
    /// ```
    pub fn y(&self) -> u32 {
        self.y
    }

    /// Converts an index to the x-coordinate of the equivalent point
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_x(5), 1);
    /// ```
    pub fn to_x(&self, index: usize) -> u32 {
        index as u32 % self.x
    }

    /// Converts an index to the y-coordinate of the equivalent point
    ///
    /// ```
    ///# use rust_roguelike_core::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_y(5), 2);
    /// ```
    pub fn to_y(&self, index: usize) -> u32 {
        index as u32 / self.x
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
}

impl Add for Size2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Size2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Size2d {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Size2d {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
