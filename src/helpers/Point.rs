use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point {
    pub(crate) fn bearing(&self) -> f64 {
        (self.y).atan2(self.x)
    }

    pub(crate) fn rotate(&mut self, angle: f64) -> Self {
        let x_temp = angle.cos() * self.x - angle.sin() * self.y;
        let y_temp = angle.sin() * self.x + angle.cos() * self.y;

        //note: update done after so that it is using previous values
        //rounds number to the 3rd digit
        self.x = (1000.0 * x_temp).round() / 1000.0;
        self.y = (1000.0 * y_temp).round() / 1000.0;
        Self {
            x: self.x,
            y: self.y,
        }
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
