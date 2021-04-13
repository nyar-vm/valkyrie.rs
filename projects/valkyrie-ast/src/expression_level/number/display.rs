use super::*;

impl Display for NumberLiteralNode {
    /// `16⁂FF.AA⁑shift;`
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.base {
            10 if self.shift == 0 => match &self.unit {
                Some(unit) => write!(f, "{}_{}", self.digits, unit),
                _ => write!(f, "{}", self.digits),
            },
            10 => match &self.unit {
                Some(unit) => write!(f, "{}⁑{}_{}", self.digits, self.shift, unit),
                _ => write!(f, "{}⁑{}", self.digits, self.shift),
            },
            base => match self.shift {
                0 => match &self.unit {
                    Some(unit) => write!(f, "{}_{}", self.digits, unit),
                    _ => write!(f, "{}", self.digits),
                },
                shift => match &self.unit {
                    Some(unit) => write!(f, "{}⁑{}_{}", self.digits, self.shift, unit),
                    _ => write!(f, "{}⁑{}", self.digits, self.shift),
                },
            },
        }
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
