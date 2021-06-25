use std::ops::{Add, Sub};

use uom::si::f64::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector<T> {
    pub magnitude: T,
    pub bearing: Angle,
}

impl Vector<Length> {
    pub fn new(magnitude: Length, bearing: Angle) -> Vector<Length> {
        return Vector { magnitude, bearing };
    }

    pub fn x(&self) -> Length {
        return self.magnitude * self.bearing.value.cos();
    }

    pub fn y(&self) -> Length {
        return self.magnitude * self.bearing.value.sin();
    }

    pub fn rotate(&mut self, angle: f64) -> Vector<Length> {
        let x2 = angle.cos() * self.x() - angle.sin() * self.y();
        let y2 = angle.sin() * self.x() + angle.cos() * self.y();

        self.magnitude =
            (x2.powi(uom::typenum::P2::new()) + y2.powi(uom::typenum::P2::new())).sqrt();
        self.bearing = y2.atan2(x2);

        self.magnitude = self.magnitude;
        self.bearing = self.bearing;
        Vector {
            magnitude: self.magnitude,
            bearing: self.bearing,
        }
    }
}
