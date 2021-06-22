use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Vector {
     pub magnitude: f64,
     pub bearing: f64,
}

impl Vector {
     fn x(&self) -> f64 {
          self.magnitude * self.bearing.cos()
     }

     fn y(&self) -> f64 {
          self.magnitude * self.bearing.sin()
     }

     fn rotate(&self, angle: f64) -> Vector {
          let x2 = angle.cos() * self.x() - angle.sin() * self.y();
          let y2 = angle.sin() * self.x() + angle.cos() * self.y();

          Vector {
               magnitude: (x2.powf(2.0) + y2.powf(2.0)).sqrt(),
               bearing: (y2 / x2).tan(),
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
