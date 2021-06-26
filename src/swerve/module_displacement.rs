use uom::si::f64::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ModuleDisplacement {
    pub distance: Length,
    pub bearing: Angle,
}

impl ModuleDisplacement {
    pub fn new(distance: Length, bearing: Angle) -> ModuleDisplacement {
        return ModuleDisplacement { distance, bearing };
    }
}
