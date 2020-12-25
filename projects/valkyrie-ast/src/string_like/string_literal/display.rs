use super::*;

impl Display for StringTextNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_char('\'')?;
        for c in self.text.chars() {
            match c {
                '\n' => f.write_str("\\n")?,
                '\r' => f.write_str("\\r")?,
                '\t' => f.write_str("\\t")?,
                '\\' => f.write_str("\\\\")?,
                '\'' => f.write_str("\\'")?,
                _ => f.write_char(c)?,
            }
        }
        f.write_char('\'')
    }
}

impl Display for StringLiteralNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if let Some(unit) = &self.handler {
            f.write_str(&unit.name)?;
        }
        Display::fmt(&self.as_raw(), f)
    }
}

impl PrettyPrint for StringLiteralNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.string(self.to_string())
    }
}

impl PrettyPrint for StringTextNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.string(self.to_string())
    }
}
