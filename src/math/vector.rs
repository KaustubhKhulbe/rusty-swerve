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
          self.bearing = y2.atan2(x2);

          self.magnitude = (1000.0 * self.magnitude).round() / 1000.0;
          self.bearing = (1000.0 * self.bearing).round() / 1000.0;
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
          let new_x = self.x() + other.x();
          let new_y = self.y() + other.y();
          Self {
               magnitude: (new_x.powf(2.0) + new_y.powf(2.0)).sqrt(),
               bearing: (1000.0 * new_y.atan2(new_x)).round() / 1000.0,
          }
     }
}

impl Sub for Vector {
     type Output = Self;

     /* Need to double check logic here, but seems okay */
     fn sub(self, other: Self) -> Self {
          let new_x = self.x() - other.x();
          let new_y = self.y() - other.y();
          Self {
               magnitude: (new_x.powf(2.0) + new_y.powf(2.0)).sqrt(),
               bearing: (1000.0 * new_y.atan2(new_x)).round() / 1000.0,
          }
     }
}
