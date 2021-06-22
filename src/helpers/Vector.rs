use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
     pub magnitude: f64,
     pub bearing: f64,
}

impl Vector {
     pub fn x(&self) -> f64 {
          self.magnitude * self.bearing.cos()
     }

     pub fn y(&self) -> f64 {
          self.magnitude * self.bearing.sin()
     }

     pub fn rotate(&mut self, angle: f64) -> Vector {
          let x2 = angle.cos() * self.x() - angle.sin() * self.y();
          let y2 = angle.sin() * self.x() + angle.cos() * self.y();

          self.magnitude = (x2.powf(2.0) + y2.powf(2.0)).sqrt();
          self.bearing = (y2 / x2).tan();

          Vector {
               magnitude: self.magnitude,
               bearing: self.bearing,
          }
     }
}

impl Add for Vector {
     type Output = Self;

     /* Need to double check logic here, but seems okay */
     fn add(self, other: Self) -> Self {
          Self {
               magnitude: ((self.x() + other.x()).powf(2.0) + (self.y() + other.y()).powf(2.0))
                    .sqrt(),
               bearing: self.bearing + (other.bearing - self.bearing),
          }
     }
}

impl Sub for Vector {
     type Output = Self;

     /* Need to double check logic here, but seems okay */
     fn sub(self, other: Self) -> Self {
          Self {
               magnitude: ((self.x() - other.x()).powf(2.0) + (self.y() - other.y()).powf(2.0))
                    .sqrt(),
               bearing: self.bearing - (other.bearing - self.bearing),
          }
     }
}
