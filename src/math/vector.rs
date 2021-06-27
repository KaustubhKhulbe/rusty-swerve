use uom::si::f64::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector<T> {
    pub magnitude: T,
    pub bearing: Angle,
}

impl Vector<Length> {
    pub fn new(magnitude: Length, bearing: Angle) -> Vector<Length> {
        Vector { magnitude, bearing }
    }

    pub fn x(&self) -> Length {
        self.magnitude * self.bearing.value.cos()
    }

    pub fn y(&self) -> Length {
        self.magnitude * self.bearing.value.sin()
    }

    pub fn rotate(&mut self, angle: f64) -> Vector<Length> {
        let x2 = angle.cos() * self.x() - angle.sin() * self.y();
        let y2 = angle.sin() * self.x() + angle.cos() * self.y();

        let magnitude =
            (x2.powi(uom::typenum::P2::new()) + y2.powi(uom::typenum::P2::new())).sqrt();
        let bearing = y2.atan2(x2);

        self.magnitude = magnitude;
        self.bearing = bearing;
        Vector {
            magnitude: self.magnitude,
            bearing: self.bearing,
        }
    }
}
