use super::*;
use lispify::ListString;

impl Display for StringLiteralNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)?;
        if let Some(unit) = &self.unit {
            f.write_str(" ")?;
            f.write_str(&unit.name)?;
        }
        Ok(())
    }
}

impl Lispify for StringLiteralNode {
    type Output = ListString;

    fn lispify(&self) -> Self::Output {
        ListString { text: self.value.to_owned(), unit: self.unit.clone().map(|u| u.name).unwrap_or_default() }
    }
}
