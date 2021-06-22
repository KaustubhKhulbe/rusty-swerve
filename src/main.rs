pub mod helpers;
mod odometry;

use crate::helpers::Position::Position;
use crate::helpers::Vector::Vector;
use std::f64::consts::PI;
use crate::helpers::Point::Point;

fn main() {
    test_point();
    test_position();
}

fn test_point() {
    let mut p1 = Point {
        x: 5.0,
        y: 5.0,
    };

    let mut p2 = Point {
        x: 5.0,
        y: 5.0,
    };

    assert_eq!(p1 + p2, Point {
        x: 10.0,
        y: 10.0,
    });

    assert_eq!(p1 - p2, Point {
        x: 0.0,
        y: 0.0,
    });

    assert_eq!(p1.rotate(PI / 4.0), Point {
        x: 0.0,
        y: 7.071,
    });

    assert_eq!(p1.bearing(), PI/2.0);
}

fn test_position() {
    let pos1 = Position{
        x: 100.0,
        y: 200.0,
        bearing: 0.0
    };

    let pos2 = Position{
        x: 30.0,
        y: 40.0,
        bearing: PI / 2.0,
    };

    assert_eq!(pos1 + pos2, Position{
        x: 130.0,
        y: 240.0,
        bearing: PI / 2.0
    });
}

fn test_vector() {
    let mut v1 = Vector {
        magnitude: 2.0,
        bearing: (PI / 4.0),
    };

    let mut v2 = Vector {
        magnitude: 1.0,
        bearing: (PI / 4.0),
    };

    assert_eq!(
        v1 + v2,
        Vector {
            magnitude: 3.0,
            bearing: (PI / 4.0)
        }
    );

    assert_eq!(
        v1 - v2,
        Vector {
            magnitude: 1.0,
            bearing: (PI / 4.0)
        }
    );

    assert_eq!(
        v1.rotate(PI / 4.0),
        Vector {
            magnitude: 2.0,
            bearing: PI / 2.0,
        }
    )
}
