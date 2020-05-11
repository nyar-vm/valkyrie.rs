use super::*;

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

impl Lispify for ValkyrieNumber {
    type Output = LispNumber;

    fn lispify(&self) -> Self::Output {
        LispNumber { number: self.value.clone(), unit: self.unit.clone().map(|s| s.name).unwrap_or_default() }
    }
}
