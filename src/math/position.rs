use std::ops::{Add, Sub};
use uom::si::f64::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    pub x: Length,
    pub y: Length,
    pub bearing: Angle,
}

impl Position {
    pub fn new(x: Length, y: Length, bearing: Angle) -> Position {
        Position { x, y, bearing }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            bearing: self.bearing + other.bearing,
        }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            bearing: self.bearing - other.bearing,
        }
    }
}
