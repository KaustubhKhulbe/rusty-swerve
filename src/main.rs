pub mod helpers;
use crate::helpers::Position::Position;
use crate::helpers::Vector::Vector;
use std::f64::consts::PI;

fn main() {
    testVector();
}

fn testVector() {
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
