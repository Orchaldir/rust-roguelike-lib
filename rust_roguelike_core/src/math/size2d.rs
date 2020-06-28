use std::ops::{Add, Mul};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Size2d {
    x: u32,
    y: u32,
}

pub const ZERO: Size2d = Size2d { x: 0, y: 0 };

impl Size2d {
    pub fn new(x: u32, y: u32) -> Size2d {
        Size2d { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
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
