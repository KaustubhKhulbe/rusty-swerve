use crate::math::point::Point;
use crate::math::position::Position;

pub struct SwerveControl {
     pub radius_vecs: Vec<Point>,
     pub pos: Position,
     pub radius_width: f64,
     pub radius_length: f64,
}

impl SwerveControl {
     pub fn get_swerve_module_angles(
          &mut self,
          target_velocity: Point,
          target_angular_velocity: f64,
     ) -> [(f64, f64); 4] {
          self.radius_vecs = vec![
               Point::new(self.radius_length, -self.radius_width),
               Point::new(self.radius_length, self.radius_width),
               Point::new(-self.radius_length, self.radius_width),
               Point::new(-self.radius_length, -self.radius_width),
          ];

          let mut wheel_vecs = [(0.0, 0.0); 4];

          for i in 0..4 {
               let vec = Point::new(
                    target_velocity.x + target_angular_velocity * self.radius_vecs[i].x,
                    target_velocity.y + target_angular_velocity * self.radius_vecs[i].y,
               );

               wheel_vecs[i] = (vec.magnitude(), vec.bearing());
          }

          return wheel_vecs;
     }
}
