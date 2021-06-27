use uom::si::angle::radian;
use uom::si::f64::*;

use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<Length> {
    pub fn new(x: Length, y: Length) -> Point<Length> {
        Point { x, y }
    }

    pub fn bearing(&self) -> Angle {
        Angle::new::<radian>(self.y.value.atan2(self.x.value))
    }

    pub fn magnitude(&self) -> Length {
        return (self.y.powi(::uom::typenum::P2::new()) + self.x.powi(::uom::typenum::P2::new()))
            .sqrt();
    }

    pub fn rotate(&mut self, angle: f64) -> Self {
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

impl Point<Velocity> {
    pub fn new(x: Velocity, y: Velocity) -> Point<Velocity> {
        Point { x, y }
    }

    pub fn bearing(&self) -> Angle {
        Angle::new::<radian>(self.y.value.atan2(self.x.value))
    }

    pub fn magnitude(&self) -> Velocity {
        return (self.y.powi(::uom::typenum::P2::new()) + self.x.powi(::uom::typenum::P2::new()))
            .sqrt();
    }

    pub fn rotate(&mut self, angle: f64) -> Self {
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

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
