pub mod algos;
pub mod math;

// math functions
use crate::math::point::Point;
use crate::math::position::Position;
use crate::math::vector::Vector;
use std::f64::consts::PI;

use crate::algos::control::SwerveControl;
use crate::algos::odometry::SwerveOdometry;

fn main() {
    test_point();
    test_position();
    test_vector();
    test_swerve_odometry();
    test_swerve_control();
}

fn test_point() {
    let mut p1 = Point { x: 5.0, y: 5.0 };

    let mut p2 = Point { x: 5.0, y: 5.0 };

    assert_eq!(p1 + p2, Point { x: 10.0, y: 10.0 });

    assert_eq!(p1 - p2, Point { x: 0.0, y: 0.0 });

    assert_eq!(p1.rotate(PI / 4.0), Point { x: 0.0, y: 7.071 });

    assert_eq!(p1.bearing(), PI / 2.0);
}

fn test_position() {
    let pos1 = Position {
        x: 100.0,
        y: 200.0,
        bearing: 0.0,
    };

    let pos2 = Position {
        x: 30.0,
        y: 40.0,
        bearing: PI / 2.0,
    };

    assert_eq!(
        pos1 + pos2,
        Position {
            x: 130.0,
            y: 240.0,
            bearing: PI / 2.0
        }
    );
}

fn test_vector() {
    let mut v1 = Vector {
        magnitude: 1.0,
        bearing: PI / 4.0,
    };
    let v2 = Vector {
        magnitude: 1.0,
        bearing: PI / 4.0,
    };

    assert_eq!(
        Vector {
            magnitude: 2.0,
            bearing: (1000.0 * PI / 4.0).round() / 1000.0
        },
        v1 + v2
    );

    assert_eq!(
        Vector {
            magnitude: 0.0,
            bearing: 0.0
        },
        v1 - v2
    );

    v1.rotate(PI / 4.0);
    assert_eq!(
        Vector {
            magnitude: 1.0,
            bearing: (1000.0 * PI / 2.0).round() / 1000.0
        },
        v1
    );
}

fn test_swerve_odometry() {
    let mut odom = SwerveOdometry {
        radius_width: 2.0,
        radius_length: 2.0,
        pos: Position {
            x: 0.0,
            y: 0.0,
            bearing: 0.0,
        },
    };

    //move in straight line
    let mut w1 = (5.0, 0.0);
    let mut w2 = (5.0, 0.0);
    let mut w3 = (5.0, 0.0);
    let mut w4 = (5.0, 0.0);

    let mut modules = vec![&mut w1, &mut w2, &mut w3, &mut w4];
    odom.update(&mut modules);

    assert_eq!(
        odom.pos,
        Position {
            x: 5.0,
            y: 0.0,
            bearing: 0.0
        }
    );

    odom.reset_position();

    //move diagonally
    let mut w1 = (5.0, PI / 4.0);
    let mut w2 = (5.0, PI / 4.0);
    let mut w3 = (5.0, PI / 4.0);
    let mut w4 = (5.0, PI / 4.0);

    let mut modules = vec![&mut w1, &mut w2, &mut w3, &mut w4];
    odom.update(&mut modules);

    assert_eq!(
        odom.pos,
        Position {
            x: (5.0 / 2.0f64.sqrt() * 1000.0).round() / 1000.0,
            y: (5.0 / 2.0f64.sqrt() * 1000.0).round() / 1000.0,
            bearing: 0.0
        }
    );

    odom.reset_position();
}

fn test_swerve_control() {
    let mut control = SwerveControl {
        radius_vecs: vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 0.0),
            Point::new(0.0, 0.0),
            Point::new(0.0, 0.0),
        ],
        pos: Position {
            x: 0.0,
            y: 0.0,
            bearing: 0.0,
        },
        radius_width: 2.0,
        radius_length: 2.0,
    };

    let module_angles = control.get_swerve_module_angles(Point::new(0.0, 3.0), 0.0);
    println!("{:#?}", module_angles);

    // Need to put JUnit tests
}
