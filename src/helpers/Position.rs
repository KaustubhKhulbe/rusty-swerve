use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
     pub x: f64,
     pub y: f64,
     pub bearing: f64,
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
