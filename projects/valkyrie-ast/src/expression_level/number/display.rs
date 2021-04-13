use super::*;

impl Display for NumberLiteralNode {
    /// `base⁂digits.decimal⁑shift_unit;`
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.base {
            10 => {
                self.write_digits(f)?;
                match self.shift {
                    0 => {}
                    shift => write!(f, "⁑{}", shift)?,
                }
                if let Some(id) = &self.unit {
                    write!(f, "_{}", id.name)?
                }
            }
            base => {
                write!(f, "{}⁂", base)?;
                self.write_digits(f)?;
                match &self.unit {
                    Some(unit) if self.shift == 0 => write!(f, "⁑{}", unit.name)?,
                    Some(unit) => write!(f, "⁑{}_{}", self.shift, unit.name)?,
                    None if self.shift != 0 => write!(f, "⁑{}", self.shift)?,
                    None => {}
                }
            }
        }
        Ok(())
    }
}

impl NumberLiteralNode {
    fn write_digits(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.integer.as_str() {
            "" => f.write_str("0")?,
            s => f.write_str(s)?,
        }
        match self.decimal.as_str() {
            "" => {}
            s => {
                f.write_str(".")?;
                f.write_str(s)?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for NumberLiteralNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let num = theme.number(self.value.to_string());
        match &self.unit {
            Some(s) => {
                let unit = theme.annotation(s.name.to_string());
                num.append(unit)
            }
            None => num,
        }
    }
}

#[cfg(feature = "lispify")]
impl Lispify for NumberLiteralNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let number = Lisp::number(self.value.clone());
        match &self.unit {
            Some(s) => number & Lisp::unit(s.name.clone()),
            None => number,
        }
    }
}
