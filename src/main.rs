pub mod math;
pub mod swerve;

use crate::math::point::Point;

use uom::si::f64::*;
use uom::si::length::foot;
use uom::si::time::second;

fn main() {
    let p1 = Point {
        x: Length::new::<foot>(5.0) / Time::new::<second>(2.0),
        y: Length::new::<foot>(5.0) / Time::new::<second>(2.0),
    };

    println!("{}", p1.x.value);
}
