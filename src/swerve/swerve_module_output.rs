use uom::si::f64::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SwerveModuleOutput {
    pub velocity: Velocity,
    pub bearing: Angle,
}
