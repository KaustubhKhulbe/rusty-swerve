use crate::math::point::Point;
use crate::math::position::Position;
use crate::swerve::swerve_drivetrain_output::SwerveDrivetrainOutput;
use crate::swerve::swerve_module_output::SwerveModuleOutput;
use uom::si::angular_velocity::degree_per_second;
use uom::si::f64::*;
use uom::si::length::foot;
use uom::si::time::second;

pub struct Control {
    pub radius_vecs: Vec<Point<Length>>,
    pub pos: Position,
    pub radius_width: Length,
    pub radius_length: Length,
}

impl Control {
    pub fn get_swerve_module_angles(
        &mut self,
        target_velocity: Point<Velocity>,
        target_angular_velocity: AngularVelocity,
    ) -> SwerveDrivetrainOutput {
        self.radius_vecs = vec![
            Point::<Length>::new(self.radius_length, -self.radius_width),
            Point::<Length>::new(self.radius_length, self.radius_width),
            Point::<Length>::new(-self.radius_length, self.radius_width),
            Point::<Length>::new(-self.radius_length, -self.radius_width),
        ];

        // need to confirm if name: index is in the right order
        return SwerveDrivetrainOutput {
            tr: self.get_module_output(2, target_velocity, target_angular_velocity),
            br: self.get_module_output(3, target_velocity, target_angular_velocity),
            bl: self.get_module_output(0, target_velocity, target_angular_velocity),
            tl: self.get_module_output(1, target_velocity, target_angular_velocity),
        };
    }

    fn get_module_output(
        &mut self,
        i: usize,
        target_velocity: Point<Velocity>,
        target_angular_velocity: AngularVelocity,
    ) -> SwerveModuleOutput {
        let vec = Point::<Velocity>::new(
            target_velocity.x + target_angular_velocity * self.radius_vecs[i].x,
            target_velocity.y + target_angular_velocity * self.radius_vecs[i].y,
        );

        return SwerveModuleOutput {
            velocity: vec.magnitude(),
            bearing: vec.bearing(),
        };
    }
}
