use std::ops::{Add, Sub};

use uom::si::f64::*;
use uom::si::length::foot;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    pub magnitude: Length,
    pub bearing: Angle,
}

impl Vector {
    pub fn x(&self) -> Length {
        return self.magnitude * self.bearing.value.cos();
    }

    pub fn y(&self) -> Length {
        return self.magnitude * self.bearing.value.sin();
    }

    pub fn rotate(&mut self, angle: f64) -> Vector {
        let x2 = angle.cos() * self.x() - angle.sin() * self.y();
        let y2 = angle.sin() * self.x() + angle.cos() * self.y();

        self.magnitude = Length::new::<foot>((x2.value.powf(2.0) + y2.value.powf(2.0)).sqrt());
        self.bearing = y2.atan2(x2);

        self.magnitude = self.magnitude;
        self.bearing = self.bearing;
        Vector {
            magnitude: self.magnitude,
            bearing: self.bearing,
        }
    }
}
