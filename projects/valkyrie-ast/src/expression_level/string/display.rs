use super::*;

impl Display for StringLiteralNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.value)?;
        if let Some(unit) = &self.unit {
            f.write_str(" ")?;
            f.write_str(&unit.name)?;
        }
        Ok(())
    }
}
