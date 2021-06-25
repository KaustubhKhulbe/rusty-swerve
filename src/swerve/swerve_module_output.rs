use crate::math::point::Point;
use uom::si::f64::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SwerveModuleOutput {
    pub velocity: Point<Velocity>,
    pub bearing: Angle,
}
