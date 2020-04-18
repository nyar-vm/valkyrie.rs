use std::fmt::{Display, Formatter};
use crate::number::ValkyrieNumber;

impl Display for ValkyrieNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)?;
        if let Some(unit) = &self.unit {
            f.write_str(" ")?;
            f.write_str(&unit.name)?;
        }
        Ok(())
    }
}