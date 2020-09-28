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
        if let Some(unit) = &self.unit {
            f.write_str(&unit.name)?;
        }
        Display::fmt(&self.as_raw(), f)
    }
}

impl PrettyPrint for StringLiteralNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.string(self.to_string())
    }
}

impl PrettyPrint for StringTextNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.string(self.to_string())
    }
}
