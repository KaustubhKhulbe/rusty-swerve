use crate::math::position::Position;
use std::f64::consts::PI;

pub struct SwerveOdometry {
    pub radius_width: f64,
    pub radius_length: f64,
    pub pos: Position,
}

impl SwerveOdometry {
    fn avg(&self, first: f64, other: f64) -> f64 {
        (first + other) / 2.0
    }
    pub fn update(&mut self, modules: &mut Vec<&mut (f64, f64)>) {
        let mut xyComps: Vec<(f64, f64)> = Vec::new();
        for i in 0..modules.len() {
            xyComps.push((
                modules[i].1.sin() * modules[i].0,
                modules[i].1.cos() * modules[i].0,
            ));
        }

        let top = self.avg(xyComps[0].0, xyComps[1].0);
        let bottom = self.avg(xyComps[3].0, xyComps[2].0);
        let left = self.avg(xyComps[0].1, xyComps[3].1);
        let right = self.avg(xyComps[1].1, xyComps[2].1);

        let omega1 = (top - bottom) / self.radius_width;
        let omega2 = (left - right) / self.radius_length;
        let omega = (omega1 + omega2) / 2.0;

        let upDown = left + right;
        let leftRight = top + bottom;

        let b1 = self.pos.bearing;
        let b2 = self.pos.bearing + (PI / 2.0);

        self.pos = self.pos
            + Position {
                x: (1000.0 * ((upDown * b1.cos() + leftRight * b2.cos()) / 2.0)).round() / 1000.0,
                y: (1000.0 * ((upDown * b1.sin() + leftRight * b2.sin()) / 2.0)).round() / 1000.0,
                bearing: omega,
            }
    }

    pub fn reset_position(&mut self) {
        self.pos = Position::new(0.0, 0.0, 0.0);
    }
}
