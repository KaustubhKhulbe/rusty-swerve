use crate::swerve::module_displacement::ModuleDisplacement;

pub struct DriveTrainDistanceOutput {
    pub tr: ModuleDisplacement,
    pub br: ModuleDisplacement,
    pub bl: ModuleDisplacement,
    pub tl: ModuleDisplacement,
}

impl DriveTrainDistanceOutput {
    pub fn get_vec(&self) -> Vec<ModuleDisplacement> {
        return vec![self.tl, self.tr, self.bl, self.br];
    }
}
