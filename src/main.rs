pub mod algos;
pub mod math;
pub mod swerve;
pub mod tests;

use crate::tests::math_and_algo_tests;

use bevy::prelude::*;

fn main() {
    math_and_algo_tests::math_and_algo_tests::run_tests();

    App::build().add_plugins(DefaultPlugins).run();
}
