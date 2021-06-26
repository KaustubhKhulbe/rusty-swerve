use crate::swerve::module_displacement::ModuleDisplacement;

pub struct DriveTrainDistanceOutput {
    pub tl: ModuleDisplacement,
    pub tr: ModuleDisplacement,
    pub bl: ModuleDisplacement,
    pub br: ModuleDisplacement,
}

impl DriveTrainDistanceOutput {
    pub fn get_vec(&self) -> Vec<ModuleDisplacement> {
        return vec![self.tl.clone(), self.tr, self.bl, self.br];
    }
}
