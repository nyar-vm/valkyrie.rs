use crate::{ValkyrieError, ValkyrieResult};

#[repr(u8)]
pub enum LicenseType {
    Allow,
    Warning,
    Deny,
    Forbid,
}

/// The mutability of a value. Only reference types have this concept, value types are immutable.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MutableType {
    /// The value cannot be changed
    Immutable,
    /// The value can only be change in the constructor
    ReadOnly,
    /// The value can be changed at any time
    Mutable,
}

impl Default for MutableType {
    fn default() -> Self {
        Self::Immutable
    }
}

/// The laziness of a value.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LazyType {
    /// The value must be computed at compile time
    Constant,
    /// The value must be computed at runtime
    Eager,
    /// This value will be initialized to null, but you need to ensure that this value has been assigned when you call
    Late,
    /// The value must be computed at the first access
    Lazy,
}

impl Default for LazyType {
    fn default() -> Self {
        Self::Eager
    }
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
