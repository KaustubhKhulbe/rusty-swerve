use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
     pub x: f64,
     pub y: f64,
     pub bearing: f64,
}

impl Position {
     pub fn new(x: f64, y: f64, bearing: f64) -> Position {
          Position {
               x: (1000.0 * x).round() / 1000.0,
               y: (1000.0 * y).round() / 1000.0,
               bearing,
          }
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
