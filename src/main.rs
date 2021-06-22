pub mod helpers;
use crate::helpers::Position::Position;
use crate::helpers::Vector::Vector;
use std::f64::consts::PI;

fn main() {
    let v1 = Vector {
        magnitude: 1.0,
        bearing: (PI / 4.0),
    };

    let v2 = Vector {
        magnitude: 1.0,
        bearing: (PI / 4.0),
    };

    println!("Value is: {:?}", (v1 - v2));
}
