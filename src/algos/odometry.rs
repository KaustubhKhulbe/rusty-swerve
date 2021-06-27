use crate::math::point::Point;
use crate::math::position::Position;
use crate::swerve::drivetrain_distance_output::DriveTrainDistanceOutput;
use std::f64::consts::PI;
use uom::si::angle::radian;
use uom::si::f64::*;

pub struct Odometry {
    pub radius_width: Length,
    pub radius_length: Length,
    pub pos: Position,
}

impl Odometry {
    fn avg(&self, first: Length, other: Length) -> Length {
        (first + other) / 2.0
    }

    pub fn update(&mut self, module_outputs: &DriveTrainDistanceOutput) -> &Position {
        let mut xy_comps: Vec<Point<Length>> = Vec::new();

        let modules = module_outputs.get_vec();

        for module in modules.iter().take(4) {
            xy_comps.push(Point::<Length> {
                x: module.bearing.sin() * module.distance,
                y: module.bearing.cos() * module.distance,
            });
        }

        let top = self.avg(xy_comps[0].x, xy_comps[1].x);
        let bottom = self.avg(xy_comps[3].x, xy_comps[2].x);
        let left = self.avg(xy_comps[0].y, xy_comps[3].y);
        let right = self.avg(xy_comps[1].y, xy_comps[2].y);

        let omega1 = (top - bottom) / self.radius_width;
        let omega2 = (left - right) / self.radius_length;
        let omega = Angle::new::<radian>((omega1.value + omega2.value) / 2.0);

        let up_down = left + right;
        let left_right = top + bottom;

        let b1 = self.pos.bearing;
        let b2 = self.pos.bearing + Angle::new::<radian>(PI / 2.0);

        self.pos = self.pos
            + Position {
                x: ((up_down * b1.cos() + left_right * b2.cos()) / 2.0),
                y: ((up_down * b1.sin() + left_right * b2.sin()) / 2.0),
                bearing: omega,
            };
        &self.pos
    }
}
