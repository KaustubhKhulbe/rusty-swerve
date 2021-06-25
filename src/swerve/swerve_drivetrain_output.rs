use crate::swerve::swerve_module_output::SwerveModuleOutput;

pub struct SwerveDrivetrainOutput {
    pub tl: SwerveModuleOutput,
    pub tr: SwerveModuleOutput,
    pub bl: SwerveModuleOutput,
    pub br: SwerveModuleOutput,
}

impl SwerveDrivetrainOutput {
    pub fn new(
        tl: SwerveModuleOutput,
        tr: SwerveModuleOutput,
        bl: SwerveModuleOutput,
        br: SwerveModuleOutput,
    ) -> SwerveDrivetrainOutput {
        SwerveDrivetrainOutput { tl, tr, bl, br }
    }

    pub fn get_vector(&self) -> Vec<SwerveModuleOutput> {
        return vec![self.tl, self.tr, self.bl, self.br];
    }
}
