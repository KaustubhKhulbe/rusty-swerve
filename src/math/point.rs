use uom::si::angle::radian;
use uom::si::f64::*;
use uom::si::length::foot;

use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: Length,
    pub y: Length,
}

impl Point {
    pub fn new(x: Length, y: Length) -> Point {
        Point { x, y }
    }

    pub fn bearing(&self) -> Angle {
        return Angle::new::<radian>(self.y.value.atan2(self.x.value));
    }

    pub fn magnitude(&self) -> Length {
        return Length::new::<foot>((self.y.value.powf(2.0) + self.x.value.powf(2.0)).sqrt());
    }

    pub(crate) fn rotate(&mut self, angle: f64) -> Self {
        let x_temp = angle.cos() * self.x - angle.sin() * self.y;
        let y_temp = angle.sin() * self.x + angle.cos() * self.y;

        //note: update done after so that it is using previous values
        self.x = x_temp;
        self.y = y_temp;
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
