use crate::{ValkyrieError, ValkyrieResult};

#[repr(u8)]
pub enum LicenseType {
    Allow,
    Warning,
    Deny,
    Forbid,
}

impl LicenseType {
    pub fn allow(&mut self) -> ValkyrieResult<()> {
        match self {
            LicenseType::Forbid => Err(ValkyrieError::custom("License is forbidden")),
            _ => {
                *self = LicenseType::Allow;
                Ok(())
            }
        }
    }
    pub fn warn(&mut self) -> ValkyrieResult<()> {
        match self {
            LicenseType::Forbid => Err(ValkyrieError::custom("License is forbidden")),
            _ => {
                *self = LicenseType::Warning;
                Ok(())
            }
        }
    }
    pub fn deny(&mut self) -> ValkyrieResult<()> {
        match self {
            LicenseType::Forbid => Err(ValkyrieError::custom("License is forbidden")),
            _ => {
                *self = LicenseType::Deny;
                Ok(())
            }
        }
    }
    pub fn forbid(&mut self) -> ValkyrieResult<()> {
        match self {
            LicenseType::Forbid => Err(ValkyrieError::custom("License is forbidden")),
            _ => {
                *self = LicenseType::Forbid;
                Ok(())
            }
        }
    }
}
