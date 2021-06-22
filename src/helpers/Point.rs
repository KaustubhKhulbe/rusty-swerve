use std::ops::{Add, Sub};

pub struct Point {
     x: f64,
     y: f64,
}

impl Point {
     fn bearing(&self) -> f64 {
          (self.y / self.x).tan()
     }

     fn rotate(&mut self, angle: f64) {
          self.x = angle.cos() * self.x - angle.sin() * self.y;
          self.y = angle.sin() * self.x + angle.cos() * self.y;
     }
}

impl Add for Point {
     type Output = Self;
     fn add(self, other: Self) -> Self {
          Point {
               x: self.x + other.x,
               y: self.y + other.y,
          }
     }
}

impl Sub for Point {
     type Output = Self;
     fn sub(self, other: Self) -> Self {
          Point {
               x: self.x - other.x,
               y: self.y - other.y,
          }
     }
}
