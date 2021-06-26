# RustySwerveAlgos

FRC Swerve Robot Odometry and Control Algorithms, written in Rust.

Just run `cargo run` to run the algorithms. It does several assertion tests to validate the algorithm behaves as expected.

Structure:
```
src
│   ├── math - util structs such as points and position
│   ├── algos - key algorithms: control + odometry
│   └── swerve - swerve specific structs
```
